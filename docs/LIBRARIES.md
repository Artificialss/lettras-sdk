# Libraries

One compiled engine, many ways to call it: ten libraries and three tools. Every library runs the engine **in-process** (WebAssembly): no network, no API key, no hosted service. The same input and `seed` give
the same puzzle in every language, on every platform.

| Language | Package | Install | Runtime that runs the engine | Platforms |
| --- | --- | --- | --- | --- |
| JavaScript / TypeScript | `lettras` (npm) | `npm install lettras` | the platform's own WebAssembly | Node, browsers, Deno, Bun, edge runtimes |
| Kotlin | `org.lettras.artificialss:lettras` (Maven Central) | `implementation("org.lettras.artificialss:lettras:<version>")` | Chicory | JVM and Android |
| Java | the Kotlin package | same coordinates | Chicory | JVM 17+ |
| Rust | `lettras` (crates.io) | `cargo add lettras` | wasmi | everything Rust builds for |
| Go | `github.com/Artificialss/lettras-sdk/go` | `go get github.com/Artificialss/lettras-sdk/go` | wazero (pure Go, no cgo) | every `GOOS`/`GOARCH` |
| Dart · Flutter | `lettras` (pub.dev) | `dart pub add lettras` | wasm_run (wasmi), needs `wasm_run_flutter` in Flutter apps | Dart VM; Flutter through `wasm_run_flutter` |
| Swift | `Lettras` (Swift Package Manager) | add `https://github.com/Artificialss/lettras-sdk.git` | WasmKit (pure Swift) | macOS, iOS, tvOS, watchOS, visionOS, Linux |
| .NET (C#, F#) | `Lettras` (NuGet) | `dotnet add package Lettras` | Wasmtime | .NET 8+ on Windows, macOS, Linux |
| MCP server | `org.lettras/word-search` | `https://mcp.lettras.org/mcp` | wasmi, on Vercel | any MCP client |
| Command line | `lettras` (npm CLI) | `npx lettras --words ...` | WebAssembly | Node |
| Command line | Homebrew formula | `brew install artificialss/lettras/lettras` | Node, via the npm package | macOS, Linux |
| VS Code | `artificialss.lettras` | VS Code Marketplace | the npm package, bundled | VS Code 1.85+ |
| JetBrains IDEs | `ai.artificialss.lettras` (name Lettras) | JetBrains Marketplace ([upload guide](JETBRAINS_MARKETPLACE.md)) | the Kotlin library | IntelliJ Platform 2024.1+ |

Registries: npm, Maven Central and the MCP Registry are live. The Rust, Go, Swift, .NET and Dart packages, the editor extensions and the Homebrew formula are in this repository and are being published to crates.io, pkg.go.dev, the Swift
Package Index, NuGet, pub.dev, the two marketplaces and a Homebrew tap; until they are listed, depend on this repository (Go and Swift resolve it by Git tag).

## What every library offers

| Operation | Does | npm | Kotlin / Java | Rust | Go | Swift | .NET |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Word search | Hide words in a grid, any of 6 languages | `generate` | `generate` | `generate` | `Generate` | `generate` | `Generate` |
| Fill | Random letters in the empty cells, language-aware | `fill` | `fill` | `fill` | `Fill` | `fill` | `Fill` |
| Text styles | 22 Unicode styles that keep accents | `stylize`, `unstyle`, `listStyles` | `stylize`, `unstyle`, `listStyles` | `stylize`, `styles` | `Stylize`, `Styles` | `stylize`, `styles` | `Stylize`, `Styles` |
| Crossword | Layout with numbers, clues and a solution | `crossword` | `crossword` | `crossword` | `Crossword` | `crossword` | `Crossword` |
| Sudoku | One unique solution, 4 to 16 cells | `sudoku` | `sudoku` | `sudoku` | `Sudoku` | `sudoku` | `Sudoku` |
| Solve | Solve a sudoku, say if the solution is unique | `solveSudoku` | `solveSudoku` | `solve_sudoku` | `SolveSudoku` | `solveSudoku` | `SolveSudoku` |

`unstyle` and `findBlocked` exist in the npm and Kotlin libraries only; the Rust, Go, Swift, .NET and Dart libraries expose the operations above (Dart uses camelCase names like Swift).

## Options (the same everywhere)

Names are camelCase JSON in every language; typed wrappers (npm, Kotlin) map them to their own types.

- **Word search:** `words`, `rows`, `cols`, `position` (`horizontal`, `vertical`, `mixed`), `difficulty` 1 to 4, `clustering` 0 to 1, `seed`, `lang` (`es` default, `en` `pt` `fr` `de` `it`), `classicMode`, `fill`.
- **Fill:** `grid` (or the puzzle), `lang`, `accents`, `seed`, `words`. Without a `seed` every call differs.
- **Crossword:** `entries` (`word` and `clue`), `rows`, `cols`, `seed`. Offer more words than you need: the engine keeps the best layout.
- **Sudoku:** `size` (4, 6, 9, 12, 16), `difficulty` 1 to 4, `seed`. `0` is an empty cell.
- **Styles:** `text`, `style` (one id or a list), `classic`.

Limits: grids up to 100×100, 500 words of up to 100 characters, text styles up to 10,000 characters. Bad input comes back as a readable error, never a crash.

## Errors

| Language | Bad input | The engine could not run |
| --- | --- | --- |
| npm | throws `Error` | throws `Error` |
| Kotlin / Java | `LettrasException` | `LettrasException` |
| Rust | `Error::Input(message)` | `Error::Engine(message)` |
| Go | `*lettras.InputError` | a plain `error` |
| Dart | `LettrasInputException` | `LettrasEngineException` |
| Swift | `Lettras.Failure.input(message)` | `Lettras.Failure.engine(message)` |
| .NET | `LettrasInputException` | `LettrasEngineException` |

The message says what to change (for example "rows and cols must be positive integers").

## Same puzzle in every language

```text
words gato, perro, piña · 9 rows × 12 cols · position mixed · seed 8 · lang es
```

All libraries return the same grid, placements and `engineVersion` for that input. The repository tests assert it between npm, Kotlin and the MCP server; the Rust, Go, Swift and .NET libraries embed
the very same compiled engine file as the MCP server (byte-identical).

## Safety and performance

- **Fresh instance per call:** no state survives between calls, so concurrent callers cannot affect each other. Go's `Engine` and .NET's `LettrasEngine` are safe to share between threads; Swift's serialises calls.
- **CPU budget:** Rust (wasmi) and .NET (Wasmtime) cut a call off with a fuel budget; Go uses a 20-second timeout (`lettras.Timeout`); Swift (WasmKit) has neither, so keep inputs within the limits above.
- **Debug builds are slow** for WebAssembly interpreters (Swift, Rust). Generate in release builds or off the main thread for large puzzles.

## Licensing

The wrappers are MIT. The compiled engine inside each library is proprietary: free to use unmodified, as distributed, in your own applications (commercial ones included); you may not
extract, redistribute, modify or reverse-engineer it, or use it to build a competing puzzle service. See the `LICENSE` and engine license file in each library folder.

## Per-library pages

[npm](../npm/README.md) · [Kotlin and Java](../kotlin/README.md) · [Rust](../rust/README.md) · [Go](../go/README.md) · [Swift](../swift/README.md) · [.NET](../dotnet/README.md) · [Dart](../dart/README.md) · [VS Code](../vscode/README.md) · [JetBrains](../jetbrains/README.md) · [Homebrew](../homebrew/README.md) · [MCP server](../README.md#mcp-server)
