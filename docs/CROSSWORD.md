# Crossword (crucigrama)

> **Coming in 0.5.0.** This page describes the next release. It is not in the published packages yet.

Builds a crossword from words and clues, inside a maximum size. Every word crosses another, so the result is one connected shape, and
the grid is cropped to what it uses. Words that do not fit are returned in `unplaced`. The same input and `seed` always give the same grid.
It works in every language and keeps accents (`Ñ`, `ẞ` are one cell each); `classicMode` strips them.

```js
import { crossword } from 'lettras';

const out = crossword({
  entries: [
    { word: 'gato', clue: 'Maúlla' },
    { word: 'oso',  clue: 'Duerme en invierno' },
    { word: 'sol',  clue: 'Brilla de día' },
    'taza',                                   // a plain string has no clue
  ],
  rows: 13, cols: 13,   // the largest size; the result is cropped
  seed: 2,              // optional
  attempts: 30,         // optional, up to 200: more layouts tried, denser grid
});
```

Offer more words than you need: the engine keeps the best subset that fits.

## Result

Two matrices of the same size: the game position and the solution.

```json
{
  "puzzle":   [["-","-","-","1"],
               ["2","0","0","0"],
               ["-","-","-","0"],
               ["-","3","4","0"],
               ["-","-","0","-"],
               ["-","-","0","-"]],
  "solution": [["-","-","-","G"],
               ["T","A","Z","A"],
               ["-","-","-","T"],
               ["-","O","S","O"],
               ["-","-","O","-"],
               ["-","-","L","-"]],
  "rows": 6, "cols": 4,
  "placements": [
    { "number": 1, "word": "gato", "clue": "Maúlla",             "direction": "down",   "r": 0, "c": 3, "length": 4 },
    { "number": 2, "word": "taza", "clue": "",                   "direction": "across", "r": 1, "c": 0, "length": 4 },
    { "number": 3, "word": "oso",  "clue": "Duerme en invierno", "direction": "across", "r": 3, "c": 1, "length": 3 },
    { "number": 4, "word": "sol",  "clue": "Brilla de día",      "direction": "down",   "r": 3, "c": 2, "length": 3 }
  ],
  "unplaced": [], "rejected": [], "seed": 2, "engineVersion": "0.5.0"
}
```

### The `puzzle` matrix

| Cell | Meaning |
| --- | --- |
| `-` | blank, no word goes here |
| a number, such as `3` | the first cell of a word; it is that word's `number` in `placements` |
| `a/d`, such as `1/2` | the first cell of **two** words that start together: an across word numbered `a` and a down word numbered `d` |
| `0` | any other cell where a letter goes |

Every word has its own number, 1 to N, in reading order of its first cell (across before down in a shared cell). `a/d` appears only
when an across and a down word start in the same cell; every other first cell shows its single number. The markers can be changed
with `block` (blank, default `-`) and `open` (default `0`).

### The `solution` matrix
The same shape with the correct letters (one grapheme per cell) and `-` for the blanks.

### `placements`
One entry per placed word, sorted by `number`: `number`, `word`, `clue`, `direction` (`across` or `down`), `r` and `c` (row and
column of the first letter, from 0) and `length`. Draw the clue lists from it: the across clues, then the down clues.

## Options

| Option | Default | |
| --- | --- | --- |
| `entries` | | words as `{ word, clue }` or plain strings; up to 500, each up to 100 characters; letters only |
| `rows`, `cols` | | largest height and width, 1 to 100 |
| `seed` | 1 | |
| `attempts` | 30 | layouts to try, up to 200 |
| `classicMode` | `false` | strip accents |
| `block`, `open` | `-`, `0` | markers in `puzzle` |

Speed: a 13x13 grid with 22 words takes about 10 ms at 30 attempts. The layout is free-form (about half of the grid is filled), not a
dense newspaper grid.

Also in Kotlin (`Lettras.crossword(...)`) and as the MCP tool `crossword`.
