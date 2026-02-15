# NeoSoulSeek

NeoSoulSeek is an evolvable Soulseek client project focused on:

- Protocol coverage with evidence-backed mapping.
- A practical SDK/CLI surface.
- A minimal TUI for real search and download flows.

For detailed status, roadmap, and technical documentation, use the Zensical knowledge base.

## Quick Start

### 1. Configure local credentials

```bash
./scripts/setup_credentials_wizard.sh
set -a; source .env.local; set +a
```

### 2. Use the CLI

```bash
cd rust
cargo run -q -p soul-cli -- --help

# login
cargo run -q -p soul-cli -- session login --server "$NSS_TEST_SERVER" --username "$NSS_TEST_USERNAME" --password "$NSS_TEST_PASSWORD"

# search
cargo run -q -p soul-cli -- session search --server "$NSS_TEST_SERVER" --username "$NSS_TEST_USERNAME" --password "$NSS_TEST_PASSWORD" --token 123 --query "aphex twin"

# automated search -> select -> download
cargo run -q -p soul-cli -- session download-auto --server "$NSS_TEST_SERVER" --username "$NSS_TEST_USERNAME" --password "$NSS_TEST_PASSWORD" --token 123 --query "aphex twin" --output /tmp --transfer-token 555
```

### 3. Launch the TUI

```bash
set -a; source .env.local; set +a
export NSS_TUI_QUERY="aphex twin"
export NSS_TUI_OUTPUT_DIR="/tmp"

cd rust
cargo run -q -p soul-tui
```

### 4. Open the knowledge base (Zensical)

```bash
scripts/setup_zensical.sh
./.venv-tools/bin/zensical build -f zensical.toml
./.venv-tools/bin/zensical serve -f zensical.toml -a 127.0.0.1:8000
```

Then open: `http://127.0.0.1:8000`
