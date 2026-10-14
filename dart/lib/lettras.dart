/// Word-search, crossword and sudoku generators plus Unicode text styles for Spanish, English, Portuguese, French, German and
/// Italian, with native accented letters kept (one cell each).
///
/// The compiled Lettras engine is embedded and runs in-process (WebAssembly through `wasm_run`): no network, no API key at
/// run time. Inputs are JSON-compatible maps in the engine's camelCase shape; outputs are decoded JSON maps. The same input and
/// `seed` give the same puzzle. The engine is proprietary: see `ENGINE-LICENSE`.
///
/// ```dart
/// final lettras = Lettras();
/// final puzzle = lettras.generate({'words': ['gato', 'perro', 'piña'], 'rows': 9, 'cols': 12, 'seed': 8});
/// ```
///
/// In a pure Dart program, call `await WasmRunLibrary.setUp(override: false)` once (or add `wasm_run_flutter` to a Flutter app)
/// so the WebAssembly runtime can load.
library lettras;

export 'src/lettras_base.dart';
