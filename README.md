# NeoSoulSeek

NeoSoulSeek es un proyecto de reverse engineering y reconstrucción de protocolo del cliente SoulseekQt.

## Requisitos

- `python3`
- `git`
- `cargo` (Rust)
- macOS con herramientas de red si querés capturas (`tcpdump`)

## Levantar Zensical

1. Instalar Zensical en el virtualenv del repo:

```bash
scripts/setup_zensical.sh
```

2. Build del sitio de conocimiento:

```bash
./.venv-tools/bin/zensical build -f zensical.toml
```

3. Servir la KB localmente:

```bash
./.venv-tools/bin/zensical serve -f zensical.toml -a 127.0.0.1:8000
```

Sitio local: `http://127.0.0.1:8000`

## Uso del proyecto

### 1. Flujo KB-first (obligatorio)

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
```

### 2. Extracción estática de flujo search/download

```bash
scripts/extract_search_download_flow.sh
scripts/derive_message_schema.sh
```

Artefactos clave:

- `analysis/re/flow_graph.json`
- `analysis/protocol/message_schema.json`
- `docs/re/static/search-download-flow.md`
- `docs/re/static/message-schema.md`

### 3. Capturas runtime (Frida + PCAP)

Sesión normal:

```bash
scripts/capture_session.sh
```

Sesión golden:

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

### 4. Implementación y verificación Rust

Tests del workspace:

```bash
cd rust
cargo test
```

Verificación diferencial con fixtures:

```bash
cd ..
scripts/run_diff_verify.sh
```

Reporte:

- `captures/fixtures/verify-report.json`

### 5. Regresión completa

```bash
scripts/run_regression.sh
```

## Estructura del repositorio

- `analysis/`: mapas autoritativos, esquemas y artefactos estáticos.
- `docs/`: runbooks, estado del proyecto y ledger de evidencia.
- `evidence/`: evidencia forense y de reversing.
- `frida/`: hooks runtime.
- `rust/`: crates `protocol`, `core`, `cli`, `verify`.
- `scripts/`: workflows reproducibles.
