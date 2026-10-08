package org.lettras

import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlin.test.Test
import kotlin.test.assertContains
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertFalse
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

class LettrasTest {
    private val lettras = Lettras()
    private val json = Json { ignoreUnknownKeys = true }
    private val animals = listOf("gato", "perro", "piña", "mono", "cebra")

    @Test
    fun `places every word with one cell per letter`() {
        val p = lettras.generate(PuzzleRequest(animals, rows = 9, cols = 12, position = Position.MIXED, seed = 8))
        assertEquals(emptyList(), p.unplaced)
        assertEquals(9, p.grid.size)
        assertTrue(p.grid.all { it.size == 12 })
        assertTrue(p.grid.flatten().contains("Ñ"))
        assertEquals(4, p.placements.single { it.word == "piña" }.length)
    }

    @Test
    fun `every placement reads back its word from the grid`() {
        val p = lettras.generate(PuzzleRequest(animals + "corazón", rows = 12, cols = 12, position = Position.MIXED, seed = 3))
        for (pl in p.placements) {
            val read = (0 until pl.length).joinToString("") { p.grid[pl.r + pl.dr * it][pl.c + pl.dc * it] }
            assertEquals(pl.word.uppercase(), read)
        }
    }

    @Test
    fun `matches the npm package exactly (shared fixtures)`() {
        @Serializable
        class Fixture(val input: PuzzleRequest, val output: Puzzle)

        val text = LettrasTest::class.java.getResourceAsStream("/parity.json")!!.bufferedReader().readText()
        val fixtures = json.decodeFromString<List<Fixture>>(text)
        assertTrue(fixtures.size >= 4)
        fixtures.forEachIndexed { i, f -> assertEquals(f.output, lettras.generate(f.input), "fixture $i differs from the npm package") }
    }

    @Test
    fun `same seed gives the same puzzle, another seed a different one`() {
        val a = lettras.generate(animals, rows = 10, seed = 42)
        assertEquals(a, lettras.generate(animals, rows = 10, seed = 42))
        assertNotEquals(a.grid, lettras.generate(animals, rows = 10, seed = 43).grid)
    }

    @Test
    fun `horizontal and vertical are one-way`() {
        val h = lettras.generate(animals, rows = 12, position = Position.HORIZONTAL, seed = 1)
        assertTrue(h.placements.all { it.dr == 0 && it.dc == 1 })
        val v = lettras.generate(animals, rows = 12, position = Position.VERTICAL, seed = 1)
        assertTrue(v.placements.all { it.dr == 1 && it.dc == 0 })
    }

    @Test
    fun `mixed uses diagonals`() {
        val withDiagonal = (0L until 20L).count { seed ->
            lettras.generate(animals, rows = 12, position = Position.MIXED, seed = seed).placements.any { it.dr != 0 && it.dc != 0 }
        }
        assertTrue(withDiagonal >= 19, "$withDiagonal of 20")
    }

    @Test
    fun `German sharp s is a single capital cell`() {
        val p = lettras.generate(listOf("Fuß", "Straße"), rows = 8, lang = Language.DE)
        assertContains(p.grid.flatten(), "ẞ")
        assertFalse(p.grid.flatten().contains("ß"))
    }

    @Test
    fun `classic mode strips accents`() {
        val p = lettras.generate(PuzzleRequest(listOf("corazón", "piña"), rows = 8, cols = 8, classicMode = true, seed = 1))
        assertTrue(p.grid.flatten().none { it == "Ó" || it == "Ñ" })
    }

    @Test
    fun `custom fill and rectangular grids`() {
        val p = lettras.generate(PuzzleRequest(listOf("sol"), rows = 6, cols = 30, fill = "·"))
        assertEquals(6, p.grid.size)
        assertTrue(p.grid.all { it.size == 30 })
        assertEquals(6 * 30 - 3, p.grid.flatten().count { it == "·" })
    }

    @Test
    fun `invalid requests throw with the reason`() {
        val e = assertFailsWith<LettrasException> { lettras.generate(PuzzleRequest(listOf("sol"), rows = 0, cols = 5)) }
        assertContains(e.message.orEmpty(), "rows")
    }

    @Test
    fun `words that do not fit and refused words are reported`() {
        val tight = lettras.generate(PuzzleRequest(listOf("abcdefghijklmnop"), rows = 6, cols = 6))
        assertEquals(listOf("abcdefghijklmnop"), tight.unplaced)

        val r = lettras.generate(PuzzleRequest(listOf("cat", "CAT", "ice cream", "cat1", "concat", "dog"), rows = 8, cols = 8))
        assertEquals(4, r.rejected.size)
        assertEquals(listOf("concat", "dog"), r.words.sorted())
    }

    @Test
    fun `render matches the CLI format and solution hides the rest`() {
        val p = lettras.generate(PuzzleRequest(listOf("sol", "río"), rows = 6, cols = 6, position = Position.MIXED, seed = 3))
        val expected = "6×6  seed 3  engine 0.1.0\n\nS - - - - -\n- O Í R - -\n- - L - - -\n- - - - - -\n- - - - - -\n- - - - - -\n\nWords: sol, río"
        assertEquals(expected, p.render())
        val solution = p.solution()
        assertEquals(6, solution.lines().size)
        assertEquals("S · · · · ·\n· O Í R · ·\n· · L · · ·\n· · · · · ·\n· · · · · ·\n· · · · · ·", solution)
    }

    @Test
    fun `uses the bytecode compiler on the JVM`() = assertTrue(lettras.isCompiled)

    @Test
    fun `reports the engine version`() = assertEquals("0.1.0", lettras.engineVersion)

    @Test
    fun `is safe to call from many threads`() {
        val expected = lettras.generate(animals, rows = 10, position = Position.MIXED, seed = 5)
        val pool = Executors.newFixedThreadPool(8)
        try {
            val results = (1..24).map { pool.submit<Puzzle> { lettras.generate(animals, rows = 10, position = Position.MIXED, seed = 5) } }
            results.forEach { assertEquals(expected, it.get(120, TimeUnit.SECONDS)) }
        } finally {
            pool.shutdownNow()
        }
    }

    @Test
    fun `largest allowed puzzle finishes`() {
        val letters = "abcdefghijklmnopqrstuvwxyz"
        val words = (0 until 60).map { i -> "w${letters[i % 26]}${letters[(i * 7 + 3) % 26]}${letters[(i * 11 + 5) % 26]}${letters[(i / 3) % 26]}z" }
        val start = System.nanoTime()
        val p = lettras.generate(PuzzleRequest(words, rows = 30, cols = 30, position = Position.MIXED, seed = 3))
        val seconds = (System.nanoTime() - start) / 1e9
        println("30x30, ${words.size} words: ${"%.1f".format(seconds)} s, placed ${p.words.size}, unplaced ${p.unplaced.size}")
        assertTrue(seconds < 60, "took $seconds s")
    }

    @Test
    fun `interpreter path (used on Android) gives identical results`() {
        val interpreted = Lettras(useCompiler = false)
        assertFalse(interpreted.isCompiled)
        val text = LettrasTest::class.java.getResourceAsStream("/parity.json")!!.bufferedReader().readText()

        @Serializable
        class Fixture(val input: PuzzleRequest, val output: Puzzle)
        val fixture = json.decodeFromString<List<Fixture>>(text).first()
        assertEquals(fixture.output, interpreted.generate(fixture.input))

        val typical = PuzzleRequest(listOf("gato", "perro", "piña", "mono", "cebra", "oso", "pato", "vaca"), rows = 10, cols = 10, position = Position.MIXED, seed = 2)
        interpreted.generate(typical) // first call includes one-time setup
        val start = System.nanoTime()
        repeat(3) { interpreted.generate(typical) }
        println("interpreter, 10x10 with 8 words: ${"%.2f".format((System.nanoTime() - start) / 3e9)} s per puzzle")
    }

    // ---- fill ----

    @Serializable
    class FillFixture(val input: FillRequest, val output: FillResult)

    private fun fillFixtures(): List<FillFixture> {
        val text = LettrasTest::class.java.getResourceAsStream("/fill-parity.json")!!.bufferedReader().readText()
        return json.decodeFromString(text)
    }

    @Test
    fun `fill matches the npm package exactly (shared fixtures)`() {
        val fixtures = fillFixtures()
        assertTrue(fixtures.size >= 6)
        fixtures.forEachIndexed { i, f -> assertEquals(f.output, lettras.fill(f.input), "fill fixture $i differs from the npm package") }
    }

    @Test
    fun `fill completes a puzzle and keeps the hidden letters`() {
        val p = lettras.generate(PuzzleRequest(animals, rows = 12, cols = 12, position = Position.MIXED, seed = 8))
        val f = lettras.fill(p, lang = Language.ES, seed = 1)
        assertTrue(f.grid.flatten().none { it == "-" })
        assertEquals(p.grid.flatten().count { it == "-" }, f.filled)
        p.grid.forEachIndexed { r, row -> row.forEachIndexed { c, cell -> if (cell != "-") assertEquals(cell, f.grid[r][c]) } }
        assertEquals(emptyList(), f.ambiguous)
        assertEquals(12, f.render().lines().size)
    }

    @Test
    fun `fill is repeatable with a seed and different without one`() {
        val p = lettras.generate(animals, rows = 12, seed = 8)
        assertEquals(lettras.fill(p, seed = 5), lettras.fill(p, seed = 5))
        val seeds = (1..5).map { lettras.fill(p).seed }.toSet()
        assertTrue(seeds.size > 1, "unseeded calls should choose different seeds")
    }

    @Test
    fun `fill accents off is plain A-Z and accents on uses native letters`() {
        val blank = List(40) { List(40) { "-" } }
        assertTrue(lettras.fill(blank, Language.ES, accents = false, seed = 3).grid.flatten().all { it.length == 1 && it[0] in 'A'..'Z' })
        assertContains(lettras.fill(blank, Language.ES, accents = true, seed = 3).grid.flatten(), "Ñ")
        assertContains(lettras.fill(blank, Language.DE, accents = true, seed = 3).grid.flatten(), "ẞ")
        assertTrue(lettras.fill(blank, Language.EN, accents = true, seed = 3).grid.flatten().all { it[0] in 'A'..'Z' }, "English has no accents")
    }

    @Test
    fun `fill never adds a second copy of a hidden word`() {
        val p = lettras.generate(PuzzleRequest(listOf("ab", "cd", "ef", "gh"), rows = 8, cols = 8, position = Position.MIXED, seed = 2, lang = Language.EN))
        (0L until 15L).forEach { seed -> assertEquals(emptyList(), lettras.fill(p, Language.EN, accents = false, seed = seed).ambiguous) }
    }

    @Test
    fun `fill rejects bad input`() {
        assertFailsWith<LettrasException> { lettras.fill(emptyList<List<String>>()) }
        assertFailsWith<LettrasException> { lettras.fill(listOf(listOf("-", "-"), listOf("-"))) }
    }

    @Test
    fun `fill gives identical results on the interpreter path (Android)`() {
        val interpreted = Lettras(useCompiler = false)
        val f = fillFixtures().first()
        assertEquals(f.output, interpreted.fill(f.input))
    }
}
