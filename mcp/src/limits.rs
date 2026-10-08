//! Free-tier gate: a client may create a few puzzles, then gets a pointer to the Lettras page.
//!
//! Vercel functions are stateless, so the counter lives in a [`UsageStore`]. Production uses
//! [`PgStore`] (any Postgres, e.g. Neon); [`MemoryStore`] serves tests and local runs and is only
//! per-instance. Clients are identified by a hash of their IP; raw IPs are never stored.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

/// Increments a counter and returns its new value. The counter expires after `window_secs` (0 = never).
#[async_trait]
pub trait UsageStore: Send + Sync {
    async fn incr(&self, key: &str, window_secs: u64) -> Result<u64, String>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Usage {
    pub used: u64,
    pub limit: u64,
}

impl Usage {
    pub fn allowed(&self) -> bool {
        self.used <= self.limit
    }
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }
}

pub struct Gate {
    store: Arc<dyn UsageStore>,
    pub limit: u64,
    pub window_secs: u64,
    pub upgrade_url: String,
}

impl Gate {
    pub fn new(store: Arc<dyn UsageStore>, limit: u64, window_secs: u64, upgrade_url: impl Into<String>) -> Self {
        Self { store, limit, window_secs, upgrade_url: upgrade_url.into() }
    }

    /// Counts one puzzle for `client`. If the store is unreachable the request is allowed
    /// (a storage outage should not take the service down).
    pub async fn hit(&self, client: &str) -> Usage {
        let key = format!("lettras:mcp:{}", client_id(client));
        match self.store.incr(&key, self.window_secs).await {
            Ok(used) => Usage { used, limit: self.limit },
            Err(_) => Usage { used: 1, limit: self.limit },
        }
    }

    /// The message shown when the limit is reached.
    pub fn blocked_message(&self) -> String {
        format!(
            "Free limit reached: you have created {} puzzles. Visit {} to create more.",
            self.limit, self.upgrade_url
        )
    }
}

/// Stable, non-reversible id for an IP address.
pub fn client_id(ip: &str) -> String {
    let digest = Sha256::digest(ip.trim().as_bytes());
    format!("{digest:x}")[..24].to_string()
}

#[derive(Default)]
pub struct MemoryStore(Mutex<HashMap<String, (u64, Instant)>>);

#[async_trait]
impl UsageStore for MemoryStore {
    async fn incr(&self, key: &str, window_secs: u64) -> Result<u64, String> {
        let mut map = self.0.lock().map_err(|_| "poisoned".to_string())?;
        let now = Instant::now();
        let entry = map.entry(key.to_string()).or_insert((0, now));
        if window_secs > 0 && now.duration_since(entry.1) >= Duration::from_secs(window_secs) {
            *entry = (0, now);
        }
        entry.0 += 1;
        Ok(entry.0)
    }
}

/// Counters in Postgres (Neon works well: the pooled connection string suits serverless).
///
/// One row per client id: `key` (hash), `count`, `window_start`. The table is created on first use. A hit
/// inside the window adds one; the first hit after the window expires starts a new window at 1. All of it is a
/// single atomic upsert, so concurrent requests cannot lose counts.
pub struct PgStore {
    pool: PgPool,
    ready: OnceCell<()>,
}

const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS mcp_usage (
    key          text PRIMARY KEY,
    count        bigint NOT NULL,
    window_start timestamptz NOT NULL DEFAULT now()
)";

const HIT: &str = "INSERT INTO mcp_usage (key, count, window_start) VALUES ($1, 1, now())
ON CONFLICT (key) DO UPDATE SET
    count = CASE WHEN $2 > 0 AND mcp_usage.window_start < now() - make_interval(secs => $2)
                 THEN 1 ELSE mcp_usage.count + 1 END,
    window_start = CASE WHEN $2 > 0 AND mcp_usage.window_start < now() - make_interval(secs => $2)
                        THEN now() ELSE mcp_usage.window_start END
RETURNING count";

impl PgStore {
    /// Connects lazily, so a slow or sleeping database never blocks the function from starting.
    pub fn connect(database_url: &str) -> Result<Self, String> {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(database_url)
            .map_err(|_| "DATABASE_URL is not a valid Postgres connection string".to_string())?;
        Ok(Self { pool, ready: OnceCell::new() })
    }

    /// Reads `DATABASE_URL` (what the Neon integration sets). `None` if it is missing or empty.
    pub fn from_env() -> Option<Result<Self, String>> {
        std::env::var("DATABASE_URL").ok().filter(|v| !v.is_empty()).map(|url| Self::connect(&url))
    }

    /// Creates the table once per process. Several instances can start at the same moment, and Postgres
    /// can throw a duplicate-key error when `CREATE TABLE IF NOT EXISTS` runs concurrently, so the
    /// creation is serialised with a transaction-scoped advisory lock.
    async fn ensure_table(&self) -> Result<(), String> {
        self.ready
            .get_or_try_init(|| async {
                let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("SELECT pg_advisory_xact_lock(7461237)").execute(&mut *tx).await.map_err(|e| e.to_string())?;
                sqlx::query(CREATE_TABLE).execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())
            })
            .await
            .map(|_| ())
    }
}

#[async_trait]
impl UsageStore for PgStore {
    async fn incr(&self, key: &str, window_secs: u64) -> Result<u64, String> {
        self.ensure_table().await.map_err(|_| "usage store unavailable".to_string())?;
        let count: i64 = sqlx::query_scalar(HIT)
            .bind(key)
            .bind(window_secs as f64)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| "usage store unavailable".to_string())?;
        // Housekeeping: now and then drop counters whose window ended long ago.
        if count == 1 && key.len() % 7 == 0 {
            let _ = sqlx::query("DELETE FROM mcp_usage WHERE window_start < now() - interval '14 days'").execute(&self.pool).await;
        }
        u64::try_from(count).map_err(|_| "usage store sent a negative count".to_string())
    }
}
