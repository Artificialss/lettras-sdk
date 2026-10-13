# Sudoku

> Available since 0.5.0 in npm, Kotlin and the MCP server.

Builds a sudoku with **exactly one solution**, and solves any sudoku. Same `seed`, same puzzle.

```js
import { sudoku, solveSudoku } from 'lettras';

const out = sudoku({ size: 9, difficulty: 2, seed: 5 });
out.puzzle;    // 9 rows of numbers: 1..9 on the clues, 0 on the empty cells
out.solution;  // the same grid solved
out.givens;    // clues left

const solved = solveSudoku(out.puzzle);
solved.solved; // true
solved.unique; // true: it is the only solution
```

| Option | Default | |
| --- | --- | --- |
| `size` | 9 | 4 (2x2 boxes), 6 (2x3), 9 (3x3), 12 (3x4) or 16 (4x4) |
| `difficulty` | 2 | 1 easy (most clues) to 4 expert (fewest) |
| `seed` | 1 | |

Result: `puzzle`, `solution`, `size`, `boxRows`, `boxCols`, `givens`, `difficulty`, `seed`, `engineVersion`.

Clues left on a 9x9 grid are about 41 / 32 / 28 / 24 for difficulty 1 / 2 / 3 / 4. Grids up to 9x9 take under 40 ms; 12x12 and 16x16
stop taking clues out after about half a second, so their hardest levels keep a few more clues.

`solveSudoku(grid)` takes a square grid (the sizes above, `0` for empty) and returns `{ solution, solved, unique }`. A number repeated
in a row, column or box is an error.

## Kotlin

```kotlin
val s = Lettras().sudoku(size = 9, difficulty = 2, seed = 5)   // SudokuResult: puzzle, solution (List<List<Int>>), givens...
val r = Lettras().solveSudoku(s.puzzle)                        // SudokuSolution: solution, solved, unique
```

## MCP

The tools `sudoku` (`size`, `difficulty`, `seed`) and `solve_sudoku` (`puzzle`) are free and have no usage limit.
