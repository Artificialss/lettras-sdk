// Package lettras generates word searches, crosswords and sudokus, and Unicode text styles, for Spanish, English,
// Portuguese, French, German and Italian, keeping native accented letters (one cell each).
//
// The compiled Lettras engine is embedded and runs in-process (WebAssembly through wazero, pure Go, no cgo), so the
// package builds and runs on every platform Go supports. No network, no API key. Inputs are anything that marshals to
// the engine's camelCase JSON (a map or your own struct); outputs are decoded JSON objects. The same input and "seed"
// give the same puzzle.
//
// The engine is proprietary: see engine/LICENSE.
package lettras

import (
	"context"
	_ "embed"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
)

//go:embed engine/lettras_engine.wasm
var engineWasm []byte

// Timeout bounds one engine call. Worst-case inputs are cut off with an error.
const Timeout = 20 * time.Second

// InputError means the engine rejected the input. The message says what to change.
type InputError struct{ Message string }

func (e *InputError) Error() string { return e.Message }

// Result is a decoded JSON object returned by the engine.
type Result = map[string]any

// Engine is the loaded engine. It is safe for concurrent use: every call runs on a fresh instance, so no state survives
// between calls.
type Engine struct {
	runtime wazero.Runtime
	module  wazero.CompiledModule
}

// New loads the embedded engine. Call Close when done.
func New(ctx context.Context) (*Engine, error) {
	rt := wazero.NewRuntimeWithConfig(ctx, wazero.NewRuntimeConfig().WithCloseOnContextDone(true))
	m, err := rt.CompileModule(ctx, engineWasm)
	if err != nil {
		_ = rt.Close(ctx)
		return nil, fmt.Errorf("engine failed to load: %w", err)
	}
	return &Engine{runtime: rt, module: m}, nil
}

// Close releases the engine.
func (e *Engine) Close(ctx context.Context) error { return e.runtime.Close(ctx) }

// Generate makes a word search. Input: words, rows, cols, and optionally position ("horizontal", "vertical", "mixed"),
// difficulty (1 to 4), clustering (0 to 1), seed, lang ("es" "en" "pt" "fr" "de" "it"), classicMode. Empty cells are "-".
func (e *Engine) Generate(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_generate", input)
}

// Fill fills the empty cells of a word search with random letters. Input: grid, and optionally lang, accents, seed, words
// (so the filler never makes an extra copy of one). Without a seed every call differs.
func (e *Engine) Fill(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_fill", input)
}

// Stylize writes text in Unicode styles that keep accents. Input: text, and optionally style (one id or a list) and classic.
func (e *Engine) Stylize(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_stylize", input)
}

// Styles lists the available text styles: {"styles": [{"id", "label", "category"}]}.
func (e *Engine) Styles(ctx context.Context) (Result, error) {
	return e.call(ctx, "lettras_styles", "")
}

// Crossword lays out a crossword. Input: entries ([{"word", "clue"}]), rows, cols, and optionally seed. Returns puzzle,
// solution, placements and unplaced.
func (e *Engine) Crossword(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_crossword", input)
}

// Sudoku makes a sudoku with exactly one solution. Input (all optional): size (4, 6, 9, 12, 16), difficulty (1 to 4), seed.
// 0 is an empty cell.
func (e *Engine) Sudoku(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_sudoku", input)
}

// SolveSudoku solves a sudoku. Input: {"puzzle": [[...]]}. Returns solution, solved and unique.
func (e *Engine) SolveSudoku(ctx context.Context, input any) (Result, error) {
	return e.call(ctx, "lettras_sudoku_solve", input)
}

func (e *Engine) call(ctx context.Context, function string, input any) (Result, error) {
	payload, err := json.Marshal(input)
	if err != nil {
		return nil, &InputError{Message: "input is not valid JSON: " + err.Error()}
	}
	ctx, cancel := context.WithTimeout(ctx, Timeout)
	defer cancel()
	text, engineErr, err := e.run(ctx, function, payload)
	if err != nil {
		if errors.Is(ctx.Err(), context.DeadlineExceeded) {
			return nil, errors.New("puzzle took too long to generate")
		}
		return nil, err
	}
	if engineErr {
		return nil, &InputError{Message: string(text)}
	}
	var out Result
	if err := json.Unmarshal(text, &out); err != nil {
		return nil, fmt.Errorf("engine returned invalid JSON: %w", err)
	}
	return out, nil
}

// run makes one call on a fresh instance. engineErr is true when the engine answered with an error message.
func (e *Engine) run(ctx context.Context, function string, input []byte) (out []byte, engineErr bool, err error) {
	mod, err := e.runtime.InstantiateModule(ctx, e.module, wazero.NewModuleConfig().WithName(""))
	if err != nil {
		return nil, false, fmt.Errorf("engine failed to start: %w", err)
	}
	defer mod.Close(ctx)

	fn := func(name string) (api.Function, error) {
		f := mod.ExportedFunction(name)
		if f == nil {
			return nil, fmt.Errorf("engine has no %s export", name)
		}
		return f, nil
	}
	alloc, err := fn("lettras_alloc")
	if err != nil {
		return nil, false, err
	}
	free, err := fn("lettras_free")
	if err != nil {
		return nil, false, err
	}
	entry, err := fn(function)
	if err != nil {
		return nil, false, err
	}
	resultPtr, err := fn("lettras_result_ptr")
	if err != nil {
		return nil, false, err
	}

	size := uint64(len(input))
	res, err := alloc.Call(ctx, size)
	if err != nil {
		return nil, false, err
	}
	ptr := api.DecodeU32(res[0])
	if !mod.Memory().Write(ptr, input) {
		return nil, false, errors.New("engine memory write failed")
	}
	res, err = entry.Call(ctx, api.EncodeU32(ptr), size)
	if err != nil {
		return nil, false, err
	}
	n := int64(res[0])
	if _, err = free.Call(ctx, api.EncodeU32(ptr), size); err != nil {
		return nil, false, err
	}
	length := n
	if length < 0 {
		length = -length
	}
	pr, err := resultPtr.Call(ctx)
	if err != nil {
		return nil, false, err
	}
	buf, ok := mod.Memory().Read(api.DecodeU32(pr[0]), uint32(length))
	if !ok {
		return nil, false, errors.New("engine memory read failed")
	}
	return append([]byte(nil), buf...), n < 0, nil
}
