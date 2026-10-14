# Lettras for .NET

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each). The engine runs in-process: no network, no API key.

```bash
dotnet add package Lettras
```

```csharp
using Lettras;

using var lettras = new LettrasEngine();

var puzzle = lettras.Generate(new { words = new[] { "gato", "perro", "piña" }, rows = 9, cols = 12, position = "mixed", seed = 8 });
var done = lettras.Fill(new { grid = puzzle["grid"]!.DeepClone(), lang = "es", accents = true });   // random letters in the empty cells
Console.WriteLine(done["grid"]);

var crossword = lettras.Crossword(new { entries = new[] { new { word = "gato", clue = "Felino" }, new { word = "toro", clue = "Bovino" } }, rows = 10, cols = 10 });
var sudoku = lettras.Sudoku(new { size = 9, difficulty = 2, seed = 4 });          // exactly one solution, 0 = empty
var solved = lettras.SolveSudoku(new { puzzle = sudoku["puzzle"]!.DeepClone() }); // solution, solved, unique
var bold = lettras.Stylize(new { text = "Ñandú", style = "bold" });
```

Inputs are anything that serialises to the engine's camelCase JSON; results are `System.Text.Json.Nodes.JsonObject`. Options are the same as the
[npm package](https://www.npmjs.com/package/lettras): `position`, `difficulty`, `clustering`, `seed` (same input and seed give the same puzzle), `lang`, `classicMode`.
Bad input throws `LettrasInputException` with a message that says what to change.

**How it works.** The compiled engine is embedded in the assembly as WebAssembly and run with [Wasmtime](https://www.nuget.org/packages/Wasmtime). Each call runs on a fresh
instance with a CPU budget, so one call cannot affect the next or run forever. Wasmtime ships native libraries for Windows, macOS and Linux (x64 and arm64).

**License.** The wrapper is MIT. The compiled engine is proprietary: free to use unmodified, as distributed in this package, in your own applications (commercial ones
included); no extraction, modification, reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `ENGINE-LICENSE.txt`.
