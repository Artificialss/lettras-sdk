/* tslint:disable */
/* eslint-disable */

export type Position = 'horizontal' | 'vertical' | 'mixed';
export interface GenerateInput {
    words: string[];
    rows: number;
    cols: number;
    position?: Position;
    difficulty?: 1 | 2 | 3 | 4;
    clustering?: number;
    seed?: number;
    lang?: string;
    classicMode?: boolean;
    fill?: string;
}
export interface Placement { word: string; r: number; c: number; dr: number; dc: number; length: number }
export interface FindBlockedInput { grid: string[][]; blocked?: string[] }
export interface BlockedHit { word: string; r: number; c: number; dr: number; dc: number; length: number }
export interface FillInput {
    grid: string[][];
    avoidBlocked?: boolean; // default true
    blocked?: string[];
    lang?: string;
    accents?: boolean;
    seed?: number;
    empty?: string;
    words?: string[];
}
export interface FillOutput {
    grid: string[][];
    filled: number;
    seed: number;
    ambiguous: string[];
    blockedLeft: string[];
    engineVersion: string;
}
export interface StylizeInput { text: string; style?: string | string[]; classic?: boolean }
export interface StyledPart { text: string; missing: boolean }
export interface StyledResult {
    id: string; label: string; category: string;
    text: string;
    notNative: string[];
    parts: StyledPart[];
}
export interface StylizeOutput { results: StyledResult[]; engineVersion: string }
export interface StyleInfo { id: string; label: string; category: string }
export type CrosswordEntry = string | { word: string; clue?: string };
export interface CrosswordInput {
    entries: CrosswordEntry[];
    rows: number;
    cols: number;
    seed?: number;
    classicMode?: boolean;
    attempts?: number; // layouts to try, default 30, at most 200
    block?: string; // marker of the blank cells, default '-'
    open?: string; // marker of the letter cells that do not start a word in `puzzle`, default '0'
}
export interface CrosswordPlacement {
    number: number; // unique per word, 1..N in reading order
    word: string; clue: string;
    direction: 'across' | 'down';
    r: number; c: number; length: number;
}
export interface CrosswordOutput {
    puzzle: string[][];   // word number on a first cell ('3', or '3/4' when an across and a down word start there), '0' on the other letter cells, '-' for blanks
    solution: string[][]; // same matrix with the correct letters
    rows: number;
    cols: number;
    placements: CrosswordPlacement[];
    unplaced: string[];
    rejected: { word: string; reason: string }[];
    seed: number;
    engineVersion: string;
}
export interface SudokuInput { size?: 4 | 6 | 9 | 12 | 16; difficulty?: 1 | 2 | 3 | 4; seed?: number }
export interface SudokuOutput {
    puzzle: number[][];   // 1..size on the clues, 0 on the empty cells
    solution: number[][]; // the same grid solved
    size: number; boxRows: number; boxCols: number;
    givens: number; difficulty: number; seed: number;
    engineVersion: string;
}
export interface SudokuSolveInput { puzzle: number[][] }
export interface SudokuSolveOutput { solution: number[][]; solved: boolean; unique: boolean; engineVersion: string }
export interface GenerateOutput {
    grid: string[][];
    placements: Placement[];
    words: string[];
    unplaced: string[];
    rejected: { word: string; reason: string }[];
    seed: number;
    engineVersion: string;
}



/**
 * `crossword(inputJson) -> outputJson`: a crossword layout with numbers and clues. Throws on invalid input.
 */
export function crossword(input_json: string): string;

/**
 * `fill(inputJson) -> outputJson`: random letters for the empty cells of a grid. Throws on invalid input.
 */
export function fill(input_json: string): string;

/**
 * `findBlocked(inputJson) -> hitsJson`: blocked words that read in a grid, in all 8 directions. Throws on invalid input.
 */
export function findBlocked(input_json: string): string;

/**
 * `generate(inputJson) -> outputJson`. Throws on invalid input.
 */
export function generate(input_json: string): string;

/**
 * `listStyles() -> json`: the ids, English names and categories of every style.
 */
export function listStyles(): string;

/**
 * Plain-text view of an output JSON, for terminals.
 */
export function renderPuzzle(output_json: string): string;

/**
 * `solveSudoku(inputJson) -> outputJson`: solves a puzzle and says if the answer is the only one. Throws on invalid input.
 */
export function solveSudoku(input_json: string): string;

/**
 * `stylize(inputJson) -> outputJson`: text in one, several or every Unicode style. Throws on invalid input.
 */
export function stylize(input_json: string): string;

/**
 * `sudoku(inputJson) -> outputJson`: a sudoku puzzle with a unique solution. Throws on invalid input.
 */
export function sudoku(input_json: string): string;

/**
 * `unstyle(inputJson) -> outputJson`: styled characters back to plain ones. Throws on invalid input.
 */
export function unstyle(input_json: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly crossword: (a: number, b: number, c: number) => void;
    readonly fill: (a: number, b: number, c: number) => void;
    readonly findBlocked: (a: number, b: number, c: number) => void;
    readonly generate: (a: number, b: number, c: number) => void;
    readonly listStyles: (a: number) => void;
    readonly renderPuzzle: (a: number, b: number, c: number) => void;
    readonly solveSudoku: (a: number, b: number, c: number) => void;
    readonly stylize: (a: number, b: number, c: number) => void;
    readonly sudoku: (a: number, b: number, c: number) => void;
    readonly unstyle: (a: number, b: number, c: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
