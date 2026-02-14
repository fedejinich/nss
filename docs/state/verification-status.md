# Verification Status

## Objective

Asegurar trazabilidad de evidencia y paridad protocolar para Stage 3A (`auth+search real` + diff semantico).

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
3. Modo por defecto: `semantic` (`VERIFY_MODE=semantic`), con compatibilidad `bytes`.

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

- `code=64` se normaliza con parser summary fallback (`room-list`) para mantener comparación semántica estable.
- Búsqueda real en esta etapa usa parseo `summary` (no parseo exhaustivo de todos los atributos opcionales de resultados).

## Current Auth Evidence

- Login autenticado real verificado contra servidor oficial con tuple `160/1`.
- `session.login` y `session.search` reales validados con credencial de prueba local (`.env.local` no versionado).
- Capturas obligatorias redacted refrescadas y verificadas en modo semántico:
  - `login-only`
  - `login-search`
  - `login-search-download`
  - `upload-deny`
  - `upload-accept`
