import Foundation
import WasmKit

/// Word-search, crossword and sudoku generators plus Unicode text styles for Spanish, English, Portuguese, French, German
/// and Italian, with native accented letters kept (one cell each).
///
/// The compiled Lettras engine is bundled and runs in-process (WebAssembly through WasmKit): no network, no API key.
/// Inputs are JSON-compatible dictionaries in the engine's camelCase shape; outputs are decoded JSON objects. The same
/// input and `seed` give the same puzzle. The engine is proprietary: see `ENGINE-LICENSE`.
public final class Lettras: @unchecked Sendable {
    /// What can go wrong.
    public enum Failure: Error, CustomStringConvertible {
        /// The engine rejected the input. The message says what to change.
        case input(String)
        /// The engine failed to load or run.
        case engine(String)

        public var description: String {
            switch self {
            case .input(let m), .engine(let m): return m
            }
        }
    }

    public typealias Object = [String: Any]

    private let module: Module
    private let lock = NSLock()

    /// Loads the bundled engine.
    public init() throws {
        guard let url = Bundle.module.url(forResource: "lettras_engine", withExtension: "wasm"),
              let data = try? Data(contentsOf: url)
        else { throw Failure.engine("engine file is missing") }
        do { module = try parseWasm(bytes: [UInt8](data)) } catch { throw Failure.engine("engine failed to load: \(error)") }
    }

    /// A word search. Input: `words`, `rows`, `cols`, and optionally `position` (`horizontal`, `vertical`, `mixed`),
    /// `difficulty` (1 to 4), `clustering` (0 to 1), `seed`, `lang` (`es` `en` `pt` `fr` `de` `it`), `classicMode`.
    /// Empty cells are `"-"`.
    public func generate(_ input: Object) throws -> Object { try call("lettras_generate", input) }

    /// Fills the empty cells of a word search with random letters. Input: `grid`, and optionally `lang`, `accents`, `seed`,
    /// `words` (so the filler never makes an extra copy of one). Without a `seed` every call differs.
    public func fill(_ input: Object) throws -> Object { try call("lettras_fill", input) }

    /// Text in Unicode styles that keep accents. Input: `text`, and optionally `style` (one id or a list) and `classic`.
    public func stylize(_ input: Object) throws -> Object { try call("lettras_stylize", input) }

    /// The available text styles: `["styles": [["id", "label", "category"]]]`.
    public func styles() throws -> Object { try call("lettras_styles", "") }

    /// A crossword. Input: `entries` (`[["word": ..., "clue": ...]]`), `rows`, `cols`, and optionally `seed`. Returns
    /// `puzzle`, `solution`, `placements` and `unplaced`.
    public func crossword(_ input: Object) throws -> Object { try call("lettras_crossword", input) }

    /// A sudoku with exactly one solution. Input (all optional): `size` (4, 6, 9, 12, 16), `difficulty` (1 to 4), `seed`.
    /// `0` is an empty cell.
    public func sudoku(_ input: Object = [:]) throws -> Object { try call("lettras_sudoku", input) }

    /// Solves a sudoku. Input: `["puzzle": [[Int]]]`. Returns `solution`, `solved` and `unique`.
    public func solveSudoku(_ input: Object) throws -> Object { try call("lettras_sudoku_solve", input) }

    private func call(_ function: String, _ input: Any) throws -> Object {
        let payload: Data
        if let text = input as? String, text.isEmpty {
            payload = Data("\"\"".utf8)
        } else {
            guard JSONSerialization.isValidJSONObject(input) else { throw Failure.input("input is not valid JSON") }
            payload = try JSONSerialization.data(withJSONObject: input)
        }
        let (bytes, isError) = try run(function, [UInt8](payload))
        let text = String(decoding: bytes, as: UTF8.self)
        if isError { throw Failure.input(text) }
        guard let object = try? JSONSerialization.jsonObject(with: Data(bytes)) as? Object else {
            throw Failure.engine("engine returned invalid JSON")
        }
        return object
    }

    /// One call on a fresh instance, so no state survives between calls.
    private func run(_ function: String, _ input: [UInt8]) throws -> ([UInt8], Bool) {
        lock.lock()
        defer { lock.unlock() }
        do {
            let store = Store(engine: Engine())
            let instance = try module.instantiate(store: store)
            guard let memory = instance.exports[memory: "memory"],
                  let alloc = instance.exports[function: "lettras_alloc"],
                  let free = instance.exports[function: "lettras_free"],
                  let entry = instance.exports[function: function],
                  let resultPtr = instance.exports[function: "lettras_result_ptr"]
            else { throw Failure.engine("engine exports are missing") }

            let size = UInt32(input.count)
            guard case .i32(let ptr)? = try alloc([.i32(size)]).first else { throw Failure.engine("engine alloc failed") }
            memory.withUnsafeMutableBufferPointer(offset: UInt(ptr), count: input.count) { buffer in
                buffer.copyBytes(from: input)
            }
            guard case .i64(let raw)? = try entry([.i32(ptr), .i32(size)]).first else { throw Failure.engine("engine call failed") }
            _ = try free([.i32(ptr), .i32(size)])

            let signed = Int64(bitPattern: raw)
            let length = Int(signed.magnitude)
            guard case .i32(let outPtr)? = try resultPtr([]).first else { throw Failure.engine("engine result failed") }
            var out = [UInt8](repeating: 0, count: length)
            if length > 0 {
                memory.withUnsafeMutableBufferPointer(offset: UInt(outPtr), count: length) { buffer in
                    out.withUnsafeMutableBytes { $0.copyMemory(from: UnsafeRawBufferPointer(buffer)) }
                }
            }
            return (out, signed < 0)
        } catch let failure as Failure {
            throw failure
        } catch {
            throw Failure.engine("\(error)")
        }
    }
}
