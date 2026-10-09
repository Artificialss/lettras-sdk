# Changelog

All three distributions (npm `lettras`, Maven `org.lettras.artificialss:lettras`, the MCP server) are versioned together.

## 0.2.1
- **Fix:** `engineVersion` now reports the real engine version (it said `0.1.0` since the first release).
- **MCP:** IPv6 addresses in the same `/64` count as one client, and IPv4-mapped IPv6 as IPv4, so rotating addresses no longer
  dodges the free limit. `fill_word_search` has its own limit (100 per day). Usage counters are now actually cleaned up (the
  cleanup condition could never be true, so the table would have grown forever). Store failures are logged as a
  `usage_store_error` line with a safe category, never connection details.
- **Engine hardening:** limits on grid size, word count, word length and cell length; a letter or digit as the empty-cell
  marker is rejected (it made grids ambiguous); a fuzz test with 100,000 hostile inputs finds no crash.
- **Docs:** filler example with real output, limits, privacy notes, Android status, ESM note.

## 0.2.0
- `fill`: random letters for the empty cells, by language, with accents on or off (npm `fill()` and CLI `--random`, Kotlin
  `Lettras.fill()`, MCP tool `fill_word_search`).
- Kotlin library published to Maven Central; MCP listed in the official MCP Registry; shared Postgres counter for the free tier.

## 0.1.0
- npm library and CLI, Kotlin library, Rust MCP server (`generate_word_search`, `list_languages`), 5 free puzzles per day.
