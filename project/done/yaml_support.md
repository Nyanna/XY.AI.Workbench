Wir benötigen eine optimierte Darstellung von YAML analog der Anpassung in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py`. Die native Darstellung von Treesitter reicht bis auf jeden einzelnen Knoten hinunter, was imperformant ist.
Erzeuge analog eine eigene Engine auf Basis der Treesitter Engine.

- Der Treesitter AST soll dabei kollabiert werden auf Blätter mit einer Größe von nicht mehr als 1500 Zeichen.

```json
{
  "results" : [ {
    "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py",
    "nodes" : [ {
      "id" : "LjorUp",
      "type" : "statements"
    }, {
      "id" : "SlXmI5",
      "type" : "imports"
    }, {
      "id" : "CnEXKe",
      "type" : "statements"
    }, {
      "id" : "md_level",
      "type" : "FunctionDef",
      "signature" : "def _md_level(node: Any) -> int:",
      "docstring" : "Heading level (1-6) from an ``atx_h<N>_marker`` or ``setext_h<N>_underline`` ch…"
    }, {
      "id" : "md_flatten",
      "type" : "FunctionDef",
      "signature" : "def _md_flatten(node: Any, out: list[Any]) -> None:",
      "docstring" : "Collect nodes in document order, transparently unwrapping tree-sitter's own 'se…"
    }, {
      "id" : "md_split_pages",
      "type" : "FunctionDef",
      "signature" : "def _md_split_pages(flat: list[Any]) -> list[list[Any]]:",
      "docstring" : "Split a flat block list on 'thematic_break' ('---'), dropping empty pages."
    }, {
      "id" : "MdSection",
      "type" : "ClassDef",
      "signature" : "class _MdSection:",
      "docstring" : "Builder for one rebuilt heading section, before it is frozen into a ``_SynthNod…",
      "children" : [ {
        "id" : "MdSection.S4DmQY",
        "type" : "statements"
      }, {
        "id" : "MdSection.init",
        "type" : "FunctionDef",
        "signature" : "def __init__(self, level: int, heading: Any) -> None:"
      } ]
    }, {
      "id" : "md_nest_headings",
      "type" : "FunctionDef",
      "signature" : "def _md_nest_headings(items: list[Any]) -> list[Any]:",
      "docstring" : "Rebuild H1-H6 nesting from a flat block list, independent of tree-sitter's own …"
    }, {
      "id" : "md_finalize",
      "type" : "FunctionDef",
      "signature" : "def _md_finalize(nodes: list[Any], source: bytes) -> list[Any]:",
      "docstring" : "Freeze ``_MdSection`` builders (and their descendants) into ``_SynthNode('secti…"
    }, {
      "id" : "md_root_children",
      "type" : "FunctionDef",
      "signature" : "def _md_root_children(root_node: Any, source: bytes) -> list[Any]:",
      "docstring" : "Top-level children of a Markdown file: Pages if the file uses '---', else Secti…"
    }, {
      "id" : "MarkdownEngine",
      "type" : "ClassDef",
      "signature" : "class MarkdownEngine(TreeSitterEngine):",
      "docstring" : "Tree-sitter Markdown restructured into the fixed node set above.",
      "children" : [ {
        "id" : "MarkdownEngine.zKty4h",
        "type" : "statements"
      }, {
        "id" : "MarkdownEngine.init",
        "type" : "FunctionDef",
        "signature" : "def __init__(self) -> None:"
      }, {
        "id" : "MarkdownEngine.locate_all",
        "type" : "FunctionDef",
        "signature" : "def locate_all(self, tree: Tree) -> list[Located]:"
      }, {
        "id" : "MarkdownEngine.addressable",
        "type" : "FunctionDef",
        "signature" : "@staticmethod def _addressable(child: Any, depth: int) -> bool:"
      } ]
    } ]
  } ]
}
```
SystemInit:  id=f3f81522-d74d-487a-a252-76a2f87a9ac6, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 33a1-16
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
    ids:
    - LjorUp
    - SlXmI5
    - CnEXKe
    - md_level
    - md_flatten
    - md_split_pages
    - MdSection
    - md_nest_headings
    - md_finalize
    - md_root_children
    - MarkdownEngine
  reason: "Vollständigen Quellcode der Markdown-Engine lesen, um die Muster für die neue YAML-Engine zu übernehmen."
```
/answer 33a1-16 allow
Control Request:
```yaml
id: 33a1-17
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
      nodes:
      - id: LjorUp
        type: statements
        code: "\"Markdown tree-sitter engine: a fixed, MDAST-inspired node set.\\n\\nTree-sitter's own ``section`` nesting only follows heading level and has no\\nnotion of ``---`` page breaks (and is inconsistent for setext headings), so\\nthe structure is rebuilt from scratch - Page > Heading H1-H6 > Paragraph/\\nScriptBlock, mirroring the custom MDAST model in\\n``xy.ai.workbench.editor.mdast.nodes.Elements`` - from a flattened,\\ndocument-order block list (see :func:`_md_root_children`).\\n\""
      - id: SlXmI5
        type: imports
        code: |-
          from __future__ import annotations
          from typing import Any
          from xy.ai.mcpc.tools.ast.base import Located, Tree
          from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder, _SynthNode
      - id: CnEXKe
        type: statements
        code: |-
          __all__ = ['MarkdownEngine']
          _MD_HEADING_TYPES = ('atx_heading', 'setext_heading')
          '#: The only node types Markdown ever exposes as separately addressable'
          "#: (besides 'page' itself, always addressable via ``depth == 0``)."
          _MD_ADDRESSABLE_TYPES = {'page', 'section', 'paragraph', 'fenced_code_block'}
      - id: md_level
        type: FunctionDef
        code: |-
          def _md_level(node: Any) -> int:
              """Heading level (1-6) from an ``atx_h<N>_marker`` or ``setext_h<N>_underline`` child."""
              for child in node.named_children:
                  if 'marker' in child.type or 'underline' in child.type:
                      digits = ''.join((ch for ch in child.type if ch.isdigit()))
                      if digits:
                          return int(digits)
              return 1
      - id: md_flatten
        type: FunctionDef
        code: |-
          def _md_flatten(node: Any, out: list[Any]) -> None:
              """Collect nodes in document order, transparently unwrapping tree-sitter's own 'section'."""
              for child in node.named_children:
                  if child.type == 'section':
                      _md_flatten(child, out)
                  else:
                      out.append(child)
      - id: md_split_pages
        type: FunctionDef
        code: |-
          def _md_split_pages(flat: list[Any]) -> list[list[Any]]:
              """Split a flat block list on 'thematic_break' ('---'), dropping empty pages."""
              pages: list[list[Any]] = [[]]
              for node in flat:
                  if node.type == 'thematic_break':
                      pages.append([])
                  else:
                      pages[-1].append(node)
              return [page for page in pages if page]
      - id: MdSection
        type: ClassDef
        signature: "class _MdSection:"
        docstring: "Builder for one rebuilt heading section, before it is frozen into a ``_SynthNod…"
        children:
        - id: MdSection.S4DmQY
          type: statements
          code: |-
            'Builder for one rebuilt heading section, before it is frozen into a ``_SynthNode``.'
            __slots__ = ('level', 'items')
        - id: MdSection.init
          type: FunctionDef
          code: |-
            def __init__(self, level: int, heading: Any) -> None:
                self.level = level
                self.items: list[Any] = [heading]
      - id: md_nest_headings
        type: FunctionDef
        code: |-
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
      - id: md_finalize
        type: FunctionDef
        code: |-
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
      - id: md_root_children
        type: FunctionDef
        code: |-
          def _md_root_children(root_node: Any, source: bytes) -> list[Any]:
              """Top-level children of a Markdown file: Pages if the file uses '---', else Sections/Paragraphs directly."""
              flat: list[Any] = []
              _md_flatten(root_node, flat)
              if not any((node.type == 'thematic_break' for node in flat)):
                  pages = _md_split_pages(flat)
                  return _md_finalize(_md_nest_headings(pages[0]), source) if pages else []
              return [_SynthNode('page', _md_finalize(_md_nest_headings(page), source), source) for page in _md_split_pages(flat)]
      - id: MarkdownEngine
        type: ClassDef
        signature: "class MarkdownEngine(TreeSitterEngine):"
        docstring: Tree-sitter Markdown restructured into the fixed node set above.
        children:
        - id: MarkdownEngine.zKty4h
          type: statements
          code: '''Tree-sitter Markdown restructured into the fixed node set above.'''
        - id: MarkdownEngine.init
          type: FunctionDef
          code: |-
            def __init__(self) -> None:
                super().__init__('markdown')
        - id: MarkdownEngine.locate_all
          type: FunctionDef
          code: |-
            def locate_all(self, tree: Tree) -> list[Located]:
                root = _RootHolder(_md_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
                return self._locate_from(tree, root, self._addressable)
        - id: MarkdownEngine.addressable
          type: FunctionDef
          code: |-
            @staticmethod
            def _addressable(child: Any, depth: int) -> bool:
                return depth == 0 or child.type in _MD_ADDRESSABLE_TYPES
```
/answer 33a1-17 allow
ReasoningToken: 91
Control Request:
```yaml
id: 33a1-18
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic
  reason: "Verzeichnis der generischen AST-Engines auflisten, um vorhandene Sprachengines und Registrierung zu finden."
```
/answer 33a1-18 deny TreeSitter findest du hier `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py`
ReasoningToken: 50
Control Request:
```yaml
id: 33a1-19
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
  reason: "Basis-Engine-Klasse (TreeSitterEngine, _RootHolder, _SynthNode, _locate_from) verstehen, um analoge YAML-Engine zu bauen."
```
/answer 33a1-19 allow
Control Request:
```yaml
id: 33a1-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: BtswSM
        type: statements
      - id: KarOwC
        type: imports
      - id: 5mhVsN
        type: statements
      - id: SynthNode
        type: ClassDef
        signature: "class _SynthNode:"
        docstring: Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds itself…
        children:
        - id: SynthNode.yS8Eci
          type: statements
        - id: SynthNode.init
          type: FunctionDef
          signature: "def __init__(self, node_type: str, children: list[Any], source: bytes) -> None:"
        - id: SynthNode.text
          type: FunctionDef
          signature: "@property def text(self) -> bytes:"
        - id: SynthNode.child_by_field_name
          type: FunctionDef
          signature: "@staticmethod def child_by_field_name(_field: str) -> None:"
      - id: RootHolder
        type: ClassDef
        signature: "class _RootHolder:"
        docstring: Fake container so the shared node-walker can start from a plain child list.
        children:
        - id: RootHolder.Rk25Eh
          type: statements
        - id: RootHolder.init
          type: FunctionDef
          signature: "def __init__(self, children: list[Any]) -> None:"
      - id: TreeSitterEngine
        type: ClassDef
        signature: "class TreeSitterEngine(Engine):"
        docstring: One tree-sitter grammar exposed through the common :class:`Engine` API. Instanc…
        children:
        - id: TreeSitterEngine.jU5ePL
          type: statements
        - id: TreeSitterEngine.init
          type: FunctionDef
          signature: "def __init__(self, symbol: str) -> None:"
        - id: TreeSitterEngine.parse
          type: FunctionDef
          signature: "def _parse(self, data: bytes):"
        - id: TreeSitterEngine.parse_1
          type: FunctionDef
          signature: "def parse(self, source: str, path: Path | None=None) -> Tree:"
        - id: TreeSitterEngine.empty_tree
          type: FunctionDef
          signature: "def empty_tree(self, path: Path | None=None) -> Tree:"
        - id: TreeSitterEngine.serialize
          type: FunctionDef
          signature: "def serialize(self, tree: Tree) -> str:"
        - id: TreeSitterEngine.validate
          type: FunctionDef
          signature: "def validate(self, source: str) -> str | None:"
        - id: TreeSitterEngine.name
          type: FunctionDef
          signature: "def _name(self, node: Any) -> str | None:"
        - id: TreeSitterEngine.clean_heading
          type: FunctionDef
          signature: "@staticmethod def _clean_heading(raw: bytes) -> str:"
        - id: TreeSitterEngine.clean
          type: FunctionDef
          signature: "@staticmethod def _clean(raw: bytes) -> str:"
        - id: TreeSitterEngine.locate_all
          type: FunctionDef
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
        - id: TreeSitterEngine.locate_from
          type: FunctionDef
          signature: "def _locate_from(self, tree: Tree, root: Any, addressable: Callable[[Any, int], bool]) -> list[Located]:"
          docstring: "Shared node-walker: ``root`` and ``addressable`` let subclasses curate a differ…"
        - id: TreeSitterEngine.signature
          type: FunctionDef
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: TreeSitterEngine.docstring
          type: FunctionDef
          signature: "def docstring(self, node: Any) -> str | None:"
        - id: TreeSitterEngine.node_code
          type: FunctionDef
          signature: "def node_code(self, node: Any) -> str:"
        - id: TreeSitterEngine.splice
          type: FunctionDef
          signature: "def _splice(self, tree: Tree, start: int, end: int, text: str) -> None:"
        - id: TreeSitterEngine.replace
          type: FunctionDef
          signature: "def replace(self, loc: Located, code: str) -> None:"
        - id: TreeSitterEngine.insert
          type: FunctionDef
          signature: "def insert(self, loc: Located, code: str, position: str) -> int:"
        - id: TreeSitterEngine.delete
          type: FunctionDef
          signature: "def delete(self, loc: Located) -> None:"
        - id: TreeSitterEngine.append
          type: FunctionDef
          signature: "def append(self, tree: Tree, code: str) -> int:"
```
/answer 33a1-1a allow
Control Request:
```yaml
id: 33a1-1b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    ids:
    - BtswSM
    - KarOwC
    - 5mhVsN
    - SynthNode
    - RootHolder
    - TreeSitterEngine.init
    - TreeSitterEngine.locate_all
    - TreeSitterEngine.locate_from
    - TreeSitterEngine.name
    - TreeSitterEngine.signature
    - TreeSitterEngine.docstring
  reason: "Detaillierten Code der Basisklasse verstehen (locate_from, name, signature), um die YAML-Engine korrekt zu implementieren."
```
/answer 33a1-1b allow
Control Request:
```yaml
id: 33a1-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: BtswSM
        type: statements
        code: "\"Generic tree-sitter back-end for every non-Python language/format.\\n\\nGrammars come from ``tree_sitter_language_pack`` (~370 languages, compatible\\nwith tree-sitter 0.26+). Unlike the Python engine there is no ``unparse``:\\nmutations are plain source-text operations - splice a node's byte-range, or\\nconcatenate - followed by a re-parse, matching how these formats are edited\\nin practice.\\n\\nNodes are addressed engine-independently by a single dotted ``id`` over the\\n*named* child hierarchy, name/hash based (a Markdown heading always collapses\\nto a 6-char hex hash) or numeric where no name exists. By default every named\\nchild is addressable, exposing the grammar's native structure as-is; a\\nlanguage whose native tree doesn't match how it's actually edited overrides\\n:meth:`TreeSitterEngine.locate_all` in its own subclass (see\\n:mod:`xy.ai.mcpc.tools.ast.generic._markdown` for Markdown's Page/Heading\\nmodel) instead of special-casing this base engine.\\n\""
      - id: KarOwC
        type: imports
        code: |-
          from __future__ import annotations
          from pathlib import Path
          from typing import Any, Callable
          from tree_sitter_language_pack import get_parser
          from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, Tree, id_segment
      - id: 5mhVsN
        type: statements
        code: |-
          __all__ = ['TreeSitterEngine']
          "#: Named child types that usually carry a node's identifier/key."
          _NAME_TYPES = {
              'identifier',
              'property_identifier',
              'field_identifier',
              'type_identifier',
              'constant',
              'key',
              'string',
              'bare_key',
              'dotted_key',
              'flow_node',
              'plain_scalar',
              'tag'}
      - id: SynthNode
        type: ClassDef
        signature: "class _SynthNode:"
        docstring: Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds itself…
        children:
        - id: SynthNode.yS8Eci
          type: statements
          code: |-
            'Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds\n    itself (a contiguous run of real children treated as one unit).'
            __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_source')
        - id: SynthNode.init
          type: FunctionDef
          code: |-
            def __init__(self, node_type: str, children: list[Any], source: bytes) -> None:
                self.type = node_type
                self.named_children = children
                self.start_byte = children[0].start_byte
                self.end_byte = children[-1].end_byte
                self.start_point = children[0].start_point
                self.end_point = children[-1].end_point
                self._source = source
        - id: SynthNode.text
          type: FunctionDef
          code: |-
            @property
            def text(self) -> bytes:
                return self._source[self.start_byte:self.end_byte]
        - id: SynthNode.child_by_field_name
          type: FunctionDef
          code: |-
            @staticmethod
            def child_by_field_name(_field: str) -> None:
                return None
      - id: RootHolder
        type: ClassDef
        signature: "class _RootHolder:"
        docstring: Fake container so the shared node-walker can start from a plain child list.
        children:
        - id: RootHolder.Rk25Eh
          type: statements
          code: |-
            'Fake container so the shared node-walker can start from a plain child list.'
            __slots__ = ('named_children', 'type')
        - id: RootHolder.init
          type: FunctionDef
          code: |-
            def __init__(self, children: list[Any]) -> None:
                self.named_children = children
                self.type = None
      - id: TreeSitterEngine.init
        type: FunctionDef
        code: |-
          def __init__(self, symbol: str) -> None:
              self.symbol = symbol
              self.name = f'tree-sitter:{symbol}'
              self._parser = None
      - id: TreeSitterEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              return self._locate_from(tree, tree.raw.root_node, lambda child, depth: True)
      - id: TreeSitterEngine.locate_from
        type: FunctionDef
        code: |-
          def _locate_from(self, tree: Tree, root: Any, addressable: Callable[[Any, int], bool]) -> list[Located]:
              """Shared node-walker: ``root`` and ``addressable`` let subclasses curate a
                  different (possibly synthetic, see ``_markdown._RootHolder``) node set."""
              results: list[Located] = []

              def walk(node: Any, path: str, depth: int) -> None:
                  used: dict[str, int] = {}
                  for index, child in enumerate(node.named_children):
                      if not addressable(child, depth):
                          continue
                      is_hashed = child.type in ('section', 'page')
                      name = self._name(child)
                      seg = id_segment(name, index, used, hash_only=is_hashed, content=self.node_code(child))
                      nid = f'{path}.{seg}' if path else seg
                      addr_children = [c for c in child.named_children if addressable(c, depth + 1)]
                      results.append(
                          Located(
                              tree=tree,
                              node=child,
                              parent=node,
                              index=index,
                              node_id=nid,
                              node_type=child.type,
                              name=name,
                              lineno=child.start_point[0] + 1,
                              end_lineno=child.end_point[0] + 1,
                              parent_type=node.type,
                              expandable=bool(addr_children)))
                      walk(child, nid, depth + 1)
              walk(root, '', 0)
              return results
      - id: TreeSitterEngine.name
        type: FunctionDef
        code: |-
          def _name(self, node: Any) -> str | None:
              if node.type == 'section':
                  for child in node.named_children:
                      if child.type.endswith('heading'):
                          return self._clean_heading(child.text)
              for field in ('name', 'key', 'tag'):
                  child = node.child_by_field_name(field)
                  if child is not None:
                      return self._clean(child.text)
              for child in node.named_children:
                  if child.type in _NAME_TYPES:
                      return self._clean(child.text)
              return None
      - id: TreeSitterEngine.signature
        type: FunctionDef
        code: |-
          def signature(self, node: Any, limit: int=80) -> str:
              first_line = node.text.decode('utf-8', 'replace').splitlines()[0] if node.text else ''
              first_line = first_line.strip()
              return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
      - id: TreeSitterEngine.docstring
        type: FunctionDef
        code: |-
          def docstring(self, node: Any) -> str | None:
              return None
```
/answer 33a1-1c allow
Control Request:
```yaml
id: 33a1-1d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: MarkdownEngine|yaml|Yaml|YAML
    include:
    - '*.py'
    exclude:
    - '*.min.js'
  reason: Registrierungsstelle für Engines finden (wo MarkdownEngine referenziert wird) und prüfen ob bereits eine YAML-Engine existiert.
```
/answer 33a1-1d deny Hier `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py`
ReasoningToken: 81
Control Request:
```yaml
id: 33a1-1e
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: "Registrierungsstelle für Engines prüfen (wo MarkdownEngine registriert wird), um dort die YAML-Engine analog einzutragen."
```
/answer 33a1-1e allow
Control Request:
```yaml
id: 33a1-1f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: guMKQx
        type: statements
      - id: CjFjhH
        type: imports
      - id: MaD68b
        type: statements
      - id: XdqYqL
        type: statements
      - id: language_for_extension
        type: FunctionDef
        signature: "def language_for_extension(ext: str) -> str | None:"
      - id: 4zaPLB
        type: statements
      - id: get_engine
        type: FunctionDef
        signature: "def get_engine(symbol: str) -> TreeSitterEngine:"
      - id: byGZAx
        type: statements
      - id: fallback_engine
        type: FunctionDef
        signature: "def fallback_engine() -> PlainTextEngine:"
        docstring: Whole-file engine used when no tree-sitter grammar claims the extension.
```
/answer 33a1-1f allow
Control Request:
```yaml
id: 33a1-20
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - guMKQx
    - CjFjhH
    - MaD68b
    - XdqYqL
    - language_for_extension
    - 4zaPLB
    - get_engine
    - byGZAx
  reason: "Vollständigen Registrierungscode lesen, um YAML-Engine analog einzuhängen."
```
/answer 33a1-20 allow
Control Request:
```yaml
id: 33a1-21
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: guMKQx
        type: statements
        code: "\"Generic tree-sitter back-ends for every non-Python language/format.\\n\\nSplit into :mod:`xy.ai.mcpc.tools.ast.generic._engine` (the universal\\n:class:`TreeSitterEngine`, exposing a grammar's native structure as-is) and\\nper-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`\\n(:class:`MarkdownEngine`); this module re-exports the package's public\\nsurface, dispatches a language symbol to its engine, and hands out the\\nwhole-file :class:`~xy.ai.mcpc.tools.ast.generic._text.PlainTextEngine`\\nfallback (via :func:`fallback_engine`) for extensions no grammar covers.\\n\""
      - id: CjFjhH
        type: imports
        code: |-
          from __future__ import annotations
          from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine
          from xy.ai.mcpc.tools.ast.generic._java import JavaEngine
          from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
          from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
      - id: MaD68b
        type: statements
        code: |-
          __all__ = [
              'TreeSitterEngine',
              'JavaEngine',
              'MarkdownEngine',
              'PlainTextEngine',
              'language_for_extension',
              'get_engine',
              'fallback_engine']
          '#: File extension -> ``tree_sitter_language_pack`` language identifier.'
      - id: XdqYqL
        type: statements
        code: |-
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
      - id: language_for_extension
        type: FunctionDef
        code: |-
          def language_for_extension(ext: str) -> str | None:
              return EXT_LANGUAGE.get(ext.lower())
      - id: 4zaPLB
        type: statements
        code: |-
          '#: Language symbol -> dedicated Engine subclass; anything absent here falls'
          '#: back to the universal :class:`TreeSitterEngine`.'
          _ENGINE_CLASSES: dict[str, type[TreeSitterEngine]] = {'markdown': MarkdownEngine, 'java': JavaEngine}
          _ENGINES: dict[str, TreeSitterEngine] = {}
      - id: get_engine
        type: FunctionDef
        code: |-
          def get_engine(symbol: str) -> TreeSitterEngine:
              engine = _ENGINES.get(symbol)
              if engine is None:
                  engine_cls = _ENGINE_CLASSES.get(symbol)
                  engine = engine_cls() if engine_cls else TreeSitterEngine(symbol)
                  _ENGINES[symbol] = engine
              return engine
      - id: byGZAx
        type: statements
        code: _FALLBACK_ENGINE = PlainTextEngine()
```
/answer 33a1-21 allow
Control Request:
```yaml
id: 33a1-22
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('yaml')
    src = b"""
    a: 1
    b:
      c: 2
      d:
        - x
        - y
        - z: 1
          w: 2
    list:
      - name: one
        value: 1
      - name: two
        value: 2
    str: |
      multi
      line
      block
    anchor: &a
      foo: bar
    ref: *a
    """
    tree = parser.parse(src)
    def walk(node, depth=0, field=None):
        print('  '*depth, field, node.type, node.start_byte, node.end_byte, node.is_named)
        for child in node.children:
            fname = None
            for f in node.child_by_field_name.__self__.__class__.__dict__:
                pass
            walk(child, depth+1)
    walk(tree.root_node)
  reason: "Reale Node-Typen der tree-sitter-yaml Grammatik untersuchen, um die Struktur für die kollabierte Engine zu verstehen."
```
/answer 33a1-22 allow
Control Request:
```yaml
id: 33a1-23
result:
  structuredContent:
    exit_code: 0
    stdout: |2
       None stream 1 179 True
         None document 1 179 True
           None block_node 1 179 True
             None block_mapping 1 179 True
               None block_mapping_pair 1 5 True
                 None flow_node 1 2 True
                   None plain_scalar 1 2 True
                     None string_scalar 1 2 True
                 None : 2 3 False
                 None flow_node 4 5 True
                   None plain_scalar 4 5 True
                     None integer_scalar 4 5 True
               None block_mapping_pair 6 58 True
                 None flow_node 6 7 True
                   None plain_scalar 6 7 True
                     None string_scalar 6 7 True
                 None : 7 8 False
                 None block_node 11 58 True
                   None block_mapping 11 58 True
                     None block_mapping_pair 11 15 True
                       None flow_node 11 12 True
                         None plain_scalar 11 12 True
                           None string_scalar 11 12 True
                       None : 12 13 False
                       None flow_node 14 15 True
                         None plain_scalar 14 15 True
                           None integer_scalar 14 15 True
                     None block_mapping_pair 18 58 True
                       None flow_node 18 19 True
                         None plain_scalar 18 19 True
                           None string_scalar 18 19 True
                       None : 19 20 False
                       None block_node 25 58 True
                         None block_sequence 25 58 True
                           None block_sequence_item 25 28 True
                             None - 25 26 False
                             None flow_node 27 28 True
                               None plain_scalar 27 28 True
                                 None string_scalar 27 28 True
                           None block_sequence_item 33 36 True
                             None - 33 34 False
                             None flow_node 35 36 True
                               None plain_scalar 35 36 True
                                 None string_scalar 35 36 True
                           None block_sequence_item 41 58 True
                             None - 41 42 False
                             None block_node 43 58 True
                               None block_mapping 43 58 True
                                 None block_mapping_pair 43 47 True
                                   None flow_node 43 44 True
                                     None plain_scalar 43 44 True
                                       None string_scalar 43 44 True
                                   None : 44 45 False
                                   None flow_node 46 47 True
                                     None plain_scalar 46 47 True
                                       None integer_scalar 46 47 True
                                 None block_mapping_pair 54 58 True
                                   None flow_node 54 55 True
                                     None plain_scalar 54 55 True
                                       None string_scalar 54 55 True
                                   None : 55 56 False
                                   None flow_node 57 58 True
                                     None plain_scalar 57 58 True
                                       None integer_scalar 57 58 True
               None block_mapping_pair 59 118 True
                 None flow_node 59 63 True
                   None plain_scalar 59 63 True
                     None string_scalar 59 63 True
                 None : 63 64 False
                 None block_node 67 118 True
                   None block_sequence 67 118 True
                     None block_sequence_item 67 91 True
                       None - 67 68 False
                       None block_node 69 91 True
                         None block_mapping 69 91 True
                           None block_mapping_pair 69 78 True
                             None flow_node 69 73 True
                               None plain_scalar 69 73 True
                                 None string_scalar 69 73 True
                             None : 73 74 False
                             None flow_node 75 78 True
                               None plain_scalar 75 78 True
                                 None string_scalar 75 78 True
                           None block_mapping_pair 83 91 True
                             None flow_node 83 88 True
                               None plain_scalar 83 88 True
                                 None string_scalar 83 88 True
                             None : 88 89 False
                             None flow_node 90 91 True
                               None plain_scalar 90 91 True
                                 None integer_scalar 90 91 True
                     None block_sequence_item 94 118 True
                       None - 94 95 False
                       None block_node 96 118 True
                         None block_mapping 96 118 True
                           None block_mapping_pair 96 105 True
                             None flow_node 96 100 True
                               None plain_scalar 96 100 True
                                 None string_scalar 96 100 True
                             None : 100 101 False
                             None flow_node 102 105 True
                               None plain_scalar 102 105 True
                                 None string_scalar 102 105 True
                           None block_mapping_pair 110 118 True
                             None flow_node 110 115 True
                               None plain_scalar 110 115 True
                                 None string_scalar 110 115 True
                             None : 115 116 False
                             None flow_node 117 118 True
                               None plain_scalar 117 118 True
                                 None integer_scalar 117 118 True
               None block_mapping_pair 119 148 True
                 None flow_node 119 122 True
                   None plain_scalar 119 122 True
                     None string_scalar 119 122 True
                 None : 122 123 False
                 None block_node 124 148 True
                   None block_scalar 124 148 True
                     None | 124 125 False
               None block_mapping_pair 149 170 True
                 None flow_node 149 155 True
                   None plain_scalar 149 155 True
                     None string_scalar 149 155 True
                 None : 155 156 False
                 None block_node 157 170 True
                   None anchor 157 159 True
                     None & 157 158 False
                     None anchor_name 158 159 True
                   None block_mapping 162 170 True
                     None block_mapping_pair 162 170 True
                       None flow_node 162 165 True
                         None plain_scalar 162 165 True
                           None string_scalar 162 165 True
                       None : 165 166 False
                       None flow_node 167 170 True
                         None plain_scalar 167 170 True
                           None string_scalar 167 170 True
               None block_mapping_pair 171 178 True
                 None flow_node 171 174 True
                   None plain_scalar 171 174 True
                     None string_scalar 171 174 True
                 None : 174 175 False
                 None flow_node 176 178 True
                   None alias 176 178 True
                     None * 176 177 False
                     None alias_name 177 178 True
```
/answer 33a1-23 allow
ReasoningToken: 2288
Control Request:
```yaml
id: 33a1-24
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('yaml')
    src = b"""
    flowmap: {a: 1, b: 2}
    flowseq: [1, 2, 3]
    multidoc: 1
    ---
    doc2: true
    """
    tree = parser.parse(src)
    def walk(node, depth=0):
        print('  '*depth, node.type, node.start_byte, node.end_byte)
        for child in node.children:
            walk(child, depth+1)
    walk(tree.root_node)
  reason: Flow-Mapping/-Sequence und Multi-Dokument-Struktur der YAML-Grammatik prüfen.
```
/answer 33a1-24 allow
Control Request:
```yaml
id: 33a1-25
result:
  structuredContent:
    exit_code: 0
    stdout: |2
       stream 1 69
         document 1 53
           block_node 1 53
             block_mapping 1 53
               block_mapping_pair 1 22
                 flow_node 1 8
                   plain_scalar 1 8
                     string_scalar 1 8
                 : 8 9
                 flow_node 10 22
                   flow_mapping 10 22
                     { 10 11
                     flow_pair 11 15
                       flow_node 11 12
                         plain_scalar 11 12
                           string_scalar 11 12
                       : 12 13
                       flow_node 14 15
                         plain_scalar 14 15
                           integer_scalar 14 15
                     , 15 16
                     flow_pair 17 21
                       flow_node 17 18
                         plain_scalar 17 18
                           string_scalar 17 18
                       : 18 19
                       flow_node 20 21
                         plain_scalar 20 21
                           integer_scalar 20 21
                     } 21 22
               block_mapping_pair 23 41
                 flow_node 23 30
                   plain_scalar 23 30
                     string_scalar 23 30
                 : 30 31
                 flow_node 32 41
                   flow_sequence 32 41
                     [ 32 33
                     flow_node 33 34
                       plain_scalar 33 34
                         integer_scalar 33 34
                     , 34 35
                     flow_node 36 37
                       plain_scalar 36 37
                         integer_scalar 36 37
                     , 37 38
                     flow_node 39 40
                       plain_scalar 39 40
                         integer_scalar 39 40
                     ] 40 41
               block_mapping_pair 42 53
                 flow_node 42 50
                   plain_scalar 42 50
                     string_scalar 42 50
                 : 50 51
                 flow_node 52 53
                   plain_scalar 52 53
                     integer_scalar 52 53
         document 54 69
           --- 54 57
           block_node 58 69
             block_mapping 58 69
               block_mapping_pair 58 68
                 flow_node 58 62
                   plain_scalar 58 62
                     string_scalar 58 62
                 : 62 63
                 flow_node 64 68
                   plain_scalar 64 68
                     boolean_scalar 64 68
```
/answer 33a1-25 allow
ReasoningToken: 701
Control Request:
```yaml
id: 33a1-26
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('yaml')
    src = b"a: 1\nb:\n  c: 2\n"
    tree = parser.parse(src)
    doc = tree.root_node.named_children[0]
    bn = doc.named_children[0]
    bm = bn.named_children[0]
    pair = bm.named_children[0]
    print('pair type', pair.type)
    key = pair.child_by_field_name('key')
    value = pair.child_by_field_name('value')
    print('key field', key, key.type if key else None)
    print('value field', value, value.type if value else None)
    seq_src = b"items:\n  - a\n  - b\n"
    tree2 = parser.parse(seq_src)
    def find_type(node, t):
        if node.type == t:
            return node
        for c in node.children:
            r = find_type(c, t)
            if r: return r
        return None
    item = find_type(tree2.root_node, 'block_sequence_item')
    print('item type', item.type, [c.type for c in item.children])
    print('item fields: value=', item.child_by_field_name('value'))
  reason: "Prüfen, ob die YAML-Grammatik Feldnamen (key/value) für block_mapping_pair und block_sequence_item bereitstellt, um sinnvolle Node-Namen zu erzeugen."
```
/answer 33a1-26 allow
Control Request:
```yaml
id: 33a1-27
result:
  structuredContent:
    exit_code: 0
    stdout: |
      pair type block_mapping_pair
      key field (flow_node (plain_scalar (string_scalar))) flow_node
      value field (flow_node (plain_scalar (integer_scalar))) flow_node
      item type block_sequence_item ['-', 'flow_node']
      item fields: value= None
```
/answer 33a1-27 allow
ReasoningToken: 9420
Control Request:
```yaml
id: 33a1-28
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: def id_segment
    include:
    - '*.py'
    exclude: []
  reason: "Implementierung von id_segment finden, um Namensvergabe-Verhalten bei fehlendem Namen zu verstehen."
```
/answer 33a1-28 allow Eine Möglichkeit ist Contenthash, zu bevorzugen sind aber stabilere Varianten. Markdown würde erlauben auf basis der Knotensemantik zu Unterscheiden. Eine List.Object.Liste ist etwas stabiler als ein Conten Hash.
Control Request:
```yaml
id: 33a1-29
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      pattern: def id_segment
      matches:
      - path: base.py
        lineno: 142
        match: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
```
/answer 33a1-29 allow
ReasoningToken: 1385
Control Request:
```yaml
id: 33a1-2a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  name: id_segment
  reason: "Vollständige Implementierung von id_segment lesen, um Namensgebung für YAML-Knoten (Key vs. Index vs. Hash) korrekt zu gestalten."
```
/answer 33a1-2a allow
Control Request:
```yaml
id: 33a1-2b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: id_segment
        type: FunctionDef
        code: |-
          def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:
              """Return a unique-within-siblings id segment, name-based when feasible.

              A clean, short name becomes the segment verbatim; a long/awkward name collapses
              to a short hash; a nameless node falls back to a 6-char content hash (derived
              from ``content``, stable across edits made elsewhere in the file) or, lacking
              that, its numeric ``index``. With ``hash_only`` the name is *always* reduced to
              a 6-char hex hash (used for Markdown headings, whose id must never be the
              literal heading text). Collisions among siblings get a numeric suffix.
              """
              seg: str | None = None
              if name:
                  if hash_only:
                      seg = _hash(name, 6)
                  else:
                      cleaned = _ID_CLEAN_RE.sub('_', name).strip('_')
                      seg = cleaned if cleaned and len(cleaned) <= 40 else 'h' + _hash(name, 8)
              if not seg:
                  seg = _content_hash(content, 6) if content else str(index)
              count = used.get(seg, 0)
              used[seg] = count + 1
              return seg if count == 0 else f'{seg}_{count}'
```
/answer 33a1-2b allow
ReasoningToken: 169
Control Request:
```yaml
id: 33a1-2c
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  name: Located
  reason: "Located-Datenklasse verstehen, um sie im eigenen Walker korrekt zu befüllen."
```
/answer 33a1-2c allow Ich denke Markdown hat durchaus eine Mögliche Namespace Struktur, wo es auf eindeutig benannte elemente trifft können diese in Kombination mit Index Fallback verwendet werden. Für alle andere braucht es eine andere Methode.
Control Request:
```yaml
id: 33a1-2d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: Located
        type: ClassDef
        code: |-
          @dataclass
          class Located:
              """A node with the engine-independent metadata the selectors match on.

              Attributes:
                  tree: The owning tree (for engine dispatch).
                  node: Engine-native node object.
                  parent: Engine-native container node.
                  index: Position of ``node`` among its parent's addressable children.
                  node_id: The node's unique ``id`` — its fully-qualified path from the
                      root (e.g. ``"MyClass.method"``), in name/hash form or, for nameless
                      nodes/segments, a stable content-hash fallback. There is no separate FQN.
                  node_type: Engine-reported node type name.
                  name: Simple name, if the node carries one.
                  lineno / end_lineno: 1-based inclusive line span.
                  parent_type: Type name of ``parent``, or ``None`` at the top level.
                  expandable: Whether ``read`` should descend into children instead of
                      returning the node's full source (a pure container of nested defs).
              """
              tree: Tree
              node: Any
              parent: Any
              index: int
              node_id: str
              node_type: str
              name: str | None
              lineno: int
              end_lineno: int
              parent_type: str | None
              expandable: bool = False
```
/answer 33a1-2d allow
ReasoningToken: 6416
Control Request:
```yaml
id: 33a1-2e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    ids:
    - TreeSitterEngine.node_code
    - TreeSitterEngine.clean
  reason: "node_code/_clean Implementierung prüfen, um sie im eigenen Walker konsistent wiederzuverwenden."
```
/answer 33a1-2e allow Korrektur, ich meine YAML, wo es benannte YAML Knoten gibt.
Control Request:
```yaml
id: 33a1-2f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: TreeSitterEngine.node_code
        type: FunctionDef
        code: |-
          def node_code(self, node: Any) -> str:
              return node.text.decode('utf-8', 'replace')
      - id: TreeSitterEngine.clean
        type: FunctionDef
        code: |-
          @staticmethod
          def _clean(raw: bytes) -> str:
              return raw.decode('utf-8', 'replace').strip().strip('"\'')
```
/answer 33a1-2f allow
ReasoningToken: 1176
Control Request:
```yaml
id: 33a1-30
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('yaml')
    src = b"m: {a: 1, b: 2}\n"
    tree = parser.parse(src)
    def find_type(node, t):
        if node.type == t:
            return node
        for c in node.children:
            r = find_type(c, t)
            if r: return r
        return None
    fp = find_type(tree.root_node, 'flow_pair')
    print(fp.type, fp.child_by_field_name('key'))
  reason: "Prüfen, ob flow_pair ebenfalls ein 'key'-Feld besitzt wie block_mapping_pair."
```
/answer 33a1-30 allow
Control Request:
```yaml
id: 33a1-31
result:
  structuredContent:
    exit_code: 0
    stdout: |
      flow_pair (flow_node (plain_scalar (string_scalar)))
```
/answer 33a1-31 allow
ReasoningToken: 2417
Control Request:
```yaml
id: 33a1-32
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_yaml.py
    source: |
      "YAML tree-sitter engine: leaves collapsed to a size budget instead of the full grammar depth.

      Tree-sitter's own YAML tree exposes every scalar, flow wrapper ('flow_node',
      'block_node', ...) and punctuation token as its own node, which is far too
      fine-grained to address individually on large files. This engine instead
      rebuilds the tree from 'block_mapping_pair' / 'flow_pair' / 'block_sequence_item'
      units - transparently unwrapping the purely structural 'document', 'block_node',
      'flow_node', 'block_mapping', 'block_sequence', 'flow_mapping' and
      'flow_sequence' containers in between (see :func:`_yaml_units`) - and collapses
      each unit into a childless leaf as soon as its full text fits
      :data:`_YAML_LEAF_LIMIT`, only recursing into its own nested units when it
      doesn't (see :func:`_yaml_collapse`).
      "

      from __future__ import annotations

      from typing import Any

      from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
      from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder

      __all__ = ['YamlEngine']

      "#: Max size (characters) a collapsed leaf's full text may still have."
      _YAML_LEAF_LIMIT = 1500
      "#: Node types individually addressable - a mapping entry, a flow-mapping entry"
      "#: or a sequence item; everything else is either a leaf's inner detail or one"
      "#: of the wrapper types below."
      _YAML_UNIT_TYPES = {'block_mapping_pair', 'flow_pair', 'block_sequence_item'}
      "#: Purely structural nodes, transparently unwrapped on the way to the next unit."
      _YAML_WRAPPER_TYPES = {
          'document',
          'block_node',
          'flow_node',
          'block_mapping',
          'block_sequence',
          'flow_mapping',
          'flow_sequence'}
      _YAML_ADDRESSABLE_TYPES = _YAML_UNIT_TYPES | {'document'}


      def _yaml_units(node: Any) -> list[Any]:
          """Direct logical children of ``node``: its own unit-type descendants, found by
          transparently unwrapping wrapper nodes (comments/anchors/punctuation in between are dropped)."""
          units: list[Any] = []
          for child in node.named_children:
              if child.type in _YAML_UNIT_TYPES:
                  units.append(child)
              elif child.type in _YAML_WRAPPER_TYPES:
                  units.extend(_yaml_units(child))
          return units


      class _YamlUnit:
          """One collapsed YAML unit: either a childless leaf (its full text already fits
          the leaf budget, or it has no nested unit to expand into anyway) or a branch
          whose children are its own recursively collapsed sub-units."""
          __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_node', '_source')

          def __init__(self, node: Any, children: list[Any], source: bytes) -> None:
              self.type = node.type
              self.named_children = children
              self.start_byte = node.start_byte
              self.end_byte = node.end_byte
              self.start_point = node.start_point
              self.end_point = node.end_point
              self._node = node
              self._source = source

          @property
          def text(self) -> bytes:
              return self._source[self.start_byte:self.end_byte]

          def child_by_field_name(self, field: str) -> Any:
              "Delegate to the wrapped native node, so key/value based naming keeps working."
              return self._node.child_by_field_name(field)


      def _yaml_collapse(node: Any, source: bytes) -> _YamlUnit:
          """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
          nested unit to expand into), else into a branch over its own collapsed sub-units."""
          sub_units = _yaml_units(node)
          size = len(node.text.decode('utf-8', 'replace'))
          children = [] if size <= _YAML_LEAF_LIMIT or not sub_units else [_yaml_collapse(unit, source) for unit in sub_units]
          return _YamlUnit(node, children, source)


      def _yaml_root_children(root_node: Any, source: bytes) -> list[Any]:
          """Top-level children of a YAML file: one leaf/branch per unit of the (single)
          document, or one per '---'-separated document in a multi-document stream."""
          documents = [child for child in root_node.named_children if child.type == 'document']
          if len(documents) > 1:
              return [_yaml_collapse(document, source) for document in documents]
          if not documents:
              return []
          units = _yaml_units(documents[0])
          if units:
              return [_yaml_collapse(unit, source) for unit in units]
          return [_yaml_collapse(documents[0], source)] if documents[0].named_children else []


      class YamlEngine(TreeSitterEngine):
          "Tree-sitter YAML restructured into the size-collapsed leaves above."

          def __init__(self) -> None:
              super().__init__('yaml')

          def locate_all(self, tree: Tree) -> list[Located]:
              root = _RootHolder(_yaml_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
              results: list[Located] = []

              def walk(node: Any, path: str, depth: int) -> None:
                  used: dict[str, int] = {}
                  for index, child in enumerate(node.named_children):
                      if depth > 0 and child.type not in _YAML_ADDRESSABLE_TYPES:
                          continue
                      name = self._name(child)
                      # Sequence items have no natural name: a stable positional index is
                      # preferable to a content hash, which would move on every edit elsewhere.
                      content = None if child.type == 'block_sequence_item' else self.node_code(child)
                      seg = id_segment(name, index, used, content=content)
                      nid = f'{path}.{seg}' if path else seg
                      results.append(
                          Located(
                              tree=tree,
                              node=child,
                              parent=node,
                              index=index,
                              node_id=nid,
                              node_type=child.type,
                              name=name,
                              lineno=child.start_point[0] + 1,
                              end_lineno=child.end_point[0] + 1,
                              parent_type=node.type,
                              expandable=bool(child.named_children)))
                      walk(child, nid, depth + 1)
              walk(root, '', 0)
              return results
    overwrite: false
  reason: Neue YAML-Engine-Datei analog zu _markdown.py mit größenbasiertem Kollabieren der Blätter (≤1500 Zeichen) anlegen.
```
/answer 33a1-32 allow
Control Request:
```yaml
id: 33a1-33
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_yaml.py
      error: "Syntax error: unterminated string literal (detected at line 1) (line 1)"
```
/answer 33a1-33 allow
Control Request:
```yaml
id: 33a1-34
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_yaml.py
    source: |
      """YAML tree-sitter engine: leaves collapsed to a size budget instead of the full grammar depth.

      Tree-sitter's own YAML tree exposes every scalar, flow wrapper ('flow_node',
      'block_node', ...) and punctuation token as its own node, which is far too
      fine-grained to address individually on large files. This engine instead
      rebuilds the tree from 'block_mapping_pair' / 'flow_pair' / 'block_sequence_item'
      units - transparently unwrapping the purely structural 'document', 'block_node',
      'flow_node', 'block_mapping', 'block_sequence', 'flow_mapping' and
      'flow_sequence' containers in between (see :func:`_yaml_units`) - and collapses
      each unit into a childless leaf as soon as its full text fits
      :data:`_YAML_LEAF_LIMIT`, only recursing into its own nested units when it
      doesn't (see :func:`_yaml_collapse`).
      """

      from __future__ import annotations

      from typing import Any

      from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
      from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder

      __all__ = ['YamlEngine']

      "#: Max size (characters) a collapsed leaf's full text may still have."
      _YAML_LEAF_LIMIT = 1500
      "#: Node types individually addressable - a mapping entry, a flow-mapping entry"
      "#: or a sequence item; everything else is either a leaf's inner detail or one"
      "#: of the wrapper types below."
      _YAML_UNIT_TYPES = {'block_mapping_pair', 'flow_pair', 'block_sequence_item'}
      "#: Purely structural nodes, transparently unwrapped on the way to the next unit."
      _YAML_WRAPPER_TYPES = {
          'document',
          'block_node',
          'flow_node',
          'block_mapping',
          'block_sequence',
          'flow_mapping',
          'flow_sequence'}
      _YAML_ADDRESSABLE_TYPES = _YAML_UNIT_TYPES | {'document'}


      def _yaml_units(node: Any) -> list[Any]:
          """Direct logical children of ``node``: its own unit-type descendants, found by
          transparently unwrapping wrapper nodes (comments/anchors/punctuation in between are dropped)."""
          units: list[Any] = []
          for child in node.named_children:
              if child.type in _YAML_UNIT_TYPES:
                  units.append(child)
              elif child.type in _YAML_WRAPPER_TYPES:
                  units.extend(_yaml_units(child))
          return units


      class _YamlUnit:
          """One collapsed YAML unit: either a childless leaf (its full text already fits
          the leaf budget, or it has no nested unit to expand into anyway) or a branch
          whose children are its own recursively collapsed sub-units."""
          __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_node', '_source')

          def __init__(self, node: Any, children: list[Any], source: bytes) -> None:
              self.type = node.type
              self.named_children = children
              self.start_byte = node.start_byte
              self.end_byte = node.end_byte
              self.start_point = node.start_point
              self.end_point = node.end_point
              self._node = node
              self._source = source

          @property
          def text(self) -> bytes:
              return self._source[self.start_byte:self.end_byte]

          def child_by_field_name(self, field: str) -> Any:
              """Delegate to the wrapped native node, so key/value based naming keeps working."""
              return self._node.child_by_field_name(field)


      def _yaml_collapse(node: Any, source: bytes) -> _YamlUnit:
          """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
          nested unit to expand into), else into a branch over its own collapsed sub-units."""
          sub_units = _yaml_units(node)
          size = len(node.text.decode('utf-8', 'replace'))
          children = [] if size <= _YAML_LEAF_LIMIT or not sub_units else [_yaml_collapse(unit, source) for unit in sub_units]
          return _YamlUnit(node, children, source)


      def _yaml_root_children(root_node: Any, source: bytes) -> list[Any]:
          """Top-level children of a YAML file: one leaf/branch per unit of the (single)
          document, or one per '---'-separated document in a multi-document stream."""
          documents = [child for child in root_node.named_children if child.type == 'document']
          if len(documents) > 1:
              return [_yaml_collapse(document, source) for document in documents]
          if not documents:
              return []
          units = _yaml_units(documents[0])
          if units:
              return [_yaml_collapse(unit, source) for unit in units]
          return [_yaml_collapse(documents[0], source)] if documents[0].named_children else []


      class YamlEngine(TreeSitterEngine):
          """Tree-sitter YAML restructured into the size-collapsed leaves above."""

          def __init__(self) -> None:
              super().__init__('yaml')

          def locate_all(self, tree: Tree) -> list[Located]:
              root = _RootHolder(_yaml_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
              results: list[Located] = []

              def walk(node: Any, path: str, depth: int) -> None:
                  used: dict[str, int] = {}
                  for index, child in enumerate(node.named_children):
                      if depth > 0 and child.type not in _YAML_ADDRESSABLE_TYPES:
                          continue
                      name = self._name(child)
                      # Sequence items have no natural name: a stable positional index is
                      # preferable to a content hash, which would move on every edit elsewhere.
                      content = None if child.type == 'block_sequence_item' else self.node_code(child)
                      seg = id_segment(name, index, used, content=content)
                      nid = f'{path}.{seg}' if path else seg
                      results.append(
                          Located(
                              tree=tree,
                              node=child,
                              parent=node,
                              index=index,
                              node_id=nid,
                              node_type=child.type,
                              name=name,
                              lineno=child.start_point[0] + 1,
                              end_lineno=child.end_point[0] + 1,
                              parent_type=node.type,
                              expandable=bool(child.named_children)))
                      walk(child, nid, depth + 1)
              walk(root, '', 0)
              return results
    overwrite: true
  reason: Neue YAML-Engine-Datei mit korrekt terminierten Docstrings (Triple-Quotes) anlegen.
```
/answer 33a1-34 allow
Control Request:
```yaml
id: 33a1-35
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: CjFjhH
    new_text: |-
      from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
      from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
      from xy.ai.mcpc.tools.ast.generic._yaml import YamlEngine
    old_text: |-
      from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
      from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: MaD68b
    new_text: |-
      __all__ = [
          'TreeSitterEngine',
          'JavaEngine',
          'MarkdownEngine',
          'PlainTextEngine',
          'YamlEngine',
          'language_for_extension',
          'get_engine',
          'fallback_engine']
    old_text: |-
      __all__ = [
          'TreeSitterEngine',
          'JavaEngine',
          'MarkdownEngine',
          'PlainTextEngine',
          'language_for_extension',
          'get_engine',
          'fallback_engine']
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: 4zaPLB
    new_text: "_ENGINE_CLASSES: dict[str, type[TreeSitterEngine]] = {'markdown': MarkdownEngine, 'java': JavaEngine, 'yaml': YamlEngine}"
    old_text: "_ENGINE_CLASSES: dict[str, type[TreeSitterEngine]] = {'markdown': MarkdownEngine, 'java': JavaEngine}"
  reason: "YamlEngine importieren, exportieren und für das Symbol 'yaml' registrieren."
```
/answer 33a1-35 allow
Control Request:
```yaml
id: 33a1-36
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_yaml.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: Syntaxprüfung der neuen und geänderten Dateien.
```
/answer 33a1-36 allow
Control Request:
```yaml
id: 33a1-37
toolName: python
arguments:
  script: |2

    import sys
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.generic import get_engine

    engine = get_engine('yaml')

    small = """
    a: 1
    b:
      c: 2
      d:
        - x
        - y
        - name: three
          value: 3
    list:
      - name: one
        value: 1
      - name: two
        value: 2
    """

    tree = engine.parse(small)
    for loc in engine.locate_all(tree):
        print(loc.node_id, loc.node_type, loc.name, loc.expandable, loc.lineno, loc.end_lineno)
    """large test"""
    big_item = "  - name: item%d\n    value: %d\n    description: %s\n"
    big = "items:\n" + "".join(big_item % (i, i, "x"*40) for i in range(200))
    tree2 = engine.parse(big)
    locs2 = engine.locate_all(tree2)
    print("count", len(locs2))
    for loc in locs2[:6]:
        print(loc.node_id, loc.node_type, loc.name, loc.expandable, len(engine.node_code(loc.node)))
  reason: "Neue YamlEngine funktional testen: IDs, Namen, Expandable-Flags und Kollabierungsverhalten bei großen Dateien prüfen."
```
/answer 33a1-37 allow
ReasoningToken: 1844
Control Request:
```yaml
id: 33a1-38
result:
  structuredContent:
    exit_code: 0
    stdout: |
      a block_mapping_pair a False 2 2
      b block_mapping_pair b False 3 9
      list block_mapping_pair list False 10 15
      count 201
      items block_mapping_pair items True 17987
      items.0 block_sequence_item None False 84
      items.1 block_sequence_item None False 84
      items.2 block_sequence_item None False 84
      items.3 block_sequence_item None False 84
      items.4 block_sequence_item None False 84
```
/answer 33a1-38 allow
Control Request:
```yaml
id: 33a1-39
toolName: python
arguments:
  script: |2

    import sys
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.generic import get_engine
    engine = get_engine('yaml')

    huge_field = "x"*2000
    src = f"""
    items:
      - name: bigitem
        blob: "{huge_field}"
        nested:
          k1: v1
          k2: v2
    """
    tree = engine.parse(src)
    for loc in engine.locate_all(tree):
        print(loc.node_id, loc.node_type, loc.name, loc.expandable)

    multidoc = "a: 1\n---\nb: 2\n---\nc: 3\n"
    tree2 = engine.parse(multidoc)
    for loc in engine.locate_all(tree2):
        print(loc.node_id, loc.node_type, loc.name)
  reason: Verzweigungsverhalten (Branch statt Leaf) bei großen Items und Multi-Dokument-Streams testen.
```
/answer 33a1-39 allow
Control Request:
```yaml
id: 33a1-3a
result:
  structuredContent:
    exit_code: 0
    stdout: |
      items block_mapping_pair items True
      items.0 block_sequence_item None True
      items.0.name block_mapping_pair name False
      items.0.blob block_mapping_pair blob False
      items.0.nested block_mapping_pair nested False
      8UcpoZ document None
      21TWLD document None
      EnjvNX document None
```
/answer 33a1-3a allow
ReasoningToken: 362
Text:
 Nun die Registrierung in `__init__.py` ergänzen:
Text:
 Jetzt validieren und funktional testen:
ReasoningToken: 140
ReasoningToken: 177
Result Stats:  id=f3f81522-d74d-487a-a252-76a2f87a9ac6, total: 0, in: 175, out: 38232, reason: 0, read: 691864, write: 60373
Funktioniert wie erwartet: große Items werden zu Branches (`items.0` expandiert in `name`/`blob`/`nested`), Multi-Dokument-Streams adressieren jedes `document` separat. Die neue `YamlEngine` ist implementiert und registriert.
