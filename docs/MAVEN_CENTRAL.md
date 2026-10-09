# Publishing the Kotlin library to Maven Central

The library is `org.lettras:lettras` (the `kotlin/` folder), and the Kotlin code lives in the package `org.lettras` too. The Gradle build is already configured with the
[`com.vanniktech.maven.publish`](https://vanniktech.github.io/gradle-maven-publish-plugin/) plugin: it builds the library,
sources and documentation jars (Dokka), the POM with licenses, developer and SCM, signs everything, and uploads to the
Central Portal. Uploads are **manual-release**: the bundle waits in the portal until you press **Publish**, because a
Maven Central release can never be changed or deleted.

Run `cd kotlin && ./gradlew publishToMavenLocal` at any time to see exactly what would be published, in
`~/.m2/repository/org/lettras/lettras/<version>/`.

## One-time setup

1. **Claim the namespace `org.lettras`.** In the [Central Portal](https://central.sonatype.com): Namespaces, Add Namespace,
   `org.lettras`. A namespace is a reversed domain you control: `org.lettras` is proven on **`lettras.org`**. The portal shows a
   verification key; add it as a **TXT record** on the root of `lettras.org` (host `@`, value = that key) and click Verify.
   It sits next to the MCP Registry TXT record (`v=MCPv1; …`); several TXT records on the same name are fine. Central checks
   the exact reversed domain, so a record on a subdomain does not count, and the namespace must be exactly `org.lettras`
   (a longer one such as `org.lettras.sdk` would be a different request).
2. **Create a signing key** (Central rejects unsigned artifacts):
   ```bash
   gpg --full-generate-key                      # RSA 4096 or ed25519; use a passphrase; note the KEYID
   gpg --keyserver keyserver.ubuntu.com --send-keys KEYID   # Central looks the public key up on key servers
   ```
   Back up the private key and passphrase in a password manager.
3. **Generate a portal user token**: Central Portal, Account, Generate User Token. You get a username and a password; they
   are not your login.

## Each release

1. Bump `version` in `kotlin/build.gradle.kts` (and mention the change in the README).
2. From a shell (never commit these values):
   ```bash
   export ORG_GRADLE_PROJECT_mavenCentralUsername='<token username>'
   export ORG_GRADLE_PROJECT_mavenCentralPassword='<token password>'
   export ORG_GRADLE_PROJECT_signingInMemoryKey="$(gpg --armor --export-secret-keys KEYID)"
   export ORG_GRADLE_PROJECT_signingInMemoryKeyPassword='<key passphrase>'
   cd kotlin && ./gradlew publishToMavenCentral
   ```
3. Open the [Deployments page](https://central.sonatype.com/publishing/deployments). Once validation passes, check the
   contents and press **Publish**. It appears on Maven Central after a few minutes (search can lag by an hour or two).

Signing is only switched on when `signingInMemoryKey` is set, so local builds and tests never need the key.

## Using it

```kotlin
dependencies {
    implementation("org.lettras:lettras:0.2.0")
}
```
Gradle projects need `mavenCentral()` in their repositories (Android projects have it by default). The library brings
Chicory (a WebAssembly runtime in pure Java) and kotlinx.serialization with it.

## About the bundled engine

The compiled engine is inside the jar and is proprietary. The POM therefore lists **two licenses**: MIT for the library code
and the Lettras Engine License for the bundled binary. Keep both entries, and keep `LICENSE-ENGINE` in the jar.
