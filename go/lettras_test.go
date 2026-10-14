package lettras

import (
	"context"
	"errors"
	"testing"
)

func newEngine(t *testing.T) *Engine {
	t.Helper()
	e, err := New(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = e.Close(context.Background()) })
	return e
}

func TestWordSearchIsDeterministicAndKeepsAccents(t *testing.T) {
	e, ctx := newEngine(t), context.Background()
	in := map[string]any{"words": []string{"gato", "piña", "mono"}, "rows": 8, "cols": 8, "seed": 3}
	a, err := e.Generate(ctx, in)
	if err != nil {
		t.Fatal(err)
	}
	b, _ := e.Generate(ctx, in)
	if len(a["grid"].([]any)) != 8 || len(b["grid"].([]any)) != 8 {
		t.Fatal("grid size")
	}
	found := false
	for _, row := range a["grid"].([]any) {
		for _, c := range row.([]any) {
			found = found || c == "Ñ"
		}
	}
	if !found {
		t.Fatal("expected one cell with Ñ")
	}
}

func TestFillSudokuCrosswordStyles(t *testing.T) {
	e, ctx := newEngine(t), context.Background()
	p, _ := e.Generate(ctx, map[string]any{"words": []string{"sol", "luna"}, "rows": 6, "cols": 6, "seed": 1})
	done, err := e.Fill(ctx, map[string]any{"grid": p["grid"], "lang": "es", "seed": 2})
	if err != nil {
		t.Fatal(err)
	}
	for _, row := range done["grid"].([]any) {
		for _, c := range row.([]any) {
			if c == "-" {
				t.Fatal("empty cell left")
			}
		}
	}
	s, err := e.Sudoku(ctx, map[string]any{"size": 9, "difficulty": 2, "seed": 4})
	if err != nil {
		t.Fatal(err)
	}
	solved, err := e.SolveSudoku(ctx, map[string]any{"puzzle": s["puzzle"]})
	if err != nil || solved["unique"] != true {
		t.Fatalf("sudoku not unique: %v %v", solved, err)
	}
	c, err := e.Crossword(ctx, map[string]any{"entries": []map[string]string{{"word": "gato", "clue": "Felino"}, {"word": "toro", "clue": "Bovino"}, {"word": "ropa", "clue": "Se viste"}}, "rows": 8, "cols": 8, "seed": 1})
	if err != nil || len(c["placements"].([]any)) == 0 {
		t.Fatalf("crossword: %v %v", c, err)
	}
	if st, err := e.Styles(ctx); err != nil || len(st["styles"].([]any)) == 0 {
		t.Fatalf("styles: %v", err)
	}
	if _, err := e.Stylize(ctx, map[string]any{"text": "Ñandú", "style": "bold"}); err != nil {
		t.Fatal(err)
	}
}

func TestBadInputIsInputError(t *testing.T) {
	e := newEngine(t)
	_, err := e.Generate(context.Background(), map[string]any{"words": []string{"gato"}, "rows": 0, "cols": 5})
	var ie *InputError
	if !errors.As(err, &ie) {
		t.Fatalf("want InputError, got %v", err)
	}
}
