//! Vercel function behind `/mcp` (see `vercel.json`): one axum router adapted by `VercelLayer`.
//!
//! Optional environment variables (none are required to run):
//! - `LETTRAS_FREE_LIMIT`        puzzles per client per window (default 5)
//! - `LETTRAS_LIMIT_WINDOW_SECS` window length in seconds, 0 = never resets (default 86400)
//! - `LETTRAS_UPGRADE_URL`       page shown when the limit is reached (default https://lettras.org)
//! - `DATABASE_URL`              Postgres connection string (Neon: use the pooled one) for the shared counter.
//!   Without it the counter is per-instance memory and NOT reliable on serverless.
use std::sync::Arc;

use lettras_mcp::http::{router, AppState};
use lettras_mcp::{Gate, LocalEngine, MemoryStore, PgStore, UsageStore};
use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;
use vercel_runtime::{run, Error};

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let backend = LocalEngine::new().map_err(|e| Error::from(e.as_str()))?;

    let store: Arc<dyn UsageStore> = match PgStore::from_env() {
        Some(pg) => Arc::new(pg.map_err(|e| Error::from(e.as_str()))?),
        None => Arc::new(MemoryStore::default()),
    };
    let gate = Gate::new(
        store,
        env_u64("LETTRAS_FREE_LIMIT", 5),
        env_u64("LETTRAS_LIMIT_WINDOW_SECS", 86_400),
        std::env::var("LETTRAS_UPGRADE_URL").unwrap_or_else(|_| "https://lettras.org".into()),
    );

    let app = router(AppState { backend: Arc::new(backend), gate: Arc::new(gate) });
    let service = ServiceBuilder::new().layer(VercelLayer::new()).service(app);
    run(service).await
}
