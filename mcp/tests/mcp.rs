//! End-to-end: JSON-RPC messages and the HTTP layer, running the real embedded engine.
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use lettras_mcp::http::{router, AppState};
use lettras_mcp::limits::client_id;
use lettras_mcp::{handle_message, Gate, LocalEngine, MemoryStore, UsageStore};
use serde_json::{json, Value};
use tower::ServiceExt;

fn engine() -> Arc<LocalEngine> {
    LocalEngine::new().expect("engine loads")
}

fn gate(limit: u64) -> Gate {
    Gate::new(Arc::new(MemoryStore::default()), limit, 0, "https://lettras.org/lib")
}

async fn rpc(msg: Value) -> Option<Value> {
    handle_message(&msg, &engine(), &gate(5), "203.0.113.9").await
}

fn call(args: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "generate_word_search", "arguments": args } })
}

#[tokio::test]
async fn initialize_negotiates_version_and_advertises_tools() {
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-03-26" } })).await.unwrap();
    assert_eq!(r["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(r["result"]["serverInfo"]["name"], "lettras");
    assert!(r["result"]["capabilities"]["tools"].is_object());
    // unknown versions fall back to the newest we support
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 2, "method": "initialize", "params": { "protocolVersion": "1999-01-01" } })).await.unwrap();
    assert_eq!(r["result"]["protocolVersion"], "2025-06-18");
}

#[tokio::test]
async fn notifications_get_no_reply_and_ping_works() {
    assert!(rpc(json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).await.is_none());
    assert_eq!(rpc(json!({ "jsonrpc": "2.0", "id": 7, "method": "ping" })).await.unwrap()["result"], json!({}));
}

#[tokio::test]
async fn lists_tools_with_schemas() {
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" })).await.unwrap();
    let tools = r["result"]["tools"].as_array().unwrap();
    let names: Vec<_> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["generate_word_search", "list_languages"]);
    assert_eq!(tools[0]["inputSchema"]["required"], json!(["words", "rows", "cols"]));
}

#[tokio::test]
async fn generates_a_puzzle_with_the_local_engine() {
    let r = rpc(call(json!({ "words": ["gato", "perro", "piña"], "rows": 9, "cols": 12, "position": "mixed", "seed": 8 }))).await.unwrap();
    let res = &r["result"];
    assert_eq!(res["isError"], false);
    assert_eq!(res["structuredContent"]["grid"].as_array().unwrap().len(), 9);
    assert_eq!(res["structuredContent"]["unplaced"], json!([]));
    let text = res["content"][0]["text"].as_str().unwrap();
    assert!(text.starts_with("9×12"), "{text}");
    assert!(text.contains("Words: gato, perro, piña"));
    assert!(text.contains('Ñ'));
}

#[tokio::test]
async fn matches_the_npm_package_exactly() {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/parity.json")).unwrap();
    assert!(fixtures.len() >= 4);
    let e = engine();
    for (i, f) in fixtures.iter().enumerate() {
        let got = lettras_mcp::PuzzleBackend::generate(&e, f["input"].clone()).await.expect("generate");
        assert_eq!(got, f["output"], "fixture {i} differs from the npm package");
    }
}

#[tokio::test]
async fn bad_arguments_become_tool_errors_the_model_can_read() {
    for (args, expect) in [
        (json!({ "words": [], "rows": 8, "cols": 8 }), "words"),
        (json!({ "words": ["sol"], "rows": 5, "cols": 8 }), "rows"),
        (json!({ "words": ["sol"], "rows": 8, "cols": 31 }), "cols"),
        (json!({ "words": ["sol"], "rows": 8, "cols": 8, "position": "diagonal" }), "position"),
        (json!({ "words": ["sol"], "rows": 8, "cols": 8, "lang": "xx" }), "lang"),
        (json!({ "words": ["sol"], "rows": 8, "cols": 8, "seed": -1 }), "seed"),
        (json!({ "words": ["sol"], "rows": 8.5, "cols": 8 }), "rows"),
        (json!({ "words": ["sol"], "rows": 8, "cols": 8, "surprise": 1 }), "unknown argument"),
        (json!({ "words": [1, 2], "rows": 8, "cols": 8 }), "words"),
    ] {
        let r = rpc(call(args.clone())).await.unwrap();
        assert_eq!(r["result"]["isError"], true, "{args}");
        assert!(r["result"]["content"][0]["text"].as_str().unwrap().contains(expect), "{args}");
    }
}

#[tokio::test]
async fn protocol_errors() {
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 1, "method": "nope" })).await.unwrap();
    assert_eq!(r["error"]["code"], -32601);
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "ghost" } })).await.unwrap();
    assert_eq!(r["error"]["code"], -32602);
    let r = rpc(json!({ "jsonrpc": "1.0", "id": 1, "method": "ping" })).await.unwrap();
    assert_eq!(r["error"]["code"], -32600);
}

#[tokio::test]
async fn lists_languages() {
    let r = rpc(json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_languages" } })).await.unwrap();
    assert!(r["result"]["content"][0]["text"].as_str().unwrap().contains("ẞ"));
}

#[tokio::test]
async fn largest_allowed_puzzle_finishes_within_the_cpu_budget() {
    let words: Vec<String> = (0..60).map(|i| format!("w{}{}{}", ["abc", "def", "ghi", "jkl", "mno", "pqr"][i / 10], ["x", "y", "z", "q", "u"][i % 5], "abcdefghij".chars().nth(i % 10).unwrap())).collect();
    let r = rpc(call(json!({ "words": words, "rows": 30, "cols": 30, "position": "mixed", "seed": 3 }))).await.unwrap();
    assert_eq!(r["result"]["isError"], false, "{}", r["result"]["content"][0]["text"]);
}

async fn post(app: axum::Router, body: &str) -> (StatusCode, String) {
    let res = app.oneshot(Request::post("/mcp").header("content-type", "application/json").body(Body::from(body.to_string())).unwrap()).await.unwrap();
    let status = res.status();
    (status, String::from_utf8(res.into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap())
}

fn app() -> axum::Router {
    router(AppState { backend: Arc::new(engine()), gate: Arc::new(gate(5)) })
}

#[tokio::test]
async fn http_layer() {
    let (s, body) = post(app(), r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#).await;
    assert_eq!((s, body.as_str()), (StatusCode::OK, r#"{"id":1,"jsonrpc":"2.0","result":{}}"#));

    let (s, body) = post(app(), r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#).await;
    assert_eq!((s, body.as_str()), (StatusCode::ACCEPTED, ""));

    let (s, _) = post(app(), "not json").await;
    assert_eq!(s, StatusCode::BAD_REQUEST);

    let (s, body) = post(app(), r#"[{"jsonrpc":"2.0","id":1,"method":"ping"},{"jsonrpc":"2.0","method":"notifications/initialized"},{"jsonrpc":"2.0","id":2,"method":"ping"}]"#).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(serde_json::from_str::<Value>(&body).unwrap().as_array().unwrap().len(), 2);

    let get = app().oneshot(Request::get("/mcp").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(get.status(), StatusCode::METHOD_NOT_ALLOWED);

    let opt = app().oneshot(Request::options("/mcp").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(opt.status(), StatusCode::NO_CONTENT);
    assert_eq!(opt.headers()["access-control-allow-origin"], "*");
}

fn small(seed: u64) -> Value {
    call(json!({ "words": ["sol", "luna"], "rows": 6, "cols": 6, "seed": seed }))
}

#[tokio::test]
async fn blocks_after_the_free_limit_and_points_to_the_page() {
    let (e, g) = (engine(), gate(5));
    for i in 1..=5u64 {
        let r = handle_message(&small(i), &e, &g, "198.51.100.1").await.unwrap();
        assert_eq!(r["result"]["isError"], false, "puzzle {i}");
        let text = r["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains(&format!("Free puzzles left: {} of 5.", 5 - i)), "{text}");
    }
    let r = handle_message(&small(6), &e, &g, "198.51.100.1").await.unwrap();
    assert_eq!(r["result"]["isError"], true);
    let text = r["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("Free limit reached") && text.contains("https://lettras.org/lib"), "{text}");
    assert_eq!(r["result"]["structuredContent"]["limitReached"], true);
    assert!(r["result"]["structuredContent"]["grid"].is_null(), "no puzzle once blocked");
    // still blocked on the next try
    let r = handle_message(&small(7), &e, &g, "198.51.100.1").await.unwrap();
    assert_eq!(r["result"]["isError"], true);
}

#[tokio::test]
async fn limits_are_per_client() {
    let (e, g) = (engine(), gate(1));
    assert_eq!(handle_message(&small(1), &e, &g, "198.51.100.1").await.unwrap()["result"]["isError"], false);
    assert_eq!(handle_message(&small(2), &e, &g, "198.51.100.1").await.unwrap()["result"]["isError"], true);
    assert_eq!(handle_message(&small(1), &e, &g, "198.51.100.2").await.unwrap()["result"]["isError"], false);
}

#[tokio::test]
async fn invalid_requests_and_list_languages_do_not_use_up_the_limit() {
    let (e, g) = (engine(), gate(1));
    for _ in 0..4 {
        let bad = call(json!({ "words": [], "rows": 8, "cols": 8 }));
        assert_eq!(handle_message(&bad, &e, &g, "198.51.100.1").await.unwrap()["result"]["isError"], true);
        let langs = json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "list_languages" } });
        assert_eq!(handle_message(&langs, &e, &g, "198.51.100.1").await.unwrap()["result"]["isError"], false);
    }
    assert_eq!(handle_message(&small(1), &e, &g, "198.51.100.1").await.unwrap()["result"]["isError"], false);
}

#[tokio::test]
async fn counter_resets_after_the_window() {
    let store = MemoryStore::default();
    assert_eq!(store.incr("k", 1).await.unwrap(), 1);
    assert_eq!(store.incr("k", 1).await.unwrap(), 2);
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    assert_eq!(store.incr("k", 1).await.unwrap(), 1, "window expired");
    assert_eq!(store.incr("never", 0).await.unwrap(), 1);
    assert_eq!(store.incr("never", 0).await.unwrap(), 2);
}

#[test]
fn client_ids_are_stable_and_hide_the_ip() {
    assert_eq!(client_id("203.0.113.9"), client_id(" 203.0.113.9 "));
    assert_ne!(client_id("203.0.113.9"), client_id("203.0.113.10"));
    assert!(!client_id("203.0.113.9").contains("203"));
    assert_eq!(client_id("x").len(), 24);
}

#[tokio::test]
async fn http_limit_follows_the_forwarded_address() {
    let app = router(AppState { backend: Arc::new(engine()), gate: Arc::new(gate(1)) });
    let body = small(1).to_string();
    let send = |ip: &'static str| {
        let app = app.clone();
        let body = body.clone();
        async move {
            let res = app.oneshot(Request::post("/mcp").header("content-type", "application/json").header("x-forwarded-for", ip).body(Body::from(body)).unwrap()).await.unwrap();
            serde_json::from_slice::<Value>(&res.into_body().collect().await.unwrap().to_bytes()).unwrap()
        }
    };
    assert_eq!(send("198.51.100.1, 10.0.0.1").await["result"]["isError"], false);
    assert_eq!(send("198.51.100.1, 10.0.0.2").await["result"]["isError"], true, "same first hop = same client");
    assert_eq!(send("198.51.100.2").await["result"]["isError"], false);
}
