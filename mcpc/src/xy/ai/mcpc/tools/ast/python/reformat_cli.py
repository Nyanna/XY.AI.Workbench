"""CLI: recursively reformat a Python file tree via :class:`PythonEngine`.

Round-trips every ``*.py`` file through ``parse``/``serialize`` (comment-
preserving parse, formatting-aware unparse), rewriting it in place if the
result differs. Intended for one-off tree-wide conversions, e.g. from a hook.

Usage:
    python -m xy.ai.mcpc.tools.ast.python.reformat_cli <root> [--dry-run]
"""
from __future__ import annotations
import argparse
import sys
from pathlib import Path
from xy.ai.mcpc.tools.ast.base import AstError
from xy.ai.mcpc.tools.ast.python._engine import PythonEngine

def iter_python_files(root: Path):
    if root.is_file():
        yield root
        return
    yield from sorted(root.rglob('*.py'))

def reformat_file(engine: PythonEngine, path: Path, *, dry_run: bool) -> bool:
    """Reformat ``path`` in place; return whether its content changed."""
    source = path.read_text(encoding='utf-8')
    tree = engine.parse(source, path)
    formatted = engine.serialize(tree)
    if formatted == source:
        return False
    if not dry_run:
        path.write_text(formatted, encoding='utf-8')
    return True

def main(argv: list[str] | None=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', type=Path, help='File or directory to reformat')
    parser.add_argument('--dry-run', action='store_true', help='Report changes without writing')
    args = parser.parse_args(argv)
    engine = PythonEngine()
    changed = 0
    failed = 0
    for path in iter_python_files(args.root):
        try:
            if reformat_file(engine, path, dry_run=args.dry_run):
                changed += 1
                print(f'{('would reformat' if args.dry_run else 'reformatted')}: {path}')
        except AstError as exc:
            failed += 1
            print(f'error: {path}: {exc}', file=sys.stderr)
    print(f'{changed} file(s) changed, {failed} failed')
    return 1 if failed else 0
if __name__ == '__main__':
    raise SystemExit(main())