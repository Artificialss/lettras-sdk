#!/usr/bin/env node
// lettras --words sol,luna,mar --rows 8 --cols 8 [--position horizontal|vertical|mixed] [--seed 7]
//         [--difficulty 1-4] [--clustering 0-1] [--lang es] [--classic] [--fill -] [--random] [--accents on|off]
//         [--json] [--solution]
// or:     echo '{"words":["sol"],"rows":6,"cols":6}' | lettras --stdin
import { readFileSync } from 'node:fs';
import { generate, renderPuzzle } from '../engine/index.js';
import { fill } from '../src/index.js';

const HELP = `lettras: word-search generator

  lettras --words sol,luna,mar --rows 8 --cols 8 [options]

  --words a,b,c        word bank (comma separated)
  --rows N --cols N    grid size (rectangular allowed)
  --position P         horizontal | vertical | mixed (mixed = all 8 directions)
  --difficulty 1-4     directions when --position is not set
  --clustering 0-1     0 = words apart, 1 = words crossing (default 0.5)
  --seed N             same seed, same grid
  --lang xx            es en pt fr de it
  --classic            strip accents in the grid
  --fill C             empty-cell character (default -)
  --random             fill the empty cells with random letters
  --accents on|off     with --random: use the language's accented letters (default on; off with --classic)
  --json               print the JSON result
  --solution           show only the hidden words
  --stdin              read the JSON input from stdin`;

function parse(argv) {
  const flags = new Set(['classic', 'json', 'solution', 'stdin', 'help', 'random']);
  const opts = {};
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (!a.startsWith('--')) throw new Error(`unexpected argument: ${a}`);
    const key = a.slice(2);
    if (flags.has(key)) opts[key] = true;
    else if (i + 1 < argv.length) opts[key] = argv[++i];
    else throw new Error(`missing value for --${key}`);
  }
  return opts;
}

function main() {
  const o = parse(process.argv.slice(2));
  if (o.help) return console.log(HELP);
  let input;
  if (o.stdin) {
    input = JSON.parse(readFileSync(0, 'utf8'));
  } else {
    if (!o.words || !o.rows) { console.log(HELP); process.exit(o.words || o.rows ? 2 : 0); }
    input = {
      words: o.words.split(',').map((w) => w.trim()).filter(Boolean),
      rows: Number(o.rows),
      cols: Number(o.cols ?? o.rows),
    };
    if (o.position) input.position = o.position;
    if (o.difficulty) input.difficulty = Number(o.difficulty);
    if (o.clustering) input.clustering = Number(o.clustering);
    if (o.seed) input.seed = Number(o.seed);
    if (o.lang) input.lang = o.lang;
    if (o.classic) input.classicMode = true;
    if (o.fill) input.fill = o.fill;
  }
  let outJson = generate(JSON.stringify(input));
  let out = JSON.parse(outJson);
  if (o.random) {
    if (o.accents !== undefined && !['on', 'off'].includes(o.accents)) throw new Error('--accents must be on or off');
    const accents = o.accents ? o.accents === 'on' : !input.classicMode;
    const filled = fill(out, { lang: input.lang, accents, seed: input.seed });
    out = { ...out, grid: filled.grid };
    outJson = JSON.stringify(out);
  }
  if (o.json) return console.log(outJson);
  if (o.solution) {
    const hidden = new Set(out.placements.flatMap((p) => Array.from({ length: p.length }, (_, i) => `${p.r + p.dr * i},${p.c + p.dc * i}`)));
    return console.log(out.grid.map((row, r) => row.map((ch, c) => (hidden.has(`${r},${c}`) ? ch : '·')).join(' ')).join('\n'));
  }
  console.log(renderPuzzle(outJson));
}

try { main(); } catch (e) { console.error(`lettras: ${e.message ?? e}`); process.exit(1); }
