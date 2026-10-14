import 'package:lettras/lettras.dart';
import 'package:wasm_run/wasm_run.dart';

Future<void> main() async {
  await WasmRunLibrary.setUp(override: false); // pure Dart only; Flutter apps add wasm_run_flutter instead
  final lettras = Lettras();
  final puzzle = lettras.generate({'words': ['gato', 'perro', 'piña'], 'rows': 9, 'cols': 12, 'position': 'mixed', 'seed': 8});
  final done = lettras.fill({'grid': puzzle['grid'], 'lang': 'es', 'accents': true, 'words': ['gato', 'perro', 'piña']});
  for (final row in done['grid'] as List) {
    print((row as List).join(' '));
  }
}
