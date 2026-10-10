# Publishing the Kotlin library to Maven Central

The library is `org.lettras.artificialss:lettras` (the `kotlin/` folder). The group `org.lettras.artificialss` sits under the
namespace `org.lettras`, which is the one you verify in the portal (see step 1). The Kotlin code lives in the package `org.lettras`. The Gradle build is already configured with the
[`com.vanniktech.maven.publish`](https://vanniktech.github.io/gradle-maven-publish-plugin/) plugin: it builds the library,
sources and documentation jars (Dokka), the POM with licenses, developer and SCM, signs everything, and uploads to the
Central Portal. Uploads are **manual-release**: the bundle waits in the portal until you press **Publish**, because a
Maven Central release can never be changed or deleted.

Run `cd kotlin && ./gradlew publishToMavenLocal` at any time to see exactly what would be published, in
`~/.m2/repository/org/lettras/lettras/<version>/`.

**Status:** `org.lettras.artificialss:lettras` is published (0.2.0 and 0.2.1; 0.3.0 is the next release):
<https://central.sonatype.com/artifact/org.lettras.artificialss/lettras>. The one-time setup below is done (namespace
`org.lettras` verified, signing key published); for the next version only the "Each release" steps are needed.

## One-time setup

1. **Claim the namespace `org.lettras`.** In the [Central Portal](https://central.sonatype.com): Namespaces, Add Namespace,
   `org.lettras`. A namespace is a reversed domain you control: `org.lettras` is proven on **`lettras.org`**. The portal shows a
   verification key; add it as a **TXT record** on the root of `lettras.org` (host `@`, value = that key) and click Verify.
   It sits next to the MCP Registry TXT record (`v=MCPv1; …`); several TXT records on the same name are fine.
   **Register exactly `org.lettras`, not `org.lettras.artificialss`.** Central checks the exact reversed domain of the namespace
   you request, so `org.lettras.artificialss` would be looked up on `artificialss.lettras.org`. Once `org.lettras` is verified
   you may publish under any group that starts with it, such as `org.lettras.artificialss`.
2. **Create a signing key** (Central rejects unsigned artifacts):
   ```bash
   gpg --full-generate-key                      # RSA 4096 or ed25519; use a passphrase; note the KEYID
   gpg --keyserver keyserver.ubuntu.com --send-keys KEYID   # Central looks the public key up on key servers
   ```
   Back up the private key and passphrase in a password manager.
3. **Generate a portal user token**: Central Portal, Account, Generate User Token. You get a username and a password; they
   are not your login.

## Where the secrets live (never in this repository)

| Secret | Where it goes |
| --- | --- |
| Portal user token (username and password) | `~/.gradle/gradle.properties` in your **home** folder, keys `mavenCentralUsername` and `mavenCentralPassword`, file mode 600 |
| Signing key and its passphrase | Outside the repo (we keep them in `~/.config/lettras/`); passed to Gradle as environment variables for one command |

`.gitignore` blocks key and credential file types (`*.asc`, `*.gpg`, `*.p12`, `maven-central.env`, …), and
`scripts/check-secrets.sh` fails if a tracked or staged file looks like a secret. Run it before pushing. Note that
`kotlin/gradle.properties` in this repo is tracked and holds only JVM settings: **never put the token in it**.

## Each release

1. Bump `version` in `kotlin/build.gradle.kts` (and mention the change in the README). Release from `main`.
2. Make sure `~/.gradle/gradle.properties` has the two token lines, then run (the key is read from your files, not typed):
   ```bash
   cd kotlin
   ORG_GRADLE_PROJECT_signingInMemoryKey="$(cat ~/.config/lettras/maven-signing-key.asc)" \
   ORG_GRADLE_PROJECT_signingInMemoryKeyPassword="$(cat ~/.config/lettras/maven-signing-passphrase)" \
   ./gradlew publishToMavenCentral
   ```
3. Open the [Deployments page](https://central.sonatype.com/publishing/deployments). Once validation passes, check the
   contents and press **Publish**. It appears on Maven Central after a few minutes (search can lag by an hour or two).
   A deployment that is not published yet can be dropped; a published one can never be changed or removed.

Signing is only switched on when `signingInMemoryKey` is set, so local builds and tests never need the key.

## Using it

```kotlin
dependencies {
    implementation("org.lettras.artificialss:lettras:0.3.0")
}
```
Gradle projects need `mavenCentral()` in their repositories (Android projects have it by default). The library brings
Chicory (a WebAssembly runtime in pure Java) and kotlinx.serialization with it.

## About the bundled engine

The compiled engine is inside the jar and is proprietary. The POM therefore lists **two licenses**: MIT for the library code
and the Lettras Engine License for the bundled binary. Keep both entries, and keep `LICENSE-ENGINE` in the jar.
