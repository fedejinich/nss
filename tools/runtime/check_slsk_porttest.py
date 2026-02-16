#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import socket
import sys
import time
import urllib.error
import urllib.request
from contextlib import closing
from dataclasses import asdict, dataclass

PORTTEST_URL = "http://www.slsknet.org/porttest.php?port={port}"
STATUS_RE = re.compile(r"Port:\s*(?P<port>\d+)/tcp\s+(?P<status>OPEN|CLOSED)", re.IGNORECASE)
IP_RE = re.compile(r"IP:\s*(?P<ip>[0-9]{1,3}(?:\.[0-9]{1,3}){3})", re.IGNORECASE)


@dataclass
class PortTestResult:
    port: int
    status: str
    observed_ip: str | None
    source: str


def fetch_porttest(port: int, timeout: float) -> PortTestResult:
    url = PORTTEST_URL.format(port=port)
    try:
        with urllib.request.urlopen(url, timeout=timeout) as resp:
            body = resp.read().decode("utf-8", errors="replace")
    except urllib.error.URLError as exc:
        return PortTestResult(
            port=port,
            status=f"ERROR: {exc}",
            observed_ip=None,
            source=url,
        )

    m = STATUS_RE.search(body)
    ip_match = IP_RE.search(body)
    status = m.group("status").upper() if m else "UNKNOWN"
    observed_ip = ip_match.group("ip") if ip_match else None
    return PortTestResult(
        port=port,
        status=status,
        observed_ip=observed_ip,
        source=url,
    )


def listen_on_port(port: int) -> tuple[socket.socket | None, str | None]:
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    try:
        sock.bind(("0.0.0.0", port))
        sock.listen(8)
        return sock, None
    except OSError as exc:
        sock.close()
        return None, str(exc)


def parse_ports(raw_ports: list[str]) -> list[int]:
    ports: list[int] = []
    for raw in raw_ports:
        for item in raw.split(","):
            token = item.strip()
            if not token:
                continue
            value = int(token)
            if value < 1 or value > 65535:
                raise ValueError(f"invalid port: {value}")
            ports.append(value)
    if not ports:
        raise ValueError("at least one port is required")
    return ports


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run Soulseek remote port tests (optionally binding local listeners first)."
    )
    parser.add_argument("ports", nargs="+", help="Port values (space or comma separated)")
    parser.add_argument(
        "--no-listen",
        action="store_true",
        help="Do not bind a temporary local listener before each remote test",
    )
    parser.add_argument(
        "--listen-settle-secs",
        type=float,
        default=1.0,
        help="Seconds to wait after binding listener (default 1.0)",
    )
    parser.add_argument(
        "--timeout-secs",
        type=float,
        default=6.0,
        help="HTTP timeout in seconds (default 6.0)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emit machine-readable JSON only",
    )
    args = parser.parse_args()

    try:
        ports = parse_ports(args.ports)
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2

    report: list[dict[str, object]] = []
    exit_code = 0

    for port in ports:
        listener = None
        listen_error = None
        if not args.no_listen:
            listener, listen_error = listen_on_port(port)
            if listener is None:
                exit_code = 1

        if listener is not None and args.listen_settle_secs > 0:
            time.sleep(args.listen_settle_secs)

        result = fetch_porttest(port, timeout=args.timeout_secs)

        if listener is not None:
            listener.close()

        row = asdict(result)
        if listen_error is not None:
            row["local_listen_error"] = listen_error
        report.append(row)

        if result.status != "OPEN":
            exit_code = 1

    if args.json:
        print(json.dumps(report, ensure_ascii=True, indent=2))
    else:
        for row in report:
            msg = (
                f"port={row['port']} status={row['status']}"
                f" observed_ip={row.get('observed_ip') or '-'}"
            )
            if "local_listen_error" in row:
                msg += f" local_listen_error={row['local_listen_error']}"
            print(msg)

    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
