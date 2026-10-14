# Lettras for Dart and Flutter

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each). The engine is embedded and runs in your app: no network, no API key.

```bash
dart pub add lettras
flutter pub add lettras wasm_run_flutter   # Flutter apps: wasm_run_flutter provides the WebAssembly runtime
```

```dart
import 'package:lettras/lettras.dart';

final lettras = Lettras();

final puzzle = lettras.generate({'words': ['gato', 'perro', 'piña'], 'rows': 9, 'cols': 12, 'position': 'mixed', 'seed': 8});
final done = lettras.fill({'grid': puzzle['grid'], 'lang': 'es', 'accents': true});   // random letters in the empty cells

final sudoku = lettras.sudoku({'size': 9, 'difficulty': 2, 'seed': 4});               // exactly one solution, 0 = empty
final solved = lettras.solveSudoku({'puzzle': sudoku['puzzle']});                      // solution, solved, unique
final crossword = lettras.crossword({'entries': [{'word': 'gato', 'clue': 'Felino'}, {'word': 'toro', 'clue': 'Bovino'}], 'rows': 10, 'cols': 10});
final bold = lettras.stylize({'text': 'Ñandú', 'style': 'bold'});
final styles = lettras.styles();                                                       // the 22 styles
```

Inputs and results are JSON-compatible maps in the engine's camelCase shape. Options are the same as the [npm package](https://www.npmjs.com/package/lettras): `position`,
`difficulty`, `clustering`, `seed` (same input and seed give the same puzzle), `lang`, `classicMode`. Bad input throws `LettrasInputException` with a message that says
what to change.

## Running it

The engine is WebAssembly, run by [`wasm_run`](https://pub.dev/packages/wasm_run) (wasmi/wasmtime):

- **Flutter**: add `wasm_run_flutter`, which bundles the runtime for Flutter targets (see its page for the supported platforms). We test this package on the Dart VM (macOS); check your Flutter targets before shipping.
- **Pure Dart** (servers, command line): call `await WasmRunLibrary.setUp(override: false)` once. This downloads the runtime library for your platform the first time (`dart run wasm_run:setup`
  does the same ahead of time); after that nothing is downloaded, and puzzles never touch the network.

Each call runs on a fresh instance, so no state survives between calls. Debug builds of WebAssembly interpreters are slow; generate large puzzles in release builds.

## License

The wrapper is MIT. The compiled engine is proprietary: free to use unmodified, as distributed in this package, in your own applications (commercial ones included); no
extraction, modification, reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `ENGINE-LICENSE`.
