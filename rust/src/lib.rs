//! Word-search, crossword and sudoku generators plus Unicode text styles for Spanish, English, Portuguese, French,
//! German and Italian, with native accented letters kept (one cell each).
//!
//! The compiled Lettras engine is embedded and runs in-process (WebAssembly through `wasmi`): no network, no API key.
//! Inputs are anything that serialises to the engine's camelCase JSON (a `serde_json::json!` value or your own
//! `Serialize` struct); outputs are `serde_json::Value`. The same input and `seed` give the same puzzle.
//!
//! ```
//! use lettras::{json, Lettras};
//!
//! let engine = Lettras::new()?;
//! let puzzle = engine.generate(&json!({ "words": ["gato", "perro", "piña"], "rows": 9, "cols": 12, "seed": 8 }))?;
//! assert_eq!(puzzle["grid"].as_array().map(Vec::len), Some(9));
//! # Ok::<(), lettras::Error>(())
//! ```
//!
//! The engine is proprietary: see `engine/LICENSE`.

use std::fmt;

use serde::Serialize;
pub use serde_json::{json, Value};
use wasmi::{Config, Engine, Linker, Module, Store};

/// The compiled Lettras engine (C ABI build). Proprietary: see `engine/LICENSE`.
static ENGINE_WASM: &[u8] = include_bytes!("../engine/lettras_engine.wasm");

/// CPU budget per call, in wasmi fuel units. Bounds worst-case inputs.
const FUEL: u64 = 20_000_000_000;

/// What can go wrong: the engine refused the input (its message explains why) or it could not run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The engine rejected the input. The text says what to change.
    Input(String),
    /// The engine failed to load or run.
    Engine(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Input(m) | Error::Engine(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for Error {}

/// The engine, loaded once. Cheap to share by reference; every call runs on a fresh instance, so no state survives
/// between calls.
pub struct Lettras {
    engine: Engine,
    module: Module,
}

impl Lettras {
    /// Loads the embedded engine.
    pub fn new() -> Result<Self, Error> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let module = Module::new(&engine, ENGINE_WASM).map_err(|e| Error::Engine(format!("engine failed to load: {e}")))?;
        Ok(Self { engine, module })
    }

    /// A word search. Input: `words`, `rows`, `cols`, and optionally `position` (`horizontal`, `vertical`, `mixed`),
    /// `difficulty` (1 to 4), `clustering` (0 to 1), `seed`, `lang` (`es` `en` `pt` `fr` `de` `it`), `classicMode`.
    /// Returns `grid`, `placements`, `unplaced` and more; empty cells are `-`.
    pub fn generate(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_generate", input)
    }

    /// Fills the empty cells of a word search with random letters. Input: `grid`, and optionally `lang`, `accents`,
    /// `seed`, `words` (so the filler never makes an extra copy of one). Without a `seed` every call differs.
    pub fn fill(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_fill", input)
    }

    /// Text in Unicode styles that keep accents. Input: `text`, and optionally `style` (one id or a list) and `classic`.
    /// Returns `results` and `engineVersion`.
    pub fn stylize(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_stylize", input)
    }

    /// The available text styles: `{ "styles": [{ "id", "label", "category" }] }`.
    pub fn styles(&self) -> Result<Value, Error> {
        self.call("lettras_styles", &"")
    }

    /// A crossword from words with clues. Input: `entries` (`[{ "word", "clue" }]`), `rows`, `cols`, and optionally
    /// `seed`. Returns `puzzle`, `solution`, `placements` and `unplaced`.
    pub fn crossword(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_crossword", input)
    }

    /// A sudoku with exactly one solution. Input (all optional): `size` (4, 6, 9, 12, 16), `difficulty` (1 to 4), `seed`.
    /// `0` is an empty cell.
    pub fn sudoku(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_sudoku", input)
    }

    /// Solves a sudoku. Input: `{ "puzzle": [[..]] }`. Returns `solution`, `solved` and `unique`.
    pub fn solve_sudoku(&self, input: &impl Serialize) -> Result<Value, Error> {
        self.call("lettras_sudoku_solve", input)
    }

    fn call(&self, function: &str, input: &impl Serialize) -> Result<Value, Error> {
        let json = serde_json::to_string(input).map_err(|e| Error::Input(format!("input is not valid JSON: {e}")))?;
        let text = self.run(function, &json)?.map_err(Error::Input)?;
        serde_json::from_str(&text).map_err(|e| Error::Engine(format!("engine returned invalid JSON: {e}")))
    }

    /// One call on a fresh instance. The outer `Result` is a failure to run, the inner one is the engine's answer.
    fn run(&self, function: &str, input: &str) -> Result<Result<String, String>, Error> {
        let fail = |e: &dyn fmt::Display| Error::Engine(e.to_string());
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(FUEL).map_err(|e| fail(&e))?;
        let instance = Linker::<()>::new(&self.engine)
            .instantiate_and_start(&mut store, &self.module)
            .map_err(|e| Error::Engine(format!("engine failed to start: {e}")))?;

        let memory = instance.get_memory(&store, "memory").ok_or_else(|| Error::Engine("engine has no memory export".into()))?;
        let alloc = instance.get_typed_func::<u32, u32>(&store, "lettras_alloc").map_err(|e| fail(&e))?;
        let free = instance.get_typed_func::<(u32, u32), ()>(&store, "lettras_free").map_err(|e| fail(&e))?;
        let entry = instance.get_typed_func::<(u32, u32), i64>(&store, function).map_err(|e| fail(&e))?;
        let result_ptr = instance.get_typed_func::<(), u32>(&store, "lettras_result_ptr").map_err(|e| fail(&e))?;

        let bytes = input.as_bytes();
        let len = u32::try_from(bytes.len()).map_err(|_| Error::Input("input too large".into()))?;
        let ptr = alloc.call(&mut store, len).map_err(|e| fail(&e))?;
        memory.write(&mut store, ptr as usize, bytes).map_err(|e| fail(&e))?;
        let r = entry.call(&mut store, (ptr, len)).map_err(|e| {
            if store.get_fuel().is_ok_and(|f| f == 0) { Error::Engine("puzzle took too long to generate".into()) } else { fail(&e) }
        })?;
        free.call(&mut store, (ptr, len)).map_err(|e| fail(&e))?;

        let out_len = r.unsigned_abs() as usize;
        let out_ptr = result_ptr.call(&mut store, ()).map_err(|e| fail(&e))?;
        let mut buf = vec![0u8; out_len];
        memory.read(&store, out_ptr as usize, &mut buf).map_err(|e| fail(&e))?;
        let text = String::from_utf8(buf).map_err(|_| Error::Engine("engine returned invalid UTF-8".into()))?;
        Ok(if r >= 0 { Ok(text) } else { Err(text) })
    }
}
