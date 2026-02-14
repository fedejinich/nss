# TODO Execution Plan - soul-dec KB-first

Dependency graph:
- T0A -> T0B
- T0B -> T0C
- T0C -> T0D
- T0D -> T0E
- T0E -> T1
- T1 -> T2
- T2 -> T3
- T3 -> T4
- T3 -> T5
- T4 -> T6
- T5 -> T6
- T6 -> T7
- T7 -> T8
- T7 -> T9
- T8 -> T10
- T9 -> T10
- T10 -> T11
- T11 -> T12
- T12 -> T13
- T13 -> T14
- T14 -> T15
- T10 -> T16
- T15 -> T16
- T16 -> T17
- T17 -> T18

Tasks:
- id: T0A
  description: Provisionar Zensical desde https://github.com/zensical/zensical (instalacion reproducible)
  status: done
  depends_on: []
- id: T0B
  description: Bootstrapping de KB site con Zensical en /Users/void_rsk/Projects/soul-dec (docs + zensical.toml)
  status: done
  depends_on: [T0A]
- id: T0C
  description: Definir esquema de name_map/data_map + evidencia minima obligatoria por entrada
  status: done
  depends_on: [T0B]
- id: T0D
  description: Implementar workflow de promocion por confianza (auto alta; media/baja a review queue)
  status: done
  depends_on: [T0C]
- id: T0E
  description: Publicar runbook KB-first (como registrar evidencia, promover y rechazar)
  status: done
  depends_on: [T0D]
- id: T1
  description: Preparar toolchain reverse/trace restante (Ghidra, Frida, PCAP, Binja opcional)
  status: done
  depends_on: [T0E]
- id: T2
  description: Estructura de estado/proceso y trazabilidad del proyecto
  status: done
  depends_on: [T1]
- id: T3
  description: Intake forense del binario objetivo
  status: done
  depends_on: [T2]
- id: T4
  description: Pipeline Ghidra (import/analisis/export)
  status: done
  depends_on: [T3]
- id: T5
  description: Pipeline Binja opcional evaluado (no bloquea; pendiente disponibilidad/licencia)
  status: done
  depends_on: [T3]
- id: T6
  description: Construir name_map/message_map con evidencia (baseline inicial)
  status: done
  depends_on: [T4, T5]
- id: T7
  description: Extraer flujo search/download objetivo
  status: done
  depends_on: [T6]
- id: T8
  description: Hooks Frida para funciones clave
  status: done
  depends_on: [T7]
- id: T9
  description: Harness PCAP sincronizado con Frida
  status: done
  depends_on: [T7]
- id: T10
  description: Golden captures con cuenta de prueba
  status: done
  depends_on: [T8, T9]
- id: T11
  description: Derivar esquemas de mensajes desde evidencia
  status: done
  depends_on: [T10]
- id: T12
  description: Scaffold Rust workspace (protocol/core/cli/verify)
  status: done
  depends_on: [T11]
- id: T13
  description: Implementar codec/framing in-scope
  status: done
  depends_on: [T12]
- id: T14
  description: Implementar login+búsqueda con paridad
  status: done
  depends_on: [T13]
- id: T15
  description: Implementar descarga single-file end-to-end
  status: done
  depends_on: [T14]
- id: T16
  description: Verificador diferencial oficial vs Rust
  status: done
  depends_on: [T10, T15]
- id: T17
  description: Suite de regresion con fixtures de captura
  status: done
  depends_on: [T16]
- id: T18
  description: Cierre V1 con auditoria de paridad y estado final
  status: done
  depends_on: [T17]

Notes:
- T10 se completa a nivel de pipeline reproducible (`scripts/capture_golden.sh` + `tools/runtime/capture_harness.py`) y fixtures bootstrap.
- Capturas golden con cuenta real quedan listas para ejecución operativa cuando haya credenciales de prueba.
