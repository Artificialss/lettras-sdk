# Lettras for VS Code

Insert word searches and sudokus, and write selected text in **22 Unicode styles** that keep accents (bold, italic, script, fraktur, circled, strikethrough…). Six languages:
Spanish, English, Portuguese, French, German and Italian. The engine runs locally inside the extension: no network, no API key.

## Commands

Open the Command Palette (`Cmd/Ctrl+Shift+P`) and type **Lettras**.

| Command | What it does |
| --- | --- |
| **Lettras: Write selection in a Unicode style** | Pick one of the 22 styles; the selection is replaced. Characters a style cannot draw are listed in a warning. |
| **Lettras: Insert a word search** | Type your words; a grid with every word hidden is inserted at the cursor, empty cells filled with letters of the chosen language, with the word list below. |
| **Lettras: Insert a sudoku** | Pick a size (4 to 16) and a level; a sudoku with exactly one solution is inserted, `.` for empty cells. |

## Settings

- `lettras.language`: `es` (default), `en`, `pt`, `fr`, `de`, `it`. Accented letters stay one cell each.

Made with the [`lettras`](https://www.npmjs.com/package/lettras) npm package; the same engine is in the web app at [lettras.org](https://www.lettras.org) and in the other
[Lettras libraries](https://github.com/Artificialss/lettras-sdk/blob/main/docs/LIBRARIES.md).

## License

The extension code is MIT. The compiled engine inside is proprietary: free to use unmodified, as distributed here, in your own work (commercial included); no extraction, modification,
reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `ENGINE-LICENSE`.
