"""Language-aware Jinja2 environment factory: templates/<language> is the template root."""
from functools import lru_cache
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, StrictUndefined
from xy.cgen.lang import get_language
TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'

def file_extension(language: str) -> str:
    return get_language(language).extension

@lru_cache(maxsize=None)
def get_env(language: str) -> Environment:
    """One cached Jinja2 Environment per language, rooted at templates/<language>."""
    env = Environment(loader=FileSystemLoader(str(TEMPLATES_ROOT / language)), trim_blocks=True,
                      lstrip_blocks=True, keep_trailing_newline=True, undefined=StrictUndefined)
    env.filters.update(get_language(language).jinja_filters)
    return env