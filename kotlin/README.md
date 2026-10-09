# Lettras for Kotlin

Word-search generator for **Spanish, English, Portuguese, French, German and Italian**, with native accented letters
kept as one cell each. Kotlin library for the **JVM and Android**. It runs the same compiled engine as the npm package
and the MCP server, locally: no network, no API key, and the same seed gives the same puzzle on every platform.

```kotlin
val lettras = Lettras()   // create once and reuse; thread-safe

val puzzle = lettras.generate(
    PuzzleRequest(
        words = listOf("gato", "perro", "piña", "mono", "cebra"),
        rows = 9,
        cols = 12,
        position = Position.MIXED, // HORIZONTAL | VERTICAL | MIXED (all 8 directions)
        seed = 8,                  // same request and seed, same grid
    ),
)

println(puzzle.render())
```

```
9×12  seed 8  engine 0.1.0

- - - - - - - - - - - -
- - - - - - - - - - - -
- - - - C - - - - - - -
- - - - - E - O N O M -
- - A - - - B - R - - -
O - - Ñ - - - R - - - -
T - - - I - E - A - - -
A - - - - P - - - - - -
G - - - - - - - - - - -

Words: gato, perro, piña, mono, cebra
```

`piña` takes four cells (the `Ñ` is one). `puzzle.grid` is plain data: `List<List<String>>`, one letter per cell.

## Filling the empty cells

`generate` leaves empty cells as `-`. `fill` completes them with random letters, in a language, with accents on or off:

```kotlin
val puzzle = lettras.generate(PuzzleRequest(listOf("gato", "perro", "piña"), rows = 9, cols = 12, position = Position.MIXED, seed = 8))
val done = lettras.fill(puzzle, lang = Language.ES, accents = true)   // protects the puzzle's words from accidental copies
done.grid                                  // List<List<String>>, no "-" left
val filledPuzzle = puzzle.copy(grid = done.grid)
lettras.fill(puzzle.grid, Language.DE, accents = false, seed = 5)   // a bare matrix, plain A-Z, repeatable
```

Letters follow how common they are in the language (`Language.EN` never has accents, `Language.DE` can include `ẞ`).
Without a `seed` every call gives different letters. `FillResult` has `grid`, `filled`, `seed` and `ambiguous`.

## Install

Maven Central coordinates (available once the first release is published; see
[`docs/MAVEN_CENTRAL.md`](../docs/MAVEN_CENTRAL.md)):

```kotlin
dependencies {
    implementation("org.lettras:lettras:0.2.0")
}
```

Until then, build it locally and use it from `mavenLocal()`:

```bash
cd kotlin
./gradlew publishToMavenLocal      # then add mavenLocal() to your repositories
```

Requires JDK 17 to build. The library itself targets Java 11, so it runs on Android (minSdk 26 or higher recommended)
and any JVM from 11.

## API

```kotlin
class Lettras {
    fun generate(request: PuzzleRequest): Puzzle
    fun generate(words: List<String>, rows: Int, cols: Int = rows, position: Position? = null, seed: Long? = null, lang: Language? = null): Puzzle
    val engineVersion: String
    val isCompiled: Boolean   // true = bytecode compiler (JVM), false = interpreter (Android)
}
```

| `PuzzleRequest` field | Meaning |
| --- | --- |
| `words`, `rows`, `cols` | Words to hide and grid size. `cols` may differ from `rows`. |
| `position` | `HORIZONTAL` (left→right), `VERTICAL` (top→bottom), `MIXED` (all 8 directions). Overrides `difficulty`. |
| `difficulty` | 1 (right, down) to 4 (all 8 directions), used when `position` is null. |
| `clustering` | 0.0 words apart · 1.0 words crossing · engine default 0.5. |
| `seed` | Repeatable puzzles. |
| `lang` | `Language.ES` (default), `EN`, `PT`, `FR`, `DE`, `IT`. |
| `classicMode` | Strip accents in the grid (`ñ` becomes `N`). |
| `fill` | Character for empty cells, default `"-"`. |

Options you leave null use the engine's defaults.

**`Puzzle`**

| Field | Meaning |
| --- | --- |
| `grid` | `rows` lists of `cols` one-letter strings. |
| `placements` | Where each word is hidden: start (`r`, `c`), step (`dr`, `dc`), `length`. |
| `words` | Words that were placed, in input order. |
| `unplaced` | Words that did not fit. Never dropped silently: show them or retry with a bigger grid. |
| `rejected` | Words refused, with a reason (spaces, digits, duplicates, contained in another word). |
| `seed`, `engineVersion` | To reproduce a stored puzzle later. |

`puzzle.render()` gives the text view above; `puzzle.solution()` shows only the hidden words.
Invalid requests (for example zero rows) throw `LettrasException` with the reason.

## Performance and Android

The engine is WebAssembly, run by [Chicory](https://github.com/dylibso/chicory), a WebAssembly runtime written in
pure Java. There are no native libraries, so there is nothing to build per CPU architecture.

| Where | How it runs | Measured (desktop JVM) |
| --- | --- | --- |
| JVM | Compiled to bytecode on first use | 30×30 grid with 60 words in about 3 s; a typical 10×10 puzzle in milliseconds |
| Android | Interpreter (ART cannot load generated JVM bytecode) | A typical 10×10 puzzle with 8 words in about 0.35 s on a desktop JVM; expect a few times slower on a phone |

The first call on each thread pays a one-time setup cost. Create one `Lettras` and reuse it. On Android, call it off
the main thread.

For the heaviest use (huge batches, real-time generation in a game loop), a native build of the engine per platform is
the next step; the API above stays the same.

## Tests

```bash
cd kotlin && ./gradlew test      # 24 tests
```

Besides behaviour tests (positions, German `ẞ`, classic mode, rejected and unplaced words, thread safety, a 30×30
puzzle), the suite checks that the library returns **exactly** the grids the npm package returns for the same input,
using the fixtures shared with the MCP server (`mcp/tests/fixtures/parity.json`). It also runs the interpreter path that
Android uses and checks it against the same fixture.

## License

Library code: MIT. The bundled engine (`src/main/resources/org/lettras/lettras_engine.wasm`) is proprietary; see
`LICENSE-ENGINE` next to it. You may use it unmodified, through this library, in your own products, including commercial
ones; you may not extract, redistribute, modify or reverse-engineer it.
