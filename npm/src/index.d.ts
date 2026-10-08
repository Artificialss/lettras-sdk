import type { GenerateInput, GenerateOutput, Placement, Position } from '../engine/index.js';

export type { GenerateInput, GenerateOutput, Placement, Position };

/** Generate a word-search puzzle locally (WebAssembly). Same input and seed give the same grid. */
export function generate(input: GenerateInput): GenerateOutput;

/** Plain-text view of a puzzle (grid and word bank). */
export function render(output: GenerateOutput): string;
