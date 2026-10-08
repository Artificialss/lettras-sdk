package org.lettras

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
