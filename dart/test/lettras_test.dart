import 'package:lettras/lettras.dart';
import 'package:test/test.dart';
import 'package:wasm_run/wasm_run.dart';

void main() {
  late Lettras lettras;

  setUpAll(() async {
    await WasmRunLibrary.setUp(override: false);
    lettras = Lettras();
  });

  test('word search is deterministic and keeps accents', () {
    final input = {'words': ['gato', 'piña', 'mono'], 'rows': 8, 'cols': 8, 'seed': 3};
    final a = lettras.generate(input);
    expect(a['grid'], hasLength(8));
    expect(a['grid'], lettras.generate(input)['grid']);
    expect((a['grid'] as List).expand((r) => r as List), contains('Ñ'));
  });

  test('fill leaves no empty cells', () {
    final p = lettras.generate({'words': ['sol', 'luna'], 'rows': 6, 'cols': 6, 'seed': 1});
    final done = lettras.fill({'grid': p['grid'], 'lang': 'es', 'seed': 2});
    expect((done['grid'] as List).expand((r) => r as List), isNot(contains('-')));
  });

  test('sudoku, crossword and styles', () {
    final s = lettras.sudoku({'size': 9, 'difficulty': 2, 'seed': 4});
    expect(lettras.solveSudoku({'puzzle': s['puzzle']})['unique'], isTrue);
    final c = lettras.crossword({
      'entries': [
        {'word': 'gato', 'clue': 'Felino'},
        {'word': 'toro', 'clue': 'Bovino'},
        {'word': 'ropa', 'clue': 'Se viste'},
      ],
      'rows': 8,
      'cols': 8,
      'seed': 1,
    });
    expect(c['placements'], isNotEmpty);
    expect(lettras.styles()['styles'], isNotEmpty);
    expect(lettras.stylize({'text': 'Ñandú', 'style': 'bold'}), isNotNull);
  });

  test('bad input is an input exception', () {
    expect(() => lettras.generate({'words': ['gato'], 'rows': 0, 'cols': 5}), throwsA(isA<LettrasInputException>()));
  });
}
