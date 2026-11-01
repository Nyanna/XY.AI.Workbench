"""#!/usr/bin/env python3"""
import re
import sys
from pathlib import Path
PLAN_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident/plan')
RUNS_DIR = PLAN_DIR / 'runs'
MARKER = re.compile('SystemInit:\\s+id=')

def main() -> int:
    RUNS_DIR.mkdir(exist_ok=True)
    for src in sorted((p for p in PLAN_DIR.iterdir() if p.is_file())):
        lines = src.read_text(encoding='utf-8').splitlines(keepends=True)
        idx = next((i for i, l in enumerate(lines) if MARKER.search(l)), None)
        if idx is None:
            continue
        dst = RUNS_DIR / src.name
        if dst.exists():
            print(f'skip (exists): {dst}', file=sys.stderr)
            continue
        dst.write_text(''.join(lines[idx:]), encoding='utf-8')
        src.write_text(''.join(lines[:idx]), encoding='utf-8')
        print(f'moved {len(lines) - idx} lines: {src.name}')
    return 0
if __name__ == '__main__':
    sys.exit(main())