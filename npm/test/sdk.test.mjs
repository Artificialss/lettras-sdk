import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';
import { findBlocked, generate, render } from '../src/index.js';

const cli = new URL('../bin/lettras.js', import.meta.url).pathname;
const run = (...args) => execFileSync('node', [cli, ...args], { encoding: 'utf8' });

test('generate places every word, one grapheme per cell', () => {
  const out = generate({ words: ['gato', 'piña', 'corazón'], rows: 9, cols: 12, position: 'mixed', seed: 8 });
  assert.deepEqual(out.unplaced, []);
  assert.equal(out.grid.length, 9);
  assert.equal(out.grid[0].length, 12);
  assert.ok(out.grid.flat().includes('Ñ'));
  assert.equal(out.placements.find((p) => p.word === 'piña').length, 4);
});

test('same seed, same grid; different seed, different grid', () => {
  const a = generate({ words: ['sol', 'luna', 'mar'], rows: 8, cols: 8, seed: 5 });
  assert.deepEqual(a, generate({ words: ['sol', 'luna', 'mar'], rows: 8, cols: 8, seed: 5 }));
  assert.notDeepEqual(a.grid, generate({ words: ['sol', 'luna', 'mar'], rows: 8, cols: 8, seed: 6 }).grid);
});

test('positions: horizontal and vertical are one-way', () => {
  const h = generate({ words: ['gato', 'perro', 'mono'], rows: 10, cols: 10, position: 'horizontal', seed: 1 });
  assert.ok(h.placements.every((p) => p.dr === 0 && p.dc === 1));
  const v = generate({ words: ['gato', 'perro', 'mono'], rows: 10, cols: 10, position: 'vertical', seed: 1 });
  assert.ok(v.placements.every((p) => p.dr === 1 && p.dc === 0));
});

test('German ß stays a single ẞ cell', () => {
  const out = generate({ words: ['Fuß', 'Straße'], rows: 8, cols: 8, lang: 'de' });
  assert.ok(out.grid.flat().includes('ẞ'));
  assert.ok(!out.grid.flat().includes('ß'));
});

test('invalid input throws; unplaceable words are reported', () => {
  assert.throws(() => generate({ words: ['sol'], rows: 0, cols: 5 }));
  const tight = generate({ words: ['abcdefghijklmnop'], rows: 6, cols: 6 });
  assert.deepEqual(tight.unplaced, ['abcdefghijklmnop']);
});

test('render prints the grid and the word bank', () => {
  const text = render(generate({ words: ['sol', 'río'], rows: 6, cols: 6, seed: 3 }));
  assert.match(text, /6×6/);
  assert.match(text, /Words: sol, río/);
});

test('CLI works', () => {
  assert.match(run('--words', 'sol,luna', '--rows', '7', '--seed', '2'), /Words: sol, luna/);
  const json = JSON.parse(run('--words', 'piña', '--rows', '6', '--json'));
  assert.equal(json.unplaced.length, 0);
});

import { fill } from '../src/index.js';

const animals = ['gato', 'perro', 'piña', 'corazón', 'ñandú', 'mono', 'cebra'];
const hasEmpty = (grid) => grid.flat().includes('-');

test('fill completes a puzzle: no empty cells, hidden letters untouched', () => {
  const p = generate({ words: animals, rows: 12, cols: 12, position: 'mixed', seed: 8 });
  assert.ok(hasEmpty(p.grid));
  const f = fill(p, { lang: 'es', seed: 1 });
  assert.ok(!hasEmpty(f.grid));
  assert.equal(f.filled, p.grid.flat().filter((c) => c === '-').length);
  p.grid.forEach((row, r) => row.forEach((cell, c) => { if (cell !== '-') assert.equal(f.grid[r][c], cell); }));
});

test('fill: same seed same filler, no seed a different filler each time', () => {
  const p = generate({ words: animals, rows: 12, cols: 12, seed: 8 });
  assert.deepEqual(fill(p, { seed: 5 }), fill(p, { seed: 5 }));
  const seeds = new Set(Array.from({ length: 5 }, () => fill(p).seed));
  assert.ok(seeds.size > 1, 'unseeded calls should pick different seeds');
});

test('fill: accents off gives plain A-Z, accents on gives native letters', () => {
  const grid = Array.from({ length: 40 }, () => Array(40).fill('-'));
  assert.ok(fill(grid, { lang: 'es', accents: false, seed: 3 }).grid.flat().every((c) => /^[A-Z]$/.test(c)));
  const on = fill(grid, { lang: 'es', accents: true, seed: 3 }).grid.flat();
  assert.ok(on.includes('Ñ'));
  assert.ok(!fill(grid, { lang: 'en', accents: true, seed: 3 }).grid.flat().some((c) => /[^A-Z]/.test(c)), 'english never has accents');
  assert.ok(fill(grid, { lang: 'de', accents: true, seed: 3 }).grid.flat().includes('ẞ'));
});

test('fill never adds a second copy of a hidden word', () => {
  const p = generate({ words: ['ab', 'cd', 'ef', 'gh'], rows: 8, cols: 8, position: 'mixed', seed: 2, lang: 'en' });
  for (let seed = 0; seed < 25; seed++) {
    const f = fill(p, { lang: 'en', accents: false, seed });
    assert.deepEqual(f.ambiguous, []);
  }
});

test('fill accepts a bare matrix and an input object, and rejects bad input', () => {
  const p = generate({ words: ['sol', 'luna'], rows: 6, cols: 6, seed: 1 });
  assert.deepEqual(fill(p.grid, { seed: 4 }).grid, fill({ grid: p.grid, seed: 4 }).grid);
  assert.throws(() => fill([]), /grid/);
  assert.throws(() => fill(p.grid, { lang: 'xx' }), /lang/);
});

test('CLI --random fills the grid; --accents off keeps it plain', () => {
  const text = run('--words', 'gato,piña', '--rows', '9', '--cols', '9', '--seed', '3', '--random', '--accents', 'off', '--json');
  const out = JSON.parse(text);
  assert.ok(!hasEmpty(out.grid));
  assert.ok(out.grid.flat().filter((c) => c === 'Ñ').length === 1, 'only the Ñ of piña, no accents in the filler');
  assert.ok(out.grid.flat().every((c) => c === 'Ñ' || /^[A-Z]$/.test(c)));
  assert.equal(run('--words', 'gato', '--rows', '7', '--seed', '2', '--random', '--accents', 'off', '--json'),
               run('--words', 'gato', '--rows', '7', '--seed', '2', '--random', '--accents', 'off', '--json'), 'seeded CLI output is repeatable');
  assert.throws(() => run('--words', 'gato', '--rows', '7', '--random', '--accents', 'maybe'));
});

test('reports the real engine version', () => {
  assert.equal(generate({ words: ['sol'], rows: 6, cols: 6 }).engineVersion, '0.3.0');
  assert.equal(fill([['-']]).engineVersion, '0.3.0');
});

test('a letter or digit cannot be the empty marker; oversized input is rejected', () => {
  for (const bad of ['A', '7', 'ab']) {
    assert.throws(() => fill([['A', '-']], { empty: bad }), /empty/);
    assert.throws(() => generate({ words: ['sol'], rows: 8, cols: 8, fill: bad }), /fill/);
  }
  assert.throws(() => generate({ words: ['sol'], rows: 101, cols: 8 }), /at most/);
  assert.throws(() => generate({ words: Array(501).fill('ab'), rows: 8, cols: 8 }), /words/);
});

import { readFileSync } from 'node:fs';

test('the published entry points never reference a .wasm file or build a URL (bundlers would fail to resolve it)', () => {
  // 0.2.1 shipped a dead `new URL('lettras_engine_bg.wasm', import.meta.url)`; the file is not published, so webpack and
  // Turbopack failed to build any app importing the package. Plain Node never noticed.
  for (const name of ['lettras_engine.js']) {
    const text = readFileSync(new URL(`../engine/${name}`, import.meta.url), 'utf8');
    assert.doesNotMatch(text, /_bg\.wasm/, `${name} references the .wasm file`);
    assert.doesNotMatch(text, /new URL\(/, `${name} builds a URL`);
    assert.doesNotMatch(text, /import\.meta\.url/, `${name} uses import.meta.url`);
  }
});

test('findBlocked finds a word in any direction and fill keeps blocked words out', () => {
  const only = (hits) => hits.filter((h) => h.word === 'GATO');
  assert.equal(only(findBlocked({ grid: [['Q', 'Q', 'Q', 'Q']], blocked: ['gato'] })).length, 0);
  const hits = only(findBlocked({ grid: [['Q', 'O', 'T', 'A', 'G', 'Q']], blocked: ['gato'] }));
  assert.deepEqual(hits.map((h) => [h.word, h.dc, h.length]), [['GATO', -1, 4]]);
  const extra = ['ESA', 'ASE', 'OSO'];
  const out = fill(Array.from({ length: 12 }, () => Array(12).fill('-')), { lang: 'es', seed: 5, blocked: extra });
  assert.deepEqual(findBlocked({ grid: out.grid, blocked: extra }), []);
  assert.equal(out.blockedLeft, undefined);
});
