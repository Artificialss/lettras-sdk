<p align="center"><img alt="Lettras" src="https://raw.githubusercontent.com/Artificialss/lettras-sdk/main/assets/svg/lettras-logo.svg" width="260"></p>

# Lettras for Kotlin

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept. Kotlin library for the **JVM and Android**. It runs the same compiled engine as the npm package
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
9×12  seed 8  engine 0.5.0

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

Example: the same Spanish puzzle (`seed = 8`) filled with `seed = 5` and accents on gives, for the empty cells of the grid
below, exactly the letters shown in the npm package's example, because every SDK runs the same engine:

```kotlin
val puzzle = lettras.generate(PuzzleRequest(listOf("gato", "perro", "piña", "mono", "cebra"), rows = 8, cols = 10, position = Position.MIXED, seed = 8))
println(lettras.fill(puzzle, Language.ES, accents = true, seed = 5).render())
```
```
C E B R A R O G G J
D R A E E R C T O D
B Ó E E M Í M R A Ñ
T S C O N R R A E G
T R N E A E R O E M
I O O E P S N P A O
O C O O A P P A U A
I R E A A S A Ñ I P
```

## Install

Published on [Maven Central](https://central.sonatype.com/artifact/org.lettras.artificialss/lettras):

```kotlin
dependencies {
    implementation("org.lettras.artificialss:lettras:0.5.0")
}
```

Make sure `mavenCentral()` is in your repositories (Android projects have it by default). The code you import is in the
package `org.lettras`. The jar comes with sources and Dokka documentation, and every file is signed (key
`48A6307F56EF491A`, published on `keyserver.ubuntu.com`).

To work on the library itself: `cd kotlin && ./gradlew publishToMavenLocal` and add `mavenLocal()` to your repositories.
Building requires JDK 17; the library targets Java 11 and runs on any JVM from 11.

## API

```kotlin
class Lettras {
    fun generate(request: PuzzleRequest): Puzzle
    fun generate(words: List<String>, rows: Int, cols: Int = rows, position: Position? = null, seed: Long? = null, lang: Language? = null): Puzzle
    fun stylize(text: String, style: String? = null, classic: Boolean = false): StylizeResult
    fun stylize(text: String, styles: List<String>, classic: Boolean = false): StylizeResult
    fun stylize(request: StylizeRequest): StylizeResult
    fun crossword(request: CrosswordRequest): CrosswordResult
    fun crossword(entries: List<Pair<String, String>>, rows: Int, cols: Int = rows, seed: Long? = null): CrosswordResult
    fun sudoku(request: SudokuRequest = SudokuRequest()): SudokuResult
    fun sudoku(size: Int, difficulty: Int = 2, seed: Long? = null): SudokuResult
    fun solveSudoku(puzzle: List<List<Int>>): SudokuSolution
    fun unstyle(text: String): String
    fun listStyles(): List<StyleInfo>
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

## Unicode text styles

Write text in 22 Unicode styles (bold, italic, script, fraktur, circled, strikethrough...) that **keep accents**. It is the engine behind Social Media Styles at [lettras.org](https://lettras.org), and it runs locally.

### Use it

```kotlin
val lettras = Lettras()

val all = lettras.stylize("Hola piña")                                  // all 22 styles
val bold = lettras.stylize("piña", style = "bold").results.single()     // one style
val two = lettras.stylize("Lettras", styles = listOf("bold", "circled"))  // several

bold.text        // 𝐩𝐢𝐧̃𝐚   (ñ = styled n + combining tilde)
bold.notNative   // [ñ]
bold.parts       // pieces; missing = true marks what the style cannot draw
lettras.unstyle(bold.text)   // piña
lettras.listStyles()         // 22 StyleInfo(id, label, category) in display order
```

Each result is a `StyledText(id, label, category, text, notNative, parts)`. Unknown styles and text over 10,000 characters throw `LettrasException`.

### Example output

The snippets below show the npm calls; Kotlin has the same functions and result fields.

#### The name in every style

`stylize('Lettras')` (real output). The list is in display order, which is also the order of `listStyles()`.

| Style id | Name | Lettras |
| --- | --- | --- |
| `bold` | Bold | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 |
| `italic` | Italic | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 |
| `bold-italic` | Bold italic | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 |
| `sans` | Sans-serif | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 |
| `sans-bold` | Sans-serif bold | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 |
| `sans-italic` | Sans-serif italic | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 |
| `sans-bold-italic` | Sans-serif bold italic | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 |
| `script` | Script | ℒℯ𝓉𝓉𝓇𝒶𝓈 |
| `bold-script` | Bold script | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 |
| `fraktur` | Fraktur | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 |
| `bold-fraktur` | Bold fraktur | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 |
| `double-struck` | Double-struck | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 |
| `monospace` | Monospace | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 |
| `small-caps` | Small caps | ʟᴇᴛᴛʀᴀꜱ |
| `fullwidth` | Fullwidth | Ｌｅｔｔｒａｓ |
| `circled` | Circled | Ⓛⓔⓣⓣⓡⓐⓢ |
| `negative-circled` | Negative circled | 🅛🅔🅣🅣🅡🅐🅢 |
| `squared` | Squared | 🄻🄴🅃🅃🅁🄰🅂 |
| `negative-squared` | Negative squared | 🅻🅴🆃🆃🆁🅰🆂 |
| `parenthesized` | Parenthesized | 🄛⒠⒯⒯⒭⒜⒮ |
| `strikethrough` | Strikethrough | L̶e̶t̶t̶r̶a̶s̶ |
| `underline` | Underline | L̲e̲t̲t̲r̲a̲s̲ |

#### Examples

Every style applied to `Lettras ejemplo: piña` (real output of `stylize`). The accent in `piña` is the styled `n` plus a combining
tilde, so how it looks depends on the font; the decoration styles (strikethrough, underline) mark every character, accents included.

| Style id | Result |
| --- | --- |
| `bold` | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 𝐞𝐣𝐞𝐦𝐩𝐥𝐨: 𝐩𝐢𝐧̃𝐚 |
| `italic` | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 𝑒𝑗𝑒𝑚𝑝𝑙𝑜: 𝑝𝑖𝑛̃𝑎 |
| `bold-italic` | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 𝒆𝒋𝒆𝒎𝒑𝒍𝒐: 𝒑𝒊𝒏̃𝒂 |
| `sans` | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 𝖾𝗃𝖾𝗆𝗉𝗅𝗈: 𝗉𝗂𝗇̃𝖺 |
| `sans-bold` | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 𝗲𝗷𝗲𝗺𝗽𝗹𝗼: 𝗽𝗶𝗻̃𝗮 |
| `sans-italic` | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 𝘦𝘫𝘦𝘮𝘱𝘭𝘰: 𝘱𝘪𝘯̃𝘢 |
| `sans-bold-italic` | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 𝙚𝙟𝙚𝙢𝙥𝙡𝙤: 𝙥𝙞𝙣̃𝙖 |
| `script` | ℒℯ𝓉𝓉𝓇𝒶𝓈 ℯ𝒿ℯ𝓂𝓅𝓁ℴ: 𝓅𝒾𝓃̃𝒶 |
| `bold-script` | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 𝓮𝓳𝓮𝓶𝓹𝓵𝓸: 𝓹𝓲𝓷̃𝓪 |
| `fraktur` | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 𝔢𝔧𝔢𝔪𝔭𝔩𝔬: 𝔭𝔦𝔫̃𝔞 |
| `bold-fraktur` | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 𝖊𝖏𝖊𝖒𝖕𝖑𝖔: 𝖕𝖎𝖓̃𝖆 |
| `double-struck` | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 𝕖𝕛𝕖𝕞𝕡𝕝𝕠: 𝕡𝕚𝕟̃𝕒 |
| `monospace` | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 𝚎𝚓𝚎𝚖𝚙𝚕𝚘: 𝚙𝚒𝚗̃𝚊 |
| `small-caps` | ʟᴇᴛᴛʀᴀꜱ ᴇᴊᴇᴍᴘʟᴏ: ᴘɪɴ̃ᴀ |
| `fullwidth` | Ｌｅｔｔｒａｓ ｅｊｅｍｐｌｏ： ｐｉｎ̃ａ |
| `circled` | Ⓛⓔⓣⓣⓡⓐⓢ ⓔⓙⓔⓜⓟⓛⓞ: ⓟⓘⓝ̃ⓐ |
| `negative-circled` | 🅛🅔🅣🅣🅡🅐🅢 🅔🅙🅔🅜🅟🅛🅞: 🅟🅘🅝̃🅐 |
| `squared` | 🄻🄴🅃🅃🅁🄰🅂 🄴🄹🄴🄼🄿🄻🄾: 🄿🄸🄽̃🄰 |
| `negative-squared` | 🅻🅴🆃🆃🆁🅰🆂 🅴🅹🅴🅼🅿🅻🅾: 🅿🅸🅽̃🅰 |
| `parenthesized` | 🄛⒠⒯⒯⒭⒜⒮ ⒠⒥⒠⒨⒫⒧⒪: ⒫⒤⒩̃⒜ |
| `strikethrough` | L̶e̶t̶t̶r̶a̶s̶ e̶j̶e̶m̶p̶l̶o̶:̶ p̶i̶ñ̶a̶ |
| `underline` | L̲e̲t̲t̲r̲a̲s̲ e̲j̲e̲m̲p̲l̲o̲:̲ p̲i̲ñ̲a̲ |

**What a style cannot draw.** `ñ`, `ó`, `ü`... are flagged in `notNative` for every letter style, because the accent is a combining mark. `ß`, `Æ`, `Œ`, `Ø` and non-Latin scripts have no styled version and stay plain:

```js
stylize('Straße', { style: 'script' }).results[0].parts;
// [ { text: '𝒮𝓉𝓇𝒶', missing: false }, { text: 'ß', missing: true }, { text: 'ℯ', missing: false } ]
// show the missing piece as an empty box, or warn the user
```

**Classic mode** strips accents first, so nothing is flagged:

```js
stylize('piña', { style: 'bold', classic: true }).results[0];
// { id: 'bold', text: '𝐩𝐢𝐧𝐚', notNative: [], ... }
```

**One style, several styles, or all** (`results` is always an array):

```js
stylize('Hola', { style: 'bold' }).results.length;              // 1
stylize('Hola', { style: ['bold', 'script'] }).results.length;  // 2
stylize('Hola').results.length;                                 // 22
```

### How it works

1. Each letter and digit is swapped for its look-alike in the Unicode "Mathematical Alphanumeric Symbols" (and the circled, squared, fullwidth... blocks), so the result is plain text you can paste anywhere that accepts Unicode: bios, posts, names.
2. **Accents are kept.** An accented letter becomes its styled base letter plus the Unicode combining mark (`ñ` = styled `n` + combining tilde), so `piña` stays readable. How an accent looks depends on the font of the place you paste it.
3. **What a style cannot draw is reported, not hidden.** `notNative` lists the input characters the style cannot draw natively (`ß`, `Æ`, `Œ`, `Ø`, non-Latin scripts, and accents that only exist as a combining mark), and `parts` cuts the text into pieces where `missing: true` marks what to show as an empty box.
4. `classic: true` strips the accents first. `unstyle(text)` turns styled text back into plain text. Text is limited to 10,000 characters; unknown style ids throw.

## Crossword and sudoku

### Crossword (crucigrama)

#### Use it

```kotlin
val lettras = Lettras()

val c = lettras.crossword(
    listOf(
        "gato" to "Maúlla y caza ratones", "perro" to "El mejor amigo del hombre", "mono" to "Trepa por los árboles",
        "cebra" to "Rayas blancas y negras", "oso" to "Duerme en invierno", "rata" to "Roedor pequeño",
        "toro" to "Embiste al capote", "piña" to "Fruta tropical con corona", "ñu" to "Antílope africano",
    ),
    rows = 12, cols = 12, seed = 4,
)
c.puzzle        // List<List<String>>: "-", "1", "5/6", "0"
c.solution      // the same matrix with letters
c.placements    // CrosswordPlacement(number, word, clue, direction = "across" | "down", r, c, length)
println(c.clues())   // "Across", then "4. Embiste al capote (4)"..., then "Down", "1. Rayas blancas y negras (5)"...
```

#### Example output

The result is the same as in the npm package (`puzzle` and `solution`, 7 x 11):

```
-   -   -   -   -   1   -   2   -   -   -
-   -   -   -   -   0   -   0   -   -   3
-   -   -   -   -   0   -   4   0   0   0
-   -   -   5/6 0   0   0   0   -   -   0
-   -   -   0   -   0   -   -   7   0   0
-   -   -   8   0   -   -   -   -   -   -
9   0   0   0   -   -   -   -   -   -   -
```

```
- - - - - C - G - - -
- - - - - E - A - - M
- - - - - B - T O R O
- - - P E R R O - - N
- - - I - A - - O S O
- - - Ñ U - - - - - -
R A T A - - - - - - -
```

Options go in a `CrosswordRequest(entries, rows, cols, seed, attempts, classicMode, block, open)`; invalid input throws `LettrasException`.

#### How it works

1. **You give words with clues** and the biggest size the crossword may take (`rows` x `cols`). Offer more words than you need.
2. **The engine tries many layouts** (`attempts`, default 30). Each one starts with the longest word; every next word must cross a word
   already on the board, and words only touch where they cross, so every run of two or more letters is a real entry.
3. **The best layout wins**: most words placed, then most crossings, then the smallest box. The grid is cropped to what it uses, and words
   that did not fit come back in `unplaced`. The same input and `seed` always give the same crossword.
4. **Every word gets its own number**, 1 to N in reading order of its first cell. When an across word and a down word start in the same
   cell, that cell shows both numbers as **`across/down`** (`5/6` below: word 5 goes across, word 6 goes down).
5. **You get two matrices of the same size**: `puzzle` is what the player sees, `solution` has the answers. Show `puzzle` with an empty
   box where a `0` is, and check the player's letters against `solution`.

| Cell in `puzzle` | Meaning |
| --- | --- |
| `-` | Blank: no word goes through this cell (change it with `block`). |
| `1`, `2`, `3`... | A word starts here. Its number matches `placements[].number`. |
| `5/6` | An across word (5) and a down word (6) both start here. |
| `0` | A letter goes here, but no word starts here (change it with `open`). |

In `solution` every letter cell holds one grapheme (`Ñ` and `ẞ` are one cell) and blanks are `-`.

### Sudoku

#### Use it

```kotlin
val s = lettras.sudoku(size = 9, difficulty = 2, seed = 5)   // SudokuResult
s.puzzle     // List<List<Int>>, 0 = empty cell (32 clues here)
s.solution   // the same grid solved

val r = lettras.solveSudoku(s.puzzle)   // SudokuSolution(solution, solved, unique, engineVersion)
```

#### Example output

The grids are the same as the npm example (`seed = 5`):

`s.puzzle` (`·` is a `0`) and `s.solution`:

```
· 7 5 2 3 · 6 9 ·        8 7 5 2 3 4 6 9 1
· · · · · 7 4 2 3        1 6 9 8 5 7 4 2 3
· · · · · 1 5 · 7        3 2 4 9 6 1 5 8 7
· · · · · 3 2 5 ·        9 4 8 1 7 3 2 5 6
2 · 6 · 9 8 · 1 ·        2 3 6 5 9 8 7 1 4
· 1 · · · · · · ·        5 1 7 6 4 2 8 3 9
· · · · 1 · 9 · 8        7 5 2 3 1 6 9 4 8
4 · 3 · · · · · 2        4 9 3 7 8 5 1 6 2
6 8 · 4 · 9 · · ·        6 8 1 4 2 9 3 7 5
```

Solve a grid (yours or a generated one):

```js
solveSudoku(s.puzzle); // { solution: [...], solved: true, unique: true, engineVersion: '0.5.0' }
```

A small one for children (`size: 4`, `difficulty: 1`, `seed: 1`):

```json
{ "puzzle":   [[4,1,3,0],[2,3,0,4],[0,0,0,0],[0,4,0,3]],
  "solution": [[4,1,3,2],[2,3,1,4],[3,2,4,1],[1,4,2,3]], "size": 4, "boxRows": 2, "boxCols": 2, "givens": 8 }
```

Result fields: `puzzle`, `solution`, `size`, `boxRows`, `boxCols`, `givens`, `difficulty`, `seed`, `engineVersion`.
Invalid input (a size that does not exist, a number repeated in a row, column or box) throws with the reason.

#### How it works

1. **A full valid grid is built** with a randomized search (every row, column and box holds each number once), so each `seed` gives a different one.
2. **Cells are taken out one at a time**, in random order, and a removal is kept only if the puzzle **still has exactly one solution**.
   `difficulty` sets how many clues stay: 1 easy (most clues) to 4 expert (fewest).
3. **You get `puzzle`** (clues as numbers, `0` for the empty cells) **and `solution`** (the same grid solved). The same input and `seed` give the same sudoku.
4. **`solveSudoku`** solves any grid you give it and says whether the answer is the only one (`unique`).

| `size` | Boxes | Numbers |
| --- | --- | --- |
| 4 | 2 x 2 | 1 to 4 |
| 6 | 2 x 3 | 1 to 6 |
| 9 | 3 x 3 | 1 to 9 (the classic) |
| 12 | 3 x 4 | 1 to 12 |
| 16 | 4 x 4 | 1 to 16 |

Clues left on a 9 x 9 grid: about 41 / 32 / 28 / 24 for difficulty 1 / 2 / 3 / 4. The 12 x 12 and 16 x 16 grids stop removing clues after about half a second, so their hardest levels keep a few more.

Reference: [crossword](../docs/CROSSWORD.md) · [sudoku](../docs/SUDOKU.md).

## Performance and Android

> **Android is not verified on a device yet.** The code is written for it (no native libraries, no bytecode generation on
> Android) and the interpreter path it would use is tested on a desktop JVM, but it has not been run on a phone or
> emulator, and Android's Java API level and desugaring requirements have not been checked. Treat Android support as
> experimental until you have tried it in your app, and tell us what you find.

The engine is WebAssembly, run by [Chicory](https://github.com/dylibso/chicory), a WebAssembly runtime written in
pure Java. There are no native libraries, so there is nothing to build per CPU architecture.

| Where | How it runs | Measured (desktop JVM) |
| --- | --- | --- |
| JVM | Compiled to bytecode on first use | 30×30 grid with 60 words in about 3 s; a typical 10×10 puzzle in milliseconds |
| Android | Interpreter (ART cannot load generated JVM bytecode) | A typical 10×10 puzzle with 8 words in about 0.35 s on a desktop JVM (a phone is unmeasured and will be slower) |

The first call on each thread pays a one-time setup cost, and each thread keeps its own engine instance (a few MB), so use a bounded thread pool. Create one `Lettras` and reuse it. On Android, call it off
the main thread.

For the heaviest use (huge batches, real-time generation in a game loop), a native build of the engine per platform is
the next step; the API above stays the same.

## Tests

```bash
cd kotlin && ./gradlew test      # 26 tests
```

Besides behaviour tests (positions, German `ẞ`, classic mode, rejected and unplaced words, thread safety, a 30×30
puzzle), the suite checks that the library returns **exactly** the grids the npm package returns for the same input,
using the fixtures shared with the MCP server (`mcp/tests/fixtures/parity.json`). It also runs the interpreter path that
Android uses and checks it against the same fixture.

## License

Library code: MIT. The bundled engine (`src/main/resources/org/lettras/lettras_engine.wasm`) is proprietary; see
`LICENSE-ENGINE` next to it. You may use it unmodified, through this library, in your own products, including commercial
ones; you may not extract, redistribute, modify or reverse-engineer it.

## Java

The library is plain Java-friendly: the shorthand methods keep their defaults through overloads.

```java
import org.lettras.*;
import java.util.List;
import kotlin.Pair;

Lettras lettras = new Lettras();

Puzzle puzzle = lettras.generate(List.of("gato", "perro", "piña"), 9);          // rows = 9, cols = 9
FillResult filled = lettras.fill(puzzle);                                       // random letters in the empty cells
SudokuResult sudoku = lettras.sudoku(9);
boolean unique = lettras.solveSudoku(sudoku.getPuzzle()).getUnique();
CrosswordResult crossword = lettras.crossword(List.of(new Pair<>("gato", "Felino"), new Pair<>("toro", "Bovino")), 10);
StylizeResult bold = lettras.stylize("Ñandú");
```

Gradle: `implementation("org.lettras.artificialss:lettras:<version>")`; Maven: the same coordinates as `groupId:artifactId:version`. Records such as `Puzzle` expose
`getX()` accessors from Java. The overloads need a release after 0.5.0; with 0.5.0 pass every argument (use `null` for the optional ones).
