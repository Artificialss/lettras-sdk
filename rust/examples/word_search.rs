use lettras::{json, Lettras};

fn main() -> Result<(), lettras::Error> {
    let engine = Lettras::new()?;
    let puzzle = engine.generate(&json!({ "words": ["gato", "perro", "piña"], "rows": 9, "cols": 12, "position": "mixed", "seed": 8 }))?;
    let done = engine.fill(&json!({ "grid": puzzle["grid"], "lang": "es", "accents": true, "words": ["gato", "perro", "piña"], "seed": 5 }))?;
    for row in done["grid"].as_array().into_iter().flatten() {
        let cells: Vec<&str> = row.as_array().into_iter().flatten().filter_map(|c| c.as_str()).collect();
        println!("{}", cells.join(" "));
    }
    Ok(())
}
