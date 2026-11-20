Die auf Tree Sitter basierende Markdown AST-Engine `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py` soll analog `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py` auf eine bessere RPC basierte Engine in `/home/user/xyan/xy.ai.workbench/ast-engines/markdown` umgestellt werden.
SystemInit:  id=8d367516-0e0c-4674-80a5-34402faaa645, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die bestehenden Dateien.
Control Request:
```yaml
id: 2d71-ba-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
  reason: "Outline der Markdown- und Java-Engine, um die Struktur zu verstehen"
```
/answer 2d71-ba-0 allow
Control Request:
```yaml
id: 2d71-bb-ba
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
        nodes:
        - id: Bficuz|LjorUp
          excert: "\"Markdown tree-sitter engine: a fixed, MDAST-inspired node set.\\n\\nTree-sitter'…"
        - id: Q1lGPc|SlXmI5
          excert: from __future__ import annotations
        - id: rULMC2|CnEXKe
          excert: "__all__ = ['MarkdownEngine']"
        - id: md_level
          excert: "def _md_level(node: Any) -> int:"
          docstring: Heading level (1-6) from an ``atx_h<N>_marker`` or ``setext_h<N>_underline`` ch…
        - id: md_flatten
          excert: "def _md_flatten(node: Any, out: list[Any]) -> None:"
          docstring: "Collect nodes in document order, transparently unwrapping tree-sitter's own 'se…"
        - id: md_split_pages
          excert: "def _md_split_pages(flat: list[Any]) -> list[list[Any]]:"
          docstring: "Split a flat block list on 'thematic_break' ('---'), dropping empty pages."
        - id: MdSection
          excert: "class _MdSection:"
          docstring: "Builder for one rebuilt heading section, before it is frozen into a ``_SynthNod…"
          children:
          - id: MdSection.oLfXbj|S4DmQY
            excert: "'Builder for one rebuilt heading section, before it is frozen into a ``_SynthNo…"
          - id: MdSection.init
            excert: "def __init__(self, level: int, heading: Any) -> None:"
        - id: md_nest_headings
          excert: "def _md_nest_headings(items: list[Any]) -> list[Any]:"
          docstring: "Rebuild H1-H6 nesting from a flat block list, independent of tree-sitter's own …"
        - id: md_finalize
          excert: "def _md_finalize(nodes: list[Any], source: bytes) -> list[Any]:"
          docstring: Freeze ``_MdSection`` builders (and their descendants) into ``_SynthNode('secti…
        - id: md_root_children
          excert: "def _md_root_children(root_node: Any, source: bytes) -> list[Any]:"
          docstring: "Top-level children of a Markdown file: Pages if the file uses '---', else Secti…"
        - id: MarkdownEngine
          excert: "class MarkdownEngine(TreeSitterEngine):"
          docstring: Tree-sitter Markdown restructured into the fixed node set above.
          children:
          - id: MarkdownEngine.AvTNlk|zKty4h
            excert: '''Tree-sitter Markdown restructured into the fixed node set above.'''
          - id: MarkdownEngine.init
            excert: "def __init__(self) -> None:"
          - id: MarkdownEngine.locate_all
            excert: "def locate_all(self, tree: Tree) -> list[Located]:"
          - id: MarkdownEngine.addressable
            excert: "@staticmethod def _addressable(child: Any, depth: int) -> bool:"
          - id: MarkdownEngine.signature
            excert: "def signature(self, node: Any, limit: int=30) -> str:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
        nodes:
        - id: zrXNbR|3dRepm
          excert: "\"Java engine: JavaParser-based ``ast-java`` process, driven over RPC/PIC.\\n\\nRe…"
        - id: ILN4fu|sCWK6E
          excert: from __future__ import annotations
        - id: JpPcRr|tupArz
          excert: "__all__ = ['JavaEngine']"
        - id: workspace_root
          excert: "def _workspace_root() -> Path:"
        - id: vz3nHu|7E9CVU
          excert: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'java'
        - id: JavaProcess
          excert: "class JavaProcess(RpcProcess):"
          docstring: Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build`…
          children:
          - id: JavaProcess.0qSEGQ|3JgJWY
            excert: '''Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build…'
          - id: JavaProcess.default_port
            excert: "def default_port(self) -> int:"
          - id: JavaProcess.cwd
            excert: "def cwd(self) -> str | None:"
          - id: JavaProcess.command
            excert: "def command(self, port: int) -> list[str]:"
          - id: JavaProcess.java_tool
            excert: "def _java_tool(self, name: str) -> str:"
          - id: JavaProcess.classpath
            excert: "def _classpath(self) -> str:"
        - id: g8i7OL|HTI1Ea
          excert: _PROCESS = JavaProcess()
        - id: JavaEngine
          excert: "class JavaEngine(RpcEngine):"
          docstring: "Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process."
          children:
          - id: JavaEngine.1qmNSs|TC4ByX
            excert: "'Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process.'"
          - id: JavaEngine.init
            excert: "def __init__(self) -> None:"
```
/answer 2d71-bb-ba allow
ReasoningToken: 13
Control Request:
```yaml
id: 2d71-bc-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    pattern: ^(?!.*(/target/|/node_modules/|/\.git/|/build/|/\.gradle/)).*$
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: \.py$
  reason: Struktur der Engines und des RPC-Pakets erfassen
```
/answer 2d71-bc-0 allow
Control Request:
```yaml
id: 2d71-bd-bc
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      entries:
      - ".: (1 file below)"
      - "./java: (4 files below)"
      - "./java/.bin/xy/ai: (64 files below)"
      - "./java/.settings: (2 files below)"
      - "./java/libs: (6 files below)"
      - "./java/scripts: (1 file below)"
      - "./java/src/xy/ai: (57 files below)"
      - "./markdown: (7 files below)"
      - "./markdown/src/xy/ai: (62 files below)"
      - "./rust: (6 files below)"
      - "./rust/.bin: (2 files below)"
      - "./rust/.bin/debug: (5 files below)"
      - "./rust/.bin/debug/.fingerprint: (214 files below)"
      - "./rust/.bin/debug/deps: (108 files below)"
      - "./rust/.bin/debug/incremental: (542 files below)"
      - "./rust/.bin/doc: (5 files below)"
      - "./rust/.bin/doc/search.index: (169 files below)"
      - "./rust/.bin/doc/src: (52 files below)"
      - "./rust/.bin/doc/static.files: (34 files below)"
      - "./rust/.bin/doc/syn: (1317 files below)"
      - "./rust/.bin/doc/trait.impl: (40 files below)"
      - "./rust/.bin/doc/type.impl: (2 files below)"
      - "./rust/.bin/release: (5 files below)"
      - "./rust/.bin/release/.fingerprint: (154 files below)"
      - "./rust/.bin/release/deps: (80 files below)"
      - "./rust/.cargo: (1 file below)"
      - "./rust/src: (1 file below)"
      - "./rust/src/engine: (8 files below)"
      - "./rust/src/generated: (1 file below)"
      - "./rust/src/generated/engine: (112 files below)"
      warnings:
      - Result reduced to directories up to depth 4 because the number of directories with matches (370) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      entries:
      - ".:"
      - __init__.py
      - base.py
      - common.py
      - core.py
      - create.py
      - delete.py
      - edit.py
      - edit_block.py
      - edit_marks.py
      - find.py
      - imports.py
      - insert.py
      - outcomes.py
      - outline.py
      - read.py
      - replace.py
      - retrieve.py
      - script.py
      - validate.py
      - "./generic:"
      - __init__.py
      - _css.py
      - _engine.py
      - _html.py
      - _java.py
      - _javascript.py
      - _markdown.py
      - _text.py
      - _xml.py
      - _yaml.py
      - "./python:"
      - __init__.py
      - _comments.py
      - _engine.py
      - _imports.py
      - _nodes.py
      - reformat_cli.py
      - "./rpc:"
      - __init__.py
      - _engine.py
      - _java.py
      - _process.py
      - _rust.py
```
/answer 2d71-bd-bc allow
Control Request:
```yaml
id: 2d71-be-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: ^(?!.*/\.bin/).*$
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    recursive: false
  reason: Vorhandenen Stand von ast-engines/markdown und java (ohne Build-Artefakte) ansehen
```
/answer 2d71-be-0 allow
Control Request:
```yaml
id: 2d71-bf-be
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown
      entries:
      - ".: (7 files)"
      - .gitignore
      - "./src/xy/ai/mcpc/ast/engine: (13 files)"
      - AddressableNode.ts
      - "./src/xy/ai/mcpc/ast/openapi: (4 files)"
      - AppendInfoNodesValidateClient.ts
      - "./src/xy/ai/mcpc/ast/openapi/components: (6 files)"
      - CandidatesList.ts
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/nodeid/insert/post/json: (2 files)"
      - InsertRequest.ts
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/post/json: (1 file)"
      - LocateRequest.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/append: (1 file)"
      - AppendResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code200/json: (2 files)"
      - AppendResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code422/json: (1 file)"
      - AppendResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/info: (1 file)"
      - InfoResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/info/code200/json: (2 files)"
      - EngineInfo.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes: (1 file)"
      - NodesResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code200/json: (3 files)"
      - LocateResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code422/json: (1 file)"
      - NodesResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid: (1 file)"
      - NodesNodeIdResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code200/json: (1 file)"
      - NodesNodeIdResponseCode200Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code404/json: (1 file)"
      - NodesNodeIdResponseCode404Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code409/json: (1 file)"
      - NodesNodeIdResponseCode409Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code422/json: (1 file)"
      - NodesNodeIdResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete: (1 file)"
      - NodesNodeIdDeleteResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code200/json: (1 file)"
      - NodesNodeIdDeleteResponseCode200Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code404/json: (1 file)"
      - NodesNodeIdDeleteResponseCode404Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code409/json: (1 file)"
      - NodesNodeIdDeleteResponseCode409Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code422/json: (1 file)"
      - NodesNodeIdDeleteResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert: (1 file)"
      - NodesNodeIdInsertResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code200/json: (2 files)"
      - InsertResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code404/json: (1 file)"
      - NodesNodeIdInsertResponseCode404Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code409/json: (1 file)"
      - NodesNodeIdInsertResponseCode409Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code422/json: (1 file)"
      - NodesNodeIdInsertResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace: (1 file)"
      - NodesNodeIdReplaceResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code200/json: (1 file)"
      - NodesNodeIdReplaceResponseCode200Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code404/json: (1 file)"
      - NodesNodeIdReplaceResponseCode404Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code409/json: (1 file)"
      - NodesNodeIdReplaceResponseCode409Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code422/json: (1 file)"
      - NodesNodeIdReplaceResponseCode422Json.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/validate: (1 file)"
      - ValidateResponse.ts
      - "./src/xy/ai/mcpc/ast/openapi/response/validate/code200/json: (2 files)"
      - ValidateResponse.ts
      warnings:
      - Result reduced because the total number of entries (69) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      entries:
      - ".: (4 files)"
      - .classpath
      - "./.settings: (2 files)"
      - org.eclipse.core.resources.prefs
      - "./libs: (6 files)"
      - jackson-annotations-3.0-rc5.jar
      - "./scripts: (1 file)"
      - openapi_generate.sh
      - "./src/xy/ai/mcpc/ast/engine: (8 files)"
      - AddressableNode.java
      - "./src/xy/ai/mcpc/ast/openapi: (4 files)"
      - AppendInfoNodesValidateClient.java
      - "./src/xy/ai/mcpc/ast/openapi/components: (6 files)"
      - CandidatesList.java
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/nodeid/insert/post/json: (2 files)"
      - InsertRequest.java
      - "./src/xy/ai/mcpc/ast/openapi/request/nodes/post/json: (1 file)"
      - LocateRequest.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append: (1 file)"
      - AppendResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code200/json: (2 files)"
      - AppendResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/append/code422/json: (1 file)"
      - AppendResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/info: (1 file)"
      - InfoResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/info/code200/json: (2 files)"
      - EngineInfo.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes: (1 file)"
      - NodesResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code200/json: (3 files)"
      - LocateResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/code422/json: (1 file)"
      - NodesResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid: (1 file)"
      - NodesNodeIdResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code200/json: (1 file)"
      - NodesNodeIdResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code404/json: (1 file)"
      - NodesNodeIdResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code409/json: (1 file)"
      - NodesNodeIdResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code422/json: (1 file)"
      - NodesNodeIdResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete: (1 file)"
      - NodesNodeIdDeleteResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code200/json: (1 file)"
      - NodesNodeIdDeleteResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code404/json: (1 file)"
      - NodesNodeIdDeleteResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code409/json: (1 file)"
      - NodesNodeIdDeleteResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code422/json: (1 file)"
      - NodesNodeIdDeleteResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert: (1 file)"
      - NodesNodeIdInsertResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code200/json: (2 files)"
      - InsertResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code404/json: (1 file)"
      - NodesNodeIdInsertResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code409/json: (1 file)"
      - NodesNodeIdInsertResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code422/json: (1 file)"
      - NodesNodeIdInsertResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace: (1 file)"
      - NodesNodeIdReplaceResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code200/json: (1 file)"
      - NodesNodeIdReplaceResponseCode200Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code404/json: (1 file)"
      - NodesNodeIdReplaceResponseCode404Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code409/json: (1 file)"
      - NodesNodeIdReplaceResponseCode409Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code422/json: (1 file)"
      - NodesNodeIdReplaceResponseCode422Json.java
      - "./src/xy/ai/mcpc/ast/openapi/response/validate: (1 file)"
      - ValidateResponse.java
      - "./src/xy/ai/mcpc/ast/openapi/response/validate/code200/json: (2 files)"
      - ValidateResponse.java
      warnings:
      - Result reduced because the total number of entries (70) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      entries:
      - ".:"
      - openapi.yaml
```
/answer 2d71-bf-be allow
ReasoningToken: 32
Control Request:
```yaml
id: 2d71-c0-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
    regexp: (?s).*
  reason: "Java-RPC-Engine, Rust-Engine und RPC-Basis lesen"
```
/answer 2d71-c0-0 allow
Control Request:
```yaml
id: 2d71-c1-c0
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
        nodes:
        - id: JavaEngine.init.dYiadH|KMutDo
          code: "super().__init__('java', _PROCESS)"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
        nodes:
        - id: RustEngine.init.ErdR77|mJ96xk
          code: "super().__init__('rust', _PROCESS)"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
        nodes:
        - id: VMvyha|JK431n
          code: "__all__ = ['RpcEngine', 'RpcProcess', 'JavaEngine', 'RustEngine']"
```
/answer 2d71-c1-c0 allow
Control Request:
```yaml
id: 2d71-c2-0
toolName: python
arguments:
  script: |-
    import os,subprocess
    base='/home/user/xyan/xy.ai.workbench/ast-engines'
    for d in ['markdown','java']:
        for r,ds,fs in os.walk(os.path.join(base,d)):
            ds[:]=[x for x in ds if x not in ('.bin','node_modules','openapi')]
            for f in fs: print(os.path.join(r,f))
    print(subprocess.run('cd /home/user/xyan/xy.ai.workbench && git status --short | head -30 && git log --oneline | head -8 && cat ast-engines/markdown/.gitignore && ls -la ast-engines/markdown ast-engines/java',shell=True,capture_output=True,text=True).stdout)
  reason: "Dateien (ohne Build-Artefakte) im markdown-, java-Ordner auflisten und die Hauptdateien lesen"
```
/answer 2d71-c2-0 deny verwende kein Git
ReasoningToken: 67
Control Request:
```yaml
id: 2d71-c3-0
toolName: python
arguments:
  script: |-
    d='/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/'
    for f in ['_java.py','_rust.py','_engine.py','_process.py','__init__.py']:
        print('=====',f);print(open(d+f).read())
  reason: Quelltext der Java-RPC-Anbindung und der RPC-Basis lesen
```
/answer 2d71-c3-0 deny benutze AST dafür
Text:
 Verstanden: kein Git, und Lesen läuft über die AST-Tools. Ich lese die RPC-Dateien jetzt darüber.
Control Request:
```yaml
id: 2d71-c4-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
    ids:
    - zrXNbR|3dRepm
    - ILN4fu|sCWK6E
    - JpPcRr|tupArz
    - workspace_root
    - vz3nHu|7E9CVU
    - JavaProcess
    - g8i7OL|HTI1Ea
    - JavaEngine
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_process.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
  reason: Java-RPC-Anbindung vollständig per AST lesen
```
/answer 2d71-c4-0 allow
Control Request:
```yaml
id: 2d71-c5-c4
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_process.py
        nodes:
        - id: sJdz87|kxB6zB
          excert: '"Base class for lazily-started AST engine subprocesses speaking the RPC/PIC\nHT…'
        - id: tHArjg|wBrnUS
          excert: from __future__ import annotations
        - id: iU20Uw|vXEFts
          excert: logger = logging.getLogger('xy.ai.mcpc.tools.ast.rpc')
        - id: RpcProcess
          excert: "class RpcProcess(ABC):"
          docstring: "Supervises a single long-lived RPC engine subprocess, started on first use. Thr…"
          children:
          - id: RpcProcess.DXlVqi|lLI6Mv
            excert: "'Supervises a single long-lived RPC engine subprocess, started on first use.\\n\\…"
          - id: RpcProcess.init
            excert: "def __init__(self) -> None:"
          - id: RpcProcess.default_port
            excert: "@abstractmethod def default_port(self) -> int:"
            docstring: TCP port the engine should listen on.
          - id: RpcProcess.command
            excert: "@abstractmethod def command(self, port: int) -> list[str]:"
            docstring: "Argv that launches the engine, listening on ``port``."
          - id: RpcProcess.cwd
            excert: "def cwd(self) -> str | None:"
            docstring: Working directory for the subprocess; ``None`` inherits the caller's.
          - id: RpcProcess.base_url
            excert: "def base_url(self) -> str:"
            docstring: "URL of the (lazily started) engine's API root, e.g. ``http://127.0.0.1:8787``. …"
          - id: RpcProcess.ensure_running
            excert: "def _ensure_running(self) -> None:"
          - id: RpcProcess.wait_until_ready
            excert: "def _wait_until_ready(self, port: int) -> None:"
          - id: RpcProcess.terminate
            excert: "def _terminate(self) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: BQIdfu|0WxhPP
          excert: '"Generic :class:`Engine` back-end for any parser exposed via the RPC/PIC\nHTTP …'
        - id: Da4IMM|t2aMCb
          excert: from __future__ import annotations
        - id: DLMY8E|Z2twV0
          excert: from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest…
        - id: BshuIz|Kr3xsO
          excert: from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
        - id: ibsDCF|76lOk2
          excert: "__all__ = ['RpcEngine']"
        - id: RpcNode
          excert: "@dataclass class _RpcNode:"
          docstring: One node as reported by ``POST /nodes``. Carries everything :meth:`RpcEngine.si…
        - id: RpcEngine
          excert: "class RpcEngine(Engine):"
          docstring: One RPC/PIC engine process exposed through the common :class:`Engine` API. Stru…
          children:
          - id: RpcEngine.95ZeHe|fDr6kO
            excert: '"One RPC/PIC engine process exposed through the common :class:`Engine` API.\n\n…'
          - id: RpcEngine.init
            excert: "def __init__(self, symbol: str, process: RpcProcess) -> None:"
          - id: RpcEngine.rpc
            excert: "def _rpc(self) -> AppendInfoNodesValidateClientImpl:"
          - id: RpcEngine.unwrap
            excert: "@staticmethod def _unwrap(response: Any) -> Any:"
            docstring: "Return a 200 response's typed body, or raise on any other status."
          - id: RpcEngine.MltvBv|wKemBs
            excert: '''# -- Engine ------------------------------------------------------------'''
          - id: RpcEngine.parse
            excert: "def parse(self, source: str, path: Path | None=None) -> Tree:"
          - id: RpcEngine.empty_tree
            excert: "def empty_tree(self, path: Path | None=None) -> Tree:"
          - id: RpcEngine.serialize
            excert: "def serialize(self, tree: Tree) -> str:"
          - id: RpcEngine.validate
            excert: "def validate(self, source: str) -> str | None:"
          - id: RpcEngine.is_definition
            excert: "def is_definition(self, node_type: str) -> bool:"
          - id: RpcEngine.locate_all
            excert: "def locate_all(self, tree: Tree) -> list[Located]:"
          - id: RpcEngine.signature
            excert: "def signature(self, node: _RpcNode) -> str:"
          - id: RpcEngine.docstring
            excert: "def docstring(self, node: _RpcNode) -> str | None:"
          - id: RpcEngine.node_code
            excert: "def node_code(self, node: _RpcNode) -> str:"
          - id: RpcEngine.replace
            excert: "def replace(self, loc: Located, code: str) -> None:"
          - id: RpcEngine.insert
            excert: "def insert(self, loc: Located, code: str, position: str) -> int:"
          - id: RpcEngine.delete
            excert: "def delete(self, loc: Located) -> None:"
          - id: RpcEngine.append
            excert: "def append(self, tree: Tree, code: str) -> int:"
          - id: RpcEngine.apply_imports
            excert: "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
        nodes:
        - id: reOPSB|2w0163
          excert: "\"Rust engine: syn-based ``xy_ai_ast_rust`` process, driven over RPC/PIC.\\n\\nSta…"
        - id: ILN4fu|sCWK6E
          excert: from __future__ import annotations
        - id: b5aB3x|qU00Yb
          excert: "__all__ = ['RustEngine']"
        - id: workspace_root
          excert: "def _workspace_root() -> Path:"
        - id: mnFJcA|J3zcEx
          excert: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'rust'
        - id: RustProcess
          excert: "class RustProcess(RpcProcess):"
          docstring: Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build``…
          children:
          - id: RustProcess.0qSEGQ|1SP6FA
            excert: '''Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build`…'
          - id: RustProcess.default_port
            excert: "def default_port(self) -> int:"
          - id: RustProcess.cwd
            excert: "def cwd(self) -> str | None:"
          - id: RustProcess.command
            excert: "def command(self, port: int) -> list[str]:"
        - id: F0cCaO|KRBgB9
          excert: _PROCESS = RustProcess()
        - id: RustEngine
          excert: "class RustEngine(RpcEngine):"
          docstring: "Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process."
          children:
          - id: RustEngine.pPxVPO|QPQZCQ
            excert: "'Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process.'"
          - id: RustEngine.init
            excert: "def __init__(self) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
        nodes:
        - id: Y0bbSV|7bGzjK
          excert: '''``Engine`` back-ends delegating to external parsers over the RPC/PIC HTTP\npro…'
        - id: Mx1s7V|VobIpP
          excert: from __future__ import annotations
        - id: VMvyha|JK431n
          excert: "__all__ = ['RpcEngine', 'RpcProcess', 'JavaEngine', 'RustEngine']"
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
        nodes:
        - id: zrXNbR|3dRepm
          type: statements
          code: "\"Java engine: JavaParser-based ``ast-java`` process, driven over RPC/PIC.\\n\\nReplaces the old tree-sitter :class:`~xy.ai.mcpc.tools.ast.generic._java.\\nJavaEngine` (kept around, but no longer wired up) with the real Java parser\\nunder ``ast-engines/java``, started lazily -- and kept running for the\\nserver's lifetime -- the same way ``ast-engines/java/run.sh`` does, minus the\\nshell: we invoke ``java`` (from ``$JAVA_HOME`` if set) directly against the\\nprebuilt classpath; building it (``run.sh --build``) stays a separate,\\nexternally-guaranteed step.\\n\""
        - id: ILN4fu|sCWK6E
          type: imports
          code: |-
            from __future__ import annotations
            import os
            from pathlib import Path
            from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
            from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
        - id: JpPcRr|tupArz
          type: statements
          code: "__all__ = ['JavaEngine']"
        - id: workspace_root
          type: FunctionDef
          code: |-
            def _workspace_root() -> Path:
                for ancestor in Path(__file__).resolve().parents:
                    if (ancestor / 'ast-engines').is_dir():
                        return ancestor
                raise RuntimeError(f"could not locate an 'ast-engines' directory above {__file__}")
        - id: vz3nHu|7E9CVU
          type: statements
          code: |-
            _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'java'
            _SRC_DIR = _ENGINE_DIR / 'src'
            _BIN_DIR = _ENGINE_DIR / '.bin'
            _LIBS_DIR = _ENGINE_DIR / 'libs'
            _MAIN_CLASS = 'xy.ai.mcpc.ast.engine.Main'
        - id: JavaProcess
          type: ClassDef
          excert: "class JavaProcess(RpcProcess):"
          docstring: Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build`…
          children:
          - id: JavaProcess.0qSEGQ|3JgJWY
            type: statements
            code: '''Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build``).'''
          - id: JavaProcess.default_port
            type: FunctionDef
            code: |-
              def default_port(self) -> int:
                  return int(os.environ.get('AST_JAVA_PORT', '8787'))
          - id: JavaProcess.cwd
            type: FunctionDef
            code: |-
              def cwd(self) -> str | None:
                  return str(_ENGINE_DIR)
          - id: JavaProcess.command
            type: FunctionDef
            code: |-
              def command(self, port: int) -> list[str]:
                  return [self._java_tool('java'), '-cp', self._classpath(), _MAIN_CLASS, str(port)]
          - id: JavaProcess.java_tool
            type: FunctionDef
            code: |-
              def _java_tool(self, name: str) -> str:
                  java_home = os.environ.get('JAVA_HOME')
                  if java_home:
                      candidate = Path(java_home) / 'bin' / name
                      if candidate.exists():
                          return str(candidate)
                  return name
          - id: JavaProcess.classpath
            type: FunctionDef
            code: |-
              def _classpath(self) -> str:
                  jars = sorted((str(p) for p in _LIBS_DIR.glob('*.jar') if not p.name.endswith(('-sources.jar', '-javadoc.jar'))))
                  return os.pathsep.join([str(_BIN_DIR), *jars])
        - id: g8i7OL|HTI1Ea
          type: statements
          code: _PROCESS = JavaProcess()
        - id: JavaEngine
          type: ClassDef
          excert: "class JavaEngine(RpcEngine):"
          docstring: "Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process."
          children:
          - id: JavaEngine.1qmNSs|TC4ByX
            type: statements
            code: |-
              'Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process.'
              validates_syntax = True
          - id: JavaEngine.init
            type: FunctionDef
            code: |-
              def __init__(self) -> None:
                  super().__init__('java', _PROCESS)
```
/answer 2d71-c5-c4 allow
ReasoningToken: 79
Control Request:
```yaml
id: 2d71-c6-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
    ids:
    - Bficuz|LjorUp
    - md_level
    - md_flatten
    - md_split_pages
    - MdSection
    - md_nest_headings
    - md_finalize
    - md_root_children
    - MarkdownEngine
  reason: Bestehende Markdown-Engine vollständig lesen
```
/answer 2d71-c6-0 allow
Control Request:
```yaml
id: 2d71-c7-c6
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_markdown.py
        nodes:
        - id: Bficuz|LjorUp
          type: statements
          code: "\"Markdown tree-sitter engine: a fixed, MDAST-inspired node set.\\n\\nTree-sitter's own ``section`` nesting only follows heading level and has no\\nnotion of ``---`` page breaks (and is inconsistent for setext headings), so\\nthe structure is rebuilt from scratch - Page > Heading H1-H6 > Paragraph/\\nScriptBlock, mirroring the custom MDAST model in\\n``xy.ai.workbench.editor.mdast.nodes.Elements`` - from a flattened,\\ndocument-order block list (see :func:`_md_root_children`).\\n\""
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
          excert: "class _MdSection:"
          docstring: "Builder for one rebuilt heading section, before it is frozen into a ``_SynthNod…"
          children:
          - id: MdSection.oLfXbj|S4DmQY
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
          excert: "class MarkdownEngine(TreeSitterEngine):"
          docstring: Tree-sitter Markdown restructured into the fixed node set above.
          children:
          - id: MarkdownEngine.AvTNlk|zKty4h
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
          - id: MarkdownEngine.signature
            type: FunctionDef
            code: |-
              def signature(self, node: Any, limit: int=30) -> str:
                  if node.type == 'section':
                      for child in node.named_children:
                          if child.type.endswith('heading'):
                              return self._clean_heading(child.text)
                  text = node.text.decode('utf-8', 'replace').strip()
                  return text if len(text) <= limit else text[:limit - 1] + '…'
```
/answer 2d71-c7-c6 allow
ReasoningToken: 211
Control Request:
```yaml
id: 2d71-c8-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/scripts
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/openapi
    recursive: false
  reason: Top-Level-Dateien und Engine-Quellen von markdown/java/rust gezielt auflisten
```
/answer 2d71-c8-0 allow
Control Request:
```yaml
id: 2d71-c9-c8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown
      entries:
      - ".:"
      - .gitignore
      - package-lock.json
      - package.json
      - remark.js
      - run.sh
      - test.sh
      - tsconfig.json
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      entries:
      - ".:"
      - .classpath
      - .gitignore
      - .project
      - run.sh
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust
      entries:
      - ".:"
      - .gitignore
      - .project
      - Cargo.lock
      - Cargo.toml
      - build.sh
      - run.sh
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine
      entries:
      - ".:"
      - AddressableNode.ts
      - AstEngineException.ts
      - AstTask.ts
      - AstWorker.ts
      - DocumentCache.ts
      - DocumentService.ts
      - Main.ts
      - MarkdownAstEngine.test.ts
      - MarkdownAstEngine.ts
      - MarkdownAstServer.ts
      - MarkdownProcessor.ts
      - NodeLocator.ts
      - TaskExecutors.ts
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
      entries:
      - ".:"
      - AddressableNode.java
      - AstEngineException.java
      - DocumentCache.java
      - JavaAstEngine.java
      - JavaAstServer.java
      - Main.java
      - NodeGroup.java
      - NodeLocator.java
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/scripts
      entries:
      - ".:"
      - openapi_generate.sh
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/openapi
      entries:
      - ".:"
      - AppendInfoNodesValidateClient.ts
      - AppendInfoNodesValidateClientImpl.ts
      - AppendInfoNodesValidateServer.ts
      - JsonSupport.ts
```
/answer 2d71-c9-c8 allow
Text:
 Im Ordner `ast-engines/markdown` liegt schon eine TypeScript-Engine. Ich prüfe, wie vollständig sie ist, und suche die Stellen, an denen die Engines eingebunden werden.
ReasoningToken: 162
Control Request:
```yaml
id: 2d71-ca-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/run.sh
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/package.json
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/test.sh
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/tsconfig.json
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/.gitignore
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/run.sh
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine/Main.ts
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/remark.js
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: MarkdownEngine|_markdown|JavaEngine|rpc\._java|'markdown'
    include:
    - '*.py'
    - '*.toml'
    - '*.md'
    - '*.json'
    - '*.sh'
    - '*.yaml'
    - '*.yml'
    exclude:
    - node_modules/**
    - .venv/**
    - '**/__pycache__/**'
    limit: 40
  reason: Start-/Build-Skripte und Entry-Point der TS-Engine sowie Java-run.sh lesen; Registrierungsstellen der Engines suchen
```
/answer 2d71-ca-0 deny die Engine in `ast-engines/markdown` ist fertig implementiert
Text:
 Alles klar, ich behandle die Engine als fertig und lasse ihre Implementierung in Ruhe. Ich suche nur die Einbindung auf der mcpc-Seite.
ReasoningToken: 207
Control Request:
```yaml
id: 2d71-cb-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: MarkdownEngine|generic\._markdown|JavaEngine|rpc\._java|RustEngine
    include:
    - '*.py'
    - '*.toml'
    - '*.md'
    - '*.sh'
    - '*.yaml'
    - '*.yml'
    exclude:
    - node_modules/**
    - .venv/**
    - '**/__pycache__/**'
    limit: 40
    intent: locate_for_edit
  reason: "Stellen finden, an denen Markdown-/Java-Engines in mcpc registriert oder referenziert werden"
```
/answer 2d71-cb-0 allow Es gibt keine aktuelle Einbindung der Typescript basierten Markdown AST engine, das ist die Aufgabe
Control Request:
```yaml
id: 2d71-cc-cb
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc
        matches:
        - path: tests/test_ast_rust_bridge.py
          nodes:
          - id: Mx1s7V|mOqPjE
            matches:
            - from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
          - id: engine
            matches:
            - "def engine() -> RustEngine:"
          - id: engine.JeWQtc|nwxs0M
            matches:
            - return RustEngine()
          - id: test_parse_and_serialize_roundtrip
            matches:
            - "def test_parse_and_serialize_roundtrip(engine: RustEngine) -> None:"
          - id: test_locate_all_finds_struct_impl_and_fn
            matches:
            - "def test_locate_all_finds_struct_impl_and_fn(engine: RustEngine) -> None:"
          - id: test_replace_mutates_and_stays_valid
            matches:
            - "def test_replace_mutates_and_stays_valid(engine: RustEngine) -> None:"
          - id: test_validate_reports_syntax_error
            matches:
            - "def test_validate_reports_syntax_error(engine: RustEngine) -> None:"
        - path: src/xy/ai/mcpc/tools/ast/base.py
          nodes:
          - id: Engine.is_definition.79Ctxs|QdTJiQ
            matches:
            - :class:`xy.ai.mcpc.tools.ast.generic._java.JavaEngine`) should override
        - path: src/xy/ai/mcpc/tools/ast/rpc/__init__.py
          nodes:
          - id: Mx1s7V|VobIpP
            matches:
            - from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
            - from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
          - id: VMvyha|JK431n
            matches:
            - "__all__ = ['RpcEngine', 'RpcProcess', 'JavaEngine', 'RustEngine']"
        - path: src/xy/ai/mcpc/tools/ast/rpc/_java.py
          nodes:
          - id: zrXNbR|3dRepm
            matches:
            - "JavaEngine` (kept around, but no longer wired up) with the real Java parser"
          - id: JpPcRr|tupArz
            matches:
            - "__all__ = ['JavaEngine']"
          - id: JavaEngine
            matches:
            - "class JavaEngine(RpcEngine):"
        - path: src/xy/ai/mcpc/tools/ast/rpc/_rust.py
          nodes:
          - id: b5aB3x|qU00Yb
            matches:
            - "__all__ = ['RustEngine']"
          - id: RustEngine
            matches:
            - "class RustEngine(RpcEngine):"
        - path: src/xy/ai/mcpc/tools/ast/generic/_markdown.py
          nodes:
          - id: rULMC2|CnEXKe
            matches:
            - "__all__ = ['MarkdownEngine']"
          - id: MarkdownEngine
            matches:
            - "class MarkdownEngine(TreeSitterEngine):"
        - path: src/xy/ai/mcpc/tools/ast/generic/__init__.py
          nodes:
          - id: Nna9mi|guMKQx
            matches:
            - per-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`
            - (:class:`MarkdownEngine`); this module re-exports the package's public
          - id: LvTUnt|ti8sGW
            matches:
            - from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
          - id: 95AfuA|pyxV6G
            matches:
            - from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
            - from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
          - id: 9C41E7|yvHPZT
            matches:
            - "'JavaEngine',"
            - "'RustEngine',"
            - "'MarkdownEngine',"
          - id: jxTk2k|zmn29i
            matches:
            - "type[Engine]] = {'markdown': MarkdownEngine,"
            - "'java': JavaEngine,"
            - "'rust': RustEngine,"
        - path: src/xy/ai/mcpc/tools/ast/generic/_java.py
          nodes:
          - id: sXmiXg|B2wymS
            matches:
            - "__all__ = ['JavaEngine']"
          - id: JavaEngine
            matches:
            - "class JavaEngine(TreeSitterEngine):"
        - path: src/xy/ai/mcpc/tools/ast/generic/_engine.py
          nodes:
          - id: 1wOyVk|BtswSM
            matches:
            - :mod:`xy.ai.mcpc.tools.ast.generic._markdown` for Markdown's Page/Heading
```
/answer 2d71-cc-cb allow
Text:
 Die Aufgabe ist also, die TS-Engine in mcpc einzubinden. Dafür muss ich wissen, wie sie gestartet wird. Ich lese die Start-Skripte und die Registrierung nur lesend.
ReasoningToken: 132
Control Request:
```yaml
id: 2d71-cd-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/run.sh
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/package.json
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/run.sh
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - Nna9mi|guMKQx
    - LvTUnt|ti8sGW
    - 95AfuA|pyxV6G
    - 9C41E7|yvHPZT
    - jxTk2k|zmn29i
  reason: "Nur lesend: Startkommando (run.sh, package.json) der fertigen TS-Engine und Java-Vergleich sowie die Registrierung in generic/__init__.py, um den Python-Wrapper korrekt zu bauen"
```
/answer 2d71-cd-0 allow
Control Request:
```yaml
id: 2d71-ce-cd
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/run.sh
        nodes:
        - id: wsOXc0|h9tL3l
          excert: "#!/usr/bin/env bash"
        - id: set
          excert: set -euo pipefail
        - id: SCRIPT_DIR
          excert: "SCRIPT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\""
        - id: cd
          excert: cd "$SCRIPT_DIR"
        - id: 3KaaJ1|9m6gcu
          excert: "if [[ \"${1:-}\" == \"--build\" ]]; then"
        - id: exec
          excert: exec node src/xy/ai/mcpc/ast/engine/Main.ts "$@"
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/package.json
        nodes:
        - id: PNK1gN|vGziDk
          excert: "{"
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/run.sh
        nodes:
        - id: wsOXc0|h9tL3l
          excert: "#!/usr/bin/env bash"
        - id: set
          excert: set -euo pipefail
        - id: SCRIPT_DIR
          excert: "SCRIPT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\""
        - id: cd
          excert: cd "$SCRIPT_DIR"
        - id: SRC_DIR
          excert: SRC_DIR="src"
        - id: BIN_DIR
          excert: BIN_DIR=".bin"
        - id: LIBS_DIR
          excert: LIBS_DIR="libs"
        - id: MAIN_CLASS
          excert: MAIN_CLASS="xy.ai.mcpc.ast.engine.Main"
        - id: CP
          excert: CP="$BIN_DIR:$(find "$LIBS_DIR" -maxdepth 1 -name '*.jar' ! -name '*-sources.ja…
        - id: mkdir
          excert: mkdir -p "$BIN_DIR"
        - id: Xv34QT|T2p5W3
          excert: "if [[ \"${1:-}\" == \"--build\" ]]; then"
        - id: exec
          excert: exec java -cp "$CP" "$MAIN_CLASS" "$@"
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
        nodes:
        - id: Nna9mi|guMKQx
          type: statements
          code: "\"Generic tree-sitter back-ends for every non-Python language/format.\\n\\nSplit into :mod:`xy.ai.mcpc.tools.ast.generic._engine` (the universal\\n:class:`TreeSitterEngine`, exposing a grammar's native structure as-is) and\\nper-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`\\n(:class:`MarkdownEngine`); this module re-exports the package's public\\nsurface, dispatches a language symbol to its engine, and hands out the\\nwhole-file :class:`~xy.ai.mcpc.tools.ast.generic._text.PlainTextEngine`\\nfallback (via :func:`fallback_engine`) for extensions no grammar covers.\\n\""
        - id: LvTUnt|ti8sGW
          type: imports
          code: |-
            from __future__ import annotations
            from xy.ai.mcpc.tools.ast.base import Engine
            from xy.ai.mcpc.tools.ast.generic._css import CssEngine
            from xy.ai.mcpc.tools.ast.generic._engine import GenericEngine, TreeSitterEngine
            from xy.ai.mcpc.tools.ast.generic._html import HtmlEngine
            from xy.ai.mcpc.tools.ast.generic._javascript import JavaScriptEngine
            from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
            from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
        - id: 95AfuA|pyxV6G
          type: imports
          code: |-
            from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
            from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
            from xy.ai.mcpc.tools.ast.generic._xml import XmlEngine
            from xy.ai.mcpc.tools.ast.generic._yaml import YamlEngine
        - id: 9C41E7|yvHPZT
          type: statements
          code: |-
            __all__ = [
                'TreeSitterEngine',
                'GenericEngine',
                'CssEngine',
                'HtmlEngine',
                'JavaEngine',
                'JavaScriptEngine',
                'RustEngine',
                'MarkdownEngine',
                'PlainTextEngine',
                'XmlEngine',
                'YamlEngine',
                'language_for_extension',
                'get_engine',
                'fallback_engine']
            '#: File extension -> ``tree_sitter_language_pack`` language identifier.'
        - id: jxTk2k|zmn29i
          type: statements
          code: |-
            _ENGINE_CLASSES: dict[str,
                                  type[Engine]] = {'markdown': MarkdownEngine,
                                                   'java': JavaEngine,
                                                   'rust': RustEngine,
                                                   'yaml': YamlEngine,
                                                   'css': CssEngine,
                                                   'html': HtmlEngine,
                                                   'javascript': JavaScriptEngine,
                                                   'xml': XmlEngine}
```
/answer 2d71-ce-cd allow
Control Request:
```yaml
id: 2d71-cf-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/run.sh
    ids:
    - 3KaaJ1|9m6gcu
    - exec
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/package.json
    ids:
    - PNK1gN|vGziDk
  outline:
  - /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine/Main.ts
  reason: "Nur Startkontrakt (Port-Argument, Build-Schritt, Node-Aufruf) ermitteln, keine Änderungen an der Engine"
```
/answer 2d71-cf-0 allow
Control Request:
```yaml
id: 2d71-d0-cf
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine/Main.ts
        nodes:
        - id: qXwxqk|d9nIOV
          excert: "import { availableParallelism } from 'node:os';"
        - id: qyWVfp|PTjKCv
          excert: "import { readdir } from 'node:fs/promises';"
        - id: J9yhTk|pe2GQB
          excert: "import { join } from 'node:path';"
        - id: JbSwgM|DVVE4W
          excert: "import { MarkdownAstServer } from './MarkdownAstServer.ts';"
        - id: 2zhOhM|5MabEk
          excert: "import { WorkerPool } from './TaskExecutors.ts';"
        - id: K4tXJO|kOm8tq
          excert: /** Starts the remark AST engine as a standalone JSON/HTTP backed engine. */
        - id: main
          excert: "async function main(args: string[]): Promise<void> {"
        - id: workerCount
          excert: "function workerCount(): number {"
        - id: 5paQRD|aHergU
          excert: /** Recursively re-prints every `.md` file under `root` in the engine's normali…
        - id: convert
          excert: "async function convert(root: string, workers: number): Promise<void> {"
        - id: QIkeIH|Ff0dXj
          excert: await main(process.argv.slice(2));
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/run.sh
        nodes:
        - id: 3KaaJ1|9m6gcu
          type: if_statement
          code: |-
            if [[ "${1:-}" == "--build" ]]; then
              ./node_modules/.bin/tsc -p .
              shift
            fi
        - id: exec
          type: command
          code: exec node src/xy/ai/mcpc/ast/engine/Main.ts "$@"
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/package.json
        nodes:
        - id: PNK1gN|vGziDk
          type: object
          code: |-
            {
              "name": "markdown",
              "description": "Augmentation for Agent based Markdown file handlind",
              "type": "module",
              "main": "",
              "scripts": {
                "start": "node src/xy/ai/mcpc/ast/engine/Main.ts",
                "typecheck": "tsc -p .",
                "test": "node --test \"src/**/*.test.ts\""
              },
              "keywords": [
                "Agent",
                "Markdown",
                "MDAST"
              ],
              "dependencies": {
                "remark": "^15.0.1",
                "remark-behead": "^3.1.0",
                "remark-frontmatter": "^5.0.0",
                "to-vfile": "^8.0.0"
              },
              "devDependencies": {
                "@types/node": "^26.6.6",
                "typescript": "^7.0.2"
              }
            }
```
/answer 2d71-d0-cf allow
ReasoningToken: 167
Control Request:
```yaml
id: 2d71-d1-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
    ids:
    - reOPSB|2w0163
    - mnFJcA|J3zcEx
    - RustProcess
    - RustEngine
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
  reason: Rust-Prozess als zweites Vorbild für Port/Env-Konventionen und Rust-Bridge-Test als Testvorlage lesen
```
/answer 2d71-d1-0 allow
Control Request:
```yaml
id: 2d71-d2-d1
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
        nodes:
        - id: dU2g3m|CuR1rs
          excert: '''Eigenständiger Test für die Rust-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._rust`…'
        - id: Mx1s7V|mOqPjE
          excert: from __future__ import annotations
        - id: WjQzAk|VtGINc
          excert: "SOURCE = 'struct Point {\\n    x: i32,\\n    y: i32,\\n}\\n\\nimpl Point {\\n    fn n…"
        - id: engine
          excert: "@pytest.fixture(scope='module') def engine() -> RustEngine:"
        - id: test_parse_and_serialize_roundtrip
          excert: "def test_parse_and_serialize_roundtrip(engine: RustEngine) -> None:"
        - id: test_locate_all_finds_struct_impl_and_fn
          excert: "def test_locate_all_finds_struct_impl_and_fn(engine: RustEngine) -> None:"
        - id: test_replace_mutates_and_stays_valid
          excert: "def test_replace_mutates_and_stays_valid(engine: RustEngine) -> None:"
        - id: test_validate_reports_syntax_error
          excert: "def test_validate_reports_syntax_error(engine: RustEngine) -> None:"
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
        nodes:
        - id: reOPSB|2w0163
          type: statements
          code: "\"Rust engine: syn-based ``xy_ai_ast_rust`` process, driven over RPC/PIC.\\n\\nStarted lazily -- and kept running for the server's lifetime -- the same way\\n``ast-engines/rust/run.sh`` does, minus the shell: we invoke the prebuilt\\nrelease binary directly; building it (``run.sh --build``) stays a separate,\\nexternally-guaranteed step.\\n\""
        - id: mnFJcA|J3zcEx
          type: statements
          code: |-
            _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'rust'
            _BINARY = _ENGINE_DIR / '.bin' / 'release' / 'xy_ai_ast_rust'
        - id: RustProcess
          type: ClassDef
          excert: "class RustProcess(RpcProcess):"
          docstring: Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build``…
          children:
          - id: RustProcess.0qSEGQ|1SP6FA
            type: statements
            code: '''Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build``).'''
          - id: RustProcess.default_port
            type: FunctionDef
            code: |-
              def default_port(self) -> int:
                  return int(os.environ.get('AST_RUST_PORT', '8788'))
          - id: RustProcess.cwd
            type: FunctionDef
            code: |-
              def cwd(self) -> str | None:
                  return str(_ENGINE_DIR)
          - id: RustProcess.command
            type: FunctionDef
            code: |-
              def command(self, port: int) -> list[str]:
                  return [str(_BINARY), str(port)]
        - id: RustEngine
          type: ClassDef
          excert: "class RustEngine(RpcEngine):"
          docstring: "Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process."
          children:
          - id: RustEngine.pPxVPO|QPQZCQ
            type: statements
            code: |-
              'Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process.'
              validates_syntax = True
          - id: RustEngine.init
            type: FunctionDef
            code: |-
              def __init__(self) -> None:
                  super().__init__('rust', _PROCESS)
```
/answer 2d71-d2-d1 allow
Control Request:
```yaml
id: 2d71-d3-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine/Main.ts
    ids:
    - main
    - workerCount
  reason: "Nur main() der TS-Engine lesen, um Port-Argument-Konvention zu bestätigen"
```
/answer 2d71-d3-0 allow
Control Request:
```yaml
id: 2d71-d4-d3
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/ast-engines/markdown/src/xy/ai/mcpc/ast/engine/Main.ts
        nodes:
        - id: main
          type: function_declaration
          code: |-
            async function main(args: string[]): Promise<void> {
              const workers = workerCount();
              if (args[0] === 'convert') {
                if (args[1] === undefined) {
                  console.error('usage: convert <directory>');
                  process.exit(1);
                }
                await convert(args[1], workers);
                return;
              }
              const port = args[0] !== undefined ? Number(args[0]) : Number(process.env['PORT'] ?? '8788');
              if (!Number.isInteger(port) || port < 0 || port > 65535) {
                console.error(`invalid port: ${args[0] ?? process.env['PORT']}`);
                process.exit(1);
              }
              const host = process.env['HOST'] ?? '127.0.0.1';
              const server = await new MarkdownAstServer({ workers }).start(port, host);
              const address = server.address();
              console.log(`markdown-remark ast engine listening on port ${typeof address === 'object' && address ? address.port : port} (${workers} worker threads)`);
            }
        - id: workerCount
          type: function_declaration
          code: |-
            function workerCount(): number {
              const raw = process.env['WORKERS'];
              if (raw === undefined)
                return availableParallelism();
              const n = Number(raw);
              if (!Number.isInteger(n) || n < 0) {
                console.error(`invalid WORKERS: ${raw}`);
                process.exit(1);
              }
              return n;
            }
```
/answer 2d71-d4-d3 allow
ReasoningToken: 78
Control Request:
```yaml
id: 2d71-d5-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: validates_syntax
    include:
    - '*.py'
    limit: 20
  reason: "Basisklasse Engine: Bedeutung von validates_syntax und weitere Klassenattribute prüfen"
```
/answer 2d71-d5-0 allow
Control Request:
```yaml
id: 2d71-d6-d5
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: base.py
          matches:
          - "validates_syntax: bool = False"
        - path: rpc/_java.py
          matches:
          - validates_syntax = True
        - path: rpc/_engine.py
          matches:
          - '``validates_syntax``).'
        - path: rpc/_rust.py
          matches:
          - validates_syntax = True
        - path: edit_marks.py
          matches:
          - "if engine.validates_syntax:"
          - "max_level=3 if tree.engine.validates_syntax else 2,"
        - path: edit_block.py
          matches:
          - "if engine.validates_syntax:"
          - "max_level=3 if tree.engine.validates_syntax else 2,"
        - path: python/_engine.py
          matches:
          - validates_syntax = True
        - path: imports.py
          matches:
          - error = engine.validate(source) if engine.validates_syntax else None
```
/answer 2d71-d6-d5 allow
Thinking:
Die Startkonvention ist festgelegt: `node src/.../Main.ts <port>` im Engine-Verzeichnis, mit Markdown auf Port 8789. Als Nächstes lege ich den Wrapper an und schalte die Registrierungen um.


ReasoningToken: 476
Control Request:
```yaml
id: 2d71-d7-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
    source: |+
      """Markdown engine: remark-based ``ast-engines/markdown`` process, driven over RPC/PIC.

      Replaces the old tree-sitter :class:`~xy.ai.mcpc.tools.ast.generic._markdown.
      MarkdownEngine` (kept around, but no longer wired up). The Node.js engine is
      started lazily -- and kept running for the server's lifetime -- the same way
      ``ast-engines/markdown/run.sh`` does, minus the shell: we invoke ``node``
      directly on the engine's entry point; installing its dependencies
      (``npm install``) stays a separate, externally-guaranteed step.
      """

      from __future__ import annotations

      import os
      from pathlib import Path

      from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
      from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess

      __all__ = ['MarkdownEngine']


      def _workspace_root() -> Path:
          for ancestor in Path(__file__).resolve().parents:
              if (ancestor / 'ast-engines').is_dir():
                  return ancestor
          raise RuntimeError(f"could not locate an 'ast-engines' directory above {__file__}")


      _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'markdown'
      _MAIN = Path('src') / 'xy' / 'ai' / 'mcpc' / 'ast' / 'engine' / 'Main.ts'


      class MarkdownProcess(RpcProcess):
          """Starts the ``ast-engines/markdown`` remark engine on Node.js."""

          def default_port(self) -> int:
              return int(os.environ.get('AST_MARKDOWN_PORT', '8789'))

          def cwd(self) -> str | None:
              return str(_ENGINE_DIR)

          def command(self, port: int) -> list[str]:
              return [os.environ.get('NODE', 'node'), str(_MAIN), str(port)]


      _PROCESS = MarkdownProcess()


      class MarkdownEngine(RpcEngine):
          """Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC process."""

          def __init__(self) -> None:
              super().__init__('markdown', _PROCESS)


  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: LvTUnt|ti8sGW
    old_text: |
      from xy.ai.mcpc.tools.ast.generic._markdown import MarkdownEngine
    new_text: ""
  reason: RPC-Wrapper für die TS-Markdown-Engine anlegen und in rpc/__init__ und generic/__init__ statt der Tree-Sitter-Engine registrieren
```
/answer 2d71-d7-0 allow
Control Request:
```yaml
id: 2d71-d8-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: 95AfuA|pyxV6G
    old_text: |
      from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
    new_text: |
      from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
      from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
    id: Mx1s7V|VobIpP
    old_text: |
      from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
    new_text: |
      from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
      from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
    id: VMvyha|JK431n
    old_text: "'JavaEngine', 'RustEngine']"
    new_text: "'JavaEngine', 'MarkdownEngine', 'RustEngine']"
  reason: "Import der RPC-Markdown-Engine in generic/__init__ ergänzen, rpc/__init__ exportieren, Whitespace-Rest im neuen Modul entfernen"
```
/answer 2d71-d8-0 allow
ReasoningToken: 41
Control Request:
```yaml
id: 2d71-d9-d8
result:
  structuredContent:
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
        id: Mx1s7V|VobIpP
        new_id: Mx1s7V|VobIpP
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
        id: VMvyha|JK431n
        new_id: VMvyha|JK431n
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
        id: 95AfuA|pyxV6G
        error: Text not found in node.
        reason: not_found
        next_step: reread_node
```
/answer 2d71-d9-d8 allow
Control Request:
```yaml
id: 2d71-da-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
    id: MarkdownEngine
    old_text: |+
      super().__init__('markdown', _PROCESS)

    new_text: |
      super().__init__('markdown', _PROCESS)
  reason: Überflüssige Leerzeile am Dateiende des neuen Moduls entfernen
```
/answer 2d71-da-0 allow
Control Request:
```yaml
id: 2d71-db-da
result:
  structuredContent:
    edit_block:
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
        id: MarkdownEngine
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: |2-

              def __init__(self) -> None:
                  super().__init__('markdown', _PROCESS)
```
/answer 2d71-db-da allow
Text:
 Zwei Edits sind fehlgeschlagen, weil die Node-IDs und der Text nicht passten. Ich lese die betroffenen Dateien neu ein.
Control Request:
```yaml
id: 2d71-dc-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
  reason: Aktuellen Zustand der Imports in generic/__init__ und des neuen Moduls prüfen
```
/answer 2d71-dc-0 allow
Control Request:
```yaml
id: 2d71-dd-dc
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
        nodes:
        - id: Nna9mi|guMKQx
          excert: '"Generic tree-sitter back-ends for every non-Python language/format.\n\nSplit i…'
        - id: P4OPWt|0VgAY8
          excert: from __future__ import annotations
        - id: 95AfuA|bMYW5n
          excert: from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
        - id: 9C41E7|yvHPZT
          excert: "__all__ = ["
        - id: acL0qu|XdqYqL
          excert: "EXT_LANGUAGE = {"
        - id: language_for_extension
          excert: "def language_for_extension(ext: str) -> str | None:"
        - id: FnS4Dz|ZBavT9
          excert: "'#: Language symbol -> dedicated Engine subclass; anything absent here falls'"
        - id: jxTk2k|zmn29i
          excert: "_ENGINE_CLASSES: dict[str,"
        - id: Ygq4pQ|qTOATu
          excert: "_ENGINES: dict[str, Engine] = {}"
        - id: get_engine
          excert: "def get_engine(symbol: str) -> Engine:"
        - id: 5P6pDx|byGZAx
          excert: _FALLBACK_ENGINE = PlainTextEngine()
        - id: fallback_engine
          excert: "def fallback_engine() -> PlainTextEngine:"
          docstring: Whole-file engine used when no tree-sitter grammar claims the extension.
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
        nodes:
        - id: 8fSL6x|fEXSFN
          excert: "\"Markdown engine: remark-based ``ast-engines/markdown`` process, driven over RP…"
        - id: ILN4fu|sCWK6E
          excert: from __future__ import annotations
        - id: HrsM8B|ZAPiHS
          excert: "__all__ = ['MarkdownEngine']"
        - id: workspace_root
          excert: "def _workspace_root() -> Path:"
        - id: pwufUY|RHnCVl
          excert: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'markdown'
        - id: MarkdownProcess
          excert: "class MarkdownProcess(RpcProcess):"
          docstring: Starts the ``ast-engines/markdown`` remark engine on Node.js.
          children:
          - id: MarkdownProcess.T89awu|KUD046
            excert: '''Starts the ``ast-engines/markdown`` remark engine on Node.js.'''
          - id: MarkdownProcess.default_port
            excert: "def default_port(self) -> int:"
          - id: MarkdownProcess.cwd
            excert: "def cwd(self) -> str | None:"
          - id: MarkdownProcess.command
            excert: "def command(self, port: int) -> list[str]:"
        - id: U6RlhD|cmSjIL
          excert: _PROCESS = MarkdownProcess()
        - id: MarkdownEngine
          excert: "class MarkdownEngine(RpcEngine):"
          docstring: "Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC pro…"
          children:
          - id: MarkdownEngine.oP8rfU|ZXkRnE
            excert: "'Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC pr…"
          - id: MarkdownEngine.init
            excert: "def __init__(self) -> None:"
```
/answer 2d71-dd-dc allow
ReasoningToken: 124
Control Request:
```yaml
id: 2d71-de-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - P4OPWt|0VgAY8
    - 95AfuA|bMYW5n
  reason: Import-Knoten der generic/__init__ im Wortlaut lesen
```
/answer 2d71-de-0 allow
Control Request:
```yaml
id: 2d71-df-de
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
        nodes:
        - id: P4OPWt|0VgAY8
          type: imports
          code: |-
            from __future__ import annotations
            from xy.ai.mcpc.tools.ast.base import Engine
            from xy.ai.mcpc.tools.ast.generic._css import CssEngine
            from xy.ai.mcpc.tools.ast.generic._engine import GenericEngine, TreeSitterEngine
            from xy.ai.mcpc.tools.ast.generic._html import HtmlEngine
            from xy.ai.mcpc.tools.ast.generic._javascript import JavaScriptEngine
            from xy.ai.mcpc.tools.ast.generic._text import PlainTextEngine
            from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
        - id: 95AfuA|bMYW5n
          type: imports
          code: |-
            from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
            from xy.ai.mcpc.tools.ast.generic._xml import XmlEngine
            from xy.ai.mcpc.tools.ast.generic._yaml import YamlEngine
```
/answer 2d71-df-de allow
Control Request:
```yaml
id: 2d71-e0-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: P4OPWt|0VgAY8
    old_text: from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
    new_text: |-
      from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
      from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
    id: MarkdownEngine.init
    old_text: "super().__init__('markdown', _PROCESS)"
    new_text: "super().__init__('markdown', _PROCESS)"
  reason: RPC-MarkdownEngine in generic/__init__ importieren; Leerzeile am Ende des neuen Moduls entfernen
```
/answer 2d71-e0-0 allow
Control Request:
```yaml
id: 2d71-e1-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
    node_type: imports|statements
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
    name: MarkdownEngine
  reason: "Kontrolle: rpc/__init__ und Ende von _markdown.py"
```
/answer 2d71-e1-0 allow
Control Request:
```yaml
id: 2d71-e2-e1
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/__init__.py
        nodes:
        - id: Y0bbSV|7bGzjK
          type: statements
          code: "'``Engine`` back-ends delegating to external parsers over the RPC/PIC HTTP\\nprotocol (``ast-engines/openapi.yaml``), each running as a lazily-started,\\nlong-lived subprocess.\\n\\n:mod:`xy.ai.mcpc.tools.ast.rpc._process` and :mod:`._engine` provide the\\nreusable process-supervision/``Engine`` plumbing; per-language modules like\\n:mod:`._java` only add the concrete subprocess command and static metadata.\\n'"
        - id: Mx1s7V|nm6HRa
          type: imports
          code: |-
            from __future__ import annotations
            from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
            from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
            from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
            from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
            from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
        - id: VMvyha|utMSaH
          type: statements
          code: "__all__ = ['RpcEngine', 'RpcProcess', 'JavaEngine', 'MarkdownEngine', 'RustEngine']"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_markdown.py
        nodes:
        - id: MarkdownEngine
          code: |-
            class MarkdownEngine(RpcEngine):
                """Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC process."""

                def __init__(self) -> None:
                    super().__init__('markdown', _PROCESS)
```
/answer 2d71-e2-e1 allow
Text:
 Die Verdrahtung steht. Ich prüfe noch, ob Tests oder Doku die alte Engine voraussetzen.
ReasoningToken: 113
Control Request:
```yaml
id: 2d71-e3-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "markdown|\\.md['\\\"]"
    include:
    - test_*.py
    - conftest.py
    - pyproject.toml
    - README*
    exclude:
    - node_modules/**
    - .venv/**
    limit: 30
    intent: survey
  reason: "Tests/Doku finden, die Markdown-Engine-Verhalten oder das alte Modul referenzieren"
```
/answer 2d71-e3-0 allow
Control Request:
```yaml
id: 2d71-e4-e3
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc
        matches:
        - path: tests/test_edit_tolerance.py
          matches:
          - "_PROBE_SOURCE = '\"\"\"``web_fetch_exa`` - stage 1 of the two-stage Exa fetch retrieval.\\n\\nFetches page content and caches each full result (incl. text …sults`` with the returned ids to resolve url and text.\\n\"\"\"\\n\\n#: line, then the page\\'s extracted markdown content; consecutive urls are\\nNEXT = 1\\n'"
          - "old_text=\"'#: line, then the page's extracted markdown content; consecutive urls are\\\\''\","
        - path: tests/test_mcpcignore.py
          matches:
          - (tmp_path / '.mcpcignore').write_text('*.py\n*.txt\n*.md\n')
          - assert not ignore.is_allowed(sub / 'a.md')
          - (tmp_path / 'x.md').write_text('x')
          - "assert out.results[0].entries == ['.:', 'x.md', './sub:', 'a.py', './sub/deep:', 'd.py']"
        - path: tests/test_persist.py
          matches:
          - target = tmp_path / 'sub' / 'notes.md'
          - "assert names == ['First-Result.md', 'Second.md']"
          - assert 'body two' not in (tmp_path / 'out' / 'First-Result.md').read_text(encoding='utf-8')
          - assert (tmp_path / 'First-Result.md').is_file()
          - "assert sorted((p.name for p in tmp_path.iterdir())) == ['First-Result-2.md', 'First-Result.md']"
          - "result = persist_result(_docs()[:1], str(tmp_path / 'o.md'), ctx, reason='test')"
          - "assert persist_result([], str(tmp_path / 'x.md'), ctx, reason='test').is_error"
          - "result = persist_result(_docs()[:1], str(blocker / 'child.md'), ctx, reason='test')"
          - text = (tmp_path / 'A-Paper.md').read_text(encoding='utf-8')
          - "'r.md')}))"
          - text = (tmp_path / 'r.md').read_text(encoding='utf-8')
        - path: pyproject.toml
          matches:
          - readme = "README.md"
        warnings:
        - 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 2d71-e4-e3 allow
Text:
 Es gibt keine Markdown-Tests, die das alte Verhalten festschreiben. Ich lese den Rust-Bridge-Test als Vorlage für einen gleichartigen Markdown-Test.
ReasoningToken: 144
Control Request:
```yaml
id: 2d71-e5-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
    ids:
    - dU2g3m|CuR1rs
    - Mx1s7V|mOqPjE
    - WjQzAk|VtGINc
    - engine
    - test_parse_and_serialize_roundtrip
    - test_locate_all_finds_struct_impl_and_fn
    - test_replace_mutates_and_stays_valid
  reason: Rust-Bridge-Test im Wortlaut als Vorlage lesen
```
/answer 2d71-e5-0 allow
Control Request:
```yaml
id: 2d71-e6-e5
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
        nodes:
        - id: dU2g3m|CuR1rs
          type: statements
          code: '''Eigenständiger Test für die Rust-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._rust``).\n\nStartet den echten ``xy_ai_ast_rust``-Prozess lazy und prüft Parse/Serialize/\nLocate/Mutate/Validate end-to-end.\n'''
        - id: Mx1s7V|mOqPjE
          type: imports
          code: |-
            from __future__ import annotations
            import pytest
            from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
        - id: WjQzAk|VtGINc
          type: statements
          code: "SOURCE = 'struct Point {\\n    x: i32,\\n    y: i32,\\n}\\n\\nimpl Point {\\n    fn new(x: i32, y: i32) -> Self {\\n        Point { x, y }\\n    }\\n\\n    fn sum(&self) -> i32 {\\n        self.x + self.y\\n    }\\n}\\n\\nfn main() {\\n    let p = Point::new(1, 2);\\n    println!(\"{}\", p.sum());\\n}\\n'"
        - id: engine
          type: FunctionDef
          code: |-
            @pytest.fixture(scope='module')
            def engine() -> RustEngine:
                return RustEngine()
        - id: test_parse_and_serialize_roundtrip
          type: FunctionDef
          code: |-
            def test_parse_and_serialize_roundtrip(engine: RustEngine) -> None:
                tree = engine.parse(SOURCE)
                assert engine.serialize(tree) == SOURCE
        - id: test_locate_all_finds_struct_impl_and_fn
          type: FunctionDef
          code: |-
            def test_locate_all_finds_struct_impl_and_fn(engine: RustEngine) -> None:
                tree = engine.parse(SOURCE)
                located = engine.locate_all(tree)
                names = {loc.name for loc in located}
                assert 'Point' in names
                assert 'main' in names
                assert 'sum' in names
        - id: test_replace_mutates_and_stays_valid
          type: FunctionDef
          code: |-
            def test_replace_mutates_and_stays_valid(engine: RustEngine) -> None:
                tree = engine.parse(SOURCE)
                located = engine.locate_all(tree)
                sum_fn = next((loc for loc in located if loc.name == 'sum'))
                engine.replace(sum_fn, 'fn sum(&self) -> i32 {\n        self.x * self.y\n    }')
                mutated = engine.serialize(tree)
                assert 'self.x * self.y' in mutated
                assert engine.validate(mutated) is None
```
/answer 2d71-e6-e5 allow
ReasoningToken: 321
Control Request:
```yaml
id: 2d71-e7-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    source: |
      """Eigenständiger Test für die Markdown-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._markdown``).

      Startet den echten ``ast-engines/markdown``-Prozess lazy und prüft Parse/Serialize/
      Locate/Mutate end-to-end.
      """

      from __future__ import annotations

      import pytest

      from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine

      SOURCE = (
          '# Title\n\n'
          'Intro text.\n\n'
          '## Section A\n\n'
          'First paragraph.\n\n'
          '## Section B\n\n'
          'Second paragraph.\n'
      )


      @pytest.fixture(scope='module')
      def engine() -> MarkdownEngine:
          return MarkdownEngine()


      def test_parse_and_serialize_roundtrip(engine: MarkdownEngine) -> None:
          tree = engine.parse(SOURCE)
          assert engine.serialize(tree) == SOURCE


      def test_locate_all_finds_headings(engine: MarkdownEngine) -> None:
          tree = engine.parse(SOURCE)
          located = engine.locate_all(tree)
          assert located
          assert any('Section A' in loc.name for loc in located)
          assert any('Section B' in loc.name for loc in located)


      def test_replace_mutates_paragraph(engine: MarkdownEngine) -> None:
          tree = engine.parse(SOURCE)
          located = engine.locate_all(tree)
          target = next(loc for loc in located if 'Second paragraph.' in engine.node_code(loc.node))
          engine.replace(target, 'Changed paragraph.')
          mutated = engine.serialize(tree)
          assert 'Changed paragraph.' in mutated
          assert 'First paragraph.' in mutated
  reason: End-to-End-Test für die Markdown-RPC-Bridge analog zum Rust-Bridge-Test anlegen
```
/answer 2d71-e7-0 allow
Text:
 Ich führe den Test aus, um die Annahmen zu prüfen.
ReasoningToken: 68
Control Request:
```yaml
id: 2d71-e8-0
toolName: python
arguments:
  script: |-
    import subprocess,sys
    r=subprocess.run([sys.executable,'-m','pytest','-x','-q','tests/test_ast_markdown_bridge.py','tests/test_ast_rust_bridge.py','--no-header','-p','no:cacheprovider'],cwd='/home/user/xyan/xy.ai.workbench/mcpc',capture_output=True,text=True,timeout=300)
    print(r.stdout[-6000:]);print(r.stderr[-2000:])
  reason: "Neuen Bridge-Test sowie die Rust-Bridge zur Regression laufen lassen (nur pytest, kein Git)"
```
/answer 2d71-e8-0 allow
Control Request:
```yaml
id: 2d71-e9-e8
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      .F
      =================================== FAILURES ===================================
      ________________________ test_locate_all_finds_headings ________________________

      engine = <xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x770dc3bcbf80>

          def test_locate_all_finds_headings(engine: MarkdownEngine) -> None:
              tree = engine.parse(SOURCE)
              located = engine.locate_all(tree)
              assert located
      >       assert any(('Section A' in loc.name for loc in located))

      tests/test_ast_markdown_bridge.py:23:
      _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _

      .0 = <list_iterator object at 0x770dc6610340>

      >   assert any(('Section A' in loc.name for loc in located))
      E   TypeError: argument of type 'NoneType' is not iterable

      tests/test_ast_markdown_bridge.py:23: TypeError
      =========================== short test summary info ============================
      FAILED tests/test_ast_markdown_bridge.py::test_locate_all_finds_headings - Ty...
      !!!!!!!!!!!!!!!!!!!!!!!!!! stopping after 1 failures !!!!!!!!!!!!!!!!!!!!!!!!!!!
      1 failed, 1 passed in 0.65s
```
/answer 2d71-e9-e8 allow
Text:
 Der Prozess startet und der Roundtrip stimmt. Nur `loc.name` ist bei Markdown `None`. Ich schaue mir an, was die Engine tatsächlich liefert.
ReasoningToken: 60
Control Request:
```yaml
id: 2d71-ea-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
    S='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n'
    e=MarkdownEngine(); t=e.parse(S)
    for l in e.locate_all(t):
        print(l.__class__.__name__, {k:(v if k!='node' else None) for k,v in vars(l).items()} if hasattr(l,'__dict__') else l, '|', e.signature(l.node), '|', repr(e.node_code(l.node)))
  reason: "Laufzeit-Probe: welche Felder Located/Signatur für Markdown liefern, um den Test korrekt zu formulieren"
```
/answer 2d71-ea-0 allow
Control Request:
```yaml
id: 2d71-eb-ea
result:
  structuredContent:
    exit_code: 0
    stdout: |
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 0, 'node_id': 'Title', 'node_type': 'Section', 'name': 'Title', 'lineno': 1, 'end_lineno': 11, 'parent_type': 'Root', 'expandable': True} | # Title | '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n'
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 0, 'node_id': 'Title.KoDeyc|9U4Xa6', 'node_type': 'BlockGroup', 'name': None, 'lineno': 3, 'end_lineno': 3, 'parent_type': 'Section', 'expandable': False} | Intro text. | 'Intro text.\n'
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 1, 'node_id': 'Title.Section_A', 'node_type': 'Section', 'name': 'Section A', 'lineno': 5, 'end_lineno': 7, 'parent_type': 'Section', 'expandable': False} | ## Section A | '## Section A\n\nFirst paragraph.\n'
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 0, 'node_id': 'Title.Section_A.pgsc8P|70yLP0', 'node_type': 'BlockGroup', 'name': None, 'lineno': 7, 'end_lineno': 7, 'parent_type': 'Section', 'expandable': False} | First paragraph. | 'First paragraph.\n'
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 2, 'node_id': 'Title.Section_B', 'node_type': 'Section', 'name': 'Section B', 'lineno': 9, 'end_lineno': 11, 'parent_type': 'Section', 'expandable': False} | ## Section B | '## Section B\n\nSecond paragraph.\n'
      Located {'tree': Tree(engine=<xy.ai.mcpc.tools.ast.rpc._markdown.MarkdownEngine object at 0x72d55062aed0>, raw=None, source='# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n', path=None), 'node': None, 'parent': None, 'index': 0, 'node_id': 'Title.Section_B.zxoy2Z|mfTt5s', 'node_type': 'BlockGroup', 'name': None, 'lineno': 11, 'end_lineno': 11, 'parent_type': 'Section', 'expandable': False} | Second paragraph. | 'Second paragraph.\n'
```
/answer 2d71-eb-ea allow
Text:
 Die Engine arbeitet korrekt. Im Test passe ich nur die Annahmen an: Namen können `None` sein, und der Absatz ist eine `BlockGroup`.
ReasoningToken: 83
Control Request:
```yaml
id: 2d71-ec-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    id: test_locate_all_finds_headings
    old_text: |2-
          assert located
          assert any('Section A' in loc.name for loc in located)
          assert any('Section B' in loc.name for loc in located)
    new_text: |2-
          names = {loc.name for loc in located if loc.node_type == 'Section'}
          assert names == {'Title', 'Section A', 'Section B'}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    id: test_replace_mutates_paragraph
    old_text: target = next(loc for loc in located if 'Second paragraph.' in engine.node_code(loc.node))
    new_text: target = next(loc for loc in located if loc.node_type == 'BlockGroup' and 'Second' in engine.node_code(loc.node))
  reason: Testannahmen an die tatsächlichen Located-Felder anpassen
```
/answer 2d71-ec-0 allow
Control Request:
```yaml
id: 2d71-ed-ec
result:
  structuredContent:
    edit_block:
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
        id: test_locate_all_finds_headings
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: |2-
              assert located
              assert any(('Section A' in loc.name for loc in located))
              assert any(('Section B' in loc.name for loc in located))
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
        id: test_replace_mutates_paragraph
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: '    target = next((loc for loc in located if ''Second paragraph.'' in engine.node_code(loc.node)))'
```
/answer 2d71-ed-ec allow
Text:
 Die Datei wurde beim Anlegen umformatiert. Ich ersetze deshalb die beiden Funktionen komplett.
Control Request:
```yaml
id: 2d71-ee-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    id: test_locate_all_finds_headings
    source: |-
      def test_locate_all_finds_headings(engine: MarkdownEngine) -> None:
          tree = engine.parse(SOURCE)
          located = engine.locate_all(tree)
          names = {loc.name for loc in located if loc.node_type == 'Section'}
          assert names == {'Title', 'Section A', 'Section B'}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    id: test_replace_mutates_paragraph
    source: |-
      def test_replace_mutates_paragraph(engine: MarkdownEngine) -> None:
          tree = engine.parse(SOURCE)
          located = engine.locate_all(tree)
          target = next(
              loc for loc in located
              if loc.node_type == 'BlockGroup' and 'Second' in engine.node_code(loc.node)
          )
          engine.replace(target, 'Changed paragraph.')
          mutated = engine.serialize(tree)
          assert 'Changed paragraph.' in mutated
          assert 'Second paragraph.' not in mutated
          assert 'First paragraph.' in mutated
  reason: Zwei Testfunktionen mit korrigierten Annahmen ersetzen
```
/answer 2d71-ee-0 allow
Control Request:
```yaml
id: 2d71-ef-0
toolName: python
arguments:
  script: |-
    import subprocess,sys
    r=subprocess.run([sys.executable,'-m','pytest','-q','--no-header','-p','no:cacheprovider','tests'],cwd='/home/user/xyan/xy.ai.workbench/mcpc',capture_output=True,text=True,timeout=1200)
    print(r.stdout[-5000:]);print(r.stderr[-1500:])
  reason: Bridge-Tests und die gesamte Test-Suite zur Regressionsprüfung ausführen
```
/answer 2d71-ef-0 allow
Control Request:
```yaml
id: 2d71-f0-ef
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      ........................................................................ [ 18%]
      ........................................................................ [ 36%]
      ........................................................................ [ 54%]
      ........................................................................ [ 72%]
      ........................................................................ [ 90%]
      .......................................                                  [100%]
      399 passed in 11.25s
```
/answer 2d71-f0-ef allow
Text:
 Zum Schluss korrigiere ich noch den Docstring in `generic/__init__.py`, der die alte Markdown-Engine nennt.
ReasoningToken: 42
Control Request:
```yaml
id: 2d71-f1-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    id: Nna9mi|guMKQx
    old_text: per-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._markdown`\n(:class:`MarkdownEngine`); this module
    new_text: "per-language overrides such as :mod:`xy.ai.mcpc.tools.ast.generic._css`; the\\nJava, Rust and Markdown back-ends live in :mod:`xy.ai.mcpc.tools.ast.rpc`.\\nThis module"
    exact: false
  reason: Docstring an die neue Verdrahtung anpassen
```
/answer 2d71-f1-0 allow
Result Stats: id=8d367516-0e0c-4674-80a5-34402faaa645, total: 71277, in: 50, out: 16526, reason: 0, read: 867472, write: 54701, start: 1795166249557, end: 1795166258008, time: 8394, total: 144450, io: 0, control: 57, sent: 0, recv: 182
Die Markdown-Engine läuft jetzt über die RPC-Engine in `ast-engines/markdown`. Die gesamte Test-Suite läuft durch (399 Tests). Den Docstring habe ich erst nach diesem Lauf geändert, und er wurde danach nicht mehr getestet.

- **Neues Modul `rpc/_markdown.py`:** Es ist nach dem Muster von `_java.py` und `_rust.py` gebaut. `MarkdownProcess` startet die Engine lazy mit `node src/xy/ai/mcpc/ast/engine/Main.ts <port>` im Verzeichnis `ast-engines/markdown`. Der Port ist `8789` und lässt sich über `AST_MARKDOWN_PORT` ändern; 8787 und 8788 sind schon für Java und Rust belegt. Mit der Umgebungsvariable `NODE` kannst du ein anderes Node-Binary angeben.
- **Registrierung:** `generic/__init__.py` importiert `MarkdownEngine` jetzt aus `rpc._markdown`, und `rpc/__init__.py` exportiert ihn. Der Docstring in `generic/__init__.py` nennt jetzt die RPC-Back-Ends.
- **Alte Engine:** `generic/_markdown.py` habe ich nicht gelöscht. Sie ist nur nicht mehr eingebunden, so wie schon bei `generic/_java.py`.
- **Neuer Test `tests/test_ast_markdown_bridge.py`:** Er startet den echten Node-Prozess und prüft drei Dinge. Parse und Serialize ergeben dasselbe Dokument. Die gefundenen Abschnitte sind `Title`, `Section A` und `Section B`. Das Ersetzen eines Absatzes verändert die Nachbarabschnitte nicht.

`validates_syntax` habe ich auf dem Standardwert `False` gelassen. Jeder Text ist gültiges Markdown, anders als bei Java und Rust. Wenn die Engine über `/validate` doch Fehler melden soll, setzt du `validates_syntax = True` auf `MarkdownEngine`.

Die Engine selbst habe ich nicht angefasst. Ich habe nur `run.sh` und `Main.ts` gelesen, um den Startaufruf zu übernehmen. Die Knotentypen der neuen Engine (`Section`, `BlockGroup`) unterscheiden sich von denen der alten Tree-Sitter-Engine (`section`, `page`). Die Tests in `tests/` prüfen das nicht, bei Prompts oder Skripten, die die alten Typnamen oder Seiten-Knoten voraussetzen, solltest du es prüfen.
Teste die Anbindung umfänglich
SystemInit:  id=8d367516-0e0c-4674-80a5-34402faaa645, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich lese zuerst die RPC-Engine und die Engine-Basisklasse, damit die Tests die ganze Schnittstelle abdecken.
Control Request:
```yaml
id: 2d71-f2-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    ids:
    - RpcNode
    - RpcEngine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_process.py
    ids:
    - RpcProcess
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: "RpcEngine und Engine-Basis im Wortlaut lesen, um die zu testende API-Fläche zu kennen"
```
/answer 2d71-f2-0 allow
ReasoningToken: 197
Control Request:
```yaml
id: 2d71-f3-f2
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
        nodes:
        - id: ZjoNfB|uJIfID
          excert: '''Engine-agnostic model shared by every ``ast_*`` tool.\n\nThe tools address nod…'
        - id: oRSlHi|kUrRiw
          excert: from __future__ import annotations
        - id: K1Mt8u|UQj1Lh
          excert: "if TYPE_CHECKING:"
        - id: AstError
          excert: "class AstError(Exception):"
          docstring: "A user-facing, path-free error raised by the AST tools."
        - id: AstAmbiguous
          excert: "class AstAmbiguous(AstError):"
          docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
          children:
          - id: AstAmbiguous.FSy0jO|jchvnz
            excert: '''Raised when a text-based node search (no id given) finds several\n    unrelate…'
          - id: AstAmbiguous.init
            excert: "def __init__(self, message: str, candidates: list[str]) -> None:"
        - id: AstTextError
          excert: "class AstTextError(AstError):"
          docstring: Raised when a text/marker-based edit's search text could not be applied. Carrie…
          children:
          - id: AstTextError.xHQ70j|YI3gjQ
            excert: '"Raised when a text/marker-based edit''s search text could not be applied.\n\n  …'
          - id: AstTextError.init
            excert: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
        - id: Tree
          excert: "@dataclass class Tree:"
          docstring: "A parsed file/snippet plus the engine that owns it. Attributes: engine: The eng…"
        - id: Located
          excert: "@dataclass class Located:"
          docstring: A node with the engine-independent metadata the selectors match on. Attributes:…
        - id: OutlineNode
          excert: "@dataclass(frozen=True) class OutlineNode:"
          docstring: "One node in a structural (list/find/read) result. ``id`` is the node's unique, …"
        - id: line_range
          excert: "def line_range(loc: Located) -> str:"
          docstring: "Return ``loc``'s start line, or a ``\"start-end\"`` range if it spans several."
        - id: 1vtTBH|EXC8hj
          excert: _ID_CLEAN_RE = re.compile('\\W+')
        - id: hash
          excert: "def _hash(name: str, length: int) -> str:"
        - id: ThMOZm|vomnxc
          excert: _ID_HASH_ALPHABET = string.digits + string.ascii_letters
        - id: base62_hash
          excert: "def _base62_hash(text: str, length: int) -> str:"
          docstring: Base62 (0-9a-zA-Z) digest of ``text``.
        - id: content_hash
          excert: "def _content_hash(content: str, length: int=6) -> str:"
          docstring: "Base62 digest of ``content``, stable across unrelated tree edits."
        - id: content_prefix_hash
          excert: "def _content_prefix_hash(content: str, length: int=6) -> str:"
          docstring: Base62 digest of ``content``'s whitespace-stripped first/last 20 chars. Forms a…
        - id: mFoC5C|Xl4C8a
          excert: _ID_SUFFIX_RE = re.compile('_\\d+$')
        - id: id_prefix
          excert: "def _id_prefix(segment: str) -> str | None:"
          docstring: "An anonymous id segment's stable prefix (before ``'|'``), if any."
        - id: resolve_by_prefix
          excert: "def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':"
          docstring: Fallback selector for a ``target_id`` that matches no node exactly. Used when a…
        - id: id_segment
          excert: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
          docstring: "Return a unique-within-siblings id segment, name-based when feasible. A clean, …"
        - id: node_outline
          excert: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, with_type: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
          docstring: "Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``,…"
        - id: compact
          excert: "def _compact(value: Any) -> Any:"
          docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
        - id: to_dict
          excert: "def to_dict(node: OutlineNode) -> dict:"
          docstring: "Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."
        - id: TreeNode
          excert: "@dataclass class _TreeNode:"
        - id: build_forest
          excert: "def _build_forest(located: list[Located]) -> list[_TreeNode]:"
          docstring: Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes.
        - id: build_outline
          excert: "def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
          docstring: "Build the nested outline of ``located`` (source only if ``with_code``, lines on…"
        - id: outline_nodes
          excert: "def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
          docstring: "Convert a forest into OutlineNodes, collapsing non-expandable nodes to full sou…"
        - id: resolve_by_name
          excert: "def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:"
          docstring: Resolve ``key`` against node names when it doesn't match an id directly. Tries …
        - id: read_subtrees
          excert: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:"
          docstring: "Return one read subtree per resolvable ``keys`` entry. Each key is matched, in …"
        - id: matches
          excert: "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
        - id: find
          excert: "def find(tree: Tree, **filters: object) -> list[Located]:"
        - id: most_specific
          excert: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
          docstring: "Return the smallest node in *located* fully containing lines [lineno, end_linen…"
        - id: Engine
          excert: "class Engine(ABC):"
          docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
          children:
          - id: Engine.yeTUw1|1zTXCW
            excert: "'A parser back-end turning source into an addressable, mutable tree.\\n\\n    Str…"
          - id: Engine.dqC7gX|bz1VNl
            excert: "'#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'"
          - id: Engine.parse
            excert: "@abstractmethod def parse(self, source: str, path: Path | None=None) -> Tree:"
            docstring: "Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."
          - id: Engine.empty_tree
            excert: "@abstractmethod def empty_tree(self, path: Path | None=None) -> Tree:"
            docstring: "Return an empty tree, used when appending to a not-yet-existing file."
          - id: Engine.serialize
            excert: "@abstractmethod def serialize(self, tree: Tree) -> str:"
            docstring: Render ``tree`` back to source text for writing to disk.
          - id: Engine.validate
            excert: "@abstractmethod def validate(self, source: str) -> str | None:"
            docstring: "Return an error message if ``source`` is malformed, else ``None``."
          - id: Engine.locate_all
            excert: "@abstractmethod def locate_all(self, tree: Tree) -> list[Located]:"
            docstring: "Flatten ``tree`` into every addressable node, in document order."
          - id: Engine.is_definition
            excert: "def is_definition(self, node_type: str) -> bool:"
            docstring: Whether ``node_type`` is "def-like" enough for a ``excert`` to make sense. Defa…
          - id: Engine.signature
            excert: "@abstractmethod def signature(self, node: Any) -> str:"
            docstring: One-line rendering of ``node``'s header (or the node itself).
          - id: Engine.default_signature
            excert: "def default_signature(self, node: Any, limit: int=80) -> str:"
            docstring: "Fallback used when ``is_definition`` is False: the first non-blank line of the …"
          - id: Engine.docstring
            excert: "@abstractmethod def docstring(self, node: Any) -> str | None:"
            docstring: "Short docstring of ``node``, if the format has such a concept."
          - id: Engine.node_code
            excert: "@abstractmethod def node_code(self, node: Any) -> str:"
            docstring: Full source of a single ``node``.
          - id: Engine.replace
            excert: "@abstractmethod def replace(self, loc: Located, code: str) -> None:"
            docstring: Replace ``loc``'s node with ``code``.
          - id: Engine.insert
            excert: "@abstractmethod def insert(self, loc: Located, code: str, position: str) -> int:"
            docstring: Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted.
          - id: Engine.delete
            excert: "@abstractmethod def delete(self, loc: Located) -> None:"
            docstring: Delete ``loc``'s node from its container.
          - id: Engine.append
            excert: "@abstractmethod def append(self, tree: Tree, code: str) -> int:"
            docstring: Append ``code`` at ``tree``'s top level; return units appended.
          - id: Engine.apply_imports
            excert: "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':"
            docstring: Add missing / remove present import statements (``addImport``/``removeImport``)…
        - id: require_path
          excert: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
          docstring: "Validate a mandatory absolute path, raising :class:`AstError` on failure."
        - id: GaNJOk|lfM39j
          excert: "_CONTROL_CHAR_RE = re.compile('[\\x00-\\x08\\x0b\\x0c\\x0e-\\x1f\\x7f]')"
        - id: check_no_control_chars
          excert: "def check_no_control_chars(code: str) -> None:"
          docstring: Raise :class:`AstError` if ``code`` contains abnormal control characters (e.g. …
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: RpcNode
          type: ClassDef
          code: |-
            @dataclass
            class _RpcNode:
                """One node as reported by ``POST /nodes``.

                Carries everything :meth:`RpcEngine.signature`/``docstring``/``node_code``
                need without a further round-trip: signature/docstring come straight from
                the server, and code -- unless eagerly included -- is sliced from the
                already-known ``tree.source`` by line range.
                """
                tree: Tree
                lineno: int
                end_lineno: int
                signature_text: str | None
                docstring_text: str | None
                code: str | None
        - id: RpcEngine
          type: ClassDef
          excert: "class RpcEngine(Engine):"
          docstring: One RPC/PIC engine process exposed through the common :class:`Engine` API. Stru…
          children:
          - id: RpcEngine.95ZeHe|fDr6kO
            type: statements
            code: "\"One RPC/PIC engine process exposed through the common :class:`Engine` API.\\n\\n    Structure, signatures and docstrings are computed remotely; mutations\\n    send the current full source and node id and replace ``tree.source``\\n    with the server's answer, mirroring how :class:`TreeSitterEngine` edits\\n    ``Tree.source`` in place so :meth:`serialize` just returns it.\\n    \""
          - id: RpcEngine.init
            type: FunctionDef
            code: |-
              def __init__(self, symbol: str, process: RpcProcess) -> None:
                  self.symbol = symbol
                  self.name = symbol
                  self._process = process
                  self._client: AppendInfoNodesValidateClientImpl | None = None
                  '#: node_type -> is_definition, learned from the last ``locate_all`` calls'
                  '#: (Engine.is_definition only takes a type name, not a specific node).'
                  self._is_definition: dict[str, bool] = {}
          - id: RpcEngine.rpc
            type: FunctionDef
            code: |-
              def _rpc(self) -> AppendInfoNodesValidateClientImpl:
                  if self._client is None:
                      self._client = AppendInfoNodesValidateClientImpl(self._process.base_url())
                  return self._client
          - id: RpcEngine.unwrap
            type: FunctionDef
            code: |-
              @staticmethod
              def _unwrap(response: Any) -> Any:
                  """Return a 200 response's typed body, or raise on any other status."""
                  code = response.status_code
                  getter = getattr(response, f'get_code_{code}', None)
                  view = getter() if getter else None
                  payload = view.get_Json() if view is not None else None
                  if code == '200':
                      return payload
                  error = payload if payload is not None else Error({})
                  message = error.get_Message() or f'AST engine request failed (HTTP {code}).'
                  if code == '409':
                      candidates = error.get_Candidates()
                      items = [candidates.get(i) for i in range(len(candidates))] if candidates is not None else []
                      raise AstAmbiguous(message, items)
                  raise AstError(message)
          - id: RpcEngine.MltvBv|wKemBs
            type: statements
            code: '''# -- Engine ------------------------------------------------------------'''
          - id: RpcEngine.parse
            type: FunctionDef
            code: |-
              def parse(self, source: str, path: Path | None=None) -> Tree:
                  error = self.validate(source)
                  if error:
                      raise AstError(error)
                  return Tree(self, None, source, path)
          - id: RpcEngine.empty_tree
            type: FunctionDef
            code: |-
              def empty_tree(self, path: Path | None=None) -> Tree:
                  return Tree(self, None, '', path)
          - id: RpcEngine.serialize
            type: FunctionDef
            code: |-
              def serialize(self, tree: Tree) -> str:
                  return tree.source
          - id: RpcEngine.validate
            type: FunctionDef
            code: |-
              def validate(self, source: str) -> str | None:
                  request = SourceRequest()
                  request.set_Source(source)
                  payload = self._unwrap(self._rpc().validateSource(request))
                  return payload.get_Error() if payload is not None else None
          - id: RpcEngine.is_definition
            type: FunctionDef
            code: |-
              def is_definition(self, node_type: str) -> bool:
                  return self._is_definition.get(node_type, True)
          - id: RpcEngine.locate_all
            type: FunctionDef
            code: |-
              def locate_all(self, tree: Tree) -> list[Located]:
                  request = LocateRequest()
                  request.set_Source(tree.source)
                  if tree.path is not None:
                      request.set_Path(str(tree.path))
                  request.set_IncludeCode(False)
                  payload = self._unwrap(self._rpc().listNodes(request))
                  nodes = payload.get_Nodes() if payload is not None else None
                  count = len(nodes) if nodes is not None else 0
                  results: list[Located] = []
                  sibling_index: dict[str, int] = {}
                  for i in range(count):
                      node = nodes.get(i)
                      node_id = node.get_Id()
                      node_type = node.get_Type()
                      self._is_definition[node_type] = bool(node.get_IsDefinition())
                      parent_id = node_id.rsplit('.', 1)[0] if '.' in node_id else ''
                      index = sibling_index.get(parent_id, 0)
                      sibling_index[parent_id] = index + 1
                      lineno = node.get_Lineno()
                      end_lineno = node.get_EndLineno()
                      results.append(
                          Located(
                              tree=tree,
                              node=_RpcNode(
                                  tree=tree,
                                  lineno=lineno,
                                  end_lineno=end_lineno,
                                  signature_text=node.get_Signature(),
                                  docstring_text=node.get_Docstring(),
                                  code=node.get_Code()),
                              parent=None,
                              index=index,
                              node_id=node_id,
                              node_type=node_type,
                              name=node.get_Name(),
                              lineno=lineno,
                              end_lineno=end_lineno,
                              parent_type=node.get_ParentType(),
                              expandable=bool(
                                  node.get_Expandable())))
                  return results
          - id: RpcEngine.signature
            type: FunctionDef
            code: |-
              def signature(self, node: _RpcNode) -> str:
                  return node.signature_text if node.signature_text else self.default_signature(node)
          - id: RpcEngine.docstring
            type: FunctionDef
            code: |-
              def docstring(self, node: _RpcNode) -> str | None:
                  return node.docstring_text
          - id: RpcEngine.node_code
            type: FunctionDef
            code: |-
              def node_code(self, node: _RpcNode) -> str:
                  if node.code is not None:
                      return node.code
                  lines = node.tree.source.splitlines(keepends=True)
                  return ''.join(lines[node.lineno - 1:node.end_lineno])
          - id: RpcEngine.replace
            type: FunctionDef
            code: |-
              def replace(self, loc: Located, code: str) -> None:
                  check_no_control_chars(code)
                  request = CodeRequest()
                  request.set_Source(loc.tree.source)
                  if loc.tree.path is not None:
                      request.set_Path(str(loc.tree.path))
                  request.set_Code(code)
                  payload = self._unwrap(self._rpc().replaceNode(loc.node_id, request))
                  loc.tree.source = payload.get_Source()
          - id: RpcEngine.insert
            type: FunctionDef
            code: |-
              def insert(self, loc: Located, code: str, position: str) -> int:
                  check_no_control_chars(code)
                  request = InsertRequest()
                  request.set_Source(loc.tree.source)
                  if loc.tree.path is not None:
                      request.set_Path(str(loc.tree.path))
                  request.set_Code(code)
                  request.set_Position(PositionEnum.BEFORE if position == 'before' else PositionEnum.AFTER)
                  payload = self._unwrap(self._rpc().insertRelativeToNode(loc.node_id, request))
                  loc.tree.source = payload.get_Source()
                  return payload.get_UnitsInserted()
          - id: RpcEngine.delete
            type: FunctionDef
            code: |-
              def delete(self, loc: Located) -> None:
                  request = SourceRequest()
                  request.set_Source(loc.tree.source)
                  if loc.tree.path is not None:
                      request.set_Path(str(loc.tree.path))
                  payload = self._unwrap(self._rpc().deleteNode(loc.node_id, request))
                  loc.tree.source = payload.get_Source()
          - id: RpcEngine.append
            type: FunctionDef
            code: |-
              def append(self, tree: Tree, code: str) -> int:
                  check_no_control_chars(code)
                  request = CodeRequest()
                  request.set_Source(tree.source)
                  if tree.path is not None:
                      request.set_Path(str(tree.path))
                  request.set_Code(code)
                  payload = self._unwrap(self._rpc().appendTopLevel(request))
                  tree.source = payload.get_Source()
                  return payload.get_UnitsAppended()
          - id: RpcEngine.apply_imports
            type: FunctionDef
            code: |-
              def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
                  return apply_rpc(self, tree, add, remove)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_process.py
        nodes:
        - id: RpcProcess
          type: ClassDef
          excert: "class RpcProcess(ABC):"
          docstring: "Supervises a single long-lived RPC engine subprocess, started on first use. Thr…"
          children:
          - id: RpcProcess.DXlVqi|lLI6Mv
            type: statements
            code: |-
              'Supervises a single long-lived RPC engine subprocess, started on first use.\n\n    Thread-safe: concurrent callers are serialized so only one subprocess is\n    ever spawned. Once started, the process is left running (registered via\n    :mod:`atexit`) for the lifetime of this Python process.\n    '
              host = '127.0.0.1'
              startup_timeout = 30.0
          - id: RpcProcess.init
            type: FunctionDef
            code: |-
              def __init__(self) -> None:
                  self._lock = threading.Lock()
                  self._process: subprocess.Popen | None = None
                  self._port: int | None = None
          - id: RpcProcess.default_port
            type: FunctionDef
            code: |-
              @abstractmethod
              def default_port(self) -> int:
                  """TCP port the engine should listen on."""
          - id: RpcProcess.command
            type: FunctionDef
            code: |-
              @abstractmethod
              def command(self, port: int) -> list[str]:
                  """Argv that launches the engine, listening on ``port``."""
          - id: RpcProcess.cwd
            type: FunctionDef
            code: |-
              def cwd(self) -> str | None:
                  """Working directory for the subprocess; ``None`` inherits the caller's."""
                  return None
          - id: RpcProcess.base_url
            type: FunctionDef
            code: |-
              def base_url(self) -> str:
                  """URL of the (lazily started) engine's API root, e.g. ``http://127.0.0.1:8787``.

                      Note: unlike the openapi document's example server URL, ``ast-java``
                      mounts its routes at the root, not under ``/v1``.
                      """
                  self._ensure_running()
                  return f'http://{self.host}:{self._port}'
          - id: RpcProcess.ensure_running
            type: FunctionDef
            code: |-
              def _ensure_running(self) -> None:
                  with self._lock:
                      if self._process is not None and self._process.poll() is None:
                          return
                      port = self.default_port()
                      argv = self.command(port)
                      logger.info('%s: starting %s', self.__class__.__name__, ' '.join(argv))
                      self._process = subprocess.Popen(
                          argv,
                          cwd=self.cwd(),
                          stdout=subprocess.DEVNULL,
                          stderr=subprocess.PIPE,
                          text=True)
                      self._port = port
                      atexit.register(self._terminate)
                      self._wait_until_ready(port)
                      logger.info('%s: ready on port %d (pid %d)', self.__class__.__name__, port, self._process.pid)
          - id: RpcProcess.wait_until_ready
            type: FunctionDef
            code: |-
              def _wait_until_ready(self, port: int) -> None:
                  deadline = time.monotonic() + self.startup_timeout
                  while time.monotonic() < deadline:
                      if self._process.poll() is not None:
                          stderr = self._process.stderr.read() if self._process.stderr else ''
                          logger.error(
                              '%s: exited during startup (code %s): %s',
                              self.__class__.__name__,
                              self._process.returncode,
                              stderr.strip())
                          raise RuntimeError(
                              f'{self.__class__.__name__} exited during startup (code {self._process.returncode}): {stderr.strip()}')
                      try:
                          with socket.create_connection((self.host, port), timeout=0.5):
                              return
                      except OSError:
                          time.sleep(0.1)
                  logger.error('%s: did not become ready on port %d within %ss', self.__class__.__name__, port, self.startup_timeout)
                  self._terminate()
                  raise TimeoutError(f'{self.__class__.__name__} did not become ready on port {port} within {self.startup_timeout}s.')
          - id: RpcProcess.terminate
            type: FunctionDef
            code: |-
              def _terminate(self) -> None:
                  process, self._process = (self._process, None)
                  if process is not None and process.poll() is None:
                      logger.info('%s: terminating pid %d', self.__class__.__name__, process.pid)
                      process.terminate()
                      try:
                          process.wait(timeout=5)
                      except subprocess.TimeoutExpired:
                          process.kill()
```
/answer 2d71-f3-f2 allow
Control Request:
```yaml
id: 2d71-f4-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ast|edit|conftest
  reason: Vorhandene Tests für AST-Tools als Vorlage für Tool-Ebene-Tests erfassen
```
/answer 2d71-f4-0 allow
Control Request:
```yaml
id: 2d71-f5-f4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
      entries:
      - ".:"
      - conftest.py
      - test_ast_engines.py
      - test_ast_imports.py
      - test_ast_imports_rpc.py
      - test_ast_markdown_bridge.py
      - test_ast_retrieve.py
      - test_ast_rust_bridge.py
      - test_edit_diagnostics.py
      - test_edit_tolerance.py
      - test_grep_with_ast.py
```
/answer 2d71-f5-f4 allow
Control Request:
```yaml
id: 2d71-f6-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/conftest.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "Bestehende Testmuster (Tool-Ebene, Engines, Imports-RPC) und Tool-Einstiegspunkte erfassen"
```
/answer 2d71-f6-0 allow
Control Request:
```yaml
id: 2d71-f7-f6
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
        nodes:
        - id: yGxCcZ|eOnjml
          excert: '''Tests for the multi-engine ``ast_*`` tool family.\n\nCovers engine selection b…'
        - id: 0ekC7u|9IXYUc
          excert: from __future__ import annotations
        - id: Rpzfkv|9iKcDL
          excert: '''# Make the ``src`` layout importable without requiring an editable install.'''
        - id: 8Pl1k7|GddZCa
          excert: from xy.ai.mcpc.tools.ast import core
        - id: 3Mpw8b|t0uVQE
          excert: "'# noqa: E402'"
        - id: Jod45v|byUhT4
          excert: "from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create"
        - id: 3Mpw8b|t0uVQE_1
          excert: "'# noqa: E402'"
        - id: NREYuD|nugLQv
          excert: "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
        - id: 3Mpw8b|t0uVQE_2
          excert: "'# noqa: E402'"
        - id: GuxMWv|egumyg
          excert: "from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks"
        - id: 3Mpw8b|t0uVQE_3
          excert: "'# noqa: E402'"
        - id: LgnMR1|79MOp3
          excert: from xy.ai.mcpc.tools.ast.find import ast_find
        - id: 3Mpw8b|t0uVQE_4
          excert: "'# noqa: E402'"
        - id: cNhh2C|88ZZ6p
          excert: "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
        - id: 3Mpw8b|t0uVQE_5
          excert: "'# noqa: E402'"
        - id: JD7CYF|Tclncr
          excert: from xy.ai.mcpc.tools.ast.outline import ast_outline
        - id: 3Mpw8b|t0uVQE_6
          excert: "'# noqa: E402'"
        - id: oEgkx1|6grABN
          excert: "from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read"
        - id: 3Mpw8b|t0uVQE_7
          excert: "'# noqa: E402'"
        - id: mO2aS8|5v2aZQ
          excert: "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
        - id: 3Mpw8b|t0uVQE_8
          excert: "'# noqa: E402'"
        - id: Txu2YL|gFjfI4
          excert: from xy.ai.mcpc.tools.ast.validate import ast_validate
        - id: jS0K2v|hUR3iy
          excert: PY_SOURCE = 'import os\n\nclass A:\n    def foo(self):\n        x = 1\n        …
        - id: py_file
          excert: "@pytest.fixture def py_file(tmp_path: Path) -> str:"
        - id: json_file
          excert: "@pytest.fixture def json_file(tmp_path: Path) -> str:"
        - id: iz0N40|zFMezT
          excert: '''# --------------------------------------------------------------------------- …'
        - id: test_engine_selection_by_extension
          excert: "def test_engine_selection_by_extension(tmp_path: Path):"
        - id: h766ca315
          excert: "def test_engine_selection_unsupported_extension(tmp_path: Path):"
          docstring: Unknown extensions no longer raise; they fall back to the whole-file plain-text…
        - id: test_snippet_defaults_to_python
          excert: "def test_snippet_defaults_to_python():"
        - id: iz0N40|tC4SaG
          excert: '''# --------------------------------------------------------------------------- …'
        - id: test_python_outline
          excert: "def test_python_outline(py_file):"
        - id: test_python_find_by_name_and_type
          excert: "def test_python_find_by_name_and_type(py_file):"
        - id: test_python_read_returns_source
          excert: "def test_python_read_returns_source(py_file):"
        - id: test_python_full_crud_roundtrip
          excert: "def test_python_full_crud_roundtrip(py_file):"
        - id: iz0N40|icJAmV
          excert: '''# --------------------------------------------------------------------------- …'
        - id: test_generic_uses_treesitter_engine
          excert: "def test_generic_uses_treesitter_engine(json_file):"
        - id: test_generic_qualified_names
          excert: "def test_generic_qualified_names(json_file):"
        - id: test_generic_replace_by_qualified_name
          excert: "def test_generic_replace_by_qualified_name(json_file):"
        - id: test_generic_edit_between_markers
          excert: "def test_generic_edit_between_markers(json_file):"
        - id: test_generic_validate_reports_error
          excert: "def test_generic_validate_reports_error(tmp_path: Path):"
        - id: iz0N40|KdNRVD
          excert: '''# --------------------------------------------------------------------------- …'
        - id: hf5a87d78
          excert: "def test_addressing_by_id_matches_located_node(json_file):"
        - id: test_addressing_by_id_is_parser_agnostic
          excert: "def test_addressing_by_id_is_parser_agnostic(py_file):"
        - id: test_list_filters_by_type
          excert: "def test_list_filters_by_type(py_file):"
        - id: iz0N40|gINtIU
          excert: '''# --------------------------------------------------------------------------- …'
        - id: h824e2be2
          excert: "def test_cache_returns_same_tree_until_changed(py_file):"
        - id: test_cache_is_shared_across_engines
          excert: "def test_cache_is_shared_across_engines(py_file, json_file):"
        - id: iz0N40|SjpxQG
          excert: '''# --------------------------------------------------------------------------- …'
        - id: test_create_file_typescript
          excert: "def test_create_file_typescript(tmp_path: Path):"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
        nodes:
        - id: DPWwCQ|KJ4Rhy
          excert: '''Import handling against the real Java (JavaParser) and Rust (syn) RPC processe…'
        - id: 0ekC7u|9IXYUc
          excert: from __future__ import annotations
        - id: EDFjeC|StWP8k
          excert: "_SRC = Path(__file__).resolve().parents[1] / 'src'"
        - id: pb9L1n|aXbIio
          excert: from xy.ai.mcpc.tools.ast.base import AstError
        - id: 3Mpw8b|t0uVQE
          excert: "'# noqa: E402'"
        - id: fzPWql|imcXpO
          excert: from xy.ai.mcpc.tools.ast.generic import get_engine
        - id: T7uK1P|kxMiK0
          excert: JAVA = 'package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic c…
        - id: java
          excert: "@pytest.fixture(scope='module') def java():"
        - id: rust
          excert: "@pytest.fixture(scope='module') def rust():"
        - id: test_java_add_present_and_remove
          excert: "def test_java_add_present_and_remove(java):"
        - id: hc0c9aa89
          excert: "def test_java_without_imports_goes_behind_package(java):"
        - id: test_java_without_package_and_imports
          excert: "def test_java_without_package_and_imports(java):"
        - id: h836b0c56
          excert: "def test_java_non_import_raises_and_leaves_source(java):"
        - id: test_rust_add_present_and_remove
          excert: "def test_rust_add_present_and_remove(rust):"
        - id: test_rust_without_imports
          excert: "def test_rust_without_imports(rust):"
        - id: h240d4d24
          excert: "def test_rust_non_import_raises_and_leaves_source(rust):"
        - id: test_rust_keeps_surrounding_formatting
          excert: "def test_rust_keeps_surrounding_formatting(rust):"
        - id: he73488e6
          excert: "def test_rust_import_does_not_split_doc_comment_from_item(rust):"
        - id: test_java_keeps_surrounding_formatting
          excert: "def test_java_keeps_surrounding_formatting(java):"
        - id: test_java_add_and_remove_in_one_pass
          excert: "def test_java_add_and_remove_in_one_pass(java):"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
        nodes:
        - id: DaYVs5|eLrdax
          excert: from __future__ import annotations
        - id: write_sample
          excert: "def _write_sample(tmp_path) -> str:"
        - id: retrieve
          excert: "def _retrieve(registry: ToolRegistry, arguments: dict):"
        - id: test_schema_has_all_sections
          excert: "def test_schema_has_all_sections(registry: ToolRegistry):"
        - id: test_empty_call_fails
          excert: "def test_empty_call_fails(registry: ToolRegistry):"
        - id: test_outline_and_find_in_one_call
          excert: "def test_outline_and_find_in_one_call(registry: ToolRegistry, tmp_path):"
        - id: h1d9c184c
          excert: "def test_search_returns_node_ids_and_read_uses_them(registry: ToolRegistry, tmp_path):"
        - id: test_multiple_find_queries_are_merged
          excert: "def test_multiple_find_queries_are_merged(registry: ToolRegistry, tmp_path):"
        - id: h4eb01286
          excert: "def test_failing_section_does_not_abort_others(registry: ToolRegistry, tmp_path):"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/conftest.py
        nodes:
        - id: qmgX3W|tqzV3O
          excert: "'Test setup: make the ``src`` layout importable without installation.'"
        - id: 5VUlLh|r01CsA
          excert: from __future__ import annotations
        - id: iRf0tk|qB8MoX
          excert: SRC = Path(__file__).resolve().parent.parent / 'src'
        - id: x3hNJ6|DzQJSD
          excert: import pytest
        - id: 3Mpw8b|t0uVQE
          excert: "'# noqa: E402'"
        - id: Rvzd9A|64yxm4
          excert: from xy.ai.mcpc.cli.manager import CliSessionManager
        - id: 3Mpw8b|t0uVQE_1
          excert: "'# noqa: E402'"
        - id: ISA9QJ|HpQxxJ
          excert: from xy.ai.mcpc.config import ServerConfig
        - id: 3Mpw8b|t0uVQE_2
          excert: "'# noqa: E402'"
        - id: u9WjXo|amOnuc
          excert: from xy.ai.mcpc.server.session import SessionStore
        - id: 3Mpw8b|t0uVQE_3
          excert: "'# noqa: E402'"
        - id: CMkg8Q|JCVbVw
          excert: from xy.ai.mcpc.tools import register_tools
        - id: 3Mpw8b|t0uVQE_4
          excert: "'# noqa: E402'"
        - id: HuFMzr|NS1kqD
          excert: "from xy.ai.mcpc.tools.agent.profiles import DEFAULT_PROFILES, ProfileRegistry"
        - id: 3Mpw8b|t0uVQE_5
          excert: "'# noqa: E402'"
        - id: 53fjeN|DGVP0G
          excert: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - id: 3Mpw8b|t0uVQE_6
          excert: "'# noqa: E402'"
        - id: Sf6qeQ|UMWgY5
          excert: from xy.ai.mcpc.tools.tool_context import AppEnvironment
        - id: 3Mpw8b|t0uVQE_7
          excert: "'# noqa: E402'"
        - id: PQJTLs|q1ToOF
          excert: from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        - id: registry
          excert: "@pytest.fixture def registry() -> ToolRegistry:"
          docstring: "A ``ToolRegistry`` populated by ``register_tools()``, wired like the real serve…"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
        nodes:
        - id: foxd99|7aiScK
          excert: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
        - id: 1EBKpT|N6PPdc
          excert: from __future__ import annotations
        - id: AiyIX3|Ohci6H
          excert: "__all__ = ["
        - id: ImportOutcome
          excert: "@dataclass class ImportOutcome:"
          docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
        - id: string_list
          excert: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
        - id: parse_import_args
          excert: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
          docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
        - id: ImportCheckedBatchResult
          excert: "class ImportCheckedBatchResult(BatchResult[R, E]):"
          docstring: Batch result whose call-wide warning is derived from its items' unchecked impor…
          children:
          - id: ImportCheckedBatchResult.AtNmpo|lNEn03
            excert: '"Batch result whose call-wide warning is derived from its items'' unchecked impo…'
          - id: ImportCheckedBatchResult.derived_warnings
            excert: "def derived_warnings(self) -> tuple[str, ...]:"
        - id: import_entry
          excert: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
          docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
        - id: S64wGg|AyaH5Z
          excert: _SPACE = re.compile('\\s+')
        - id: normalize_import
          excert: "def normalize_import(text: str) -> str:"
          docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
        - id: checked
          excert: "def _checked(statement: str) -> str:"
        - id: unique
          excert: "def _unique(statements: list[str]) -> list[str]:"
        - id: statement_spans
          excert: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
          docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
        - id: line_extent
          excert: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
          docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
        - id: delete_span
          excert: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
          docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
        - id: next_line_start
          excert: "def _next_line_start(data: bytes, offset: int) -> int:"
        - id: line_starts
          excert: "def _line_starts(data: bytes) -> list[int]:"
        - id: store
          excert: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
        - id: apply_generic
          excert: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
          docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
        - id: prologue_end
          excert: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
          docstring: Offset after leading shebang/directives and header comments (comments followed …
        - id: snippet
          excert: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
        - id: apply_treesitter
          excert: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
          docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
        - id: fs86A7|AnrSrF
          excert: "'#: Node types under which the RPC processes report a run of imports.'"
        - id: rpc_statement
          excert: "def _rpc_statement(statement: str) -> str:"
        - id: rpc_present
          excert: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
          docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
        - id: g7FpPW|tkBF2z
          excert: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
        - id: leading_start
          excert: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
          docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
        - id: rpc_insertion
          excert: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
          docstring: "Edit that places ``code``: behind the last import, else behind ``package``/modu…"
        - id: apply_rpc
          excert: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
          docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
        nodes:
        - id: V6fdSR|z5Q7Tq
          excert: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engi…"
        - id: OsZYRc|1ZfHhi
          excert: from __future__ import annotations
        - id: 64y9R1|MAcfZT
          excert: from xy.ai.mcpc.tools.ast import python
        - id: fr6KXO|uhDdPM
          excert: logger = logging.getLogger(__name__)
        - id: engine_for_path
          excert: "def engine_for_path(path: Path) -> Engine:"
          docstring: Return the engine responsible for ``path`` based on its extension.
        - id: CacheEntry
          excert: "@dataclass class _CacheEntry:"
        - id: AstCache
          excert: "class AstCache:"
          docstring: Content-hash validated cache of parsed trees keyed by absolute path.
          children:
          - id: AstCache.AzRmHX|ayni5v
            excert: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
          - id: AstCache.init
            excert: "def __init__(self) -> None:"
          - id: AstCache.get_tree
            excert: "def get_tree(self, path: Path) -> Tree:"
          - id: AstCache.save
            excert: "def save(self, path: Path, tree: Tree) -> str:"
            docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
          - id: AstCache.invalidate
            excert: "def invalidate(self, path: Path) -> None:"
        - id: dRq1FJ|5EG9eJ
          excert: "'#: Process-wide shared cache instance.'"
        - id: load
          excert: "def load(path_str: str) -> tuple[Path, Tree]:"
          docstring: Resolve *path_str* and return it together with its cached tree.
        - id: parse_source
          excert: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
          docstring: Parse *source* with *engine* (Python by default for path-less snippets).
        - id: parse_for
          excert: "def parse_for(path_str: str, code: str) -> Tree:"
          docstring: Parse *code* with the engine selected for *path_str*'s extension.
        - id: locate_all
          excert: "def locate_all(tree: Tree) -> list[Located]:"
        - id: edit_node_source
          excert: "def edit_node_source(loc: Located) -> str:"
        - id: relocate
          excert: "def relocate(tree: Tree, old: Located) -> Located | None:"
          docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
        - id: replace_node
          excert: "def replace_node(loc: Located, code: str) -> str | None:"
          docstring: Replace ``loc``'s node with ``code``; return its new id.
        - id: insert_node
          excert: "def insert_node(loc: Located, code: str, position: str) -> int:"
        - id: delete_node
          excert: "def delete_node(loc: Located) -> None:"
        - id: append_nodes
          excert: "def append_nodes(tree: Tree, code: str) -> int:"
        - id: apply_imports
          excert: "def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:"
          docstring: Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``. …
        - id: empty_tree
          excert: "def empty_tree(path: Path) -> Tree:"
        - id: validate_source
          excert: "def validate_source(path: Path, source: str) -> str | None:"
```
/answer 2d71-f7-f6 allow
ReasoningToken: 222
Control Request:
```yaml
id: 2d71-f8-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: '''\.md''|"\.md"|markdown'
    include:
    - '*.py'
    exclude:
    - '**/generic/_markdown.py'
    - '**/rpc/_markdown.py'
    - '**/test_ast_markdown_bridge.py'
    limit: 30
  reason: Verwendungen von md/markdown-Dateiendungen in der Engine-Zuordnung
```
/answer 2d71-f8-0 allow
Control Request:
```yaml
id: 2d71-f9-f8
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
        matches:
        - path: xy/ai/mcpc/config.py
          matches:
          - "'# : Node.js package environment used by the ``markdown`` tool (provides'"
          - "markdown_env_dir: Path = field(default=Path('/home/user/xyan/xy.ai.workbench/claude-code/markdown/remark'))"
          - "kwargs['markdown_env_dir'] = Path(env['MCPC_MARKDOWN_ENV_DIR'])"
        - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
          matches:
          - _DESCRIPTION = "Read a webpage's full content as clean markdown. Use to read any URL."
          - "'#: Matches the start of one item in the markdown-ish plain-text format the'"
          - "'#: markdown H1 title line immediately followed by a ``URL:`` line, a blank'"
          - "\"#: line, then the page's extracted markdown content; consecutive urls are\""
          - <page content as markdown>
          - '"""Read one or more webpages'' full content as clean markdown.'
        - path: xy/ai/mcpc/tools/mcp/exa/search.py
          matches:
          - "'#: markdown-ish plain-text format the Exa remote MCP server sends when it'"
        - path: xy/ai/mcpc/tools/mcp/exa/core.py
          matches:
          - parse Exa's markdown-ish plain-text format into result dicts instead of
          - "\"Exa '%s': remote sent unstructured text instead of structured data; recovered %d item(s) via markdown fallback parsing.\","
          - "\"Exa '%s': remote sent unstructured text but the markdown fallback parser found no items in it (keys=%s).\","
        - path: xy/ai/mcpc/tools/ast/rpc/_markdown.py
          matches:
          - "\"\"\"Markdown engine: remark-based ``ast-engines/markdown`` process, driven over RPC/PIC."
          - Replaces the old tree-sitter :class:`~xy.ai.mcpc.tools.ast.generic._markdown.
          - "``ast-engines/markdown/run.sh`` does, minus the shell: we invoke ``node``"
          - _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'markdown'
          - '"""Starts the ``ast-engines/markdown`` remark engine on Node.js."""'
          - "\"\"\"Markdown, parsed and mutated by remark via the ``ast-engines/markdown`` RPC process.\"\"\""
          - "super().__init__('markdown', _PROCESS)"
        - path: xy/ai/mcpc/tools/ast/rpc/__init__.py
          matches:
          - from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
        - path: xy/ai/mcpc/tools/ast/generic/_markdown.py
          matches:
          - super().__init__('markdown')
        - path: xy/ai/mcpc/tools/ast/generic/__init__.py
          matches:
          - from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
          - "'.md': 'markdown',"
          - "'.markdown': 'markdown',"
          - "type[Engine]] = {'markdown': MarkdownEngine,"
        - path: xy/ai/mcpc/tools/ast/generic/_engine.py
          matches:
          - :mod:`xy.ai.mcpc.tools.ast.generic._markdown` for Markdown's Page/Heading
          - "different (possibly synthetic, see ``_markdown._RootHolder``) node set.\"\"\""
        - path: xy/ai/mcpc/tools/process.py
          matches:
          - "``bash``, ``python`` and ``markdown`` all do the same thing: run a child"
        - path: xy/ai/mcpc/tools/skills/__init__.py
          matches:
          - "'markdown_format']"
        warnings:
        - "Limit of 30 matches reached (total 54); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 2d71-f9-f8 allow
ReasoningToken: 128
Control Request:
```yaml
id: 2d71-fa-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
    ids:
    - json_file
    - test_generic_uses_treesitter_engine
    - test_python_full_crud_roundtrip
    - test_generic_replace_by_qualified_name
    - test_generic_edit_between_markers
    - test_generic_validate_reports_error
    - test_create_file_typescript
    - test_engine_selection_by_extension
    - h824e2be2
    - test_cache_is_shared_across_engines
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
    ids:
    - EDFjeC|StWP8k
    - java
    - test_java_add_present_and_remove
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
    ids:
    - retrieve
    - test_outline_and_find_in_one_call
    - h1d9c184c
  reason: Aufrufmuster der Tool-Ebene (ast_outline/find/read/replace/insert/delete/edit_marks/create/validate) aus bestehenden Tests ablesen
```
/answer 2d71-fa-0 allow
Control Request:
```yaml
id: 2d71-fb-fa
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
        nodes:
        - id: json_file
          type: FunctionDef
          code: |-
            @pytest.fixture
            def json_file(tmp_path: Path) -> str:
                path = tmp_path / 'c.json'
                path.write_text(JSON_SOURCE, encoding='utf-8')
                return str(path)
        - id: test_generic_uses_treesitter_engine
          type: FunctionDef
          code: |-
            def test_generic_uses_treesitter_engine(json_file):
                _, tree = core.load(json_file)
                assert tree.engine.name == 'tree-sitter:json'
        - id: test_python_full_crud_roundtrip
          type: FunctionDef
          code: |-
            def test_python_full_crud_roundtrip(py_file):
                ast_replace(items=[ReplaceItem(path=py_file, source='def bar():\n    return 42', id='bar')])
                ast_edit_marks(
                    items=[
                        EditMarksItem(
                            path=py_file,
                            start_marker='x = 1',
                            end_marker='return x',
                            content='return 99',
                            id='A.foo')])
                ast_insert(items=[InsertItem(path=py_file, source='z = 5', id='bar', position='after')])
                import_id = ast_find(paths=[py_file], node_type='imports').files[0].nodes[0].id
                ast_delete(items=[DeleteItem(path=py_file, id=import_id)])
                assert ast_validate([py_file]).all_ok
                text = Path(py_file).read_text()
                assert 'return 42' in text
                assert 'return 99' in text
                assert 'z = 5' in text
                assert 'import os' not in text
        - id: test_generic_replace_by_qualified_name
          type: FunctionDef
          code: |-
            def test_generic_replace_by_qualified_name(json_file):
                _, tree = core.load(json_file)
                obj_id = core.locate_all(tree)[0].node_id
                ast_replace(items=[ReplaceItem(path=json_file, source='{"a": 111}', id=obj_id)])
                assert '"a": 111' in Path(json_file).read_text()
                assert ast_validate([json_file]).all_ok
        - id: test_generic_edit_between_markers
          type: FunctionDef
          code: |-
            def test_generic_edit_between_markers(json_file):
                _, tree = core.load(json_file)
                obj_id = core.locate_all(tree)[0].node_id
                ast_edit_marks(
                    items=[
                        EditMarksItem(
                            path=json_file,
                            start_marker='"a": 1',
                            end_marker='"b": 2',
                            content='"a": 10,\n    "b": 20',
                            id=obj_id)])
                text = Path(json_file).read_text()
                assert '"a": 10' in text and '"b": 20' in text
                assert ast_validate([json_file]).all_ok
        - id: test_generic_validate_reports_error
          type: FunctionDef
          code: |-
            def test_generic_validate_reports_error(tmp_path: Path):
                bad = tmp_path / 'bad.json'
                bad.write_text('{"a": }\n', encoding='utf-8')
                check = ast_validate([str(bad)]).files[0]
                assert not check.ok
                assert check.error
        - id: test_create_file_typescript
          type: FunctionDef
          code: |-
            def test_create_file_typescript(tmp_path: Path):
                ts = tmp_path / 'app.ts'
                ast_create(
                    items=[
                        CreateFileItem(
                            path=str(ts),
                            source='function greet(name: string): string {\n  return name;\n}\n')])
                hits = ast_find(paths=[str(ts)], node_type='function_declaration').files[0].nodes
                assert hits and hits[0].id == 'greet'
                assert ast_validate([str(ts)]).all_ok
        - id: test_engine_selection_by_extension
          type: FunctionDef
          code: |-
            def test_engine_selection_by_extension(tmp_path: Path):
                assert core.engine_for_path(tmp_path / 'x.py').name == 'python'
                assert core.engine_for_path(tmp_path / 'x.pyi').name == 'python'
                assert core.engine_for_path(tmp_path / 'x.json').name == 'tree-sitter:json'
                assert core.engine_for_path(tmp_path / 'x.yaml').name == 'tree-sitter:yaml'
        - id: h824e2be2
          type: FunctionDef
          code: |-
            def test_cache_returns_same_tree_until_changed(py_file):
                path = Path(py_file)
                first = core.CACHE.get_tree(path)
                assert core.CACHE.get_tree(path) is first
                core.CACHE.invalidate(path)
                assert core.CACHE.get_tree(path) is not first
        - id: test_cache_is_shared_across_engines
          type: FunctionDef
          code: |-
            def test_cache_is_shared_across_engines(py_file, json_file):
                _, py_tree = core.load(py_file)
                _, json_tree = core.load(json_file)
                assert py_tree.engine.name == 'python'
                assert json_tree.engine.name == 'tree-sitter:json'
                '# Same cache instance serves both.'
                assert core.CACHE.get_tree(Path(py_file)) is py_tree
                assert core.CACHE.get_tree(Path(json_file)) is json_tree
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
        nodes:
        - id: EDFjeC|StWP8k
          type: statements
          code: |-
            _SRC = Path(__file__).resolve().parents[1] / 'src'
            if _SRC.is_dir() and str(_SRC) not in sys.path:
                sys.path.insert(0, str(_SRC))
            '# noqa: E402'
        - id: java
          type: FunctionDef
          code: |-
            @pytest.fixture(scope='module')
            def java():
                return get_engine('java')
        - id: test_java_add_present_and_remove
          type: FunctionDef
          code: |-
            def test_java_add_present_and_remove(java):
                tree = java.parse(JAVA)
                outcome = java.apply_imports(tree, ['import java.util.Set;', 'import java.util.List'], ['import java.util.Map;'])
                assert outcome.added == ['import java.util.Set;']
                assert outcome.removed == ['import java.util.Map;']
                assert not outcome.unchecked
                assert 'import java.util.Set;' in tree.source and 'java.util.Map' not in tree.source
                assert tree.source.count('import java.util.List;') == 1
                assert java.validate(tree.source) is None
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
        nodes:
        - id: retrieve
          type: FunctionDef
          code: |-
            def _retrieve(registry: ToolRegistry, arguments: dict):
                tool = registry.get('ast_retrieve')
                assert tool is not None
                session = Session(id='retrieve')
                session.enabled_tools.add('ast_retrieve')
                return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))
        - id: test_outline_and_find_in_one_call
          type: FunctionDef
          code: |-
            def test_outline_and_find_in_one_call(registry: ToolRegistry, tmp_path):
                path = _write_sample(tmp_path)
                result = _retrieve(registry, {'outline': [path], 'find': [{'paths': [path], 'text': 'needle'}]})
                assert not result.is_error
                content = result.structured_content
                assert set(content) == {'outline', 'find'}
                assert content['outline']['results']
                assert content['find']['results']
        - id: h1d9c184c
          type: FunctionDef
          code: |-
            def test_search_returns_node_ids_and_read_uses_them(registry: ToolRegistry, tmp_path):
                path = _write_sample(tmp_path)
                search_item = {'directory': [str(tmp_path)], 'pattern': 'needle', 'include': ['*.py'], 'intent': 'edit'}
                first = _retrieve(registry, {'search': [search_item]})
                assert not first.is_error
                entry = first.structured_content['search']['results'][0]['matches'][0]
                node_id = entry['nodes'][0]['id']
                assert node_id
                second = _retrieve(registry, {'read': [{'path': path, 'ids': [node_id]}]})
                assert not second.is_error
                assert 'needle' in str(second.structured_content['read'])
```
/answer 2d71-fb-fa allow
Text:
 Als Nächstes erkunde ich das reale Verhalten der Engine mit einem Laufzeit-Probelauf. Danach schreibe ich die Tests gegen belegte Erwartungen und nicht gegen Vermutungen.
ReasoningToken: 218
Control Request:
```yaml
id: 2d71-fc-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
    e=MarkdownEngine()
    def dump(s,title):
        print('=====',title);
        try:
            t=e.parse(s)
        except Exception as ex:
            print('PARSE ERR',type(ex).__name__,ex); return
        for l in e.locate_all(t):
            print(f"{l.node_id!r:55} {l.node_type:12} name={l.name!r} L{l.lineno}-{l.end_lineno} par={l.parent_type} exp={l.expandable} sig={e.signature(l.node)!r} doc={e.docstring(l.node)!r}")
    dump('---\ntitle: X\n---\n\n# H1\n\ntext\n','frontmatter')
    dump('# A\n\nx\n\n---\n\n# B\n\ny\n\n---\n\nz\n','pages')
    dump('Setext\n======\n\npara\n\nSub\n---\n\nmore\n','setext')
    dump('# A\n\n- a\n- b\n\n1. one\n2. two\n\n```py\nprint(1)\n```\n\n> quote\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<div>x</div>\n','blocks')
    dump('para only\n','nohead')
    dump('','empty')
    dump('# A\n\n#### Deep\n\ntext\n\n## B\n\n# A\n\ndup\n','levels/dups')
    dump('# Ünï ✓ 日本\n\nText äöü 😀\n','unicode')
    dump('# A\r\n\r\ntext\r\n','crlf')
    print(repr(e.validate('# A\n')), repr(e.validate('')), repr(e.validate('\x00')))
  reason: "Laufzeit-Erkundung: Struktur (Frontmatter, Pages, Setext, Listen, Code, Tabellen) und Mutationen der Markdown-RPC-Engine"
```
/answer 2d71-fc-0 allow
Control Request:
```yaml
id: 2d71-fd-fc
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ===== frontmatter
      'h2xfp2|zxmNvS'                                         Frontmatter  name=None L1-3 par=Root exp=False sig='---' doc=None
      'H1'                                                    Section      name='H1' L5-7 par=Root exp=False sig='# H1' doc='text'
      'H1.GgaB61|KEbEsP'                                      BlockGroup   name=None L7-7 par=Section exp=False sig='text' doc=None
      ===== pages
      'A'                                                     Section      name='A' L1-5 par=Root exp=False sig='# A' doc='x'
      'A.VFyAj9|17yFfa'                                       BlockGroup   name=None L3-5 par=Section exp=False sig='x' doc=None
      'B'                                                     Section      name='B' L7-13 par=Root exp=False sig='# B' doc='y'
      'B.hOzyUt|FRqP45'                                       BlockGroup   name=None L9-13 par=Section exp=False sig='y' doc=None
      ===== setext
      'Setext'                                                Section      name='Setext' L1-9 par=Root exp=True sig='# Setext' doc='para'
      'Setext.U8rkkX|zmwE0W'                                  BlockGroup   name=None L4-4 par=Section exp=False sig='para' doc=None
      'Setext.Sub'                                            Section      name='Sub' L6-9 par=Section exp=False sig='## Sub' doc='more'
      'Setext.Sub.fdyyz3|ALd8yY'                              BlockGroup   name=None L9-9 par=Section exp=False sig='more' doc=None
      ===== blocks
      'A'                                                     Section      name='A' L1-19 par=Root exp=False sig='# A' doc='| a | b | |---|---| | 1 | 2 |'
      'A.zQpY6G|ssSJ8Y'                                       BlockGroup   name=None L3-19 par=Section exp=False sig='- a' doc=None
      ===== nohead
      '1WWQ3j|uk9i5t'                                         BlockGroup   name=None L1-1 par=Root exp=False sig='para only' doc=None
      ===== empty
      ===== levels/dups
      'A'                                                     Section      name='A' L1-7 par=Root exp=True sig='# A' doc=None
      'A.Deep'                                                Section      name='Deep' L3-5 par=Section exp=False sig='#### Deep' doc='text'
      'A.Deep.GgaB61|KEbEsP'                                  BlockGroup   name=None L5-5 par=Section exp=False sig='text' doc=None
      'A.B'                                                   Section      name='B' L7-7 par=Section exp=False sig='## B' doc=None
      'A_2'                                                   Section      name='A' L9-11 par=Root exp=False sig='# A' doc='dup'
      'A_2.xtjzmC|bnvhEg'                                     BlockGroup   name=None L11-11 par=Section exp=False sig='dup' doc=None
      ===== unicode
      'Ünï_日本'                                                Section      name='Ünï ✓ 日本' L1-3 par=Root exp=False sig='# Ünï ✓ 日本' doc='Text äöü 😀'
      'Ünï_日本.Dd7U7z|K6MKug'                                  BlockGroup   name=None L3-3 par=Section exp=False sig='Text äöü 😀' doc=None
      ===== crlf
      'A'                                                     Section      name='A' L1-3 par=Root exp=False sig='# A' doc='text'
      'A.GgaB61|KEbEsP'                                       BlockGroup   name=None L3-3 par=Section exp=False sig='text' doc=None
      None None None
```
/answer 2d71-fd-fc allow
Text:
 Die Struktur sieht plausibel aus. Der wichtigste Risikopunkt bei einer remark-Engine ist, ob Mutationen den restlichen Text umformatieren. Das probiere ich jetzt aus, zusammen mit Fehlerfällen.
ReasoningToken: 581
Control Request:
```yaml
id: 2d71-fe-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src')
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
    from xy.ai.mcpc.tools.ast.base import AstError, AstAmbiguous
    e=MarkdownEngine()
    MESSY='# Title\n\n* item one\n* item two\n\n\nSome   *emph*   and __strong__ text.\nSecond line.\n\n## Sec A\n\n1) one\n2) two\n\n    indented code\n\n## Sec B\n\nlast\n'
    def get(t,i): return next(l for l in e.locate_all(t) if l.node_id==i)
    def show(label,s): print('-----',label); print(repr(s))
    t=e.parse(MESSY); print('roundtrip same:',e.serialize(t)==MESSY)
    for l in e.locate_all(t): print(l.node_id,l.node_type,l.lineno,l.end_lineno)
    # replace paragraph in sec B
    t=e.parse(MESSY); l=get(t,'Title.Sec_B');
    bg=[x for x in e.locate_all(t) if x.node_id.startswith('Title.Sec_B.')][0]
    e.replace(bg,'changed'); show('replace para SecB',t.source)
    # replace section by section
    t=e.parse(MESSY); e.replace(get(t,'Title.Sec_A'),'## Sec A2\n\nnew body'); show('replace SecA',t.source)
    # insert before/after
    t=e.parse(MESSY); n=e.insert(get(t,'Title.Sec_B'),'## Sec Mid\n\nmid','before'); show(f'insert before units={n}',t.source)
    t=e.parse(MESSY); n=e.insert(get(t,'Title.Sec_B'),'## Sec After\n\nx','after'); show(f'insert after units={n}',t.source)
    t=e.parse(MESSY); n=e.insert(get(t,'Title.Sec_B'),'p1\n\np2','after'); show(f'insert 2 paras units={n}',t.source)
    # delete
    t=e.parse(MESSY); e.delete(get(t,'Title.Sec_A')); show('delete SecA',t.source)
    t=e.parse(MESSY); e.delete(get(t,'Title')); show('delete Title',repr(t.source))
    # append
    t=e.parse(MESSY); n=e.append(t,'## Sec C\n\nappended'); show(f'append units={n}',t.source)
    t=e.empty_tree(); n=e.append(t,'# New\n\nbody'); show(f'append to empty units={n}',t.source)
    # errors
    t=e.parse(MESSY)
    for label,fn in [('replace unknown id',lambda: e.replace(type(get(t,'Title'))(**{**get(t,'Title').__dict__,'node_id':'Nope'}),'x')),]:
        try: fn(); print(label,'no error')
        except Exception as ex: print(label,type(ex).__name__,ex)
    # CRLF
    C='# A\r\n\r\ntext\r\n\r\n## B\r\n\r\nmore\r\n'
    t=e.parse(C); print('crlf roundtrip',e.serialize(t)==C)
    e.replace([x for x in e.locate_all(t) if x.node_id.startswith('A.B.')][0],'new'); show('crlf replace',t.source)
    # frontmatter
    F='---\ntitle: X\n---\n\n# H\n\ntext\n'
    t=e.parse(F); e.replace(get(t,'H'),'# H\n\nchanged'); show('frontmatter keep',t.source)
  reason: "Laufzeit-Erkundung: Mutationen (replace/insert/delete/append), Formatierungstreue, Fehlerfälle, CRLF"
```
/answer 2d71-fe-0 allow Neuformartierung ist aber erwartet. Normalisierung ist eine erwartete Funktion
Control Request:
```yaml
id: 2d71-ff-fe
result:
  structuredContent:
    exit_code: 0
    stdout: |
      roundtrip same: True
      Title Section 1 19
      Title.W4x9eh|Jkn2KL BlockGroup 3 8
      Title.Sec_A Section 10 15
      Title.Sec_A.nbFnym|BM7o7E BlockGroup 12 15
      Title.Sec_B Section 17 19
      Title.Sec_B.F2qHzd|Kcumr1 BlockGroup 19 19
      ----- replace para SecB
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A\n\n1. one\n2. two\n\n   indented code\n\n## Sec B\n\nchanged\n'
      ----- replace SecA
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A2\n\nnew body\n\n## Sec B\n\nlast\n'
      ----- insert before units=2
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A\n\n1. one\n2. two\n\n   indented code\n\n## Sec Mid\n\nmid\n\n## Sec B\n\nlast\n'
      ----- insert after units=2
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A\n\n1. one\n2. two\n\n   indented code\n\n## Sec B\n\nlast\n\n## Sec After\n\nx\n'
      ----- insert 2 paras units=2
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A\n\n1. one\n2. two\n\n   indented code\n\n## Sec B\n\nlast\n\np1\n\np2\n'
      ----- delete SecA
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec B\n\nlast\n'
      ----- delete Title
      "''"
      ----- append units=2
      '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\nSecond line.\n\n## Sec A\n\n1. one\n2. two\n\n   indented code\n\n## Sec B\n\nlast\n\n## Sec C\n\nappended\n'
      ----- append to empty units=2
      '# New\n\nbody\n'
      replace unknown id AstError node not found: Nope
      crlf roundtrip True
      ----- crlf replace
      '# A\n\ntext\n\n## B\n\nnew\n'
      ----- frontmatter keep
      '---\ntitle: X\n---\n\n# H\n\nchanged\n'
```
/answer 2d71-ff-fe allow
Thinking:
Die Tests prüfen den normalisierten Text, nicht die Originalformatierung. Als Nächstes teste ich Fehlerpfade, Nebenläufigkeit, Dateiebene und Imports.


ReasoningToken: 418
Control Request:
```yaml
id: 2d71-100-0
toolName: python
arguments:
  script: |-
    import sys,tempfile,threading,time,os,signal
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src')
    from pathlib import Path
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine, _PROCESS
    from xy.ai.mcpc.tools.ast.base import AstError, AstAmbiguous
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.generic import get_engine, language_for_extension
    e=MarkdownEngine()
    S='# T\n\nintro\n\n## A\n\nx\n\n## A\n\ny\n'
    t=e.parse(S)
    for l in e.locate_all(t): print(l.node_id,l.node_type,l.lineno,l.end_lineno)
    loc=e.locate_all(t)[0]
    import dataclasses
    def ghost(i): return dataclasses.replace(loc,node_id=i)
    for label,fn in [('insert ghost',lambda:e.insert(ghost('Nope'),'x','after')),('delete ghost',lambda:e.delete(ghost('Nope'))),('replace ghost',lambda:e.replace(ghost('Nope'),'x')),
      ('replace empty code',lambda:e.replace(loc,'')),('replace ctrl',lambda:e.replace(loc,'a\x01b')),('append ctrl',lambda:e.append(t,'a\x01'))]:
        try: r=fn(); print(label,'ok',repr(t.source[:60]))
        except Exception as ex: print(label,type(ex).__name__,str(ex)[:100])
    print('engine lookups', get_engine('markdown') is get_engine('markdown'), type(get_engine('markdown')).__module__, language_for_extension('.md'), language_for_extension('.markdown'))
    # tool level
    from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create
    from xy.ai.mcpc.tools.ast.find import ast_find
    from xy.ai.mcpc.tools.ast.outline import ast_outline
    from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read
    from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
    from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
    from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete
    from xy.ai.mcpc.tools.ast.validate import ast_validate
    from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks
    d=Path(tempfile.mkdtemp()); p=d/'doc.md'
    r=ast_create(items=[CreateFileItem(path=str(p),source='# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 2\n\nbeta\n')]); print('create',r)
    print('outline',ast_outline([str(p)]))
    print('find',ast_find(paths=[str(p)],node_type='Section'))
    print('read',ast_read(items=[ReadItem(path=str(p),ids=['Doc.Part_1'])]) if 'ReadItem' in dir() else '')
    print(ast_replace(items=[ReplaceItem(path=str(p),source='## Part 1\n\nALPHA',id='Doc.Part_1')]))
    print(ast_insert(items=[InsertItem(path=str(p),source='## Part 1b\n\nmid',id='Doc.Part_1',position='after')]))
    print(ast_delete(items=[DeleteItem(path=str(p),id='Doc.Part_2')]))
    print(ast_edit_marks(items=[EditMarksItem(path=str(p),start_marker='ALPHA',end_marker='ALPHA',content='alpha2',id='Doc.Part_1')]))
    print(repr(p.read_text()), ast_validate([str(p)]))
    # imports
    try: print(e.apply_imports(e.parse('# A\n'),['import x'],[]))
    except Exception as ex: print('imports',type(ex).__name__,ex)
  reason: "Laufzeit-Erkundung: Fehlercodes (404/409/422), Nebenläufigkeit, Tool-Ebene mit Dateien, Imports, Prozess-Neustart"
```
/answer 2d71-100-0 allow
Control Request:
```yaml
id: 2d71-101-100
result:
  structuredContent:
    exit_code: 0
    stdout: |
      T Section 1 11
      T.8vS9lo|p3TE1H BlockGroup 3 3
      T.A Section 5 7
      T.A.AcpLT8|qoyKL2 BlockGroup 7 7
      T.A_2 Section 9 11
      T.A_2.u2RurT|ySVM1X BlockGroup 11 11
      insert ghost AstError node not found: Nope
      delete ghost AstError node not found: Nope
      replace ghost AstError node not found: Nope
      replace empty code AstError no markdown content found: code is empty
      replace ctrl AstError input contains an abnormal control character: '\x01' at offset 1
      append ctrl AstError input contains an abnormal control character: '\x01' at offset 1
      engine lookups True xy.ai.mcpc.tools.ast.rpc._markdown markdown markdown
      create CreateFilesResult(results=[CreateFileResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md')], errors=[], warnings=())
      outline OutlineNodesBatchResult(results=[OutlineNodesResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', nodes=[OutlineNode(id='Doc', type=None, lines='1-11', excert='# Doc', docstring='intro', code=None, children=[OutlineNode(id='Doc.8vS9lo|p3TE1H', type=None, lines='3', excert='intro', docstring=None, code=None, children=[]), OutlineNode(id='Doc.Part_1', type=None, lines='5-7', excert='## Part 1', docstring='alpha', code=None, children=[]), OutlineNode(id='Doc.Part_2', type=None, lines='9-11', excert='## Part 2', docstring='beta', code=None, children=[])])])], errors=[], warnings=())
      find FindNodesResult(files=[FileNodesResult(path='/tmp/tmpdfd96hyw/doc.md', nodes=[OutlineNode(id='Doc', type='Section', lines='1-11', excert='# Doc', docstring='intro', code=None, children=[OutlineNode(id='Doc.Part_1', type='Section', lines='5-7', excert=None, docstring=None, code='## Part 1\n\nalpha\n', children=[]), OutlineNode(id='Doc.Part_2', type='Section', lines='9-11', excert=None, docstring=None, code='## Part 2\n\nbeta\n', children=[])])])])
      read ReadBatchResult(results=[ReadResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', nodes=[OutlineNode(id='Doc.Part_1', type='Section', lines='5-7', excert=None, docstring=None, code='## Part 1\n\nalpha\n', children=[])])], errors=[], warnings=())
      ReplaceBatchResult(results=[ReplaceResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', id='Doc.Part_1', new_id='Doc.Part_1', imports=None)], errors=[], warnings=())
      InsertBatchResult(results=[InsertResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', id='Doc.Part_1', inserted=2, ids=['Doc.Part_1b', 'Doc.Part_1b.1EZ2TK|h3JWBY'], imports=None)], errors=[], warnings=())
      DeleteBatchResult(results=[DeleteResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', id='Doc.Part_2', imports=None)], errors=[], warnings=())
      EditMarksBatchResult(results=[EditMarksResult(warnings=(), path='/tmp/tmpdfd96hyw/doc.md', id='Doc.Part_1', new_id='Doc.Part_1', imports=None)], errors=[], warnings=())
      '# Doc\n\nintro\n\n## Part 1\n\nalpha2\n\n## Part 1b\n\nmid\n' ValidateResult(all_ok=True, files=[FileCheck(path='/tmp/tmpdfd96hyw/doc.md', ok=True, error=None)])
      imports AstError Not recognized as an import by the parser: ['import x;']
```
/answer 2d71-101-100 allow
Text:
 Die Tool-Kette funktioniert durchgängig. Es fehlen noch Robustheit (Nebenläufigkeit, große Dokumente, Prozessabsturz) und Mehrdeutigkeiten.
ReasoningToken: 241
Control Request:
```yaml
id: 2d71-102-0
toolName: python
arguments:
  script: |-
    import sys,tempfile,threading,time,os,signal
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src')
    from pathlib import Path
    from concurrent.futures import ThreadPoolExecutor
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine, _PROCESS
    from xy.ai.mcpc.tools.ast.base import AstError, AstAmbiguous
    e=MarkdownEngine()
    # concurrency
    def work(i):
        s=f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nbody {i}\n'
        t=e.parse(s); loc=[l for l in e.locate_all(t) if l.node_id==f'Doc{i}.Sub'][0]
        e.replace(loc,f'## Sub\n\nnew {i}')
        return t.source==f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nnew {i}\n'
    t0=time.time()
    with ThreadPoolExecutor(16) as ex: res=list(ex.map(work,range(200)))
    print('concurrent ok',all(res),len(res),round(time.time()-t0,2),'s')
    # large doc
    big=''.join(f'## Section {i}\n\n'+''.join(f'Paragraph {i}.{j} lorem ipsum.\n\n' for j in range(5)) for i in range(2000))
    big='# Big\n\n'+big
    t0=time.time(); t=e.parse(big); loc=e.locate_all(t); print('large nodes',len(loc),'bytes',len(big),round(time.time()-t0,2),'s')
    t0=time.time(); l=[x for x in loc if x.node_id=='Big.Section_1999'][0]; e.replace(l,'## Section 1999\n\nX'); print('large replace',round(time.time()-t0,2),'s','X' in t.source)
    # ambiguous by name via tool layer
    from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
    d=Path(tempfile.mkdtemp()); p=d/'a.md'; p.write_text('# T\n\n## A\n\nx\n\n## A\n\ny\n')
    print(ast_replace(items=[ReplaceItem(path=str(p),source='## A\n\nz',id='A')]))
    print(ast_replace(items=[ReplaceItem(path=str(p),source='## A\n\nz',id='T.Nope')]))
    print(repr(p.read_text()))
    # kill and recover?
    pid=_PROCESS._process.pid; print('pid',pid); os.kill(pid,signal.SIGKILL); time.sleep(0.5)
    try: print('after kill', e.validate('# a\n'))
    except Exception as ex: print('after kill',type(ex).__name__,str(ex)[:150])
    try:
        e2=MarkdownEngine(); print('new engine after kill', e2.validate('# a\n'), 'pid', _PROCESS._process.pid)
    except Exception as ex: print('new engine',type(ex).__name__,str(ex)[:150])
  reason: "Laufzeit-Erkundung: Nebenläufigkeit, große Dokumente, Abbruch/Neustart des Node-Prozesses, mehrdeutige Namen"
```
/answer 2d71-102-0 allow
Control Request:
```yaml
id: 2d71-103-102
result:
  structuredContent:
    exit_code: 0
    stdout: |
      concurrent ok True 200 1.02 s
      large nodes 4001 bytes 337347 1.31 s
      large replace 0.72 s True
      ReplaceBatchResult(results=[], errors=[ReplaceError(error='No node matched the selector. verify the node with `search` or `ast_find`', path='/tmp/tmpo_bei0e1/a.md', id='A')], warnings=())
      ReplaceBatchResult(results=[], errors=[ReplaceError(error='No node matched the selector. verify the node with `search` or `ast_find`', path='/tmp/tmpo_bei0e1/a.md', id='T.Nope')], warnings=())
      '# T\n\n## A\n\nx\n\n## A\n\ny\n'
      pid 99115
      after kill URLError <urlopen error [Errno 111] Connection refused>
      new engine after kill None pid 99207
```
/answer 2d71-103-102 allow
Thinking:
Ein Befund: Wenn der Node-Prozess stirbt, bleibt die Engine-Instanz kaputt, weil `RpcEngine` den Client cacht und den Prozess nicht neu startet. Ich prüfe noch ein paar Randfälle, bevor ich die Tests schreibe.


Control Request:
```yaml
id: 2d71-104-0
toolName: python
arguments:
  script: |-
    import sys,tempfile
    sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/src'); sys.path.insert(0,'/home/user/xyan/xy.ai.workbench/mcpc/tests')
    from pathlib import Path
    from xy.ai.mcpc.tools.ast.rpc._markdown import MarkdownEngine
    e=MarkdownEngine()
    def dump(s):
        t=e.parse(s); print(repr(s))
        for l in e.locate_all(t): print('  ',l.node_id,l.node_type,l.lineno,l.end_lineno,repr(e.signature(l.node)))
        return t
    def get(t,i): return next(l for l in e.locate_all(t) if l.node_id==i)
    dump('# Real\n\n```\n# not heading\n```\n\n> # quoted\n\n- # in list\n\n## Next\n')
    t=dump('# A\n\nx\n\n---\n\n# B\n\ny\n')
    e.replace(get(t,'A'),'# A\n\nchanged'); print(repr(t.source))
    t=dump('# A\n\n## B\n\nb\n\n## C\n\nc\n')
    e.replace(get(t,'A.B'),'### B3\n\nlevel3'); print(repr(t.source))
    t=dump('# A\n\np1\n\np2\n\np3\n')
    bgs=[l for l in e.locate_all(t) if l.node_type=='BlockGroup']; print(len(bgs), repr(e.node_code(bgs[0].node)))
    e.insert(get(t,'A'),'prefix','before'); print(repr(t.source))
    t=e.parse('# A\n\np1\n\n- l1\n- l2\n\n```js\nx\n```\n')
    dump('# A\n\np1\n\n- l1\n- l2\n\n```js\nx\n```\n')
    # tools
    from xy.ai.mcpc.tools.ast import edit_block as eb
    print([n for n in dir(eb) if 'dit' in n])
    EOF=None
  reason: "Laufzeit-Erkundung: Code-Fences/Blockquotes mit '#', Seitenumbrüche bei Mutation, Level-Wechsel, edit_block-Tool, Registry-Pfad"
```
/answer 2d71-104-0 allow
ReasoningToken: 1681
Control Request:
```yaml
id: 2d71-105-104
result:
  structuredContent:
    exit_code: 0
    stdout: |
      '# Real\n\n```\n# not heading\n```\n\n> # quoted\n\n- # in list\n\n## Next\n'
         Real Section 1 11 '# Real'
         Real.VOGizc|5dgWJa BlockGroup 3 9 '```'
         Real.Next Section 11 11 '## Next'
      '# A\n\nx\n\n---\n\n# B\n\ny\n'
         A Section 1 5 '# A'
         A.VFyAj9|17yFfa BlockGroup 3 5 'x'
         B Section 7 9 '# B'
         B.u2RurT|ySVM1X BlockGroup 9 9 'y'
      '# A\n\nchanged\n\n# B\n\ny\n'
      '# A\n\n## B\n\nb\n\n## C\n\nc\n'
         A Section 1 9 '# A'
         A.B Section 3 5 '## B'
         A.B.iMaReK|2NeinT BlockGroup 5 5 'b'
         A.C Section 7 9 '## C'
         A.C.ZIJR6Z|0sPy1R BlockGroup 9 9 'c'
      '# A\n\n### B3\n\nlevel3\n\n## C\n\nc\n'
      '# A\n\np1\n\np2\n\np3\n'
         A Section 1 7 '# A'
         A.flOn6q|gGQhWO BlockGroup 3 7 'p1'
      1 'p1\n\np2\n\np3\n'
      'prefix\n\n# A\n\np1\n\np2\n\np3\n'
      '# A\n\np1\n\n- l1\n- l2\n\n```js\nx\n```\n'
         A Section 1 10 '# A'
         A.9aiPoD|t1WMag BlockGroup 3 10 'p1'
      ['EditBlockBatchResult', 'EditBlockError', 'EditBlockItem', 'EditBlockNodeTool', 'EditBlockResult', '_edit_block_one', 'ast_edit_block', 'edit_payload']
```
/answer 2d71-105-104 allow
Control Request:
```yaml
id: 2d71-106-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^class \w+Item|^def ast_\w+\(|overwrite
    include:
    - edit_block.py
    - create.py
    - retrieve.py
    - edit.py
    limit: 20
  reason: Signaturen der edit_block-/create-Items und der registrierten Tool-Namen für Registry-Tests ermitteln
```
/answer 2d71-106-0 allow
Control Request:
```yaml
id: 2d71-107-106
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: edit_block.py
          matches:
          - "class EditBlockItem:"
          - "def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        - path: create.py
          matches:
          - "class CreateFileItem:"
          - "overwrite: Allow replacing an existing file. Defaults to ``False``."
          - "overwrite: bool = False"
          - "if file_path.exists() and (not item.overwrite):"
          - "def ast_create(items: list[CreateFileItem]) -> CreateFilesResult:"
          - "'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {"
          - "return CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False))"
```
/answer 2d71-107-106 allow
ReasoningToken: 262
Control Request:
```yaml
id: 2d71-108-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    name: EditBlockItem
  reason: "Felder von EditBlockItem lesen, um das Tool im Test korrekt aufzurufen"
```
/answer 2d71-108-0 allow
Control Request:
```yaml
id: 2d71-109-108
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
        nodes:
        - id: EditBlockItem
          code: |-
            @dataclass(frozen=True)
            class EditBlockItem:
                """One block edit to apply.

                Attributes:
                    path: Absolute path to the file to modify.
                    old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
                    new_text: Replacement text (may be empty to delete the block).
                    exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
                    replace_all: If True, replace every occurrence instead of requiring a single match.
                    id: Unique id of the target node. If omitted, the node is searched for by ``old_text``.
                    add_import: Import statements to add if missing, applied after the edit.
                    remove_import: Import statements to remove if present, applied after the edit.
                """
                path: str
                old_text: str
                new_text: str
                exact: bool = False
                replace_all: bool = False
                id: str | None = None
                add_import: list[str] | None = None
                remove_import: list[str] | None = None
```
/answer 2d71-109-108 allow
ReasoningToken: 630
Control Request:
```yaml
id: 2d71-10a-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
    ids:
    - DaYVs5|eLrdax
    - write_sample
  reason: "Import-Block von test_ast_retrieve (Session, ToolContext, ToolRegistry) für die Registry-Tests übernehmen"
```
/answer 2d71-10a-0 allow
Control Request:
```yaml
id: 2d71-10b-10a
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_retrieve.py
        nodes:
        - id: DaYVs5|eLrdax
          type: imports
          code: |-
            from __future__ import annotations
            from xy.ai.mcpc.server.session import Session
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        - id: write_sample
          type: FunctionDef
          code: |-
            def _write_sample(tmp_path) -> str:
                path = tmp_path / 'sample.py'
                path.write_text('def f():\n    return "needle"\n')
                return str(path)
```
/answer 2d71-10b-10a allow
Thinking:
Ich ersetze jetzt den kleinen Bridge-Test durch eine umfassende Testdatei, die Konfiguration, Lebenszyklus, Mutationen, Fehlerfälle, Nebenläufigkeit und die Tool-Ebene abdeckt.


ReasoningToken: 3130
Text:
 Die JSON-Eingabe war ungültig: `\U0001f600` ist kein gültiges JSON-Escape. Ich sende die Datei mit direkten Unicode-Zeichen neu und vereinfache dabei einen umständlichen Test.
ReasoningToken: 231
Control Request:
```yaml
id: 2d71-10c-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_markdown_bridge.py
    overwrite: true
    source: |
      """Eigenständige Tests für die Markdown-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._markdown``).

      Startet den echten ``ast-engines/markdown``-Prozess (Node/remark) lazy und prüft
      Konfiguration, Prozess-Lifecycle, Struktur, Mutationen, Fehlerfälle,
      Nebenläufigkeit und die komplette ``ast_*``-Tool-Kette end-to-end.

      Die Engine normalisiert beim Mutieren den gesamten Text (remark-stringify);
      die Tests prüfen daher bewusst den normalisierten Output.
      """

      from __future__ import annotations

      import dataclasses
      import os
      import shutil
      import signal
      import socket
      from concurrent.futures import ThreadPoolExecutor
      from pathlib import Path

      import pytest

      from xy.ai.mcpc.server.session import Session
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.base import AstError, Located, OutlineNode, Tree
      from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create
      from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete
      from xy.ai.mcpc.tools.ast.edit_block import EditBlockItem, ast_edit_block
      from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks
      from xy.ai.mcpc.tools.ast.find import ast_find
      from xy.ai.mcpc.tools.ast.generic import get_engine, language_for_extension
      from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
      from xy.ai.mcpc.tools.ast.outline import ast_outline
      from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read
      from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
      from xy.ai.mcpc.tools.ast.rpc import MarkdownEngine as ExportedMarkdownEngine
      from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
      from xy.ai.mcpc.tools.ast.rpc._markdown import _ENGINE_DIR, _MAIN, _PROCESS, MarkdownEngine, MarkdownProcess
      from xy.ai.mcpc.tools.ast.validate import ast_validate
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolRegistry

      requires_engine = pytest.mark.skipif(
          shutil.which('node') is None or not (_ENGINE_DIR / 'node_modules').is_dir(),
          reason='node or the engine dependencies (npm install in ast-engines/markdown) are missing',
      )

      DOC = (
          '# Title\n\n'
          'Intro text.\n\n'
          '## Section A\n\n'
          'First paragraph.\n\n'
          '## Section B\n\n'
          'Second paragraph.\n'
      )

      FILE_DOC = '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 2\n\nbeta\n'


      @pytest.fixture(scope='module')
      def engine() -> MarkdownEngine:
          return MarkdownEngine()


      def _find(engine: MarkdownEngine, tree: Tree, node_id: str) -> Located:
          return next(loc for loc in engine.locate_all(tree) if loc.node_id == node_id)


      def _ids(engine: MarkdownEngine, source: str) -> list[str]:
          return [loc.node_id for loc in engine.locate_all(engine.parse(source))]


      def _free_port() -> int:
          with socket.socket() as sock:
              sock.bind(('127.0.0.1', 0))
              return sock.getsockname()[1]


      def _flatten(nodes: list[OutlineNode]) -> list[OutlineNode]:
          flat: list[OutlineNode] = []
          for node in nodes:
              flat.append(node)
              flat.extend(_flatten(list(node.children)))
          return flat


      # ---------------------------------------------------------------------------
      # Konfiguration & Registrierung (ohne Prozess)
      # ---------------------------------------------------------------------------


      def test_engine_dir_and_entry_point_exist() -> None:
          assert _ENGINE_DIR.is_dir()
          assert (_ENGINE_DIR / _MAIN).is_file()


      def test_process_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
          monkeypatch.delenv('AST_MARKDOWN_PORT', raising=False)
          monkeypatch.delenv('NODE', raising=False)
          process = MarkdownProcess()
          assert process.default_port() == 8789
          assert process.cwd() == str(_ENGINE_DIR)
          assert process.command(1234) == ['node', str(_MAIN), '1234']


      def test_process_honours_environment(monkeypatch: pytest.MonkeyPatch) -> None:
          monkeypatch.setenv('AST_MARKDOWN_PORT', '9911')
          monkeypatch.setenv('NODE', '/opt/node/bin/node')
          process = MarkdownProcess()
          assert process.default_port() == 9911
          assert process.command(9911)[0] == '/opt/node/bin/node'


      def test_default_port_does_not_collide_with_other_engines(monkeypatch: pytest.MonkeyPatch) -> None:
          from xy.ai.mcpc.tools.ast.rpc._java import JavaProcess
          from xy.ai.mcpc.tools.ast.rpc._rust import RustProcess

          for var in ('AST_JAVA_PORT', 'AST_RUST_PORT', 'AST_MARKDOWN_PORT'):
              monkeypatch.delenv(var, raising=False)
          ports = {JavaProcess().default_port(), RustProcess().default_port(), MarkdownProcess().default_port()}
          assert len(ports) == 3


      def test_engine_is_registered_as_rpc_engine() -> None:
          resolved = get_engine('markdown')
          assert isinstance(resolved, RpcEngine)
          assert isinstance(resolved, MarkdownEngine)
          assert type(resolved).__module__ == 'xy.ai.mcpc.tools.ast.rpc._markdown'
          assert resolved is get_engine('markdown')
          assert ExportedMarkdownEngine is MarkdownEngine


      @pytest.mark.parametrize('name', ['x.md', 'x.markdown'])
      def test_extensions_resolve_to_markdown_engine(tmp_path: Path, name: str) -> None:
          assert language_for_extension(Path(name).suffix) == 'markdown'
          assert core.engine_for_path(tmp_path / name).name == 'markdown'
          assert isinstance(core.engine_for_path(tmp_path / name), MarkdownEngine)


      def test_engine_metadata() -> None:
          resolved = MarkdownEngine()
          assert resolved.symbol == 'markdown'
          assert resolved.name == 'markdown'
          assert resolved.validates_syntax is False


      def test_constructing_engine_does_not_start_process() -> None:
          process = MarkdownProcess()
          RpcEngine('markdown', process)
          assert process._process is None


      # ---------------------------------------------------------------------------
      # Prozess-Lifecycle
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_process_starts_lazily_and_terminates(monkeypatch: pytest.MonkeyPatch) -> None:
          port = _free_port()
          monkeypatch.setenv('AST_MARKDOWN_PORT', str(port))
          process = MarkdownProcess()
          isolated = RpcEngine('markdown', process)
          assert process._process is None

          assert isolated.validate('# a\n') is None
          assert process._process is not None and process._process.poll() is None
          assert process.base_url() == f'http://127.0.0.1:{port}'
          first_pid = process._process.pid
          assert isolated.validate('# b\n') is None
          assert process._process.pid == first_pid

          process._terminate()
          with pytest.raises(OSError):
              socket.create_connection(('127.0.0.1', port), timeout=1).close()


      def test_missing_node_binary_fails_fast(monkeypatch: pytest.MonkeyPatch) -> None:
          monkeypatch.setenv('NODE', '/nonexistent/node')
          monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
          with pytest.raises((OSError, RuntimeError)):
              MarkdownProcess().base_url()


      def test_invalid_port_configuration_is_rejected(monkeypatch: pytest.MonkeyPatch) -> None:
          monkeypatch.setenv('AST_MARKDOWN_PORT', 'not-a-port')
          with pytest.raises(ValueError):
              MarkdownProcess().base_url()


      # ---------------------------------------------------------------------------
      # Parse / Serialize / Validate
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_parse_and_serialize_roundtrip(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          assert engine.serialize(tree) == DOC


      @requires_engine
      @pytest.mark.parametrize(
          'source',
          [
              '',
              '\n',
              'plain\n',
              '---\ntitle: X\n---\n\n# H\n',
              '# A\r\n\r\ntext\r\n',
              '* a\n* b\n',
              'no trailing newline',
              '# Ünï ✓ 日本\n\nText äöü 😀\n',
          ],
      )
      def test_parse_does_not_modify_source(engine: MarkdownEngine, source: str) -> None:
          tree = engine.parse(source)
          assert engine.serialize(tree) == source


      @requires_engine
      @pytest.mark.parametrize('source', ['', '# ok\n', 'text', '\x00', '```\nunclosed fence', '<div>', '[broken](link'])
      def test_any_text_is_valid_markdown(engine: MarkdownEngine, source: str) -> None:
          assert engine.validate(source) is None
          engine.parse(source)


      @requires_engine
      def test_empty_tree_and_empty_document(engine: MarkdownEngine, tmp_path: Path) -> None:
          assert engine.serialize(engine.empty_tree()) == ''
          assert engine.empty_tree(tmp_path / 'x.md').path == tmp_path / 'x.md'
          assert engine.locate_all(engine.empty_tree()) == []
          assert engine.locate_all(engine.parse('')) == []


      # ---------------------------------------------------------------------------
      # Struktur
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_locate_all_builds_heading_tree(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse(DOC))
          assert [loc.node_type for loc in located] == ['Section', 'BlockGroup', 'Section', 'BlockGroup', 'Section', 'BlockGroup']
          sections = [loc for loc in located if loc.node_type == 'Section']
          assert [loc.node_id for loc in sections] == ['Title', 'Title.Section_A', 'Title.Section_B']
          assert [loc.name for loc in sections] == ['Title', 'Section A', 'Section B']
          assert located[0].parent_type == 'Root'
          assert all(loc.parent_type == 'Section' for loc in located[1:])


      @requires_engine
      def test_line_ranges_are_one_based_and_inclusive(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          ranges = {loc.node_id: (loc.lineno, loc.end_lineno) for loc in engine.locate_all(tree) if loc.node_type == 'Section'}
          assert ranges == {'Title': (1, 11), 'Title.Section_A': (5, 7), 'Title.Section_B': (9, 11)}


      @requires_engine
      def test_child_ids_extend_parent_id_and_are_unique(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse(DOC))
          ids = [loc.node_id for loc in located]
          assert len(ids) == len(set(ids))
          for loc in located[1:]:
              assert loc.node_id.startswith('Title.')


      @requires_engine
      def test_anonymous_block_ids_are_content_stable(engine: MarkdownEngine) -> None:
          def block_ids(source: str) -> set[str]:
              return {loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'BlockGroup'}

          first = block_ids(DOC)
          assert first == block_ids(DOC)
          third = block_ids(DOC.replace('Second paragraph.', 'Other words.'))
          assert len(first & third) == 2 and len(third - first) == 1


      @requires_engine
      def test_signature_docstring_and_code(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          title = _find(engine, tree, 'Title')
          section_a = _find(engine, tree, 'Title.Section_A')
          assert engine.signature(title.node) == '# Title'
          assert engine.signature(section_a.node) == '## Section A'
          assert engine.docstring(section_a.node) == 'First paragraph.'
          assert engine.node_code(section_a.node) == '## Section A\n\nFirst paragraph.\n'
          assert engine.node_code(title.node) == DOC
          block = next(loc for loc in engine.locate_all(tree) if loc.node_type == 'BlockGroup')
          assert engine.signature(block.node) == 'Intro text.'
          assert engine.node_code(block.node) == 'Intro text.\n'


      @requires_engine
      def test_document_without_headings_has_root_blocks(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse('just text\n\nmore text\n'))
          assert [loc.node_type for loc in located] == ['BlockGroup']
          assert located[0].parent_type == 'Root'
          assert (located[0].lineno, located[0].end_lineno) == (1, 3)


      @requires_engine
      def test_frontmatter_is_its_own_node(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse('---\ntitle: X\n---\n\n# H1\n\ntext\n'))
          assert [loc.node_type for loc in located] == ['Frontmatter', 'Section', 'BlockGroup']
          assert (located[0].lineno, located[0].end_lineno) == (1, 3)
          assert located[0].parent_type == 'Root'
          assert located[1].node_id == 'H1'


      @requires_engine
      def test_setext_headings_are_sections(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse('Setext\n======\n\npara\n\nSub\n---\n\nmore\n'))
          sections = [loc for loc in located if loc.node_type == 'Section']
          assert [loc.node_id for loc in sections] == ['Setext', 'Setext.Sub']
          assert [engine.signature(loc.node) for loc in sections] == ['# Setext', '## Sub']


      @requires_engine
      def test_skipped_heading_levels_nest_under_previous(engine: MarkdownEngine) -> None:
          ids = _ids(engine, '# A\n\n#### Deep\n\ntext\n\n## B\n')
          assert [i for i in ids if '|' not in i] == ['A', 'A.Deep', 'A.B']


      @requires_engine
      def test_duplicate_headings_get_numbered_ids(engine: MarkdownEngine) -> None:
          ids = [i for i in _ids(engine, '# T\n\n## A\n\nx\n\n## A\n\ny\n') if '|' not in i]
          assert ids == ['T', 'T.A', 'T.A_2']
          ids = [i for i in _ids(engine, '# A\n\n# A\n') if '|' not in i]
          assert ids == ['A', 'A_2']


      @requires_engine
      def test_headings_inside_code_quote_and_list_are_not_sections(engine: MarkdownEngine) -> None:
          source = '# Real\n\n```\n# not heading\n```\n\n> # quoted\n\n- # in list\n\n## Next\n'
          sections = [loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']
          assert sections == ['Real', 'Real.Next']


      @requires_engine
      def test_mixed_blocks_form_one_block_group(engine: MarkdownEngine) -> None:
          source = (
              '# A\n\n- a\n- b\n\n1. one\n2. two\n\n```py\nprint(1)\n```\n\n> quote\n\n'
              '| a | b |\n|---|---|\n| 1 | 2 |\n\n<div>x</div>\n'
          )
          located = engine.locate_all(engine.parse(source))
          assert [loc.node_type for loc in located] == ['Section', 'BlockGroup']
          assert (located[1].lineno, located[1].end_lineno) == (3, 19)


      @requires_engine
      def test_unicode_headings_and_content(engine: MarkdownEngine) -> None:
          tree = engine.parse('# Ünï ✓ 日本\n\nText äöü 😀\n')
          section = engine.locate_all(tree)[0]
          assert section.name == 'Ünï ✓ 日本'
          assert engine.node_code(section.node) == '# Ünï ✓ 日本\n\nText äöü 😀\n'
          engine.replace(section, '# Ünï ✓ 日本\n\nNeu ß 🚀')
          assert engine.serialize(tree) == '# Ünï ✓ 日本\n\nNeu ß 🚀\n'


      @requires_engine
      def test_thematic_breaks_do_not_split_sections(engine: MarkdownEngine) -> None:
          located = engine.locate_all(engine.parse('# A\n\nx\n\n---\n\n# B\n\ny\n'))
          assert [loc.node_id for loc in located if loc.node_type == 'Section'] == ['A', 'B']
          assert all(loc.parent_type != 'Page' for loc in located)


      # ---------------------------------------------------------------------------
      # Mutationen
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_replace_paragraph(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          block = next(
              loc for loc in engine.locate_all(tree) if loc.node_type == 'BlockGroup' and 'Second' in engine.node_code(loc.node)
          )
          engine.replace(block, 'Changed paragraph.')
          assert engine.serialize(tree) == DOC.replace('Second paragraph.', 'Changed paragraph.')


      @requires_engine
      def test_replace_section_with_new_heading_and_body(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          engine.replace(_find(engine, tree, 'Title.Section_A'), '## Section A2\n\nnew body')
          assert engine.serialize(tree) == (
              '# Title\n\nIntro text.\n\n## Section A2\n\nnew body\n\n## Section B\n\nSecond paragraph.\n'
          )
          assert 'Title.Section_A2' in _ids(engine, tree.source)
          assert 'Title.Section_A' not in _ids(engine, tree.source)


      @requires_engine
      def test_replace_changes_heading_level(engine: MarkdownEngine) -> None:
          tree = engine.parse('# A\n\n## B\n\nb\n\n## C\n\nc\n')
          engine.replace(_find(engine, tree, 'A.B'), '### B3\n\nlevel3')
          assert engine.serialize(tree) == '# A\n\n### B3\n\nlevel3\n\n## C\n\nc\n'


      @requires_engine
      def test_replace_root_section_replaces_whole_document(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          engine.replace(_find(engine, tree, 'Title'), '# Only\n\nleft')
          assert engine.serialize(tree) == '# Only\n\nleft\n'


      @requires_engine
      def test_replace_with_multiple_blocks(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          block = next(loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Section_B.'))
          engine.replace(block, 'one\n\n- a\n- b\n\n```js\nx\n```')
          assert engine.serialize(tree).endswith('## Section B\n\none\n\n- a\n- b\n\n```js\nx\n```\n')


      @requires_engine
      def test_insert_section_before_and_after(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          assert engine.insert(_find(engine, tree, 'Title.Section_B'), '## Mid\n\nmid', 'before') == 2
          assert engine.serialize(tree) == (
              '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Mid\n\nmid\n\n'
              '## Section B\n\nSecond paragraph.\n'
          )
          tree = engine.parse(DOC)
          assert engine.insert(_find(engine, tree, 'Title.Section_B'), '## After\n\nx', 'after') == 2
          assert engine.serialize(tree).endswith('Second paragraph.\n\n## After\n\nx\n')


      @requires_engine
      def test_insert_returns_number_of_blocks(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          assert engine.insert(_find(engine, tree, 'Title.Section_A'), 'single', 'after') == 1
          tree = engine.parse(DOC)
          assert engine.insert(_find(engine, tree, 'Title.Section_A'), 'p1\n\np2\n\np3', 'after') == 3


      @requires_engine
      def test_insert_before_first_node(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          engine.insert(_find(engine, tree, 'Title'), 'prefix', 'before')
          assert engine.serialize(tree) == 'prefix\n\n' + DOC


      @requires_engine
      def test_delete_section(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          engine.delete(_find(engine, tree, 'Title.Section_A'))
          assert engine.serialize(tree) == '# Title\n\nIntro text.\n\n## Section B\n\nSecond paragraph.\n'


      @requires_engine
      def test_delete_paragraph_and_whole_document(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          block = next(loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Section_A.'))
          engine.delete(block)
          assert 'First paragraph.' not in engine.serialize(tree)
          assert '## Section A' in engine.serialize(tree)
          tree = engine.parse(DOC)
          engine.delete(_find(engine, tree, 'Title'))
          assert engine.serialize(tree) == ''
          assert engine.locate_all(tree) == []


      @requires_engine
      def test_append(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          assert engine.append(tree, '## Section C\n\nappended') == 2
          assert engine.serialize(tree) == DOC + '\n## Section C\n\nappended\n'


      @requires_engine
      def test_append_to_empty_tree(engine: MarkdownEngine) -> None:
          tree = engine.empty_tree()
          assert engine.append(tree, '# New\n\nbody') == 2
          assert engine.serialize(tree) == '# New\n\nbody\n'
          assert engine.append(tree, 'tail') == 1
          assert engine.serialize(tree) == '# New\n\nbody\n\ntail\n'


      @requires_engine
      def test_mutation_normalizes_document(engine: MarkdownEngine) -> None:
          source = '# Title\n\n* item one\n* item two\n\n\nSome   *emph*   and __strong__ text.\n\n## Sec\n\nlast\n'
          tree = engine.parse(source)
          assert engine.serialize(tree) == source
          block = next(loc for loc in engine.locate_all(tree) if loc.node_id.startswith('Title.Sec.'))
          engine.replace(block, 'changed')
          assert engine.serialize(tree) == (
              '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\n\n## Sec\n\nchanged\n'
          )


      @requires_engine
      def test_mutation_normalizes_crlf_to_lf(engine: MarkdownEngine) -> None:
          tree = engine.parse('# A\r\n\r\ntext\r\n\r\n## B\r\n\r\nmore\r\n')
          block = next(loc for loc in engine.locate_all(tree) if loc.node_id.startswith('A.B.'))
          engine.replace(block, 'new')
          assert '\r' not in engine.serialize(tree)
          assert engine.serialize(tree) == '# A\n\ntext\n\n## B\n\nnew\n'


      @requires_engine
      def test_mutation_keeps_frontmatter(engine: MarkdownEngine) -> None:
          tree = engine.parse('---\ntitle: X\n---\n\n# H\n\ntext\n')
          engine.replace(_find(engine, tree, 'H'), '# H\n\nchanged')
          assert engine.serialize(tree) == '---\ntitle: X\n---\n\n# H\n\nchanged\n'


      @requires_engine
      def test_mutations_chain_on_the_same_tree(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          engine.replace(_find(engine, tree, 'Title.Section_A'), '## A\n\nfirst')
          engine.insert(_find(engine, tree, 'Title.A'), '## A2\n\nsecond', 'after')
          engine.delete(_find(engine, tree, 'Title.Section_B'))
          engine.append(tree, '## Z\n\nlast')
          assert engine.serialize(tree) == (
              '# Title\n\nIntro text.\n\n## A\n\nfirst\n\n## A2\n\nsecond\n\n## Z\n\nlast\n'
          )


      # ---------------------------------------------------------------------------
      # Fehlerfälle
      # ---------------------------------------------------------------------------


      @requires_engine
      @pytest.mark.parametrize('operation', ['replace', 'insert', 'delete'])
      def test_unknown_node_id_raises_and_keeps_source(engine: MarkdownEngine, operation: str) -> None:
          tree = engine.parse(DOC)
          ghost = dataclasses.replace(_find(engine, tree, 'Title'), node_id='Does.Not.Exist')
          with pytest.raises(AstError, match='not found'):
              if operation == 'replace':
                  engine.replace(ghost, 'x')
              elif operation == 'insert':
                  engine.insert(ghost, 'x', 'after')
              else:
                  engine.delete(ghost)
          assert tree.source == DOC


      @requires_engine
      def test_empty_replacement_is_rejected(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          with pytest.raises(AstError):
              engine.replace(_find(engine, tree, 'Title.Section_A'), '')
          assert tree.source == DOC


      @requires_engine
      def test_control_characters_are_rejected_before_the_call(engine: MarkdownEngine) -> None:
          tree = engine.parse(DOC)
          loc = _find(engine, tree, 'Title.Section_A')
          with pytest.raises(AstError, match='control character'):
              engine.replace(loc, 'a\x01b')
          with pytest.raises(AstError, match='control character'):
              engine.insert(loc, 'a\x01b', 'after')
          with pytest.raises(AstError, match='control character'):
              engine.append(tree, 'a\x01b')
          assert tree.source == DOC


      @requires_engine
      def test_imports_are_not_supported_for_markdown(engine: MarkdownEngine) -> None:
          tree = engine.parse('# A\n')
          with pytest.raises(AstError):
              engine.apply_imports(tree, ['import x'], [])
          assert tree.source == '# A\n'


      # ---------------------------------------------------------------------------
      # Robustheit
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_concurrent_requests_are_isolated(engine: MarkdownEngine) -> None:
          def work(i: int) -> bool:
              tree = engine.parse(f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nbody {i}\n')
              engine.replace(_find(engine, tree, f'Doc{i}.Sub'), f'## Sub\n\nnew {i}')
              return tree.source == f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nnew {i}\n'

          with ThreadPoolExecutor(16) as pool:
              assert all(pool.map(work, range(96)))


      @requires_engine
      def test_concurrent_first_use_spawns_a_single_process(monkeypatch: pytest.MonkeyPatch) -> None:
          monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
          process = MarkdownProcess()
          isolated = RpcEngine('markdown', process)
          try:
              with ThreadPoolExecutor(8) as pool:
                  results = list(pool.map(lambda _: isolated.validate('# a\n'), range(8)))
              assert results == [None] * 8
              assert process._process is not None
          finally:
              process._terminate()


      @requires_engine
      def test_large_document(engine: MarkdownEngine) -> None:
          sections = 300
          source = '# Big\n\n' + ''.join(
              f'## Section {i}\n\n' + ''.join(f'Paragraph {i}.{j} lorem ipsum.\n\n' for j in range(5)) for i in range(sections)
          )
          tree = engine.parse(source)
          assert len(engine.locate_all(tree)) == 1 + 2 * sections
          engine.replace(_find(engine, tree, f'Big.Section_{sections - 1}'), f'## Section {sections - 1}\n\nX')
          assert tree.source.endswith(f'## Section {sections - 1}\n\nX\n')
          assert 'Section 0' in tree.source


      @requires_engine
      def test_long_single_line(engine: MarkdownEngine) -> None:
          line = 'word ' * 20000
          tree = engine.parse(f'# A\n\n{line}\n')
          engine.append(tree, 'tail')
          assert tree.source.endswith('tail\n')
          assert line.strip() in tree.source


      # ---------------------------------------------------------------------------
      # Tool-Ebene (Dateien, Cache, ast_*-Tools)
      # ---------------------------------------------------------------------------


      @pytest.fixture
      def md_file(tmp_path: Path) -> str:
          path = tmp_path / 'doc.md'
          path.write_text(FILE_DOC, encoding='utf-8')
          return str(path)


      @requires_engine
      def test_load_uses_markdown_rpc_engine(md_file: str) -> None:
          _, tree = core.load(md_file)
          assert tree.engine.name == 'markdown'
          assert isinstance(tree.engine, RpcEngine)


      @requires_engine
      def test_outline_lists_sections_and_blocks(md_file: str) -> None:
          nodes = ast_outline([md_file]).results[0].nodes
          assert [node.id for node in nodes] == ['Doc']
          child_ids = [child.id for child in nodes[0].children]
          assert 'Doc.Part_1' in child_ids and 'Doc.Part_2' in child_ids
          assert nodes[0].excert == '# Doc'


      @requires_engine
      def test_find_by_type_and_name(md_file: str) -> None:
          sections = _flatten(ast_find(paths=[md_file], node_type='Section').files[0].nodes)
          assert [node.id for node in sections] == ['Doc', 'Doc.Part_1', 'Doc.Part_2']
          named = _flatten(ast_find(paths=[md_file], name='Part 2').files[0].nodes)
          assert [node.id for node in named] == ['Doc.Part_2']
          assert named[0].code == '## Part 2\n\nbeta\n'


      @requires_engine
      def test_read_returns_section_source(md_file: str) -> None:
          result = ast_read(items=[ReadItem(path=md_file, ids=['Doc.Part_1'])])
          assert not result.errors
          assert result.results[0].nodes[0].code == '## Part 1\n\nalpha\n'


      @requires_engine
      def test_replace_via_tool_writes_file(md_file: str) -> None:
          result = ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nALPHA', id='Doc.Part_1')])
          assert not result.errors
          assert result.results[0].new_id == 'Doc.Part_1'
          assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nALPHA\n\n## Part 2\n\nbeta\n'


      @requires_engine
      def test_insert_via_tool_reports_ids(md_file: str) -> None:
          result = ast_insert(items=[InsertItem(path=md_file, source='## Part 1b\n\nmid', id='Doc.Part_1', position='after')])
          assert not result.errors
          assert result.results[0].inserted == 2
          assert result.results[0].ids[0] == 'Doc.Part_1b'
          assert Path(md_file).read_text(encoding='utf-8') == (
              '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 1b\n\nmid\n\n## Part 2\n\nbeta\n'
          )


      @requires_engine
      def test_delete_via_tool(md_file: str) -> None:
          result = ast_delete(items=[DeleteItem(path=md_file, id='Doc.Part_2')])
          assert not result.errors
          assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nalpha\n'


      @requires_engine
      def test_edit_marks_via_tool(md_file: str) -> None:
          result = ast_edit_marks(
              items=[EditMarksItem(path=md_file, start_marker='alpha', end_marker='alpha', content='gamma', id='Doc.Part_1')]
          )
          assert not result.errors
          text = Path(md_file).read_text(encoding='utf-8')
          assert 'gamma' in text and 'alpha' not in text and 'beta' in text


      @requires_engine
      def test_edit_block_via_tool(md_file: str) -> None:
          result = ast_edit_block(items=[EditBlockItem(path=md_file, old_text='beta', new_text='delta', id='Doc.Part_2')])
          assert not result.errors
          text = Path(md_file).read_text(encoding='utf-8')
          assert 'delta' in text and 'beta' not in text and 'alpha' in text


      @requires_engine
      def test_batch_of_edits_in_one_call(md_file: str) -> None:
          result = ast_replace(
              items=[
                  ReplaceItem(path=md_file, source='## Part 1\n\nONE', id='Doc.Part_1'),
                  ReplaceItem(path=md_file, source='## Part 2\n\nTWO', id='Doc.Part_2'),
              ]
          )
          assert not result.errors and len(result.results) == 2
          text = Path(md_file).read_text(encoding='utf-8')
          assert 'ONE' in text and 'TWO' in text and 'alpha' not in text and 'beta' not in text


      @requires_engine
      def test_unknown_id_via_tool_is_reported_and_file_untouched(md_file: str) -> None:
          result = ast_replace(items=[ReplaceItem(path=md_file, source='## X\n\ny', id='Doc.Nope')])
          assert result.errors and not result.results
          assert Path(md_file).read_text(encoding='utf-8') == FILE_DOC


      @requires_engine
      def test_validate_via_tool_accepts_any_markdown(tmp_path: Path) -> None:
          path = tmp_path / 'odd.md'
          path.write_text('```\nunclosed\n\n[x](\n', encoding='utf-8')
          assert ast_validate([str(path)]).all_ok


      @requires_engine
      def test_create_via_tool_and_overwrite(tmp_path: Path) -> None:
          path = tmp_path / 'new.md'
          first = ast_create(items=[CreateFileItem(path=str(path), source='# One\n\ntext\n')])
          assert not first.errors
          assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
          refused = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n')])
          assert refused.errors
          assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
          again = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n', overwrite=True)])
          assert not again.errors
          assert path.read_text(encoding='utf-8') == '# Two\n'


      @requires_engine
      def test_cache_follows_external_changes(md_file: str) -> None:
          path = Path(md_file)
          _, first = core.load(md_file)
          assert core.CACHE.get_tree(path) is first
          path.write_text('# Other\n\nchanged\n', encoding='utf-8')
          _, second = core.load(md_file)
          assert [loc.node_id for loc in core.locate_all(second) if loc.node_type == 'Section'] == ['Other']


      @requires_engine
      def test_cache_follows_tool_edits(md_file: str) -> None:
          ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nNEW', id='Doc.Part_1')])
          _, tree = core.load(md_file)
          assert 'NEW' in tree.source
          result = ast_read(items=[ReadItem(path=md_file, ids=['Doc.Part_1'])])
          assert result.results[0].nodes[0].code == '## Part 1\n\nNEW\n'


      @requires_engine
      def test_unicode_file_roundtrip_via_tool(tmp_path: Path) -> None:
          path = tmp_path / 'u.md'
          path.write_text('# Grüße\n\nText 日本 😀\n\n## Teil\n\nalt\n', encoding='utf-8')
          ast_replace(items=[ReplaceItem(path=str(path), source='## Teil\n\nneu äöü', id='Grüße.Teil')])
          assert path.read_text(encoding='utf-8') == '# Grüße\n\nText 日本 😀\n\n## Teil\n\nneu äöü\n'


      # ---------------------------------------------------------------------------
      # ast_retrieve über die Tool-Registry
      # ---------------------------------------------------------------------------


      def _retrieve(registry: ToolRegistry, arguments: dict):
          tool = registry.get('ast_retrieve')
          assert tool is not None
          session = Session(id='markdown-bridge')
          session.enabled_tools.add('ast_retrieve')
          return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))


      @requires_engine
      def test_retrieve_outline_find_read(registry: ToolRegistry, md_file: str) -> None:
          result = _retrieve(
              registry,
              {
                  'outline': [md_file],
                  'find': [{'paths': [md_file], 'name': 'Part 1'}],
                  'read': [{'path': md_file, 'ids': ['Doc.Part_2']}],
              },
          )
          assert not result.is_error
          content = result.structured_content
          assert set(content) == {'outline', 'find', 'read'}
          assert 'Doc.Part_1' in str(content['find'])
          assert 'beta' in str(content['read'])


      @requires_engine
      def test_retrieve_search_returns_node_ids_usable_for_read(registry: ToolRegistry, md_file: str) -> None:
          directory = str(Path(md_file).parent)
          first = _retrieve(
              registry,
              {'search': [{'directory': [directory], 'pattern': 'beta', 'include': ['*.md'], 'intent': 'edit'}]},
          )
          assert not first.is_error
          entry = first.structured_content['search']['results'][0]['matches'][0]
          node_id = entry['nodes'][0]['id']
          assert node_id.startswith('Doc.Part_2')
          second = _retrieve(registry, {'read': [{'path': md_file, 'ids': [node_id]}]})
          assert not second.is_error
          assert 'beta' in str(second.structured_content['read'])


      # ---------------------------------------------------------------------------
      # Absturz des Engine-Prozesses (zuletzt: stellt den Prozess danach wieder her)
      # ---------------------------------------------------------------------------


      @requires_engine
      def test_new_engine_instance_restarts_crashed_process() -> None:
          MarkdownEngine().validate('# warm\n')
          process = _PROCESS._process
          assert process is not None
          os.kill(process.pid, signal.SIGKILL)
          process.wait(timeout=5)
          fresh = MarkdownEngine()
          assert fresh.validate('# a\n') is None
          assert _PROCESS._process is not process


      @requires_engine
      @pytest.mark.xfail(
          strict=True,
          reason='RpcEngine caches its client and only starts the process when the client is created, '
          'so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.',
      )
      def test_existing_engine_instance_recovers_after_process_crash() -> None:
          used = MarkdownEngine()
          used.validate('# warm\n')
          process = _PROCESS._process
          assert process is not None
          os.kill(process.pid, signal.SIGKILL)
          process.wait(timeout=5)
          try:
              assert used.validate('# a\n') is None
          finally:
              MarkdownEngine().validate('# restore\n')
  reason: "Umfassende Test-Suite für die Markdown-RPC-Anbindung anlegen (ersetzt die kleine Bridge-Testdatei), diesmal mit gültigem JSON"
```
/answer 2d71-10c-0 allow
Text:
 Datei angelegt. Jetzt laufen die neuen Tests.
ReasoningToken: 71
Control Request:
```yaml
id: 2d71-10d-0
toolName: python
arguments:
  script: |-
    import subprocess,sys
    r=subprocess.run([sys.executable,'-m','pytest','-q','--no-header','-p','no:cacheprovider','-rfxX','--tb=short','tests/test_ast_markdown_bridge.py'],cwd='/home/user/xyan/xy.ai.workbench/mcpc',capture_output=True,text=True,timeout=900)
    print(r.stdout[-9000:]);print(r.stderr[-1500:])
  reason: Neue Markdown-Bridge-Tests ausführen
```
/answer 2d71-10d-0 allow
Control Request:
```yaml
id: 2d71-10e-10d
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      ........................................................................ [ 80%]
      ................x                                                        [100%]
      =========================== short test summary info ============================
      XFAIL tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash - RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
      88 passed, 1 xfailed in 5.18s
```
/answer 2d71-10e-10d allow
Thinking:
88 Tests laufen grün, einer davon als erwarteter Fehlschlag (xfail). Als Nächstes lasse ich die Suite zweimal mit umgekehrter Dateireihenfolge laufen, um zu prüfen, ob der absichtliche Absturz andere Tests stört.


ReasoningToken: 147
Control Request:
```yaml
id: 2d71-10f-0
toolName: python
arguments:
  script: |-
    import subprocess,sys,glob,os
    cwd='/home/user/xyan/xy.ai.workbench/mcpc'
    def run(args):
        r=subprocess.run([sys.executable,'-m','pytest','-q','--no-header','-p','no:cacheprovider','--tb=short']+args,cwd=cwd,capture_output=True,text=True,timeout=1500)
        print(r.stdout[-1800:]); print(r.stderr[-500:])
    run(['tests'])
    files=sorted(glob.glob(cwd+'/tests/test_*.py'),reverse=True)
    run(files)
    run(['tests/test_ast_markdown_bridge.py','tests/test_ast_markdown_bridge.py'.replace('markdown','rust')])
  reason: "Gesamte Suite normal und in umgekehrter Dateireihenfolge laufen lassen, um Wechselwirkungen durch den Prozess-Absturz-Test auszuschließen"
```
/answer 2d71-10f-0 allow
Control Request:
```yaml
id: 2d71-110-10f
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      ........................................................................ [ 14%]
      .................................................................x...... [ 29%]
      ........................................................................ [ 44%]
      ........................................................................ [ 59%]
      ........................................................................ [ 74%]
      ........................................................................ [ 89%]
      .....................................................                    [100%]
      484 passed, 1 xfailed in 15.60s


      ........................................................................ [ 14%]
      ........................................................................ [ 29%]
      ........................................................................ [ 44%]
      ........................................................................ [ 59%]
      ........................................................................ [ 74%]
      ........................................................................ [ 89%]
      ...x.................................................                    [100%]
      484 passed, 1 xfailed in 15.85s


      ........................................................................ [ 77%]
      ................x....                                                    [100%]
      92 passed, 1 xfailed in 5.11s
```
/answer 2d71-110-10f allow
Text:
 Alles grün. Zuletzt prüfe ich, dass die Tests keine verwaisten Node-Prozesse zurücklassen.
ReasoningToken: 85
Control Request:
```yaml
id: 2d71-111-0
toolName: python
arguments:
  script: |-
    import subprocess
    print(subprocess.run("pgrep -af 'ast/engine/Main.ts' || echo 'keine verwaisten Engine-Prozesse'",shell=True,capture_output=True,text=True).stdout)
  reason: "Prüfen, ob nach den Testläufen Node-Engine-Prozesse zurückbleiben"
```
/answer 2d71-111-0 allow
Control Request:
```yaml
id: 2d71-112-111
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      104562 /bin/sh -c pgrep -af 'ast/engine/Main.ts' || echo 'keine verwaisten Engine-Prozesse'
```
/answer 2d71-112-111 allow
Result Stats: id=8d367516-0e0c-4674-80a5-34402faaa645, total: 209792, in: 82, out: 71095, reason: 0, read: 2373794, write: 138615, start: 1795166967459, end: 1795166979531, time: 12026, total: 510956, io: 0, control: 46, sent: 0, recv: 335
Die Anbindung funktioniert. Gefunden habe ich einen echten Mangel: Eine bereits benutzte Engine-Instanz erholt sich nicht von einem Absturz des Node-Prozesses. Die gesamte Suite läuft grün durch (484 bestanden, 1 erwarteter Fehlschlag). Sie ging in der normalen und in umgekehrter Dateireihenfolge durch. Nach den Läufen bleiben keine Node-Prozesse übrig.

Ich habe `tests/test_ast_markdown_bridge.py` auf 89 Tests erweitert. Sie starten den echten Node-Prozess und überspringen sich, wenn `node` oder `node_modules` fehlen.

## Was geprüft wird
- **Konfiguration:**
  - Der Einstiegspunkt der Engine existiert.
  - Port 8789 kollidiert nicht mit Java oder Rust.
  - Die Umgebungsvariablen `AST_MARKDOWN_PORT` und `NODE` wirken.
  - `.md` und `.markdown` führen zur neuen Engine, und `get_engine` liefert immer dieselbe Instanz.
- **Prozess:**
  - Er startet erst beim ersten Aufruf und beendet sich sauber.
  - Bei parallelem ersten Zugriff entsteht genau ein Prozess.
  - Ein fehlendes `node` oder ein ungültiger Port führt zu einem Fehler.
- **Struktur:**
  - Frontmatter, Setext-Überschriften, übersprungene Ebenen, doppelte Überschriften (`A`, `A_2`) und Unicode werden korrekt erkannt.
  - `#` in Code-Blöcken, Zitaten und Listen erzeugt keine Section.
  - Zeilenbereiche und stabile Block-IDs stimmen.
- **Mutationen:** Ersetzen, Einfügen davor und danach, Löschen und Anhängen funktionieren, auch verkettet auf demselben Baum. Dazu kommen Ebenenwechsel, mehrere Blöcke und ein leeres Dokument.
- **Normalisierung:** Wie von dir erwartet, prüfen die Tests den normalisierten Text, zum Beispiel `*` zu `-`, `*emph*` zu `_emph_` und CRLF zu LF. Frontmatter bleibt erhalten.
- **Fehler:**
  - Unbekannte IDs, leerer Ersatz und Steuerzeichen werfen einen `AstError`.
  - Der Quelltext bleibt dabei unverändert.
  - Imports werden für Markdown abgelehnt.
- **Robustheit:** 96 parallele Anfragen, ein Dokument mit rund 600 Knoten und eine Zeile mit 100 KB laufen sauber durch.
- **Tool-Kette:** `ast_create`, `outline`, `find`, `read`, `replace`, `insert`, `delete`, `edit_marks`, `edit_block` und `validate` laufen auf echten Dateien. Dazu kommen der Cache bei externen und Tool-Änderungen, ein Batch mit mehreren Edits und `ast_retrieve` mit `outline`, `find`, `read` und `search`.

## Befund: Absturz des Node-Prozesses
- **Problem:** `RpcEngine` cacht seinen HTTP-Client und startet den Prozess nur beim Anlegen des Clients. Stirbt der Prozess, wirft eine bereits benutzte Engine-Instanz dauerhaft `URLError: Connection refused`. Das betrifft auch die Instanz aus `get_engine('markdown')`, bis der Server neu startet.
- **Test:** Eine frisch erzeugte Engine startet den Prozess korrekt neu, und das sichert ein Test ab. Der Fall der bereits benutzten Instanz steht als `xfail(strict=True)` in der Suite. Sobald er behoben ist, schlägt dieser Test fehl und zeigt das an.
- **Umfang:** Der Fehler liegt in der gemeinsamen Basisklasse `rpc/_engine.py`. Java und Rust sind sehr wahrscheinlich ebenfalls betroffen. Ich habe das nicht geprüft und nichts geändert.

## Verhalten zum Beachten
- **Seitenumbrüche:** `---` erzeugt keine eigenen Knoten und gehört zum Block davor. Ersetzt du eine Section, die mit `---` endet, durch Text ohne `---`, geht der Umbruch verloren. Das passt zum bisherigen Entwurf, ist aber leicht zu übersehen.
- **Einfügen nach einer Section:** Ein Absatz, den du nach einer Section einfügst, landet in dieser Section, weil Markdown keine Abschnittsgrenze kennt.

Soll ich den Absturzfehler in `RpcEngine` beheben? Dann würde der Client bei jedem Aufruf zuerst prüfen, ob der Prozess noch läuft. Das würde auch Java und Rust helfen.
