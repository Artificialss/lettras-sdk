//! Minimal MCP over JSON-RPC 2.0: initialize, ping, tools/list, tools/call.
use serde_json::{json, Value};

use crate::backend::PuzzleBackend;
use crate::limits::Gate;
use crate::tools;

pub const SUPPORTED_VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

fn ok(id: &Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn err(id: &Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

/// Handles one JSON-RPC message. `None` means a notification: reply with `202 Accepted` and no body.
pub async fn handle_message(msg: &Value, backend: &dyn PuzzleBackend, gate: &Gate, client: &str) -> Option<Value> {
    let id = msg.get("id")?; // no id = notification
    if msg["jsonrpc"] != "2.0" {
        return Some(err(id, -32600, "invalid request: jsonrpc must be \"2.0\""));
    }
    let Some(method) = msg["method"].as_str() else {
        return Some(err(id, -32600, "invalid request: method is required"));
    };

    Some(match method {
        "initialize" => {
            let wanted = msg["params"]["protocolVersion"].as_str().unwrap_or("");
            let version = if SUPPORTED_VERSIONS.contains(&wanted) { wanted } else { SUPPORTED_VERSIONS[0] };
            ok(
                id,
                json!({
                    "protocolVersion": version,
                    "capabilities": { "tools": { "listChanged": false } },
                    "serverInfo": { "name": "lettras", "title": "Lettras word puzzles", "version": env!("CARGO_PKG_VERSION") },
                    "instructions": "Use generate_word_search to build word-search puzzles in es, en, pt, fr, de or it, then fill_word_search to complete the empty cells with random letters (accents on or off). Accents and letters like Ñ and ẞ are kept as single cells."
                }),
            )
        }
        "ping" => ok(id, json!({})),
        "tools/list" => ok(id, json!({ "tools": tools::list() })),
        "tools/call" => {
            let name = msg["params"]["name"].as_str().unwrap_or("");
            let args = msg["params"].get("arguments").cloned().unwrap_or_else(|| json!({}));
            match tools::call(name, &args, backend, gate, client).await {
                Some(result) => ok(id, result),
                None => err(id, -32602, &format!("unknown tool: {name}")),
            }
        }
        _ => err(id, -32601, &format!("method not found: {method}")),
    })
}
