import type { FillInput, FillOutput, GenerateInput, GenerateOutput, Placement, Position } from '../engine/index.js';

export type { FillInput, FillOutput, GenerateInput, GenerateOutput, Placement, Position };

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
