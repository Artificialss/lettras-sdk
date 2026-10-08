import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';
import { generate, render } from '../src/index.js';

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
