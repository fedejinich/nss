# Stage 2 Parity Audit

## Date

- 2026-02-14

## Scope Auditado

- Core P2P MVP:
  - Login servidor.
  - Búsqueda.
  - Descarga single-file.
  - Upload manual accept/deny.
- Contrato de 25 mensajes core.

## Resultado

- `message_map.csv`: `25/25` mensajes presentes.
- Confianza: `high=25`, `medium=0`, `low=0`.
- `message_schema.json`: cobertura `25/25` con evidencia enlazada.
- `scripts/run_diff_verify.sh`: fixture diff + capture diff por escenario obligatorio.
- `scripts/run_regression.sh`: verde con Python + Rust + KB validate + diff verify.
- `captures/redacted/*`: reemplazado con runs runtime derivados desde `captures/raw/*`.

## Gaps abiertos

1. Login runtime actual responde `INVALIDVERSION` con la versión enviada; falta fijar tuple de versión aceptada para sesión autenticada exitosa.
2. Differential normalizer aún compara frames completos; se puede extender a normalización semántica por campo.

## Recomendación siguiente

- Resolver tuple de versión de login aceptada por servidor y repetir lote login/search/download con sesión autenticada completa.
