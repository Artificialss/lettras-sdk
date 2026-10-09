//! Postgres-backed counter. Needs a real database, so it only runs on request:
//!   TEST_DATABASE_URL=postgres://... cargo test --release --test pg -- --ignored
//! It creates and uses the `mcp_usage` table (keys are prefixed per test and removed afterwards).
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use lettras_mcp::{handle_message, Gate, LocalEngine, PgStore, UsageStore};
use serde_json::json;

fn store() -> Arc<PgStore> {
    let url = std::env::var("TEST_DATABASE_URL").expect("set TEST_DATABASE_URL");
    Arc::new(PgStore::connect(&url).expect("valid url"))
}

fn key(name: &str) -> String {
    format!("test:{name}:{}", std::process::id())
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn counts_per_key_and_creates_the_table() {
    let s = store();
    let (a, b) = (key("a"), key("b"));
    assert_eq!(s.incr(&a, 0).await.unwrap(), 1);
    assert_eq!(s.incr(&a, 0).await.unwrap(), 2);
    assert_eq!(s.incr(&a, 0).await.unwrap(), 3);
    assert_eq!(s.incr(&b, 0).await.unwrap(), 1, "keys are independent");
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn window_resets_after_it_expires_and_zero_never_resets() {
    let s = store();
    let (w, n) = (key("window"), key("never"));
    assert_eq!(s.incr(&w, 1).await.unwrap(), 1);
    assert_eq!(s.incr(&w, 1).await.unwrap(), 2);
    tokio::time::sleep(Duration::from_millis(1300)).await;
    assert_eq!(s.incr(&w, 1).await.unwrap(), 1, "new window starts at 1");
    assert_eq!(s.incr(&w, 1).await.unwrap(), 2);

    assert_eq!(s.incr(&n, 0).await.unwrap(), 1);
    tokio::time::sleep(Duration::from_millis(1300)).await;
    assert_eq!(s.incr(&n, 0).await.unwrap(), 2, "window 0 = never resets");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore = "needs TEST_DATABASE_URL"]
async fn concurrent_requests_never_lose_a_count() {
    let s = store();
    let k = key("concurrent");
    let tasks: Vec<_> = (0..60)
        .map(|_| {
            let (s, k) = (s.clone(), k.clone());
            tokio::spawn(async move { s.incr(&k, 0).await.unwrap() })
        })
        .collect();
    let mut seen = HashSet::new();
    for t in tasks {
        seen.insert(t.await.unwrap());
    }
    assert_eq!(seen.len(), 60, "every request got its own number");
    assert_eq!(*seen.iter().max().unwrap(), 60);
    assert_eq!(s.incr(&k, 0).await.unwrap(), 61);
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn the_free_limit_holds_through_postgres() {
    let gate = Gate::new(store(), 5, 0, "https://lettras.org");
    let engine = LocalEngine::new().unwrap();
    let ip = format!("198.51.100.{}", std::process::id() % 250);
    let call = json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "generate_word_search",
        "arguments": { "words": ["sol", "luna"], "rows": 6, "cols": 6, "seed": 1 } } });
    for i in 1..=5 {
        let r = handle_message(&call, &engine, &gate, &ip).await.unwrap();
        assert_eq!(r["result"]["isError"], false, "puzzle {i}");
    }
    let r = handle_message(&call, &engine, &gate, &ip).await.unwrap();
    assert_eq!(r["result"]["isError"], true);
    assert!(r["result"]["content"][0]["text"].as_str().unwrap().contains("https://lettras.org"));
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn an_unreachable_database_lets_requests_through() {
    let broken = PgStore::connect("postgres://nobody:x@127.0.0.1:1/none").unwrap();
    let gate = Gate::new(Arc::new(broken), 5, 0, "https://lettras.org");
    let usage = gate.hit("203.0.113.7").await;
    assert!(usage.allowed(), "fail open: a storage outage must not take the service down");
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn stale_counters_are_cleaned_up_but_fresh_and_never_reset_ones_are_kept() {
    let url = std::env::var("TEST_DATABASE_URL").expect("set TEST_DATABASE_URL");
    let s = PgStore::connect(&url).unwrap();
    // create the table and two old rows through a direct connection
    let pool = sqlx_pool(&url).await;
    s.incr("test:warmup:0", 86_400).await.unwrap();
    for (k, age) in [("test:stale:old", "10 days"), ("test:stale:recent", "1 hour")] {
        sqlx_exec(&pool, &format!("INSERT INTO mcp_usage (key, count, window_start) VALUES ('{k}', 3, now() - interval '{age}') ON CONFLICT (key) DO UPDATE SET window_start = now() - interval '{age}'")).await;
    }
    // a key ending in 0 that starts a new window triggers the cleanup (window = 1 day => rows older than 3 days go)
    s.incr(&format!("test:cleanup:{}0", std::process::id()), 86_400).await.unwrap();
    assert!(!sqlx_exists(&pool, "test:stale:old").await, "10-day-old counter should be deleted");
    assert!(sqlx_exists(&pool, "test:stale:recent").await, "recent counter must stay");
    // window 0 (never resets) must never delete anything
    sqlx_exec(&pool, "INSERT INTO mcp_usage (key, count, window_start) VALUES ('test:stale:forever', 3, now() - interval '400 days') ON CONFLICT (key) DO UPDATE SET window_start = now() - interval '400 days'").await;
    s.incr(&format!("test:cleanup0:{}0", std::process::id()), 0).await.unwrap();
    assert!(sqlx_exists(&pool, "test:stale:forever").await, "window 0 counters are permanent");
    sqlx_exec(&pool, "DELETE FROM mcp_usage WHERE key LIKE 'test:%'").await;
}

#[tokio::test]
#[ignore = "needs TEST_DATABASE_URL"]
async fn an_unreachable_store_reports_a_safe_category_not_connection_details() {
    let (user, pw, host) = ("nobody", "secret-pw", "db.internal.example");
    let broken = PgStore::connect(&format!("postgres://{user}:{pw}@{host}:1/none")).unwrap();
    let err = broken.incr("k", 0).await.unwrap_err();
    assert!(!err.contains(pw) && !err.contains(host) && !err.contains(user), "leaked: {err}");
    assert!(err.starts_with("table: ") || err.starts_with("query: "), "{err}");
}

async fn sqlx_pool(url: &str) -> sqlx::PgPool {
    sqlx::PgPool::connect(url).await.unwrap()
}
async fn sqlx_exec(pool: &sqlx::PgPool, sql: &str) {
    sqlx::query(sql).execute(pool).await.unwrap();
}
async fn sqlx_exists(pool: &sqlx::PgPool, key: &str) -> bool {
    sqlx::query_scalar::<_, i64>("SELECT count(*) FROM mcp_usage WHERE key = $1").bind(key).fetch_one(pool).await.unwrap() > 0
}
