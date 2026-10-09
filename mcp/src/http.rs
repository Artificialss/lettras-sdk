use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::backend::PuzzleBackend;
use crate::limits::Gate;
use crate::protocol::handle_message;

#[derive(Clone)]
pub struct AppState {
    pub backend: Arc<dyn PuzzleBackend>,
    pub gate: Arc<Gate>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/mcp", post(rpc).get(no_stream).options(preflight))
        .route("/", get(index))
        .with_state(state)
}

fn cors(mut res: Response) -> Response {
    let h = res.headers_mut();
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    h.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "content-type, accept, mcp-protocol-version, mcp-session-id".parse().unwrap());
    h.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS".parse().unwrap());
    res
}

async fn preflight() -> Response {
    cors(StatusCode::NO_CONTENT.into_response())
}

/// This server does not push events, so the optional GET stream is not offered.
async fn no_stream() -> Response {
    cors(StatusCode::METHOD_NOT_ALLOWED.into_response())
}

/// Caller address as set by the platform proxy: `x-real-ip` when present, else the first hop of `x-forwarded-for`.
/// (Vercel overwrites both, so a client cannot choose its own.)
fn client_ip(headers: &HeaderMap) -> String {
    let first = |name: &str| headers.get(name).and_then(|v| v.to_str().ok()).map(|v| v.split(',').next().unwrap_or("").trim().to_string());
    first("x-real-ip").or_else(|| first("x-forwarded-for")).filter(|v| !v.is_empty()).unwrap_or_else(|| "unknown".into())
}

async fn index() -> Json<Value> {
    Json(json!({ "name": "lettras-mcp", "endpoint": "/mcp", "transport": "streamable-http", "docs": "https://github.com/Artificialss/lettras-sdk" }))
}

async fn rpc(State(state): State<AppState>, headers: HeaderMap, body: String) -> Response {
    let client = client_ip(&headers);
    let Ok(parsed) = serde_json::from_str::<Value>(&body) else {
        let e = json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": "parse error" } });
        return cors((StatusCode::BAD_REQUEST, Json(e)).into_response());
    };

    // A batch is an array of messages; replies only for those with an id.
    if let Some(batch) = parsed.as_array() {
        if batch.is_empty() {
            let e = json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32600, "message": "empty batch" } });
            return cors((StatusCode::BAD_REQUEST, Json(e)).into_response());
        }
        let mut replies = Vec::new();
        for m in batch {
            if let Some(r) = handle_message(m, state.backend.as_ref(), &state.gate, &client).await {
                replies.push(r);
            }
        }
        return cors(if replies.is_empty() { StatusCode::ACCEPTED.into_response() } else { Json(replies).into_response() });
    }

    cors(match handle_message(&parsed, state.backend.as_ref(), &state.gate, &client).await {
        Some(reply) => Json(reply).into_response(),
        None => StatusCode::ACCEPTED.into_response(),
    })
}
