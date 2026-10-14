package org.lettras.jetbrains

import org.lettras.Language
import org.lettras.Lettras

/** The pure logic behind the actions, so it can be tested without the IDE. */
object Core {
    // One engine per IDE session: loading it compiles the WebAssembly module.
    val lettras: Lettras by lazy { Lettras() }

    /** Splits "gato, perro; piña" into words. */
    fun parseWords(text: String): List<String> = text.split(Regex("[,;\n]+")).map { it.trim() }.filter { it.isNotEmpty() }

    /** The 22 styles for a chooser. */
    fun styleChoices(): List<Pair<String, String>> = lettras.listStyles().map { it.id to it.label }

    /** The text in one style, plus the characters that style cannot draw. */
    fun styleText(text: String, id: String): Pair<String, List<String>> {
        val r = lettras.stylize(text, id).results.first()
        return r.text to r.notNative
    }

    /** A word search as text: the grid with letters in the empty cells, then the word list. */
    fun wordSearchText(words: List<String>, lang: Language, size: Int = 12): String {
        val n = size.coerceIn(8, 30)
        val puzzle = lettras.generate(words, rows = n, cols = n, lang = lang)
        val done = lettras.fill(puzzle, lang = lang, accents = true)
        val unplaced = if (puzzle.unplaced.isEmpty()) "" else "\n(not placed: ${puzzle.unplaced.joinToString(", ")})"
        return done.grid.joinToString("\n") { it.joinToString(" ") } + "\n\nWords: ${puzzle.words.joinToString(", ")}$unplaced\n"
    }

    /** A sudoku as text, `.` for empty cells. */
    fun sudokuText(size: Int = 9, difficulty: Int = 2): String {
        val s = lettras.sudoku(size, difficulty)
        fun cell(n: Int) = when {
            n == 0 -> "."
            n > 9 -> (55 + n).toChar().toString()
            else -> n.toString()
        }
        return s.puzzle.joinToString("\n") { row -> row.joinToString(" ") { cell(it) } } + "\n"
    }
}
