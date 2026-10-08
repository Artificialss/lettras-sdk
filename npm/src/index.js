import { generate as generateRaw, renderPuzzle as renderRaw } from '../engine/index.js';

/**
 * Generate a word-search puzzle. Runs locally (WebAssembly), no network needed.
 * Deterministic: the same input and seed give the same grid.
 * @param {import('../engine/index.js').GenerateInput} input
 * @returns {import('../engine/index.js').GenerateOutput}
 */
export function generate(input) {
  return JSON.parse(generateRaw(JSON.stringify(input)));
}

/**
 * Plain-text view of a puzzle (grid and word bank), for terminals and logs.
 * @param {import('../engine/index.js').GenerateOutput} output
 * @returns {string}
 */
export function render(output) {
  return renderRaw(JSON.stringify(output));
}
