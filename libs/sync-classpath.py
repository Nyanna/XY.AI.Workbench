#!/usr/bin/env python3
"""Sync JAR paths in .classpath with the actual files under libs/.

Matches each classpathentry to a current file by artifact name (filename
minus version), using the union of all allowed-artifacts.txt files as the
dictionary of known names, longest-prefix-wins. Never aborts: 0 or >1
candidates just produce a warning and leave that entry untouched. Also warns
about libs/ jars not referenced by any entry.

Usage: python3 libs/sync-classpath.py [--dry-run]
Writes a .classpath.bak backup before overwriting.
"""
import argparse
import re
import sys
from pathlib import Path

LIBS_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = LIBS_DIR.parent
CLASSPATH_FILE = PROJECT_ROOT / ".classpath"

ENTRY_RE = re.compile(
    r'(<classpathentry\b[^>]*\bkind="lib"[^>]*\bpath=")(libs/[^"]+)(")'
)


def load_known_artifact_ids() -> list[str]:
    """Known artifact names from all allowed-artifacts.txt, longest first."""
    ids: set[str] = set()
    for f in LIBS_DIR.rglob("allowed-artifacts.txt"):
        for line in f.read_text().splitlines():
            line = line.strip()
            if line:
                ids.add(line)
    return sorted(ids, key=len, reverse=True)


def artifact_id_for_filename(filename: str, known_ids: list[str]) -> str | None:
    """Longest-prefix match of '<artifactId>-<version>.jar' against known_ids."""
    if not filename.endswith(".jar"):
        return None
    stem = filename[: -len(".jar")]
    for aid in known_ids:
        if stem == aid or stem.startswith(aid + "-"):
            return aid
    return None


def build_actual_jar_index(known_ids: list[str]) -> dict[str, list[str]]:
    """artifact name -> list of current relative paths (relative to PROJECT_ROOT)."""
    index: dict[str, list[str]] = {}
    for jar in LIBS_DIR.rglob("*.jar"):
        rel = jar.relative_to(PROJECT_ROOT).as_posix()
        aid = artifact_id_for_filename(jar.name, known_ids)
        if aid is None:
            print(f"WARNUNG: JAR ohne bekannten Artefakt-Namen: {rel}", file=sys.stderr)
            continue
        index.setdefault(aid, []).append(rel)
    return index


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true",
                         help="Nur anzeigen, .classpath nicht schreiben")
    args = parser.parse_args()

    if not CLASSPATH_FILE.is_file():
        print(f"Fehler: {CLASSPATH_FILE} nicht gefunden", file=sys.stderr)
        return 1

    known_ids = load_known_artifact_ids()
    if not known_ids:
        print("Fehler: keine allowed-artifacts.txt unter libs/ gefunden", file=sys.stderr)
        return 1

    actual_index = build_actual_jar_index(known_ids)

    original_text = CLASSPATH_FILE.read_text()
    used_paths: set[str] = set()
    replaced = 0
    unmatched_entries: list[str] = []
    ambiguous_entries: list[tuple[str, list[str]]] = []

    def replace(match: re.Match) -> str:
        nonlocal replaced
        prefix, old_path, suffix = match.group(1), match.group(2), match.group(3)
        old_filename = old_path.rsplit("/", 1)[-1]
        aid = artifact_id_for_filename(old_filename, known_ids)

        if aid is None:
            unmatched_entries.append(old_path)
            return match.group(0)

        candidates = actual_index.get(aid, [])
        if len(candidates) == 0:
            unmatched_entries.append(old_path)
            return match.group(0)
        if len(candidates) > 1:
            ambiguous_entries.append((old_path, candidates))
            return match.group(0)

        new_path = candidates[0]
        used_paths.add(new_path)
        if new_path == old_path:
            return match.group(0)
        replaced += 1
        print(f"ERSETZT: {old_path}  ->  {new_path}")
        return f"{prefix}{new_path}{suffix}"

    new_text = ENTRY_RE.sub(replace, original_text)

    print()
    if unmatched_entries:
        print(f"WARNUNG: {len(unmatched_entries)} classpathentry-Pfad(e) ohne "
              f"passendes Artefakt gefunden, unveraendert gelassen:")
        for p in unmatched_entries:
            print(f"  - {p}")

    if ambiguous_entries:
        print(f"WARNUNG: {len(ambiguous_entries)} classpathentry-Pfad(e) mehrdeutig "
              f"(mehrere Kandidaten), unveraendert gelassen:")
        for p, cands in ambiguous_entries:
            print(f"  - {p} -> Kandidaten: {', '.join(cands)}")

    # Re-scan the (possibly updated) text to catch orphans among entries that
    # already pointed at the right file and were never touched above.
    all_referenced_after = set(m.group(2) for m in ENTRY_RE.finditer(new_text))
    all_actual = {p for paths in actual_index.values() for p in paths}
    orphan_jars = sorted(all_actual - all_referenced_after)
    if orphan_jars:
        print(f"WARNUNG: {len(orphan_jars)} JAR(s) unter libs/ werden von keinem "
              f"classpathentry referenziert:")
        for p in orphan_jars:
            print(f"  - {p}")

    print()
    print(f"Zusammenfassung: {replaced} Pfad(e) ersetzt, "
          f"{len(unmatched_entries)} ohne Treffer, "
          f"{len(ambiguous_entries)} mehrdeutig, "
          f"{len(orphan_jars)} verwaiste JAR(s).")

    if args.dry_run:
        print("(--dry-run: .classpath wurde NICHT geschrieben)")
        return 0

    if replaced > 0:
        backup = CLASSPATH_FILE.parent / ".classpath.bak"
        backup.write_text(original_text)
        CLASSPATH_FILE.write_text(new_text)
        print(f"\n.classpath aktualisiert (Backup: {backup})")
    else:
        print("\nKeine Aenderungen noetig, .classpath unveraendert.")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
