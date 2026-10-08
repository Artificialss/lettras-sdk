package org.lettras

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/** Word orientation. */
@Serializable
public enum class Position {
    /** Left to right only. */
    @SerialName("horizontal") HORIZONTAL,

    /** Top to bottom only. */
    @SerialName("vertical") VERTICAL,

    /** All 8 directions, diagonals and reversed words included. */
    @SerialName("mixed") MIXED,
}

/** Languages with their own alphabet. Native letters (Ñ, Ç, Ã, Ä, ẞ, È…) are one grid cell each. */
@Serializable
public enum class Language {
    @SerialName("es") ES,
    @SerialName("en") EN,
    @SerialName("pt") PT,
    @SerialName("fr") FR,
    @SerialName("de") DE,
    @SerialName("it") IT,
}

/**
 * What to generate. Same request and [seed] always give the same puzzle.
 *
 * @property words words to hide, in normal spelling (accents are kept)
 * @property rows grid height; [cols] may differ for a rectangular grid
 * @property position directions to use; overrides [difficulty]
 * @property difficulty 1 (right, down) to 4 (all 8 directions); used when [position] is null
 * @property clustering 0.0 keeps words apart, 1.0 makes them cross; engine default 0.5
 * @property seed makes the puzzle repeatable; engine default 1
 * @property lang language of the words; engine default [Language.ES]
 * @property classicMode strip accents in the grid (ñ becomes N)
 * @property fill character for empty cells; engine default "-"
 */
@Serializable
public data class PuzzleRequest(
    val words: List<String>,
    val rows: Int,
    val cols: Int,
    val position: Position? = null,
    val difficulty: Int? = null,
    val clustering: Double? = null,
    val seed: Long? = null,
    val lang: Language? = null,
    val classicMode: Boolean? = null,
    val fill: String? = null,
)

/** Where a word is hidden: starts at ([r], [c]) and steps ([dr], [dc]) for [length] cells. */
@Serializable
public data class Placement(
    val word: String,
    val r: Int,
    val c: Int,
    val dr: Int,
    val dc: Int,
    val length: Int,
)

/** A word the engine refused, with the reason. */
@Serializable
public data class RejectedWord(val word: String, val reason: String)

/**
 * A generated puzzle. [grid] is `rows` lists of `cols` one-letter strings.
 *
 * @property words words that were placed, in input order
 * @property unplaced words that did not fit (never dropped silently)
 * @property rejected words refused for being invalid, duplicated or contained in another word
 * @property seed the seed that produced the grid (may be higher than requested if the engine retried)
 * @property engineVersion engine version, to reproduce stored puzzles later
 */
@Serializable
public data class Puzzle(
    val grid: List<List<String>>,
    val placements: List<Placement>,
    val words: List<String>,
    val unplaced: List<String>,
    val rejected: List<RejectedWord>,
    val seed: Long,
    val engineVersion: String,
) {
    /** Plain-text view: grid, word bank, and anything that did not fit. Matches the CLI output. */
    public fun render(): String = buildString {
        append("${grid.size}×${grid.firstOrNull()?.size ?: 0}  seed $seed  engine $engineVersion\n\n")
        append(gridText(solutionOnly = false))
        append("\n\nWords: ${words.joinToString(", ")}")
        if (unplaced.isNotEmpty()) append("\nUNPLACED: ${unplaced.joinToString(", ")}")
        if (rejected.isNotEmpty()) append("\nRejected: ${rejected.joinToString("; ") { "${it.word} (${it.reason})" }}")
    }

    /** The grid with only the hidden words shown and every other cell as `·`. */
    public fun solution(): String = gridText(solutionOnly = true)

    private fun gridText(solutionOnly: Boolean): String {
        val hidden = placements.flatMap { p -> (0 until p.length).map { (p.r + p.dr * it) to (p.c + p.dc * it) } }.toSet()
        return grid.mapIndexed { r, row ->
            row.mapIndexed { c, cell -> if (solutionOnly && (r to c) !in hidden) "·" else cell }.joinToString(" ")
        }.joinToString("\n")
    }
}

/**
 * Fill the empty cells of a grid with random letters.
 *
 * @property grid the matrix from [Puzzle.grid]: rows of one-letter strings, empty cells hold [empty]
 * @property lang language of the letters; engine default [Language.ES]. Letters follow its letter frequency.
 * @property accents true (engine default) uses the language's accented/native letters (Ñ, Ç, Ã, Ä, ẞ…); false uses A-Z only
 * @property seed same request and seed, same filler. [Lettras.fill] picks a random one when this is null.
 * @property empty what marks an empty cell; engine default "-"
 * @property words the hidden words; when given, the filler never creates an extra copy of one
 */
@Serializable
public data class FillRequest(
    val grid: List<List<String>>,
    val lang: Language? = null,
    val accents: Boolean? = null,
    val seed: Long? = null,
    val empty: String? = null,
    val words: List<String>? = null,
)

/**
 * The completed grid.
 *
 * @property filled number of cells that were filled
 * @property seed the seed used, so the filler can be reproduced
 * @property ambiguous words that still have an accidental extra copy (practically always empty)
 */
@Serializable
public data class FillResult(
    val grid: List<List<String>>,
    val filled: Int,
    val seed: Long,
    val ambiguous: List<String>,
    val engineVersion: String,
) {
    /** The grid as text, one row per line. */
    public fun render(): String = grid.joinToString("\n") { it.joinToString(" ") }
}

/** The request was invalid or the engine failed. [message] says why. */
public class LettrasException(message: String, cause: Throwable? = null) : RuntimeException(message, cause)
