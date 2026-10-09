# Security policy

## Reporting a vulnerability

Please do **not** open a public issue for a security problem. Email **info@artificialss.ai** with the subject
"Security: Lettras", saying what you found and how to reproduce it. We aim to reply within a few working days and will
credit you if you wish.

## What is in scope

- The `lettras` npm package, the Kotlin library `org.lettras.artificialss:lettras`, and the hosted MCP server at
  `https://mcp.lettras.org/mcp`.
- Things like crashes or hangs triggered by crafted input, ways around the free-tier limit that expose us to cost, leaks of
  data, or supply-chain problems in the published artifacts.

Out of scope: the compiled engine's internals (it is proprietary; reverse engineering is not permitted by its license),
denial-of-service by sheer traffic, and findings in third-party services we use.

## Supported versions

The latest release of each package. Releases are signed: Maven artifacts with key `48A6307F56EF491A`
(published on `keyserver.ubuntu.com` and `keys.openpgp.org`).

## What the hosted MCP server stores

A SHA-256 hash of the caller's IP address (IPv6 collapsed to its /64), a count and a timestamp, for the free-tier limit of 5
puzzles, kept for as long as the limit lasts (forever). No addresses, words or puzzles are stored. See the README for details.
