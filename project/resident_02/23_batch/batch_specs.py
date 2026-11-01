"""#!/usr/bin/env python3"""
import argparse
import json
import re
import time
from datetime import datetime
import sys
from pathlib import Path
import yaml
from jinja2 import Environment, FileSystemLoader, StrictUndefined
from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
from xy.ai.mcpc.cli.session import TokenStats
from xy.ai.mcpc.tools.tool_registry import ToolResult
BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch')
SYSTEM_PROMPT = ''
READ_SPEC_SCHEMA = {
    'type': 'object',
    'properties': {
        'modules': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'minItems': 1,
            'description': 'List of module names'}},
    'required': ['modules']}

def slugify(name: str) -> str:
    slug = re.sub('[^\\w]+', '_', name.strip(), flags=re.UNICODE).strip('_')
    return slug.lower() or 'module'

def spec_path(spec_dir: Path, name: str) -> Path:
    return spec_dir / f'{slugify(name)}.md'

def format_stats(stats: TokenStats, elapsed: float | None=None) -> str:
    parts = [
        f'total={
            stats.total_tokens}', f'in={
                stats.input_tokens}', f'out={
                    stats.output_tokens}', f'cacheRead={
                        stats.cache_read}', f'cacheCreate={
                            stats.cache_create}', f'reasoning={
                                stats.reasoning_tokens}', f'turns={
                                    stats.turns}', f'cost=${
                                        stats.cost_usd:.4f}', f'cli={
                                            stats.duration_ms / 1000:.1f}s']
    if elapsed is not None:
        parts.append(f'wall={elapsed:.1f}s')
    return ' | '.join(parts)

class Reporter:
    """Writes each line to STDOUT (STDERR for errors) and to the log file."""

    def __init__(self, log_file) -> None:
        self.log_file = log_file

    def __call__(self, message: str, *, error: bool=False) -> None:
        print(message, file=sys.stderr if error else sys.stdout, flush=True)
        self.log_file.write(f'{datetime.now():%Y-%m-%d %H:%M:%S} {message}\n')
        self.log_file.flush()

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--modules', type=Path, default=BASE_DIR / 'modules.yaml')
    parser.add_argument('--template', type=Path, default=BASE_DIR / 'prompt.md.j2')
    parser.add_argument('--spec-dir', type=Path, default=BASE_DIR / 'specs')
    parser.add_argument(
        '--log-file',
        type=Path,
        default=None,
        help='log file (default: <base>/logs/batch_<timestamp>.log)')
    parser.add_argument('--force', action='store_true', help='regenerate existing specifications')
    parser.add_argument('--profile', default='personal', help='Claude config profile (~/.claude-<profile>)')
    args = parser.parse_args()
    log_path = args.log_file or BASE_DIR / 'logs' / f'batch_{datetime.now():%Y%m%d_%H%M%S}.log'
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with log_path.open('a', encoding='utf-8') as log_file:
        return run(args, Reporter(log_file))

def run(args, report: Reporter) -> int:
    modules = yaml.safe_load(args.modules.read_text(encoding='utf-8'))
    modules.sort(key=lambda m: m.get('Order', 0))
    args.spec_dir.mkdir(parents=True, exist_ok=True)
    env = Environment(loader=FileSystemLoader(str(args.template.parent)),
                      undefined=StrictUndefined, keep_trailing_newline=True)
    template = env.get_template(args.template.name)
    total = TokenStats()
    started = time.monotonic()

    def finish(code: int) -> int:
        report(f'[stat] TOTAL | {format_stats(total, time.monotonic() - started)}')
        return code
    with EmbeddedMcpc(builtin_tools=[], auto_approve=True) as mcpc:

        @mcpc.tool('read_specifications', 'Read the finished specification of a module by name.', READ_SPEC_SCHEMA)
        def read_specifications(ctx):
            names = ctx.arguments.get('modules', [])
            if isinstance(names, str):
                names = [names]
            results = []
            for name in map(str, names):
                path = spec_path(args.spec_dir, name)
                if path.is_file():
                    results.append({'module': name, 'specification': path.read_text(encoding='utf-8')})
                else:
                    results.append({'module': name, 'error': f"Specification for module '{name}' is still pending."})
            all_missing = not results or all(('error' in r for r in results))
            text = json.dumps(results, ensure_ascii=False, indent=2)
            if all_missing:
                return ToolResult(content=[{'type': 'text', 'text': text}], is_error=True)
            return text
        done = []
        for module in modules:
            name = module['Name']
            target = spec_path(args.spec_dir, name)
            if target.exists() and (not args.force):
                report(f'[skip] {name}')
                done.append(name)
                continue
            module.setdefault('Dependencies', [])
            prompt = template.render(module=module, previous_modules=list(done))
            report(f'[run ] {name}')
            module_started = time.monotonic()
            with mcpc.start_agent(SYSTEM_PROMPT, tools={'read_specifications'}, model=Model.HAIKU, effort=Effort.LOW, cc_profile=args.profile) as agent:
                result = agent.prompt(prompt)
            total.add(result.stats)
            report(f'[stat] {name} | {format_stats(result.stats, time.monotonic() - module_started)}')
            if result.is_error or not result.text.strip():
                report(f'[fail] {name}: {result.text}', error=True)
                return finish(1)
            target.write_text(result.text, encoding='utf-8')
            done.append(name)
            report(f'[ok  ] {target}')
        return finish(0)
if __name__ == '__main__':
    sys.exit(main())