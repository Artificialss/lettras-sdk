use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use wasmi::{Config, Engine, Linker, Module, Store};

/// The compiled Lettras engine (C ABI build). Proprietary: see `engine/LICENSE`.
static ENGINE_WASM: &[u8] = include_bytes!("../engine/lettras_engine.wasm");

/// CPU budget per call, in wasmi fuel units. Bounds worst-case inputs.
const FUEL: u64 = 20_000_000_000;

/// Where puzzles come from. Production uses [`LocalEngine`]; tests use a fake.
#[async_trait]
pub trait PuzzleBackend: Send + Sync {
    /// Takes the engine input (camelCase JSON), returns the puzzle JSON or a message for the model.
    async fn generate(&self, input: Value) -> Result<Value, String>;
}

/// Runs the embedded engine in-process (wasmi). No network, no credentials, no environment variables.
pub struct LocalEngine {
    engine: Engine,
    module: Module,
}

impl LocalEngine {
    pub fn new() -> Result<Arc<Self>, String> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let module = Module::new(&engine, ENGINE_WASM).map_err(|e| format!("engine failed to load: {e}"))?;
        Ok(Arc::new(Self { engine, module }))
    }

    /// One call on a fresh instance, so no state survives between requests.
    fn run(&self, input: &str) -> Result<Result<String, String>, String> {
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(FUEL).map_err(|e| e.to_string())?;
        let instance = Linker::<()>::new(&self.engine)
            .instantiate_and_start(&mut store, &self.module)
            .map_err(|e| format!("engine failed to start: {e}"))?;

        let memory = instance.get_memory(&store, "memory").ok_or("engine has no memory export")?;
        let alloc = instance.get_typed_func::<u32, u32>(&store, "lettras_alloc").map_err(|e| e.to_string())?;
        let free = instance.get_typed_func::<(u32, u32), ()>(&store, "lettras_free").map_err(|e| e.to_string())?;
        let generate = instance.get_typed_func::<(u32, u32), i64>(&store, "lettras_generate").map_err(|e| e.to_string())?;
        let result_ptr = instance.get_typed_func::<(), u32>(&store, "lettras_result_ptr").map_err(|e| e.to_string())?;

        let bytes = input.as_bytes();
        let len = u32::try_from(bytes.len()).map_err(|_| "input too large".to_string())?;
        let ptr = alloc.call(&mut store, len).map_err(|e| e.to_string())?;
        memory.write(&mut store, ptr as usize, bytes).map_err(|e| e.to_string())?;
        let r = generate.call(&mut store, (ptr, len)).map_err(|e| {
            if store.get_fuel().is_ok_and(|f| f == 0) { "puzzle took too long to generate".to_string() } else { e.to_string() }
        })?;
        free.call(&mut store, (ptr, len)).map_err(|e| e.to_string())?;

        let out_len = r.unsigned_abs() as usize;
        let out_ptr = result_ptr.call(&mut store, ()).map_err(|e| e.to_string())?;
        let mut buf = vec![0u8; out_len];
        memory.read(&store, out_ptr as usize, &mut buf).map_err(|e| e.to_string())?;
        let text = String::from_utf8(buf).map_err(|_| "engine returned invalid UTF-8".to_string())?;
        Ok(if r >= 0 { Ok(text) } else { Err(text) })
    }
}

#[async_trait]
impl PuzzleBackend for Arc<LocalEngine> {
    async fn generate(&self, input: Value) -> Result<Value, String> {
        let engine = Arc::clone(self);
        let json = input.to_string();
        tokio::task::spawn_blocking(move || engine.run(&json))
            .await
            .map_err(|_| "engine task failed".to_string())?
            .and_then(|r| r)
            .and_then(|text| serde_json::from_str(&text).map_err(|e| format!("engine returned invalid JSON: {e}")))
    }
}
