using System.Text.Json.Nodes;
using Lettras;
using Xunit;

public class EngineTests
{
    private static readonly LettrasEngine Engine = new();

    [Fact]
    public void WordSearchKeepsAccentsAndIsDeterministic()
    {
        var input = new { words = new[] { "gato", "piña", "mono" }, rows = 8, cols = 8, seed = 3 };
        var a = Engine.Generate(input);
        var b = Engine.Generate(input);
        Assert.Equal(8, a["grid"]!.AsArray().Count);
        Assert.Equal(a["grid"]!.ToJsonString(), b["grid"]!.ToJsonString());
        Assert.Contains("\"Ñ\"", a["grid"]!.ToJsonString().Replace("\\u00D1", "Ñ"));
    }

    [Fact]
    public void FillLeavesNoEmptyCells()
    {
        var p = Engine.Generate(new { words = new[] { "sol", "luna" }, rows = 6, cols = 6, seed = 1 });
        var done = Engine.Fill(new { grid = p["grid"]!.DeepClone(), lang = "es", seed = 2 });
        Assert.DoesNotContain("\"-\"", done["grid"]!.ToJsonString());
    }

    [Fact]
    public void SudokuCrosswordAndStyles()
    {
        var s = Engine.Sudoku(new { size = 9, difficulty = 2, seed = 4 });
        var solved = Engine.SolveSudoku(new { puzzle = s["puzzle"]!.DeepClone() });
        Assert.True(solved["unique"]!.GetValue<bool>());
        var c = Engine.Crossword(new
        {
            entries = new[] { new { word = "gato", clue = "Felino" }, new { word = "toro", clue = "Bovino" }, new { word = "ropa", clue = "Se viste" } },
            rows = 8, cols = 8, seed = 1,
        });
        Assert.NotEmpty(c["placements"]!.AsArray());
        Assert.NotEmpty(Engine.Styles()["styles"]!.AsArray());
        Assert.NotNull(Engine.Stylize(new { text = "Ñandú", style = "bold" }));
    }

    [Fact]
    public void BadInputIsAnInputException() =>
        Assert.Throws<LettrasInputException>(() => Engine.Generate(new { words = new[] { "gato" }, rows = 0, cols = 5 }));
}
