package org.lettras.jetbrains

import com.intellij.openapi.actionSystem.ActionUpdateThread
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.ui.popup.JBPopupFactory
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
        JBPopupFactory.getInstance()
            .createPopupChooserBuilder(choices.map { it.second })
            .setTitle("Lettras: pick a Unicode style")
            .setItemChosenCallback { name ->
                val choice = choices.firstOrNull { it.second == name } ?: return@setItemChosenCallback
                val (styled, missing) = Core.styleText(selected, choice.first)
                editor.replaceSelection(styled)
                if (missing.isNotEmpty()) Messages.showWarningDialog(editor.project, "This style cannot draw: ${missing.joinToString(" ")}", "Lettras")
            }
            .createPopup()
            .showInBestPositionFor(editor)
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
            Messages.showErrorDialog(editor.project, ex.message ?: "Could not make the word search", "Lettras")
        }
    }
}

class InsertSudokuAction : EditorAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val sizes = arrayOf("4", "6", "9", "12", "16")
        val s = Messages.showDialog(editor.project, "Size", "Lettras: Sudoku", sizes, 2, null)
        if (s < 0) return
        val levels = arrayOf("1 (easy)", "2", "3", "4 (hard)")
        val l = Messages.showDialog(editor.project, "Difficulty", "Lettras: Sudoku", levels, 1, null)
        if (l < 0) return
        editor.insertAtCaret(Core.sudokuText(sizes[s].toInt(), l + 1))
    }
}
