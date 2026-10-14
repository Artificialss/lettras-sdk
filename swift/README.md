# Lettras for Swift

Word-search, crossword (crucigrama) and sudoku generators, plus Unicode text styles, for **Spanish, English, Portuguese, French, German and Italian**, with native
accented letters kept (one cell each). The engine runs in-process: no network, no API key. macOS, iOS, tvOS, watchOS, visionOS and Linux.

Swift Package Manager (Xcode: File > Add Package Dependencies, or `Package.swift`):

```swift
.package(url: "https://github.com/Artificialss/lettras-sdk.git", from: "0.5.2")
// target dependency: .product(name: "Lettras", package: "lettras-sdk")
```

```swift
import Lettras

let lettras = try Lettras()

let puzzle = try lettras.generate(["words": ["gato", "perro", "piña"], "rows": 9, "cols": 12, "position": "mixed", "seed": 8])
let done = try lettras.fill(["grid": puzzle["grid"]!, "lang": "es", "accents": true])   // random letters in the empty cells
print(done["grid"]!)

let sudoku = try lettras.sudoku(["size": 9, "difficulty": 2, "seed": 4])                // exactly one solution, 0 = empty
let solved = try lettras.solveSudoku(["puzzle": sudoku["puzzle"]!])                     // solution, solved, unique
let crossword = try lettras.crossword(["entries": [["word": "gato", "clue": "Felino"], ["word": "toro", "clue": "Bovino"]], "rows": 10, "cols": 10])
let bold = try lettras.stylize(["text": "Ñandú", "style": "bold"])
```

Inputs and results are JSON-compatible dictionaries in the engine's camelCase shape. Options are the same as the [npm package](https://www.npmjs.com/package/lettras).
Bad input throws `Lettras.Failure.input` with a message that says what to change.

**How it works.** The compiled engine is bundled with the package as WebAssembly and run with [WasmKit](https://github.com/swiftwasm/WasmKit), a pure-Swift runtime. Each call
runs on a fresh instance, so no state survives between calls. The engine is much faster in release builds than in debug builds: generate in a release build or on a
background queue for large puzzles.

**License.** The wrapper is MIT. The compiled engine is proprietary: free to use unmodified, as distributed in this package, in your own applications (commercial ones
included); no extraction, modification, reverse engineering, or use to build a competing puzzle service. See `swift/Sources/Lettras/Resources/ENGINE-LICENSE`.
