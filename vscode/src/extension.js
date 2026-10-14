import * as vscode from 'vscode';
import { parseWords, styleChoices, styleText, sudokuText, wordSearchText } from './core.js';

async function styleSelection() {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return;
  const picked = await vscode.window.showQuickPick(styleChoices(), { placeHolder: 'Pick a Unicode style' });
  if (!picked) return;
  const missing = new Set();
  await editor.edit((edit) => {
    for (const sel of editor.selections) {
      if (sel.isEmpty) continue;
      const out = styleText(editor.document.getText(sel), picked.id);
      out.missing.forEach((m) => missing.add(m));
      edit.replace(sel, out.text);
    }
  });
  if (missing.size) vscode.window.showWarningMessage(`This style cannot draw: ${[...missing].join(' ')}`);
}

async function insertWordSearch() {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return;
  const input = await vscode.window.showInputBox({ prompt: 'Words, separated by commas', placeHolder: 'gato, perro, piña' });
  const words = input ? parseWords(input) : [];
  if (!words.length) return;
  const lang = vscode.workspace.getConfiguration('lettras').get('language', 'es');
  try {
    await editor.edit((edit) => edit.insert(editor.selection.active, wordSearchText(words, lang)));
  } catch (e) {
    vscode.window.showErrorMessage(`Lettras: ${e.message}`);
  }
}

async function insertSudoku() {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return;
  const size = await vscode.window.showQuickPick(['4', '6', '9', '12', '16'], { placeHolder: 'Size', });
  if (!size) return;
  const level = await vscode.window.showQuickPick(['1', '2', '3', '4'], { placeHolder: 'Difficulty (1 easy, 4 hard)' });
  if (!level) return;
  await editor.edit((edit) => edit.insert(editor.selection.active, sudokuText(Number(size), Number(level))));
}

export function activate(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand('lettras.styleSelection', styleSelection),
    vscode.commands.registerCommand('lettras.insertWordSearch', insertWordSearch),
    vscode.commands.registerCommand('lettras.insertSudoku', insertSudoku),
  );
}

export function deactivate() {}
