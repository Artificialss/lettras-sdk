//! Free-tier gate: a client may create a few puzzles, then gets a pointer to the Lettras page.
//!
//! Vercel functions are stateless, so the counter lives in a [`UsageStore`]. Production uses
//! [`RedisRestStore`] (Upstash Redis over its REST API); [`MemoryStore`] serves tests and local runs
//! and is only per-instance. Clients are identified by a hash of their IP; raw IPs are never stored.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

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

/// Upstash Redis through its REST API (what the Vercel Marketplace integration provisions).
pub struct RedisRestStore {
    client: reqwest::Client,
    url: String,
    token: String,
}

impl RedisRestStore {
    pub fn new(url: impl Into<String>, token: impl Into<String>) -> Self {
        Self { client: reqwest::Client::new(), url: url.into().trim_end_matches('/').to_string(), token: token.into() }
    }

    /// Reads `UPSTASH_REDIS_REST_URL`/`_TOKEN`, or the `KV_REST_API_URL`/`_TOKEN` names the Marketplace sets.
    pub fn from_env() -> Option<Self> {
        let get = |a: &str, b: &str| std::env::var(a).or_else(|_| std::env::var(b)).ok().filter(|v| !v.is_empty());
        Some(Self::new(
            get("UPSTASH_REDIS_REST_URL", "KV_REST_API_URL")?,
            get("UPSTASH_REDIS_REST_TOKEN", "KV_REST_API_TOKEN")?,
        ))
    }
}

#[async_trait]
impl UsageStore for RedisRestStore {
    async fn incr(&self, key: &str, window_secs: u64) -> Result<u64, String> {
        let mut commands = vec![json!(["INCR", key])];
        if window_secs > 0 {
            // NX: only the first hit starts the window.
            commands.push(json!(["EXPIRE", key, window_secs.to_string(), "NX"]));
        }
        let res = self
            .client
            .post(format!("{}/pipeline", self.url))
            .bearer_auth(&self.token)
            .json(&commands)
            .send()
            .await
            .map_err(|_| "usage store unreachable".to_string())?;
        if !res.status().is_success() {
            return Err(format!("usage store returned {}", res.status()));
        }
        let body: Value = res.json().await.map_err(|_| "usage store sent invalid JSON".to_string())?;
        body[0]["result"].as_u64().ok_or_else(|| "usage store sent an unexpected reply".to_string())
    }
}
