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

CLI flags mirror these: `--words a,b,c --rows N --cols N --position P --difficulty N --clustering N --seed N --lang xx
--classic --fill C`, plus `--json`, `--solution` and `--stdin`.

`generate` throws on invalid input (for example zero rows). Words that cannot be placed are listed in `unplaced`;
words that are refused (spaces, digits, duplicates) are listed in `rejected` with a reason.

## License

Wrapper, CLI and types: MIT. The compiled engine in `engine/` is proprietary: use it unmodified through this package;
see `engine/LICENSE`. Source and issues: <https://github.com/Artificialss/lettras-sdk>.
