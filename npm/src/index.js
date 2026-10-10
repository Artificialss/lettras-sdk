import { generate as generateRaw, fill as fillRaw, findBlocked as findBlockedRaw, renderPuzzle as renderRaw } from '../engine/index.js';

/**
 * Generate a word-search puzzle. Runs locally (WebAssembly), no network needed.
 * Deterministic: the same input and seed give the same grid. Empty cells hold `-` (or `fill`);
 * pass the result to `fill()` to complete it with random letters.
 * @param {import('../engine/index.js').GenerateInput} input
 * @returns {import('../engine/index.js').GenerateOutput}
 */
export function generate(input) {
  return JSON.parse(generateRaw(JSON.stringify(input)));
}

function randomSeed() {
  return globalThis.crypto.getRandomValues(new Uint32Array(1))[0];
}

/**
 * Fill the empty cells of a grid with random letters.
 *
 * Pass the puzzle from `generate()` (its hidden words are then protected from accidental copies), or just a
 * matrix. Letters follow the language's letter frequency; `accents: false` limits them to A-Z. Without a `seed`
 * the filler is different every call; with one it is repeatable.
 *
 * @param {string[][] | import('../engine/index.js').GenerateOutput | import('../engine/index.js').FillInput} source
 * @param {Omit<import('../engine/index.js').FillInput, 'grid'>} [options]
 * @returns {import('../engine/index.js').FillOutput}
 */
export function fill(source, options = {}) {
  let base;
  if (Array.isArray(source)) base = { grid: source };
  else if ('placements' in source) base = { grid: source.grid, words: source.words }; // a puzzle: keep its own seed out of it
  else base = { ...source };
  const input = { ...base, ...options };
  if (input.seed === undefined) input.seed = randomSeed();
  return JSON.parse(fillRaw(JSON.stringify(input)));
}

/**
 * Plain-text view of a puzzle (grid and word bank), for terminals and logs.
 * @param {import('../engine/index.js').GenerateOutput} output
 * @returns {string}
 */
export function render(output) {
  return renderRaw(JSON.stringify(output));
}

/**
 * Blocked words that read in a grid, in all 8 directions, ignoring case and accents. Uses the engine's bank of unwanted
 * words from all six languages, plus any `blocked` words you add. Each hit has the start (`r`, `c`),
 * the direction (`dr`, `dc`) and the `length`.
 * @param {import('../engine/index.js').FindBlockedInput} input
 * @returns {import('../engine/index.js').BlockedHit[]}
 */
export function findBlocked(input) {
  return JSON.parse(findBlockedRaw(JSON.stringify(input)));
}
