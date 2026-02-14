from __future__ import annotations

import hashlib
import socket
import struct
import time
from datetime import datetime, timezone
from typing import Any


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def u32(value: int) -> bytes:
    return struct.pack("<I", value)


def u64(value: int) -> bytes:
    return struct.pack("<Q", value)


def s(value: str) -> bytes:
    payload = value.encode("utf-8")
    return u32(len(payload)) + payload


def frame(code: int, payload: bytes) -> bytes:
    body = u32(code) + payload
    return u32(len(body)) + body


def compute_login_md5hash(username: str, password: str) -> str:
    return hashlib.md5(f"{username}{password}".encode("utf-8")).hexdigest()


def sm_login(username: str, password: str, client_version: int, minor_version: int) -> bytes:
    md5hash = compute_login_md5hash(username, password)
    return frame(1, s(username) + s(password) + u32(client_version) + s(md5hash) + u32(minor_version))


def parse_frame(frame_bytes: bytes) -> tuple[int, bytes]:
    if len(frame_bytes) < 8:
        raise ValueError(f"frame too short: {len(frame_bytes)}")
    code = struct.unpack("<I", frame_bytes[4:8])[0]
    return code, frame_bytes[8:]


def parse_login_response(payload: bytes) -> dict[str, Any]:
    if not payload:
        return {"ok": False, "error": "empty_payload"}

    offset = 0
    ok = payload[offset] != 0
    offset += 1

    def read_u32() -> int:
        nonlocal offset
        if offset + 4 > len(payload):
            raise ValueError("truncated_u32")
        value = struct.unpack("<I", payload[offset : offset + 4])[0]
        offset += 4
        return value

    def read_string() -> str:
        nonlocal offset
        size = read_u32()
        end = offset + size
        if end > len(payload):
            raise ValueError("truncated_string")
        value = payload[offset:end].decode("utf-8", errors="replace")
        offset = end
        return value

    if not ok:
        reason = read_string()
        detail = read_string() if offset + 4 <= len(payload) else ""
        return {"ok": False, "reason": reason, "detail": detail}

    greeting = read_string()
    ip_raw = read_u32()
    ip = socket.inet_ntoa(struct.pack("<I", ip_raw))
    md5hash = read_string()
    supporter = payload[offset] != 0 if offset < len(payload) else False
    return {
        "ok": True,
        "greeting": greeting,
        "ip": ip,
        "md5hash": md5hash,
        "is_supporter": supporter,
    }


def read_n(sock: socket.socket, total: int) -> bytes:
    chunks: list[bytes] = []
    remaining = total
    while remaining > 0:
        try:
            chunk = sock.recv(remaining)
        except socket.timeout:
            continue
        if not chunk:
            raise ConnectionError("socket closed")
        chunks.append(chunk)
        remaining -= len(chunk)
    return b"".join(chunks)


def try_read_frame(sock: socket.socket) -> bytes | None:
    prev_timeout = sock.gettimeout()
    hdr = b""
    while len(hdr) < 4:
        try:
            chunk = sock.recv(4 - len(hdr))
        except socket.timeout:
            if not hdr:
                return None
            continue
        if not chunk:
            if not hdr:
                return None
            raise ConnectionError("truncated frame header")
        hdr += chunk

    frame_len = struct.unpack("<I", hdr)[0]
    if prev_timeout is not None and prev_timeout < 2.0:
        sock.settimeout(2.0)
    body = read_n(sock, frame_len)
    if prev_timeout is not None:
        sock.settimeout(prev_timeout)
    return hdr + body


def drain_frames(sock: socket.socket, duration_s: float = 0.8) -> list[bytes]:
    frames: list[bytes] = []
    deadline = time.monotonic() + duration_s
    while time.monotonic() < deadline:
        timeout = max(0.05, deadline - time.monotonic())
        sock.settimeout(timeout)
        got = try_read_frame(sock)
        if got is None:
            continue
        frames.append(got)
    return frames


def send_server_sequence(server: str, outbound_frames: list[bytes]) -> list[bytes]:
    host, port_raw = server.rsplit(":", 1)
    port = int(port_raw)

    wire: list[bytes] = []
    with socket.create_connection((host, port), timeout=6) as sock:
        sock.settimeout(0.6)
        for frm in outbound_frames:
            sock.sendall(frm)
            wire.append(frm)
            wire.extend(drain_frames(sock, duration_s=0.35))
        wire.extend(drain_frames(sock, duration_s=1.2))

    return wire
