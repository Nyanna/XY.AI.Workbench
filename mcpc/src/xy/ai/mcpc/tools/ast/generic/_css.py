"""CSS tree-sitter engine: Rules/at-rules -> nested rules/keyframe blocks, mirroring
the Java engine's shape (block-bearing statements are individually addressable,
everything else collapses into 'imports'/'declarations' segments capped at
``_CSS_SEGMENT_MAX_CHARS``) - the plain native tree-sitter grammar (every
selector/value/punctuation token addressable) is far too fine-grained to be
useful, and unlike Java/YAML, CSS files are usually flat (one level of rules,
rarely more than a media/supports/keyframes wrapper deep), so no separate
size-based collapsing (as in :mod:`xy.ai.mcpc.tools.ast.generic._yaml`) is
needed - only the grouped 'declarations'/'imports' segments enforce a budget.
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
__all__ = ['CssEngine']
"#: Max size (characters) a grouped 'imports'/'declarations' segment may reach."
_CSS_SEGMENT_MAX_CHARS = 1500
'#: Individually addressable "rule-like" node types: selector/at-rule preludes'
"#: with a nested block (or, for 'at_rule', possibly none - e.g. '@layer a, b;')."
_DEF_TYPES = {'rule_set', 'media_statement', 'supports_statement', 'keyframes_statement', 'at_rule', 'keyframe_block'}
"#: A rule's own body-container child, whose children are its declarations/nested rules."
_BODY_TYPES = {'block', 'keyframe_block_list'}
"#: Single-statement at-rules, grouped like Java's package/import declarations."
_IMPORT_TYPES = {'import_statement', 'charset_statement', 'namespace_statement'}

def _css_body(node: Any) -> Any | None:
    for child in node.named_children:
        if child.type in _BODY_TYPES:
            return child
    return None

def _css_prelude(node: Any, body: Any | None) -> str:
    """Everything before ``node``'s own block (its selector/at-rule header), single-lined.

    Doubles as both ``name`` (id/outline label) and the base for ``signature`` -
    there is no separate short identifier to extract: a rule's "name" *is* its
    selector or at-rule prelude, just like a Java method's is its full header.
    """
    end = body.start_byte if body is not None else node.end_byte
    raw = node.text[:end - node.start_byte]
    return ' '.join(raw.decode('utf-8', 'replace').split()).rstrip(';').strip()

class CssEngine(TreeSitterEngine):
    """Tree-sitter CSS restructured like the Java engine: rules/at-rules as real
    nodes (recursing into nested rules and keyframe blocks), everything else
    grouped into declaration/import segments."""

    def __init__(self) -> None:
        super().__init__('css')

    def is_definition(self, node_type: str) -> bool:
        return node_type in _DEF_TYPES

    def signature(self, node: Any, limit: int=80) -> str:
        text = _css_prelude(node, _css_body(node))
        return text if len(text) <= limit else text[:limit - 1] + '…'

    def locate_all(self, tree: Tree) -> list[Located]:
        results: list[Located] = []
        source = tree.source.encode('utf-8')

        def walk(children: list[Any], container: Any, path: str) -> None:
            used: dict[str, int] = {}
            i, n = (0, len(children))
            while i < n:
                node = children[i]
                if node.type in _DEF_TYPES:
                    body = _css_body(node)
                    name = _css_prelude(node, body) or None
                    seg = id_segment(name, i, used, content=self.node_code(node))
                    nid = f'{path}.{seg}' if path else seg
                    expandable = bool(body) and any((c.type in _DEF_TYPES for c in body.named_children))
                    results.append(
                        Located(
                            tree=tree,
                            node=node,
                            parent=container,
                            index=i,
                            node_id=nid,
                            node_type=node.type,
                            name=name,
                            lineno=node.start_point[0] + 1,
                            end_lineno=node.end_point[0] + 1,
                            parent_type=container.type,
                            expandable=expandable))
                    if body is not None:
                        walk(body.named_children, body, nid)
                    i += 1
                    continue
                start = i
                kind = 'imports' if node.type in _IMPORT_TYPES else 'declarations'
                length = 0
                while i < n:
                    current = children[i]
                    if current.type in _DEF_TYPES:
                        break
                    current_kind = 'imports' if current.type in _IMPORT_TYPES else 'declarations'
                    if current_kind != kind:
                        break
                    piece = current.end_byte - current.start_byte
                    if i > start and length + piece > _CSS_SEGMENT_MAX_CHARS:
                        break
                    length += piece
                    i += 1
                group = _SynthNode(kind, children[start:i], source)
                seg = id_segment(None, start, used, content=self.node_code(group))
                nid = f'{path}.{seg}' if path else seg
                results.append(
                    Located(
                        tree=tree,
                        node=group,
                        parent=container,
                        index=start,
                        node_id=nid,
                        node_type=kind,
                        name=None,
                        lineno=group.start_point[0] + 1,
                        end_lineno=group.end_point[0] + 1,
                        parent_type=container.type,
                        expandable=False))
        walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
        return results