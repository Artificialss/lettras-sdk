package org.lettras.jetbrains

import com.intellij.openapi.actionSystem.ActionUpdateThread
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.ui.Messages
import org.lettras.Language

abstract class EditorAction : AnAction() {
    override fun getActionUpdateThread() = ActionUpdateThread.BGT
    override fun update(e: AnActionEvent) {
        e.presentation.isEnabledAndVisible = e.getData(CommonDataKeys.EDITOR) != null
    }
}

private fun Editor.replaceSelection(text: String) =
    WriteCommandAction.runWriteCommandAction(project) {
        val sel = selectionModel
        document.replaceString(sel.selectionStart, sel.selectionEnd, text)
    }

private fun Editor.insertAtCaret(text: String) =
    WriteCommandAction.runWriteCommandAction(project) { document.insertString(caretModel.offset, text) }

class StyleSelectionAction : EditorAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val selected = editor.selectionModel.selectedText ?: return
        val choices = Core.styleChoices()
        val index = Messages.showChooseDialog("Pick a Unicode style", "Lettras", choices.map { it.second }.toTypedArray(), choices.first().second, null)
        if (index < 0) return
        val (styled, missing) = Core.styleText(selected, choices[index].first)
        editor.replaceSelection(styled)
        if (missing.isNotEmpty()) Messages.showWarningDialog("This style cannot draw: ${missing.joinToString(" ")}", "Lettras")
    }
}

class InsertWordSearchAction : EditorAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val input = Messages.showInputDialog(editor.project, "Words, separated by commas", "Lettras: Word Search", null) ?: return
        val words = Core.parseWords(input)
        if (words.isEmpty()) return
        val lang = Language.entries.firstOrNull { it.name.equals(System.getProperty("user.language"), ignoreCase = true) } ?: Language.ES
        try {
            editor.insertAtCaret(Core.wordSearchText(words, lang))
        } catch (ex: Exception) {
            Messages.showErrorDialog(ex.message ?: "Could not make the word search", "Lettras")
        }
    }
}

class InsertSudokuAction : EditorAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val sizes = arrayOf("4", "6", "9", "12", "16")
        val s = Messages.showChooseDialog("Size", "Lettras: Sudoku", sizes, "9", null)
        if (s < 0) return
        val levels = arrayOf("1 (easy)", "2", "3", "4 (hard)")
        val l = Messages.showChooseDialog("Difficulty", "Lettras: Sudoku", levels, "2", null)
        if (l < 0) return
        editor.insertAtCaret(Core.sudokuText(sizes[s].toInt(), l + 1))
    }
}
