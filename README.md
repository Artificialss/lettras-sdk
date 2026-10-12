<div align="center">

  <img alt="Lettras" src="assets/svg/lettras-logo.svg" width="300">

  # Lettras SDK

  **Word-search puzzles and Unicode text styles in six languages, with every accent intact.**

</div>

<p align="center">
  <a href="https://www.npmjs.com/package/lettras"><img alt="npm" src="https://img.shields.io/npm/v/lettras?label=npm&color=cb3837"></a>
  <a href="https://central.sonatype.com/artifact/org.lettras.artificialss/lettras"><img alt="Maven Central" src="https://img.shields.io/maven-central/v/org.lettras.artificialss/lettras?label=maven%20central&color=orange"></a>
  <a href="https://registry.modelcontextprotocol.io/v0.1/servers?search=org.lettras/word-search"><img alt="MCP Registry" src="https://img.shields.io/badge/mcp%20registry-org.lettras%2Fword--search-informational.svg"></a>
  <img alt="Languages" src="https://img.shields.io/badge/languages-es%20·%20en%20·%20pt%20·%20fr%20·%20de%20·%20it-blue.svg">
  <img alt="Rust" src="https://img.shields.io/badge/mcp-rust-orange.svg">
  <img alt="MCP" src="https://img.shields.io/badge/protocol-MCP-informational.svg">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT%20%2B%20compiled%20engine-lightgrey.svg"></a>
</p>

Developer tools for [Lettras](https://lettras.org), the free word-puzzle platform. Generate word searches from any
word list in **Spanish, English, Portuguese, French, German and Italian**, with native letters (Ñ, Ç, Ã, Ä, ẞ, È…)
kept as one cell each, from your own code, a terminal, or an AI assistant.

| Tool | What it is | Install / connect | Source |
| --- | --- | --- | --- |
| **`lettras` on npm** | JavaScript/TypeScript library and CLI. Runs locally, no network or API key | `npm install lettras` ([npm](https://www.npmjs.com/package/lettras)) | [`npm/`](npm) |
| **`lettras` for Kotlin** | Kotlin library for the JVM and Android. Same engine, runs locally | `org.lettras.artificialss:lettras` ([Maven Central](https://central.sonatype.com/artifact/org.lettras.artificialss/lettras)) | [`kotlin/`](kotlin) |
| **Lettras MCP server** | Lets Claude and other MCP clients create puzzles. Written in Rust, runs on Vercel | `https://mcp.lettras.org/mcp` ([MCP Registry](https://registry.modelcontextprotocol.io/v0.1/servers?search=org.lettras/word-search)) | [`mcp/`](mcp) |

> **Published.** All three are live: the npm package, the Kotlin library on Maven Central, and the hosted MCP server
> (also listed in the official MCP Registry). A documentation site is planned.

## Table of contents

- [npm package](#npm-package)
- [Kotlin library](#kotlin-library)
- [MCP server](#mcp-server)
- [Text styles](#text-styles)
- [Crossword and sudoku (coming in 0.5.0)](#crossword-and-sudoku-coming-in-050)
- [How puzzles are built](#how-puzzles-are-built)
- [Architecture and licensing](#architecture-and-licensing)
- [Repository layout](#repository-layout)
- [Development](#development)
- [License](#license)
- [About](#about)

## npm package

```bash
npm install lettras
```

```js
import { generate, render } from 'lettras';

const puzzle = generate({
  words: ['gato', 'perro', 'piña', 'mono', 'cebra'],
  rows: 9,
  cols: 12,
  position: 'mixed', // horizontal | vertical | mixed (all 8 directions)
  seed: 8,           // same input and seed, same grid
});

console.log(render(puzzle));
```

```
9×12  seed 8  engine 0.4.0

- - - - - - - - - - - -
- - - - - - - - - - - -
- - - - C - - - - - - -
- - - - - E - O N O M -
- - A - - - B - R - - -
O - - Ñ - - - R - - - -
T - - - I - E - A - - -
A - - - - P - - - - - -
G - - - - - - - - - - -

Words: gato, perro, piña, mono, cebra
```

`piña` takes four cells (the `Ñ` is one), `mono` is written right to left, and `perro` and `cebra` run on diagonals.
The grid is plain data: `puzzle.grid` is an array of rows, each an array of one-letter strings.

**CLI**

```bash
npx lettras --words gato,perro,piña --rows 9 --cols 12 --position mixed --seed 8
npx lettras --words sol,luna,mar --rows 8 --json          # raw JSON
npx lettras --words sol,luna,mar --rows 8 --solution      # only the hidden words
```

**Options**

| Option | Values |
| --- | --- |
| `words` | Words to hide, in normal spelling. Accents are kept. |
| `rows`, `cols` | Grid size. They can differ for a rectangular grid. |
| `position` | `horizontal` (left→right), `vertical` (top→bottom), `mixed` (all 8 directions, diagonals included). |
| `difficulty` | `1`–`4`. Sets the directions when `position` is not given. |
| `clustering` | `0` words apart · `1` words crossing · default `0.5`. |
| `seed` | Repeatable puzzles. |
| `lang` | `es` (default) `en` `pt` `fr` `de` `it`. |
| `classicMode` | Strip accents in the grid (`ñ` becomes `N`). |
| `fill` | Character for empty cells. Default `-`. |

**Fill the empty cells with random letters.** `generate` leaves empty cells as `-`. Pass the result to `fill`, with the
language and accents on or off:

```js
import { generate, fill } from 'lettras';

const puzzle = generate({ words: ['gato', 'perro', 'piña'], rows: 9, cols: 12, position: 'mixed', seed: 8 });
const done = fill(puzzle, { lang: 'es', accents: true });   // or: fill(puzzle.grid, { lang: 'es', accents: false })
done.grid;   // the same matrix, every "-" replaced by a random letter
```

| `fill` option | Meaning |
| --- | --- |
| `lang` | `es` (default) `en` `pt` `fr` `de` `it`. Letters follow how common they are in that language. |
| `accents` | `true` (default): include the language's accented/native letters (Ñ, Ç, Ã, Ä, ẞ…). `false`: plain A-Z only. |
| `seed` | Repeatable filler. Without one, every call gives different letters. |
| `words` | The hidden words. Added automatically when you pass the puzzle; the filler never creates an extra copy of one. |

On the CLI: `lettras --words gato,piña --rows 9 --random --accents off`.

**Example.** The same puzzle before and after `fill` (Spanish, seed 5):

```js
const puzzle = generate({ words: ['gato', 'perro', 'piña', 'mono', 'cebra'], rows: 8, cols: 10, position: 'mixed', seed: 8 });
const done = fill(puzzle, { lang: 'es', accents: true, seed: 5 });
```

`puzzle.grid` has `-` in every empty cell; `done.grid` replaces them (58 cells here):

```
C E B R A - O - - -
- - - - - - - T O -
- - - - M - - R A -
- - - O - - R - - G
- - N - - E - - - -
- O - - P - - - - -
- - - - - - - - - -
- - - - - - A Ñ I P
```
```
C E B R A R O G G J
D R A E E R C T O D
B Ó E E M Í M R A Ñ
T S C O N R R A E G
T R N E A E R O E M
I O O E P S N P A O
O C O O A P P A U A
I R E A A S A Ñ I P
```

With `accents: false` the filler uses plain A-Z only (the one `Ñ` left is the hidden word *piña*, which keeps its spelling):

```
C E B R A R O G G I
D Q A E E P C T O D
B Y E E M X M R A N
S R C O N R R A E G
S R N E A E R O E M
I O O E P S N O A O
O B O O A P O A T A
H R E A A S A Ñ I P
```

Same grid, language, accents and `seed` always give the same filler on every platform. Without a `seed` the wrappers
pick a random one, so every call gives different letters.

**Result:** `grid`, `placements` (`word`, start `r`/`c`, step `dr`/`dc`, `length`), `words` (placed), `unplaced`
(words that did not fit, never dropped silently), `rejected` (with reasons), `seed`, `engineVersion`.
TypeScript types are included. Full notes: [`npm/README.md`](npm/README.md).

## Kotlin library

For JVM and Android apps (and the mobile game). Same engine, same results, no network.

```kotlin
val lettras = Lettras()   // create once, reuse
val puzzle = lettras.generate(
    PuzzleRequest(words = listOf("gato", "perro", "piña"), rows = 9, cols = 12, position = Position.MIXED, seed = 8),
)
puzzle.grid        // List<List<String>>, one letter per cell
puzzle.placements  // where each word is hidden
puzzle.render()    // text view

val done = lettras.fill(puzzle, lang = Language.ES, accents = true)   // empty cells become random letters
done.grid
```

The engine is the same WebAssembly binary the MCP server uses, run by [Chicory](https://github.com/dylibso/chicory)
(pure Java, no native libraries to build per CPU). On the JVM it is compiled to bytecode; on Android it uses the
interpreter. The tests check it returns exactly the grids the npm package returns. Install notes, API, performance
and Android details: [`kotlin/README.md`](kotlin/README.md).

## MCP server

An [MCP](https://modelcontextprotocol.io) server that lets an AI assistant create puzzles for you. Ask for
*"a 12×12 sopa de letras about animals, mixed directions, with random letters"* and it calls `generate_word_search`, then `fill_word_search`. It can also write text in Unicode styles with `stylize_text`.

- **Transport:** streamable HTTP, `POST /mcp` (JSON-RPC). Protocol versions 2025-06-18, 2025-03-26, 2024-11-05.
- **Runs the engine locally:** the compiled engine is embedded in the server and executed in-process. It makes no
  outside calls and needs no credentials to start.
- **Written in Rust** on Vercel's [Rust runtime](https://vercel.com/docs/functions/runtimes/rust) (axum).

**Connect a client** (hosted endpoint: `https://mcp.lettras.org/mcp`):

```bash
# Claude Code
claude mcp add --transport http lettras https://mcp.lettras.org/mcp
```

```json
// Claude Desktop, Cursor and other clients that take a remote server URL
{ "mcpServers": { "lettras": { "url": "https://mcp.lettras.org/mcp" } } }
```

**Tools**

| Tool | Purpose |
| --- | --- |
| `generate_word_search` | Create a puzzle. Arguments match the [options above](#npm-package) (`words`, `rows`, `cols`, `position`, `difficulty`, `clustering`, `seed`, `lang`, `classicMode`). Returns a readable grid and the full structured result. |
| `fill_word_search` | Complete a puzzle: takes the grid from `generate_word_search` (empty cells `-`) and fills them with random letters in the chosen `lang`, with `accents` on or off. Pass the puzzle's `words` so the filler never creates an extra copy. Does not use up the puzzle limit; it has its own, larger one. |
| `stylize_text` | Text in Unicode styles (bold, italic, script, fraktur, circled, strikethrough... 22 in all) that **keep accents**. `style` is one id, several, or omitted for all; `classic` strips accents first. Reports in `notNative` and `parts[].missing` what a style cannot draw (ß, Æ, Œ, Ø, non-Latin scripts, accents that only exist as combining marks), so you can warn the user. **No usage limit.** |
| `list_styles` | The style ids `stylize_text` accepts, with an English name and a category. No usage limit. |
| `list_languages` | The supported languages and the native letters each adds to its grid. |

Invalid arguments come back as a tool error written for the model to act on (for example
`rows: an integer from 6 to 30 is required`).

**Free tiers**

| What | Free use |
| --- | --- |
| `generate_word_search`, **small**: up to **10×10** with **at most 10 words** | **Unlimited.** Never counted. |
| `generate_word_search`, **larger**: a bigger grid (up to 30×30) or more than 10 words | **5 puzzles per client, in total, forever** (there is no daily reset). |
| `fill_word_search` | 100 calls per client, separate from the puzzles. |
| `stylize_text`, `list_styles`, `list_languages` | **No limit.** |

After a limit the tool answers with *"Free limit reached… Visit https://lettras.org"*. Invalid requests never count.

**Limits and privacy.** A client is identified by a SHA-256 hash of its IP address, with the
whole IPv6 `/64` treated as one client so that rotating addresses does not dodge the limit. Because the limit never
resets, the server keeps one row per client indefinitely: the hash, a count and a timestamp (no addresses, no words, no
puzzles). The request log carries the same hash prefix, never the address. A hash of an IP address can still count as
personal data in some jurisdictions, so mention it in your own privacy notice if you operate your own copy. If the
database is unreachable the request is allowed (and a `usage_store_error` line is logged), so a storage outage never takes
the service down. The hosted endpoint is public and unauthenticated; small puzzles, styles and languages are free to use, while bigger puzzles are a free trial of five, not a bulk
service. To run your own copy with a daily limit instead, set `LETTRAS_LIMIT_WINDOW_SECS=86400`.

**Deploy on Vercel**

1. Create a Vercel project from this repository and set **Root Directory** to `mcp`.
2. Optional but recommended: a Postgres database so the 5-puzzle counter is shared across instances (without it the
   counter lives in each instance's memory and is not reliable). Create a [Neon](https://neon.tech) project, copy its
   **pooled** connection string into the project's environment variables as `DATABASE_URL` (mark it sensitive). The
   server creates its own `mcp_usage` table on first use; it stores a hash, a count and a timestamp per client.
3. Optional settings, all in [`.env.example`](.env.example): `LETTRAS_FREE_LIMIT`, `LETTRAS_FILL_LIMIT`,
   `LETTRAS_LIMIT_WINDOW_SECS` (0 = lifetime total, the default), `LETTRAS_UPGRADE_URL`.
4. Optional: add your own domain under the project's Settings → Domains (ours is `mcp.lettras.org`, a `CNAME` named `mcp` pointing to `cname.vercel-dns.com`; do not change the domain's nameservers).
5. Deploy. Every puzzle request writes one JSON line to the function logs (hashed client, count, allowed or blocked,
   grid size, language), which you can read in Vercel Logs or forward with a log drain.

The server is described for MCP registries in [`server.json`](server.json); publishing steps are in
[`docs/REGISTRY.md`](docs/REGISTRY.md). Publishing the Kotlin library to Maven Central:
[`docs/MAVEN_CENTRAL.md`](docs/MAVEN_CENTRAL.md).

No secret is stored in this repository. Real values belong in Vercel's project settings.

## Text styles

Besides puzzles, the same engine writes text in Unicode styles that keep accents (22 styles). It is what powers Social Media Styles at
[lettras.org](https://lettras.org).

```js
import { stylize, unstyle, listStyles } from 'lettras';

const out = stylize('Lettras ejemplo: piña, corazón', { style: ['bold', 'script'] });
out.results[0].text;      // 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 𝐞𝐣𝐞𝐦𝐩𝐥𝐨: 𝐩𝐢𝐧̃𝐚, 𝐜𝐨𝐫𝐚𝐳𝐨́𝐧   (ñ = styled n + combining tilde)
out.results[0].notNative; // ['ñ', 'ó']: accents that exist in no style as one character
unstyle(out.results[0].text); // 'Lettras ejemplo: piña, corazón'
listStyles();             // [{ id: 'bold', label: 'Bold', category: 'bold-italic' }, ...]
```

`style` is one id, an array of ids, or omitted for all 22. `classic: true` strips accents first. `parts` cuts the output into pieces,
and `missing: true` marks what a style cannot draw (ß, Æ, Œ, Ø, non-Latin scripts), so a UI can show an empty box like a phone without
that glyph. Text is limited to 10,000 characters. The same functions exist in Kotlin (`stylize`, `unstyle`, `listStyles`) and as the MCP tools
`stylize_text` and `list_styles`.

### The name in every style

`stylize('Lettras')` (real output). The list is in display order, which is also the order of `listStyles()`.

| Style id | Name | Lettras |
| --- | --- | --- |
| `bold` | Bold | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 |
| `italic` | Italic | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 |
| `bold-italic` | Bold italic | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 |
| `sans` | Sans-serif | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 |
| `sans-bold` | Sans-serif bold | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 |
| `sans-italic` | Sans-serif italic | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 |
| `sans-bold-italic` | Sans-serif bold italic | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 |
| `script` | Script | ℒℯ𝓉𝓉𝓇𝒶𝓈 |
| `bold-script` | Bold script | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 |
| `fraktur` | Fraktur | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 |
| `bold-fraktur` | Bold fraktur | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 |
| `double-struck` | Double-struck | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 |
| `monospace` | Monospace | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 |
| `small-caps` | Small caps | ʟᴇᴛᴛʀᴀꜱ |
| `fullwidth` | Fullwidth | Ｌｅｔｔｒａｓ |
| `circled` | Circled | Ⓛⓔⓣⓣⓡⓐⓢ |
| `negative-circled` | Negative circled | 🅛🅔🅣🅣🅡🅐🅢 |
| `squared` | Squared | 🄻🄴🅃🅃🅁🄰🅂 |
| `negative-squared` | Negative squared | 🅻🅴🆃🆃🆁🅰🆂 |
| `parenthesized` | Parenthesized | 🄛⒠⒯⒯⒭⒜⒮ |
| `strikethrough` | Strikethrough | L̶e̶t̶t̶r̶a̶s̶ |
| `underline` | Underline | L̲e̲t̲t̲r̲a̲s̲ |

### Examples

Every style applied to `Lettras ejemplo: piña` (real output of `stylize`). The accent in `piña` is the styled `n` plus a combining
tilde, so how it looks depends on the font; the decoration styles (strikethrough, underline) mark every character, accents included.

| Style id | Result |
| --- | --- |
| `bold` | 𝐋𝐞𝐭𝐭𝐫𝐚𝐬 𝐞𝐣𝐞𝐦𝐩𝐥𝐨: 𝐩𝐢𝐧̃𝐚 |
| `italic` | 𝐿𝑒𝑡𝑡𝑟𝑎𝑠 𝑒𝑗𝑒𝑚𝑝𝑙𝑜: 𝑝𝑖𝑛̃𝑎 |
| `bold-italic` | 𝑳𝒆𝒕𝒕𝒓𝒂𝒔 𝒆𝒋𝒆𝒎𝒑𝒍𝒐: 𝒑𝒊𝒏̃𝒂 |
| `sans` | 𝖫𝖾𝗍𝗍𝗋𝖺𝗌 𝖾𝗃𝖾𝗆𝗉𝗅𝗈: 𝗉𝗂𝗇̃𝖺 |
| `sans-bold` | 𝗟𝗲𝘁𝘁𝗿𝗮𝘀 𝗲𝗷𝗲𝗺𝗽𝗹𝗼: 𝗽𝗶𝗻̃𝗮 |
| `sans-italic` | 𝘓𝘦𝘵𝘵𝘳𝘢𝘴 𝘦𝘫𝘦𝘮𝘱𝘭𝘰: 𝘱𝘪𝘯̃𝘢 |
| `sans-bold-italic` | 𝙇𝙚𝙩𝙩𝙧𝙖𝙨 𝙚𝙟𝙚𝙢𝙥𝙡𝙤: 𝙥𝙞𝙣̃𝙖 |
| `script` | ℒℯ𝓉𝓉𝓇𝒶𝓈 ℯ𝒿ℯ𝓂𝓅𝓁ℴ: 𝓅𝒾𝓃̃𝒶 |
| `bold-script` | 𝓛𝓮𝓽𝓽𝓻𝓪𝓼 𝓮𝓳𝓮𝓶𝓹𝓵𝓸: 𝓹𝓲𝓷̃𝓪 |
| `fraktur` | 𝔏𝔢𝔱𝔱𝔯𝔞𝔰 𝔢𝔧𝔢𝔪𝔭𝔩𝔬: 𝔭𝔦𝔫̃𝔞 |
| `bold-fraktur` | 𝕷𝖊𝖙𝖙𝖗𝖆𝖘 𝖊𝖏𝖊𝖒𝖕𝖑𝖔: 𝖕𝖎𝖓̃𝖆 |
| `double-struck` | 𝕃𝕖𝕥𝕥𝕣𝕒𝕤 𝕖𝕛𝕖𝕞𝕡𝕝𝕠: 𝕡𝕚𝕟̃𝕒 |
| `monospace` | 𝙻𝚎𝚝𝚝𝚛𝚊𝚜 𝚎𝚓𝚎𝚖𝚙𝚕𝚘: 𝚙𝚒𝚗̃𝚊 |
| `small-caps` | ʟᴇᴛᴛʀᴀꜱ ᴇᴊᴇᴍᴘʟᴏ: ᴘɪɴ̃ᴀ |
| `fullwidth` | Ｌｅｔｔｒａｓ ｅｊｅｍｐｌｏ： ｐｉｎ̃ａ |
| `circled` | Ⓛⓔⓣⓣⓡⓐⓢ ⓔⓙⓔⓜⓟⓛⓞ: ⓟⓘⓝ̃ⓐ |
| `negative-circled` | 🅛🅔🅣🅣🅡🅐🅢 🅔🅙🅔🅜🅟🅛🅞: 🅟🅘🅝̃🅐 |
| `squared` | 🄻🄴🅃🅃🅁🄰🅂 🄴🄹🄴🄼🄿🄻🄾: 🄿🄸🄽̃🄰 |
| `negative-squared` | 🅻🅴🆃🆃🆁🅰🆂 🅴🅹🅴🅼🅿🅻🅾: 🅿🅸🅽̃🅰 |
| `parenthesized` | 🄛⒠⒯⒯⒭⒜⒮ ⒠⒥⒠⒨⒫⒧⒪: ⒫⒤⒩̃⒜ |
| `strikethrough` | L̶e̶t̶t̶r̶a̶s̶ e̶j̶e̶m̶p̶l̶o̶:̶ p̶i̶ñ̶a̶ |
| `underline` | L̲e̲t̲t̲r̲a̲s̲ e̲j̲e̲m̲p̲l̲o̲:̲ p̲i̲ñ̲a̲ |

**What a style cannot draw.** `ñ`, `ó`, `ü`... are flagged in `notNative` for every letter style, because the accent is a combining mark. `ß`, `Æ`, `Œ`, `Ø` and non-Latin scripts have no styled version and stay plain:

```js
stylize('Straße', { style: 'script' }).results[0].parts;
// [ { text: '𝒮𝓉𝓇𝒶', missing: false }, { text: 'ß', missing: true }, { text: 'ℯ', missing: false } ]
// show the missing piece as an empty box, or warn the user
```

**Classic mode** strips accents first, so nothing is flagged:

```js
stylize('piña', { style: 'bold', classic: true }).results[0];
// { id: 'bold', text: '𝐩𝐢𝐧𝐚', notNative: [], ... }
```

**One style, several styles, or all** (`results` is always an array):

```js
stylize('Hola', { style: 'bold' }).results.length;              // 1
stylize('Hola', { style: ['bold', 'script'] }).results.length;  // 2
stylize('Hola').results.length;                                 // 22
```

**Kotlin and MCP.** `lettras.stylize("piña", style = "bold")` returns the same results; in an MCP client, ask for *"write 'Lettras ejemplo' in bold and script"*
and the assistant calls `stylize_text` with `{ "text": "Lettras ejemplo", "style": ["bold", "script"] }`.

## Crossword and sudoku (coming in 0.5.0)

Two more generators from the same engine, in the next release:

- **Crossword (crucigrama):** words and clues in, a connected crossword out. The result has a `puzzle` matrix (the clue number on the
  first cell of each word, `0` on the other letter cells, `-` for blanks, and `a/d` such as `1/2` when an across and a down word start in
  the same cell) and a `solution` matrix with the letters. See [docs/CROSSWORD.md](docs/CROSSWORD.md).
- **Sudoku:** puzzles with a single solution (4, 6, 9, 12 or 16 wide, difficulty 1 to 4) and a solver. See [docs/SUDOKU.md](docs/SUDOKU.md).

## How puzzles are built

1. **Normalize.** Each word becomes one grapheme per cell, uppercase, with `ß` → `ẞ` (a plain uppercase would give
   `SS`). Words with spaces, digits or punctuation, duplicates, and words contained in another word are rejected and
   reported with a reason.
2. **Place longest first.** Every spot that fits is considered: inside the grid, on empty cells or on cells holding the
   same letter.
3. **Choose the spot.** The least-used direction goes first, so diagonals and reversed words really appear. Each word
   then either crosses existing words or sits apart from them; `clustering` sets the odds.
4. **Verify.** Every word must appear exactly once, otherwise the engine retries with the next seed. A word that cannot
   be placed is returned in `unplaced`.
5. **Fill** (a separate call). Empty cells get random letters, weighted by how common each letter is in the chosen
   language, with accents on or off. About 30% copy letters already in the grid so rare letters do not give the hidden
   words away, and any accidental extra copy of a hidden word is repaired.

Deterministic: the same input and seed give the same grid everywhere, on every platform.

## Architecture and licensing

```
   Lettras engine (Rust, proprietary source, private repository)
              │  compiled, stripped, size-optimised
      ┌───────┴──────────────────────────────┐
      ▼                                      ▼
 npm/engine/                       mcp/engine/ (and kotlin/ resources)
 WebAssembly + loader              WebAssembly (C ABI)
 used by the npm package           MCP server (wasmi), Kotlin library (Chicory)
```

This repository contains the **wrappers, CLI, MCP server, tests and documentation**. The puzzle engine itself is
developed in a private repository and ships here only as compiled WebAssembly in `npm/engine/`, `mcp/engine/` and
`kotlin/src/main/resources/`.
All copies come from the same build and are checked against each other: the MCP server's and the Kotlin library's
tests assert that they return exactly the same grids as the npm package for the same input.

## Repository layout

```
npm/                 the `lettras` package: library, CLI, types, tests
  engine/            compiled engine (proprietary, see engine/LICENSE)
kotlin/              Kotlin library (JVM and Android): API, Chicory host, tests
  src/main/resources compiled engine (proprietary, see LICENSE-ENGINE)
assets/              brand kit: logos, icons and social images (see assets/README.md)
mcp/                 MCP server (Rust): protocol, tools, free-tier gate, Vercel entry point
  engine/            compiled engine (proprietary, see engine/LICENSE)
.env.example         every optional setting, with no values
```

## Development

```bash
# npm package
cd npm && npm test

# Kotlin library (JDK 17)
cd kotlin && ./gradlew test         # 26 tests, including exact parity with the npm package

# MCP server (Rust 1.80+)
cd mcp && cargo test --release      # 30 tests, including exact parity with the npm package
# counter against a real Postgres (optional):
#   TEST_DATABASE_URL=postgres://... cargo test --release --test pg -- --ignored
```

The compiled engine is refreshed from the private repository by its maintainers; pull requests that change
`engine/` cannot be accepted. Issues and suggestions about the wrappers, the MCP server or the docs are welcome.

## Limits of the engine

The engine accepts grids up to 100×100, 500 words of up to 100 characters and cells of up to 8 characters; the MCP server and the
public examples use 6-30. The empty-cell marker (`fill` when generating, `empty` when filling) must be one character that is
not a letter or digit. Anything else returns a readable error. A fuzz test in the engine's repository throws 100,000 hostile
inputs (combining marks, emoji, right-to-left text, zero-width joiners, garbage JSON) at it with no crash.

## License

The code in this repository is released under the [MIT License](LICENSE), **except** the compiled engine in
`npm/engine/`, `mcp/engine/` and `kotlin/src/main/resources/org/lettras/`, which is proprietary and covered by its own license (`engine/LICENSE`): you may use it
unmodified, through this package or server, in your own products, including commercial ones, but you may not extract,
redistribute, modify or reverse-engineer it.

## About

Logos and brand rules: [`assets/`](assets/README.md).

Lettras is built by **[Artificialss](https://artificialss.ai)**.

---

<p align="center">Built by <a href="https://artificialss.ai">Artificialss</a></p>
