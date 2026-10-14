# lettras

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each in a grid). The engine runs in-process: no network, no API key.

```bash
cargo add lettras
```

[Source and docs](https://github.com/Artificialss/lettras-sdk) · [lettras.org](https://www.lettras.org) · same engine as the [npm package](https://www.npmjs.com/package/lettras) and the Kotlin library

## Word search

```rust
use lettras::{json, Lettras};

fn main() -> Result<(), lettras::Error> {
    let engine = Lettras::new()?;
    let puzzle = engine.generate(&json!({
        "words": ["gato", "perro", "piña"], "rows": 9, "cols": 12, "position": "mixed", "seed": 8
    }))?;
    // puzzle["grid"]: rows of one-letter cells, "-" where empty; puzzle["placements"], puzzle["unplaced"]

    let done = engine.fill(&json!({ "grid": puzzle["grid"], "lang": "es", "accents": true, "words": ["gato", "perro", "piña"] }))?;
    println!("{}", done["grid"]);
    Ok(())
}
```

Options: `position` (`horizontal`, `vertical`, `mixed`), `difficulty` 1 to 4, `clustering` 0 to 1, `seed` (same input and seed give the same grid), `lang`
(`es` default, `en` `pt` `fr` `de` `it`), `classicMode` (strip accents). `fill` completes the empty cells with letters that follow the language's frequency.

## Crossword and sudoku

```rust
let crossword = engine.crossword(&json!({
    "entries": [{ "word": "gato", "clue": "Felino doméstico" }, { "word": "toro", "clue": "Bovino" }],
    "rows": 10, "cols": 10, "seed": 1
}))?;                                            // puzzle, solution, placements, unplaced

let sudoku = engine.sudoku(&json!({ "size": 9, "difficulty": 2, "seed": 4 }))?;   // exactly one solution, 0 = empty
let solved = engine.solve_sudoku(&json!({ "puzzle": sudoku["puzzle"] }))?;        // solution, solved, unique
```

## Text styles

```rust
let styled = engine.stylize(&json!({ "text": "Ñandú", "style": "bold" }))?;
let all = engine.styles()?;   // the 22 styles: id, label, category
```

## How it works

The compiled engine is embedded in the crate as WebAssembly and run with [`wasmi`](https://crates.io/crates/wasmi). Inputs are anything that serialises to the engine's
JSON (`json!` or your own `Serialize` struct); outputs are `serde_json::Value`. Every call runs on a fresh instance with a CPU budget, so one call cannot affect the next
or run forever. Errors are `Error::Input` (the message says what to change) or `Error::Engine`.

## License

The wrapper is MIT. The compiled engine in `engine/` is proprietary: free to use unmodified, as distributed in this crate, in your own applications (commercial ones
included); no extraction, modification, reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `engine/LICENSE`.
