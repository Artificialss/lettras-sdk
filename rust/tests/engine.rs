use lettras::{json, Error, Lettras};

#[test]
fn word_search_is_deterministic_and_keeps_accents() {
    let e = Lettras::new().unwrap();
    let input = json!({ "words": ["gato", "piña", "mono"], "rows": 8, "cols": 8, "seed": 3 });
    let a = e.generate(&input).unwrap();
    assert_eq!(a, e.generate(&input).unwrap());
    let cells: Vec<&str> = a["grid"].as_array().unwrap().iter().flat_map(|r| r.as_array().unwrap()).filter_map(|c| c.as_str()).collect();
    assert!(cells.contains(&"Ñ"), "one cell per Ñ");
}

#[test]
fn fill_replaces_empty_cells() {
    let e = Lettras::new().unwrap();
    let p = e.generate(&json!({ "words": ["sol", "luna"], "rows": 6, "cols": 6, "seed": 1 })).unwrap();
    let done = e.fill(&json!({ "grid": p["grid"], "lang": "es", "seed": 2 })).unwrap();
    assert!(done["grid"].as_array().unwrap().iter().flat_map(|r| r.as_array().unwrap()).all(|c| c != "-"));
}

#[test]
fn sudoku_solves_to_a_unique_answer() {
    let e = Lettras::new().unwrap();
    let s = e.sudoku(&json!({ "size": 9, "difficulty": 2, "seed": 4 })).unwrap();
    let solved = e.solve_sudoku(&json!({ "puzzle": s["puzzle"] })).unwrap();
    assert_eq!(solved["unique"], true);
}

#[test]
fn crossword_places_words() {
    let e = Lettras::new().unwrap();
    let c = e
        .crossword(&json!({ "entries": [{"word":"gato","clue":"Felino"},{"word":"toro","clue":"Bovino"},{"word":"ropa","clue":"Se viste"}], "rows": 8, "cols": 8, "seed": 1 }))
        .unwrap();
    assert!(!c["placements"].as_array().unwrap().is_empty());
}

#[test]
fn styles_and_stylize_work() {
    let e = Lettras::new().unwrap();
    assert!(!e.styles().unwrap()["styles"].as_array().unwrap().is_empty());
    assert!(e.stylize(&json!({ "text": "Ñandú", "style": "bold" })).is_ok());
}

#[test]
fn bad_input_is_an_input_error() {
    let e = Lettras::new().unwrap();
    assert!(matches!(e.generate(&json!({ "words": ["gato"], "rows": 0, "cols": 5 })), Err(Error::Input(_))));
}
