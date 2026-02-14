# Verification Status

## Objective

Asegurar trazabilidad de evidencia y paridad protocolar para el scope core de Stage 2.

## Gates

- KB validation:

```bash
python3 scripts/kb_validate.py
```

Checks:

- Name/data maps con evidencia válida.
- `message_map.csv` con `25/25` mensajes core y umbrales de confianza.
- `message_schema.json` con evidencia enlazada y umbrales de confianza.

## Differential Verification

```bash
scripts/run_diff_verify.sh
```

Runs:

1. Fixture parity (`captures/fixtures/*`).
2. Capture parity para runs obligatorios en `captures/redacted/*`.

## Regression Suite

```bash
scripts/run_regression.sh
```

Includes:

1. Python unit tests (`tests/kb`, `tests/protocol`, `tests/runtime`).
2. Rust unit/integration tests (`cargo test`).
3. KB validation gates.
4. Differential verification gates.

## Residual Risk

- Runs redacted actuales provienen de runtime (`captures/raw/*`) y validan `high=25` para core.
- Login al servidor oficial en este lote devuelve `INVALIDVERSION`; se requiere ajustar tuple de versión para autenticación completa end-to-end.
