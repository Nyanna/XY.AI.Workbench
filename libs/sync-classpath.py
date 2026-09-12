"""#!/usr/bin/env python3"""
'Sync JAR paths in .classpath with the actual files under libs/.\n\nMatches each classpathentry to a current file by artifact name (filename\nminus version), using the union of all allowed-artifacts.txt files as the\ndictionary of known names, longest-prefix-wins. Never aborts: 0 or >1\ncandidates just produce a warning and leave that entry untouched. Also warns\nabout libs/ jars not referenced by any entry.\n\nUsage: python3 libs/sync-classpath.py [--dry-run]\nWrites a .classpath.bak backup before overwriting.\n'
import argparse
import re
import sys
from pathlib import Path
LIBS_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = LIBS_DIR.parent
CLASSPATH_FILE = PROJECT_ROOT / '.classpath'
MANIFEST_FILE = PROJECT_ROOT / 'META-INF' / 'MANIFEST.MF'
ENTRY_RE = re.compile('(<classpathentry\\b[^>]*\\bkind="lib"[^>]*\\bpath=")(libs/[^"]+)(")')
MANIFEST_JAR_RE = re.compile('(libs/[^\\s,"]+\\.jar)')

def load_known_artifact_ids() -> list[str]:
    """Known artifact names from all allowed-artifacts.txt, longest first."""
    ids: set[str] = set()
    for f in LIBS_DIR.rglob('allowed-artifacts.txt'):
        for line in f.read_text().splitlines():
            line = line.strip()
            if line:
                ids.add(line)
    return sorted(ids, key=len, reverse=True)

def artifact_id_for_filename(filename: str, known_ids: list[str]) -> str | None:
    """Longest-prefix match of '<artifactId>-<version>.jar' against known_ids."""
    if not filename.endswith('.jar'):
        return None
    stem = filename[:-len('.jar')]
    for aid in known_ids:
        if stem == aid or stem.startswith(aid + '-'):
            return aid
    return None

def build_actual_jar_index(known_ids: list[str]) -> dict[str, list[str]]:
    """artifact name -> list of current relative paths (relative to PROJECT_ROOT)."""
    index: dict[str, list[str]] = {}
    for jar in LIBS_DIR.rglob('*.jar'):
        rel = jar.relative_to(PROJECT_ROOT).as_posix()
        aid = artifact_id_for_filename(jar.name, known_ids)
        if aid is None:
            print(f'WARNING: JAR with unknown artifact name: {rel}', file=sys.stderr)
            continue
        index.setdefault(aid, []).append(rel)
    return index

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--dry-run', action='store_true', help='Only show, do not write files')
    args = parser.parse_args()
    if not CLASSPATH_FILE.is_file():
        print(f'Error: {CLASSPATH_FILE} not found', file=sys.stderr)
        return 1
    if not MANIFEST_FILE.is_file():
        print(f'Error: {MANIFEST_FILE} not found', file=sys.stderr)
        return 1
    known_ids = load_known_artifact_ids()
    if not known_ids:
        print('Error: no allowed-artifacts.txt found under libs/', file=sys.stderr)
        return 1
    actual_index = build_actual_jar_index(known_ids)
    used_paths: set[str] = set()
    total_replaced = 0
    unmatched: list[str] = []
    ambiguous: list[tuple[str, list[str]]] = []

    def resolve(old_path: str) -> str | None:
        """Return the new path for old_path, or None if unmatched/ambiguous (already reported)."""
        nonlocal total_replaced
        old_filename = old_path.rsplit('/', 1)[-1]
        aid = artifact_id_for_filename(old_filename, known_ids)
        if aid is None:
            unmatched.append(old_path)
            return None
        candidates = actual_index.get(aid, [])
        if len(candidates) == 0:
            unmatched.append(old_path)
            return None
        if len(candidates) > 1:
            ambiguous.append((old_path, candidates))
            return None
        new_path = candidates[0]
        used_paths.add(new_path)
        return new_path

    def classpath_replace(match: re.Match) -> str:
        nonlocal total_replaced
        prefix, old_path, suffix = (match.group(1), match.group(2), match.group(3))
        new_path = resolve(old_path)
        if new_path is None or new_path == old_path:
            return match.group(0)
        total_replaced += 1
        print(f'REPLACED [.classpath]: {old_path}  ->  {new_path}')
        return f'{prefix}{new_path}{suffix}'

    def manifest_replace(match: re.Match) -> str:
        nonlocal total_replaced
        old_path = match.group(1)
        new_path = resolve(old_path)
        if new_path is None or new_path == old_path:
            return match.group(0)
        total_replaced += 1
        print(f'REPLACED [MANIFEST.MF]: {old_path}  ->  {new_path}')
        return new_path
    original_classpath_text = CLASSPATH_FILE.read_text()
    new_classpath_text = ENTRY_RE.sub(classpath_replace, original_classpath_text)
    original_manifest_text = MANIFEST_FILE.read_text()
    new_manifest_text = MANIFEST_JAR_RE.sub(manifest_replace, original_manifest_text)
    print()
    if unmatched:
        print(f'WARNING: {len(unmatched)} path(s) with no matching artifact found, left unchanged:')
        for p in unmatched:
            print(f'  - {p}')
    if ambiguous:
        print(f'WARNING: {len(ambiguous)} path(s) ambiguous (multiple candidates), left unchanged:')
        for p, cands in ambiguous:
            print(f'  - {p} -> candidates: {', '.join(cands)}')
    all_referenced_after = set((m.group(2) for m in ENTRY_RE.finditer(new_classpath_text)))
    all_referenced_after |= set(MANIFEST_JAR_RE.findall(new_manifest_text))
    all_actual = {p for paths in actual_index.values() for p in paths}
    orphan_jars = sorted(all_actual - all_referenced_after)
    if orphan_jars:
        print(f'WARNING: {len(orphan_jars)} JAR(s) under libs/ are not referenced by any classpathentry/Bundle-ClassPath:')
        for p in orphan_jars:
            print(f'  - {p}')
    print()
    print(
        f'Summary: {total_replaced} path(s) replaced, {
            len(unmatched)} unmatched, {
                len(ambiguous)} ambiguous, {
                    len(orphan_jars)} orphaned JAR(s).')
    if args.dry_run:
        print('(--dry-run: .classpath/MANIFEST.MF were NOT written)')
        return 0
    if total_replaced > 0:
        classpath_backup = CLASSPATH_FILE.parent / '.classpath.bak'
        classpath_backup.write_text(original_classpath_text)
        CLASSPATH_FILE.write_text(new_classpath_text)
        manifest_backup = MANIFEST_FILE.parent / 'MANIFEST.MF.bak'
        manifest_backup.write_text(original_manifest_text)
        MANIFEST_FILE.write_text(new_manifest_text)
        print(f'\n.classpath updated (backup: {classpath_backup})')
        print(f'MANIFEST.MF updated (backup: {manifest_backup})')
    else:
        print('\nNo changes needed.')
    return 0
if __name__ == '__main__':
    raise SystemExit(main())