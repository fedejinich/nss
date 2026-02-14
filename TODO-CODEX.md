# TODO Execution Plan - NeoSoulSeek

## Stage 2 - Core P2P MVP + 25 mensajes

Dependency graph:

- `S2-T01 -> S2-T02`
- `S2-T01 -> S2-T03`
- `S2-T02 -> S2-T04`
- `S2-T03 -> S2-T04`
- `S2-T04 -> S2-T05`
- `S2-T04 -> S2-T06`
- `S2-T05 -> S2-T07`
- `S2-T06 -> S2-T07`
- `S2-T07 -> S2-T08`
- `S2-T08 -> S2-T09`
- `S2-T09 -> S2-T10`
- `S2-T10 -> S2-T11`

Tasks:

- id: S2-T01
  description: Publicar contrato de etapa (scope core P2P, 25 mensajes target, criterios de calidad)
  status: done
  depends_on: []

- id: S2-T02
  description: Ejecutar decompilacion orientada a 25 mensajes (callsites, handlers, serializers) con evidencia estatica
  status: done
  depends_on: [S2-T01]

- id: S2-T03
  description: Implementar pipeline de redaccion de capturas runtime (raw->redacted) y politica documental
  status: done
  depends_on: [S2-T01]

- id: S2-T04
  description: Correr capturas para escenarios core (login, search, download, upload accept/deny)
  status: done
  depends_on: [S2-T02, S2-T03]

- id: S2-T05
  description: Actualizar message_map/message_schema/name_map con 25 mensajes core y evidencia trazable
  status: done
  depends_on: [S2-T04]

- id: S2-T06
  description: Extender rust/protocol y rust/core para sesion core P2P + upload manual
  status: done
  depends_on: [S2-T04]

- id: S2-T07
  description: Extender rust/cli con comandos session/transfer/verify y compatibilidad temporal
  status: done
  depends_on: [S2-T05, S2-T06]

- id: S2-T08
  description: Implementar verificador diferencial sobre artefactos redacted (official vs NeoSoulSeek)
  status: done
  depends_on: [S2-T07]

- id: S2-T09
  description: Expandir suite de regresion (unit + integration + fixtures + capture replay)
  status: done
  depends_on: [S2-T08]

- id: S2-T10
  description: Cierre de etapa con auditoria de paridad core y reporte de gaps
  status: done
  depends_on: [S2-T09]

- id: S2-T11
  description: Publicar backlog de cobertura para mapear protocolo completo por lotes
  status: done
  depends_on: [S2-T10]

Notes:

- Los escenarios en `captures/redacted/*` fueron reemplazados por runs runtime generados desde `captures/raw/*`.
- Login al servidor oficial fue capturado en runtime; la respuesta observada en este lote fue `INVALIDVERSION` para la combinación de version enviada.

## Stage 2R - Runtime Capture Refresh + Confidence Promotion

Dependency graph:

- `S2R-T01 -> S2R-T02`
- `S2R-T02 -> S2R-T03`
- `S2R-T03 -> S2R-T04`
- `S2R-T04 -> S2R-T05`
- `S2R-T05 -> S2R-T06`
- `S2R-T06 -> S2R-T07`

Tasks:

- id: S2R-T01
  description: Generar capturas runtime reales para escenarios login/search/download/upload accept/deny y escribir raw manifests/frames
  status: done
  depends_on: []

- id: S2R-T02
  description: Reemplazar captures/redacted con artefactos derivados de runs runtime (raw->redacted)
  status: done
  depends_on: [S2R-T01]

- id: S2R-T03
  description: Promover los 7 mensajes medium a high con evidencia runtime enlazada valida
  status: done
  depends_on: [S2R-T02]

- id: S2R-T04
  description: Regenerar esquema/docs KB y validar gates de calidad
  status: done
  depends_on: [S2R-T03]

- id: S2R-T05
  description: Ejecutar scripts/run_diff_verify.sh con escenarios redacted reemplazados
  status: done
  depends_on: [S2R-T04]

- id: S2R-T06
  description: Ejecutar scripts/run_regression.sh completo
  status: done
  depends_on: [S2R-T05]

- id: S2R-T07
  description: Commit y push a main con evidencia runtime y promoción de confianza
  status: done
  depends_on: [S2R-T06]

## DOC-T0 - Institutionalizar documentacion continua

Dependency graph:

- `DOC-T01 -> DOC-T02`
- `DOC-T02 -> DOC-T03`
- `DOC-T03 -> DOC-T04`

Tasks:

- id: DOC-T01
  description: Crear/actualizar AGENTS.md con reglas obligatorias de mantenimiento de conocimiento
  status: done
  depends_on: []

- id: DOC-T02
  description: Publicar runbook de disciplina documental en Zensical docs
  status: done
  depends_on: [DOC-T01]

- id: DOC-T03
  description: Enlazar la disciplina documental desde docs/index.md como regla operativa
  status: done
  depends_on: [DOC-T02]

- id: DOC-T04
  description: Validar y registrar que TODO-CODEX/AGENTS/KB se actualizan en cada iteración
  status: done
  depends_on: [DOC-T03]

## Stage 3A - Login autenticado real + paridad semantica

Dependency graph:

- `S3A-T01 -> S3A-T02`
- `S3A-T02 -> S3A-T03`
- `S3A-T03 -> S3A-T04`
- `S3A-T04 -> S3A-T05`
- `S3A-T05 -> S3A-T06`
- `S3A-T06 -> S3A-T07`
- `S3A-T07 -> S3A-T08`

Tasks:

- id: S3A-T01
  description: Determinar tuple de version/login aceptada por servidor oficial y registrar evidencia runtime
  status: done
  depends_on: []

- id: S3A-T02
  description: Implementar codec/login request correcto (con md5hash) y parser tipado de login response
  status: done
  depends_on: [S3A-T01]

- id: S3A-T03
  description: Actualizar SessionClient login state machine para LoggedIn solo tras Success real
  status: done
  depends_on: [S3A-T02]

- id: S3A-T04
  description: Extender CLI y tools para credenciales seguras por env + probe de version
  status: done
  depends_on: [S3A-T03]

- id: S3A-T05
  description: Capturar escenarios runtime autenticados y refrescar raw->redacted
  status: done
  depends_on: [S3A-T04]

- id: S3A-T06
  description: Implementar verificador diferencial semantico y mantener modo bytes por compatibilidad
  status: done
  depends_on: [S3A-T05]

- id: S3A-T07
  description: Actualizar mapas/schema/docs/ledger y KB de estado con evidencia autenticada real
  status: done
  depends_on: [S3A-T06]

- id: S3A-T08
  description: Cierre de etapa con run_regression verde, PR documentado y retrospectiva de mantenibilidad
  status: done
  depends_on: [S3A-T07]

Notes:

- Tuple autenticado confirmado por runtime probe: `client_version=160`, `minor_version=1`.
- Login stateful validado: `LoggedIn` solo tras `LoginResponsePayload::Success`; en failure mantiene `Connected`.
- `scripts/run_diff_verify.sh` ahora corre por defecto en modo semantico (`VERIFY_MODE=semantic`) y mantiene fallback `bytes`.
- Capturas redacted obligatorias fueron regeneradas con credencial real (`login-only`, `login-search`, `login-search-download`, `upload-deny`, `upload-accept`).
