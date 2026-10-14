# Lettras for Go

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each). The engine runs in-process: no network, no API key.

```bash
go get github.com/Artificialss/lettras-sdk/go
```

```go
package main

import (
	"context"
	"fmt"

	lettras "github.com/Artificialss/lettras-sdk/go"
)

func main() {
	ctx := context.Background()
	engine, err := lettras.New(ctx)
	if err != nil {
		panic(err)
	}
	defer engine.Close(ctx)

	puzzle, _ := engine.Generate(ctx, map[string]any{"words": []string{"gato", "perro", "piña"}, "rows": 9, "cols": 12, "position": "mixed", "seed": 8})
	done, _ := engine.Fill(ctx, map[string]any{"grid": puzzle["grid"], "lang": "es", "accents": true}) // random letters in the empty cells
	fmt.Println(done["grid"])

	sudoku, _ := engine.Sudoku(ctx, map[string]any{"size": 9, "difficulty": 2, "seed": 4})        // exactly one solution, 0 = empty
	solved, _ := engine.SolveSudoku(ctx, map[string]any{"puzzle": sudoku["puzzle"]})            // solution, solved, unique
	crossword, _ := engine.Crossword(ctx, map[string]any{"entries": []map[string]string{{"word": "gato", "clue": "Felino"}, {"word": "toro", "clue": "Bovino"}}, "rows": 10, "cols": 10})
	bold, _ := engine.Stylize(ctx, map[string]any{"text": "Ñandú", "style": "bold"})
	_, _, _ = solved, crossword, bold
}
```

Inputs are anything that marshals to the engine's camelCase JSON (a map or your own struct); results are decoded JSON objects. Options are the same as the
[npm package](https://www.npmjs.com/package/lettras). Bad input returns an `*InputError` whose message says what to change.

**Every platform.** The engine runs through [wazero](https://wazero.io), a pure-Go WebAssembly runtime: no cgo, no native libraries, so it builds for any `GOOS`/`GOARCH`
Go supports (Linux, macOS, Windows, FreeBSD; amd64, arm64 and more) and cross-compiles with a plain `go build`. Each call runs on a fresh instance with a 20-second timeout
(`lettras.Timeout`), and an `Engine` is safe for concurrent use.

**License.** The wrapper is MIT. The compiled engine is proprietary: free to use unmodified, as distributed in this module, in your own applications (commercial ones
included); no extraction, modification, reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `engine/LICENSE`.
