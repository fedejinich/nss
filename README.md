# NeoSoulSeek

NeoSoulSeek es un proyecto KB-first para construir una app Soulseek evolutiva propia.

Objetivo actual:

- Entregar funcionalidades core (`login`, `search`, `download` single-file, `upload` manual accept/deny).
- Mapear formalmente el protocolo Soulseek con evidencia trazable.
- Iterar por etapas, sin buscar un clon exacto del cliente oficial.

## Requisitos

- `python3`
- `git`
- `cargo` (Rust)
- macOS + `tcpdump` para capturas runtime

## Levantar Zensical

1. Instalar Zensical en el virtualenv del repo:

```bash
scripts/setup_zensical.sh
```

2. Build del sitio KB:

```bash
./.venv-tools/bin/zensical build -f zensical.toml
```

3. Servir local:

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

### 2. Derivación de esquema de protocolo

```bash
scripts/derive_message_schema.sh
```

Artefactos:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/re/static/message-schema.md`

### 3. Capturas runtime (raw -> redacted)

Sesión estándar:

```bash
scripts/capture_session.sh
```

Escenario específico:

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Redacción manual:

```bash
RUN_DIR=captures/raw/<run_id> scripts/redact_capture_run.sh
```

Política:

- Raw local no versionado: `captures/raw/*`
- Redacted versionado: `captures/redacted/*`

### 4. SDK/CLI Rust

Tests:

```bash
cd rust
cargo test
```

Comandos principales:

```bash
cd rust
cargo run -q -p soul-cli -- session login --server <host:port> --username <user> --password-md5 <md5>
cargo run -q -p soul-cli -- session search --server <host:port> --username <user> --password-md5 <md5> --token 123 --query "aphex twin"
cargo run -q -p soul-cli -- transfer download --peer <host:port> --token 555 --path "Music\\Track.flac" --size 1234 --output /tmp/out.bin
cargo run -q -p soul-cli -- transfer serve-upload --manual --decision accept --source-file /tmp/file.bin
cargo run -q -p soul-cli -- verify captures --run login-search-download --base-dir ../captures/redacted
```

### 5. Verificación diferencial y regresión

```bash
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

## Estructura

- `analysis/`: mapas autoritativos y artefactos estáticos.
- `captures/`: fixtures, raw local, redacted versionado.
- `docs/`: runbooks, estado y evidencia.
- `evidence/`: evidencia forense/reverse.
- `frida/`: hooks runtime.
- `rust/`: `protocol`, `core`, `cli`, `verify`.
- `scripts/`: workflows reproducibles.
- `tools/`: utilidades KB/protocol/runtime.
