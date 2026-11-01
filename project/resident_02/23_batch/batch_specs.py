"""#!/usr/bin/env python3"""
import argparse
import re
import sys
from pathlib import Path
import yaml
from jinja2 import Environment, FileSystemLoader, StrictUndefined
from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
from xy.ai.mcpc.tools.tool_registry import ToolResult
BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch')
SYSTEM_PROMPT = 'You are a software architect writing precise module specifications. Use the tool `read_specifications` to read specifications of other modules.'
READ_SPEC_SCHEMA = {
    'type': 'object',
    'properties': {
        'module': {
            'type': 'string',
            'description': 'Module name'}},
    'required': ['module']}

def slugify(name: str) -> str:
    slug = re.sub('[^\\w]+', '_', name.strip(), flags=re.UNICODE).strip('_')
    return slug.lower() or 'module'

def spec_path(spec_dir: Path, name: str) -> Path:
    return spec_dir / f'{slugify(name)}.md'

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--modules', type=Path, default=BASE_DIR / 'modules.yaml')
    parser.add_argument('--template', type=Path, default=BASE_DIR / 'prompt.md.j2')
    parser.add_argument('--spec-dir', type=Path, default=BASE_DIR / 'specs')
    parser.add_argument('--force', action='store_true', help='regenerate existing specifications')
    parser.add_argument('--profile', default='personal', help='Claude config profile (~/.claude-<profile>)')
    args = parser.parse_args()
    modules = yaml.safe_load(args.modules.read_text(encoding='utf-8'))
    modules.sort(key=lambda m: m.get('Order', 0))
    args.spec_dir.mkdir(parents=True, exist_ok=True)
    env = Environment(loader=FileSystemLoader(str(args.template.parent)),
                      undefined=StrictUndefined, keep_trailing_newline=True)
    template = env.get_template(args.template.name)
    with EmbeddedMcpc(builtin_tools=[], auto_approve=True) as mcpc:

        @mcpc.tool('read_specifications', 'Read the finished specification of a module by name.', READ_SPEC_SCHEMA)
        def read_specifications(ctx):
            name = str(ctx.arguments.get('module', ''))
            path = spec_path(args.spec_dir, name)
            if not path.is_file():
                return ToolResult(
                    content=[{'type': 'text', 'text': f"Specification for module '{name}' is still pending."}], is_error=True)
            return path.read_text(encoding='utf-8')
        done = []
        for module in modules:
            name = module['Name']
            target = spec_path(args.spec_dir, name)
            if target.exists() and (not args.force):
                print(f'[skip] {name}')
                done.append(name)
                continue
            module.setdefault('Dependencies', [])
            prompt = template.render(module=module, previous_modules=list(done))
            print(f'[run ] {name}')
            with mcpc.start_agent(SYSTEM_PROMPT, tools={'read_specifications'}, model=Model.HAIKU, effort=Effort.LOW, cc_profile=args.profile) as agent:
                result = agent.prompt(prompt)
            if result.is_error or not result.text.strip():
                print(f'[fail] {name}: {result.text}', file=sys.stderr)
                return 1
            target.write_text(result.text, encoding='utf-8')
            done.append(name)
            print(f'[ok  ] {target}')
        return 0
if __name__ == '__main__':
    sys.exit(main())