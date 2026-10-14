class Lettras < Formula
  desc "Word-search, crossword and sudoku generator and Unicode text styles for 6 languages"
  homepage "https://www.lettras.org"
  url "https://registry.npmjs.org/lettras/-/lettras-0.5.1.tgz"
  sha256 "28f0d52e75b01b811eae917d60925516c8911aa6d193b31f45e724d7e1e5bd6d"
  license :cannot_represent # MIT wrapper; the compiled engine is proprietary, see the package LICENSE

  depends_on "node"

  def install
    system "npm", "install", *std_npm_args
    bin.install_symlink Dir["#{libexec}/bin/*"]
  end

  test do
    output = shell_output("#{bin}/lettras --words sol,luna,mar --rows 6 --cols 6 --seed 1")
    assert_match "SOL", output.upcase
    assert_match "LUNA", output.upcase
  end
end
