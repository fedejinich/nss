# Golden Captures

This directory stores canonical synchronized sessions used for parity checks.

Required contents per run:

- `manifest.json`
- `frida-events.jsonl`
- `traffic.pcap` (unless explicitly skipped in manifest)

Run command:

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Naming convention:

- `captures/golden/<YYYYMMDDTHHMMSSZ>-<scenario>/`

Minimum scenarios:

1. `login-only`
2. `login-search`
3. `login-search-download`
