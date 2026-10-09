#!/usr/bin/env bash
# Run before pushing: fails if a tracked or staged file looks like a secret.
#   scripts/check-secrets.sh
set -u
cd "$(git rev-parse --show-toplevel)"
fail=0
files=$( { git ls-files; git diff --cached --name-only; } | sort -u )

bad_names=$(echo "$files" | grep -E '\.(asc|gpg|pgp|p12|jks|kbx|keystore|pem|key)$|(^|/)\.env($|\.[^e])|maven-central\.env|maven-signing-|secring|signing\.properties' | grep -v '^\.env\.example$')
if [ -n "$bad_names" ]; then echo "Secret-looking file names are tracked or staged:"; echo "$bad_names"; fail=1; fi

# credentials and keys inside text files (binary engine files are skipped)
hits=$(echo "$files" | grep -vE '\.(wasm|jar)$|^npm/engine/index\.js$|Cargo\.lock$|^scripts/check-secrets\.sh$' | xargs -I{} grep -nIE \
  'BEGIN (PGP|RSA|EC|OPENSSH) PRIVATE KEY|PRIVATE KEY BLOCK|mavenCentral(Username|Password) *= *[^ ]+|signingInMemoryKey(Password)? *= *[^ ]+|eyJ[A-Za-z0-9_-]{20,}|npg_[A-Za-z0-9]{10,}|postgres(ql)?://[^ :]+:[^@ ]{6,}@' {} /dev/null 2>/dev/null | grep -vE '= *(<|PASTE|\$\(|\"\$\()')
if [ -n "$hits" ]; then echo "Credentials or keys found in files:"; echo "$hits" | cut -c1-110; fail=1; fi

if [ $fail -eq 0 ]; then echo "check-secrets: clean ($(echo "$files" | wc -l | tr -d ' ') files checked)"; fi
exit $fail
