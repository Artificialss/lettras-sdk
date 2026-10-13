<p align="center"><img alt="Lettras" src="https://raw.githubusercontent.com/Artificialss/lettras-sdk/main/assets/svg/lettras-logo.svg" width="260"></p>

# lettras

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each in a grid, one styled letter plus its accent in text). Runs locally (WebAssembly): no
network, no API key. Library and CLI.

```bash
npm install lettras
```

[npm package](https://www.npmjs.com/package/lettras) · [source and docs](https://github.com/Artificialss/lettras-sdk)

What is inside: [word search](#word-search-options) · [Unicode text styles](#unicode-text-styles) · [crossword](#crossword-crucigrama) · [sudoku](#sudoku)

```js
import { generate, render } from 'lettras';

const puzzle = generate({ words: ['gato', 'perro', 'piña'], rows: 9, cols: 12, position: 'mixed', seed: 8 });
console.log(render(puzzle));
puzzle.grid;        // string[][], one letter per cell, '-' for empty cells
puzzle.placements;  // where each word is hidden: start (r, c), step (dr, dc), length
puzzle.unplaced;    // words that did not fit
```

```js
import { generate, fill, render } from 'lettras';

const puzzle = generate({ words: ['gato', 'perro', 'piña'], rows: 9, cols: 12, position: 'mixed', seed: 8 });
const done = fill(puzzle, { lang: 'es', accents: true }); // random letters in the empty cells
console.log(done.grid.map((row) => row.join(' ')).join('\n'));
```

```bash
npx lettras --words gato,perro,piña --rows 9 --cols 12 --position mixed --random --accents on
```

## Word search options

| Option | Values |
| --- | --- |
| `words`, `rows`, `cols` | Word list and grid size (rectangular grids are fine). |
| `position` | `horizontal`, `vertical` or `mixed` (all 8 directions). |
| `difficulty` | 1–4, used when `position` is not set. |
| `clustering` | 0 words apart · 1 words crossing · default 0.5. |
| `seed` | Same input and seed give the same grid. |
| `lang` | `es` (default) `en` `pt` `fr` `de` `it`. |
| `classicMode` | Strip accents in the grid. |
| `fill` | Empty-cell character, default `-`. |

`generate` leaves empty cells as `-`. **`fill(puzzleOrGrid, { lang, accents, seed })`** completes them: letters follow the
language's letter frequency, `accents: false` limits them to A-Z, and without a `seed` every call is different. Passing the
puzzle (instead of a bare matrix) also protects its words from accidental extra copies. CLI: `--random` and `--accents on|off`.

**Example.** The same puzzle before and after `fill` (Spanish, seed 5):

```js
const puzzle = generate({ words: ['gato', 'perro', 'piña', 'mono', 'cebra'], rows: 8, cols: 10, position: 'mixed', seed: 8 });
const done = fill(puzzle, { lang: 'es', accents: true, seed: 5 });
```

`puzzle.grid` has `-` in every empty cell; `done.grid` replaces them (58 cells here):

```
C E B R A - O - - -
- - - - - - - T O -
- - - - M - - R A -
- - - O - - R - - G
- - N - - E - - - -
- O - - P - - - - -
- - - - - - - - - -
- - - - - - A Ñ I P
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

With `accents: false` the filler uses plain A-Z only (the one `Ñ` left is the hidden word *piña*, which keeps its spelling):

```
C E B R A R O G G I
D Q A E E P C T O D
B Y E E M X M R A N
S R C O N R R A E G
S R N E A E R O E M
I O O E P S N O A O
O B O O A P O A T A
H R E A A S A Ñ I P
```

Same grid, language, accents and `seed` always give the same filler on every platform. Without a `seed` the wrappers
pick a random one, so every call gives different letters.

CLI flags mirror these: `--words a,b,c --rows N --cols N --position P --difficulty N --clustering N --seed N --lang xx
--classic --fill C`, plus `--json`, `--solution` and `--stdin`.

`generate` throws on invalid input (for example zero rows). Words that cannot be placed are listed in `unplaced`;
words that are refused (spaces, digits, duplicates) are listed in `rejected` with a reason.

## Unicode text styles

Write text in 22 Unicode styles (bold, italic, script, fraktur, circled, strikethrough...) that **keep accents**. It is the engine behind Social Media Styles at [lettras.org](https://lettras.org), and it runs locally.

### Use it

```js
import { stylize, unstyle, listStyles } from 'lettras';

// all 22 styles
stylize('Hola piña').results.map((r) => r.text);

// one style, or several
stylize('Lettras ejemplo: piña, corazón', { style: 'bold' }).results[0].text;
const out = stylize('Lettras', { style: ['bold', 'circled'] });
out.results.map((r) => r.text);   // [ '𝐋𝐞𝐭𝐭𝐫𝐚𝐬', 'Ⓛⓔⓣⓣⓡⓐⓢ' ]

// each result: { id, label, category, text, notNative, parts }
const bold = stylize('piña', { style: 'bold' }).results[0];
bold.text;       // 𝐩𝐢𝐧̃𝐚   (ñ = styled n + combining tilde)
bold.notNative;  // [ 'ñ' ]
unstyle(bold.text);   // 'piña'
listStyles();         // [ { id: 'bold', label: 'Bold', category: 'bold-italic' }, ... 22 in display order ]
```

**Kotlin**

```kotlin
val bold = Lettras().stylize("piña", style = "bold").results.single()
bold.text; bold.notNative; Lettras().unstyle(bold.text); Lettras().listStyles()
```

**MCP** (tools `stylize_text` and `list_styles`, free, no usage limit): ask *"write 'Lettras ejemplo' in bold and script"*, or call it with

```json
{ "name": "stylize_text", "arguments": { "text": "Lettras ejemplo", "style": ["bold", "script"] } }
```

### Example output

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

## Crossword (crucigrama)

Words with clues in, a crossword out: a `puzzle` matrix to play and a `solution` matrix to check. Every language works and accents are kept.

### Use it

```js
import { crossword } from 'lettras';

const out = crossword({
  entries: [
    { word: 'gato',  clue: 'Maúlla y caza ratones' },
    { word: 'perro', clue: 'El mejor amigo del hombre' },
    { word: 'mono',  clue: 'Trepa por los árboles' },
    { word: 'cebra', clue: 'Rayas blancas y negras' },
    { word: 'oso',   clue: 'Duerme en invierno' },
    { word: 'rata',  clue: 'Roedor pequeño' },
    { word: 'toro',  clue: 'Embiste al capote' },
    { word: 'piña',  clue: 'Fruta tropical con corona' },
    { word: 'ñu',    clue: 'Antílope africano' },
  ],
  rows: 12, cols: 12,   // the largest it may take; the result is cropped
  seed: 4,              // optional: same input and seed, same crossword
});
```

**Kotlin**

```kotlin
val c = Lettras().crossword(listOf("gato" to "Maúlla y caza ratones", "perro" to "El mejor amigo del hombre"), rows = 12, seed = 4)
c.puzzle; c.solution; c.placements; c.clues()
```

**MCP** (tool `crossword`, free, no usage limit): ask *"make a Spanish animals crossword with these words and clues"*, or call it with

```json
{ "name": "crossword", "arguments": { "entries": [ { "word": "gato", "clue": "Maúlla y caza ratones" }, { "word": "perro", "clue": "El mejor amigo del hombre" } ], "rows": 12, "seed": 4 } }
```


### Example output

`out.puzzle` (7 rows x 11 columns here, what the player sees):

```
-   -   -   -   -   1   -   2   -   -   -
-   -   -   -   -   0   -   0   -   -   3
-   -   -   -   -   0   -   4   0   0   0
-   -   -   5/6 0   0   0   0   -   -   0
-   -   -   0   -   0   -   -   7   0   0
-   -   -   8   0   -   -   -   -   -   -
9   0   0   0   -   -   -   -   -   -   -
```

`out.solution` (the answers):

```
- - - - - C - G - - -
- - - - - E - A - - M
- - - - - B - T O R O
- - - P E R R O - - N
- - - I - A - - O S O
- - - Ñ U - - - - - -
R A T A - - - - - - -
```

`out.placements` lists every word with its number, clue, direction and first cell (`r` row, `c` column, both from 0):

```json
{ "number": 1, "word": "cebra", "clue": "Rayas blancas y negras",      "direction": "down",   "r": 0, "c": 5, "length": 5 }
{ "number": 5, "word": "perro", "clue": "El mejor amigo del hombre",   "direction": "across", "r": 3, "c": 3, "length": 5 }
{ "number": 6, "word": "piña",  "clue": "Fruta tropical con corona",   "direction": "down",   "r": 3, "c": 3, "length": 4 }
```

The clue lists for the player come straight from `placements`:

```
Across                                  Down
4. Embiste al capote (4)                1. Rayas blancas y negras (5)
5. El mejor amigo del hombre (5)        2. Maúlla y caza ratones (4)
7. Duerme en invierno (3)               3. Trepa por los árboles (4)
8. Antílope africano (2)                6. Fruta tropical con corona (4)
9. Roedor pequeño (4)
```

Other fields: `rows`, `cols` (size of the cropped grid), `unplaced`, `rejected` (words refused, with a reason), `seed`, `engineVersion`.
Options: `entries`, `rows`, `cols`, `seed`, `attempts` (1 to 200, default 30: more attempts, denser crossword), `classicMode` (strip accents),
`block` and `open` (the markers above). Every language works and accents are kept.

### How it works

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

Reference: [docs/CROSSWORD.md](https://github.com/Artificialss/lettras-sdk/blob/main/docs/CROSSWORD.md).

## Sudoku

Puzzles with **exactly one solution**, in five sizes, and a solver.

### Use it

```js
import { sudoku, solveSudoku } from 'lettras';

const s = sudoku({ size: 9, difficulty: 2, seed: 5 });
s.givens;    // 32 clues
s.puzzle;    // 9 rows of numbers, 0 = empty cell
s.solution;  // the same grid solved
```

**Kotlin**

```kotlin
val s = Lettras().sudoku(size = 9, difficulty = 2, seed = 5)
val r = Lettras().solveSudoku(s.puzzle)   // solution, solved, unique
```

**MCP** (tools `sudoku` and `solve_sudoku`, free, no usage limit): ask *"give me an easy 4x4 sudoku for a child"*, or call them with

```json
{ "name": "sudoku", "arguments": { "size": 9, "difficulty": 2, "seed": 5 } }
{ "name": "solve_sudoku", "arguments": { "puzzle": [[0,7,5,2,3,0,6,9,0], "... 8 more rows ..."] } }
```


### Example output

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

### How it works

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

Reference: [docs/SUDOKU.md](https://github.com/Artificialss/lettras-sdk/blob/main/docs/SUDOKU.md).

## Module format

The package is ESM only: use `import { generate, fill } from 'lettras'`. From CommonJS use `const { generate } = await import('lettras')`.
It needs Node 20 or newer (or any modern browser bundler) and ships its own TypeScript types.

## License

Wrapper, CLI and types: MIT. The compiled engine in `engine/` is proprietary: use it unmodified through this package;
see `engine/LICENSE`. Source and issues: <https://github.com/Artificialss/lettras-sdk>.

---

<p align="center">Made by <a href="https://artificialss.ai">Artificialss</a> · <a href="https://lettras.org">lettras.org</a></p>
