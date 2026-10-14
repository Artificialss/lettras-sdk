# Lettras for JetBrains IDEs

Insert word searches and sudokus, and write selected text in **22 Unicode styles** that keep accents. Six languages: Spanish, English, Portuguese, French, German and Italian.
The engine runs locally inside the IDE (through the [Lettras Kotlin library](../kotlin/README.md)): no network, no API key. Works in IntelliJ IDEA, PyCharm, WebStorm, GoLand,
Android Studio and the other IDEs built on the IntelliJ Platform 2024.1 or newer.

## Actions

**Tools > Lettras**

| Action | What it does |
| --- | --- |
| **Write Selection in a Unicode Style** | Pick one of the 22 styles; the selection is replaced. A warning lists the characters a style cannot draw. |
| **Insert a Word Search** | Type your words; a grid with every word hidden is inserted at the caret, with the word list below. The language follows the IDE's language (Spanish when it is not one of the six). |
| **Insert a Sudoku** | Pick a size (4 to 16) and a level; a sudoku with one solution is inserted, `.` for empty cells. |

## Build

```bash
cd jetbrains
./gradlew test          # logic tests (no IDE needed to run them)
./gradlew buildPlugin   # build/distributions/lettras-jetbrains-<version>.zip
./gradlew runIde        # try it in a sandbox IDE
```

The first build downloads the IntelliJ Platform (about 1 GB). Install the zip from **Settings > Plugins > gear icon > Install Plugin from Disk**, or publish it to the
JetBrains Marketplace (steps and listing text: [`docs/JETBRAINS_MARKETPLACE.md`](../docs/JETBRAINS_MARKETPLACE.md)).

## License

The plugin code is MIT. The compiled engine inside the Kotlin library is proprietary: free to use unmodified, as distributed here, in your own work (commercial included); no
extraction, modification, reverse engineering, or use to build a competing puzzle service. See `LICENSE` and `ENGINE-LICENSE`.
