from __future__ import annotations

import argparse
import hashlib
import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

IP_RE = re.compile(r"\b(?:\d{1,3}\.){3}\d{1,3}(?::\d{1,5})?\b")
UNIX_PATH_RE = re.compile(r"/(?:[^\s/]+/)+[^\s/]+")
WIN_PATH_RE = re.compile(r"[A-Za-z]:\\(?:[^\\\s]+\\)*[^\\\s]+")

USER_KEYS = {"user", "username", "from_user", "to_user", "nick", "peer_user", "target_user", "login_as"}
ADDR_KEYS = {"ip", "address", "ip_address", "peer_addr", "server", "host"}
PATH_KEYS = {"path", "virtual_path", "source_file", "target_path", "output_path", "file", "file_path", "profile_root"}
TEXT_KEYS = {"message", "text", "chat_message", "private_message", "payload_sample", "value_sample", "note"}
SECRET_KEYS = {"password", "passwd", "password_md5", "md5hash"}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def hash_token(kind: str, value: str, salt: str) -> str:
    digest = hashlib.sha256(f"{salt}|{kind}|{value}".encode("utf-8")).hexdigest()[:12]
    return f"<redacted:{kind}:{digest}>"


def path_ref(path: Path, *, repo_root: Path) -> str:
    path_resolved = path.resolve()
    repo_resolved = repo_root.resolve()
    try:
        return path_resolved.relative_to(repo_resolved).as_posix()
    except ValueError:
        digest = hashlib.sha256(str(path_resolved).encode("utf-8")).hexdigest()[:12]
        return f"<external:path:{digest}>"


def redact_string(value: str, *, key: str, salt: str, stats: dict[str, int]) -> str:
    lowered_key = key.lower()

    if lowered_key in USER_KEYS:
        stats["username"] = stats.get("username", 0) + 1
        return hash_token("username", value, salt)

    if lowered_key in ADDR_KEYS:
        stats["endpoint"] = stats.get("endpoint", 0) + 1
        return hash_token("endpoint", value, salt)

    if lowered_key in PATH_KEYS:
        stats["path"] = stats.get("path", 0) + 1
        return hash_token("path", value, salt)

    if lowered_key in TEXT_KEYS:
        stats["text"] = stats.get("text", 0) + 1
        return hash_token("text", value, salt)

    if lowered_key in SECRET_KEYS:
        stats["secret"] = stats.get("secret", 0) + 1
        return hash_token("secret", value, salt)

    def repl_ip(match: re.Match[str]) -> str:
        stats["endpoint"] = stats.get("endpoint", 0) + 1
        return hash_token("endpoint", match.group(0), salt)

    def repl_unix(match: re.Match[str]) -> str:
        stats["path"] = stats.get("path", 0) + 1
        return hash_token("path", match.group(0), salt)

    def repl_win(match: re.Match[str]) -> str:
        stats["path"] = stats.get("path", 0) + 1
        return hash_token("path", match.group(0), salt)

    redacted = IP_RE.sub(repl_ip, value)
    redacted = UNIX_PATH_RE.sub(repl_unix, redacted)
    redacted = WIN_PATH_RE.sub(repl_win, redacted)
    return redacted


def redact_obj(obj: Any, *, key: str, salt: str, stats: dict[str, int]) -> Any:
    if isinstance(obj, dict):
        return {k: redact_obj(v, key=k, salt=salt, stats=stats) for k, v in obj.items()}
    if isinstance(obj, list):
        return [redact_obj(v, key=key, salt=salt, stats=stats) for v in obj]
    if isinstance(obj, str):
        return redact_string(obj, key=key, salt=salt, stats=stats)
    return obj


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows

    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            rows.append({"raw_line": line})
    return rows


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as fh:
        for row in rows:
            fh.write(json.dumps(row, ensure_ascii=True) + "\n")


def copy_hex_lines(source: Path, target: Path) -> int:
    if not source.exists():
        return 0

    lines = []
    for raw in source.read_text(encoding="utf-8").splitlines():
        line = raw.strip().lower()
        if not line or line.startswith("#"):
            continue
        lines.append(line)

    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("\n".join(lines) + ("\n" if lines else ""), encoding="utf-8")
    return len(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Redact raw capture run into a committable artifact set")
    parser.add_argument("--run-dir", required=True, help="Path to captures/raw/<run_id>")
    parser.add_argument("--out-root", default="captures/redacted")
    parser.add_argument("--run-id", default="")
    parser.add_argument("--salt", default="")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    run_dir = Path(args.run_dir)
    if not run_dir.is_absolute():
        run_dir = (repo_root / run_dir).resolve()

    if not run_dir.exists():
        raise SystemExit(f"run-dir not found: {run_dir}")

    run_id = args.run_id or run_dir.name
    salt = args.salt or run_id

    out_root = Path(args.out_root)
    if not out_root.is_absolute():
        out_root = (repo_root / out_root).resolve()
    out_dir = out_root / run_id
    out_dir.mkdir(parents=True, exist_ok=True)

    stats: dict[str, int] = {}

    manifest_candidates = [run_dir / "manifest.raw.json", run_dir / "manifest.json"]
    manifest_src = next((p for p in manifest_candidates if p.exists()), None)
    if manifest_src is None:
        raise SystemExit(f"manifest not found in {run_dir}")

    manifest = read_json(manifest_src)
    manifest_redacted = redact_obj(manifest, key="manifest", salt=salt, stats=stats)
    manifest_redacted["redaction"] = {
        "policy": "redact+commit",
        "policy_version": "2",
        "generated_at": now_iso(),
        "raw_source": path_ref(run_dir, repo_root=repo_root),
        "payload_sampling": {
            "mode": "sampled",
            "max_payload_chars": 160,
            "sensitive_fields_redacted": True,
        },
        "stats": stats,
    }

    write_json(out_dir / "manifest.redacted.json", manifest_redacted)

    frida_src_candidates = [run_dir / "frida-events.raw.jsonl", run_dir / "frida-events.jsonl"]
    frida_src = next((p for p in frida_src_candidates if p.exists()), None)
    if frida_src is not None:
        frida_rows = read_jsonl(frida_src)
        frida_redacted = [redact_obj(row, key="frida_event", salt=salt, stats=stats) for row in frida_rows]
        write_jsonl(out_dir / "frida-events.redacted.jsonl", frida_redacted)

    io_src_candidates = [run_dir / "io-events.raw.jsonl", run_dir / "io-events.jsonl"]
    io_src = next((p for p in io_src_candidates if p.exists()), None)
    if io_src is not None:
        io_rows = read_jsonl(io_src)
        io_redacted = [redact_obj(row, key="io_event", salt=salt, stats=stats) for row in io_rows]
        write_jsonl(out_dir / "io-events.redacted.jsonl", io_redacted)

    official_candidates = [
        run_dir / "official_frames.raw.hex",
        run_dir / "frames.raw.hex",
        run_dir / "frames.hex",
    ]
    official_src = next((p for p in official_candidates if p.exists()), None)
    if official_src is not None:
        copy_hex_lines(official_src, out_dir / "official_frames.hex")

    neo_candidates = [run_dir / "neo_frames.raw.hex", run_dir / "neo_frames.hex"]
    neo_src = next((p for p in neo_candidates if p.exists()), None)
    if neo_src is not None:
        copy_hex_lines(neo_src, out_dir / "neo_frames.hex")

    notes_src = run_dir / "notes.raw.txt"
    if notes_src.exists():
        notes = redact_string(notes_src.read_text(encoding="utf-8"), key="text", salt=salt, stats=stats)
        (out_dir / "notes.redacted.txt").write_text(notes, encoding="utf-8")

    summary = {
        "run_id": run_id,
        "created_at": now_iso(),
        "raw_dir": path_ref(run_dir, repo_root=repo_root),
        "redacted_dir": path_ref(out_dir, repo_root=repo_root),
        "stats": stats,
        "artifacts": {
            "manifest": path_ref(out_dir / "manifest.redacted.json", repo_root=repo_root),
            "frida_events": path_ref(out_dir / "frida-events.redacted.jsonl", repo_root=repo_root),
            "io_events": path_ref(out_dir / "io-events.redacted.jsonl", repo_root=repo_root),
            "official_frames": path_ref(out_dir / "official_frames.hex", repo_root=repo_root),
            "neo_frames": path_ref(out_dir / "neo_frames.hex", repo_root=repo_root),
        },
    }
    write_json(out_dir / "redaction-summary.json", summary)

    print(json.dumps({"run_id": run_id, "redacted_dir": str(out_dir)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
