# Golden Capture Notes

Golden runtime capture execution now writes raw artifacts under `captures/raw/<run_id>/` and then produces commit-safe redacted artifacts under `captures/redacted/<run_id>/`.

Run command:

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Default behavior:

- Raw capture is recorded to `captures/raw`.
- Redaction runs automatically (`AUTO_REDACT=1`) and writes `captures/redacted`.

Mandatory stage 2 scenarios:

1. `login-only`
2. `login-search`
3. `login-search-download`
4. `upload-deny`
5. `upload-accept`
