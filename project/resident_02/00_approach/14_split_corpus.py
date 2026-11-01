#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


def slugify(text: str) -> str:
    text = re.sub(r"[^\w\s-]", "", text, flags=re.UNICODE).strip().lower()
    return re.sub(r"[\s_-]+", "-", text) or "abschnitt"


def split_markdown(content: str):
    """Teilt an '## '-Überschriften (außerhalb von Codeblöcken)."""
    sections = []  # (titel, zeilen)
    current_title = None
    current = []
    in_fence = False
    fence = ""

    for line in content.splitlines(keepends=True):
        m = re.match(r"^\s{0,3}(`{3,}|~{3,})", line)
        if m:
            marker = m.group(1)
            if not in_fence:
                in_fence, fence = True, marker[0]
            elif marker[0] == fence:
                in_fence = False

        if not in_fence and re.match(r"^## ", line):
            sections.append((current_title, current))
            current_title = line[3:].strip()
            current = [line]
        else:
            current.append(line)

    sections.append((current_title, current))
    return sections


def main():
    p = argparse.ArgumentParser(description="Markdown an '## '-Überschriften splitten")
    p.add_argument("input", type=Path, help="Eingabe-Markdowndatei")
    p.add_argument("-o", "--output", type=Path, default=Path("out"), help="Ausgabeverzeichnis")
    args = p.parse_args()

    content = args.input.read_text(encoding="utf-8")
    args.output.mkdir(parents=True, exist_ok=True)

    used = set()
    idx = 0
    for title, lines in split_markdown(content):
        body = "".join(lines)
        if title is None:
            if not body.strip():
                continue
            name = "00-einleitung"
        else:
            idx += 1
            name = f"{idx:02d}-{slugify(title)}"
        while name in used:
            name += "_"
        used.add(name)
        (args.output / f"{name}.md").write_text(body, encoding="utf-8")


if __name__ == "__main__":
    main()