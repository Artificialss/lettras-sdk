// Pure functions behind the commands, so they can be tested without VS Code.
import { generate, fill, render, sudoku, stylize, listStyles } from 'lettras';

/** Splits "gato, perro; piña" into words. */
export function parseWords(text) {
  return text.split(/[,;\n]+/).map((w) => w.trim()).filter(Boolean);
}

/** The styles for a quick pick, in display order. */
export function styleChoices() {
  return listStyles().map((s) => ({ id: s.id, label: s.label, description: s.category }));
}

/** The selection written in one style. Characters the style cannot draw are reported in `missing`. */
export function styleText(text, id) {
  const [r] = stylize(text, { style: id }).results;
  return { text: r.text, missing: r.notNative ?? [] };
}

/** A word search as text: the grid with random letters in the empty cells, then the word list. */
export function wordSearchText(words, lang, size = 12) {
  const rows = Math.max(8, Math.min(size, 30));
  const puzzle = generate({ words, rows, cols: rows, position: 'mixed', lang });
  const done = fill(puzzle, { lang, accents: true });
  const grid = done.grid.map((r) => r.join(' ')).join('\n');
  const unplaced = puzzle.unplaced?.length ? `\n(not placed: ${puzzle.unplaced.join(', ')})` : '';
  return `${grid}\n\nWords: ${puzzle.words.join(', ')}${unplaced}\n`;
}

/** A sudoku as text, `.` for empty cells. */
export function sudokuText(size = 9, difficulty = 2) {
  const s = sudoku({ size, difficulty });
  const cell = (n) => (n === 0 ? '.' : n > 9 ? String.fromCharCode(55 + n) : String(n));
  const grid = s.puzzle.map((r) => r.map(cell).join(' ')).join('\n');
  return `${grid}\n`;
}
