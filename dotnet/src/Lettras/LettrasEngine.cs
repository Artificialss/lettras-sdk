
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using Wasmtime;

namespace Lettras;

/// <summary>The engine rejected the input. The message says what to change.</summary>
public sealed class LettrasInputException(string message) : Exception(message);

/// <summary>The engine failed to load or run.</summary>
public sealed class LettrasEngineException(string message, Exception? inner = null) : Exception(message, inner);

/// <summary>
/// Word-search, crossword and sudoku generators plus Unicode text styles for Spanish, English, Portuguese, French, German
/// and Italian, with native accented letters kept (one cell each).
/// The compiled Lettras engine is embedded and runs in-process (WebAssembly through Wasmtime): no network, no API key.
/// Inputs are anything that serialises to the engine's camelCase JSON (an anonymous object, a dictionary or your own
/// type); outputs are <see cref="JsonObject"/>. The same input and <c>seed</c> give the same puzzle.
/// The engine is proprietary: see ENGINE-LICENSE.txt.
/// </summary>
public sealed class LettrasEngine : IDisposable
{
    /// <summary>CPU budget per call, in Wasmtime fuel units. Bounds worst-case inputs.</summary>
    private const ulong Fuel = 20_000_000_000;

    private static readonly JsonSerializerOptions Json = new() { PropertyNamingPolicy = JsonNamingPolicy.CamelCase, DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull };

    private readonly Engine _engine;
    private readonly Module _module;

    /// <summary>Loads the embedded engine.</summary>
    public LettrasEngine()
    {
        try
        {
            _engine = new Engine(new Config().WithFuelConsumption(true));
            using var stream = System.Reflection.Assembly.GetExecutingAssembly().GetManifestResourceStream("lettras_engine.wasm")
                ?? throw new LettrasEngineException("engine file is missing");
            using var bytes = new MemoryStream();
            stream.CopyTo(bytes);
            _module = Module.FromBytes(_engine, "lettras", bytes.ToArray());
        }
        catch (LettrasEngineException) { throw; }
        catch (Exception e) { throw new LettrasEngineException($"engine failed to load: {e.Message}", e); }
    }

    /// <summary>A word search. Input: words, rows, cols, and optionally position ("horizontal", "vertical", "mixed"), difficulty (1 to 4), clustering (0 to 1), seed, lang ("es" "en" "pt" "fr" "de" "it"), classicMode. Empty cells are "-".</summary>
    public JsonObject Generate(object input) => Call("lettras_generate", input);

    /// <summary>Fills the empty cells of a word search with random letters. Input: grid, and optionally lang, accents, seed, words (so the filler never makes an extra copy of one). Without a seed every call differs.</summary>
    public JsonObject Fill(object input) => Call("lettras_fill", input);

    /// <summary>Text in Unicode styles that keep accents. Input: text, and optionally style (one id or a list) and classic.</summary>
    public JsonObject Stylize(object input) => Call("lettras_stylize", input);

    /// <summary>The available text styles: { styles: [{ id, label, category }] }.</summary>
    public JsonObject Styles() => Call("lettras_styles", "");

    /// <summary>A crossword. Input: entries ([{ word, clue }]), rows, cols, and optionally seed. Returns puzzle, solution, placements and unplaced.</summary>
    public JsonObject Crossword(object input) => Call("lettras_crossword", input);

    /// <summary>A sudoku with exactly one solution. Input (all optional): size (4, 6, 9, 12, 16), difficulty (1 to 4), seed. 0 is an empty cell.</summary>
    public JsonObject Sudoku(object? input = null) => Call("lettras_sudoku", input ?? new { });

    /// <summary>Solves a sudoku. Input: { puzzle: [[...]] }. Returns solution, solved and unique.</summary>
    public JsonObject SolveSudoku(object input) => Call("lettras_sudoku_solve", input);

    private JsonObject Call(string function, object input)
    {
        string payload;
        try { payload = JsonSerializer.Serialize(input, Json); }
        catch (Exception e) { throw new LettrasInputException($"input is not valid JSON: {e.Message}"); }

        var (text, isError) = Run(function, Encoding.UTF8.GetBytes(payload));
        if (isError) throw new LettrasInputException(text);
        return JsonNode.Parse(text) as JsonObject ?? throw new LettrasEngineException("engine returned invalid JSON");
    }

    /// <summary>One call on a fresh instance, so no state survives between calls.</summary>
    private (string Text, bool IsError) Run(string function, byte[] input)
    {
        try
        {
            using var store = new Store(_engine);
            store.Fuel = Fuel;
            using var linker = new Linker(_engine);
            var instance = linker.Instantiate(store, _module);

            var memory = instance.GetMemory("memory") ?? throw new LettrasEngineException("engine has no memory export");
            var alloc = instance.GetFunction<int, int>("lettras_alloc") ?? throw new LettrasEngineException("engine has no lettras_alloc");
            var free = instance.GetAction<int, int>("lettras_free") ?? throw new LettrasEngineException("engine has no lettras_free");
            var entry = instance.GetFunction<int, int, long>(function) ?? throw new LettrasEngineException($"engine has no {function}");
            var resultPtr = instance.GetFunction<int>("lettras_result_ptr") ?? throw new LettrasEngineException("engine has no lettras_result_ptr");

            var ptr = alloc(input.Length);
            input.AsSpan().CopyTo(memory.GetSpan<byte>(ptr, input.Length));
            long r;
            try { r = entry(ptr, input.Length); }
            catch (TrapException) when (store.Fuel == 0) { throw new LettrasEngineException("puzzle took too long to generate"); }
            free(ptr, input.Length);

            var length = (int)Math.Abs(r);
            var text = Encoding.UTF8.GetString(memory.GetSpan<byte>(resultPtr(), length));
            return (text, r < 0);
        }
        catch (LettrasEngineException) { throw; }
        catch (Exception e) { throw new LettrasEngineException($"the engine failed: {e.Message}", e); }
    }

    /// <summary>Releases the engine.</summary>
    public void Dispose()
    {
        _module.Dispose();
        _engine.Dispose();
    }
}
