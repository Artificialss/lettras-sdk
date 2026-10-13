# Changelog

All three distributions (npm `lettras`, Maven `org.lettras.artificialss:lettras`, the MCP server) are versioned together.

## 0.5.1 (npm only)
- npm package page: dedicated sections for the crossword, the sudoku and the Unicode text styles, each with code to use it, real example output and how it works. No code changes.

## 0.5.0
- **Crossword (crucigrama):** `crossword({ entries, rows, cols, ... })` returns a `puzzle` matrix (word numbers, `0`, `-`, and `a/d` where an
  across and a down word start in the same cell), a `solution` matrix and `placements` with the clue of each word. npm, Kotlin and MCP.
  See `docs/CROSSWORD.md`.
- **Sudoku:** `sudoku({ size, difficulty, seed })` (unique solution) and `solveSudoku(grid)`. npm, Kotlin and MCP. See `docs/SUDOKU.md`.
- MCP: new tools `crossword`, `sudoku` and `solve_sudoku`, free with no usage limit (they do not use up puzzles or fills).
- Engine 0.5.0 (`engineVersion`); word searches, fills and styles for the same seed are identical to 0.4.0.

## 0.4.1 (npm only)
- npm package page: the README now introduces the Unicode text styles (`stylize`, `unstyle`, `listStyles`), shows the examples and the Artificialss credit. No code change: Maven stays at 0.4.0 and the MCP server at 0.4.0.

## 0.4.0
- **New: Unicode text styles** (the engine behind Social Media Styles). 22 styles (bold, italic, script, fraktur, double-struck,
  monospace, small caps, fullwidth, circled, squared, parenthesized, strikethrough, underline and more) that **keep accents**: an
  accented letter is its styled base letter plus the combining mark, so "piña" stays readable. What a style cannot draw (ß, Æ, Œ, Ø,
  non-Latin scripts, or an accent that only exists as a combining mark) is reported in `notNative` and as `parts` with
  `missing: true`, so a UI can show an empty box like a phone without that glyph. `classic: true` strips accents first.
  - npm: `stylize(text, { style?, classic? })`, `unstyle(text)`, `listStyles()`.
  - Kotlin: `Lettras.stylize(...)`, `unstyle(...)`, `listStyles()`.
  - MCP: `stylize_text` and `list_styles`. **No usage limit**, and they do not use up puzzles or fills.
- **MCP free tiers:** puzzles up to **10×10 with at most 10 words are free and unlimited** (never counted). Bigger grids, or more than
  10 words, use the 5 free puzzles per client (a lifetime total, as before). `fill_word_search` keeps its separate limit of 100.
  `stylize_text`, `list_styles` and `list_languages` have no limit. Invalid requests still do not count.
- The engine is 0.4.0 (`engineVersion`); puzzles and fills for the same seed are identical to 0.3.0.

## 0.3.0
- **Blocked words in the filler:** `fill` (npm `fill()`, CLI `--random`, Kotlin `Lettras.fill()`, MCP `fill_word_search`) no longer completes an
  unwanted word: the engine carries a bank of about 30,600 blocked words from all six languages and rewrites any filler cell
  that would spell one, in all 8 directions (a 30x30 scan takes about 3 ms; nothing is built at startup). It is on by default; `avoidBlocked: false`
  turns it off and `blocked: [...]` adds your own words. `blockedLeft` in the result lists blocked words that still read in
  the grid (only when there are any, normally a word you hid yourself). Fills for the same seed can differ from 0.2.x
  wherever the old filler would have formed a blocked word.
- **New:** `findBlocked({ grid, blocked? })` returns every blocked word that reads in a grid, with position and direction.
- **Brand:** the Lettras logos, icons and social images are in `assets/` (with usage rules); the READMEs show the logo and
  `server.json` lists the icon for the MCP Registry.
- **MCP:** the free limit of 5 puzzles is now a lifetime total per client (the default window is 0 = never resets), not 5 per day.
  `LETTRAS_LIMIT_WINDOW_SECS=86400` brings back a daily limit for your own deployment. Messages and docs no longer say "per day".

## 0.2.2 (npm only)
- **Fix:** `npm install lettras` could not be built by bundlers (webpack, Turbopack, and likely Vite): the loader contained a
  dead `new URL('lettras_engine_bg.wasm', import.meta.url)` whose file is not shipped, so the build failed with
  `Can't resolve 'lettras_engine_bg.wasm'`. Plain Node was not affected, which is why the tests missed it. Verified with a real
  webpack build against 0.2.1 (fails with that exact error) and 0.2.2 (builds). Kotlin/Maven and the MCP server do not use this
  loader and stay at 0.2.1.

## 0.2.1
- **Fix:** `engineVersion` now reports the real engine version (it said `0.1.0` since the first release).
- **MCP:** IPv6 addresses in the same `/64` count as one client, and IPv4-mapped IPv6 as IPv4, so rotating addresses no longer
  dodges the free limit. `fill_word_search` has its own limit (100 per day). Usage counters are now actually cleaned up (the
  cleanup condition could never be true, so the table would have grown forever). Store failures are logged as a
  `usage_store_error` line with a safe category, never connection details.
- **Engine hardening:** limits on grid size, word count, word length and cell length; a letter or digit as the empty-cell
  marker is rejected (it made grids ambiguous); a fuzz test with 100,000 hostile inputs finds no crash.
- **Docs:** filler example with real output, limits, privacy notes, Android status, ESM note.

## 0.2.0
- `fill`: random letters for the empty cells, by language, with accents on or off (npm `fill()` and CLI `--random`, Kotlin
  `Lettras.fill()`, MCP tool `fill_word_search`).
- Kotlin library published to Maven Central; MCP listed in the official MCP Registry; shared Postgres counter for the free tier.

## 0.1.0
- npm library and CLI, Kotlin library, Rust MCP server (`generate_word_search`, `list_languages`), 5 free puzzles per day.
