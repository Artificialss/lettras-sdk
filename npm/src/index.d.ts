import type {
  BlockedHit,
  CrosswordInput,
  CrosswordOutput,
  FillInput,
  FillOutput,
  FindBlockedInput,
  GenerateInput,
  GenerateOutput,
  Placement,
  Position,
  StyleInfo,
  SudokuInput,
  SudokuOutput,
  SudokuSolveInput,
  SudokuSolveOutput,
  StyledPart,
  StyledResult,
  StylizeInput,
  StylizeOutput,
} from '../engine/index.js';

export type { BlockedHit, CrosswordInput, CrosswordOutput, SudokuInput, SudokuOutput, SudokuSolveInput, SudokuSolveOutput, FillInput, FillOutput, FindBlockedInput, GenerateInput, GenerateOutput, Placement, Position, StyleInfo, StyledPart, StyledResult, StylizeInput, StylizeOutput };

/** Options for {@link fill}: everything in {@link FillInput} except the grid. */
export type FillOptions = Omit<FillInput, 'grid'>;

/** Generate a word-search puzzle locally (WebAssembly). Same input and seed give the same grid. */
export function generate(input: GenerateInput): GenerateOutput;

/**
 * Fill the empty cells with random letters, in the given language, with accents on or off.
 * Pass the puzzle from `generate()` to protect its words from accidental copies. Without a `seed` the
 * filler is different every call.
 */
export function fill(puzzle: GenerateOutput, options?: FillOptions): FillOutput;
export function fill(grid: string[][], options?: FillOptions): FillOutput;
export function fill(input: FillInput, options?: FillOptions): FillOutput;

/** Plain-text view of a puzzle (grid and word bank). */
export function render(output: GenerateOutput): string;

/** Blocked words that read in a grid, in all 8 directions, ignoring case and accents (all six languages). */
export function findBlocked(input: FindBlockedInput): BlockedHit[];

/** Options for {@link stylize}: everything in {@link StylizeInput} except the text. */
export type StylizeOptions = Omit<StylizeInput, 'text'>;

/**
 * Text in Unicode styles that keep accents (22 styles: bold, italic, script, fraktur, circled, strikethrough...).
 * `style` is one id, an array of ids, or omitted for all. `notNative` and `parts[].missing` report what a style cannot
 * draw. Throws on an unknown style id, invalid input, or text over 10,000 characters.
 */
export function stylize(text: string, options?: StylizeOptions): StylizeOutput;
export function stylize(input: StylizeInput, options?: StylizeOptions): StylizeOutput;

/** Styled characters back to plain ones, decoration marks removed. */
export function unstyle(text: string): string;

/** The available styles in display order (`id`, English `label`, `category`). */
export function listStyles(): StyleInfo[];

/**
 * A crossword (crucigrama) from words with clues: `puzzle` (blank `-`, word numbers, `0` on other letter cells, `a/d` when an
 * across and a down word start in the same cell) and `solution` (letters), plus `placements` with numbers and clues.
 */
export function crossword(input: Omit<CrosswordInput, 'cols'> & { cols?: number }): CrosswordOutput;

/** A sudoku with exactly one solution (sizes 4, 6, 9, 12, 16; difficulty 1 to 4). `0` is an empty cell. */
export function sudoku(input?: SudokuInput): SudokuOutput;

/** Solves a sudoku grid and says whether the solution is unique. Throws on a bad grid. */
export function solveSudoku(puzzle: number[][] | SudokuSolveInput): SudokuSolveOutput;
