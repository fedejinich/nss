from __future__ import annotations

import argparse
import json
import os
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


def require(value: str | None, env_key: str) -> str:
    if value:
        return value
    loaded = os.environ.get(env_key, "").strip()
    if loaded:
        return loaded
    raise SystemExit(f"missing {env_key}; set env var or pass CLI argument")


def find_login_response(frames: list[bytes]) -> dict:
    for wire in frames:
        code, payload = parse_frame(wire)
        if code == 1:
            try:
                parsed = parse_login_response(payload)
            except Exception:
                continue
            parsed["raw_payload_hex"] = payload.hex()
            return parsed
    return {"ok": False, "reason": "NO_LOGIN_RESPONSE"}


def main() -> int:
    parser = argparse.ArgumentParser(description="Probe official Soulseek login version tuples")
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--out", default="")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)

    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    attempts: list[dict] = []
    for client_version, minor_version in DEFAULT_TUPLES:
        outbound = [sm_login(username, password, client_version, minor_version)]
        wire = send_server_sequence(server, outbound)
        response = find_login_response(wire)
        attempts.append(
            {
                "ts": now_iso(),
                "client_version": client_version,
                "minor_version": minor_version,
                "result": response,
            }
        )
        if response.get("ok") or response.get("reason") != "INVALIDVERSION":
            break

    payload = {
        "server": server,
        "username": username,
        "attempt_count": len(attempts),
        "attempts": attempts,
    }

    rendered = json.dumps(payload, indent=2, ensure_ascii=True)
    if args.out:
        out_path = Path(args.out)
        if not out_path.is_absolute():
            out_path = (REPO_ROOT / out_path).resolve()
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(rendered + "\n", encoding="utf-8")

    print(rendered)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
