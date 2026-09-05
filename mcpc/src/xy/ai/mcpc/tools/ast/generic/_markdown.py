"""Markdown tree-sitter engine: a fixed, MDAST-inspired node set.

Tree-sitter's own ``section`` nesting only follows heading level and has no
notion of ``---`` page breaks (and is inconsistent for setext headings), so
the structure is rebuilt from scratch - Page > Heading H1-H6 > Paragraph/
ScriptBlock, mirroring the custom MDAST model in
``xy.ai.workbench.editor.mdast.nodes.Elements`` - from a flattened,
document-order block list (see :func:`_md_root_children`).
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import Located, Tree
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder, _SynthNode
__all__ = ['MarkdownEngine']
_MD_HEADING_TYPES = ('atx_heading', 'setext_heading')
'#: The only node types Markdown ever exposes as separately addressable'
"#: (besides 'page' itself, always addressable via ``depth == 0``)."
_MD_ADDRESSABLE_TYPES = {'page', 'section', 'paragraph', 'fenced_code_block'}

def _md_level(node: Any) -> int:
    """Heading level (1-6) from an ``atx_h<N>_marker`` or ``setext_h<N>_underline`` child."""
    for child in node.named_children:
        if 'marker' in child.type or 'underline' in child.type:
            digits = ''.join((ch for ch in child.type if ch.isdigit()))
            if digits:
                return int(digits)
    return 1

def _md_flatten(node: Any, out: list[Any]) -> None:
    """Collect nodes in document order, transparently unwrapping tree-sitter's own 'section'."""
    for child in node.named_children:
        if child.type == 'section':
            _md_flatten(child, out)
        else:
            out.append(child)

def _md_split_pages(flat: list[Any]) -> list[list[Any]]:
    """Split a flat block list on 'thematic_break' ('---'), dropping empty pages."""
    pages: list[list[Any]] = [[]]
    for node in flat:
        if node.type == 'thematic_break':
            pages.append([])
        else:
            pages[-1].append(node)
    return [page for page in pages if page]

class _MdSection:
    """Builder for one rebuilt heading section, before it is frozen into a ``_SynthNode``."""
    __slots__ = ('level', 'items')

    def __init__(self, level: int, heading: Any) -> None:
        self.level = level
        self.items: list[Any] = [heading]

def _md_nest_headings(items: list[Any]) -> list[Any]:
    """Rebuild H1-H6 nesting from a flat block list, independent of tree-sitter's own grouping."""
    roots: list[Any] = []
    stack: list[_MdSection] = []
    for node in items:
        if node.type in _MD_HEADING_TYPES:
            level = _md_level(node)
            while stack and stack[-1].level >= level:
                stack.pop()
            section = _MdSection(level, node)
            (stack[-1].items if stack else roots).append(section)
            stack.append(section)
        else:
            (stack[-1].items if stack else roots).append(node)
    return roots

def _md_finalize(nodes: list[Any], source: bytes) -> list[Any]:
    """Freeze ``_MdSection`` builders (and their descendants) into ``_SynthNode('section', ...)``."""
    return [
        _SynthNode(
            'section',
            _md_finalize(
                node.items,
                source),
            source) if isinstance(
            node,
            _MdSection) else node for node in nodes]

def _md_root_children(root_node: Any, source: bytes) -> list[Any]:
    """Top-level children of a Markdown file: Pages if the file uses '---', else Sections/Paragraphs directly."""
    flat: list[Any] = []
    _md_flatten(root_node, flat)
    if not any((node.type == 'thematic_break' for node in flat)):
        pages = _md_split_pages(flat)
        return _md_finalize(_md_nest_headings(pages[0]), source) if pages else []
    return [_SynthNode('page', _md_finalize(_md_nest_headings(page), source), source) for page in _md_split_pages(flat)]

class MarkdownEngine(TreeSitterEngine):
    """Tree-sitter Markdown restructured into the fixed node set above."""

    def __init__(self) -> None:
        super().__init__('markdown')

    def locate_all(self, tree: Tree) -> list[Located]:
        root = _RootHolder(_md_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
        return self._locate_from(tree, root, self._addressable)

    @staticmethod
    def _addressable(child: Any, depth: int) -> bool:
        return depth == 0 or child.type in _MD_ADDRESSABLE_TYPES