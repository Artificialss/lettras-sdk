package org.lettras.jetbrains

import org.lettras.Language
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

class CoreTest {
    @Test fun parseWords() = assertEquals(listOf("gato", "perro", "piña"), Core.parseWords("gato, perro;\npiña ,"))

    @Test fun twentyTwoStyles() = assertEquals(22, Core.styleChoices().size)

    @Test fun boldChangesText() = assertNotEquals("hola", Core.styleText("hola", "bold").first)

    @Test fun wordSearchHasGridAndWords() {
        val text = Core.wordSearchText(listOf("gato", "perro", "piña"), Language.ES)
        assertTrue(text.contains("Words: "))
        assertTrue(!text.substringBefore("\n\n").contains("-"))
    }

    @Test fun sudokuHasOneRowPerCell() = assertEquals(9, Core.sudokuText(9, 2).trim().lines().size)
}
