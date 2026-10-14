import assert from 'node:assert/strict';
import test from 'node:test';
import { parseWords, styleChoices, styleText, sudokuText, wordSearchText } from '../src/core.js';

test('parseWords splits on commas, semicolons and lines', () => {
  assert.deepEqual(parseWords('gato, perro;\npiña ,'), ['gato', 'perro', 'piña']);
});

test('there are 22 styles and bold changes the text', () => {
  assert.equal(styleChoices().length, 22);
  assert.notEqual(styleText('hola', 'bold').text, 'hola');
});

test('a style reports the characters it cannot draw', () => {
  assert.ok(Array.isArray(styleText('Ñandú', 'bold').missing));
});

test('word search text has a grid, a word list and no empty cells', () => {
  const text = wordSearchText(['gato', 'perro', 'piña'], 'es');
  assert.match(text, /Words: /);
  assert.ok(!text.split('\n\n')[0].includes('-'));
});

test('sudoku text has one row per cell of the size', () => {
  assert.equal(sudokuText(9, 2).trim().split('\n').length, 9);
});
