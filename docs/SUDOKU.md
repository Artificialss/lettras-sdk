# Sudoku

Puzzles with exactly one solution, in five sizes, and a solver. Available since 0.5.0 in npm, Kotlin and the MCP server.

## Use it

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

## Example output

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

## How it works

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
