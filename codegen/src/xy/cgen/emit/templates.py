"""Language-aware Jinja2 environment factory: templates/<language> is the template root."""
from functools import lru_cache
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, StrictUndefined
TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'
EXTENSION = {'java': 'java', 'php': 'php'}

def file_extension(language: str) -> str:
    return EXTENSION[language]

def _to_php_fqn(fqn: str) -> str:
    """Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\\pkg\\sub\\Class')."""
    return '\\' + fqn.replace('.', '\\')

def _to_namespace(package: str) -> str:
    return package.replace('.', '\\')

@lru_cache(maxsize=None)
def get_env(language: str) -> Environment:
    """One cached Jinja2 Environment per language, rooted at templates/<language>."""
    env = Environment(loader=FileSystemLoader(str(TEMPLATES_ROOT / language)), trim_blocks=True,
                      lstrip_blocks=True, keep_trailing_newline=True, undefined=StrictUndefined)
    if language == 'php':
        env.filters['phpfqn'] = _to_php_fqn
        env.filters['phpns'] = _to_namespace
    return env