#!/usr/bin/env python3
"""
Lagert `#[cfg(test)] mod tests { ... }`-Blöcke aus Rust-Dateien aus.

Für jede Datei <dir>/X.rs entsteht <dir>/test/X.rs, im Original steht danach:

    #[cfg(test)]
    #[path = "test/X.rs"]
    mod tests;

Das Modul bleibt Child von X, `use super::*;` und Zugriff auf Privates
funktionieren also unverändert.

Aufruf:
    python extract_tests.py [--root src] [--dirname test] [--dry-run] [--force]
"""
import argparse
import re
import sys
import textwrap
from pathlib import Path

PATTERN = re.compile(
    r"^(?P<indent>[ \t]*)#\[cfg\(test\)\]\s*"
    r"(?P<attrs>(?:#\[[^\]]*\]\s*)*)"
    r"mod\s+tests\s*\{",
    re.MULTILINE,
)
RAW_STR = re.compile(r'b?r(#*)"')
CHAR_LIT = re.compile(r"'(?:\\.[^']*|[^\\'])'")


def find_closing_brace(src: str, i: int) -> int:
    """i = Index direkt hinter der öffnenden '{'. Gibt Index der passenden '}' zurück."""
    depth, n = 1, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j == -1 else j
            continue
        if src.startswith("/*", i):
            level, i = 1, i + 2
            while i < n and level:
                if src.startswith("/*", i):
                    level, i = level + 1, i + 2
                elif src.startswith("*/", i):
                    level, i = level - 1, i + 2
                else:
                    i += 1
            continue
        if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = RAW_STR.match(src, i)
            if m:
                end = src.find('"' + m.group(1), m.end())
                i = n if end == -1 else end + 1 + len(m.group(1))
                continue
        if c == '"':
            i += 1
            while i < n and src[i] != '"':
                i += 2 if src[i] == "\\" else 1
            i += 1
            continue
        if c == "'":
            m = CHAR_LIT.match(src, i)
            i = m.end() if m else i + 1  # sonst Lifetime
            continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("keine passende schließende Klammer gefunden")


def convert(path: Path, dirname: str, dry: bool, force: bool) -> bool:
    text = path.read_text(encoding="utf-8")
    m = PATTERN.search(text)
    if not m:
        return False
    if m.group("indent"):
        print(f"SKIP  {path}: `mod tests` ist eingerückt (in inline-Modul?), manuell prüfen")
        return False
    if len(PATTERN.findall(text)) > 1:
        print(f"WARN  {path}: mehrere `mod tests`-Blöcke, nur der erste wird verarbeitet")

    try:
        close = find_closing_brace(text, m.end())
    except ValueError as e:
        print(f"SKIP  {path}: {e}")
        return False

    body = textwrap.dedent(text[m.end():close]).strip("\n")
    target = path.parent / dirname / path.name
    if target.exists() and not force:
        print(f"SKIP  {path}: {target} existiert bereits (--force zum Überschreiben)")
        return False

    attrs = "".join(a.strip() + "\n" for a in re.findall(r"#\[[^\]]*\]", m.group("attrs")))
    replacement = (
        f'#[cfg(test)]\n{attrs}#[path = "{dirname}/{path.name}"]\nmod tests;'
    )
    new_text = text[: m.start()] + replacement + text[close + 1:]

    print(f"{'DRY ' if dry else ''}OK    {path} -> {target}")
    if not dry:
        target.parent.mkdir(exist_ok=True)
        target.write_text(body + "\n", encoding="utf-8")
        path.write_text(new_text, encoding="utf-8")
    return True


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--root", default="src", type=Path)
    ap.add_argument("--dirname", default="test", help="Name des Testordners pro Ebene")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--force", action="store_true", help="vorhandene Zieldateien überschreiben")
    args = ap.parse_args()

    if not args.root.is_dir():
        print(f"{args.root} ist kein Verzeichnis", file=sys.stderr)
        return 1

    count = 0
    for p in sorted(args.root.rglob("*.rs")):
        if args.dirname in p.relative_to(args.root).parts[:-1]:
            continue  # bereits ausgelagerte Testdateien nicht erneut anfassen
        count += convert(p, args.dirname, args.dry_run, args.force)
    print(f"{count} Datei(en) {'würden konvertiert' if args.dry_run else 'konvertiert'}.")
    return 0


if __name__ == "__main__":
    sys.exit(main())