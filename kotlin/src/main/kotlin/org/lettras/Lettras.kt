package org.lettras

import kotlin.random.Random
import kotlinx.serialization.json.Json

/**
 * Word-search generator for Spanish, English, Portuguese, French, German and Italian.
 *
 * Runs locally on the JVM or Android: no network and no API key. Create one instance and reuse it;
 * it is thread-safe.
 *
 * ```
 * val lettras = Lettras()
 * val puzzle = lettras.generate(
 *     PuzzleRequest(words = listOf("gato", "perro", "piña"), rows = 9, cols = 12, position = Position.MIXED, seed = 8),
 * )
 * println(puzzle.render())
 * ```
 */
public class Lettras {
    private val engine: Engine

    /** Creates a generator that uses the bytecode compiler on the JVM and the interpreter on Android. */
    public constructor() {
        engine = Engine(loadEngine())
    }

    /** For tests: `useCompiler = false` forces the interpreter, the path Android takes. */
    internal constructor(useCompiler: Boolean) {
        engine = Engine(loadEngine(), useCompiler)
    }

    /** Generates a puzzle. Throws [LettrasException] if the request is invalid (for example zero rows). */
    public fun generate(request: PuzzleRequest): Puzzle {
        val output = try {
            engine.generate(json.encodeToString(PuzzleRequest.serializer(), request))
        } catch (e: LettrasException) {
            throw e
        } catch (e: Exception) {
            throw LettrasException("the engine failed: ${e.message}", e)
        }
        return json.decodeFromString(Puzzle.serializer(), output)
    }

    /**
     * Fills the empty cells with random letters. If [FillRequest.seed] is null a random one is chosen, so the filler
     * differs on every call; pass a seed for a repeatable result. Throws [LettrasException] on invalid input.
     */
    public fun fill(request: FillRequest): FillResult {
        val withSeed = if (request.seed == null) request.copy(seed = Random.nextLong(0L, 4_294_967_296L)) else request
        val output = try {
            engine.fill(json.encodeToString(FillRequest.serializer(), withSeed))
        } catch (e: LettrasException) {
            throw e
        } catch (e: Exception) {
            throw LettrasException("the engine failed: ${e.message}", e)
        }
        return json.decodeFromString(FillResult.serializer(), output)
    }

    /**
     * Completes a [Puzzle] from [generate]: its words are protected from accidental extra copies.
     * Use `puzzle.copy(grid = result.grid)` to get the filled puzzle back.
     */
    public fun fill(puzzle: Puzzle, lang: Language = Language.ES, accents: Boolean = true, seed: Long? = null): FillResult =
        fill(FillRequest(grid = puzzle.grid, lang = lang, accents = accents, seed = seed, words = puzzle.words))

    /** Fills a bare matrix (rows of one-letter strings, empty cells `-`). */
    public fun fill(grid: List<List<String>>, lang: Language = Language.ES, accents: Boolean = true, seed: Long? = null): FillResult =
        fill(FillRequest(grid = grid, lang = lang, accents = accents, seed = seed))

    /**
     * Writes text in Unicode styles that keep accents (22 styles: bold, italic, script, fraktur, circled, strikethrough...).
     * An accented letter is its styled base letter plus the combining mark. What a style cannot draw is reported in
     * [StyledText.notNative] and as [StyledPart.missing] parts, so a UI can show an empty box. Throws [LettrasException] for an
     * unknown style id or text over 10,000 characters.
     */
    public fun stylize(request: StylizeRequest): StylizeResult =
        json.decodeFromString(StylizeResult.serializer(), guard { engine.stylize(json.encodeToString(StylizeRequest.serializer(), request)) })

    /** Shorthand for [stylize]: one [style] id (or all 22 when null). */
    public fun stylize(text: String, style: String? = null, classic: Boolean = false): StylizeResult =
        stylize(StylizeRequest(text = text, style = style?.let { listOf(it) }, classic = classic))

    /** Shorthand for [stylize] with several style ids. */
    public fun stylize(text: String, styles: List<String>, classic: Boolean = false): StylizeResult =
        stylize(StylizeRequest(text = text, style = styles, classic = classic))

    /** Styled characters back to plain ones, decoration marks removed. For search, validation and tests. */
    public fun unstyle(text: String): String =
        json.decodeFromString(TextBody.serializer(), guard { engine.unstyle(json.encodeToString(TextBody.serializer(), TextBody(text))) }).text

    /** The styles [stylize] offers, in display order. */
    public fun listStyles(): List<StyleInfo> =
        json.decodeFromString(StyleList.serializer(), guard { engine.listStyles() }).styles

    /** Runs an engine call, turning unexpected failures into [LettrasException]. */
    private fun guard(call: () -> String): String =
        try {
            call()
        } catch (e: LettrasException) {
            throw e
        } catch (e: Exception) {
            throw LettrasException("the engine failed: ${e.message}", e)
        }

    /** Shorthand for [generate] with the common options. */
    public fun generate(
        words: List<String>,
        rows: Int,
        cols: Int = rows,
        position: Position? = null,
        seed: Long? = null,
        lang: Language? = null,
    ): Puzzle = generate(PuzzleRequest(words = words, rows = rows, cols = cols, position = position, seed = seed, lang = lang))

    /** True when the engine runs as compiled JVM bytecode (fast); false on the interpreter (Android). */
    public val isCompiled: Boolean get() = engine.isCompiled

    /** Version of the bundled engine. */
    public val engineVersion: String by lazy { generate(listOf("ab"), 6).engineVersion }

    private companion object {
        val json = Json {
            explicitNulls = false // leave unset options out so the engine applies its defaults
            ignoreUnknownKeys = true
        }

        fun loadEngine(): ByteArray =
            Lettras::class.java.getResourceAsStream("/org/lettras/lettras_engine.wasm")?.use { it.readBytes() }
                ?: throw LettrasException("bundled engine not found on the classpath")
    }
}
