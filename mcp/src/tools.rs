use serde_json::{json, Value};

use crate::backend::PuzzleBackend;
use crate::limits::{is_free, Gate, Usage, FREE_MAX_SIDE, FREE_MAX_WORDS};
use crate::render::{render_matrix, render_puzzle};

const LANGS: [&str; 6] = ["es", "en", "pt", "fr", "de", "it"];
const POSITIONS: [&str; 3] = ["horizontal", "vertical", "mixed"];
const FILL_ALLOWED: [&str; 6] = ["grid", "lang", "accents", "seed", "words", "empty"];
const STYLE_ALLOWED: [&str; 3] = ["text", "style", "classic"];
const ALLOWED: [&str; 9] = ["words", "rows", "cols", "position", "difficulty", "clustering", "seed", "lang", "classicMode"];

/// Tool definitions returned by `tools/list`.
pub fn list() -> Value {
    json!([
        {
            "name": "generate_word_search",
            "title": "Generate a word search",
            "description": "Create a word-search puzzle (sopa de letras) from a list of words. Puzzles up to 10x10 with at most 10 words are free and unlimited; bigger ones use the free puzzles each client gets. Keeps native letters (Ñ, Ç, Ã, Ä, ẞ, È…) as one cell each. Returns the grid, where every word is hidden, and any words that did not fit. Empty cells are marked \"-\": call fill_word_search with the grid to complete it with random letters. The same seed always gives the same puzzle.",
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
            "name": "fill_word_search",
            "title": "Fill a word search with random letters",
            "description": "Completes a word search: takes the grid from generate_word_search (empty cells are \"-\") and fills them with random letters in the chosen language, with accents on or off. Letters follow how common they are in that language. Pass the puzzle's words so the filler never creates an extra copy of one.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "grid": { "type": "array", "items": { "type": "array", "items": { "type": "string", "minLength": 1, "maxLength": 4 }, "minItems": 1, "maxItems": 30 }, "minItems": 1, "maxItems": 30, "description": "The grid from generate_word_search: rows of one-letter strings, \"-\" for empty cells." },
                    "lang": { "type": "string", "enum": LANGS, "description": "Language of the letters. Default es." },
                    "accents": { "type": "boolean", "description": "true (default): use the language's accented/native letters (Ñ, Ç, Ã, Ä, ẞ…). false: plain A-Z only." },
                    "words": { "type": "array", "items": { "type": "string", "minLength": 1, "maxLength": 30 }, "maxItems": 60, "description": "The hidden words, so the filler cannot create an extra copy of one." },
                    "seed": { "type": "integer", "minimum": 0, "description": "Repeatable filler. Omit for a different filler every time." },
                    "empty": { "type": "string", "maxLength": 4, "description": "What marks an empty cell. Default \"-\"." }
                },
                "required": ["grid"],
                "additionalProperties": false
            }
        },
        {
            "name": "stylize_text",
            "title": "Write text in Unicode styles",
            "description": "Turns text into Unicode styles for social media bios and posts: bold, italic, script, fraktur, monospace, circled, squared, fullwidth, small caps, strikethrough, underline and more (22 styles). Accents are kept: an accented letter becomes its styled base letter plus the combining accent (ñ, ç, ü, ß work). Not every style can draw every character: letters such as ß, Æ, Œ, Ø or non-Latin scripts stay plain, and accented letters rely on a combining mark that some fonts draw badly. Those characters are listed in notNative for each style, and parts with missing: true mark them, so you can warn the user or show an empty box. Set classic to strip accents first. Pass one style id, several, or none for all styles (see list_styles). Runs locally and has no usage limit.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text": { "type": "string", "minLength": 1, "maxLength": 5000, "description": "The text to style, in its normal spelling (accents kept)." },
                    "style": {
                        "oneOf": [
                            { "type": "string", "description": "One style id, for example bold." },
                            { "type": "array", "items": { "type": "string" }, "minItems": 1, "maxItems": 22, "description": "Several style ids." }
                        ],
                        "description": "Which style(s) to use. Omit for all 22. Ids come from list_styles."
                    },
                    "classic": { "type": "boolean", "description": "Strip accents first (ñ becomes n), for places that render combining accents badly." }
                },
                "required": ["text"],
                "additionalProperties": false
            }
        },
        {
            "name": "list_styles",
            "title": "List text styles",
            "description": "The Unicode text styles stylize_text offers, with the id to pass, an English name and a category (bold-italic, sans, script, gothic, mono, enclosed, decoration, other).",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
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

/// Checks `fill_word_search` arguments. Returns the first problem, in words a model can act on.
pub fn validate_fill(args: &Value) -> Result<(), String> {
    let obj = args.as_object().ok_or("arguments must be an object")?;
    let grid = obj.get("grid").and_then(Value::as_array).ok_or("grid: an array of rows is required")?;
    if grid.is_empty() || grid.len() > 30 {
        return Err("grid: 1 to 30 rows".into());
    }
    let mut width = None;
    for row in grid {
        let cells = row.as_array().ok_or("grid: every row must be an array of strings")?;
        if cells.is_empty() || cells.len() > 30 {
            return Err("grid: 1 to 30 columns".into());
        }
        if *width.get_or_insert(cells.len()) != cells.len() {
            return Err("grid: every row must have the same number of cells".into());
        }
        if !cells.iter().all(|c| c.as_str().is_some_and(|c| (1..=4).contains(&c.chars().count()))) {
            return Err("grid: every cell must be a short string (one letter, or \"-\" for empty)".into());
        }
    }
    if let Some(v) = obj.get("lang") {
        if !v.as_str().is_some_and(|l| LANGS.contains(&l)) {
            return Err("lang: es, en, pt, fr, de or it".into());
        }
    }
    if obj.get("accents").is_some_and(|v| !v.is_boolean()) {
        return Err("accents: true or false".into());
    }
    if obj.get("seed").is_some_and(|v| !v.as_u64().is_some_and(|n| n <= u64::from(u32::MAX))) {
        return Err("seed: an integer from 0 to 4294967295".into());
    }
    if let Some(v) = obj.get("words") {
        let ok = v.as_array().is_some_and(|w| w.len() <= 60 && w.iter().all(|w| w.as_str().is_some_and(|w| (1..=30).contains(&w.chars().count()))));
        if !ok {
            return Err("words: up to 60 strings of 1 to 30 characters".into());
        }
    }
    if obj.get("empty").is_some_and(|v| !v.as_str().is_some_and(|e| (1..=4).contains(&e.chars().count()))) {
        return Err("empty: a short string such as \"-\"".into());
    }
    if let Some(unknown) = obj.keys().find(|k| !FILL_ALLOWED.contains(&k.as_str())) {
        return Err(format!("unknown argument: {unknown}"));
    }
    Ok(())
}

/// Checks `stylize_text` arguments. Returns the first problem, in words a model can act on.
pub fn validate_stylize(args: &Value) -> Result<(), String> {
    let obj = args.as_object().ok_or("arguments must be an object")?;
    let text = obj.get("text").and_then(Value::as_str).ok_or("text: a string of 1 to 5000 characters is required")?;
    if !(1..=5000).contains(&text.chars().count()) {
        return Err("text: 1 to 5000 characters".into());
    }
    match obj.get("style") {
        None => {}
        Some(Value::String(s)) if !s.is_empty() => {}
        Some(Value::Array(a)) if (1..=22).contains(&a.len()) && a.iter().all(|s| s.as_str().is_some_and(|s| !s.is_empty())) => {}
        Some(_) => return Err("style: a style id, or an array of 1 to 22 style ids (see list_styles)".into()),
    }
    if obj.get("classic").is_some_and(|v| !v.is_boolean()) {
        return Err("classic: true or false".into());
    }
    if let Some(unknown) = obj.keys().find(|k| !STYLE_ALLOWED.contains(&k.as_str())) {
        return Err(format!("unknown argument: {unknown}"));
    }
    Ok(())
}

async fn stylize(args: &Value, backend: &dyn PuzzleBackend) -> Value {
    // No usage limit: it is instant, local and independent of puzzles.
    if let Err(e) = validate_stylize(args) {
        return error_result(e);
    }
    match backend.stylize(args.clone()).await {
        Ok(out) => {
            let results = out["results"].as_array().cloned().unwrap_or_default();
            let mut text = results.iter().map(|r| format!("{}: {}", r["id"].as_str().unwrap_or(""), r["text"].as_str().unwrap_or(""))).collect::<Vec<_>>().join("\n");
            let mut flagged: Vec<&str> = Vec::new();
            for r in &results {
                for c in r["notNative"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                    if !flagged.contains(&c) {
                        flagged.push(c);
                    }
                }
            }
            if !flagged.is_empty() {
                text.push_str(&format!(
                    "\n\nNote: {} cannot be drawn natively by some of these styles (a combining accent that fonts draw differently, or a letter left plain). See notNative and parts[].missing in the structured result; set classic to true to strip accents.",
                    flagged.join(", ")
                ));
            }
            text_result(text, Some(out))
        }
        Err(e) => error_result(e),
    }
}

async fn list_styles(backend: &dyn PuzzleBackend) -> Value {
    match backend.list_styles().await {
        Ok(out) => {
            let text = out["styles"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|s| format!("{} ({}, {})", s["id"].as_str().unwrap_or(""), s["label"].as_str().unwrap_or(""), s["category"].as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("\n");
            text_result(text, Some(out))
        }
        Err(e) => error_result(e),
    }
}

/// A random seed for callers who did not give one: the filler should differ on every call.
fn random_seed() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut h = RandomState::new().build_hasher();
    h.write_u64(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as u64));
    h.finish() % (u64::from(u32::MAX) + 1)
}

async fn fill(args: &Value, backend: &dyn PuzzleBackend, gate: &Gate, client: &str) -> Value {
    // Not counted against the free puzzle limit (it only completes a puzzle that was already generated), but it has its own,
    // far more generous limit so it cannot be used without bound.
    if let Err(e) = validate_fill(args) {
        return error_result(e);
    }
    let usage = gate.hit_fill(client).await;
    if !usage.allowed() {
        let mut v = error_result(gate.fill_blocked_message());
        v["structuredContent"] = json!({ "limitReached": true, "limit": usage.limit, "visit": gate.upgrade_url });
        return v;
    }
    let mut request = args.clone();
    if request.get("seed").is_none() {
        request["seed"] = json!(random_seed());
    }
    match backend.fill(request).await {
        Ok(out) => {
            let n = out["filled"].as_u64().unwrap_or(0);
            let mut text = format!("{}\n\nFilled {n} empty cells.", render_matrix(&out));
            if out["ambiguous"].as_array().is_some_and(|a| !a.is_empty()) {
                text.push_str(" Note: some words appear more than once by accident; call again for a different filler.");
            }
            text_result(text, Some(out))
        }
        Err(e) => error_result(e),
    }
}

/// Runs a tool. Tool failures are results with `isError: true` (so the model can react), not protocol errors.
/// `client` identifies the caller (IP) for the free-tier limit.
pub async fn call(name: &str, args: &Value, backend: &dyn PuzzleBackend, gate: &Gate, client: &str) -> Option<Value> {
    match name {
        "generate_word_search" => Some(generate(args, backend, gate, client).await),
        "fill_word_search" => Some(fill(args, backend, gate, client).await),
        "stylize_text" => Some(stylize(args, backend).await),
        "list_styles" => Some(list_styles(backend).await),
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
    // Small puzzles (up to 10x10 and 10 words) are free and never counted; bigger ones use the per-client free puzzles.
    let free = is_free(args);
    let usage = if free { Usage { used: 0, limit: gate.limit } } else { gate.hit(client).await };
    // One JSON line per request: Vercel Logs / Observability / log drains pick it up. No raw IPs, no words.
    println!(
        "{}",
        json!({
            "event": "generate_word_search",
            "client": &crate::limits::client_id(client)[..12],
            "free": free,
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
            if free {
                text.push_str(&format!("\n\nThis puzzle is free and does not count: up to {FREE_MAX_SIDE}x{FREE_MAX_SIDE} with {FREE_MAX_WORDS} words or fewer is always free."));
            } else {
                text.push_str(&format!("\n\nFree larger puzzles left: {} of {}.", usage.remaining(), usage.limit));
            }
            text_result(text, Some(puzzle))
        }
        Err(e) => error_result(e),
    }
}
