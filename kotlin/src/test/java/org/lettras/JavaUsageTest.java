package org.lettras;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;
import kotlin.Pair;
import org.junit.jupiter.api.Test;

/** The library is usable from plain Java: shorthand methods keep their defaults through @JvmOverloads. */
class JavaUsageTest {
    @Test
    void wordSearchFillSudokuAndCrossword() {
        Lettras lettras = new Lettras();

        Puzzle puzzle = lettras.generate(List.of("gato", "piña", "mono"), 8);
        assertEquals(8, puzzle.getGrid().size());

        FillResult filled = lettras.fill(puzzle);
        assertFalse(filled.getGrid().stream().flatMap(List::stream).anyMatch("-"::equals));

        SudokuResult sudoku = lettras.sudoku(9);
        assertTrue(lettras.solveSudoku(sudoku.getPuzzle()).getUnique());

        CrosswordResult crossword = lettras.crossword(
                List.of(new Pair<>("gato", "Felino"), new Pair<>("toro", "Bovino"), new Pair<>("ropa", "Se viste")), 8);
        assertFalse(crossword.getPlacements().isEmpty());

        assertFalse(lettras.stylize("Ñandú").getResults().isEmpty());
    }
}
