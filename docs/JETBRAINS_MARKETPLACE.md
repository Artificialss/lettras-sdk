# JetBrains Marketplace

The plugin lives in [`jetbrains/`](../jetbrains/README.md). Plugin id `ai.artificialss.lettras` (permanent once published), name **Lettras**, vendor **Artificialss**.

## Upload

1. Build and check: `cd jetbrains && ./gradlew test buildPlugin verifyPluginStructure` (the zip is `build/distributions/lettras-jetbrains-<version>.zip`).
2. Sign in at <https://plugins.jetbrains.com/plugin/add>, choose the vendor *Artificialss*, upload the zip and fill the listing below.
3. JetBrains reviews the first upload (usually a few working days). Later versions: bump `version` in `jetbrains/build.gradle.kts`, update `changeNotes`, build, upload.

## What the package already carries

- `plugin.xml`: id, name, vendor with url and email, description (HTML), `since-build 241` (IntelliJ Platform 2024.1+), no upper limit.
- `pluginIcon.svg` (40x40, the Lettras mark on cream, readable in light and dark themes).
- Change notes from `build.gradle.kts`.
- No network access, no telemetry, no API key; the engine runs inside the IDE through the Kotlin library.

## Listing text

- **Name:** Lettras
- **Summary (one line):** Insert word searches and sudokus and write selected text in 22 Unicode styles, in six languages, with accents kept. Runs locally.
- **Category:** Tools / Editor
- **Tags:** word search, sudoku, puzzle, unicode, text styles, fonts, multilingual
- **Licence:** MIT for the plugin code; the compiled engine is free to use unmodified (see `jetbrains/ENGINE-LICENSE`).
- **Links:** website <https://www.lettras.org>, source <https://github.com/Artificialss/lettras-sdk>, issues the same repository, docs [`LIBRARIES.md`](LIBRARIES.md).
- **Screenshots to add:** the Tools > Lettras menu; a word search inserted in an editor; the style picker.
