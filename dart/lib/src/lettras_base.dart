import 'dart:convert';
import 'dart:typed_data';

import 'package:wasm_run/wasm_run.dart';

import 'engine_data.dart';

/// The engine rejected the input. The message says what to change.
class LettrasInputException implements Exception {
  LettrasInputException(this.message);
  final String message;
  @override
  String toString() => message;
}

/// The engine failed to load or run.
class LettrasEngineException implements Exception {
  LettrasEngineException(this.message);
  final String message;
  @override
  String toString() => message;
}

/// The engine, loaded once. Every call runs on a fresh instance, so no state survives between calls.
class Lettras {
  Lettras() : _module = _load();

  final WasmModule _module;

  static WasmModule _load() {
    try {
      return compileWasmModuleSync(base64.decode(engineBase64));
    } catch (e) {
      throw LettrasEngineException('engine failed to load: $e');
    }
  }

  /// A word search. Input: `words`, `rows`, `cols`, and optionally `position` (`horizontal`, `vertical`, `mixed`),
  /// `difficulty` (1 to 4), `clustering` (0 to 1), `seed`, `lang` (`es` `en` `pt` `fr` `de` `it`), `classicMode`.
  /// Empty cells are `"-"`.
  Map<String, dynamic> generate(Map<String, dynamic> input) => _call('lettras_generate', input);

  /// Fills the empty cells of a word search with random letters. Input: `grid`, and optionally `lang`, `accents`, `seed`,
  /// `words` (so the filler never makes an extra copy of one). Without a `seed` every call differs.
  Map<String, dynamic> fill(Map<String, dynamic> input) => _call('lettras_fill', input);

  /// Text in Unicode styles that keep accents. Input: `text`, and optionally `style` (one id or a list) and `classic`.
  Map<String, dynamic> stylize(Map<String, dynamic> input) => _call('lettras_stylize', input);

  /// The available text styles: `{styles: [{id, label, category}]}`.
  Map<String, dynamic> styles() => _call('lettras_styles', '');

  /// A crossword. Input: `entries` (`[{word, clue}]`), `rows`, `cols`, and optionally `seed`. Returns `puzzle`, `solution`,
  /// `placements` and `unplaced`.
  Map<String, dynamic> crossword(Map<String, dynamic> input) => _call('lettras_crossword', input);

  /// A sudoku with exactly one solution. Input (all optional): `size` (4, 6, 9, 12, 16), `difficulty` (1 to 4), `seed`.
  /// `0` is an empty cell.
  Map<String, dynamic> sudoku([Map<String, dynamic> input = const {}]) => _call('lettras_sudoku', input);

  /// Solves a sudoku. Input: `{puzzle: [[...]]}`. Returns `solution`, `solved` and `unique`.
  Map<String, dynamic> solveSudoku(Map<String, dynamic> input) => _call('lettras_sudoku_solve', input);

  Map<String, dynamic> _call(String function, Object input) {
    final String payload;
    try {
      payload = jsonEncode(input);
    } catch (e) {
      throw LettrasInputException('input is not valid JSON: $e');
    }
    final (text, isError) = _run(function, utf8.encode(payload));
    if (isError) throw LettrasInputException(text);
    final decoded = jsonDecode(text);
    if (decoded is! Map<String, dynamic>) throw LettrasEngineException('engine returned invalid JSON');
    return decoded;
  }

  /// One call on a fresh instance. The record is the engine's text and whether it is an error message.
  (String, bool) _run(String function, List<int> input) {
    try {
      final instance = _module.builder().buildSync();
      final memory = instance.getMemory('memory') ?? (throw LettrasEngineException('engine has no memory export'));
      Function fn(String name) =>
          (instance.getFunction(name) ?? (throw LettrasEngineException('engine has no $name export'))).inner;

      final ptr = _int(Function.apply(fn('lettras_alloc'), [input.length]));
      memory.view.setRange(ptr, ptr + input.length, input);
      final r = _int(Function.apply(fn(function), [ptr, input.length]));
      Function.apply(fn('lettras_free'), [ptr, input.length]);

      final length = r.abs();
      final outPtr = _int(Function.apply(fn('lettras_result_ptr'), []));
      final Uint8List bytes = Uint8List.fromList(memory.view.sublist(outPtr, outPtr + length));
      return (utf8.decode(bytes), r < 0);
    } on LettrasEngineException {
      rethrow;
    } catch (e) {
      throw LettrasEngineException('the engine failed: $e');
    }
  }

  /// The runtime returns `int` natively and `BigInt` in the browser for 64-bit values.
  static int _int(Object? v) => switch (v) {
        int i => i,
        BigInt b => b.toInt(),
        List l when l.isNotEmpty => _int(l.first),
        _ => throw LettrasEngineException('unexpected engine result: $v'),
      };
}
