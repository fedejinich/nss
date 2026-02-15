# Release Packaging Runbook

This runbook defines the reproducible packaging flow for NeoSoulSeek release binaries.

## Scope

The release bundle currently includes:

1. `soul-cli`
2. `soul-tui`

## Build and Package

```bash
scripts/package_release.sh
```

Optional custom version label:

```bash
scripts/package_release.sh v1-custom-tag
```

## Output Layout

Output is generated under:

- `dist/releases/<version>/`

Expected files:

1. `bin/soul-cli`
2. `bin/soul-tui`
3. `SHA256SUMS.txt`
4. `README.txt`

## Verification

```bash
cd dist/releases/<version>
shasum -a 256 -c SHA256SUMS.txt
```

## Notes

1. `dist/` is intentionally local and not committed.
2. Packaging is additive and does not modify source artifacts.
