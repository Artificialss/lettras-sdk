# Listing the MCP server in registries

Registries list **MCP servers**. In this repository that is the server in [`mcp/`](../mcp). The npm package and the
Kotlin library are libraries, not MCP servers, so they are not listed in MCP registries (the Kotlin library belongs on
Maven Central when it is published).

## Official MCP Registry

The official registry (`registry.modelcontextprotocol.io`, in preview) only stores metadata. Our entry is
[`server.json`](../server.json): a remote server, `https://mcp.lettras.org/mcp`, over streamable HTTP, named
`org.lettras/word-search`. The `org.lettras` prefix is the reversed domain `lettras.org`, which has to be proven with
DNS.

**Before publishing**
1. `https://mcp.lettras.org/mcp` must be live and publicly reachable (the registry requires it).
2. Bump `version` in `server.json` for every new publish.

**Publish** (run from the repository root)

```bash
# 1. Install the publisher: https://github.com/modelcontextprotocol/registry/releases (or: brew install mcp-publisher)

# 2. Prove you own lettras.org: generate a key pair and a DNS TXT record
openssl genpkey -algorithm Ed25519 -out key.pem        # keep this private; *.pem is gitignored
PUBLIC_KEY="$(openssl pkey -in key.pem -pubout -outform DER | tail -c 32 | base64)"
echo "lettras.org. IN TXT \"v=MCPv1; k=ed25519; p=${PUBLIC_KEY}\""
#    Add that TXT record at the DNS provider of lettras.org (on the root name, not on "mcp"), wait a few minutes.

# 3. Log in with the key and publish
PRIVATE_KEY="$(openssl pkey -in key.pem -noout -text | grep -A3 "priv:" | tail -n +2 | tr -d ' :\n')"
mcp-publisher login dns --domain lettras.org --private-key "${PRIVATE_KEY}"
mcp-publisher validate
mcp-publisher publish

# 4. Check it
curl "https://registry.modelcontextprotocol.io/v0.1/servers?search=org.lettras/word-search"
```

Never commit `key.pem` or the private key value. Store them in a password manager.

Alternative without a domain proof: GitHub login (`mcp-publisher login github`) with the name
`io.github.Artificialss/word-search`.

## Other directories

Several sites aggregate or crawl MCP servers (for example Glama, Smithery, PulseMCP and mcp.so). Most of them read the
official registry or the public GitHub repository, and some have their own submit form. Each has its own rules and
they change often, so check the site's current instructions before submitting. A good README (this repository has one)
and a working public endpoint matter more than which directory you pick.
