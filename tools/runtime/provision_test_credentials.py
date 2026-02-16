from __future__ import annotations

import argparse
import json
import os
import random
import string
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import now_iso, parse_frame, parse_login_response, send_server_sequence, sm_login


DEFAULT_TUPLES: list[tuple[int, int]] = [(160, 1), (157, 19), (157, 17), (157, 100)]


def load_dotenv_local(repo_root: Path) -> None:
    env_path = repo_root / ".env.local"
    if not env_path.exists():
        return
    for raw in env_path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        if not key or key in os.environ:
            continue
        os.environ[key] = value.strip().strip('"').strip("'")


def require_server(cli_server: str) -> str:
    if cli_server.strip():
        return cli_server.strip()
    env_server = os.environ.get("NSS_TEST_SERVER", "").strip()
    if env_server:
        return env_server
    return "server.slsknet.org:2416"


def random_username() -> str:
    suffix = "".join(random.choice(string.ascii_lowercase + string.digits) for _ in range(10))
    return f"nss_auto_{suffix}"


def random_password() -> str:
    alphabet = string.ascii_letters + string.digits
    token = "".join(random.choice(alphabet) for _ in range(14))
    return f"P{token}"


def probe_login(server: str, username: str, password: str) -> tuple[dict, tuple[int, int]]:
    for client_version, minor_version in DEFAULT_TUPLES:
        wire = send_server_sequence(server, [sm_login(username, password, client_version, minor_version)])
        for frame_bytes in wire:
            code, payload = parse_frame(frame_bytes)
            if code != 1:
                continue
            try:
                result = parse_login_response(payload)
            except Exception:
                continue
            result["raw_payload_hex"] = payload.hex()
            result["client_version"] = client_version
            result["minor_version"] = minor_version
            return result, (client_version, minor_version)
    return {"ok": False, "reason": "NO_LOGIN_RESPONSE"}, DEFAULT_TUPLES[-1]


def upsert_env_local(repo_root: Path, entries: dict[str, str]) -> None:
    env_path = repo_root / ".env.local"
    rows: dict[str, str] = {}
    if env_path.exists():
        for raw in env_path.read_text(encoding="utf-8").splitlines():
            line = raw.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, value = line.split("=", 1)
            rows[key.strip()] = value.strip()

    rows.update(entries)
    rendered = [f"{key}={value}" for key, value in sorted(rows.items())]
    env_path.write_text("\n".join(rendered) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Provision local NeoSoulSeek test credentials and validate real login"
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--write-env-local", action="store_true")
    parser.add_argument("--out", default="")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)

    server = require_server(args.server)
    username = args.username.strip() or random_username()
    password = args.password.strip() or random_password()

    result, accepted_tuple = probe_login(server, username, password)
    success = bool(result.get("ok"))

    payload = {
        "ts": now_iso(),
        "server": server,
        "username": username,
        "password": password,
        "accepted_tuple": {
            "client_version": accepted_tuple[0],
            "minor_version": accepted_tuple[1],
        },
        "login_result": result,
        "success": success,
    }

    if args.write_env_local and success:
        upsert_env_local(
            REPO_ROOT,
            {
                "NSS_TEST_SERVER": server,
                "NSS_TEST_USERNAME": username,
                "NSS_TEST_PASSWORD": password,
            },
        )
        payload["env_local_written"] = str(REPO_ROOT / ".env.local")

    rendered = json.dumps(payload, indent=2, ensure_ascii=True)
    if args.out:
        out_path = Path(args.out)
        if not out_path.is_absolute():
            out_path = (REPO_ROOT / out_path).resolve()
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(rendered + "\n", encoding="utf-8")

    print(rendered)
    return 0 if success else 1


if __name__ == "__main__":
    raise SystemExit(main())
