Generiere ein Protof file in `/home/user/xyan/xy.ai.workbench/ast-engines` auf Basis des AST Engine (Engine ABC) Interface in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py`.
Ziel ist die Anbindung weiterer AST Engines in anderen Zielplattformen als Python über RPC.
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7a0a-1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids: []
  reason: Interface für Proto-Generierung analysieren
```
/answer 7a0a-1 allow
Control Request:
```yaml
id: 7a0a-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: ZjoNfB|uJIfID
        signature: '''Engine-agnostic model shared by every ``ast_*`` tool.\n\nThe tools address nod…'
      - id: QuHMWF|e2q5UN
        signature: from __future__ import annotations
      - id: AstError
        signature: "class AstError(Exception):"
        docstring: "A user-facing, path-free error raised by the AST tools."
      - id: AstAmbiguous
        signature: "class AstAmbiguous(AstError):"
        docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
        children:
        - id: AstAmbiguous.FSy0jO|jchvnz
          signature: '''Raised when a text-based node search (no id given) finds several\n    unrelate…'
        - id: AstAmbiguous.init
          signature: "def __init__(self, message: str, candidates: list[str]) -> None:"
      - id: AstTextError
        signature: "class AstTextError(AstError):"
        docstring: Raised when a text/marker-based edit's search text could not be applied. Carrie…
        children:
        - id: AstTextError.xHQ70j|YI3gjQ
          signature: '"Raised when a text/marker-based edit''s search text could not be applied.\n\n  …'
        - id: AstTextError.init
          signature: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
      - id: Tree
        signature: "@dataclass class Tree:"
        docstring: "A parsed file/snippet plus the engine that owns it. Attributes: engine: The eng…"
      - id: Located
        signature: "@dataclass class Located:"
        docstring: A node with the engine-independent metadata the selectors match on. Attributes:…
      - id: OutlineNode
        signature: "@dataclass(frozen=True) class OutlineNode:"
        docstring: "One node in a structural (list/find/read) result. ``id`` is the node's unique, …"
      - id: line_range
        signature: "def line_range(loc: Located) -> str:"
        docstring: "Return ``loc``'s start line, or a ``\"start-end\"`` range if it spans several."
      - id: 1vtTBH|EXC8hj
        signature: _ID_CLEAN_RE = re.compile('\\W+')
      - id: hash
        signature: "def _hash(name: str, length: int) -> str:"
      - id: ThMOZm|vomnxc
        signature: _ID_HASH_ALPHABET = string.digits + string.ascii_letters
      - id: base62_hash
        signature: "def _base62_hash(text: str, length: int) -> str:"
        docstring: Base62 (0-9a-zA-Z) digest of ``text``.
      - id: content_hash
        signature: "def _content_hash(content: str, length: int=6) -> str:"
        docstring: "Base62 digest of ``content``, stable across unrelated tree edits."
      - id: content_prefix_hash
        signature: "def _content_prefix_hash(content: str, length: int=6) -> str:"
        docstring: Base62 digest of ``content``'s whitespace-stripped first/last 20 chars. Forms a…
      - id: mFoC5C|Xl4C8a
        signature: _ID_SUFFIX_RE = re.compile('_\\d+$')
      - id: id_prefix
        signature: "def _id_prefix(segment: str) -> str | None:"
        docstring: "An anonymous id segment's stable prefix (before ``'|'``), if any."
      - id: resolve_by_prefix
        signature: "def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':"
        docstring: Fallback selector for a ``target_id`` that matches no node exactly. Used when a…
      - id: id_segment
        signature: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
        docstring: "Return a unique-within-siblings id segment, name-based when feasible. A clean, …"
      - id: node_outline
        signature: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, with_type: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
        docstring: "Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``,…"
      - id: compact
        signature: "def _compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: to_dict
        signature: "def to_dict(node: OutlineNode) -> dict:"
        docstring: "Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."
      - id: TreeNode
        signature: "@dataclass class _TreeNode:"
      - id: build_forest
        signature: "def _build_forest(located: list[Located]) -> list[_TreeNode]:"
        docstring: Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes.
      - id: build_outline
        signature: "def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
        docstring: "Build the nested outline of ``located`` (source only if ``with_code``, lines on…"
      - id: outline_nodes
        signature: "def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
        docstring: "Convert a forest into OutlineNodes, collapsing non-expandable nodes to full sou…"
      - id: resolve_by_name
        signature: "def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:"
        docstring: Resolve ``key`` against node names when it doesn't match an id directly. Tries …
      - id: read_subtrees
        signature: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:"
        docstring: "Return one read subtree per resolvable ``keys`` entry. Each key is matched, in …"
      - id: matches
        signature: "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
      - id: find
        signature: "def find(tree: Tree, **filters: object) -> list[Located]:"
      - id: most_specific
        signature: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
        docstring: "Return the smallest node in *located* fully containing lines [lineno, end_linen…"
      - id: Engine
        signature: "class Engine(ABC):"
        docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
        children:
        - id: Engine.yeTUw1|1zTXCW
          signature: "'A parser back-end turning source into an addressable, mutable tree.\\n\\n    Str…"
        - id: Engine.dqC7gX|bz1VNl
          signature: "'#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'"
        - id: Engine.parse
          signature: "@abstractmethod def parse(self, source: str, path: Path | None=None) -> Tree:"
          docstring: "Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."
        - id: Engine.empty_tree
          signature: "@abstractmethod def empty_tree(self, path: Path | None=None) -> Tree:"
          docstring: "Return an empty tree, used when appending to a not-yet-existing file."
        - id: Engine.serialize
          signature: "@abstractmethod def serialize(self, tree: Tree) -> str:"
          docstring: Render ``tree`` back to source text for writing to disk.
        - id: Engine.validate
          signature: "@abstractmethod def validate(self, source: str) -> str | None:"
          docstring: "Return an error message if ``source`` is malformed, else ``None``."
        - id: Engine.locate_all
          signature: "@abstractmethod def locate_all(self, tree: Tree) -> list[Located]:"
          docstring: "Flatten ``tree`` into every addressable node, in document order."
        - id: Engine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
          docstring: Whether ``node_type`` is "def-like" enough for a ``signature`` to make sense. D…
        - id: Engine.signature
          signature: "@abstractmethod def signature(self, node: Any) -> str:"
          docstring: One-line rendering of ``node``'s header (or the node itself).
        - id: Engine.default_signature
          signature: "def default_signature(self, node: Any, limit: int=80) -> str:"
          docstring: "Fallback used when ``is_definition`` is False: the first non-blank line of the …"
        - id: Engine.docstring
          signature: "@abstractmethod def docstring(self, node: Any) -> str | None:"
          docstring: "Short docstring of ``node``, if the format has such a concept."
        - id: Engine.node_code
          signature: "@abstractmethod def node_code(self, node: Any) -> str:"
          docstring: Full source of a single ``node``.
        - id: Engine.replace
          signature: "@abstractmethod def replace(self, loc: Located, code: str) -> None:"
          docstring: Replace ``loc``'s node with ``code``.
        - id: Engine.insert
          signature: "@abstractmethod def insert(self, loc: Located, code: str, position: str) -> int:"
          docstring: Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted.
        - id: Engine.delete
          signature: "@abstractmethod def delete(self, loc: Located) -> None:"
          docstring: Delete ``loc``'s node from its container.
        - id: Engine.append
          signature: "@abstractmethod def append(self, tree: Tree, code: str) -> int:"
          docstring: Append ``code`` at ``tree``'s top level; return units appended.
      - id: require_path
        signature: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
        docstring: "Validate a mandatory absolute path, raising :class:`AstError` on failure."
      - id: GaNJOk|lfM39j
        signature: "_CONTROL_CHAR_RE = re.compile('[\\x00-\\x08\\x0b\\x0c\\x0e-\\x1f\\x7f]')"
      - id: check_no_control_chars
        signature: "def check_no_control_chars(code: str) -> None:"
        docstring: Raise :class:`AstError` if ``code`` contains abnormal control characters (e.g. …
      - id: 9cdglS|xe6IXY
        signature: "'#: JSON-Schema fragment for :class:`OutlineNode`, shared by list/find.'"
      - id: pxV5LK|b4hZTj
        signature: OUTLINE_NODE_DESCRIPTION = 'The result object represents an outline node with m…
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 7a0a-2 allow
ReasoningToken: 24
Control Request:
```yaml
id: 7a0a-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - Tree
    - Located
    - OutlineNode
    - Engine
    - AstError
    - AstAmbiguous
    - AstTextError
  reason: Volle Definitionen für Proto-Design lesen
```
/answer 7a0a-3 allow
Control Request:
```yaml
id: 7a0a-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: Tree
        type: ClassDef
        code: |-
          @dataclass
          class Tree:
              """A parsed file/snippet plus the engine that owns it.

              Attributes:
                  engine: The engine that parsed ``raw`` and knows how to mutate it.
                  raw: Engine-native tree (``ast.Module`` or ``tree_sitter.Tree``).
                  source: Current source text; the single source of truth for tree-sitter
                      engines, refreshed by the Python engine only on save.
                  path: Absolute path the tree was loaded from, or ``None`` for snippets.
              """
              engine: 'Engine'
              raw: Any
              source: str
              path: Path | None = None
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
      - id: OutlineNode
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNode:
              """One node in a structural (list/find/read) result.

              ``id`` is the node's unique, primarily name-based path used by every tool to
              address it. ``code`` carries the node's full source and is populated by
              ``find``/``read`` – ``list`` always leaves it ``None``. ``signature``/
              ``docstring`` are only set for class/function nodes whose ``code`` is
              *not* included, since the full source already makes them visible.
              Serialization drops ``None``/empty fields, see :func:`to_dict`.
              """
              id: str
              type: str | None
              lines: str | None
              signature: str | None
              docstring: str | None
              code: str | None = None
              children: list['OutlineNode'] = field(default_factory=list)
      - id: Engine
        type: ClassDef
        signature: "class Engine(ABC):"
        docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
        children:
        - id: Engine.yeTUw1|1zTXCW
          type: statements
          code: |-
            'A parser back-end turning source into an addressable, mutable tree.\n\n    Structural mutations differ fundamentally between back-ends: the Python\n    engine edits the ``ast`` object graph and re-serialises it via ``unparse``,\n    whereas generic engines splice source text at node byte-ranges and re-parse.\n    Both, however, expose the same node-oriented operations below.\n    '
            '#: Human-readable engine name (used e.g. to guard Python-only tools).'
            name: str = 'engine'
        - id: Engine.dqC7gX|bz1VNl
          type: statements
          code: |-
            '#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'
            '#: may callers rely on re-parse to catch corruption (false for markup grammars'
            '#: whose parser accepts almost any text without reporting errors).'
            validates_syntax: bool = False
        - id: Engine.parse
          type: FunctionDef
          code: |-
            @abstractmethod
            def parse(self, source: str, path: Path | None=None) -> Tree:
                """Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."""
        - id: Engine.empty_tree
          type: FunctionDef
          code: |-
            @abstractmethod
            def empty_tree(self, path: Path | None=None) -> Tree:
                """Return an empty tree, used when appending to a not-yet-existing file."""
        - id: Engine.serialize
          type: FunctionDef
          code: |-
            @abstractmethod
            def serialize(self, tree: Tree) -> str:
                """Render ``tree`` back to source text for writing to disk."""
        - id: Engine.validate
          type: FunctionDef
          code: |-
            @abstractmethod
            def validate(self, source: str) -> str | None:
                """Return an error message if ``source`` is malformed, else ``None``."""
        - id: Engine.locate_all
          type: FunctionDef
          code: |-
            @abstractmethod
            def locate_all(self, tree: Tree) -> list[Located]:
                """Flatten ``tree`` into every addressable node, in document order."""
        - id: Engine.is_definition
          type: FunctionDef
          code: |-
            def is_definition(self, node_type: str) -> bool:
                """Whether ``node_type`` is "def-like" enough for a ``signature`` to make
                    sense. Defaults to ``True`` so every node falls back to a signature
                    (via ``signature``'s own trimmed-content fallback) unless an engine
                    narrows this down; engines with a precise, known node-type set (see
                    :class:`xy.ai.mcpc.tools.ast.generic._java.JavaEngine`) should override
                    this to exclude statements/imports/etc. where a signature makes no sense."""
                return True
        - id: Engine.signature
          type: FunctionDef
          code: |-
            @abstractmethod
            def signature(self, node: Any) -> str:
                """One-line rendering of ``node``'s header (or the node itself)."""
        - id: Engine.default_signature
          type: FunctionDef
          code: |-
            def default_signature(self, node: Any, limit: int=80) -> str:
                """Fallback used when ``is_definition`` is False: the first non-blank
                line of the node's source, trimmed to ``limit`` chars."""
                first_line = next((line.strip() for line in self.node_code(node).splitlines() if line.strip()), '')
                return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
        - id: Engine.docstring
          type: FunctionDef
          code: |-
            @abstractmethod
            def docstring(self, node: Any) -> str | None:
                """Short docstring of ``node``, if the format has such a concept."""
        - id: Engine.node_code
          type: FunctionDef
          code: |-
            @abstractmethod
            def node_code(self, node: Any) -> str:
                """Full source of a single ``node``."""
        - id: Engine.replace
          type: FunctionDef
          code: |-
            @abstractmethod
            def replace(self, loc: Located, code: str) -> None:
                """Replace ``loc``'s node with ``code``."""
        - id: Engine.insert
          type: FunctionDef
          code: |-
            @abstractmethod
            def insert(self, loc: Located, code: str, position: str) -> int:
                """Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted."""
        - id: Engine.delete
          type: FunctionDef
          code: |-
            @abstractmethod
            def delete(self, loc: Located) -> None:
                """Delete ``loc``'s node from its container."""
        - id: Engine.append
          type: FunctionDef
          code: |-
            @abstractmethod
            def append(self, tree: Tree, code: str) -> int:
                """Append ``code`` at ``tree``'s top level; return units appended."""
      - id: AstError
        type: ClassDef
        code: |-
          class AstError(Exception):
              """A user-facing, path-free error raised by the AST tools."""
      - id: AstAmbiguous
        type: ClassDef
        signature: "class AstAmbiguous(AstError):"
        docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
        children:
        - id: AstAmbiguous.FSy0jO|jchvnz
          type: statements
          code: '''Raised when a text-based node search (no id given) finds several\n    unrelated candidates instead of a single node.'''
        - id: AstAmbiguous.init
          type: FunctionDef
          code: |-
            def __init__(self, message: str, candidates: list[str]) -> None:
                super().__init__(message)
                self.candidates = candidates
      - id: AstTextError
        type: ClassDef
        signature: "class AstTextError(AstError):"
        docstring: Raised when a text/marker-based edit's search text could not be applied. Carrie…
        children:
        - id: AstTextError.xHQ70j|YI3gjQ
          type: statements
          code: "\"Raised when a text/marker-based edit's search text could not be applied.\\n\\n    Carries the shared-matcher's diagnosis (see ``tools._text_match``): ``reason``\\n    classifies the cause (e.g. ``whitespace_mismatch``, ``content_changed``,\\n    ``guard_rejected``, ``marker_order``, ``ambiguous``, ``not_found``);\\n    ``corrected_text`` is a verified fix (whitespace-only difference), ``guess``\\n    an unverified single-candidate heuristic, ``next_step`` a fallback instruction.\\n    \""
        - id: AstTextError.init
          type: FunctionDef
          code: |-
            def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:
                super().__init__(message)
                self.reason = reason
                self.position = position
                self.corrected_text = corrected_text
                self.guess = guess
                self.next_step = next_step
```
/answer 7a0a-4 allow
ReasoningToken: 63
Control Request:
```yaml
id: 7a0a-5
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: "Übersicht über AST-Tool-Module, um RPC-Oberfläche zu bestimmen"
```
/answer 7a0a-5 allow
Control Request:
```yaml
id: 7a0a-6
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      error: Not a regular file.
```
/answer 7a0a-6 allow
ReasoningToken: 122
Control Request:
```yaml
id: 7a0a-7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^
    include:
    - '*.py'
    limit: 5
  reason: Dateiliste im ast tools Verzeichnis ermitteln
```
/answer 7a0a-7 allow
Control Request:
```yaml
id: 7a0a-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - lineno: 1
          match: '"""Engine-agnostic model shared by every ``ast_*`` tool.'
        - lineno: 2
          match: ""
        - lineno: 3
          match: "The tools address nodes by *selector* (id, type, name, line"
        - lineno: 4
          match: range or parent type) and never touch a concrete parser. Two engines implement
        - lineno: 5
          match: ":class:`Engine`: a Python one built on the standard-library ``ast`` module and a"
      warning: "Limit of 5 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7a0a-8 allow
Control Request:
```yaml
id: 7a0a-9
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
    source: |
      syntax = "proto3";

      // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
      // back-ends for languages other than Python (typically tree-sitter based)
      // can be plugged into the mcpc AST tools out-of-process.
      //
      // The interface is stateless: engine-native objects (ast.Module, Located.node,
      // Located.parent, ...) never cross the process boundary. Every call instead
      // carries the full current source text plus byte/line locators, and mutating
      // calls return the new full source text — mirroring how generic (tree-sitter)
      // engines already splice source at byte ranges and re-parse.
      package xy.ai.mcpc.ast.v1;

      option go_package = "xy.ai/mcpc/ast/v1;astv1";

      service AstEngine {
        // Static facts about the engine (Engine.name, Engine.validates_syntax).
        rpc Describe(DescribeRequest) returns (DescribeResponse);

        // Engine.parse + Engine.locate_all: parse source and flatten it into every
        // addressable node, in document order.
        rpc Parse(ParseRequest) returns (ParseResponse);

        // Engine.validate.
        rpc Validate(ValidateRequest) returns (ValidateResponse);

        // Engine.is_definition.
        rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

        // Engine.signature (falls back to Engine.default_signature internally).
        rpc Signature(NodeRequest) returns (SignatureResponse);

        // Engine.docstring.
        rpc Docstring(NodeRequest) returns (DocstringResponse);

        // Engine.node_code.
        rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

        // Engine.replace.
        rpc Replace(ReplaceRequest) returns (MutateResponse);

        // Engine.insert.
        rpc Insert(InsertRequest) returns (MutateResponse);

        // Engine.delete.
        rpc Delete(NodeRequest) returns (MutateResponse);

        // Engine.append.
        rpc Append(AppendRequest) returns (MutateResponse);
      }

      // Engine.name / Engine.validates_syntax.
      message DescribeRequest {}

      message DescribeResponse {
        // Human-readable engine name (e.g. "java", "typescript").
        string name = 1;
        // Whether validate()/replace() reliably reject malformed edits.
        bool validates_syntax = 2;
      }

      // Mirrors base.Located, minus the engine-native node/parent/tree/index fields
      // which never leave the engine process. `node_id` is the sole handle other
      // calls use to re-address the node against a given `source`.
      message NodeLocation {
        // Fully-qualified, engine-assigned id (name/hash path from the root).
        string node_id = 1;
        // Engine-reported node type name.
        string node_type = 2;
        // Simple name, if the node carries one.
        optional string name = 3;
        // 1-based inclusive line span.
        int32 lineno = 4;
        int32 end_lineno = 5;
        // 0-based inclusive byte offsets into `source`, for exact re-splicing.
        int32 start_byte = 6;
        int32 end_byte = 7;
        // Type name of the parent node, absent at the top level.
        optional string parent_type = 8;
        // Whether `read`-style tools should descend into children instead of
        // returning this node's full source.
        bool expandable = 9;
      }

      message ParseRequest {
        string source = 1;
        // Absolute path the source was loaded from; absent for snippets/new files.
        optional string path = 2;
      }

      message ParseResponse {
        repeated NodeLocation nodes = 1;
      }

      message ValidateRequest {
        string source = 1;
      }

      message ValidateResponse {
        // Error message if `source` is malformed, absent if valid.
        optional string error = 1;
      }

      message IsDefinitionRequest {
        string node_type = 1;
      }

      message IsDefinitionResponse {
        bool is_definition = 1;
      }

      // Common request shape for read-only per-node calls (Signature/Docstring/
      // NodeCode/Delete): the node is re-resolved against `source` via `node`.
      message NodeRequest {
        string source = 1;
        NodeLocation node = 2;
      }

      message SignatureResponse {
        string signature = 1;
      }

      message DocstringResponse {
        // Absent if the format has no docstring concept, or the node has none.
        optional string docstring = 1;
      }

      message NodeCodeResponse {
        string code = 1;
      }

      message ReplaceRequest {
        string source = 1;
        NodeLocation node = 2;
        string code = 3;
      }

      message InsertRequest {
        string source = 1;
        NodeLocation node = 2;
        string code = 3;
        // "before" or "after".
        string position = 4;
      }

      message AppendRequest {
        string source = 1;
        string code = 2;
      }

      // Shared result of every mutating call: the new full source plus how many
      // top-level units were affected (Engine.insert/append return this count;
      // Replace/Delete always report 1).
      message MutateResponse {
        string source = 1;
        int32 units_affected = 2;
      }
  reason: Proto-Datei für AST-Engine-RPC-Anbindung basierend auf dem Engine-ABC-Interface erstellen
```
/answer 7a0a-9 allow
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 23906, in: 143, out: 5011, reason: 1680, read: 58360, write: 18752
ReasoningToken: 1680
Erstellt: `/home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto`

Kerndesign: zustandslos — statt engine-nativer Objekte (`ast.Module`, `Located.node/parent`) werden bei jedem Call der volle Quelltext plus `NodeLocation` (id/type/name/Zeilen/Byte-Offsets) übergeben; mutierende RPCs (`Replace`/`Insert`/`Delete`/`Append`) liefern den neuen Volltext zurück. Fehler (`AstError`) werden als gRPC-Status transportiert, nicht als Proto-Feld.
Was liefert "Replace" zurück?
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 24204, in: 145, out: 5064, reason: 0, read: 77112, write: 18995
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
`MutateResponse`: den neuen vollständigen Quelltext (`source`) sowie `units_affected` (bei `Replace` immer `1`).
Warum?
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 24717, in: 147, out: 5516, reason: 90, read: 96107, write: 19054
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 90
Weil das Interface zustandslos ist: Es gibt keinen persistenten Tree-Handle auf der Engine-Seite, und `Located.node`/`parent` (die nativen Objekte, die `Engine.replace` im Original mutiert) dürfen nie über die Prozessgrenze wandern.

Der einzige Weg für den Aufrufer, das Ergebnis der Mutation zu erhalten, ist also der **neue vollständige Quelltext** – analog zu `Engine.serialize(tree)` nach der Mutation. Damit kann die mcpc-Seite ihn direkt weiterverarbeiten (erneut parsen, validieren, auf Platte schreiben), ohne dass die Engine Zustand halten muss.

`units_affected` ist nur der gemeinsamen Response-Form mit `Insert`/`Append` wegen dabei (dort liefert `Engine.insert`/`append` echte Zähler); bei `Replace`/`Delete` ist er trivial `1`, könnte alternativ auch weggelassen werden.
Das entspricht aber nicht dem Interface. Der aufrufer bekommt jetzt auch keinen Quelltext zurück. Was eillst du ihn dem beim Replace zurück geben?! Das was er gerade gesendet hat?!
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 26452, in: 149, out: 6711, reason: 578, read: 115161, write: 19592
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 578
Zu Recht – das ist eine bewusste Abweichung vom lokalen Signaturbild, keine 1:1-Übersetzung, und ich habe das vorher nicht klar gemacht.

Lokal gibt `Engine.replace(loc, code) -> None` nichts zurück, weil die Mutation **in-place** im gemeinsam gehaltenen `Tree`-Objekt passiert: der Engine-Prozess und der Aufrufer teilen sich `tree.raw`/`tree.source`. Nach dem Call ist `tree.source` (bzw. `tree.raw`) bereits aktualisiert; der Aufrufer liest es später selbst oder über `serialize(tree)`.

Über RPC gibt es diesen gemeinsamen Speicher nicht. Der von mir übergebene `source`-Request-Parameter ist nur eine **Kopie**, die die Engine serverseitig parst, an der Splice-Stelle (`start_byte`/`end_byte`) durch `code` ersetzt und neu zusammensetzt. Diese neue, server-berechnete Version muss explizit zurückgegeben werden – sonst hätte der Aufrufer nach dem Call gar keinen Zugriff auf das Ergebnis der Mutation. Es ist also **nicht** das, was gesendet wurde, sondern `source[:start_byte] + code + source[end_byte:]` (bzw. das Äquivalent nach Re-Parse), serverseitig berechnet.

Kurz: `None` ist im Original nur deshalb korrekt, weil die Mutation lokal sichtbar bleibt. Bei RPC muss dieselbe Information (die veränderte Quelle) als Rückgabewert transportiert werden, sonst geht das Ergebnis der Operation verloren.
Das ist eine Fehlinterpretation. Der aufrufer hat keinen Zugriff auf irgendwas hinter dem Interface. Er hat keinen Zugriff auf den Tree.
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7a0a-a
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: serialize|locate_all|\.replace\(|\.insert\(|\.delete\(|\.append\(
    include:
    - '*.py'
    exclude:
    - base.py
    limit: 40
  reason: "Verstehen, wie Aufrufer Tree, serialize und Mutationen tatsächlich nutzen"
```
/answer 7a0a-a allow
Control Request:
```yaml
id: 7a0a-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: find.py
        matches:
        - lineno: 59
          match: "nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines, with_type=with_type)"
        - lineno: 63
          match: "fallback = core.resolve_by_prefix(core.locate_all(tree), exact['id'])"
        - lineno: 82
          match: ordered.append(loc)
      - path: script.py
        matches:
        - lineno: 90
          match: "def locate_all(self) -> list[core.Located]:"
        - lineno: 91
          match: return core.locate_all(self._tree)
        - lineno: 174
          match: "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr. tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path. Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_list/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else). Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
      - path: replace.py
        matches:
        - lineno: 103
          match: results.append(_replace_one(item))
        - lineno: 105
          match: "errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))"
        - lineno: 128
          match: "def result_serializer(r: ReplaceResult) -> dict[str, Any]:"
        - lineno: 136
          match: "def error_serializer(e: ReplaceError) -> dict[str, Any]:"
        - lineno: 146
          match: "result_serializer,"
        - lineno: 147
          match: "error_serializer,"
      - path: list.py
        matches:
        - lineno: 9
          match: "from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result"
        - lineno: 69
          match: "nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines, with_type=False)"
        - lineno: 101
          match: "results.append(_list_one(real_path, with_lines=with_lines))"
        - lineno: 103
          match: "errors.append(ListNodesError(path=real_path, error=str(exc)))"
        - lineno: 128
          match: "result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}"
        - lineno: 129
          match: "error_serializer = lambda e: {'path': e.path, 'error': e.error}"
        - lineno: 130
          match: "content = serialize_batch_result(batch, result_serializer, error_serializer)"
      - path: delete.py
        matches:
        - lineno: 102
          match: results.append(_delete_one(item))
        - lineno: 104
          match: "errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))"
      - path: generic/_css.py
        matches:
        - lineno: 58
          match: "def locate_all(self, tree: Tree) -> list[Located]:"
        - lineno: 73
          match: results.append(
        - lineno: 108
          match: results.append(
      - path: generic/_html.py
        matches:
        - lineno: 111
          match: "def locate_all(self, tree: Tree) -> list[Located]:"
        - lineno: 121
          match: results.append(
      - path: generic/_yaml.py
        matches:
        - lineno: 42
          match: units.append(child)
        - lineno: 98
          match: "def locate_all(self, tree: Tree) -> list[Located]:"
        - lineno: 113
          match: results.append(
      - path: generic/_markdown.py
        matches:
        - lineno: 35
          match: out.append(child)
        - lineno: 42
          match: "pages.append([])"
        - lineno: 44
          match: "pages[-1].append(node)"
        - lineno: 65
          match: "(stack[-1].items if stack else roots).append(section)"
        - lineno: 66
          match: stack.append(section)
        - lineno: 68
          match: "(stack[-1].items if stack else roots).append(node)"
        - lineno: 98
          match: "def locate_all(self, tree: Tree) -> list[Located]:"
      - path: generic/_java.py
        matches:
        - lineno: 52
          match: "def locate_all(self, tree: Tree) -> list[Located]:"
        - lineno: 67
          match: results.append(
        - lineno: 102
          match: results.append(
      - path: generic/_text.py
        matches:
        - lineno: 34
          match: "def serialize(self, tree: Tree) -> str:"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7a0a-b allow
ReasoningToken: 552
Control Request:
```yaml
id: 7a0a-c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids: []
  reason: Genaues Verhalten von replace/insert/delete/append und Persistierung des Quelltexts verstehen"
```
/answer 7a0a-c allow
Control Request:
```yaml
id: 7a0a-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
      nodes:
      - id: wlNo2J|amT2KR
        signature: '"``ast_script`` – run restricted Python against the parsed AST.\n\nFor complex …'
      - id: HbnBMd|8t7Kee
        signature: import ast
      - id: l7jic4|D86RaJ
        signature: "__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'Scrip…"
      - id: Za6t1t|HuXBMZ
        signature: "_SAFE_BUILTINS = {"
      - id: ScriptTree
        signature: "class ScriptTree:"
        docstring: Engine-agnostic ``tree`` handle exposed to sandboxed scripts. Wraps a :class:`c…
        children:
        - id: ScriptTree.0KadZG|R2UlTU
          signature: '"Engine-agnostic ``tree`` handle exposed to sandboxed scripts.\n\n    Wraps a :…'
        - id: ScriptTree.init
          signature: "def __init__(self, tree: core.Tree) -> None:"
        - id: ScriptTree.raw
          signature: "@property def raw(self) -> Any:"
        - id: ScriptTree.source
          signature: "@property def source(self) -> str:"
        - id: ScriptTree.path
          signature: "@property def path(self) -> Any:"
        - id: ScriptTree.find
          signature: "def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:"
        - id: ScriptTree.locate_all
          signature: "def locate_all(self) -> list[core.Located]:"
        - id: ScriptTree.node_code
          signature: "def node_code(self, loc: core.Located) -> str:"
        - id: ScriptTree.replace
          signature: "def replace(self, loc: core.Located, code: str) -> str | None:"
        - id: ScriptTree.insert
          signature: "def insert(self, loc: core.Located, code: str, position: str='after') -> int:"
        - id: ScriptTree.delete
          signature: "def delete(self, loc: core.Located) -> None:"
        - id: ScriptTree.append
          signature: "def append(self, code: str) -> int:"
      - id: ScriptError
        signature: "class ScriptError(Exception):"
        docstring: Raised when an AST script cannot be run to completion.
      - id: AstScriptResult
        signature: "@dataclass(frozen=True) class AstScriptResult:"
        docstring: "Result of :func:`ast_script`. Attributes: result: Always ``\"success\"``. value: …"
      - id: ast_script
        signature: "def ast_script(path: str, code: str) -> AstScriptResult:"
        docstring: Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``. …
      - id: ScriptTool
        signature: "class ScriptTool(ToolDefinition):"
        children:
        - id: ScriptTool.dNdM46|IeCbts
          signature: name = 'ast_script'
        - id: ScriptTool.LwzTKu|Ym6sw1
          signature: "input_schema = {"
        - id: ScriptTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G65kv0|hrDhpa
        signature: "'``ast_replace`` tool: replace selected nodes with new source.'"
      - id: wlvDdw|eXR9kh
        signature: from dataclasses import dataclass
      - id: xj2nBU|s4L0vm
        signature: "__all__ = ["
      - id: ReplaceItem
        signature: "@dataclass(frozen=True) class ReplaceItem:"
        docstring: "One node replacement to apply. Attributes: path: Absolute path to the file to m…"
      - id: ReplaceResult
        signature: "@dataclass(frozen=True) class ReplaceResult:"
        docstring: "Result of a single node replacement. Attributes: path: The path exactly as give…"
      - id: ReplaceError
        signature: "@dataclass(frozen=True) class ReplaceError:"
        docstring: "Error applying a single node replacement. Attributes: path: The path exactly as…"
      - id: ReplaceBatchResult
        signature: "@dataclass(frozen=True) class ReplaceBatchResult:"
        docstring: "Result of :func:`ast_replace`. Attributes: results: One :class:`ReplaceResult` …"
      - id: replace_one
        signature: "def _replace_one(item: ReplaceItem) -> ReplaceResult:"
      - id: ast_replace
        signature: "def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:"
        docstring: Replace one or more selected nodes with new source. ``id`` and replacement ``so…
      - id: ReplaceNodeTool
        signature: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.uarnhd|Sqou3K
          signature: name = 'ast_replace'
        - id: ReplaceNodeTool.ei5nIo|Lu9IT0
          signature: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
        - id: ReplaceNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 7a0a-d allow
ReasoningToken: 234
Control Request:
```yaml
id: 7a0a-e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    ids:
    - ScriptTree.replace
    - ScriptTree.insert
    - ScriptTree.delete
    - ScriptTree.append
    - ScriptTree.init
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids:
    - replace_one
  reason: Implementierungsdetails von Mutation und Persistierung prüfen
```
/answer 7a0a-e allow
Control Request:
```yaml
id: 7a0a-f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
      nodes:
      - id: ScriptTree.replace
        type: FunctionDef
        code: |-
          def replace(self, loc: core.Located, code: str) -> str | None:
              return core.replace_node(loc, code)
      - id: ScriptTree.insert
        type: FunctionDef
        code: |-
          def insert(self, loc: core.Located, code: str, position: str='after') -> int:
              return core.insert_node(loc, code, position)
      - id: ScriptTree.delete
        type: FunctionDef
        code: |-
          def delete(self, loc: core.Located) -> None:
              core.delete_node(loc)
      - id: ScriptTree.append
        type: FunctionDef
        code: |-
          def append(self, code: str) -> int:
              return core.append_nodes(self._tree, code)
      - id: ScriptTree.init
        type: FunctionDef
        code: |-
          def __init__(self, tree: core.Tree) -> None:
              self._tree = tree
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: replace_one
        type: FunctionDef
        code: |-
          def _replace_one(item: ReplaceItem) -> ReplaceResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_path(tree, id=item.id)
              new_id = core.replace_node(target, item.source)
              core.CACHE.save(file_path, tree)
              return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id)
```
/answer 7a0a-f allow
ReasoningToken: 40
Control Request:
```yaml
id: 7a0a-10
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: def replace_node|def insert_node|def delete_node|def append_nodes|class CACHE|def save\(|def get_tree\(
    include:
    - '*.py'
    limit: 30
  reason: "core.replace_node/CACHE Implementierung finden, um Rückgabewert und Persistenz zu verstehen"
```
/answer 7a0a-10 allow
Control Request:
```yaml
id: 7a0a-11
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: core.py
        matches:
        - lineno: 49
          match: "def get_tree(self, path: Path) -> Tree:"
        - lineno: 66
          match: "def save(self, path: Path, tree: Tree) -> str:"
        - lineno: 115
          match: "def replace_node(loc: Located, code: str) -> str | None:"
        - lineno: 121
          match: "def insert_node(loc: Located, code: str, position: str) -> int:"
        - lineno: 124
          match: "def delete_node(loc: Located) -> None:"
        - lineno: 127
          match: "def append_nodes(tree: Tree, code: str) -> int:"
```
/answer 7a0a-11 allow
ReasoningToken: 334
Control Request:
```yaml
id: 7a0a-12
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    ids: []
  reason: "CACHE, replace_node, insert_node, save vollständig lesen für korrektes RPC-Design"
```
/answer 7a0a-12 allow
Control Request:
```yaml
id: 7a0a-13
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: V6fdSR|z5Q7Tq
        signature: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engi…"
      - id: CXxOCc|7iHPso
        signature: from __future__ import annotations
      - id: 85RFV8|RBFWwR
        signature: "_PYTHON_EXTENSIONS = ('.py', '.pyi')"
      - id: engine_for_path
        signature: "def engine_for_path(path: Path) -> Engine:"
        docstring: Return the engine responsible for ``path`` based on its extension.
      - id: CacheEntry
        signature: "@dataclass class _CacheEntry:"
      - id: AstCache
        signature: "class AstCache:"
        docstring: Content-hash validated cache of parsed trees keyed by absolute path.
        children:
        - id: AstCache.AzRmHX|ayni5v
          signature: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
        - id: AstCache.init
          signature: "def __init__(self) -> None:"
        - id: AstCache.get_tree
          signature: "def get_tree(self, path: Path) -> Tree:"
        - id: AstCache.save
          signature: "def save(self, path: Path, tree: Tree) -> str:"
          docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
        - id: AstCache.invalidate
          signature: "def invalidate(self, path: Path) -> None:"
      - id: dRq1FJ|5EG9eJ
        signature: "'#: Process-wide shared cache instance.'"
      - id: load
        signature: "def load(path_str: str) -> tuple[Path, Tree]:"
        docstring: Resolve *path_str* and return it together with its cached tree.
      - id: parse_source
        signature: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
        docstring: Parse *source* with *engine* (Python by default for path-less snippets).
      - id: parse_for
        signature: "def parse_for(path_str: str, code: str) -> Tree:"
        docstring: Parse *code* with the engine selected for *path_str*'s extension.
      - id: locate_all
        signature: "def locate_all(tree: Tree) -> list[Located]:"
      - id: edit_node_source
        signature: "def edit_node_source(loc: Located) -> str:"
      - id: relocate
        signature: "def relocate(tree: Tree, old: Located) -> Located | None:"
        docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
      - id: replace_node
        signature: "def replace_node(loc: Located, code: str) -> str | None:"
        docstring: Replace ``loc``'s node with ``code``; return its new id.
      - id: insert_node
        signature: "def insert_node(loc: Located, code: str, position: str) -> int:"
      - id: delete_node
        signature: "def delete_node(loc: Located) -> None:"
      - id: append_nodes
        signature: "def append_nodes(tree: Tree, code: str) -> int:"
      - id: empty_tree
        signature: "def empty_tree(path: Path) -> Tree:"
      - id: validate_source
        signature: "def validate_source(path: Path, source: str) -> str | None:"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 7a0a-13 allow
Text:
 Sie haben recht — meine Analyse war falsch. Ich habe mir `core.py`/`script.py` genauer angesehen: Der Aufrufer hält serverseitig (im mcpc-Prozess) einen `Tree` mit einem **opaken** `raw`-Feld und opaken `node`/`parent`-Handles in `Located` – er "introspiziert" diese laut Doku nie, sondern reicht sie nur als Argumente an weitere `tree.*`-Aufrufe zurück. Quelltext wird **nicht** bei jeder Mutation zurückgegeben, sondern nur explizit via `serialize(tree)` bzw. `AstCache.save()` abgerufen, wenn er tatsächlich gebraucht wird (z.B. zum Schreiben auf Platte).

Das rechtfertigt ein **zustandsbehaftetes** RPC-Design statt meines ursprünglichen: Ich ersetze das Proto so, dass es die `Engine`-ABC 1:1 abbildet, inklusive `None`-Rückgaben bei `Replace`/`Delete`.
Control Request:
```yaml
id: 7a0a-14
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
    start_marker: syntax = "proto3";
    end_marker: |
      int32 units_affected = 2;
      }
    content: |
      syntax = "proto3";

      // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
      // back-ends for languages other than Python (typically tree-sitter based)
      // can be plugged into the mcpc AST tools out-of-process.
      //
      // Stateful, mirroring the local usage: the remote engine holds the actual
      // parsed tree, keyed by an opaque `tree_handle` returned from Parse/EmptyTree
      // (stands in for the local Tree.raw). Nodes are addressed within that tree by
      // `node_id`, the same opaque, engine-assigned handle carried by Located.node
      // ("for use only as tree.* arguments, never introspect their internals").
      // Replace/Insert/Delete/Append mutate the remote tree in place, exactly as
      // their local Engine counterparts do — the resulting source is fetched
      // separately via Serialize, only when actually needed (matching
      // AstCache.save's serialize-on-demand behaviour).
      package xy.ai.mcpc.ast.v1;

      option go_package = "xy.ai/mcpc/ast/v1;astv1";

      import "google/protobuf/empty.proto";

      service AstEngine {
        // Static facts about the engine (Engine.name, Engine.validates_syntax).
        rpc Describe(DescribeRequest) returns (DescribeResponse);

        // Engine.parse: parse source into a tree, raising on error (gRPC status).
        rpc Parse(ParseRequest) returns (ParseResponse);

        // Engine.empty_tree: used when appending to a not-yet-existing file.
        rpc EmptyTree(EmptyTreeRequest) returns (ParseResponse);

        // Engine.serialize: render the tree back to source text, on demand.
        rpc Serialize(TreeRequest) returns (SerializeResponse);

        // Engine.validate. Stateless: no tree required.
        rpc Validate(ValidateRequest) returns (ValidateResponse);

        // Engine.locate_all: flatten the tree into every addressable node, in
        // document order. Also (re-)usable after a mutation to refresh ids.
        rpc LocateAll(TreeRequest) returns (LocateAllResponse);

        // Engine.is_definition. Stateless.
        rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

        // Engine.signature (falls back to Engine.default_signature internally).
        rpc Signature(NodeRequest) returns (SignatureResponse);

        // Engine.docstring.
        rpc Docstring(NodeRequest) returns (DocstringResponse);

        // Engine.node_code.
        rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

        // Engine.replace: mutates the tree in place, returns nothing.
        rpc Replace(ReplaceRequest) returns (google.protobuf.Empty);

        // Engine.insert: mutates the tree in place; returns units inserted.
        rpc Insert(InsertRequest) returns (UnitsResponse);

        // Engine.delete: mutates the tree in place, returns nothing.
        rpc Delete(NodeRequest) returns (google.protobuf.Empty);

        // Engine.append: mutates the tree in place; returns units appended.
        rpc Append(AppendRequest) returns (UnitsResponse);

        // Not part of the Engine ABC: releases a tree_handle's server-side state.
        // Local engines have no such lifecycle (garbage-collected with the
        // process); a remote engine needs it to avoid leaking held trees.
        rpc ReleaseTree(TreeRequest) returns (google.protobuf.Empty);
      }

      // Engine.name / Engine.validates_syntax.
      message DescribeRequest {}

      message DescribeResponse {
        // Human-readable engine name (e.g. "java", "typescript").
        string name = 1;
        // Whether validate()/replace() reliably reject malformed edits.
        bool validates_syntax = 2;
      }

      // Opaque handle standing in for the local Tree.raw; valid only against the
      // engine instance that issued it, until ReleaseTree.
      message TreeRequest {
        string tree_handle = 1;
      }

      message ParseRequest {
        string source = 1;
        // Absolute path the source was loaded from; absent for snippets/new files.
        optional string path = 2;
      }

      message EmptyTreeRequest {
        optional string path = 1;
      }

      message ParseResponse {
        string tree_handle = 1;
      }

      message SerializeResponse {
        string source = 1;
      }

      message ValidateRequest {
        string source = 1;
      }

      message ValidateResponse {
        // Error message if `source` is malformed, absent if valid.
        optional string error = 1;
      }

      // Mirrors base.Located, minus tree/parent/index (server-internal); `node_id`
      // is the same opaque, engine-assigned handle used as the `node` argument of
      // Signature/Docstring/NodeCode/Replace/Insert/Delete.
      message NodeLocation {
        // Opaque per-tree node handle (doubles as the human-facing, unique id
        // exposed to ast_* tools, e.g. "MyClass.method").
        string node_id = 1;
        // Engine-reported node type name.
        string node_type = 2;
        // Simple name, if the node carries one.
        optional string name = 3;
        // 1-based inclusive line span.
        int32 lineno = 4;
        int32 end_lineno = 5;
        // Type name of the parent node, absent at the top level.
        optional string parent_type = 6;
        // Whether `read`-style tools should descend into children instead of
        // returning this node's full source.
        bool expandable = 7;
      }

      message LocateAllResponse {
        repeated NodeLocation nodes = 1;
      }

      message IsDefinitionRequest {
        string node_type = 1;
      }

      message IsDefinitionResponse {
        bool is_definition = 1;
      }

      // Common request shape for read-only per-node calls (Signature/Docstring/
      // NodeCode) and for Delete: the node is addressed within its tree_handle.
      message NodeRequest {
        string tree_handle = 1;
        string node_id = 2;
      }

      message SignatureResponse {
        string signature = 1;
      }

      message DocstringResponse {
        // Absent if the format has no docstring concept, or the node has none.
        optional string docstring = 1;
      }

      message NodeCodeResponse {
        string code = 1;
      }

      message ReplaceRequest {
        string tree_handle = 1;
        string node_id = 2;
        string code = 3;
      }

      message InsertRequest {
        string tree_handle = 1;
        string node_id = 2;
        string code = 3;
        // "before" or "after".
        string position = 4;
      }

      message AppendRequest {
        string tree_handle = 1;
        string code = 2;
      }

      message UnitsResponse {
        int32 units = 1;
      }
  reason: "Proto auf zustandsbehaftetes Design korrigieren, das die Engine-ABC-Signaturen exakt abbildet"
```
/answer 7a0a-14 allow Das ist ein Trugschluss. es ist nicht Zustandbehaftet. Das die Engine den AST Cached ist eine Optimierung. Es ist zustandsfrei. Jede Aktion liest den AST neu ein und Schreibt ihn wieder raus. Der AST ist deterministisch. Das gleich Prinzip werden die RPC Engines verwenden. Zustandsfrei mit Cache.
ReasoningToken: 2372
Control Request:
```yaml
id: 7a0a-15
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
    start_marker: syntax = "proto3";
    end_marker: |
      int32 units = 1;
      }
    content: |
      syntax = "proto3";

      // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
      // back-ends for languages other than Python (typically tree-sitter based)
      // can be plugged into the mcpc AST tools out-of-process.
      //
      // Stateless: the AST is deterministic, so every call carries the full
      // current source and every mutating call returns the full resulting source.
      // No tree/node handle survives across calls. Engines may cache parsed trees
      // internally (e.g. keyed by a content hash) as a pure performance
      // optimisation — that caching is invisible to this protocol, exactly as
      // xy.ai.mcpc.tools.ast.core.AstCache is invisible to the local Engine ABC.
      package xy.ai.mcpc.ast.v1;

      option go_package = "xy.ai/mcpc/ast/v1;astv1";

      service AstEngine {
        // Static facts about the engine (Engine.name, Engine.validates_syntax).
        rpc Describe(DescribeRequest) returns (DescribeResponse);

        // Engine.parse + Engine.locate_all: parse source and flatten it into every
        // addressable node, in document order. Raises (gRPC status) on parse error.
        rpc Parse(ParseRequest) returns (ParseResponse);

        // Engine.empty_tree + Engine.locate_all, for a not-yet-existing file.
        rpc EmptyTree(EmptyTreeRequest) returns (ParseResponse);

        // Engine.validate.
        rpc Validate(ValidateRequest) returns (ValidateResponse);

        // Engine.is_definition.
        rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

        // Engine.signature (falls back to Engine.default_signature internally).
        rpc Signature(NodeRequest) returns (SignatureResponse);

        // Engine.docstring.
        rpc Docstring(NodeRequest) returns (DocstringResponse);

        // Engine.node_code.
        rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

        // Engine.replace, immediately followed by Engine.serialize.
        rpc Replace(ReplaceRequest) returns (MutateResponse);

        // Engine.insert, immediately followed by Engine.serialize.
        rpc Insert(InsertRequest) returns (MutateResponse);

        // Engine.delete, immediately followed by Engine.serialize.
        rpc Delete(NodeRequest) returns (MutateResponse);

        // Engine.append, immediately followed by Engine.serialize.
        rpc Append(AppendRequest) returns (MutateResponse);
      }

      // Engine.name / Engine.validates_syntax.
      message DescribeRequest {}

      message DescribeResponse {
        // Human-readable engine name (e.g. "java", "typescript").
        string name = 1;
        // Whether validate()/replace() reliably reject malformed edits.
        bool validates_syntax = 2;
      }

      // Mirrors base.Located, minus the engine-native node/parent/tree/index fields,
      // which never leave the engine process. `node_id` plus the byte offsets are
      // enough to re-address the node against a freshly resent `source`.
      message NodeLocation {
        // Fully-qualified, engine-assigned id (name/hash path from the root).
        string node_id = 1;
        // Engine-reported node type name.
        string node_type = 2;
        // Simple name, if the node carries one.
        optional string name = 3;
        // 1-based inclusive line span.
        int32 lineno = 4;
        int32 end_lineno = 5;
        // 0-based inclusive byte offsets into `source`, for exact re-splicing.
        int32 start_byte = 6;
        int32 end_byte = 7;
        // Type name of the parent node, absent at the top level.
        optional string parent_type = 8;
        // Whether `read`-style tools should descend into children instead of
        // returning this node's full source.
        bool expandable = 9;
      }

      message ParseRequest {
        string source = 1;
        // Absolute path the source was loaded from; absent for snippets/new files.
        optional string path = 2;
      }

      message EmptyTreeRequest {
        optional string path = 1;
      }

      message ParseResponse {
        repeated NodeLocation nodes = 1;
      }

      message ValidateRequest {
        string source = 1;
      }

      message ValidateResponse {
        // Error message if `source` is malformed, absent if valid.
        optional string error = 1;
      }

      message IsDefinitionRequest {
        string node_type = 1;
      }

      message IsDefinitionResponse {
        bool is_definition = 1;
      }

      // Common request shape for read-only per-node calls (Signature/Docstring/
      // NodeCode) and for Delete: the node is re-resolved against `source` via
      // `node`, requiring no prior call to have happened against this engine.
      message NodeRequest {
        string source = 1;
        NodeLocation node = 2;
      }

      message SignatureResponse {
        string signature = 1;
      }

      message DocstringResponse {
        // Absent if the format has no docstring concept, or the node has none.
        optional string docstring = 1;
      }

      message NodeCodeResponse {
        string code = 1;
      }

      message ReplaceRequest {
        string source = 1;
        NodeLocation node = 2;
        string code = 3;
      }

      message InsertRequest {
        string source = 1;
        NodeLocation node = 2;
        string code = 3;
        // "before" or "after".
        string position = 4;
      }

      message AppendRequest {
        string source = 1;
        string code = 2;
      }

      // Result of every mutating call: the new full source (Engine.serialize after
      // the mutation) plus how many top-level units were affected (Engine.insert/
      // append return this count; Replace/Delete always report 1).
      message MutateResponse {
        string source = 1;
        int32 units_affected = 2;
      }
  reason: Proto zurück auf zustandsfreies Design mit Source-in/Source-out je Call korrigieren
```
/answer 7a0a-15 deny was sol das?
Text:
 Verstanden — das erklärt die Diskrepanz. Ich baue das Proto zurück auf zustandsfrei um: jeder Call trägt den vollen Quelltext, jede Mutation liefert den neuen vollen Quelltext zurück; Caching (per Content-Hash) bleibt eine reine, dem Protokoll unsichtbare Implementierungsoptimierung der Engine.
Control Request:
```yaml
id: 7a0a-16
toolName: ask_user
arguments:
  question: "Ich wollte das Proto auf das zustandsfreie Source-in/Source-out-Muster zurücksetzen (jeder Call sendet den vollen Quelltext, jede Mutation liefert den neuen vollen Quelltext zurück), gemäß deiner letzten Klarstellung. War der Edit an sich das Problem (z.B. zu großer Eingriff/falscher Ansatz), oder woran genau soll ich es stattdessen ausrichten?"
  reason: "Nach abgelehntem Edit-Versuch klären, was am Vorgehen korrigiert werden soll, bevor ich erneut in die Proto-Datei schreibe."
```
/answer 7a0a-16 allow Du sollst das Interface abbilden. Ein Replace liefert keinen Quellcode zurück, warum auch, niemand braucht ihn?!
ReasoningToken: 1354
ReasoningToken: 65
Control Request:
```yaml
id: 7a0a-17
toolName: ask_user
arguments:
  question: "Um es korrekt zu treffen: Soll `Replace` (und analog `Insert`/`Delete`) 1:1 der ABC-Signatur folgen (mutiert etwas serverseitig, liefert `Empty`/nur die Zähler wie lokal), und der Aufrufer holt sich den resultierenden Quelltext bei Bedarf über einen separaten `Serialize`-Call ab – wobei zwischen den Calls kein Session-Handle nötig ist, weil jeder Call den Quelltext ohnehin neu mitschickt und serverseitig (zustandsfrei, ggf. gecacht) neu parst, mutiert und direkt mitserialisiert zurückgibt, nur eben nicht als Rückgabewert von Replace, sondern... wo dann? Bitte sag mir, welchen Weg der resultierende Quelltext nimmt, wenn nicht als Replace-Rückgabewert."
  reason: "Widerspruch zwischen 'zustandsfrei' und 'Replace liefert keinen Code' auflösen, um das Proto korrekt zu gestalten."
```
/answer 7a0a-17 allow Ich verstehe dein Problem nicht? Was daran verstehst du nicht?
ReasoningToken: 2773
Control Request:
```yaml
id: 7a0a-18
toolName: ask_user
arguments:
  question: "Arbeiten die RPC-Engines direkt auf der Datei (bekommen einen Pfad, lesen ihn selbst von der Platte ein und schreiben das Ergebnis selbst wieder dorthin zurück), sodass kein Quelltext über RPC transportiert werden muss – Replace/Insert/Delete/Append also nur Pfad + Node-Selektor + Code entgegennehmen und nichts (bzw. nur den Zähler) zurückliefern?"
  reason: "Klären, ob die RPC-Engine direkt Dateisystemzugriff hat (Pfad-basiert) statt Quelltext über die Leitung zu schicken, um das Proto korrekt zu gestalten."
```
/answer 7a0a-18 allow Ja das tun sie. Möchte der Aufrufer den Code erhalten, kann er NodeCode verwenden, aber das weißt du alles selbst du hast sogar bereits die Tools inspiziert. Deshalb kann ich nciht nachvollziehen was dich stört.
ReasoningToken: 2515
Text:
 Alles klar, jetzt passt das Bild. Ich baue das Proto entsprechend um: Engine arbeitet dateibasiert (eigener Dateisystemzugriff über `path`), keine Quelltext-Payloads, Mutationen liefern nur, was die ABC auch liefert.
Control Request:
```yaml
id: 7a0a-19
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
    start_marker: syntax = "proto3";
    end_marker: |
      int32 units_affected = 2;
      }
    content: |
      syntax = "proto3";

      // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
      // back-ends for languages other than Python (typically tree-sitter based)
      // can be plugged into the mcpc AST tools out-of-process.
      //
      // File-based and stateless: the engine has its own filesystem access and
      // reads/re-parses `path` itself on every call, mutates and writes the file
      // back out itself, exactly as the local Engine + AstCache do together. No
      // source text and no tree/node handle ever crosses the wire; a node is
      // addressed purely by `node_id` (as produced by List/Find) within `path`.
      // Engines may cache parsed trees internally (e.g. keyed by path + mtime/
      // content hash) as a pure performance optimisation, invisible to this
      // protocol. Callers who need the resulting text use NodeCode/Read directly.
      package xy.ai.mcpc.ast.v1;

      option go_package = "xy.ai/mcpc/ast/v1;astv1";

      service AstEngine {
        // Static facts about the engine (Engine.name, Engine.validates_syntax).
        rpc Describe(DescribeRequest) returns (DescribeResponse);

        // Engine.parse + Engine.locate_all (or Engine.empty_tree if `path` doesn't
        // exist yet): every addressable node of `path`, in document order.
        rpc List(ListRequest) returns (ListResponse);

        // Engine.validate against `path`'s current on-disk content.
        rpc Validate(ValidateRequest) returns (ValidateResponse);

        // Engine.is_definition. Stateless, no file involved.
        rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

        // Engine.signature (falls back to Engine.default_signature internally).
        rpc Signature(NodeRequest) returns (SignatureResponse);

        // Engine.docstring.
        rpc Docstring(NodeRequest) returns (DocstringResponse);

        // Engine.node_code: the only way to retrieve source text over this API.
        rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

        // Engine.replace + write-back. No return value, as locally.
        rpc Replace(ReplaceRequest) returns (Empty);

        // Engine.insert + write-back; returns units inserted.
        rpc Insert(InsertRequest) returns (UnitsResponse);

        // Engine.delete + write-back. No return value, as locally.
        rpc Delete(NodeRequest) returns (Empty);

        // Engine.append + write-back; returns units appended.
        rpc Append(AppendRequest) returns (UnitsResponse);
      }

      message Empty {}

      // Engine.name / Engine.validates_syntax.
      message DescribeRequest {}

      message DescribeResponse {
        // Human-readable engine name (e.g. "java", "typescript").
        string name = 1;
        // Whether validate()/replace() reliably reject malformed edits.
        bool validates_syntax = 2;
      }

      // Mirrors base.Located, minus the engine-native node/parent/tree/index fields,
      // which never leave the engine process.
      message NodeLocation {
        // Fully-qualified, engine-assigned id (name/hash path from the root) — the
        // sole handle used to address the node in every other call.
        string node_id = 1;
        // Engine-reported node type name.
        string node_type = 2;
        // Simple name, if the node carries one.
        optional string name = 3;
        // 1-based inclusive line span.
        int32 lineno = 4;
        int32 end_lineno = 5;
        // Type name of the parent node, absent at the top level.
        optional string parent_type = 6;
        // Whether `read`-style tools should descend into children instead of
        // returning this node's full source.
        bool expandable = 7;
      }

      message ListRequest {
        // Absolute path to the (possibly not-yet-existing) file.
        string path = 1;
      }

      message ListResponse {
        repeated NodeLocation nodes = 1;
      }

      message ValidateRequest {
        // Absolute path; content is read by the engine itself.
        string path = 1;
      }

      message ValidateResponse {
        // Error message if the file's content is malformed, absent if valid.
        optional string error = 1;
      }

      message IsDefinitionRequest {
        string node_type = 1;
      }

      message IsDefinitionResponse {
        bool is_definition = 1;
      }

      // Common request shape for read-only per-node calls (Signature/Docstring/
      // NodeCode) and for Delete.
      message NodeRequest {
        string path = 1;
        string node_id = 2;
      }

      message SignatureResponse {
        string signature = 1;
      }

      message DocstringResponse {
        // Absent if the format has no docstring concept, or the node has none.
        optional string docstring = 1;
      }

      message NodeCodeResponse {
        string code = 1;
      }

      message ReplaceRequest {
        string path = 1;
        string node_id = 2;
        string code = 3;
      }

      message InsertRequest {
        string path = 1;
        string node_id = 2;
        string code = 3;
        // "before" or "after".
        string position = 4;
      }

      message AppendRequest {
        string path = 1;
        string code = 2;
      }

      message UnitsResponse {
        int32 units = 1;
      }
  reason: "Proto auf dateibasiertes, zustandsfreies Design entsprechend der Bestätigung umstellen"
```
/answer 7a0a-19 allow
Control Request:
```yaml
id: 7a0a-1a
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
      error: No node matched the given text; a node selector (id) is required.
```
/answer 7a0a-1a allow
Control Request:
```yaml
id: 7a0a-1b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
  node_type: .*
  reason: "Aktuellen Dateiinhalt prüfen, da vorheriger Edit fehlschlug"
```
/answer 7a0a-1b allow
Control Request:
```yaml
id: 7a0a-1c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
      nodes:
      - id: file
        type: file
        code: |+
          syntax = "proto3";

          // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
          // back-ends for languages other than Python (typically tree-sitter based)
          // can be plugged into the mcpc AST tools out-of-process.
          //
          // Stateful, mirroring the local usage: the remote engine holds the actual
          // parsed tree, keyed by an opaque `tree_handle` returned from Parse/EmptyTree
          // (stands in for the local Tree.raw). Nodes are addressed within that tree by
          // `node_id`, the same opaque, engine-assigned handle carried by Located.node
          // ("for use only as tree.* arguments, never introspect their internals").
          // Replace/Insert/Delete/Append mutate the remote tree in place, exactly as
          // their local Engine counterparts do — the resulting source is fetched
          // separately via Serialize, only when actually needed (matching
          // AstCache.save's serialize-on-demand behaviour).
          package xy.ai.mcpc.ast.v1;

          option go_package = "xy.ai/mcpc/ast/v1;astv1";

          import "google/protobuf/empty.proto";

          service AstEngine {
            // Static facts about the engine (Engine.name, Engine.validates_syntax).
            rpc Describe(DescribeRequest) returns (DescribeResponse);

            // Engine.parse: parse source into a tree, raising on error (gRPC status).
            rpc Parse(ParseRequest) returns (ParseResponse);

            // Engine.empty_tree: used when appending to a not-yet-existing file.
            rpc EmptyTree(EmptyTreeRequest) returns (ParseResponse);

            // Engine.serialize: render the tree back to source text, on demand.
            rpc Serialize(TreeRequest) returns (SerializeResponse);

            // Engine.validate. Stateless: no tree required.
            rpc Validate(ValidateRequest) returns (ValidateResponse);

            // Engine.locate_all: flatten the tree into every addressable node, in
            // document order. Also (re-)usable after a mutation to refresh ids.
            rpc LocateAll(TreeRequest) returns (LocateAllResponse);

            // Engine.is_definition. Stateless.
            rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

            // Engine.signature (falls back to Engine.default_signature internally).
            rpc Signature(NodeRequest) returns (SignatureResponse);

            // Engine.docstring.
            rpc Docstring(NodeRequest) returns (DocstringResponse);

            // Engine.node_code.
            rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

            // Engine.replace: mutates the tree in place, returns nothing.
            rpc Replace(ReplaceRequest) returns (google.protobuf.Empty);

            // Engine.insert: mutates the tree in place; returns units inserted.
            rpc Insert(InsertRequest) returns (UnitsResponse);

            // Engine.delete: mutates the tree in place, returns nothing.
            rpc Delete(NodeRequest) returns (google.protobuf.Empty);

            // Engine.append: mutates the tree in place; returns units appended.
            rpc Append(AppendRequest) returns (UnitsResponse);

            // Not part of the Engine ABC: releases a tree_handle's server-side state.
            // Local engines have no such lifecycle (garbage-collected with the
            // process); a remote engine needs it to avoid leaking held trees.
            rpc ReleaseTree(TreeRequest) returns (google.protobuf.Empty);
          }

          // Engine.name / Engine.validates_syntax.
          message DescribeRequest {}

          message DescribeResponse {
            // Human-readable engine name (e.g. "java", "typescript").
            string name = 1;
            // Whether validate()/replace() reliably reject malformed edits.
            bool validates_syntax = 2;
          }

          // Opaque handle standing in for the local Tree.raw; valid only against the
          // engine instance that issued it, until ReleaseTree.
          message TreeRequest {
            string tree_handle = 1;
          }

          message ParseRequest {
            string source = 1;
            // Absolute path the source was loaded from; absent for snippets/new files.
            optional string path = 2;
          }

          message EmptyTreeRequest {
            optional string path = 1;
          }

          message ParseResponse {
            string tree_handle = 1;
          }

          message SerializeResponse {
            string source = 1;
          }

          message ValidateRequest {
            string source = 1;
          }

          message ValidateResponse {
            // Error message if `source` is malformed, absent if valid.
            optional string error = 1;
          }

          // Mirrors base.Located, minus tree/parent/index (server-internal); `node_id`
          // is the same opaque, engine-assigned handle used as the `node` argument of
          // Signature/Docstring/NodeCode/Replace/Insert/Delete.
          message NodeLocation {
            // Opaque per-tree node handle (doubles as the human-facing, unique id
            // exposed to ast_* tools, e.g. "MyClass.method").
            string node_id = 1;
            // Engine-reported node type name.
            string node_type = 2;
            // Simple name, if the node carries one.
            optional string name = 3;
            // 1-based inclusive line span.
            int32 lineno = 4;
            int32 end_lineno = 5;
            // Type name of the parent node, absent at the top level.
            optional string parent_type = 6;
            // Whether `read`-style tools should descend into children instead of
            // returning this node's full source.
            bool expandable = 7;
          }

          message LocateAllResponse {
            repeated NodeLocation nodes = 1;
          }

          message IsDefinitionRequest {
            string node_type = 1;
          }

          message IsDefinitionResponse {
            bool is_definition = 1;
          }

          // Common request shape for read-only per-node calls (Signature/Docstring/
          // NodeCode) and for Delete: the node is addressed within its tree_handle.
          message NodeRequest {
            string tree_handle = 1;
            string node_id = 2;
          }

          message SignatureResponse {
            string signature = 1;
          }

          message DocstringResponse {
            // Absent if the format has no docstring concept, or the node has none.
            optional string docstring = 1;
          }

          message NodeCodeResponse {
            string code = 1;
          }

          message ReplaceRequest {
            string tree_handle = 1;
            string node_id = 2;
            string code = 3;
          }

          message InsertRequest {
            string tree_handle = 1;
            string node_id = 2;
            string code = 3;
            // "before" or "after".
            string position = 4;
          }

          message AppendRequest {
            string tree_handle = 1;
            string code = 2;
          }

          message UnitsResponse {
            int32 units = 1;
          }
```
/answer 7a0a-1c allow
Control Request:
```yaml
id: 7a0a-1d
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto
    id: file
    source: |
      syntax = "proto3";

      // RPC contract mirroring xy.ai.mcpc.tools.ast.base.Engine, so that AST
      // back-ends for languages other than Python (typically tree-sitter based)
      // can be plugged into the mcpc AST tools out-of-process.
      //
      // File-based and stateless: the engine has its own filesystem access and
      // reads/re-parses `path` itself on every call, mutates and writes the file
      // back out itself, exactly as the local Engine + AstCache do together. No
      // source text and no tree/node handle ever crosses the wire; a node is
      // addressed purely by `node_id` (as produced by List) within `path`.
      // Engines may cache parsed trees internally (e.g. keyed by path + mtime/
      // content hash) as a pure performance optimisation, invisible to this
      // protocol. Callers who need the resulting text use NodeCode directly.
      package xy.ai.mcpc.ast.v1;

      option go_package = "xy.ai/mcpc/ast/v1;astv1";

      service AstEngine {
        // Static facts about the engine (Engine.name, Engine.validates_syntax).
        rpc Describe(DescribeRequest) returns (DescribeResponse);

        // Engine.parse + Engine.locate_all (or Engine.empty_tree if `path` doesn't
        // exist yet): every addressable node of `path`, in document order.
        rpc List(ListRequest) returns (ListResponse);

        // Engine.validate against `path`'s current on-disk content.
        rpc Validate(ValidateRequest) returns (ValidateResponse);

        // Engine.is_definition. Stateless, no file involved.
        rpc IsDefinition(IsDefinitionRequest) returns (IsDefinitionResponse);

        // Engine.signature (falls back to Engine.default_signature internally).
        rpc Signature(NodeRequest) returns (SignatureResponse);

        // Engine.docstring.
        rpc Docstring(NodeRequest) returns (DocstringResponse);

        // Engine.node_code: the only way to retrieve source text over this API.
        rpc NodeCode(NodeRequest) returns (NodeCodeResponse);

        // Engine.replace + write-back. No return value, as locally.
        rpc Replace(ReplaceRequest) returns (Empty);

        // Engine.insert + write-back; returns units inserted.
        rpc Insert(InsertRequest) returns (UnitsResponse);

        // Engine.delete + write-back. No return value, as locally.
        rpc Delete(NodeRequest) returns (Empty);

        // Engine.append + write-back; returns units appended.
        rpc Append(AppendRequest) returns (UnitsResponse);
      }

      message Empty {}

      // Engine.name / Engine.validates_syntax.
      message DescribeRequest {}

      message DescribeResponse {
        // Human-readable engine name (e.g. "java", "typescript").
        string name = 1;
        // Whether validate()/replace() reliably reject malformed edits.
        bool validates_syntax = 2;
      }

      // Mirrors base.Located, minus the engine-native node/parent/tree/index fields,
      // which never leave the engine process.
      message NodeLocation {
        // Fully-qualified, engine-assigned id (name/hash path from the root) — the
        // sole handle used to address the node in every other call.
        string node_id = 1;
        // Engine-reported node type name.
        string node_type = 2;
        // Simple name, if the node carries one.
        optional string name = 3;
        // 1-based inclusive line span.
        int32 lineno = 4;
        int32 end_lineno = 5;
        // Type name of the parent node, absent at the top level.
        optional string parent_type = 6;
        // Whether `read`-style tools should descend into children instead of
        // returning this node's full source.
        bool expandable = 7;
      }

      message ListRequest {
        // Absolute path to the (possibly not-yet-existing) file.
        string path = 1;
      }

      message ListResponse {
        repeated NodeLocation nodes = 1;
      }

      message ValidateRequest {
        // Absolute path; content is read by the engine itself.
        string path = 1;
      }

      message ValidateResponse {
        // Error message if the file's content is malformed, absent if valid.
        optional string error = 1;
      }

      message IsDefinitionRequest {
        string node_type = 1;
      }

      message IsDefinitionResponse {
        bool is_definition = 1;
      }

      // Common request shape for read-only per-node calls (Signature/Docstring/
      // NodeCode) and for Delete.
      message NodeRequest {
        string path = 1;
        string node_id = 2;
      }

      message SignatureResponse {
        string signature = 1;
      }

      message DocstringResponse {
        // Absent if the format has no docstring concept, or the node has none.
        optional string docstring = 1;
      }

      message NodeCodeResponse {
        string code = 1;
      }

      message ReplaceRequest {
        string path = 1;
        string node_id = 2;
        string code = 3;
      }

      message InsertRequest {
        string path = 1;
        string node_id = 2;
        string code = 3;
        // "before" or "after".
        string position = 4;
      }

      message AppendRequest {
        string path = 1;
        string code = 2;
      }

      message UnitsResponse {
        int32 units = 1;
      }
  reason: "Proto vollständig auf dateibasiertes, zustandsfreies Design umstellen"
```
/answer 7a0a-1d allow
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 84521, in: 177, out: 30125, reason: 43, read: 598669, write: 54219
ReasoningToken: 43
Text:
 Datei ist noch im (verworfenen) `tree_handle`-Stand. Ich ersetze sie jetzt vollständig mit dem dateibasierten Design.
Fertig. `/home/user/xyan/xy.ai.workbench/ast-engines/ast_engine.proto` bildet nun `Engine` 1:1 ab: dateibasiert, kein Quelltext/Handle über die Leitung, `Replace`/`Delete` liefern `Empty`, `Insert`/`Append` nur den Zähler, Text nur explizit über `NodeCode`.
Erklär mir nun die Ursache deiner Verwirrung.
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 85576, in: 179, out: 31014, reason: 23, read: 652888, write: 54383
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 23
Meine Verwirrung hatte mehrere, sich überlagernde Ursachen:

1. **Ich habe von der falschen Prämisse aus gestartet.** Ich kannte nur `base.py` (die abstrakte `Engine`-Klasse) und habe daraus spekulativ ein RPC-Design abgeleitet, statt vorher zu prüfen, wie `Engine` tatsächlich *aufgerufen* wird (`core.py`, `replace.py`, `script.py`). Dadurch habe ich Annahmen getroffen, die dem Interface nicht entsprachen.

2. **Ich habe „zustandslos" mit „Werte werden über die Leitung geschickt" verwechselt.** Als du „zustandsfrei" sagtest, bin ich reflexhaft zu „dann muss der volle Quelltext bei jedem Call hin und zurück" gesprungen – das ist ein verbreitetes, aber falsches Muster (REST-Denken). Ich habe nicht in Betracht gezogen, dass „zustandsfrei" hier schlicht heißt: *die Engine hat eigenen Dateisystemzugriff und liest/schreibt selbst* – ohne dass überhaupt Payload-Daten des Inhalts transportiert werden müssen.

3. **Ich habe `None`-Rückgaben der ABC nicht ernst genommen.** `Engine.replace/delete -> None` war ein klares Signal, dass niemand den Code zurückbraucht. Ich habe das erst als „unvollständige lokale Optimierung" abgetan und versucht, es RPC-technisch „aufzuwerten", statt die Signatur als bewusste Designentscheidung zu akzeptieren und die Frage zu stellen, *wie* der Aufrufer dann überhaupt an den Code kommt (Antwort: über `NodeCode`, separat, bei Bedarf).

4. **Ich habe zu spät gefragt.** Statt bei der ersten Unsicherheit („wie kommt der Text vom Server zum Client?") nachzufragen, habe ich mehrfach eigene, unbegründete Architekturvarianten gebaut und erst nach wiederholter Korrektur den entscheidenden Punkt (Datei-basierter Zugriff) erfragt.

Kurz: Fehlende Vorab-Recherche der Aufrufer-Seite + eine falsche Default-Annahme über „zustandsfrei" + zu spätes Nachfragen statt Spekulieren.
Okay warte. Dann hat dich Zustandslos verwirrt. Die Zieldateien sind bereits der Zustand. Sie sind nicht Teile vom Server aber der CLient kennt sie auch nicht. Sie lagern in einer unabhängigen persistenz. Der Server ist dadurch Zustandslos aber durch die Peristenz auch eigentlich nicht.
Result Stats: id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, total: 87108, in: 181, out: 31520, reason: 0, read: 707271, write: 55407
SystemInit:  id=1ba1802d-99d5-4c59-9df0-228dff54f5b8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Ja, genau das war der Kernpunkt meiner Verwirrung.

„Zustandslos" bezog sich auf den **Server-Prozess** (keine Session, kein In-Memory-Handle, kein Gedächtnis zwischen zwei RPC-Calls) – nicht darauf, dass es keinen Zustand gäbe. Der Zustand existiert sehr wohl, nur eben nicht beim Client und nicht im Server-Speicher, sondern in einer dritten, unabhängigen Instanz: der Datei auf der Platte (der Persistenz).

- Der **Client** kennt den Dateiinhalt nicht (schickt nur `path` + `node_id` + `code`).
- Der **Server** hält ihn nicht im Speicher (jeder Call liest/schreibt die Datei frisch, Cache ist nur Performance-Detail).
- Die **Persistenz** (Dateisystem) ist der eigentliche, dauerhafte Zustand – unabhängig von beiden.

Ich habe „zustandslos" fälschlich als „es gibt gar keinen greifbaren Zustand, also muss der Inhalt explizit durch die Leitung" interpretiert, statt zu erkennen, dass der Zustand einfach an einem dritten Ort liegt, auf den der Server referenziell (via `path`) zugreift. Das ist der Unterschied zwischen „stateless service" und „stateless *data*" – ich habe beides vermischt.
