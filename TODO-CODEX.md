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

- Los escenarios en `captures/redacted/*` estan versionados como `synthetic_fixture_replay` para validacion deterministica.
- Reemplazar por capturas reales con cuenta de prueba cuando el operador ejecute sesiones autenticadas.
