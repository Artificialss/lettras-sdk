<p align="center"><img alt="Lettras" src="https://raw.githubusercontent.com/Artificialss/lettras-sdk/main/assets/svg/lettras-logo.svg" width="260"></p>

# lettras

Word-search generator for **Spanish, English, Portuguese, French, German and Italian**, with native accented letters
kept as one cell each. Runs locally (WebAssembly): no network, no API key. Library and CLI.

```bash
npm install lettras
```

[npm package](https://www.npmjs.com/package/lettras) · [source and docs](https://github.com/Artificialss/lettras-sdk)

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

## Options

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

## Module format

The package is ESM only: use `import { generate, fill } from 'lettras'`. From CommonJS use `const { generate } = await import('lettras')`.
It needs Node 20 or newer (or any modern browser bundler) and ships its own TypeScript types.

## License

Wrapper, CLI and types: MIT. The compiled engine in `engine/` is proprietary: use it unmodified through this package;
see `engine/LICENSE`. Source and issues: <https://github.com/Artificialss/lettras-sdk>.
