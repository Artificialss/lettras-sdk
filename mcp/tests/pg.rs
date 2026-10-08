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
