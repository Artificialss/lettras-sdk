# Crossword (crucigrama)

Words with clues in, a crossword out: a `puzzle` matrix to play and a `solution` matrix to check. Available since 0.5.0 in npm, Kotlin and the MCP server. Every language works and accents are kept.

## Use it

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

## Example output

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

## How it works

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
