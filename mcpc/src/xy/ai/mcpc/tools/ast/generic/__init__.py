"""Generic tree-sitter back-ends for every non-Python language/format.

Split into :mod:`xy.ai.mcpc.tools.ast.generic._engine` (the universal
:class:`TreeSitterEngine`, exposing a grammar's native structure as-is) and
per-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`
(:class:`MarkdownEngine`); this module re-exports the package's public
surface and dispatches a language symbol to its engine.
"""
from __future__ import annotations
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine
from xy.ai.mcpc.tools.ast.generic._java import JavaEngine
from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
__all__ = ['TreeSitterEngine', 'JavaEngine', 'MarkdownEngine', 'language_for_extension', 'get_engine']
'#: File extension -> ``tree_sitter_language_pack`` language identifier.'
EXT_LANGUAGE = {
    '.json': 'json',
    '.jsonl': 'json',
    '.yaml': 'yaml',
    '.yml': 'yaml',
    '.toml': 'toml',
    '.xml': 'xml',
    '.html': 'html',
    '.htm': 'html',
    '.css': 'css',
    '.scss': 'css',
    '.md': 'markdown',
    '.markdown': 'markdown',
    '.js': 'javascript',
    '.jsx': 'javascript',
    '.mjs': 'javascript',
    '.cjs': 'javascript',
    '.ts': 'typescript',
    '.tsx': 'tsx',
    '.go': 'go',
    '.rs': 'rust',
    '.java': 'java',
    '.c': 'c',
    '.h': 'c',
    '.cpp': 'cpp',
    '.cc': 'cpp',
    '.cxx': 'cpp',
    '.hpp': 'cpp',
    '.hh': 'cpp',
    '.cs': 'csharp',
    '.rb': 'ruby',
    '.php': 'php',
    '.sh': 'bash',
    '.bash': 'bash',
    '.sql': 'sql',
    '.lua': 'lua',
    '.scala': 'scala',
    '.kt': 'kotlin',
    '.hs': 'haskell',
    '.ex': 'elixir',
    '.exs': 'elixir',
    '.dockerfile': 'dockerfile'}

def language_for_extension(ext: str) -> str | None:
    return EXT_LANGUAGE.get(ext.lower())
'#: Language symbol -> dedicated Engine subclass; anything absent here falls'
'#: back to the universal :class:`TreeSitterEngine`.'
_ENGINE_CLASSES: dict[str, type[TreeSitterEngine]] = {'markdown': MarkdownEngine, 'java': JavaEngine}
_ENGINES: dict[str, TreeSitterEngine] = {}

def get_engine(symbol: str) -> TreeSitterEngine:
    engine = _ENGINES.get(symbol)
    if engine is None:
        engine_cls = _ENGINE_CLASSES.get(symbol)
        engine = engine_cls() if engine_cls else TreeSitterEngine(symbol)
        _ENGINES[symbol] = engine
    return engine