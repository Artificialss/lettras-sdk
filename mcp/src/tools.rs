use serde_json::{json, Value};

use crate::backend::PuzzleBackend;
use crate::limits::Gate;
use crate::render::render_puzzle;

const LANGS: [&str; 6] = ["es", "en", "pt", "fr", "de", "it"];
const POSITIONS: [&str; 3] = ["horizontal", "vertical", "mixed"];
const ALLOWED: [&str; 9] = ["words", "rows", "cols", "position", "difficulty", "clustering", "seed", "lang", "classicMode"];

/// Tool definitions returned by `tools/list`.
pub fn list() -> Value {
    json!([
        {
            "name": "generate_word_search",
            "title": "Generate a word search",
            "description": "Create a word-search puzzle (sopa de letras) from a list of words. Keeps native letters (Ñ, Ç, Ã, Ä, ẞ, È…) as one cell each. Returns the grid, where every word is hidden, and any words that did not fit. The same seed always gives the same puzzle.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "words": { "type": "array", "items": { "type": "string", "minLength": 1, "maxLength": 30 }, "minItems": 1, "maxItems": 60, "description": "Words to hide, in their normal spelling (accents kept)." },
                    "rows": { "type": "integer", "minimum": 6, "maximum": 30, "description": "Grid height." },
                    "cols": { "type": "integer", "minimum": 6, "maximum": 30, "description": "Grid width. May differ from rows for a rectangular grid." },
                    "position": { "type": "string", "enum": POSITIONS, "description": "horizontal = left to right only, vertical = top to bottom only, mixed = all 8 directions including diagonals." },
                    "difficulty": { "type": "integer", "minimum": 1, "maximum": 4, "description": "Used when position is not set: 1 easy (right, down) to 4 expert (all 8 directions)." },
                    "clustering": { "type": "number", "minimum": 0, "maximum": 1, "description": "0 keeps words apart, 1 makes them cross. Default 0.5." },
                    "seed": { "type": "integer", "minimum": 0, "description": "Repeatable puzzles: same input and seed, same grid." },
                    "lang": { "type": "string", "enum": LANGS, "description": "Language of the words. Default es." },
                    "classicMode": { "type": "boolean", "description": "Strip accents in the grid (ñ becomes N)." }
                },
                "required": ["words", "rows", "cols"],
                "additionalProperties": false
            }
        },
        {
            "name": "list_languages",
            "title": "List supported languages",
            "description": "Languages Lettras supports, with the native letters each one adds to its grid alphabet.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        }
    ])
}

fn text_result(text: String, structured: Option<Value>) -> Value {
    let mut v = json!({ "content": [{ "type": "text", "text": text }], "isError": false });
    if let Some(s) = structured {
        v["structuredContent"] = s;
    }
    v
}

fn error_result(message: impl Into<String>) -> Value {
    json!({ "content": [{ "type": "text", "text": message.into() }], "isError": true })
}

fn int_in(v: &Value, min: i64, max: i64) -> bool {
    v.as_i64().is_some_and(|n| (min..=max).contains(&n)) && v.is_i64() | v.is_u64()
}

/// Checks tool arguments before they reach the engine. Returns the first problem, in words a model can act on.
pub fn validate(args: &Value) -> Result<(), String> {
    let obj = args.as_object().ok_or("arguments must be an object")?;
    let words = obj.get("words").and_then(Value::as_array).ok_or("words: an array of 1 to 60 strings is required")?;
    if words.is_empty() || words.len() > 60 {
        return Err("words: 1 to 60 strings".into());
    }
    if !words.iter().all(|w| w.as_str().is_some_and(|w| (1..=30).contains(&w.chars().count()))) {
        return Err("words: each word must be a string of 1 to 30 characters".into());
    }
    if !obj.get("rows").is_some_and(|v| int_in(v, 6, 30)) {
        return Err("rows: an integer from 6 to 30 is required".into());
    }
    if !obj.get("cols").is_some_and(|v| int_in(v, 6, 30)) {
        return Err("cols: an integer from 6 to 30 is required".into());
    }
    if let Some(v) = obj.get("position") {
        if !v.as_str().is_some_and(|p| POSITIONS.contains(&p)) {
            return Err("position: horizontal, vertical or mixed".into());
        }
    }
    if obj.get("difficulty").is_some_and(|v| !int_in(v, 1, 4)) {
        return Err("difficulty: an integer from 1 to 4".into());
    }
    if obj.get("clustering").is_some_and(|v| !v.as_f64().is_some_and(|n| (0.0..=1.0).contains(&n))) {
        return Err("clustering: a number from 0 to 1".into());
    }
    if obj.get("seed").is_some_and(|v| !v.as_u64().is_some_and(|n| n <= u64::from(u32::MAX))) {
        return Err("seed: an integer from 0 to 4294967295".into());
    }
    if let Some(v) = obj.get("lang") {
        if !v.as_str().is_some_and(|l| LANGS.contains(&l)) {
            return Err("lang: es, en, pt, fr, de or it".into());
        }
    }
    if obj.get("classicMode").is_some_and(|v| !v.is_boolean()) {
        return Err("classicMode: true or false".into());
    }
    if let Some(unknown) = obj.keys().find(|k| !ALLOWED.contains(&k.as_str())) {
        return Err(format!("unknown argument: {unknown}"));
    }
    Ok(())
}

/// Runs a tool. Tool failures are results with `isError: true` (so the model can react), not protocol errors.
/// `client` identifies the caller (IP) for the free-tier limit.
pub async fn call(name: &str, args: &Value, backend: &dyn PuzzleBackend, gate: &Gate, client: &str) -> Option<Value> {
    match name {
        "generate_word_search" => Some(generate(args, backend, gate, client).await),
        "list_languages" => Some(text_result(
            "es Español: Á É Í Ó Ú Ü Ñ\nen English: none\npt Português: Á Â Ã À É Ê Í Ó Ô Õ Ú Ç\nfr Français: À Â Æ Ç É È Ê Ë Î Ï Ô Œ Ù Û Ü Ÿ\nde Deutsch: Ä Ö Ü ẞ\nit Italiano: À È É Ì Ò Ù".into(),
            None,
        )),
        _ => None,
    }
}

async fn generate(args: &Value, backend: &dyn PuzzleBackend, gate: &Gate, client: &str) -> Value {
    // Invalid requests are free: they never reach the counter.
    if let Err(e) = validate(args) {
        return error_result(e);
    }
    let usage = gate.hit(client).await;
    // One JSON line per request: Vercel Logs / Observability / log drains pick it up. No raw IPs, no words.
    println!(
        "{}",
        json!({
            "event": "generate_word_search",
            "client": &crate::limits::client_id(client)[..12],
            "allowed": usage.allowed(),
            "used": usage.used,
            "limit": usage.limit,
            "words": args["words"].as_array().map_or(0, Vec::len),
            "rows": args["rows"],
            "cols": args["cols"],
            "position": args.get("position").cloned().unwrap_or(Value::Null),
            "lang": args.get("lang").cloned().unwrap_or(Value::Null),
        })
    );
    if !usage.allowed() {
        let mut v = error_result(gate.blocked_message());
        v["structuredContent"] = json!({ "limitReached": true, "limit": usage.limit, "visit": gate.upgrade_url });
        return v;
    }
    match backend.generate(args.clone()).await {
        Ok(puzzle) => {
            let mut text = render_puzzle(&puzzle);
            text.push_str(&format!("\n\nFree puzzles left: {} of {}.", usage.remaining(), usage.limit));
            text_result(text, Some(puzzle))
        }
        Err(e) => error_result(e),
    }
}
