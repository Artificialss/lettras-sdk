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
export interface FillInput {
    grid: string[][];
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
    engineVersion: string;
}
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
 * `fill(inputJson) -> outputJson`: random letters for the empty cells of a grid. Throws on invalid input.
 */
export function fill(input_json: string): string;

/**
 * `generate(inputJson) -> outputJson`. Throws on invalid input.
 */
export function generate(input_json: string): string;

/**
 * Plain-text view of an output JSON, for terminals.
 */
export function renderPuzzle(output_json: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly fill: (a: number, b: number, c: number) => void;
    readonly generate: (a: number, b: number, c: number) => void;
    readonly renderPuzzle: (a: number, b: number, c: number) => void;
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
