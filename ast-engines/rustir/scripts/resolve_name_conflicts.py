#!/usr/bin/env python3
import argparse
import http.client
import json
import queue
import re
import sys
import threading
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent


class McpSseClient:
    def __init__(self, host, port, timeout=300):
        self.host, self.port, self.timeout = host, port, timeout
        self.events = queue.Queue()
        self.next_id = 0
        threading.Thread(target=self._read, daemon=True).start()
        kind, self.endpoint = self.events.get(timeout=30)
        if kind != "endpoint":
            raise RuntimeError(f"unexpected first event: {kind}")
        self.request(
            "initialize",
            {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "resolve_name_conflicts", "version": "1"},
            },
        )
        self._post({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def _read(self):
        conn = http.client.HTTPConnection(self.host, self.port, timeout=self.timeout)
        conn.request("GET", "/sse", headers={"Accept": "text/event-stream"})
        resp = conn.getresponse()
        event = "message"
        while True:
            line = resp.fp.readline()
            if not line:
                self.events.put(("closed", None))
                return
            line = line.decode().rstrip("\r\n")
            if line.startswith("event:"):
                event = line[6:].strip()
            elif line.startswith("data:"):
                self.events.put((event, line[5:].strip()))
                event = "message"

    def _post(self, payload):
        conn = http.client.HTTPConnection(self.host, self.port, timeout=self.timeout)
        conn.request(
            "POST",
            self.endpoint,
            json.dumps(payload),
            {"Content-Type": "application/json"},
        )
        resp = conn.getresponse()
        resp.read()
        if resp.status >= 300:
            raise RuntimeError(f"POST failed: {resp.status}")

    def request(self, method, params):
        self.next_id += 1
        rid = self.next_id
        self._post({"jsonrpc": "2.0", "id": rid, "method": method, "params": params})
        while True:
            kind, data = self.events.get(timeout=self.timeout)
            if kind == "closed":
                raise RuntimeError("SSE stream closed")
            if kind != "message":
                continue
            msg = json.loads(data)
            if msg.get("id") == rid:
                if "error" in msg:
                    raise RuntimeError(msg["error"])
                return msg["result"]

    def call_tool(self, name, arguments):
        result = self.request("tools/call", {"name": name, "arguments": arguments})
        text = "\n".join(c.get("text", "") for c in result.get("content", []))
        return bool(result.get("isError")), text


def word_re(name):
    return re.compile(rf"(?<![A-Za-z0-9_]){re.escape(name)}(?![A-Za-z0-9_])")


def collect_identifiers(project):
    idents = set()
    token = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
    for path in project.rglob("*.rs"):
        if "target" in path.relative_to(project).parts:
            continue
        idents.update(token.findall(path.read_text(errors="replace")))
    return idents


def free_name(base, start, used):
    n = start
    while f"{base}{n}" in used:
        n += 1
    return f"{base}{n}"


def plan(conflicts, project, number_all):
    used = collect_identifiers(project)
    steps = []
    for entry in conflicts:
        name = entry["name"]
        files = entry["files"]
        targets = files if number_all else files[1:]
        for path in targets:
            new = free_name(name, 2, used)
            used.add(new)
            steps.append((name, new, path))
    return steps


def main():
    ap = argparse.ArgumentParser(description="Resolve name conflicts via IDE rename refactoring.")
    ap.add_argument("--yaml", type=Path, default=ROOT / "name-conflicts.yaml")
    ap.add_argument("--project", type=Path, default=Path("/home/user/xyan/xy.ai.workbench/rag"))
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=64522)
    ap.add_argument("--number-all", action="store_true", help="also number the first occurrence")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    project = args.project.resolve()
    conflicts = (yaml.safe_load(args.yaml.read_text()) or {}).get("conflicts", [])
    steps = plan(conflicts, project, args.number_all)

    client = None if args.dry_run else McpSseClient(args.host, args.port)
    failures = 0
    for old, new, path in steps:
        file = Path(path)
        rel = file.resolve().relative_to(project)
        if not file.is_file() or not word_re(old).search(file.read_text(errors="replace")):
            print(f"SKIP  {old} in {rel} (not present)")
            continue
        if client is None:
            print(f"PLAN  {old} -> {new} in {rel}")
            continue
        is_error, text = client.call_tool(
            "rename_refactoring",
            {
                "pathInProject": str(rel),
                "symbolName": old,
                "newName": new,
                "projectPath": str(project),
            },
        )
        status = "FAIL" if is_error else "OK"
        failures += is_error
        print(f"{status:5} {old} -> {new} in {rel} {text.strip()[:200]}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
