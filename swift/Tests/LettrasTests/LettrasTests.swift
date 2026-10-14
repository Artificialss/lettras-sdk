import XCTest
@testable import Lettras

final class LettrasTests: XCTestCase {
    func testWordSearchKeepsAccentsAndIsDeterministic() throws {
        let e = try Lettras()
        let input: Lettras.Object = ["words": ["gato", "piña", "mono"], "rows": 8, "cols": 8, "seed": 3]
        let a = try e.generate(input)
        let b = try e.generate(input)
        let grid = a["grid"] as? [[String]]
        XCTAssertEqual(grid?.count, 8)
        XCTAssertEqual(grid, b["grid"] as? [[String]])
        XCTAssertTrue(grid?.joined().contains("Ñ") ?? false)
    }

    func testFillLeavesNoEmptyCells() throws {
        let e = try Lettras()
        let p = try e.generate(["words": ["sol", "luna"], "rows": 6, "cols": 6, "seed": 1])
        let done = try e.fill(["grid": p["grid"] as Any, "lang": "es", "seed": 2])
        XCTAssertFalse((done["grid"] as? [[String]] ?? []).joined().contains("-"))
    }

    func testSudokuCrosswordAndStyles() throws {
        let e = try Lettras()
        let s = try e.sudoku(["size": 9, "difficulty": 2, "seed": 4])
        let solved = try e.solveSudoku(["puzzle": s["puzzle"] as Any])
        XCTAssertEqual(solved["unique"] as? Bool, true)
        let c = try e.crossword(["entries": [["word": "gato", "clue": "Felino"], ["word": "toro", "clue": "Bovino"], ["word": "ropa", "clue": "Se viste"]], "rows": 8, "cols": 8, "seed": 1])
        XCTAssertFalse((c["placements"] as? [Any] ?? []).isEmpty)
        XCTAssertFalse(((try e.styles())["styles"] as? [Any] ?? []).isEmpty)
        _ = try e.stylize(["text": "Ñandú", "style": "bold"])
    }

    func testBadInputIsAnInputError() throws {
        let e = try Lettras()
        XCTAssertThrowsError(try e.generate(["words": ["gato"], "rows": 0, "cols": 5])) { error in
            guard case Lettras.Failure.input = error else { return XCTFail("\(error)") }
        }
    }
}
