Füge eine für XML optimierte Engine analog der HTML Engine hinzu.

`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py`
SystemInit:  id=7a5dc88d-01de-41d6-9552-9de1ca97ef50, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: eefa-10
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
    ids:
    - __root__
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - __root__
  reason: "Bestehende HTML-Engine und Modul-Init analysieren, um analoge XML-Engine zu erstellen."
```
/answer eefa-10 allow
Control Request:
```yaml
id: eefa-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
      nodes: []
      errors:
      - No node matched '__root__'.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes: []
      errors:
      - No node matched '__root__'.
```
/answer eefa-11 allow
ReasoningToken: 12
Control Request:
```yaml
id: eefa-12
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: "Struktur der Dateien ermitteln, um passende Node-IDs für ast_read zu finden."
```
/answer eefa-12 allow
Control Request:
```yaml
id: eefa-13
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
      nodes:
      - id: y8mVTk|6CKDFw
        signature: "'HTML tree-sitter engine: elements collapsed to a size budget, mirroring\\n:mod:…"
      - id: jBljOR|qkfu8N
        signature: from __future__ import annotations
      - id: gZoS4T|gb4FvV
        signature: "__all__ = ['HtmlEngine']"
      - id: html_units
        signature: "def _html_units(node: Any) -> list[Any]:"
      - id: HtmlUnit
        signature: "class _HtmlUnit:"
        docstring: "One collapsed HTML unit: either a childless leaf (its full text already fits th…"
        children:
        - id: HtmlUnit.WvyMBm|jsiKiz
          signature: "'One collapsed HTML unit: either a childless leaf (its full text already fits\\n…"
        - id: HtmlUnit.init
          signature: "def __init__(self, node: Any, children: list[Any], source: bytes) -> None:"
        - id: HtmlUnit.text
          signature: "@property def text(self) -> bytes:"
        - id: HtmlUnit.start_tag
          signature: "@property def start_tag(self) -> Any | None:"
          docstring: "The wrapped native node's own 'start_tag' child (tag name + attributes), looked…"
      - id: html_collapse
        signature: "def _html_collapse(node: Any, source: bytes) -> _HtmlUnit:"
        docstring: Collapse ``node`` into a leaf if its full text fits the budget (or it has no ne…
      - id: html_root_children
        signature: "def _html_root_children(root_node: Any, source: bytes) -> list[Any]:"
        docstring: "Top-level children of an HTML file: the doctype plus the root element(s)."
      - id: html_attr
        signature: "def _html_attr(start_tag: Any, attr_name: str) -> str | None:"
        docstring: "Value of ``attr_name`` on ``start_tag``, or ``None`` if absent/valueless."
      - id: HtmlEngine
        signature: "class HtmlEngine(TreeSitterEngine):"
        docstring: Tree-sitter HTML restructured into the size-collapsed leaves above.
        children:
        - id: HtmlEngine.p6ekgQ|XcU3kQ
          signature: '''Tree-sitter HTML restructured into the size-collapsed leaves above.'''
        - id: HtmlEngine.init
          signature: "def __init__(self) -> None:"
        - id: HtmlEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: HtmlEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: HtmlEngine.html_name
          signature: "def _html_name(self, node: Any) -> str | None:"
        - id: HtmlEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: Nna9mi|guMKQx
        signature: '"Generic tree-sitter back-ends for every non-Python language/format.\n\nSplit i…'
      - id: LvTUnt|47Sn4S
        signature: from __future__ import annotations
      - id: 95AfuA|X87q5l
        signature: from xy.ai.mcpc.tools.ast.generic._yaml import YamlEngine
      - id: 9C41E7|tM8KNW
        signature: "__all__ = ["
      - id: acL0qu|XdqYqL
        signature: "EXT_LANGUAGE = {"
      - id: language_for_extension
        signature: "def language_for_extension(ext: str) -> str | None:"
      - id: FnS4Dz|ZBavT9
        signature: "'#: Language symbol -> dedicated Engine subclass; anything absent here falls'"
      - id: BPw90W|PZ1iMb
        signature: "_ENGINE_CLASSES: dict[str,"
      - id: get_engine
        signature: "def get_engine(symbol: str) -> TreeSitterEngine:"
      - id: 5P6pDx|byGZAx
        signature: _FALLBACK_ENGINE = PlainTextEngine()
      - id: fallback_engine
        signature: "def fallback_engine() -> PlainTextEngine:"
        docstring: Whole-file engine used when no tree-sitter grammar claims the extension.
```
/answer eefa-13 allow
Control Request:
```yaml
id: eefa-14
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
    ids:
    - y8mVTk|6CKDFw
    - jBljOR|qkfu8N
    - gZoS4T|gb4FvV
    - html_units
    - HtmlUnit
    - html_collapse
    - html_root_children
    - html_attr
    - HtmlEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - Nna9mi|guMKQx
    - LvTUnt|47Sn4S
    - 95AfuA|X87q5l
    - 9C41E7|tM8KNW
    - acL0qu|XdqYqL
    - language_for_extension
    - FnS4Dz|ZBavT9
    - BPw90W|PZ1iMb
    - get_engine
    - 5P6pDx|byGZAx
    - fallback_engine
  reason: Volltext von HTML-Engine und generic __init__ lesen für exakten Nachbau der XML-Engine.
```
/answer eefa-14 allow
Control Request:
```yaml
id: eefa-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
      nodes:
      - id: y8mVTk|6CKDFw
        type: statements
        code: "'HTML tree-sitter engine: elements collapsed to a size budget, mirroring\\n:mod:`xy.ai.mcpc.tools.ast.generic._yaml` - unlike Java\\'s fixed def-node types,\\nan HTML document\\'s addressable unit (an \\'element\\') nests to arbitrary, content-\\ndependent depth (a \\'<li>\\' vs. a \\'<body>\\' wrapping a whole page), so there is\\nno fixed \"definition\" level to cut at. Every element/script/style/comment/\\ndoctype collapses into a childless leaf (its full text already fits\\n:data:`_HTML_LEAF_LIMIT`, see :func:`_html_collapse`) unless still too large,\\nin which case it becomes a branch over its own (recursively collapsed) child\\nelements - attributes/text content never get their own node, they\\'re too fine-\\ngrained to be independently useful.\\n'"
      - id: jBljOR|qkfu8N
        type: imports
        code: |-
          from __future__ import annotations
          from typing import Any
          from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
          from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder
      - id: gZoS4T|gb4FvV
        type: statements
        code: |-
          __all__ = ['HtmlEngine']
          "#: Max size (characters) a collapsed leaf's full text may still have."
          _HTML_LEAF_LIMIT = 1500
          '#: Individually addressable node types - everything with its own start/end tag,'
          '#: plus comments/doctype, which are already always leaves.'
          _HTML_UNIT_TYPES = {'element', 'script_element', 'style_element', 'comment', 'doctype'}
      - id: html_units
        type: FunctionDef
        code: |-
          def _html_units(node: Any) -> list[Any]:
              return [child for child in node.named_children if child.type in _HTML_UNIT_TYPES]
      - id: HtmlUnit
        type: ClassDef
        signature: "class _HtmlUnit:"
        docstring: "One collapsed HTML unit: either a childless leaf (its full text already fits th…"
        children:
        - id: HtmlUnit.WvyMBm|jsiKiz
          type: statements
          code: |-
            'One collapsed HTML unit: either a childless leaf (its full text already fits\n    the leaf budget, or it has no nested unit to expand into anyway) or a branch\n    whose children are its own recursively collapsed sub-units.'
            __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_node', '_source')
        - id: HtmlUnit.init
          type: FunctionDef
          code: |-
            def __init__(self, node: Any, children: list[Any], source: bytes) -> None:
                self.type = node.type
                self.named_children = children
                self.start_byte = node.start_byte
                self.end_byte = node.end_byte
                self.start_point = node.start_point
                self.end_point = node.end_point
                self._node = node
                self._source = source
        - id: HtmlUnit.text
          type: FunctionDef
          code: |-
            @property
            def text(self) -> bytes:
                return self._source[self.start_byte:self.end_byte]
        - id: HtmlUnit.start_tag
          type: FunctionDef
          code: |-
            @property
            def start_tag(self) -> Any | None:
                """The wrapped native node's own 'start_tag' child (tag name + attributes),
                    looked up on the *native* tree - never among the collapsed unit children,
                    which only ever hold nested elements."""
                return next((c for c in self._node.named_children if c.type == 'start_tag'), None)
      - id: html_collapse
        type: FunctionDef
        code: |-
          def _html_collapse(node: Any, source: bytes) -> _HtmlUnit:
              """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
              nested unit to expand into), else into a branch over its own collapsed sub-units."""
              sub_units = _html_units(node)
              size = len(node.text.decode('utf-8', 'replace'))
              children = [] if size <= _HTML_LEAF_LIMIT or not sub_units else [_html_collapse(unit, source) for unit in sub_units]
              return _HtmlUnit(node, children, source)
      - id: html_root_children
        type: FunctionDef
        code: |-
          def _html_root_children(root_node: Any, source: bytes) -> list[Any]:
              """Top-level children of an HTML file: the doctype plus the root element(s)."""
              return [_html_collapse(unit, source) for unit in _html_units(root_node)]
      - id: html_attr
        type: FunctionDef
        code: |-
          def _html_attr(start_tag: Any, attr_name: str) -> str | None:
              """Value of ``attr_name`` on ``start_tag``, or ``None`` if absent/valueless."""
              for attr in start_tag.named_children:
                  if attr.type != 'attribute':
                      continue
                  name_node = next((c for c in attr.named_children if c.type == 'attribute_name'), None)
                  if name_node is None or name_node.text.decode('utf-8', 'replace') != attr_name:
                      continue
                  value_node = next((c for c in attr.named_children if c.type in (
                      'quoted_attribute_value', 'attribute_value')), None)
                  if value_node is not None and value_node.type == 'quoted_attribute_value':
                      value_node = next((c for c in value_node.named_children if c.type == 'attribute_value'), None)
                  return value_node.text.decode('utf-8', 'replace').strip() if value_node is not None else ''
              return None
      - id: HtmlEngine
        type: ClassDef
        signature: "class HtmlEngine(TreeSitterEngine):"
        docstring: Tree-sitter HTML restructured into the size-collapsed leaves above.
        children:
        - id: HtmlEngine.p6ekgQ|XcU3kQ
          type: statements
          code: '''Tree-sitter HTML restructured into the size-collapsed leaves above.'''
        - id: HtmlEngine.init
          type: FunctionDef
          code: |-
            def __init__(self) -> None:
                super().__init__('html')
        - id: HtmlEngine.is_definition
          type: FunctionDef
          code: |-
            def is_definition(self, node_type: str) -> bool:
                return node_type in _HTML_UNIT_TYPES
        - id: HtmlEngine.signature
          type: FunctionDef
          code: |-
            def signature(self, node: Any, limit: int=80) -> str:
                start_tag = node.start_tag if isinstance(node, _HtmlUnit) else None
                raw = start_tag.text if start_tag is not None else node.text
                text = ' '.join(raw.decode('utf-8', 'replace').split())
                return text if len(text) <= limit else text[:limit - 1] + '…'
        - id: HtmlEngine.html_name
          type: FunctionDef
          code: |-
            def _html_name(self, node: Any) -> str | None:
                start_tag = node.start_tag if isinstance(node, _HtmlUnit) else None
                if start_tag is None:
                    return None
                tag_node = next((c for c in start_tag.named_children if c.type == 'tag_name'), None)
                if tag_node is None:
                    return None
                tag = tag_node.text.decode('utf-8', 'replace')
                ident = _html_attr(start_tag, 'id')
                if ident:
                    return f'{tag}#{ident}'
                css_class = _html_attr(start_tag, 'class')
                if css_class:
                    return f'{tag}.{css_class.split()[0]}'
                return tag
        - id: HtmlEngine.locate_all
          type: FunctionDef
          code: |-
            def locate_all(self, tree: Tree) -> list[Located]:
                root = _RootHolder(_html_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
                results: list[Located] = []

                def walk(node: Any, path: str, depth: int) -> None:
                    used: dict[str, int] = {}
                    for index, child in enumerate(node.named_children):
                        name = self._html_name(child)
                        seg = id_segment(name, index, used, content=self.node_code(child))
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
                                expandable=bool(
                                    child.named_children)))
                        walk(child, nid, depth + 1)
                walk(root, '', 0)
                return results
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: Nna9mi|guMKQx
        type: statements
        code: "\"Generic tree-sitter back-ends for every non-Python language/format.\\n\\nSplit into :mod:`xy.ai.mcpc.tools.ast.generic._engine` (the universal\\n:class:`TreeSitterEngine`, exposing a grammar's native structure as-is) and\\nper-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`\\n(:class:`MarkdownEngine`); this module re-exports the package's public\\nsurface, dispatches a language symbol to its engine, and hands out the\\nwhole-file :class:`~xy.ai.mcpc.tools.ast.generic._text.PlainTextEngine`\\nfallback (via :func:`fallback_engine`) for extensions no grammar covers.\\n\""
      - id: LvTUnt|47Sn4S
        type: imports
        code: |-
          from __future__ import annotations
          from xy.ai.mcpc.tools.ast.generic._css import CssEngine
          from xy.ai.mcpc.tools.ast.generic._engine import GenericEngine, TreeSitterEngine
          from xy.ai.mcpc.tools.ast.generic._html import HtmlEngine
          from xy.ai.mcpc.tools.ast.generic._java import JavaEngine
          from xy.ai.mcpc.tools.ast.generic._javascript import JavaScriptEngine
          from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
          from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
      - id: 95AfuA|X87q5l
        type: imports
        code: from xy.ai.mcpc.tools.ast.generic._yaml import YamlEngine
      - id: 9C41E7|tM8KNW
        type: statements
        code: |-
          __all__ = [
              'TreeSitterEngine',
              'GenericEngine',
              'CssEngine',
              'HtmlEngine',
              'JavaEngine',
              'JavaScriptEngine',
              'MarkdownEngine',
              'PlainTextEngine',
              'YamlEngine',
              'language_for_extension',
              'get_engine',
              'fallback_engine']
          '#: File extension -> ``tree_sitter_language_pack`` language identifier.'
      - id: acL0qu|XdqYqL
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
      - id: FnS4Dz|ZBavT9
        type: statements
        code: |-
          '#: Language symbol -> dedicated Engine subclass; anything absent here falls'
          '#: back to the universal :class:`TreeSitterEngine`.'
      - id: BPw90W|PZ1iMb
        type: statements
        code: |-
          _ENGINE_CLASSES: dict[str,
                                type[TreeSitterEngine]] = {'markdown': MarkdownEngine,
                                                           'java': JavaEngine,
                                                           'yaml': YamlEngine,
                                                           'css': CssEngine,
                                                           'html': HtmlEngine,
                                                           'javascript': JavaScriptEngine}
          _ENGINES: dict[str, TreeSitterEngine] = {}
      - id: get_engine
        type: FunctionDef
        code: |-
          def get_engine(symbol: str) -> TreeSitterEngine:
              engine = _ENGINES.get(symbol)
              if engine is None:
                  engine_cls = _ENGINE_CLASSES.get(symbol)
                  engine = engine_cls() if engine_cls else GenericEngine(symbol)
                  _ENGINES[symbol] = engine
              return engine
      - id: 5P6pDx|byGZAx
        type: statements
        code: _FALLBACK_ENGINE = PlainTextEngine()
      - id: fallback_engine
        type: FunctionDef
        code: |-
          def fallback_engine() -> PlainTextEngine:
              """Whole-file engine used when no tree-sitter grammar claims the extension."""
              return _FALLBACK_ENGINE
```
/answer eefa-15 allow
Control Request:
```yaml
id: eefa-16
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('xml')
    src = b"""<?xml version="1.0"?>
    <!DOCTYPE root>
    <!-- comment -->
    <root id="1" class="a b">
      <child attr='x'>text</child>
      <self-closing/>
      <![CDATA[ some data ]]>
    </root>
    """
    tree = parser.parse(src)

    def dump(node, depth=0):
        print('  '*depth, node.type, node.start_byte, node.end_byte)
        for c in node.children:
            dump(c, depth+1)

    dump(tree.root_node)
  reason: "Node-Typen der tree-sitter XML-Grammatik untersuchen, um die Engine analog zu HTML zu bauen."
```
/answer eefa-16 allow
Control Request:
```yaml
id: eefa-17
result:
  structuredContent:
    exit_code: 0
    stdout: |2
       document 0 164
         prolog 0 55
           XMLDecl 0 21
             <? 0 2
             xml 2 5
             version 6 13
             = 13 14
             " 14 15
             VersionNum 15 18
             " 18 19
             ?> 19 21
           doctypedecl 22 37
             <! 22 24
             DOCTYPE 24 31
             Name 32 36
             > 36 37
           Comment 38 54
         element 55 163
           STag 55 80
             < 55 56
             Name 56 60
             Attribute 61 67
               Name 61 63
               = 63 64
               AttValue 64 67
                 " 64 65
                 " 66 67
             Attribute 68 79
               Name 68 73
               = 73 74
               AttValue 74 79
                 " 74 75
                 " 78 79
             > 79 80
           content 80 156
             CharData 80 83
             element 83 111
               STag 83 99
                 < 83 84
                 Name 84 89
                 Attribute 90 98
                   Name 90 94
                   = 94 95
                   AttValue 95 98
                     ' 95 96
                     ' 97 98
                 > 98 99
               content 99 103
                 CharData 99 103
               ETag 103 111
                 </ 103 105
                 Name 105 110
                 > 110 111
             CharData 111 114
             element 114 129
               EmptyElemTag 114 129
                 < 114 115
                 Name 115 127
                 /> 127 129
             CharData 129 132
             CDSect 132 155
               CDStart 132 141
                 <![ 132 135
                 CDATA 135 140
                 [ 140 141
               CData 141 152
               ]]> 152 155
             CharData 155 156
           ETag 156 163
             </ 156 158
             Name 158 162
             > 162 163
```
/answer eefa-17 allow
ReasoningToken: 223
Control Request:
```yaml
id: eefa-18
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('xml')
    src = b'<root id="1" class="a b"><child>text</child></root>'
    tree = parser.parse(src)

    def dump(node, depth=0):
        print('  '*depth, node.type, node.is_named, repr(node.text))
        for c in node.children:
            dump(c, depth+1)

    dump(tree.root_node)
    print('---named only---')
    def dumpn(node, depth=0):
        print('  '*depth, node.type, repr(node.text))
        for c in node.named_children:
            dumpn(c, depth+1)
    dumpn(tree.root_node)
  reason: Named vs. unnamed Kindnoten und Attributwert-Text der XML-Grammatik prüfen.
```
/answer eefa-18 allow
Control Request:
```yaml
id: eefa-19
result:
  structuredContent:
    exit_code: 0
    stdout: |2
       document True b'<root id="1" class="a b"><child>text</child></root>'
         element True b'<root id="1" class="a b"><child>text</child></root>'
           STag True b'<root id="1" class="a b">'
             < False b'<'
             Name True b'root'
             Attribute True b'id="1"'
               Name True b'id'
               = False b'='
               AttValue True b'"1"'
                 " False b'"'
                 " False b'"'
             Attribute True b'class="a b"'
               Name True b'class'
               = False b'='
               AttValue True b'"a b"'
                 " False b'"'
                 " False b'"'
             > False b'>'
           content True b'<child>text</child>'
             element True b'<child>text</child>'
               STag True b'<child>'
                 < False b'<'
                 Name True b'child'
                 > False b'>'
               content True b'text'
                 CharData True b'text'
               ETag True b'</child>'
                 </ False b'</'
                 Name True b'child'
                 > False b'>'
           ETag True b'</root>'
             </ False b'</'
             Name True b'root'
             > False b'>'
      ---named only---
       document b'<root id="1" class="a b"><child>text</child></root>'
         element b'<root id="1" class="a b"><child>text</child></root>'
           STag b'<root id="1" class="a b">'
             Name b'root'
             Attribute b'id="1"'
               Name b'id'
               AttValue b'"1"'
             Attribute b'class="a b"'
               Name b'class'
               AttValue b'"a b"'
           content b'<child>text</child>'
             element b'<child>text</child>'
               STag b'<child>'
                 Name b'child'
               content b'text'
                 CharData b'text'
               ETag b'</child>'
                 Name b'child'
           ETag b'</root>'
             Name b'root'
```
/answer eefa-19 allow
ReasoningToken: 380
Control Request:
```yaml
id: eefa-1a
toolName: python
arguments:
  script: |2

    from tree_sitter_language_pack import get_parser
    parser = get_parser('xml')
    src = b"""<?xml version="1.0"?>
    <!DOCTYPE root>
    <!-- c1 -->
    <?target data?>
    <root>
      <!-- inner comment -->
      <?pi inner?>
      <![CDATA[cd]]>
      <child/>
    </root>
    <!-- trailing -->
    """
    tree = parser.parse(src)
    def dumpn(node, depth=0):
        print('  '*depth, node.type)
        for c in node.named_children:
            dumpn(c, depth+1)
    dumpn(tree.root_node)
  reason: "Struktur von document/prolog/Misc in der XML-Grammatik prüfen, um Root-Kinder korrekt zu sammeln."
```
/answer eefa-1a allow
Control Request:
```yaml
id: eefa-1b
result:
  structuredContent:
    exit_code: 0
    stdout: |2
       document
         prolog
           XMLDecl
             VersionNum
           doctypedecl
             Name
           Comment
           PI
             PITarget
         element
           STag
             Name
           content
             CharData
             Comment
             CharData
             PI
               PITarget
             CharData
             CDSect
               CDStart
               CData
             CharData
             element
               EmptyElemTag
                 Name
             CharData
           ETag
             Name
         Comment
```
/answer eefa-1b allow
ReasoningToken: 317
Control Request:
```yaml
id: eefa-1c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: def id_segment
    include:
    - '*.py'
  reason: Fallback-Namensvergabe für Knoten ohne Namen verstehen.
```
/answer eefa-1c allow
Control Request:
```yaml
id: eefa-1d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - lineno: 179
          match: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
```
/answer eefa-1d allow
ReasoningToken: 1449
Text:
 Fallback-Naming ist bereits generisch (nutzt content-Hash o.ä.), das reicht. Jetzt erstelle ich die XML-Engine analog zur HTML-Engine.
Control Request:
```yaml
id: eefa-1e
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_xml.py
    source: |
      """XML tree-sitter engine: elements collapsed to a size budget, mirroring
      :mod:`xy.ai.mcpc.tools.ast.generic._html` - unlike HTML the grammar wraps an
      element's body in an anonymous 'content' node and a document's prolog in an
      anonymous 'prolog' node, so :func:`_xml_units` drills through those container
      types to find the actual addressable units (elements, comments, processing
      instructions, CDATA sections, the doctype declaration). Every unit collapses
      into a childless leaf (its full text already fits :data:`_XML_LEAF_LIMIT`, see
      :func:`_xml_collapse`) unless still too large, in which case it becomes a
      branch over its own (recursively collapsed) child units - attributes/text
      content never get their own node, they're too fine-grained to be
      independently useful.
      """
      from __future__ import annotations

      from typing import Any

      from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
      from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder

      __all__ = ['XmlEngine']

      "#: Max size (characters) a collapsed leaf's full text may still have."
      _XML_LEAF_LIMIT = 1500
      "#: Individually addressable node types."
      _XML_UNIT_TYPES = {'element', 'Comment', 'CDSect', 'PI', 'doctypedecl'}
      "#: Anonymous wrapper types the grammar interposes between a node and its"
      "#: actual unit children - drilled through, never addressable themselves."
      _XML_CONTAINER_TYPES = {'document', 'prolog', 'content'}


      def _xml_units(node: Any) -> list[Any]:
          units = []
          for child in node.named_children:
              if child.type in _XML_UNIT_TYPES:
                  units.append(child)
              elif child.type in _XML_CONTAINER_TYPES:
                  units.extend(_xml_units(child))
          return units


      class _XmlUnit:
          """One collapsed XML unit: either a childless leaf (its full text already fits
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

          @property
          def start_tag(self) -> Any | None:
              """The wrapped native node's own 'STag'/'EmptyElemTag' child, looked up on
              the *native* tree - never among the collapsed unit children, which only
              ever hold nested elements."""
              return next((c for c in self._node.named_children if c.type in ('STag', 'EmptyElemTag')), None)


      def _xml_collapse(node: Any, source: bytes) -> _XmlUnit:
          """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
          nested unit to expand into), else into a branch over its own collapsed sub-units."""
          sub_units = _xml_units(node)
          size = len(node.text.decode('utf-8', 'replace'))
          children = [] if size <= _XML_LEAF_LIMIT or not sub_units else [_xml_collapse(unit, source) for unit in sub_units]
          return _XmlUnit(node, children, source)


      def _xml_root_children(root_node: Any, source: bytes) -> list[Any]:
          """Top-level children of an XML file: prolog declarations (doctype, comments,
          processing instructions), the root element, and any trailing misc."""
          return [_xml_collapse(unit, source) for unit in _xml_units(root_node)]


      def _xml_attr(start_tag: Any, attr_name: str) -> str | None:
          """Value of ``attr_name`` on ``start_tag``, or ``None`` if absent."""
          for attr in start_tag.named_children:
              if attr.type != 'Attribute':
                  continue
              name_node = next((c for c in attr.named_children if c.type == 'Name'), None)
              if name_node is None or name_node.text.decode('utf-8', 'replace') != attr_name:
                  continue
              value_node = next((c for c in attr.named_children if c.type == 'AttValue'), None)
              if value_node is None:
                  return ''
              return value_node.text.decode('utf-8', 'replace')[1:-1].strip()
          return None


      class XmlEngine(TreeSitterEngine):
          """Tree-sitter XML restructured into the size-collapsed leaves above."""

          def __init__(self) -> None:
              super().__init__('xml')

          def is_definition(self, node_type: str) -> bool:
              return node_type in _XML_UNIT_TYPES

          def signature(self, node: Any, limit: int = 80) -> str:
              start_tag = node.start_tag if isinstance(node, _XmlUnit) else None
              raw = start_tag.text if start_tag is not None else node.text
              text = ' '.join(raw.decode('utf-8', 'replace').split())
              return text if len(text) <= limit else text[:limit - 1] + '…'

          def _xml_name(self, node: Any) -> str | None:
              start_tag = node.start_tag if isinstance(node, _XmlUnit) else None
              if start_tag is None:
                  return None
              name_node = next((c for c in start_tag.named_children if c.type == 'Name'), None)
              if name_node is None:
                  return None
              tag = name_node.text.decode('utf-8', 'replace')
              ident = _xml_attr(start_tag, 'id')
              return f'{tag}#{ident}' if ident else tag

          def locate_all(self, tree: Tree) -> list[Located]:
              root = _RootHolder(_xml_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
              results: list[Located] = []

              def walk(node: Any, path: str, depth: int) -> None:
                  used: dict[str, int] = {}
                  for index, child in enumerate(node.named_children):
                      name = self._xml_name(child)
                      seg = id_segment(name, index, used, content=self.node_code(child))
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
                              expandable=bool(
                                  child.named_children)))
                      walk(child, nid, depth + 1)
              walk(root, '', 0)
              return results
  reason: Neue XML-Engine analog zur HTML-Engine anlegen.
```
/answer eefa-1e allow
Control Request:
```yaml
id: eefa-1f
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: LvTUnt|47Sn4S
    old_text: from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
    new_text: |-
      from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
      from xy.ai.mcpc.tools.ast.generic._xml import XmlEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: 9C41E7|tM8KNW
    old_text: |2-
          'PlainTextEngine',
          'YamlEngine',
    new_text: |2-
          'PlainTextEngine',
          'XmlEngine',
          'YamlEngine',
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: BPw90W|PZ1iMb
    old_text: "                                                 'javascript': JavaScriptEngine}"
    new_text: |2-
                                                       'javascript': JavaScriptEngine,
                                                       'xml': XmlEngine}
  reason: "XmlEngine in Modul-__init__ registrieren (Import, __all__, Dispatch-Tabelle)."
```
/answer eefa-1f allow
Control Request:
```yaml
id: eefa-20
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_xml.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: Syntax der neuen/geänderten Dateien prüfen.
```
/answer eefa-20 allow
Control Request:
```yaml
id: eefa-21
toolName: python
arguments:
  script: |2

    import sys
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.generic import get_engine
    from xy.ai.mcpc.tools.ast.base import Tree

    src = """<?xml version="1.0"?>
    <!-- header comment -->
    <root id="r1">
      <child id="c1" attr="v">text</child>
      <self-closing/>
      <?proc instr?>
      <![CDATA[data]]>
    </root>
    <!-- trailing -->
    """

    engine = get_engine('xml')
    tree = engine.parse(src)
    for loc in engine.locate_all(tree):
        print(loc.node_id, loc.node_type, loc.name, engine.signature(loc.node))
    """
    """
  reason: "Funktionalen Test der neuen XmlEngine ausführen, um locate_all/signature/naming zu prüfen."
```
/answer eefa-21 allow
Control Request:
```yaml
id: eefa-22
result:
  structuredContent:
    exit_code: 0
    stdout: |
      WNAjVq|f38iAt Comment None <!-- header comment -->
      root_r1 element root#r1 <root id="r1">
      excqiV|1X47Nk Comment None <!-- trailing -->
```
/answer eefa-22 allow
Control Request:
```yaml
id: eefa-23
toolName: python
arguments:
  script: |2

    import sys
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.generic import get_engine

    pad = "x" * 2000
    src = f"""<root id="r1">
      <child id="c1" attr="v">{pad}</child>
      <self-closing/>
      <?proc instr?>
      <![CDATA[data]]>
    </root>
    """

    engine = get_engine('xml')
    tree = engine.parse(src)
    for loc in engine.locate_all(tree):
        print(loc.node_id, loc.node_type, loc.name, engine.signature(loc.node)[:60])
  reason: Verschachtelung/Aufsplitten bei großen Elementen testen.
```
/answer eefa-23 allow
Control Request:
```yaml
id: eefa-24
result:
  structuredContent:
    exit_code: 0
    stdout: |
      root_r1 element root#r1 <root id="r1">
      root_r1.child_c1 element child#c1 <child id="c1" attr="v">
      root_r1.self_closing element self-closing <self-closing/>
      root_r1.n8l2AU|4ck3nC PI None <?proc instr?>
      root_r1.EgXD4d|6h2vam CDSect None <![CDATA[data]]>
```
/answer eefa-24 allow
Result Stats: id=7a5dc88d-01de-41d6-9552-9de1ca97ef50, total: 34289, in: 157, out: 9228, reason: 78, read: 197272, write: 24904
Text:
 Jetzt die `__init__.py` anpassen: Import, `__all__`, `_ENGINE_CLASSES`.
Text:
 Nun validieren und funktional testen.
ReasoningToken: 78
Funktioniert wie erwartet – Nesting, Namensvergabe (Tag + `#id`), Fallback-IDs für unbenannte Knoten (PI, CDSect, Comment) und Collapsing analog zur HTML-Engine.
