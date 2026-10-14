# Lettras for Homebrew

The `lettras` command-line tool (word searches, crosswords, sudokus, Unicode styles) as a Homebrew formula. It installs the [npm package](https://www.npmjs.com/package/lettras) with
Homebrew's Node, so it works on macOS and Linux.

```bash
brew install artificialss/lettras/lettras     # taps Artificialss/homebrew-lettras on first use
lettras --words sol,luna,mar --rows 8 --cols 8 --random
```

## For maintainers

Homebrew taps are repositories named `homebrew-<name>`. The formula in `Formula/lettras.rb` is the source; copy it to the root `Formula/` folder of the repository
`Artificialss/homebrew-lettras`. To release a new version, change `url` and `sha256` (`shasum -a 256` of the tarball at
`https://registry.npmjs.org/lettras/-/lettras-<version>.tgz`) and run `brew test-bot` or `brew install --build-from-source` and `brew test lettras` against a local tap
(`brew tap-new local/lettras`).

## License

The formula is MIT. The compiled engine inside the npm package is proprietary: free to use unmodified, as distributed, in your own work (commercial included); see the package `LICENSE`.
