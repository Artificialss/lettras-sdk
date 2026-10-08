use serde_json::Value;

/// Plain-text view of a puzzle JSON: the grid, then the word bank, then anything that went wrong.
pub fn render_puzzle(p: &Value) -> String {
    let grid: Vec<String> = p["grid"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|r| {
                    r.as_array()
                        .map(|cells| cells.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" "))
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();
    let rows = grid.len();
    let cols = p["grid"][0].as_array().map_or(0, Vec::len);
    let list = |key: &str| -> Vec<String> {
        p[key].as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_owned).collect()).unwrap_or_default()
    };

    let mut out = format!("{rows}×{cols}  seed {}\n\n{}\n\nWords: {}", p["seed"], grid.join("\n"), list("words").join(", "));
    let unplaced = list("unplaced");
    if !unplaced.is_empty() {
        out.push_str(&format!("\nNot placed (try a bigger grid): {}", unplaced.join(", ")));
    }
    if let Some(rej) = p["rejected"].as_array().filter(|r| !r.is_empty()) {
        let items: Vec<String> = rej
            .iter()
            .map(|r| format!("{} ({})", r["word"].as_str().unwrap_or("?"), r["reason"].as_str().unwrap_or("?")))
            .collect();
        out.push_str(&format!("\nRejected: {}", items.join("; ")));
    }
    out
}

/// Plain-text view of just a grid (a fill result): one row per line.
pub fn render_matrix(v: &Value) -> String {
    v["grid"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|r| r.as_array().map(|cells| cells.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" ")).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}
