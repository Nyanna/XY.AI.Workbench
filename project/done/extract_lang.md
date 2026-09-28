Der Codegenerator soll in Zukunft weitere Sprachen unterstützen. Extrahiere alle Sprachspezifika in das Package in `/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang`.

Das meiste befindet sich in `/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit`.

Beispiel:
```python
PARAMETER_JAVA_TYPE = {'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
PARAMETER_PHP_TYPE = {'integer': 'int', 'number': 'float', 'boolean': 'bool'}
PARAMETER_TYPE = {'java': PARAMETER_JAVA_TYPE, 'php': PARAMETER_PHP_TYPE}
PARAMETER_DEFAULT_TYPE = {'java': 'String', 'php': 'string'}
```

Projekt Readme: `/home/user/xyan/xy.ai.workbench/codegen/README.md`
Der Zweck it einen zentralen Ort zu haben an dem die Sprachspezifika konzentriert sind ("php.py"/"java.py").
So ist für das hinzufügen weiterer Sprachen nur ein Satz Templates und die Sprachspezifika erforderlich. 
SystemInit:  id=c37f561e-5bde-4ca8-8db7-a039432c33f8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 8472-1
toolName: bash
arguments:
  script: cat /home/user/xyan/xy.ai.workbench/codegen/README.md
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Projekt-README lesen für Kontext
```
/answer 8472-1 allow
Control Request:
```yaml
id: 8472-2
result:
  content:
  - type: text
    text: "Full output written to file (4891 characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-4zerfr8w.log
```
/answer 8472-2 allow
Control Request:
```yaml
id: 8472-3
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
  reason: Struktur des codegen Package prüfen
```
/answer 8472-3 allow
Control Request:
```yaml
id: 8472-4
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
      error: Too many entries (62) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 8472-4 allow
ReasoningToken: 13
Control Request:
```yaml
id: 8472-5
toolName: bash
arguments:
  script: cat /tmp/mcpc-stdout-4zerfr8w.log
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: README Inhalt lesen
```
/answer 8472-5 allow
Control Request:
```yaml
id: 8472-6
result:
  content:
  - type: text
    text: "Full output written to file (4891 characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-hd0pe9v0.log
```
/answer 8472-6 allow
Control Request:
```yaml
id: 8472-7
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang
  reason: Struktur emit und lang Verzeichnisse prüfen
```
/answer 8472-7 allow
Control Request:
```yaml
id: 8472-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
      entries:
      - ".:"
      - __init__.py
      - client_context.py
      - client_emit.py
      - io_context.py
      - io_emit.py
      - model_context.py
      - model_emit.py
      - server_context.py
      - server_emit.py
      - templates.py
      - writer.py
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang
      entries: []
```
/answer 8472-8 allow
Control Request:
```yaml
id: 8472-9
toolName: bash
arguments:
  script: echo "---README---"; cat /tmp/mcpc-stdout-hd0pe9v0.log; echo "---lang exists?---"; ls -la /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang 2>&1
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: README lesen und lang-Verzeichnis Status prüfen
```
/answer 8472-9 allow
Control Request:
```yaml
id: 8472-a
result:
  content:
  - type: text
    text: "Full output written to file (5020 characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-ewc6vjql.log
```
/answer 8472-a allow
ReasoningToken: 62
Control Request:
```yaml
id: 8472-b
toolName: bash
arguments:
  script: cat /tmp/mcpc-stdout-ewc6vjql.log
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Ausgabe lesen
```
/answer 8472-b allow
Control Request:
```yaml
id: 8472-c
result:
  content:
  - type: text
    text: "Full output written to file (5020 characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-2ndjk3_h.log
```
/answer 8472-c allow
Control Request:
```yaml
id: 8472-d
toolName: python
arguments:
  script: |2

    import subprocess
    print(open('/home/user/xyan/xy.ai.workbench/codegen/README.md').read())
    print("---LANG---")
    import os
    print(os.path.exists('/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang'))
    print(os.listdir('/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang') if os.path.exists('/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang') else "none")
  reason: README und lang Verzeichnis direkt lesen
```
/answer 8472-d allow
Control Request:
```yaml
id: 8472-e
result:
  structuredContent:
    exit_code: 0
    stdout: |
      # cgen - OpenAPI 3.1 to Code Generator

      A type-safe code generator that transforms OpenAPI 3.1 YAML specifications into well-structured, production-ready source code. The target language is selected via `--language`, which picks the corresponding `templates/<language>` directory.

      ## Overview

      **cgen** is a code generation tool designed to bridge the gap between API specifications and code implementation. It reads OpenAPI 3.1 schemas and generates corresponding models, clients, and supporting code with full type safety and zero boilerplate.

      ### Key Features

      - 🎯 **Type-Safe**: Generates fully typed code with strong compile-time safety guarantees
      - 📋 **OpenAPI 3.1 Support**: Comprehensive support for OpenAPI 3.1 YAML specifications
      - 🏗️ **Modular Architecture**: Clean separation of concerns through a multi-stage pipeline
      - 🔄 **Lossless Processing**: Preserves semantic information through schema ingestion, modeling, and naming stages

      ## Concept

      The generator operates through a well-defined, multi-stage pipeline:

      ```
      OpenAPI YAML Schema
             ↓
         [Ingest] ─────────────── Parse and validate OpenAPI specification
             ↓
         [Model] ─────────────── Build internal representation of API structure
             ↓
        [Identity] ─────────────── Compute semantic identities and resolve references
             ↓
        [Optimize] ────────────── Optimize identity and eliminate redundancy
             ↓
        [Naming] ─────────────── Assign compatible names and package hierarchy
             ↓
         [Emit] ─────────────── Generate and write source files
             ↓
         Source Code
      ```

      ### Pipeline Stages

      1. **Ingest**: Reads and parses the OpenAPI 3.1 YAML schema, validating its structure
      2. **Model**: Constructs an internal domain model representing the API's types and operations
      3. **Identity**: Computes semantic identities and resolves schema references and definitions
      4. **Optimize**: Deduplicates and optimizes the identified model for efficient code generation
      5. **Naming**: Maps internal identifiers to compliant names and determines package structure
      6. **Emit**: Generates and writes the final source files to the output directory

      ## Usage

      ### Command-Line Interface

      ```bash
      cgen --schema <path-to-openapi.yaml> --out <output-directory> [--base-package <package.name>] [--language java|php]
      ```

      ### Arguments

      - `--schema` (required): Path to the OpenAPI 3.1 YAML schema file
      - `--out` (required): Output directory where generated Java sources will be written
      - `--base-package` (optional): Root package for generated code
      - `--language` (optional): Target language, `java` or `php` (default: `java`); selects the `templates/<language>` directory

      ## Project Structure

      ```
      codegen/
      ├── src/
      │   └── xy/cgen/
      │       ├── __init__.py           # Package initialization
      │       ├── __main__.py           # Module entry point
      │       ├── cli.py                # Command-line argument parsing
      │       ├── config.py             # Configuration management
      │       ├── pipeline.py           # Pipeline orchestration
      │       ├── naming/               # Naming and package mapping
      │       │   ├── identifiers.py    # identifier generation
      │       │   ├── names.py          # Name resolution
      │       │   ├── packages.py       # Package hierarchy mapping
      │       │   ├── paths.py          # Path-based naming
      │       │   └── traverse.py       # Model traversal utilities
      │       ├── typemap/              # Type mapping configuration
      │       ├── emit/                 # Code emission
      │       │   ├── model_emit.py     # Model class generation
      │       │   ├── client_emit.py    # API client generation
      │       │   ├── model_context.py  # Model context management
      │       │   ├── client_context.py # Client context management
      │       │   ├── io_context.py     # I/O context and file writing
      │       │   └── writer.py         # File writing utilities
      │       ├── model/                # Internal domain modeling
      │       ├── identity/             # Semantic identity computation
      │       ├── ingest/               # OpenAPI schema ingestion
      │       └── naming/               # naming conventions
      ├── pyproject.toml               # Project metadata and dependencies
      ├── .gitignore                   # Git configuration
      └── README.md                    # This file
      ```

      ## Architecture

      ### Internal Data Flow

      The generator maintains a clean separation between:

      - **Parsing**: OpenAPI specification → raw AST
      - **Modeling**: Raw AST → strongly-typed domain model
      - **Semantic Analysis**: Domain model → identified/optimized representations
      - **Naming**: Semantic model → Compatible names and package structure
      - **Emission**: Named model → Source files

      Each stage is independent and can be evolved, tested, or replaced without affecting others.

      ## Development

      ### Running from Source

      ```bash
      export PYTHONPATH=src
      python3 -m xy.cgen --schema path/to/schema.yaml --out output --base-package com.example
      ```
      ---LANG---
      True
      []
```
/answer 8472-e allow
ReasoningToken: 38
Control Request:
```yaml
id: 8472-f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap
  reason: Überblick über emit und typemap Module gewinnen
```
/answer 8472-f allow
Control Request:
```yaml
id: 8472-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
      nodes:
      - id: UKZFWe|iGaZar
        signature: "'Maps IR nodes to the Java type used at their use-site (getter/setter, list\\nel…"
      - id: b1RxKE|KbNbot
        signature: "from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryN…"
      - id: l0SAq6|rpTVV7
        signature: "JAVA_PRIMITIVE_TYPE = {'string': 'String', 'integer': 'Long', 'number': 'Double…"
      - id: FuHGBT|dhQsz1
        signature: "_GENERATED_CLASS_KINDS = (EnumNode, ListNode, DictionaryNode, ObjectNode, Compo…"
      - id: map_type
        signature: "def map_type(node, named_model) -> str:"
        docstring: Resolve the target-language type for a given IR node. `named_model` supplies th…
      - id: map_primitive
        signature: "def _map_primitive(node: PrimitiveNode, language: str) -> str:"
    errors:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
      error: Not a regular file.
```
/answer 8472-10 allow
Text:
 lang existiert bereits leer. Jetzt schaue ich mir die emit-Dateien und typemap an, um alle Sprachspezifika zu identifizieren.
Text:
 emit ist ein Verzeichnis - muss als transparent expanded werden, aber hat 10 Dateien (>5). Ich liste die einzelnen Dateien.
Control Request:
```yaml
id: 8472-11
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    ids:
    - UKZFWe|iGaZar
    - l0SAq6|rpTVV7
    - FuHGBT|dhQsz1
    - map_type
    - map_primitive
  reason: Vollständigen Inhalt typemap lesen
```
/answer 8472-11 allow
Control Request:
```yaml
id: 8472-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
      nodes:
      - id: UKZFWe|iGaZar
        type: statements
        code: "'Maps IR nodes to the Java type used at their use-site (getter/setter, list\\nelement, dictionary value, ...). Used by naming/emit.\\n\\nPrimitives are always boxed (never `int`/`double`/...) since a field can be\\nabsent or explicitly null. `format`/validators are ignored -- they never\\naffect the resolved type. `null` has no Java type of its own: absent vs.\\nexplicit-null is a getter concern on the composition view that contains it\\n, never a standalone type.\\n'"
      - id: l0SAq6|rpTVV7
        type: statements
        code: |-
          JAVA_PRIMITIVE_TYPE = {'string': 'String', 'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
          PHP_PRIMITIVE_TYPE = {'string': 'string', 'integer': 'int', 'number': 'float', 'boolean': 'bool'}
          PRIMITIVE_TYPE = {'java': JAVA_PRIMITIVE_TYPE, 'php': PHP_PRIMITIVE_TYPE}
          ANY_DICTIONARY_JAVA_TYPE = 'com.fasterxml.jackson.databind.JsonNode'
          ANY_DICTIONARY_TYPE = {'java': ANY_DICTIONARY_JAVA_TYPE, 'php': 'mixed'}
          '# Node kinds whose type is the generated class assigned by naming.'
      - id: FuHGBT|dhQsz1
        type: statements
        code: "_GENERATED_CLASS_KINDS = (EnumNode, ListNode, DictionaryNode, ObjectNode, CompositionNode)"
      - id: map_type
        type: FunctionDef
        code: |-
          def map_type(node, named_model) -> str:
              """Resolve the target-language type for a given IR node.

              `named_model` supplies the generated class name/package (as a dotted fqn,
              regardless of target language -- Java uses it verbatim, PHP templates
              convert it via the `phpfqn`/`phpns` filters) for node kinds that are not
              primitives (RefNode resolves through the referenced named schema's own entry).
              """
              if isinstance(node, PrimitiveNode):
                  return _map_primitive(node, named_model.language)
              if isinstance(node, AnyDictionaryNode):
                  return ANY_DICTIONARY_TYPE[named_model.language]
              if isinstance(node, RefNode):
                  return named_model.name_of_ref(node.name).fqn
              if isinstance(node, _GENERATED_CLASS_KINDS):
                  return named_model.name_of(node).fqn
              if isinstance(node, UnsupportedNode):
                  raise ValueError(f'unsupported node has no target type (reason={node.reason!r})')
              raise TypeError(f'cannot map node kind to a target type: {node.kind!r}')
      - id: map_primitive
        type: FunctionDef
        code: |-
          def _map_primitive(node: PrimitiveNode, language: str) -> str:
              if node.primitive_type == 'null':
                  raise ValueError("'null' has no standalone type -- resolve via the enclosing composition view")
              return PRIMITIVE_TYPE[language][node.primitive_type]
```
/answer 8472-12 allow
Control Request:
```yaml
id: 8472-13
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_emit.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
  reason: Struktur der wichtigsten emit-Dateien sehen
```
/answer 8472-13 allow
Control Request:
```yaml
id: 8472-14
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py
      nodes:
      - id: rCFVcv|WI6I9X
        signature: '''Renders Java model classes from named IR nodes. No dependency on client emissi…'
      - id: 9k2Jpd|lyUX9I
        signature: from pathlib import Path
      - id: gOVtXN|92hhUO
        signature: '''# Node kinds that become a generated model class. UnsupportedNode never gets a'''
      - id: Zv2BLp|Ur7UfA
        signature: '''# never construct it. CompositionNode is one node -> one proxy-view class'''
      - id: emit_model
        signature: "def emit_model(model, writer) -> None:"
        docstring: Render and write one .java file per reachable Object/List/Dictionary/Enum/AnyDi…
      - id: render
        signature: "def _render(node, name, model, is_request_root: bool) -> str:"
      - id: render_object
        signature: "def _render_object(node: ObjectNode, name, model, is_request_root: bool) -> str:"
      - id: render_list
        signature: "def _render_list(node: ListNode, name, model, is_request_root: bool) -> str:"
      - id: render_dictionary
        signature: "def _render_dictionary(node: DictionaryNode, name, model, is_request_root: bool) -> str:"
      - id: render_enum
        signature: "def _render_enum(node: EnumNode, name, language: str) -> str:"
      - id: render_any_dictionary
        signature: "def _render_any_dictionary(name, model, is_request_root: bool) -> str:"
      - id: render_composition
        signature: "def _render_composition(node: CompositionNode, name, model, is_request_root: bool) -> str:"
      - id: Se6iUX|XDpjrz
        signature: "\"# --- Graph traversal: every node reachable from named schemas or an operation…"
      - id: reachable_nodes
        signature: "def _reachable_nodes(model):"
      - id: walk
        signature: "def _walk(node, named_nodes: dict, visited: set):"
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_emit.py
      nodes:
      - id: Pm4Q0f|OCyd8x
        signature: '''Renders the HTTP client interface and implementation. Depends on model and io …'
      - id: dOBIsk|v5gZVj
        signature: from pathlib import Path
      - id: emit_client
        signature: "def emit_client(model, writer) -> None:"
        docstring: Render the client interface and its HttpClient-based implementation.
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: fEcyji|6bViKb
        signature: '"Builds the Jinja2 template context for one model-emission node.\n\nEvery child…'
      - id: c7okcs|eSf5vv
        signature: import re
      - id: eGXYlS|GfBTZC
        signature: "JAVA_PRIMITIVE_READ_METHOD = {'string': 'asText', 'integer': 'asLong', 'number'…"
      - id: X9s0wx|D3k0eh
        signature: "'# PHP cast keyword used as a prefix cast, e.g. (int) $value -- reuses the scal…"
      - id: Accessor
        signature: "@dataclass(frozen=True) class Accessor:"
        docstring: "One child edge rendered as a getter/setter (or get/add/remove, get/put/remove)."
      - id: resolve_structural
        signature: "def resolve_structural(node, named_nodes: dict):"
        docstring: Follow a RefNode chain to the underlying structural node. Used only to pick a c…
      - id: classify
        signature: "def classify(node, named_nodes: dict) -> tuple[str, str | None]:"
        docstring: "-> (category, primitive_type). primitive_type is set for primitive/enum only. '…"
      - id: build_accessor
        signature: "def build_accessor(label: str, edge, named_model) -> Accessor | None:"
        docstring: "Build the accessor context for one edge, or None if it has no view."
      - id: RWleuK|vsSUHL
        signature: '''# --- Enum-specific context -------------------------------------------------'''
      - id: EnumConstant
        signature: "@dataclass(frozen=True) class EnumConstant:"
      - id: enum_constants
        signature: "def enum_constants(node, language: str='java') -> list[EnumConstant]:"
        docstring: "One enum constant per declared value, in declaration order (deterministic input…"
      - id: constant_base
        signature: "def _constant_base(value) -> str:"
      - id: literal
        signature: "def _literal(value, primitive_type: str, language: str) -> str:"
      - id: java_string_literal
        signature: "def _java_string_literal(text: str) -> str:"
      - id: php_string_literal
        signature: "def _php_string_literal(text: str) -> str:"
      - id: enum_raw_type
        signature: "def enum_raw_type(node, language: str='java') -> str:"
      - id: VlFitk|tm9dEi
        signature: '''# --- Composition context (allOf/anyOf/oneOf proxy views) -----------'''
      - id: y7NDex|Qy7hVD
        signature: '"# through the enclosing property''s setter (which replaces the whole bound"'
      - id: Branch
        signature: "@dataclass(frozen=True) class Branch:"
        docstring: "One composition branch, rendered as get<Name>()/is<Name>()."
      - id: branch_accessor_name
        signature: "def _branch_accessor_name(target, named_model) -> str:"
        docstring: The branch's own generated class-name fragment.
      - id: build_branches
        signature: "def build_branches(node, named_model) -> list[Branch]:"
        docstring: Build the render context for every branch of one CompositionNode.
      - id: resolve_discriminator_values
        signature: "def _resolve_discriminator_values(node, named_nodes) -> dict:"
        docstring: "Branch index -> [(value, primitive_type), ...] for the discriminator property. …"
      - id: discriminator_value_type
        signature: "def _discriminator_value_type(node, branch_ref_name: str, named_nodes) -> str:"
      - id: const_value_for_branch
        signature: "def _const_value_for_branch(target, property_name: str, named_nodes):"
        docstring: "-> (value, primitive_type) if the branch declares a single fixed value for the …"
      - id: discriminator_literal_expr
        signature: "def _discriminator_literal_expr(property_name: str, value, primitive_type: str, language: str) -> str:"
        docstring: A boolean expression testing `node`'s discriminator property against one value.
      - id: structural_applies_expr
        signature: "def _structural_applies_expr(resolved, language: str) -> str:"
        docstring: "presence of required fields / JSON type, no discriminator const available."
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
      nodes:
      - id: 3rduAy|haKDRu
        signature: '''Builds the Jinja2 template context for the client interface/implementation.\n\…'
      - id: LYiZTp|M4WQQL
        signature: import re
      - id: z0x58G|COEEvN
        signature: "PARAMETER_JAVA_TYPE = {'integer': 'Long', 'number': 'Double', 'boolean': 'Boole…"
      - id: parameter_type
        signature: "def _parameter_type(schema: dict | None, language: str) -> str:"
        docstring: Path/query parameters never enter the body tree; map their raw JSON-Schema type…
      - id: MethodParameter
        signature: "@dataclass(frozen=True) class MethodParameter:"
        docstring: "One path/query parameter, or the synthetic body parameter."
      - id: build_parameter
        signature: "def _build_parameter(param, kind: str, language: str) -> MethodParameter:"
      - id: java_string_literal
        signature: "def _java_string_literal(text: str) -> str:"
      - id: php_string_literal
        signature: "def _php_string_literal(text: str) -> str:"
      - id: id98Oo|sRRjxF
        signature: "_PHP_SCALAR_TYPES = {'int', 'float', 'string', 'bool', 'mixed'}"
      - id: php_type_hint
        signature: "def _php_type_hint(type_name: str) -> str:"
        docstring: A dotted fqn ('pkg.Class') becomes a fully-qualified PHP type hint; scalars pas…
      - id: signature
        signature: "def _signature(parameters: tuple, language: str) -> str:"
      - id: path_url_expression
        signature: "def _path_url_expression(path: str, path_params: tuple, language: str) -> str:"
        docstring: "A string-concatenation expression rebuilding the URL path, with every '{param}'…"
      - id: method_name
        signature: "def _method_name(operation) -> str:"
        docstring: operationId wins verbatim (D-decision); otherwise '<method><Path>' (e.g. postRe…
      - id: ClientMethod
        signature: "@dataclass(frozen=True) class ClientMethod:"
        docstring: "One operation's client-facing method: interface signature plus everything the i…"
      - id: build_client_methods
        signature: "def build_client_methods(named_model) -> list[ClientMethod]:"
        docstring: "One ClientMethod per operation, in deterministic (path, method) order."
      - id: top_path_segment
        signature: "def _top_path_segment(path: str) -> str:"
      - id: client_interface_name
        signature: "def client_interface_name(named_model) -> str:"
        docstring: Deterministic name from the sorted set of top-level path segments (single path …
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
      nodes:
      - id: 4o8KdJ|dFSJjr
        signature: '''Builds the Jinja2 template context for request/response root serialization.\n\…'
      - id: iWMxMr|uSY70X
        signature: from dataclasses import dataclass
      - id: request_root_node
        signature: "def request_root_node(operation_model, named_nodes):"
        docstring: The structural node whose generated class becomes this operation's request root…
      - id: request_root_node_ids
        signature: "def request_root_node_ids(model) -> set:"
        docstring: id() of every node that must render toString()/fromString().
      - id: json_support_fqn
        signature: "def json_support_fqn(base_package: str) -> str:"
      - id: ContentTypeBranch
        signature: "@dataclass(frozen=True) class ContentTypeBranch:"
        docstring: "One content-type view of a CodeNode: is<Ct>()/get<Ct>(), discriminated by heade…"
      - id: build_content_type_branches
        signature: "def build_content_type_branches(code_node, named_model) -> list:"
        docstring: "One branch per ContentTypeView, in declaration order."
      - id: CodeBranch
        signature: "@dataclass(frozen=True) class CodeBranch:"
        docstring: "One status-code view of a ResponseNode: getCode<code>() -- null unless it match…"
      - id: build_code_branches
        signature: "def build_code_branches(response_node, named_model) -> list:"
      - id: W0p47r|wrwzO7
        signature: "'# read_method -> JsonNodeFactory factory-method name, for constructing a raw'"
      - id: ResponseSetter
        signature: "@dataclass(frozen=True) class ResponseSetter:"
        docstring: "One setCode<NNN>[<ContentTypeSuffix>](...) constructor-style setter on a Respon…"
      - id: build_response_setters
        signature: "def build_response_setters(response_node, named_model) -> list:"
        docstring: "One ResponseSetter per (status code, content type) combination. The content-typ…"
```
/answer 8472-14 allow
Control Request:
```yaml
id: 8472-15
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    ids:
    - eGXYlS|GfBTZC
    - X9s0wx|D3k0eh
    - Accessor
    - resolve_structural
    - classify
    - build_accessor
    - EnumConstant
    - enum_constants
    - constant_base
    - literal
    - java_string_literal
    - php_string_literal
    - enum_raw_type
    - Branch
    - branch_accessor_name
    - build_branches
    - resolve_discriminator_values
    - discriminator_value_type
    - const_value_for_branch
    - discriminator_literal_expr
    - structural_applies_expr
  reason: Vollständigen Code model_context.py lesen für Extraktion
```
/answer 8472-15 allow
Control Request:
```yaml
id: 8472-16
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: eGXYlS|GfBTZC
        type: statements
        code: |-
          JAVA_PRIMITIVE_READ_METHOD = {'string': 'asText', 'integer': 'asLong', 'number': 'asDouble', 'boolean': 'asBoolean'}
          '# JsonNodeFactory typed-constructor names, used only where ArrayNode has no typed set() overload'
          '# (mixed/tuple lists, see list_mixed.jinja). Java-only -- PHP arrays need no factory.'
          JAVA_PRIMITIVE_FACTORY_METHOD = {
              'string': 'textNode',
              'integer': 'numberNode',
              'number': 'numberNode',
              'boolean': 'booleanNode'}
      - id: X9s0wx|D3k0eh
        type: statements
        code: |-
          '# PHP cast keyword used as a prefix cast, e.g. (int) $value -- reuses the scalar type map.'
          PHP_PRIMITIVE_CAST = PRIMITIVE_TYPE['php']
          READ_METHOD = {'java': JAVA_PRIMITIVE_READ_METHOD, 'php': PHP_PRIMITIVE_CAST}
          FACTORY_METHOD = {'java': JAVA_PRIMITIVE_FACTORY_METHOD, 'php': {}}
      - id: Accessor
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class Accessor:
              """One child edge rendered as a getter/setter (or get/add/remove, get/put/remove)."""
              label: str
              "# PascalCase accessor fragment, e.g. getTopLogprobs -> 'TopLogprobs'"
              name: str
              java_type: str
              "# 'primitive' | 'enum' | 'any_dictionary' | 'complex'"
              category: str
              read_method: str | None
              factory_method: str | None
              description: str | None
              example_repr: str | None
      - id: resolve_structural
        type: FunctionDef
        code: |-
          def resolve_structural(node, named_nodes: dict):
              """Follow a RefNode chain to the underlying structural node.

              Used only to pick a codegen strategy (which accessor shape to emit);
              the IR itself keeps $ref atomic.
              """
              seen = set()
              while isinstance(node, RefNode):
                  if node.name in seen:
                      raise ValueError(f'cyclic reference chain at {node.name!r}')
                  seen.add(node.name)
                  node = named_nodes[node.name]
              return node
      - id: classify
        type: FunctionDef
        code: |-
          def classify(node, named_nodes: dict) -> tuple[str, str | None]:
              """-> (category, primitive_type). primitive_type is set for primitive/enum only.

              'any_dictionary' mirrors map_type's own precedence: it
              only collapses to raw JsonNode for a *direct* AnyDictionaryNode edge --
              map_type never resolves through a RefNode, so a named
              additionalProperties:true schema reached via $ref keeps its own
              generated proxy class and must be classified as 'complex' here too,
              or the getter's declared return type and its body would disagree.
              """
              if isinstance(node, AnyDictionaryNode):
                  return ('any_dictionary', None)
              resolved = resolve_structural(node, named_nodes)
              if isinstance(resolved, PrimitiveNode):
                  return ('primitive', resolved.primitive_type)
              if isinstance(resolved, EnumNode):
                  return ('enum', resolved.primitive_type)
              if isinstance(resolved, UnsupportedNode):
                  return ('unsupported', None)
              if isinstance(resolved, CompositionNode):
                  return ('composition', None)
              return ('complex', None)
      - id: build_accessor
        type: FunctionDef
        code: |-
          def build_accessor(label: str, edge, named_model) -> Accessor | None:
              """Build the accessor context for one edge, or None if it has no view."""
              category, primitive_type = classify(edge.target, named_model.named_nodes)
              if category == 'unsupported':
                  return None
              language = named_model.language
              return Accessor(
                  label=label, name=to_pascal_case(
                      property_accessor_name(label)), java_type=map_type(
                          edge.target, named_model), category=category, read_method=READ_METHOD[language].get(primitive_type) if category in (
                              'primitive', 'enum') else None, factory_method=FACTORY_METHOD[language].get(primitive_type) if category in (
                                  'primitive', 'enum') else None, description=edge.description, example_repr=None if edge.example is MISSING else repr(
                                      edge.example))
      - id: EnumConstant
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EnumConstant:
              constant_name: str
              literal: str
      - id: enum_constants
        type: FunctionDef
        code: |-
          def enum_constants(node, language: str='java') -> list[EnumConstant]:
              """One enum constant per declared value, in declaration order (deterministic input)."""
              seen_names: dict = {}
              constants = []
              for value in node.values:
                  base = _constant_base(value)
                  seen_names[base] = seen_names.get(base, 0) + 1
                  name = base if seen_names[base] == 1 else f'{base}_{seen_names[base]}'
                  constants.append(EnumConstant(constant_name=name, literal=_literal(value, node.primitive_type, language)))
              return constants
      - id: constant_base
        type: FunctionDef
        code: |-
          def _constant_base(value) -> str:
              words = [w for w in _WORD_BOUNDARY.split(str(value)) if w]
              if not words:
                  return 'VALUE'
              base = '_'.join((w.upper() for w in words))
              return f'_{base}' if base[0].isdigit() else base
      - id: literal
        type: FunctionDef
        code: |-
          def _literal(value, primitive_type: str, language: str) -> str:
              if primitive_type == 'boolean':
                  return 'true' if value else 'false'
              if language == 'php':
                  if primitive_type == 'string':
                      return _php_string_literal(str(value))
                  if primitive_type == 'integer':
                      return str(int(value))
                  if primitive_type == 'number':
                      return repr(float(value))
                  raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')
              if primitive_type == 'string':
                  return _java_string_literal(str(value))
              if primitive_type == 'integer':
                  return f'{int(value)}L'
              if primitive_type == 'number':
                  return f'{float(value)}d'
              raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')
      - id: java_string_literal
        type: FunctionDef
        code: |-
          def _java_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r')
              return f'"{escaped}"'
      - id: php_string_literal
        type: FunctionDef
        code: |-
          def _php_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace("'", "\\'")
              return f"'{escaped}'"
      - id: enum_raw_type
        type: FunctionDef
        code: |-
          def enum_raw_type(node, language: str='java') -> str:
              return PRIMITIVE_TYPE[language][node.primitive_type]
      - id: Branch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class Branch:
              """One composition branch, rendered as get<Name>()/is<Name>()."""
              accessor_name: str
              "# None for the 'null' branch (no getter)"
              java_type: str | None
              "# 'primitive' | 'enum' | 'any_dictionary' | 'composition' | 'complex' | 'null'"
              category: str
              read_method: str | None
              description: str | None
              example_repr: str | None
              has_getter: bool
              '# None for allOf branches (always applies, no check)'
              applies_expr: str | None
      - id: branch_accessor_name
        type: FunctionDef
        code: |-
          def _branch_accessor_name(target, named_model) -> str:
              """The branch's own generated class-name fragment."""
              if isinstance(target, RefNode):
                  return named_model.name_of_ref(target.name).class_name
              if isinstance(target, PrimitiveNode):
                  return PRIMITIVE_BRANCH_NAME[target.primitive_type]
              return named_model.name_of(target).class_name
      - id: build_branches
        type: FunctionDef
        code: |-
          def build_branches(node, named_model) -> list[Branch]:
              """Build the render context for every branch of one CompositionNode."""
              named_nodes = named_model.named_nodes
              language = named_model.language
              needs_applies = node.keyword in ('anyOf', 'oneOf')
              discriminator_values = _resolve_discriminator_values(
                  node, named_nodes) if needs_applies and node.discriminator else {}
              branches = []
              for index, edge in enumerate(node.branches):
                  target = edge.target
                  resolved = resolve_structural(target, named_nodes)
                  if isinstance(resolved, UnsupportedNode):
                      '# no view for an unsupported branch'
                      continue
                  example_repr = None if edge.example is MISSING else repr(edge.example)
                  accessor_name = _branch_accessor_name(target, named_model)
                  applies_expr = None
                  if needs_applies:
                      values = discriminator_values.get(index)
                      if values:
                          applies_expr = ' || '.join(
                              (_discriminator_literal_expr(
                                  node.discriminator.property_name,
                                  value,
                                  primitive_type,
                                  language) for value,
                               primitive_type in values))
                      else:
                          applies_expr = _structural_applies_expr(resolved, language)
                  if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':
                      branches.append(
                          Branch(
                              accessor_name=accessor_name,
                              java_type=None,
                              category='null',
                              read_method=None,
                              description=edge.description,
                              example_repr=example_repr,
                              has_getter=False,
                              applies_expr=applies_expr))
                      continue
                  category, primitive_type = classify(target, named_nodes)
                  branches.append(
                      Branch(
                          accessor_name=accessor_name,
                          java_type=map_type(
                              target,
                              named_model),
                          category=category,
                          read_method=READ_METHOD[language].get(primitive_type) if category in (
                              'primitive',
                              'enum') else None,
                          description=edge.description,
                          example_repr=example_repr,
                          has_getter=True,
                          applies_expr=applies_expr))
              return branches
      - id: resolve_discriminator_values
        type: FunctionDef
        code: |-
          def _resolve_discriminator_values(node, named_nodes) -> dict:
              """Branch index -> [(value, primitive_type), ...] for the discriminator property.

              `mapping` (if present) wins per branch; every branch left unresolved falls
              back to reading the const/single-value-enum of its own discriminator
              property. A branch with neither yields no entry (caller falls back to a
              structural applies? check for it).
              """
              values: dict = {}
              mapping = node.discriminator.mapping
              if mapping:
                  ref_to_values: dict = {}
                  for value, ref in mapping.items():
                      key = ref.rsplit('/', maxsplit=1)[-1] if '/' in ref else ref
                      ref_to_values.setdefault(key, []).append(value)
                  for index, edge in enumerate(node.branches):
                      target = edge.target
                      if isinstance(target, RefNode) and target.name in ref_to_values:
                          values[index] = [(value, _discriminator_value_type(node, target.name, named_nodes))
                                           for value in ref_to_values[target.name]]
              for index, edge in enumerate(node.branches):
                  if index in values:
                      continue
                  resolved_value = _const_value_for_branch(edge.target, node.discriminator.property_name, named_nodes)
                  if resolved_value is not None:
                      values[index] = [resolved_value]
              return values
      - id: discriminator_value_type
        type: FunctionDef
        code: |-
          def _discriminator_value_type(node, branch_ref_name: str, named_nodes) -> str:
              resolved_value = _const_value_for_branch(
                  RefNode(
                      name=branch_ref_name),
                  node.discriminator.property_name,
                  named_nodes)
              return resolved_value[1] if resolved_value is not None else 'string'
      - id: const_value_for_branch
        type: FunctionDef
        code: |-
          def _const_value_for_branch(target, property_name: str, named_nodes):
              """-> (value, primitive_type) if the branch declares a single fixed value
              for the discriminator property (enum with exactly one value), else None."""
              resolved = resolve_structural(target, named_nodes)
              if not isinstance(resolved, ObjectNode):
                  return None
              for prop_edge in resolved.properties:
                  if prop_edge.label != property_name:
                      continue
                  prop_resolved = resolve_structural(prop_edge.target, named_nodes)
                  if isinstance(prop_resolved, EnumNode) and len(prop_resolved.values) == 1:
                      return (prop_resolved.values[0], prop_resolved.primitive_type)
                  return None
              return None
      - id: discriminator_literal_expr
        type: FunctionDef
        code: |-
          def _discriminator_literal_expr(property_name: str, value, primitive_type: str, language: str) -> str:
              """A boolean expression testing `node`'s discriminator property against one value."""
              if language == 'php':
                  accessor = f"($this->node['{property_name}'] ?? null)"
                  if primitive_type == 'boolean':
                      return f'{accessor} === {('true' if value else 'false')}'
                  return f'{accessor} === {_literal(value, primitive_type, language)}'
              '# Uses JsonNode.path() (never MissingNode == null) so no separate absence guard is needed here.'
              accessor = f'node.path("{property_name}")'
              if primitive_type == 'string':
                  return f'{_java_string_literal(str(value))}.equals({accessor}.asText())'
              if primitive_type == 'integer':
                  return f'{accessor}.asLong() == {int(value)}L'
              if primitive_type == 'number':
                  return f'{accessor}.asDouble() == {float(value)}d'
              if primitive_type == 'boolean':
                  return f'{accessor}.asBoolean() == {('true' if value else 'false')}'
              raise ValueError(f'discriminator property has no supported primitive type: {primitive_type!r}')
      - id: structural_applies_expr
        type: FunctionDef
        code: |-
          def _structural_applies_expr(resolved, language: str) -> str:
              """presence of required fields / JSON type, no discriminator const available."""
              if language == 'php':
                  if isinstance(resolved, ObjectNode):
                      if not resolved.required:
                          return 'true'
                      return ' && '.join(
                          (f"array_key_exists('{field_name}', (array) $this->node)" for field_name in sorted(resolved.required)))
                  if isinstance(resolved, ListNode):
                      return 'is_array($this->node)'
                  if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
                      return 'is_array($this->node) || is_object($this->node)'
                  if isinstance(resolved, EnumNode):
                      return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'
                  if isinstance(resolved, PrimitiveNode):
                      if resolved.primitive_type == 'null':
                          return '$this->node === null'
                      return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'
                  return 'true'
              if isinstance(resolved, ObjectNode):
                  if not resolved.required:
                      return 'true'
                  return ' && '.join((f'node.has("{field_name}")' for field_name in sorted(resolved.required)))
              if isinstance(resolved, ListNode):
                  return 'node.isArray()'
              if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
                  return 'node.isObject()'
              if isinstance(resolved, EnumNode):
                  return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'
              if isinstance(resolved, PrimitiveNode):
                  if resolved.primitive_type == 'null':
                      return 'node.isNull()'
                  return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'
              '# nested composition or other structural node: best-effort, never validated further'
              return 'true'
```
/answer 8472-16 allow
Control Request:
```yaml
id: 8472-17
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    ids:
    - z0x58G|COEEvN
    - parameter_type
    - MethodParameter
    - build_parameter
    - java_string_literal
    - php_string_literal
    - id98Oo|sRRjxF
    - php_type_hint
    - signature
    - path_url_expression
    - method_name
    - ClientMethod
    - build_client_methods
    - top_path_segment
    - client_interface_name
  reason: Vollständigen Code client_context.py lesen für Extraktion
```
/answer 8472-17 allow
Control Request:
```yaml
id: 8472-18
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
      nodes:
      - id: z0x58G|COEEvN
        type: statements
        code: |-
          PARAMETER_JAVA_TYPE = {'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
          PARAMETER_PHP_TYPE = {'integer': 'int', 'number': 'float', 'boolean': 'bool'}
          PARAMETER_TYPE = {'java': PARAMETER_JAVA_TYPE, 'php': PARAMETER_PHP_TYPE}
          PARAMETER_DEFAULT_TYPE = {'java': 'String', 'php': 'string'}
          _PATH_PARAM = re.compile('\\{([^}]+)\\}')
      - id: parameter_type
        type: FunctionDef
        code: |-
          def _parameter_type(schema: dict | None, language: str) -> str:
              """Path/query parameters never enter the body tree; map their
              raw JSON-Schema type directly to a scalar type (string is the default)."""
              return PARAMETER_TYPE[language].get((schema or {}).get('type'), PARAMETER_DEFAULT_TYPE[language])
      - id: MethodParameter
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class MethodParameter:
              """One path/query parameter, or the synthetic body parameter."""
              '# Java identifier used in the method signature'
              name: str
              java_type: str
              '# original OpenAPI parameter name (the actual URL token/query key)'
              raw_name: str
              "# 'path' | 'query' | 'body'"
              kind: str
      - id: build_parameter
        type: FunctionDef
        code: |-
          def _build_parameter(param, kind: str, language: str) -> MethodParameter:
              return MethodParameter(
                  name=property_accessor_name(
                      param.name), java_type=_parameter_type(
                          param.schema, language), raw_name=param.name, kind=kind)
      - id: java_string_literal
        type: FunctionDef
        code: |-
          def _java_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace('"', '\\"')
              return f'"{escaped}"'
      - id: php_string_literal
        type: FunctionDef
        code: |-
          def _php_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace("'", "\\'")
              return f"'{escaped}'"
      - id: id98Oo|sRRjxF
        type: statements
        code: "_PHP_SCALAR_TYPES = {'int', 'float', 'string', 'bool', 'mixed'}"
      - id: php_type_hint
        type: FunctionDef
        code: |-
          def _php_type_hint(type_name: str) -> str:
              """A dotted fqn ('pkg.Class') becomes a fully-qualified PHP type hint; scalars pass through."""
              if type_name in _PHP_SCALAR_TYPES or '.' not in type_name:
                  return type_name
              return '\\' + type_name.replace('.', '\\')
      - id: signature
        type: FunctionDef
        code: |-
          def _signature(parameters: tuple, language: str) -> str:
              if language == 'php':
                  return ', '.join((f'{_php_type_hint(p.java_type)} ${p.name}' for p in parameters))
              return ', '.join((f'{p.java_type} {p.name}' for p in parameters))
      - id: path_url_expression
        type: FunctionDef
        code: |-
          def _path_url_expression(path: str, path_params: tuple, language: str) -> str:
              """A string-concatenation expression rebuilding the URL path, with
              every '{param}' token replaced by its URL-encoded argument value."""
              by_raw_name = {p.raw_name: p for p in path_params}
              if language == 'php':
                  parts, last = ([], 0)
                  for match in _PATH_PARAM.finditer(path):
                      literal = path[last:match.start()]
                      if literal:
                          parts.append(_php_string_literal(literal))
                      param = by_raw_name[match.group(1)]
                      parts.append(f'rawurlencode((string) ${param.name})')
                      last = match.end()
                  tail = path[last:]
                  if tail or not parts:
                      parts.append(_php_string_literal(tail))
                  return ' . '.join(parts)
              parts, last = ([], 0)
              for match in _PATH_PARAM.finditer(path):
                  literal = path[last:match.start()]
                  if literal:
                      parts.append(_java_string_literal(literal))
                  param = by_raw_name[match.group(1)]
                  parts.append(
                      f'java.net.URLEncoder.encode(String.valueOf({
                          param.name}), java.nio.charset.StandardCharsets.UTF_8)')
                  last = match.end()
              tail = path[last:]
              if tail or not parts:
                  parts.append(_java_string_literal(tail))
              return ' + '.join(parts)
      - id: method_name
        type: FunctionDef
        code: |-
          def _method_name(operation) -> str:
              """operationId wins verbatim (D-decision); otherwise '<method><Path>' (e.g. postResponses)."""
              if operation.operation_id:
                  return sanitize_identifier(operation.operation_id)
              return to_camel_case(f'{operation.method}{path_to_class_fragment(operation.path)}')
      - id: ClientMethod
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ClientMethod:
              """One operation's client-facing method: interface signature plus everything
              the implementation needs to build and send the HTTP request."""
              name: str
              "# upper-case, e.g. 'POST'"
              http_method: str
              path: str
              "# ready-made Java expression for the request URL's path part"
              path_url_expression: str
              '# MethodParameter, in path-declaration order'
              path_params: tuple
              '# MethodParameter'
              query_params: tuple
              '# None if the operation has no request body'
              body_param: MethodParameter | None
              '# path_params + query_params + ([body_param]), for the method signature'
              parameters: tuple
              '# precomputed "Type name, Type name, ..." parameter list'
              signature: str
              '# FQN of the already-generated ResponseNode root class'
              response_type: str
              description: str | None
              example_repr: str | None
      - id: build_client_methods
        type: FunctionDef
        code: |-
          def build_client_methods(named_model) -> list[ClientMethod]:
              """One ClientMethod per operation, in deterministic (path, method) order."""
              methods = []
              used_names: dict[str, int] = {}
              ordered = sorted(named_model.operations, key=lambda om: (om.operation.path, om.operation.method))
              for operation_model in ordered:
                  operation = operation_model.operation
                  name = _method_name(operation)
                  "# Defensive collision guard (structurally shouldn't happen: distinct"
                  '# (path, method) pairs only collide in name if operationIds collide,'
                  '# which is itself an invalid OpenAPI document) -- numbered by the same'
                  '# deterministic (path, method) order used above, never discovery order.'
                  used_names[name] = used_names.get(name, 0) + 1
                  final_name = name if used_names[name] == 1 else f'{name}{used_names[name]}'
                  language = named_model.language
                  path_params = tuple((_build_parameter(p, 'path', language)
                                      for p in operation.parameters if p.location == 'path'))
                  query_params = tuple((_build_parameter(p, 'query', language)
                                       for p in operation.parameters if p.location == 'query'))
                  body_node = request_root_node(operation_model, named_model.named_nodes)
                  body_param = MethodParameter(
                      name='request',
                      java_type=named_model.name_of(body_node).fqn,
                      raw_name='request',
                      kind='body') if body_node is not None else None
                  request_edge = operation_model.request.body if operation_model.request is not None else None
                  example = None if request_edge is None or request_edge.example is MISSING else repr(request_edge.example)
                  parameters = path_params + query_params + ((body_param,) if body_param is not None else ())
                  methods.append(
                      ClientMethod(
                          name=final_name,
                          http_method=operation.method.upper(),
                          path=operation.path,
                          path_url_expression=_path_url_expression(
                              operation.path,
                              path_params,
                              language),
                          path_params=path_params,
                          query_params=query_params,
                          body_param=body_param,
                          parameters=parameters,
                          signature=_signature(
                              parameters,
                              language),
                          response_type=named_model.name_of(
                              operation_model.response).fqn,
                          description=operation.description,
                          example_repr=example))
              return methods
      - id: top_path_segment
        type: FunctionDef
        code: |-
          def _top_path_segment(path: str) -> str:
              for part in (path or '').split('/'):
                  part = part.strip().strip('{}')
                  if part:
                      return part
              return 'api'
      - id: client_interface_name
        type: FunctionDef
        code: |-
          def client_interface_name(named_model) -> str:
              """Deterministic name from the sorted set of top-level path segments
              (single path '/responses' -> 'ResponsesClient', per acceptance)."""
              top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
              fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
              return f'{fragment}Client'
```
/answer 8472-18 allow
Control Request:
```yaml
id: 8472-19
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    ids:
    - request_root_node
    - request_root_node_ids
    - json_support_fqn
    - ContentTypeBranch
    - build_content_type_branches
    - CodeBranch
    - build_code_branches
    - W0p47r|wrwzO7
    - ResponseSetter
    - build_response_setters
  reason: io_context.py vollständig lesen
```
/answer 8472-19 allow
Control Request:
```yaml
id: 8472-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
      nodes:
      - id: request_root_node
        type: FunctionDef
        code: |-
          def request_root_node(operation_model, named_nodes):
              """The structural node whose generated class becomes this operation's request root.

              A $ref body IS the request root: the already-generated class for that
              named schema gets serialization added, no wrapper class. An inline body's
              own node gets it directly for the same reason -- there is exactly one
              class per request body, never a duplicate.
              """
              request_node = operation_model.request
              if request_node is None or request_node.body is None:
                  return None
              target = request_node.body.target
              return named_nodes[target.name] if isinstance(target, RefNode) else target
      - id: request_root_node_ids
        type: FunctionDef
        code: |-
          def request_root_node_ids(model) -> set:
              """id() of every node that must render toString()/fromString()."""
              ids = set()
              for operation_model in model.operations:
                  node = request_root_node(operation_model, model.named_nodes)
                  if node is not None:
                      ids.add(id(node))
              return ids
      - id: json_support_fqn
        type: FunctionDef
        code: |-
          def json_support_fqn(base_package: str) -> str:
              return f'{base_package}.JsonSupport'
      - id: ContentTypeBranch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ContentTypeBranch:
              """One content-type view of a CodeNode: is<Ct>()/get<Ct>(), discriminated by header."""
              short_name: str
              content_type: str
              java_type: str
              category: str
              read_method: str | None
              description: str | None
      - id: build_content_type_branches
        type: FunctionDef
        code: |-
          def build_content_type_branches(code_node, named_model) -> list:
              """One branch per ContentTypeView, in declaration order."""
              branches = []
              for content_type_view in code_node.content_types:
                  edge = content_type_view.body
                  category, primitive_type = classify(edge.target, named_model.named_nodes)
                  if category == 'unsupported':
                      continue
                  branches.append(ContentTypeBranch(short_name=to_pascal_case(content_type_short_name(content_type_view.content_type)),
                                                    content_type=content_type_view.content_type,
                                                    java_type=map_type(edge.target,
                                                                       named_model),
                                                    category=category,
                                                    read_method=READ_METHOD[named_model.language].get(primitive_type) if category in ('primitive',
                                                                                                                                      'enum') else None,
                                                    description=edge.description))
              return branches
      - id: CodeBranch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CodeBranch:
              """One status-code view of a ResponseNode: getCode<code>() -- null unless it matches."""
              status_code: str
              java_type: str
      - id: build_code_branches
        type: FunctionDef
        code: |-
          def build_code_branches(response_node, named_model) -> list:
              return [CodeBranch(status_code=code_node.status_code, java_type=named_model.name_of(code_node).fqn)
                      for code_node in response_node.codes]
      - id: W0p47r|wrwzO7
        type: statements
        code: |-
          '# read_method -> JsonNodeFactory factory-method name, for constructing a raw'
          '# JsonNode from a primitive Java value (setCode<NNN>... on the server side).'
          PRIMITIVE_NODE_FACTORY = {
              'asText': 'textNode',
              'asLong': 'numberNode',
              'asDouble': 'numberNode',
              'asBoolean': 'booleanNode'}
      - id: ResponseSetter
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ResponseSetter:
              """One setCode<NNN>[<ContentTypeSuffix>](...) constructor-style setter on a
              ResponseNode: status code and content type are implied by the spec, the
              server implementer only supplies the already-typed body value."""
              status_code: str
              method_name: str
              content_type: str
              java_type: str
              category: str
              node_factory_method: str | None
      - id: build_response_setters
        type: FunctionDef
        code: |-
          def build_response_setters(response_node, named_model) -> list:
              """One ResponseSetter per (status code, content type) combination. The
              content-type suffix is only appended when a status code has more than one
              content type (declaration order, same as build_content_type_branches)."""
              setters = []
              for code_node in response_node.codes:
                  branches = build_content_type_branches(code_node, named_model)
                  multiple = len(branches) > 1
                  for branch in branches:
                      suffix = branch.short_name if multiple else ''
                      setters.append(
                          ResponseSetter(
                              status_code=code_node.status_code,
                              method_name=f'setCode{
                                  code_node.status_code}{suffix}',
                              content_type=branch.content_type,
                              java_type=branch.java_type,
                              category=branch.category,
                              node_factory_method=PRIMITIVE_NODE_FACTORY.get(
                                      branch.read_method) if branch.category == 'primitive' else None))
              return setters
```
/answer 8472-1a allow
Control Request:
```yaml
id: 8472-1b
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_emit.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_emit.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/writer.py
  reason: Restliche emit-Dateien überblicken
```
/answer 8472-1b allow
Control Request:
```yaml
id: 8472-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_emit.py
      nodes:
      - id: 6E70LY|5COmUX
        signature: "\"Renders request/response (de)serialization code. Depends on model, not on clie…"
      - id: RCYAIl|9cf6VQ
        signature: from pathlib import Path
      - id: emit_io
        signature: "def emit_io(model, writer) -> None:"
        docstring: Render JsonSupport plus one class per operation's ResponseNode and CodeNode.
      - id: write
        signature: "def _write(name, content: str, writer, language: str) -> None:"
      - id: emit_json_support
        signature: "def _emit_json_support(model, writer) -> None:"
      - id: emit_response
        signature: "def _emit_response(response_node, model, writer) -> None:"
      - id: emit_code
        signature: "def _emit_code(code_node, model, writer) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: Ddm4e1|INaxgR
        signature: '''Builds the Jinja2 template context for the transport-agnostic abstract server …'
      - id: iSfUpq|kwHWOM
        signature: import re
      - id: mm8SYx|RWeTpl
        signature: "_PATH_PARAM = re.compile('\\\\{([^}]+)\\\\}')"
      - id: QLdKL0|JYEPq1
        signature: "_PHP_PARSE_EXPR = {"
      - id: ServerMethod
        signature: "@dataclass(frozen=True) class ServerMethod:"
        docstring: One operation's abstract server method plus everything the generated `handle(Ht…
      - id: path_regex
        signature: "def _path_regex(path: str, path_params: tuple) -> tuple[str, tuple]:"
        docstring: Regex pattern string plus the path parameters in regex-group (path-appearance) …
      - id: binding_lines
        signature: "def _binding_lines(client_method, regex_order: tuple, body_json_support_fqn: str, language: str) -> tuple:"
      - id: binding_lines_php
        signature: "def _binding_lines_php(client_method, regex_order: tuple) -> tuple:"
      - id: build_server_methods
        signature: "def build_server_methods(named_model) -> list[ServerMethod]:"
        docstring: "One ServerMethod per operation, same order as build_client_methods."
      - id: server_class_name
        signature: "def server_class_name(named_model) -> str:"
        docstring: Deterministic name mirroring client_interface_name (e.g. 'ResponsesServer').
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_emit.py
      nodes:
      - id: PXr7Jz|YbjyPI
        signature: "\"Renders the abstract, transport-agnostic server base class. Depends on model, …"
      - id: MBhQzu|RXOK6C
        signature: from pathlib import Path
      - id: emit_server
        signature: "def emit_server(model, writer) -> None:"
        docstring: "Render the abstract server base class, one method per operation."
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
      nodes:
      - id: IbJdkk|3S3BzR
        signature: "'Language-aware Jinja2 environment factory: templates/<language> is the templat…"
      - id: JiTbQ2|nmyzQK
        signature: from functools import lru_cache
      - id: sd2GCB|4DLbTY
        signature: TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'
      - id: file_extension
        signature: "def file_extension(language: str) -> str:"
      - id: to_php_fqn
        signature: "def _to_php_fqn(fqn: str) -> str:"
        docstring: Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\pkg\sub\Class').
      - id: to_namespace
        signature: "def _to_namespace(package: str) -> str:"
      - id: get_env
        signature: "@lru_cache(maxsize=None) def get_env(language: str) -> Environment:"
        docstring: "One cached Jinja2 Environment per language, rooted at templates/<language>."
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/writer.py
      nodes:
      - id: vzdWzF|Nao0Cp
        signature: '''Writes rendered source text to files under the output directory.'''
      - id: yOtzCe|wkrevi
        signature: from pathlib import Path
      - id: FileWriter
        signature: "class FileWriter:"
        docstring: Writes generated file contents to disk under a base output directory.
        children:
        - id: FileWriter.DV3nHZ|NTfHiB
          signature: '''Writes generated file contents to disk under a base output directory.'''
        - id: FileWriter.init
          signature: "def __init__(self, output_dir: Path):"
        - id: FileWriter.write
          signature: "def write(self, relative_path: Path, content: str) -> None:"
          docstring: "Write content to output_dir/relative_path, creating parent directories."
```
/answer 8472-1c allow
ReasoningToken: 120
Text:
 Jetzt die restlichen Dateien: io_context.py, model_emit.py, client_emit.py, io_emit.py, server_context.py, server_emit.py, templates.py, writer.py.
Control Request:
```yaml
id: 8472-1d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_emit.py
    ids:
    - emit_io
    - write
    - emit_json_support
    - emit_response
    - emit_code
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    ids:
    - QLdKL0|JYEPq1
    - ServerMethod
    - path_regex
    - binding_lines
    - binding_lines_php
    - build_server_methods
    - server_class_name
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_emit.py
    ids:
    - emit_server
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    ids:
    - file_extension
    - to_php_fqn
    - to_namespace
    - get_env
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py
    ids:
    - emit_model
    - render
    - render_object
    - render_list
    - render_dictionary
    - render_enum
    - render_any_dictionary
    - render_composition
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_emit.py
    ids:
    - emit_client
  reason: Vollständigen Code weiterer Dateien lesen für Extraktion
```
/answer 8472-1d allow es würde auch helfen mal mit grep nach "php" und "java" zu suchen
Control Request:
```yaml
id: 8472-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_emit.py
      nodes:
      - id: emit_io
        type: FunctionDef
        code: |-
          def emit_io(model, writer) -> None:
              """Render JsonSupport plus one class per operation's ResponseNode and CodeNode."""
              _emit_json_support(model, writer)
              for operation_model in model.operations:
                  _emit_response(operation_model.response, model, writer)
                  for code_node in operation_model.response.codes:
                      _emit_code(code_node, model, writer)
      - id: write
        type: FunctionDef
        code: |-
          def _write(name, content: str, writer, language: str) -> None:
              relative_path = Path(*name.package.split('.')) / f'{name.class_name}.{file_extension(language)}'
              writer.write(relative_path, content)
      - id: emit_json_support
        type: FunctionDef
        code: |-
          def _emit_json_support(model, writer) -> None:
              package = model.base_package
              template = get_env(model.language).get_template('io/json_support.jinja')
              content = template.render(package=package)
              writer.write(Path(*package.split('.')) / f'JsonSupport.{file_extension(model.language)}', content)
      - id: emit_response
        type: FunctionDef
        code: |-
          def _emit_response(response_node, model, writer) -> None:
              name = model.name_of(response_node)
              codes = build_code_branches(response_node, model)
              setters = build_response_setters(response_node, model)
              template = get_env(model.language).get_template('io/response.jinja')
              content = template.render(
                  package=name.package,
                  class_name=name.class_name,
                  codes=codes,
                  setters=setters,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
              _write(name, content, writer, model.language)
      - id: emit_code
        type: FunctionDef
        code: |-
          def _emit_code(code_node, model, writer) -> None:
              name = model.name_of(code_node)
              content_types = build_content_type_branches(code_node, model)
              template = get_env(model.language).get_template('io/code.jinja')
              content = template.render(package=name.package, class_name=name.class_name, content_types=content_types)
              _write(name, content, writer, model.language)
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: QLdKL0|JYEPq1
        type: statements
        code: |-
          _PHP_PARSE_EXPR = {
              'int': '(int) {raw}',
              'float': '(float) {raw}',
              'bool': 'filter_var({raw}, FILTER_VALIDATE_BOOLEAN)',
              'string': '{raw}'}
          _PARSE_EXPR = {'java': _JAVA_PARSE_EXPR, 'php': _PHP_PARSE_EXPR}
      - id: ServerMethod
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ServerMethod:
              """One operation's abstract server method plus everything the generated
              `handle(HttpExchange)` dispatcher needs to route to it."""
              name: str
              "# upper-case, e.g. 'POST'"
              http_method: str
              response_type: str
              "# identical to the client method's signature (D-verbindlich)"
              signature: str
              description: str | None
              example_repr: str | None
              "# unique Java identifier for this operation's compiled path Pattern"
              pattern_name: str
              '# regex, one capturing group per path parameter, in path-appearance order'
              pattern_regex: str
              '# Java statements decoding matcher groups / query params / body into locals'
              binding_lines: tuple
              '# ready-made "a, b, c" argument list for calling the abstract method'
              call_args: str
              has_body: bool
      - id: path_regex
        type: FunctionDef
        code: |-
          def _path_regex(path: str, path_params: tuple) -> tuple[str, tuple]:
              """Regex pattern string plus the path parameters in regex-group
              (path-appearance) order, which need not match declaration order."""
              by_raw_name = {p.raw_name: p for p in path_params}
              parts, order, last = ([], [], 0)
              for match in _PATH_PARAM.finditer(path):
                  parts.append(re.escape(path[last:match.start()]))
                  parts.append('([^/]+)')
                  order.append(by_raw_name[match.group(1)])
                  last = match.end()
              parts.append(re.escape(path[last:]))
              return (''.join(parts), tuple(order))
      - id: binding_lines
        type: FunctionDef
        code: |-
          def _binding_lines(client_method, regex_order: tuple, body_json_support_fqn: str, language: str) -> tuple:
              if language == 'php':
                  return _binding_lines_php(client_method, regex_order)
              lines = []
              for i, param in enumerate(regex_order, start=1):
                  raw = f'URLDecoder.decode(matcher.group({i}), StandardCharsets.UTF_8)'
                  lines.append(f'{param.java_type} {param.name} = {_PARSE_EXPR['java'][param.java_type].format(raw=raw)};')
              for param in client_method.query_params:
                  raw_var = f'{param.name}Raw'
                  lines.append(f'String {raw_var} = queryParams.get("{param.raw_name}");')
                  lines.append(
                      f'{param.java_type} {param.name} = {raw_var} == null ? null : {_PARSE_EXPR['java'][param.java_type].format(raw=raw_var)};')
              if client_method.body_param is not None:
                  lines.append('String rawBody = readBody(exchange);')
                  lines.append(f'{client_method.body_param.java_type} {client_method.body_param.name} = new {client_method.body_param.java_type}({body_json_support_fqn}.parse(rawBody));')
              return tuple(lines)
      - id: binding_lines_php
        type: FunctionDef
        code: |-
          def _binding_lines_php(client_method, regex_order: tuple) -> tuple:
              lines = []
              for i, param in enumerate(regex_order, start=1):
                  raw = f'urldecode($matches[{i}])'
                  lines.append(f'${param.name} = {_PARSE_EXPR['php'][param.java_type].format(raw=raw)};')
              for param in client_method.query_params:
                  raw_var = f'${param.name}Raw'
                  lines.append(f"{raw_var} = $queryParams['{param.raw_name}'] ?? null;")
                  lines.append(
                      f'${param.name} = {raw_var} === null ? null : {_PARSE_EXPR['php'][param.java_type].format(raw=raw_var)};')
              if client_method.body_param is not None:
                  lines.append("$rawBody = file_get_contents('php://input');")
                  lines.append(
                      f'${
                          client_method.body_param.name} = new {
                              _php_type_hint(
                                  client_method.body_param.java_type)}(json_decode($rawBody, true));')
              return tuple(lines)
      - id: build_server_methods
        type: FunctionDef
        code: |-
          def build_server_methods(named_model) -> list[ServerMethod]:
              """One ServerMethod per operation, same order as build_client_methods."""
              support_fqn = json_support_fqn(named_model.base_package)
              server_methods = []
              for client_method in build_client_methods(named_model):
                  pattern_regex, regex_order = _path_regex(client_method.path, client_method.path_params)
                  binding_lines = _binding_lines(client_method, regex_order, support_fqn, named_model.language)
                  call_args = ', '.join((f'${p.name}' if named_model.language ==
                                        'php' else p.name for p in client_method.parameters))
                  server_methods.append(
                      ServerMethod(
                          name=client_method.name,
                          http_method=client_method.http_method,
                          response_type=client_method.response_type,
                          signature=client_method.signature,
                          description=client_method.description,
                          example_repr=client_method.example_repr,
                          pattern_name=f'PATTERN_{
                              client_method.name.upper()}',
                          pattern_regex=pattern_regex,
                          binding_lines=binding_lines,
                          call_args=call_args,
                          has_body=client_method.body_param is not None))
              return server_methods
      - id: server_class_name
        type: FunctionDef
        code: |-
          def server_class_name(named_model) -> str:
              """Deterministic name mirroring client_interface_name (e.g. 'ResponsesServer')."""
              top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
              fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
              return f'{fragment}Server'
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_emit.py
      nodes:
      - id: emit_server
        type: FunctionDef
        code: |-
          def emit_server(model, writer) -> None:
              """Render the abstract server base class, one method per operation."""
              if not model.operations:
                  return
              package = model.base_package
              class_name = server_class_name(model)
              methods = build_server_methods(model)
              content = get_env(model.language).get_template(
                  'server/abstract_server.jinja').render(package=package, class_name=class_name, methods=methods)
              writer.write(Path(*package.split('.')) / f'{class_name}.{file_extension(model.language)}', content)
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
      nodes:
      - id: file_extension
        type: FunctionDef
        code: |-
          def file_extension(language: str) -> str:
              return EXTENSION[language]
      - id: to_php_fqn
        type: FunctionDef
        code: |-
          def _to_php_fqn(fqn: str) -> str:
              """Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\\pkg\\sub\\Class')."""
              return '\\' + fqn.replace('.', '\\')
      - id: to_namespace
        type: FunctionDef
        code: |-
          def _to_namespace(package: str) -> str:
              return package.replace('.', '\\')
      - id: get_env
        type: FunctionDef
        code: |-
          @lru_cache(maxsize=None)
          def get_env(language: str) -> Environment:
              """One cached Jinja2 Environment per language, rooted at templates/<language>."""
              env = Environment(loader=FileSystemLoader(str(TEMPLATES_ROOT / language)), trim_blocks=True,
                                lstrip_blocks=True, keep_trailing_newline=True, undefined=StrictUndefined)
              if language == 'php':
                  env.filters['phpfqn'] = _to_php_fqn
                  env.filters['phpns'] = _to_namespace
              return env
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py
      nodes:
      - id: emit_model
        type: FunctionDef
        code: |-
          def emit_model(model, writer) -> None:
              """Render and write one .java file per reachable Object/List/Dictionary/Enum/AnyDictionary node."""
              request_root_ids = request_root_node_ids(model)
              for node in _reachable_nodes(model):
                  if not isinstance(node, _MODEL_CLASS_KINDS):
                      continue
                  name = model.name_of(node)
                  '# not a class (e.g. structurally unreachable, defensive)'
                  if name is None:
                      continue
                  content = _render(node, name, model, id(node) in request_root_ids)
                  relative_path = Path(*name.package.split('.')) / f'{name.class_name}.{file_extension(model.language)}'
                  writer.write(relative_path, content)
      - id: render
        type: FunctionDef
        code: |-
          def _render(node, name, model, is_request_root: bool) -> str:
              if isinstance(node, ObjectNode):
                  return _render_object(node, name, model, is_request_root)
              if isinstance(node, ListNode):
                  return _render_list(node, name, model, is_request_root)
              if isinstance(node, DictionaryNode):
                  return _render_dictionary(node, name, model, is_request_root)
              if isinstance(node, EnumNode):
                  '# A bare top-level enum request body is not supported as a request root:'
                  '# a Java enum cannot hold the bound JsonNode a toString()/fromString()'
                  '# pair would need.'
                  return _render_enum(node, name, model.language)
              if isinstance(node, AnyDictionaryNode):
                  return _render_any_dictionary(name, model, is_request_root)
              if isinstance(node, CompositionNode):
                  return _render_composition(node, name, model, is_request_root)
              '# pragma: no cover -- guarded by _MODEL_CLASS_KINDS'
              raise TypeError(f'no template for node kind: {node.kind!r}')
      - id: render_object
        type: FunctionDef
        code: |-
          def _render_object(node: ObjectNode, name, model, is_request_root: bool) -> str:
              accessors = [a for a in (build_accessor(edge.label, edge, model) for edge in node.properties) if a is not None]
              template = get_env(model.language).get_template('model/object.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  accessors=accessors,
                  is_request_root=is_request_root,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
      - id: render_list
        type: FunctionDef
        code: |-
          def _render_list(node: ListNode, name, model, is_request_root: bool) -> str:
              if node.mixed:
                  elements = [a for a in (build_accessor(edge.label, edge, model) for edge in node.elements) if a is not None]
                  template = get_env(model.language).get_template('model/list_mixed.jinja')
                  return template.render(
                      package=name.package,
                      class_name=name.class_name,
                      elements=elements,
                      is_request_root=is_request_root,
                      json_support_fqn=json_support_fqn(
                          model.base_package))
              element_edge, = node.elements
              element = build_accessor(element_edge.label, element_edge, model)
              template = get_env(model.language).get_template('model/list.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  element=element,
                  is_request_root=is_request_root,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
      - id: render_dictionary
        type: FunctionDef
        code: |-
          def _render_dictionary(node: DictionaryNode, name, model, is_request_root: bool) -> str:
              value = build_accessor(node.value.label, node.value, model)
              template = get_env(model.language).get_template('model/dictionary.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  value=value,
                  is_request_root=is_request_root,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
      - id: render_enum
        type: FunctionDef
        code: |-
          def _render_enum(node: EnumNode, name, language: str) -> str:
              template = get_env(language).get_template('model/enum.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  primitive_type=node.primitive_type,
                  raw_type=enum_raw_type(
                      node,
                      language),
                  constants=enum_constants(
                      node,
                      language))
      - id: render_any_dictionary
        type: FunctionDef
        code: |-
          def _render_any_dictionary(name, model, is_request_root: bool) -> str:
              template = get_env(model.language).get_template('model/any_dictionary.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  is_request_root=is_request_root,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
      - id: render_composition
        type: FunctionDef
        code: |-
          def _render_composition(node: CompositionNode, name, model, is_request_root: bool) -> str:
              branches = build_branches(node, model)
              template = get_env(model.language).get_template('model/composition.jinja')
              return template.render(
                  package=name.package,
                  class_name=name.class_name,
                  keyword=node.keyword,
                  branches=branches,
                  is_request_root=is_request_root,
                  json_support_fqn=json_support_fqn(
                      model.base_package))
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_emit.py
      nodes:
      - id: emit_client
        type: FunctionDef
        code: |-
          def emit_client(model, writer) -> None:
              """Render the client interface and its HttpClient-based implementation."""
              if not model.operations:
                  return
              package = model.base_package
              interface_name = client_interface_name(model)
              impl_name = f'{interface_name}Impl'
              interface_fqn = f'{package}.{interface_name}'
              methods = build_client_methods(model)
              env = get_env(model.language)
              ext = file_extension(model.language)
              interface_content = env.get_template('client/interface.jinja').render(package=package,
                                                                                    class_name=interface_name, methods=methods)
              writer.write(Path(*package.split('.')) / f'{interface_name}.{ext}', interface_content)
              impl_content = env.get_template('client/impl.jinja').render(package=package,
                                                                          class_name=impl_name, interface_fqn=interface_fqn, methods=methods)
              writer.write(Path(*package.split('.')) / f'{impl_name}.{ext}', impl_content)
```
/answer 8472-1e allow
Text:
 Jetzt lese ich die restlichen relevanten Dateien vollständig: io_emit.py, server_context.py, server_emit.py, templates.py, model_emit.py, client_emit.py.
Control Request:
```yaml
id: 8472-1f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src
    pattern: "'php'|'java'|php_|java_|JAVA_|PHP_|language ==|language:"
    include:
    - '*.py'
    exclude: []
    limit: 50
  reason: Alle Sprachspezifika im gesamten src-Baum finden
```
/answer 8472-1f allow
Control Request:
```yaml
id: 8472-20
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src
      matches:
      - path: xy/cgen/naming/__init__.py
        matches:
        - lineno: 48
          match: "language: str"
        - lineno: 68
          match: "def assign_names(identified_model, base_package: str, language: str='java') -> NamedModel:"
      - path: xy/cgen/naming/identifiers.py
        matches:
        - lineno: 5
          match: JAVA_KEYWORDS = frozenset(
        - lineno: 31
          match: "if cleaned in JAVA_KEYWORDS:"
      - path: xy/cgen/cli.py
        matches:
        - lineno: 17
          match: "parser.add_argument('--language', choices=('java', 'php'), default='java',"
      - path: xy/cgen/typemap/__init__.py
        matches:
        - lineno: 11
          match: "JAVA_PRIMITIVE_TYPE = {'string': 'String', 'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}"
        - lineno: 12
          match: "PHP_PRIMITIVE_TYPE = {'string': 'string', 'integer': 'int', 'number': 'float', 'boolean': 'bool'}"
        - lineno: 13
          match: "PRIMITIVE_TYPE = {'java': JAVA_PRIMITIVE_TYPE, 'php': PHP_PRIMITIVE_TYPE}"
        - lineno: 14
          match: ANY_DICTIONARY_JAVA_TYPE = 'com.fasterxml.jackson.databind.JsonNode'
        - lineno: 15
          match: "ANY_DICTIONARY_TYPE = {'java': ANY_DICTIONARY_JAVA_TYPE, 'php': 'mixed'}"
        - lineno: 39
          match: "def _map_primitive(node: PrimitiveNode, language: str) -> str:"
      - path: xy/cgen/emit/io_context.py
        matches:
        - lineno: 46
          match: "java_type: str"
        - lineno: 61
          match: "java_type=map_type(edge.target,"
        - lineno: 73
          match: "java_type: str"
        - lineno: 76
          match: "return [CodeBranch(status_code=code_node.status_code, java_type=named_model.name_of(code_node).fqn)"
        - lineno: 94
          match: "java_type: str"
        - lineno: 114
          match: "java_type=branch.java_type,"
      - path: xy/cgen/emit/model_context.py
        matches:
        - lineno: 26
          match: "JAVA_PRIMITIVE_READ_METHOD = {'string': 'asText', 'integer': 'asLong', 'number': 'asDouble', 'boolean': 'asBoolean'}"
        - lineno: 29
          match: "JAVA_PRIMITIVE_FACTORY_METHOD = {"
        - lineno: 35
          match: "PHP_PRIMITIVE_CAST = PRIMITIVE_TYPE['php']"
        - lineno: 36
          match: "READ_METHOD = {'java': JAVA_PRIMITIVE_READ_METHOD, 'php': PHP_PRIMITIVE_CAST}"
        - lineno: 37
          match: "FACTORY_METHOD = {'java': JAVA_PRIMITIVE_FACTORY_METHOD, 'php': {}}"
        - lineno: 45
          match: "java_type: str"
        - lineno: 98
          match: "property_accessor_name(label)), java_type=map_type("
        - lineno: 111
          match: "def enum_constants(node, language: str='java') -> list[EnumConstant]:"
        - lineno: 129
          match: "def _literal(value, primitive_type: str, language: str) -> str:"
        - lineno: 132
          match: "if language == 'php':"
        - lineno: 134
          match: return _php_string_literal(str(value))
        - lineno: 141
          match: return _java_string_literal(str(value))
        - lineno: 148
          match: "def _java_string_literal(text: str) -> str:"
        - lineno: 152
          match: "def _php_string_literal(text: str) -> str:"
        - lineno: 156
          match: "def enum_raw_type(node, language: str='java') -> str:"
        - lineno: 172
          match: "_PHP_TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}"
        - lineno: 179
          match: "java_type: str | None"
        - lineno: 230
          match: "java_type=None,"
        - lineno: 242
          match: java_type=map_type(
        - lineno: 306
          match: "def _discriminator_literal_expr(property_name: str, value, primitive_type: str, language: str) -> str:"
        - lineno: 308
          match: "if language == 'php':"
        - lineno: 316
          match: "return f'{_java_string_literal(str(value))}.equals({accessor}.asText())'"
        - lineno: 325
          match: "def _structural_applies_expr(resolved, language: str) -> str:"
        - lineno: 327
          match: "if language == 'php':"
        - lineno: 338
          match: "return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'"
        - lineno: 342
          match: "return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'"
      - path: xy/cgen/emit/server_context.py
        matches:
        - lineno: 12
          match: "from xy.cgen.emit.client_context import MethodParameter, _php_type_hint, _top_path_segment, build_client_methods"
        - lineno: 17
          match: "_JAVA_PARSE_EXPR = {"
        - lineno: 23
          match: "_PHP_PARSE_EXPR = {"
        - lineno: 28
          match: "_PARSE_EXPR = {'java': _JAVA_PARSE_EXPR, 'php': _PHP_PARSE_EXPR}"
        - lineno: 65
          match: "def _binding_lines(client_method, regex_order: tuple, body_json_support_fqn: str, language: str) -> tuple:"
        - lineno: 66
          match: "if language == 'php':"
        - lineno: 71
          match: "lines.append(f'{param.java_type} {param.name} = {_PARSE_EXPR['java'][param.java_type].format(raw=raw)};')"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 8472-20 allow
Control Request:
```yaml
id: 8472-21
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    ids:
    - iSfUpq|kwHWOM
    - mm8SYx|RWeTpl
  reason: "Fehlenden Teil server_context.py (Imports, JAVA_PARSE_EXPR) lesen"
```
/answer 8472-21 allow
Control Request:
```yaml
id: 8472-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: iSfUpq|kwHWOM
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from xy.cgen.emit.client_context import MethodParameter, _php_type_hint, _top_path_segment, build_client_methods
          from xy.cgen.emit.io_context import json_support_fqn
          from xy.cgen.naming.identifiers import class_identifier
      - id: mm8SYx|RWeTpl
        type: statements
        code: |-
          _PATH_PARAM = re.compile('\\{([^}]+)\\}')
          "# Java expression parsing a decoded String into the parameter's scalar type."
          _JAVA_PARSE_EXPR = {
              'Long': 'Long.valueOf({raw})',
              'Double': 'Double.valueOf({raw})',
              'Boolean': 'Boolean.valueOf({raw})',
              'String': '{raw}'}
          '# PHP expression parsing a decoded string into the parameter scalar type.'
```
/answer 8472-22 allow
Control Request:
```yaml
id: 8472-23
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
    ids: []
  reason: identifiers.py vollständig lesen (JAVA_KEYWORDS Nutzung)
```
/answer 8472-23 allow Auch alle if bedingungen müssen über Strukture im lang Package abgebildert werden. Zurückbleiben soll nur das superset und die generische Pipeline. Die für Sprachen notwendigen extras sollen auf das Minimum reduziert werden.
Control Request:
```yaml
id: 8472-24
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
      nodes:
      - id: qPtH7C|mXbLbD
        signature: "'Java identifier sanitizing: keywords, leading underscores, case conversion.'"
      - id: jvTnci|EeCI9v
        signature: import re
      - id: tPc1sR|geI8WZ
        signature: JAVA_KEYWORDS = frozenset('\n    abstract continue for new switch assert defaul…
      - id: y29IGb|KzfFPd
        signature: "_NON_IDENTIFIER_CHARS = re.compile('[^A-Za-z0-9_]+')"
      - id: sanitize_identifier
        signature: "def sanitize_identifier(raw: str) -> str:"
        docstring: "Turn an arbitrary string into a valid, non-keyword Java identifier. Strips ille…"
      - id: to_pascal_case
        signature: "def to_pascal_case(raw: str) -> str:"
        docstring: Split on non-alphanumeric boundaries and title-case each word.
      - id: to_camel_case
        signature: "def to_camel_case(raw: str) -> str:"
        docstring: Same as to_pascal_case but the first letter is lower-case.
      - id: property_accessor_name
        signature: "def property_accessor_name(label: str) -> str:"
        docstring: JSON field label -> Java accessor-name fragment (snake_case -> camelCase).
      - id: class_identifier
        signature: "def class_identifier(raw: str) -> str:"
        docstring: "Class-name-safe identifier: PascalCase, then sanitized/keyword-escaped."
      - id: content_type_short_name
        signature: "def content_type_short_name(content_type: str) -> str:"
        docstring: "'application/json' -> 'json': the subtype is enough within our JSON-only scope."
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 8472-24 allow
Control Request:
```yaml
id: 8472-25
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/__init__.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/cli.py
  reason: Struktur weiterer Module prüfen
```
/answer 8472-25 allow
Control Request:
```yaml
id: 8472-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
      nodes:
      - id: qPtH7C|mXbLbD
        signature: "'Java identifier sanitizing: keywords, leading underscores, case conversion.'"
      - id: jvTnci|EeCI9v
        signature: import re
      - id: tPc1sR|geI8WZ
        signature: JAVA_KEYWORDS = frozenset('\n    abstract continue for new switch assert defaul…
      - id: y29IGb|KzfFPd
        signature: "_NON_IDENTIFIER_CHARS = re.compile('[^A-Za-z0-9_]+')"
      - id: sanitize_identifier
        signature: "def sanitize_identifier(raw: str) -> str:"
        docstring: "Turn an arbitrary string into a valid, non-keyword Java identifier. Strips ille…"
      - id: to_pascal_case
        signature: "def to_pascal_case(raw: str) -> str:"
        docstring: Split on non-alphanumeric boundaries and title-case each word.
      - id: to_camel_case
        signature: "def to_camel_case(raw: str) -> str:"
        docstring: Same as to_pascal_case but the first letter is lower-case.
      - id: property_accessor_name
        signature: "def property_accessor_name(label: str) -> str:"
        docstring: JSON field label -> Java accessor-name fragment (snake_case -> camelCase).
      - id: class_identifier
        signature: "def class_identifier(raw: str) -> str:"
        docstring: "Class-name-safe identifier: PascalCase, then sanitized/keyword-escaped."
      - id: content_type_short_name
        signature: "def content_type_short_name(content_type: str) -> str:"
        docstring: "'application/json' -> 'json': the subtype is enough within our JSON-only scope."
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/__init__.py
      nodes:
      - id: MS5MEH|A8OebN
        signature: '"Assigns Java class names and package paths to identified IR nodes.\n\nNaming i…'
      - id: FZuGFV|1xi2Dg
        signature: "from dataclasses import dataclass, field"
      - id: NodeName
        signature: "@dataclass(frozen=True) class NodeName:"
        docstring: A resolved class name plus its package.
        children:
        - id: NodeName.Y0usZh|zsxU3s
          signature: '''A resolved class name plus its package.'''
        - id: NodeName.fqn
          signature: "@property def fqn(self) -> str:"
      - id: NamedModel
        signature: "@dataclass(frozen=True) class NamedModel:"
        docstring: The identified IR plus a name/package for every node that becomes a class.
        children:
        - id: NamedModel.zHMbHc|0I3MA4
          signature: '''The identified IR plus a name/package for every node that becomes a class.'''
        - id: NamedModel.name_of
          signature: "def name_of(self, node) -> NodeName | None:"
        - id: NamedModel.name_of_ref
          signature: "def name_of_ref(self, ref_name: str) -> NodeName | None:"
      - id: Entry
        signature: "@dataclass class _Entry:"
        docstring: "One candidate (node, package, class_name) before collision resolution."
      - id: assign_names
        signature: "def assign_names(identified_model, base_package: str, language: str='java') -> NamedModel:"
        docstring: "Derive class/package names for every named, anonymous, and transport node."
      - id: p66qLt|RdxhIb
        signature: _SHARED = object()
      - id: shared_package
        signature: "def _shared_package(node, base_package: str, named_ids: set) -> str:"
        docstring: The package a node falls back to once it counts as genuinely shared (site_count…
      - id: resolve_package
        signature: "def _resolve_package(node, edge_index, base_package: str, named_ids: set, transport_owner_packages: dict, memo: dict) -> str:"
        docstring: "Default: private under the single owner that reaches this node -- climbing the …"
      - id: transport_owner_packages
        signature: "def _transport_owner_packages(identified_model, base_package: str, methods_by_path: dict) -> dict:"
        docstring: id(node) -> package for every transport node that can own schema children direc…
      - id: schema_entries
        signature: "def _schema_entries(identified_model, class_names, base_package, edge_index, transport_owner_packages):"
      - id: transport_entries
        signature: "def _transport_entries(identified_model, class_names, base_package, methods_by_path):"
      - id: mirror_content_type_view_names
        signature: "def _mirror_content_type_view_names(identified_model, names: dict) -> None:"
        docstring: ContentTypeView shares its CodeNode sibling's resolved name (one synthetic clas…
      - id: mirror_ref_request_names
        signature: "def _mirror_ref_request_names(identified_model, names: dict) -> None:"
        docstring: "A $ref request body IS the request root: mirror the named schema's entry."
      - id: uzjkzH|sRfpHg
        signature: _MAX_CLIMB_DEPTH = 6
      - id: label_chain
        signature: "def _label_chain(owner, label, edge_index) -> list:"
        docstring: Nearest-first list of meaningful (non-numeric) labels reachable by climbing fro…
      - id: label_chains
        signature: "def _label_chains(node, edge_index) -> list:"
        docstring: One nearest-first label chain per site (incoming edge) reaching `node`.
      - id: ktpUBN|VKk8O1
        signature: "_EDGE_NAMEABLE_KINDS = (EnumNode, ListNode, DictionaryNode, AnyDictionaryNode, …"
      - id: build_edge_name
        signature: "def _build_edge_name(node, labels: list) -> str:"
        docstring: "Build a name from a chain of labels (nearest-first): farther ancestors become a…"
      - id: edge_name_rounds
        signature: "def _edge_name_rounds(node, edge_index):"
        docstring: "Yield successive rounds of edge-derived name candidates, one round per ancestor…"
      - id: apply_edge_names
        signature: "def _apply_edge_names(entries: list, edge_index) -> None:"
        docstring: "Prefer an edge-derived name ('RoleEnum', 'TagsList', 'AnyOfStatus', ...) over t…"
      - id: climb_name_rounds
        signature: "def _climb_name_rounds(entry, edge_index):"
        docstring: Yield successive rounds of ancestor-qualified name candidates for a colliding e…
      - id: resolve_collisions
        signature: "def _resolve_collisions(entries: list, edge_index) -> None:"
        docstring: "Same (package, class_name) from different structures: 1. try climbing ancestor …"
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/cli.py
      nodes:
      - id: 4syGSh|VMzSRA
        signature: '''Command-line argument parsing and entry point.'''
      - id: Gp16hP|OFg7jQ
        signature: import argparse
      - id: parse_args
        signature: "def parse_args(argv=None) -> Config:"
        docstring: Parse CLI args into a Config object.
      - id: main
        signature: "def main(argv=None) -> None:"
        docstring: "CLI entry point: parses args and runs the pipeline."
      - id: nmZoXZ|LQFRim
        signature: "if __name__ == '__main__':"
```
/answer 8472-26 allow
ReasoningToken: 183
ReasoningToken: 122
Control Request:
```yaml
id: 8472-27
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src
    pattern: sanitize_identifier|class_identifier\(
    include:
    - '*.py'
  reason: Aufrufstellen von sanitize_identifier/class_identifier finden für Sprachparameter-Erweiterung
```
/answer 8472-27 allow
Control Request:
```yaml
id: 8472-28
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src
      matches:
      - path: xy/cgen/naming/names.py
        matches:
        - lineno: 26
          match: "from xy.cgen.naming.identifiers import class_identifier, content_type_short_name, sanitize_identifier, to_pascal_case"
        - lineno: 79
          match: "named = {key: _with_kind_suffix(class_identifier(key), node) for key, node in identified_model.named_nodes.items()}"
        - lineno: 213
          match: return sanitize_identifier(label) + 'Enum'
      - path: xy/cgen/naming/__init__.py
        matches:
        - lineno: 255
          match: parts = ''.join((class_identifier(part) for part in reversed(labels)))
        - lineno: 318
          match: "f'{''.join((class_identifier(part) for part in reversed(chain[:depth])))}{entry.class_name}' for chain in chains if len(chain) >= depth]"
      - path: xy/cgen/naming/paths.py
        matches:
        - lineno: 3
          match: "from xy.cgen.naming.identifiers import class_identifier, sanitize_identifier, to_pascal_case"
        - lineno: 19
          match: "return [sanitize_identifier(segment).lower() for segment in _raw_segments(path)]"
        - lineno: 27
          match: return "".join(class_identifier(segment) for segment in segments)
      - path: xy/cgen/naming/identifiers.py
        matches:
        - lineno: 20
          match: "def sanitize_identifier(raw: str) -> str:"
        - lineno: 52
          match: return sanitize_identifier(to_camel_case(label))
        - lineno: 55
          match: "def class_identifier(raw: str) -> str:"
        - lineno: 57
          match: return sanitize_identifier(to_pascal_case(raw))
        - lineno: 63
          match: return sanitize_identifier(subtype).lower() or "json"
      - path: xy/cgen/emit/server_context.py
        matches:
        - lineno: 129
          match: fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
      - path: xy/cgen/emit/client_context.py
        matches:
        - lineno: 14
          match: "from xy.cgen.naming.identifiers import class_identifier, property_accessor_name, sanitize_identifier, to_camel_case"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 8472-28 allow
Control Request:
```yaml
id: 8472-29
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
    ids:
    - tPc1sR|geI8WZ
    - sanitize_identifier
    - to_pascal_case
    - to_camel_case
    - property_accessor_name
    - class_identifier
    - content_type_short_name
  reason: Vollständigen Code identifiers.py lesen
```
/answer 8472-29 allow
Control Request:
```yaml
id: 8472-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/naming/identifiers.py
      nodes:
      - id: tPc1sR|geI8WZ
        type: statements
        code: JAVA_KEYWORDS = frozenset('\n    abstract continue for new switch assert default goto package synchronized\n    boolean do if private this break double implements protected throw byte\n    else import public throws case enum instanceof return transient catch\n    extends int short try char final interface static void class finally\n    long strictfp volatile const float native super while var record yield\n    sealed permits non-sealed true false null\n    '.split())
      - id: sanitize_identifier
        type: FunctionDef
        code: |-
          def sanitize_identifier(raw: str) -> str:
              """Turn an arbitrary string into a valid, non-keyword Java identifier.

              Strips illegal characters, removes/replaces a leading underscore (so
              '_MisalignmentErrorType' stays readable rather than starting with '_'),
              guards against a leading digit, and escapes Java keywords with a suffix.
              """
              cleaned = _NON_IDENTIFIER_CHARS.sub('_', raw or '')
              cleaned = cleaned.strip('_') or 'Value'
              if cleaned[0].isdigit():
                  cleaned = f'_{cleaned}'
              if cleaned in JAVA_KEYWORDS:
                  cleaned = f'{cleaned}_'
              return cleaned
      - id: to_pascal_case
        type: FunctionDef
        code: |-
          def to_pascal_case(raw: str) -> str:
              """Split on non-alphanumeric boundaries and title-case each word."""
              words = [w for w in _WORD_SPLIT.split(raw or '') if w]
              if not words:
                  return 'Value'
              return ''.join((w[:1].upper() + w[1:] for w in words))
      - id: to_camel_case
        type: FunctionDef
        code: |-
          def to_camel_case(raw: str) -> str:
              """Same as to_pascal_case but the first letter is lower-case."""
              pascal = to_pascal_case(raw)
              return pascal[:1].lower() + pascal[1:]
      - id: property_accessor_name
        type: FunctionDef
        code: |-
          def property_accessor_name(label: str) -> str:
              """JSON field label -> Java accessor-name fragment (snake_case -> camelCase)."""
              return sanitize_identifier(to_camel_case(label))
      - id: class_identifier
        type: FunctionDef
        code: |-
          def class_identifier(raw: str) -> str:
              """Class-name-safe identifier: PascalCase, then sanitized/keyword-escaped."""
              return sanitize_identifier(to_pascal_case(raw))
      - id: content_type_short_name
        type: FunctionDef
        code: |-
          def content_type_short_name(content_type: str) -> str:
              """'application/json' -> 'json': the subtype is enough within our JSON-only scope."""
              subtype = (content_type or '').rsplit('/', maxsplit=1)[-1]
              return sanitize_identifier(subtype).lower() or 'json'
```
/answer 8472-2a allow
ReasoningToken: 5193
Control Request:
```yaml
id: 8472-2b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    ids:
    - IbJdkk|3S3BzR
    - JiTbQ2|nmyzQK
    - sd2GCB|4DLbTY
    - file_extension
    - to_php_fqn
    - to_namespace
    - get_env
  reason: "Vollständigen templates.py Header lesen (EXTENSION Definition, Imports)"
```
/answer 8472-2b allow Es würde sich eine LanguageSupport ABC anbieten die alle notwendigen Konfigurationen in einem uniformen Interface bereitstellen kann.
Control Request:
```yaml
id: 8472-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
      nodes:
      - id: IbJdkk|3S3BzR
        type: statements
        code: "'Language-aware Jinja2 environment factory: templates/<language> is the template root.'"
      - id: JiTbQ2|nmyzQK
        type: imports
        code: |-
          from functools import lru_cache
          from pathlib import Path
          from jinja2 import Environment, FileSystemLoader, StrictUndefined
      - id: sd2GCB|4DLbTY
        type: statements
        code: |-
          TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'
          EXTENSION = {'java': 'java', 'php': 'php'}
      - id: file_extension
        type: FunctionDef
        code: |-
          def file_extension(language: str) -> str:
              return EXTENSION[language]
      - id: to_php_fqn
        type: FunctionDef
        code: |-
          def _to_php_fqn(fqn: str) -> str:
              """Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\\pkg\\sub\\Class')."""
              return '\\' + fqn.replace('.', '\\')
      - id: to_namespace
        type: FunctionDef
        code: |-
          def _to_namespace(package: str) -> str:
              return package.replace('.', '\\')
      - id: get_env
        type: FunctionDef
        code: |-
          @lru_cache(maxsize=None)
          def get_env(language: str) -> Environment:
              """One cached Jinja2 Environment per language, rooted at templates/<language>."""
              env = Environment(loader=FileSystemLoader(str(TEMPLATES_ROOT / language)), trim_blocks=True,
                                lstrip_blocks=True, keep_trailing_newline=True, undefined=StrictUndefined)
              if language == 'php':
                  env.filters['phpfqn'] = _to_php_fqn
                  env.filters['phpns'] = _to_namespace
              return env
```
/answer 8472-2c allow
ReasoningToken: 4915
Control Request:
```yaml
id: 8472-2d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src
    pattern: _TYPE_CHECK_METHOD
    include:
    - '*.py'
  reason: Definition von _TYPE_CHECK_METHOD (Java) finden
```
/answer 8472-2d allow
Control Request:
```yaml
id: 8472-2e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src
      matches:
      - path: xy/cgen/emit/model_context.py
        matches:
        - lineno: 167
          match: "_TYPE_CHECK_METHOD = {"
        - lineno: 353
          match: "return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'"
        - lineno: 357
          match: "return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'"
```
/answer 8472-2e allow
ReasoningToken: 2855
Control Request:
```yaml
id: 8472-2f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  lineno: 167
  reason: _TYPE_CHECK_METHOD und _PHP_TYPE_CHECK Definition und PRIMITIVE_BRANCH_NAME finden
```
/answer 8472-2f allow
Control Request:
```yaml
id: 8472-30
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: y7NDex|Qy7hVD
        code: |-
          "# through the enclosing property's setter (which replaces the whole bound"
          '# node), never through the composition class itself.'
          _TYPE_CHECK_METHOD = {
              'string': 'isTextual',
              'integer': 'isIntegralNumber',
              'number': 'isNumber',
              'boolean': 'isBoolean'}
          _PHP_TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}
```
/answer 8472-30 allow
Control Request:
```yaml
id: 8472-31
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src
    pattern: PRIMITIVE_BRANCH_NAME|_WORD_BOUNDARY|_PHP_TYPE_CHECK
    include:
    - '*.py'
  reason: Weitere referenzierte Konstanten finden
```
/answer 8472-31 allow
Control Request:
```yaml
id: 8472-32
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src
      matches:
      - path: xy/cgen/naming/names.py
        matches:
        - lineno: 28
          match: "PRIMITIVE_BRANCH_NAME = {"
        - lineno: 89
          match: "return PRIMITIVE_BRANCH_NAME[node.primitive_type]"
        - lineno: 110
          match: "return PRIMITIVE_BRANCH_NAME[target.primitive_type]"
      - path: xy/cgen/emit/model_context.py
        matches:
        - lineno: 24
          match: from xy.cgen.naming.names import PRIMITIVE_BRANCH_NAME
        - lineno: 104
          match: "_WORD_BOUNDARY = re.compile('[^A-Za-z0-9]+')"
        - lineno: 123
          match: "words = [w for w in _WORD_BOUNDARY.split(str(value)) if w]"
        - lineno: 172
          match: "_PHP_TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}"
        - lineno: 194
          match: "return PRIMITIVE_BRANCH_NAME[target.primitive_type]"
        - lineno: 338
          match: "return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'"
        - lineno: 342
          match: "return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'"
```
/answer 8472-32 allow
Control Request:
```yaml
id: 8472-33
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    ids:
    - c7okcs|eSf5vv
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    ids:
    - LYiZTp|M4WQQL
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    ids:
    - b1RxKE|KbNbot
  reason: Imports der zu ändernden Dateien lesen
```
/answer 8472-33 allow
Control Request:
```yaml
id: 8472-34
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: c7okcs|eSf5vv
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from xy.cgen.model.nodes import MISSING, AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, PrimitiveNode, RefNode, UnsupportedNode
          from xy.cgen.naming.identifiers import property_accessor_name, to_pascal_case
          from xy.cgen.naming.names import PRIMITIVE_BRANCH_NAME
          from xy.cgen.typemap import PRIMITIVE_TYPE, map_type
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
      nodes:
      - id: LYiZTp|M4WQQL
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from xy.cgen.emit.io_context import request_root_node
          from xy.cgen.model.nodes import MISSING
          from xy.cgen.naming.identifiers import class_identifier, property_accessor_name, sanitize_identifier, to_camel_case
          from xy.cgen.naming.paths import path_to_class_fragment
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
      nodes:
      - id: b1RxKE|KbNbot
        type: imports
        code: "from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, PrimitiveNode, RefNode, UnsupportedNode"
```
/answer 8472-34 allow
ReasoningToken: 276
Control Request:
```yaml
id: 8472-35
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/base.py
    source: |
      """Common interface every target-language module (`java.py`, `php.py`, ...)
      implements. The generic pipeline (naming/emit) only ever talks to a
      `LanguageSupport` instance obtained via `xy.cgen.lang.get_language`, never
      to a hardcoded language name or an `if language == ...` branch. Adding a
      language means adding one module here plus a `templates/<language>` dir.
      """
      from abc import ABC, abstractmethod


      class LanguageSupport(ABC):

          name: str
          extension: str
          primitive_type: dict
          any_dictionary_type: str
          parameter_type: dict
          parameter_default_type: str
          read_method: dict
          factory_method: dict
          jinja_filters: dict

          @abstractmethod
          def string_literal(self, text: str) -> str:
              """A quoted, escaped string literal."""

          @abstractmethod
          def literal(self, value, primitive_type: str) -> str:
              """An enum-constant literal for `value` of the given primitive type."""

          @abstractmethod
          def type_hint(self, type_name: str) -> str:
              """A dotted fqn or scalar type name as it appears in a type position."""

          @abstractmethod
          def parameter_declaration(self, type_hint: str, var_name: str) -> str:
              """One '<type> <name>'-style method-signature parameter."""

          @abstractmethod
          def reference_expr(self, name: str) -> str:
              """A bare local-variable reference, e.g. for a call-argument list."""

          @abstractmethod
          def path_url_expression(self, path: str, path_params) -> str:
              """An expression rebuilding a URL path, every '{param}' token replaced
              by its URL-encoded argument value. `path_params` items have
              `.raw_name` (the '{token}') and `.name` (the local holding the value)."""

          @abstractmethod
          def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
              """A boolean expression testing a discriminator property against one value."""

          @abstractmethod
          def applies_always(self) -> str:
              """Structural-applicability expression for a branch with no check."""

          @abstractmethod
          def applies_required(self, fields: list) -> str:
              """Structural-applicability expression: all `fields` are present."""

          @abstractmethod
          def applies_array(self) -> str:
              """Structural-applicability expression: value is a JSON array."""

          @abstractmethod
          def applies_object(self) -> str:
              """Structural-applicability expression: value is a JSON object."""

          @abstractmethod
          def applies_type_check(self, primitive_type: str) -> str:
              """Structural-applicability expression: value has the given primitive type."""

          @abstractmethod
          def applies_null(self) -> str:
              """Structural-applicability expression: value is JSON null."""

          @abstractmethod
          def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
              """Statements decoding matcher groups / query params / body into locals
              for the server dispatcher; one `ServerMethod.binding_lines` tuple."""
  reason: LanguageSupport ABC als zentrales Interface für Sprachspezifika anlegen
```
/answer 8472-35 allow
ReasoningToken: 125
Control Request:
```yaml
id: 8472-36
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/java.py
    source: |
      """Java language support: types, literals, and code-fragment generation."""
      import re

      from xy.cgen.lang.base import LanguageSupport

      _PATH_PARAM = re.compile(r'\{([^}]+)\}')

      #: JsonNodeFactory typed-constructor names, used only where ArrayNode has no
      #: typed set() overload (mixed/tuple lists, see list_mixed.jinja).
      _FACTORY_METHOD = {
          'string': 'textNode',
          'integer': 'numberNode',
          'number': 'numberNode',
          'boolean': 'booleanNode'}

      #: JsonNode.is*() method per primitive type, used for structural type checks.
      _TYPE_CHECK_METHOD = {
          'string': 'isTextual',
          'integer': 'isIntegralNumber',
          'number': 'isNumber',
          'boolean': 'isBoolean'}

      #: expression parsing a decoded String into the parameter's scalar type.
      _PARSE_EXPR = {
          'Long': 'Long.valueOf({raw})',
          'Double': 'Double.valueOf({raw})',
          'Boolean': 'Boolean.valueOf({raw})',
          'String': '{raw}'}


      def _string_literal(text: str) -> str:
          escaped = text.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r')
          return f'"{escaped}"'


      class JavaSupport(LanguageSupport):

          name = 'java'
          extension = 'java'
          primitive_type = {'string': 'String', 'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
          any_dictionary_type = 'com.fasterxml.jackson.databind.JsonNode'
          parameter_type = {'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
          parameter_default_type = 'String'
          read_method = {'string': 'asText', 'integer': 'asLong', 'number': 'asDouble', 'boolean': 'asBoolean'}
          factory_method = _FACTORY_METHOD
          jinja_filters = {}

          def string_literal(self, text: str) -> str:
              return _string_literal(text)

          def literal(self, value, primitive_type: str) -> str:
              if primitive_type == 'boolean':
                  return 'true' if value else 'false'
              if primitive_type == 'string':
                  return _string_literal(str(value))
              if primitive_type == 'integer':
                  return f'{int(value)}L'
              if primitive_type == 'number':
                  return f'{float(value)}d'
              raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

          def type_hint(self, type_name: str) -> str:
              return type_name

          def parameter_declaration(self, type_hint: str, var_name: str) -> str:
              return f'{type_hint} {var_name}'

          def reference_expr(self, name: str) -> str:
              return name

          def path_url_expression(self, path: str, path_params) -> str:
              by_raw_name = {p.raw_name: p for p in path_params}
              parts, last = [], 0
              for match in _PATH_PARAM.finditer(path):
                  literal = path[last:match.start()]
                  if literal:
                      parts.append(_string_literal(literal))
                  param = by_raw_name[match.group(1)]
                  parts.append(f'java.net.URLEncoder.encode(String.valueOf({param.name}), java.nio.charset.StandardCharsets.UTF_8)')
                  last = match.end()
              tail = path[last:]
              if tail or not parts:
                  parts.append(_string_literal(tail))
              return ' + '.join(parts)

          def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
              "# Uses JsonNode.path() (never MissingNode == null) so no separate absence guard is needed here."
              accessor = f'node.path("{property_name}")'
              if primitive_type == 'string':
                  return f'{_string_literal(str(value))}.equals({accessor}.asText())'
              if primitive_type == 'integer':
                  return f'{accessor}.asLong() == {int(value)}L'
              if primitive_type == 'number':
                  return f'{accessor}.asDouble() == {float(value)}d'
              if primitive_type == 'boolean':
                  return f"{accessor}.asBoolean() == {('true' if value else 'false')}"
              raise ValueError(f'discriminator property has no supported primitive type: {primitive_type!r}')

          def applies_always(self) -> str:
              return 'true'

          def applies_required(self, fields: list) -> str:
              return ' && '.join(f'node.has("{field_name}")' for field_name in fields)

          def applies_array(self) -> str:
              return 'node.isArray()'

          def applies_object(self) -> str:
              return 'node.isObject()'

          def applies_type_check(self, primitive_type: str) -> str:
              return f'node.{_TYPE_CHECK_METHOD[primitive_type]}()'

          def applies_null(self) -> str:
              return 'node.isNull()'

          def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
              lines = []
              for i, param in enumerate(regex_order, start=1):
                  raw = f'URLDecoder.decode(matcher.group({i}), StandardCharsets.UTF_8)'
                  lines.append(f'{param.java_type} {param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
              for param in client_method.query_params:
                  raw_var = f'{param.name}Raw'
                  lines.append(f'String {raw_var} = queryParams.get("{param.raw_name}");')
                  lines.append(
                      f'{param.java_type} {param.name} = {raw_var} == null ? null : {_PARSE_EXPR[param.java_type].format(raw=raw_var)};')
              if client_method.body_param is not None:
                  lines.append('String rawBody = readBody(exchange);')
                  lines.append(
                      f'{client_method.body_param.java_type} {client_method.body_param.name} = '
                      f'new {client_method.body_param.java_type}({body_json_support_fqn}.parse(rawBody));')
              return tuple(lines)


      JAVA = JavaSupport()
  reason: Java-Sprachspezifika zentral in lang/java.py anlegen
```
/answer 8472-36 allow
ReasoningToken: 426
Text:
 Jetzt java.py:
Control Request:
```yaml
id: 8472-37
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/php.py
    source: |
      """PHP language support: types, literals, and code-fragment generation."""
      import re

      from xy.cgen.lang.base import LanguageSupport

      _PATH_PARAM = re.compile(r'\{([^}]+)\}')

      _SCALAR_TYPES = {'int', 'float', 'string', 'bool', 'mixed'}

      #: is_*() function per primitive type, used for structural type checks.
      _TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}

      #: expression parsing a decoded string into the parameter's scalar type.
      _PARSE_EXPR = {
          'int': '(int) {raw}',
          'float': '(float) {raw}',
          'bool': 'filter_var({raw}, FILTER_VALIDATE_BOOLEAN)',
          'string': '{raw}'}


      def _string_literal(text: str) -> str:
          escaped = text.replace('\\', '\\\\').replace("'", "\\'")
          return f"'{escaped}'"


      def _to_php_fqn(fqn: str) -> str:
          """Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\\pkg\\sub\\Class')."""
          return '\\' + fqn.replace('.', '\\')


      def _to_namespace(package: str) -> str:
          return package.replace('.', '\\')


      class PhpSupport(LanguageSupport):

          name = 'php'
          extension = 'php'
          primitive_type = {'string': 'string', 'integer': 'int', 'number': 'float', 'boolean': 'bool'}
          any_dictionary_type = 'mixed'
          parameter_type = {'integer': 'int', 'number': 'float', 'boolean': 'bool'}
          parameter_default_type = 'string'
          read_method = primitive_type
          factory_method = {}
          jinja_filters = {'phpfqn': _to_php_fqn, 'phpns': _to_namespace}

          def string_literal(self, text: str) -> str:
              return _string_literal(text)

          def literal(self, value, primitive_type: str) -> str:
              if primitive_type == 'boolean':
                  return 'true' if value else 'false'
              if primitive_type == 'string':
                  return _string_literal(str(value))
              if primitive_type == 'integer':
                  return str(int(value))
              if primitive_type == 'number':
                  return repr(float(value))
              raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

          def type_hint(self, type_name: str) -> str:
              """A dotted fqn ('pkg.Class') becomes a fully-qualified PHP type hint; scalars pass through."""
              if type_name in _SCALAR_TYPES or '.' not in type_name:
                  return type_name
              return _to_php_fqn(type_name)

          def parameter_declaration(self, type_hint: str, var_name: str) -> str:
              return f'{type_hint} ${var_name}'

          def reference_expr(self, name: str) -> str:
              return f'${name}'

          def path_url_expression(self, path: str, path_params) -> str:
              by_raw_name = {p.raw_name: p for p in path_params}
              parts, last = [], 0
              for match in _PATH_PARAM.finditer(path):
                  literal = path[last:match.start()]
                  if literal:
                      parts.append(_string_literal(literal))
                  param = by_raw_name[match.group(1)]
                  parts.append(f'rawurlencode((string) ${param.name})')
                  last = match.end()
              tail = path[last:]
              if tail or not parts:
                  parts.append(_string_literal(tail))
              return ' . '.join(parts)

          def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
              accessor = f"($this->node['{property_name}'] ?? null)"
              if primitive_type == 'boolean':
                  return f"{accessor} === {('true' if value else 'false')}"
              return f'{accessor} === {self.literal(value, primitive_type)}'

          def applies_always(self) -> str:
              return 'true'

          def applies_required(self, fields: list) -> str:
              return ' && '.join(f"array_key_exists('{field_name}', (array) $this->node)" for field_name in fields)

          def applies_array(self) -> str:
              return 'is_array($this->node)'

          def applies_object(self) -> str:
              return 'is_array($this->node) || is_object($this->node)'

          def applies_type_check(self, primitive_type: str) -> str:
              return f'{_TYPE_CHECK[primitive_type]}($this->node)'

          def applies_null(self) -> str:
              return '$this->node === null'

          def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
              lines = []
              for i, param in enumerate(regex_order, start=1):
                  raw = f'urldecode($matches[{i}])'
                  lines.append(f'${param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
              for param in client_method.query_params:
                  raw_var = f'${param.name}Raw'
                  lines.append(f"{raw_var} = $queryParams['{param.raw_name}'] ?? null;")
                  lines.append(
                      f'${param.name} = {raw_var} === null ? null : {_PARSE_EXPR[param.java_type].format(raw=raw_var)};')
              if client_method.body_param is not None:
                  lines.append("$rawBody = file_get_contents('php://input');")
                  lines.append(
                      f'${client_method.body_param.name} = '
                      f'new {self.type_hint(client_method.body_param.java_type)}(json_decode($rawBody, true));')
              return tuple(lines)


      PHP = PhpSupport()
  reason: PHP-Sprachspezifika zentral in lang/php.py anlegen
```
/answer 8472-37 allow
Control Request:
```yaml
id: 8472-38
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/__init__.py
    source: |
      """Registry of per-language code-generation support (`LanguageSupport`
      implementations). Naming/emit look up the target language here instead of
      branching on the language string themselves; adding a language means adding
      one module (`java.py`, `php.py`, ...) plus a `templates/<language>` dir.
      """
      from xy.cgen.lang.base import LanguageSupport
      from xy.cgen.lang.java import JAVA
      from xy.cgen.lang.php import PHP

      LANGUAGES = {'java': JAVA, 'php': PHP}


      def get_language(language: str) -> LanguageSupport:
          return LANGUAGES[language]
  reason: Registry/Lookup für LanguageSupport-Instanzen anlegen
```
/answer 8472-38 allow
Control Request:
```yaml
id: 8472-39
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    start_marker: "JAVA_PRIMITIVE_TYPE = {'string'"
    end_marker: kinds whose type is the generated class assigned by naming.'
    content: '''# Node kinds whose type is the generated class assigned by naming.'''
  reason: Sprachspezifische Dicts aus typemap entfernen
```
/answer 8472-39 allow
Control Request:
```yaml
id: 8472-3a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    old_text: "from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, PrimitiveNode, RefNode, UnsupportedNode"
    new_text: |-
      from xy.cgen.lang import get_language
      from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, PrimitiveNode, RefNode, UnsupportedNode
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    old_text: "        return ANY_DICTIONARY_TYPE[named_model.language]"
    new_text: '        return get_language(named_model.language).any_dictionary_type'
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    old_text: "    return PRIMITIVE_TYPE[language][node.primitive_type]"
    new_text: "    return get_language(language).primitive_type[node.primitive_type]"
  reason: typemap auf lang-Registry umstellen
```
/answer 8472-3a allow
Control Request:
```yaml
id: 8472-3b
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
  reason: typemap-Datei validieren
```
/answer 8472-3b allow
Control Request:
```yaml
id: 8472-3c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src
    pattern: PRIMITIVE_TYPE
    include:
    - '*.py'
  reason: Alle Verwender von PRIMITIVE_TYPE aus typemap finden
```
/answer 8472-3c allow
Control Request:
```yaml
id: 8472-3d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src
      matches:
      - path: xy/cgen/emit/model_context.py
        matches:
        - lineno: 25
          match: "from xy.cgen.typemap import PRIMITIVE_TYPE, map_type"
        - lineno: 35
          match: "PHP_PRIMITIVE_CAST = PRIMITIVE_TYPE['php']"
        - lineno: 157
          match: "return PRIMITIVE_TYPE[language][node.primitive_type]"
      - path: xy/cgen/model/build.py
        matches:
        - lineno: 32
          match: "PRIMITIVE_TYPES = (\"string\", \"integer\", \"number\", \"boolean\", \"null\")"
        - lineno: 114
          match: "if schema_type in PRIMITIVE_TYPES:"
```
/answer 8472-3d allow
Control Request:
```yaml
id: 8472-3e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    old_text: "from xy.cgen.typemap import PRIMITIVE_TYPE, map_type"
    new_text: |-
      from xy.cgen.lang import get_language
      from xy.cgen.typemap import map_type
  reason: Import in model_context.py auf lang umstellen
```
/answer 8472-3e allow
Control Request:
```yaml
id: 8472-3f
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    start_marker: "JAVA_PRIMITIVE_READ_METHOD = {'string'"
    end_marker: "FACTORY_METHOD = {'java': JAVA_PRIMITIVE_FACTORY_METHOD, 'php': {}}"
    content: "'# per-language scalar read/cast and factory methods, see xy.cgen.lang.'"
  reason: Sprachdicts aus model_context.py entfernen
```
/answer 8472-3f allow
Control Request:
```yaml
id: 8472-40
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      error: No node matched the given text; a node selector (id) is required.
```
/answer 8472-40 allow
Control Request:
```yaml
id: 8472-41
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: eGXYlS|GfBTZC
    source: "'# per-language scalar read/cast and factory methods, see xy.cgen.lang.'"
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: X9s0wx|D3k0eh
    source: ""
  reason: Java/PHP Read-Method Dicts aus model_context.py entfernen
```
/answer 8472-41 allow
Control Request:
```yaml
id: 8472-42
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      result: success
      id: eGXYlS|GfBTZC
      new_id: eGXYlS|GfBTZC
    errors:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      error: No node matched the selector.
      id: X9s0wx|D3k0eh
```
/answer 8472-42 allow
Control Request:
```yaml
id: 8472-43
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  text: PHP_PRIMITIVE_CAST
  reason: Aktuelle Node-ID für PHP_PRIMITIVE_CAST Block finden
```
/answer 8472-43 allow
Control Request:
```yaml
id: 8472-44
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: gBiyKl|RjwGU5
        code: |-
          '# per-language scalar read/cast and factory methods, see xy.cgen.lang.'
          '# PHP cast keyword used as a prefix cast, e.g. (int) $value -- reuses the scalar type map.'
          PHP_PRIMITIVE_CAST = PRIMITIVE_TYPE['php']
          READ_METHOD = {'java': JAVA_PRIMITIVE_READ_METHOD, 'php': PHP_PRIMITIVE_CAST}
          FACTORY_METHOD = {'java': JAVA_PRIMITIVE_FACTORY_METHOD, 'php': {}}
```
/answer 8472-44 allow
Control Request:
```yaml
id: 8472-45
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: gBiyKl|RjwGU5
    source: "'# per-language scalar read/cast and factory methods, see xy.cgen.lang.'"
  reason: Restliche Sprachdicts aus model_context.py entfernen
```
/answer 8472-45 allow
Text:
 Jetzt php.py:
Control Request:
```yaml
id: 8472-46
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    ids:
    - build_accessor
    - enum_constants
    - constant_base
    - literal
    - java_string_literal
    - php_string_literal
    - enum_raw_type
    - branch_accessor_name
    - build_branches
    - discriminator_literal_expr
    - structural_applies_expr
  reason: Aktuellen Stand der zu ändernden Funktionen erneut lesen (IDs könnten stabil sein)
```
/answer 8472-46 allow
Control Request:
```yaml
id: 8472-47
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: build_accessor
        type: FunctionDef
        code: |-
          def build_accessor(label: str, edge, named_model) -> Accessor | None:
              """Build the accessor context for one edge, or None if it has no view."""
              category, primitive_type = classify(edge.target, named_model.named_nodes)
              if category == 'unsupported':
                  return None
              language = named_model.language
              return Accessor(
                  label=label, name=to_pascal_case(
                      property_accessor_name(label)), java_type=map_type(
                          edge.target, named_model), category=category, read_method=READ_METHOD[language].get(primitive_type) if category in (
                              'primitive', 'enum') else None, factory_method=FACTORY_METHOD[language].get(primitive_type) if category in (
                                  'primitive', 'enum') else None, description=edge.description, example_repr=None if edge.example is MISSING else repr(
                                      edge.example))
      - id: enum_constants
        type: FunctionDef
        code: |-
          def enum_constants(node, language: str='java') -> list[EnumConstant]:
              """One enum constant per declared value, in declaration order (deterministic input)."""
              seen_names: dict = {}
              constants = []
              for value in node.values:
                  base = _constant_base(value)
                  seen_names[base] = seen_names.get(base, 0) + 1
                  name = base if seen_names[base] == 1 else f'{base}_{seen_names[base]}'
                  constants.append(EnumConstant(constant_name=name, literal=_literal(value, node.primitive_type, language)))
              return constants
      - id: constant_base
        type: FunctionDef
        code: |-
          def _constant_base(value) -> str:
              words = [w for w in _WORD_BOUNDARY.split(str(value)) if w]
              if not words:
                  return 'VALUE'
              base = '_'.join((w.upper() for w in words))
              return f'_{base}' if base[0].isdigit() else base
      - id: literal
        type: FunctionDef
        code: |-
          def _literal(value, primitive_type: str, language: str) -> str:
              if primitive_type == 'boolean':
                  return 'true' if value else 'false'
              if language == 'php':
                  if primitive_type == 'string':
                      return _php_string_literal(str(value))
                  if primitive_type == 'integer':
                      return str(int(value))
                  if primitive_type == 'number':
                      return repr(float(value))
                  raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')
              if primitive_type == 'string':
                  return _java_string_literal(str(value))
              if primitive_type == 'integer':
                  return f'{int(value)}L'
              if primitive_type == 'number':
                  return f'{float(value)}d'
              raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')
      - id: java_string_literal
        type: FunctionDef
        code: |-
          def _java_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r')
              return f'"{escaped}"'
      - id: php_string_literal
        type: FunctionDef
        code: |-
          def _php_string_literal(text: str) -> str:
              escaped = text.replace('\\', '\\\\').replace("'", "\\'")
              return f"'{escaped}'"
      - id: enum_raw_type
        type: FunctionDef
        code: |-
          def enum_raw_type(node, language: str='java') -> str:
              return PRIMITIVE_TYPE[language][node.primitive_type]
      - id: branch_accessor_name
        type: FunctionDef
        code: |-
          def _branch_accessor_name(target, named_model) -> str:
              """The branch's own generated class-name fragment."""
              if isinstance(target, RefNode):
                  return named_model.name_of_ref(target.name).class_name
              if isinstance(target, PrimitiveNode):
                  return PRIMITIVE_BRANCH_NAME[target.primitive_type]
              return named_model.name_of(target).class_name
      - id: build_branches
        type: FunctionDef
        code: |-
          def build_branches(node, named_model) -> list[Branch]:
              """Build the render context for every branch of one CompositionNode."""
              named_nodes = named_model.named_nodes
              language = named_model.language
              needs_applies = node.keyword in ('anyOf', 'oneOf')
              discriminator_values = _resolve_discriminator_values(
                  node, named_nodes) if needs_applies and node.discriminator else {}
              branches = []
              for index, edge in enumerate(node.branches):
                  target = edge.target
                  resolved = resolve_structural(target, named_nodes)
                  if isinstance(resolved, UnsupportedNode):
                      '# no view for an unsupported branch'
                      continue
                  example_repr = None if edge.example is MISSING else repr(edge.example)
                  accessor_name = _branch_accessor_name(target, named_model)
                  applies_expr = None
                  if needs_applies:
                      values = discriminator_values.get(index)
                      if values:
                          applies_expr = ' || '.join(
                              (_discriminator_literal_expr(
                                  node.discriminator.property_name,
                                  value,
                                  primitive_type,
                                  language) for value,
                               primitive_type in values))
                      else:
                          applies_expr = _structural_applies_expr(resolved, language)
                  if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':
                      branches.append(
                          Branch(
                              accessor_name=accessor_name,
                              java_type=None,
                              category='null',
                              read_method=None,
                              description=edge.description,
                              example_repr=example_repr,
                              has_getter=False,
                              applies_expr=applies_expr))
                      continue
                  category, primitive_type = classify(target, named_nodes)
                  branches.append(
                      Branch(
                          accessor_name=accessor_name,
                          java_type=map_type(
                              target,
                              named_model),
                          category=category,
                          read_method=READ_METHOD[language].get(primitive_type) if category in (
                              'primitive',
                              'enum') else None,
                          description=edge.description,
                          example_repr=example_repr,
                          has_getter=True,
                          applies_expr=applies_expr))
              return branches
      - id: discriminator_literal_expr
        type: FunctionDef
        code: |-
          def _discriminator_literal_expr(property_name: str, value, primitive_type: str, language: str) -> str:
              """A boolean expression testing `node`'s discriminator property against one value."""
              if language == 'php':
                  accessor = f"($this->node['{property_name}'] ?? null)"
                  if primitive_type == 'boolean':
                      return f'{accessor} === {('true' if value else 'false')}'
                  return f'{accessor} === {_literal(value, primitive_type, language)}'
              '# Uses JsonNode.path() (never MissingNode == null) so no separate absence guard is needed here.'
              accessor = f'node.path("{property_name}")'
              if primitive_type == 'string':
                  return f'{_java_string_literal(str(value))}.equals({accessor}.asText())'
              if primitive_type == 'integer':
                  return f'{accessor}.asLong() == {int(value)}L'
              if primitive_type == 'number':
                  return f'{accessor}.asDouble() == {float(value)}d'
              if primitive_type == 'boolean':
                  return f'{accessor}.asBoolean() == {('true' if value else 'false')}'
              raise ValueError(f'discriminator property has no supported primitive type: {primitive_type!r}')
      - id: structural_applies_expr
        type: FunctionDef
        code: |-
          def _structural_applies_expr(resolved, language: str) -> str:
              """presence of required fields / JSON type, no discriminator const available."""
              if language == 'php':
                  if isinstance(resolved, ObjectNode):
                      if not resolved.required:
                          return 'true'
                      return ' && '.join(
                          (f"array_key_exists('{field_name}', (array) $this->node)" for field_name in sorted(resolved.required)))
                  if isinstance(resolved, ListNode):
                      return 'is_array($this->node)'
                  if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
                      return 'is_array($this->node) || is_object($this->node)'
                  if isinstance(resolved, EnumNode):
                      return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'
                  if isinstance(resolved, PrimitiveNode):
                      if resolved.primitive_type == 'null':
                          return '$this->node === null'
                      return f'{_PHP_TYPE_CHECK[resolved.primitive_type]}($this->node)'
                  return 'true'
              if isinstance(resolved, ObjectNode):
                  if not resolved.required:
                      return 'true'
                  return ' && '.join((f'node.has("{field_name}")' for field_name in sorted(resolved.required)))
              if isinstance(resolved, ListNode):
                  return 'node.isArray()'
              if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
                  return 'node.isObject()'
              if isinstance(resolved, EnumNode):
                  return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'
              if isinstance(resolved, PrimitiveNode):
                  if resolved.primitive_type == 'null':
                      return 'node.isNull()'
                  return f'node.{_TYPE_CHECK_METHOD[resolved.primitive_type]}()'
              '# nested composition or other structural node: best-effort, never validated further'
              return 'true'
```
/answer 8472-47 allow
Control Request:
```yaml
id: 8472-48
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: build_accessor
    source: |-
      def build_accessor(label: str, edge, named_model) -> Accessor | None:
          """Build the accessor context for one edge, or None if it has no view."""
          category, primitive_type = classify(edge.target, named_model.named_nodes)
          if category == 'unsupported':
              return None
          lang = get_language(named_model.language)
          has_read = category in ('primitive', 'enum')
          return Accessor(
              label=label,
              name=to_pascal_case(property_accessor_name(label)),
              java_type=map_type(edge.target, named_model),
              category=category,
              read_method=lang.read_method.get(primitive_type) if has_read else None,
              factory_method=lang.factory_method.get(primitive_type) if has_read else None,
              description=edge.description,
              example_repr=None if edge.example is MISSING else repr(edge.example))
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: enum_constants
    source: |-
      def enum_constants(node, language: str='java') -> list[EnumConstant]:
          """One enum constant per declared value, in declaration order (deterministic input)."""
          lang = get_language(language)
          seen_names: dict = {}
          constants = []
          for value in node.values:
              base = _constant_base(value)
              seen_names[base] = seen_names.get(base, 0) + 1
              name = base if seen_names[base] == 1 else f'{base}_{seen_names[base]}'
              constants.append(EnumConstant(constant_name=name, literal=lang.literal(value, node.primitive_type)))
          return constants
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: literal
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: java_string_literal
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: php_string_literal
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: enum_raw_type
    source: |-
      def enum_raw_type(node, language: str='java') -> str:
          return get_language(language).primitive_type[node.primitive_type]
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: discriminator_literal_expr
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: structural_applies_expr
    source: |-
      def _structural_applies_expr(resolved, language: str) -> str:
          """presence of required fields / JSON type, no discriminator const available."""
          lang = get_language(language)
          if isinstance(resolved, ObjectNode):
              if not resolved.required:
                  return lang.applies_always()
              return lang.applies_required(sorted(resolved.required))
          if isinstance(resolved, ListNode):
              return lang.applies_array()
          if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
              return lang.applies_object()
          if isinstance(resolved, EnumNode):
              return lang.applies_type_check(resolved.primitive_type)
          if isinstance(resolved, PrimitiveNode):
              if resolved.primitive_type == 'null':
                  return lang.applies_null()
              return lang.applies_type_check(resolved.primitive_type)
          '# nested composition or other structural node: best-effort, never validated further'
          return lang.applies_always()
  reason: model_context.py Funktionen auf lang-Delegation umstellen
```
/answer 8472-48 allow
Control Request:
```yaml
id: 8472-49
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: build_branches
    source: |-
      def build_branches(node, named_model) -> list[Branch]:
          """Build the render context for every branch of one CompositionNode."""
          named_nodes = named_model.named_nodes
          lang = get_language(named_model.language)
          needs_applies = node.keyword in ('anyOf', 'oneOf')
          discriminator_values = _resolve_discriminator_values(
              node, named_nodes) if needs_applies and node.discriminator else {}
          branches = []
          for index, edge in enumerate(node.branches):
              target = edge.target
              resolved = resolve_structural(target, named_nodes)
              if isinstance(resolved, UnsupportedNode):
                  '# no view for an unsupported branch'
                  continue
              example_repr = None if edge.example is MISSING else repr(edge.example)
              accessor_name = _branch_accessor_name(target, named_model)
              applies_expr = None
              if needs_applies:
                  values = discriminator_values.get(index)
                  if values:
                      applies_expr = ' || '.join(
                          lang.discriminator_literal_expr(node.discriminator.property_name, value, primitive_type)
                          for value, primitive_type in values)
                  else:
                      applies_expr = _structural_applies_expr(resolved, named_model.language)
              if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':
                  branches.append(
                      Branch(
                          accessor_name=accessor_name,
                          java_type=None,
                          category='null',
                          read_method=None,
                          description=edge.description,
                          example_repr=example_repr,
                          has_getter=False,
                          applies_expr=applies_expr))
                  continue
              category, primitive_type = classify(target, named_nodes)
              branches.append(
                  Branch(
                      accessor_name=accessor_name,
                      java_type=map_type(target, named_model),
                      category=category,
                      read_method=lang.read_method.get(primitive_type) if category in ('primitive', 'enum') else None,
                      description=edge.description,
                      example_repr=example_repr,
                      has_getter=True,
                      applies_expr=applies_expr))
          return branches
  reason: build_branches auf lang-Delegation umstellen
```
/answer 8472-49 allow
Control Request:
```yaml
id: 8472-4a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  text: _TYPE_CHECK_METHOD
  reason: Aktuelle Node-ID für _TYPE_CHECK_METHOD Block finden
```
/answer 8472-4a allow
Control Request:
```yaml
id: 8472-4b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: y7NDex|Qy7hVD
        code: |-
          "# through the enclosing property's setter (which replaces the whole bound"
          '# node), never through the composition class itself.'
          _TYPE_CHECK_METHOD = {
              'string': 'isTextual',
              'integer': 'isIntegralNumber',
              'number': 'isNumber',
              'boolean': 'isBoolean'}
          _PHP_TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}
```
/answer 8472-4b allow
Control Request:
```yaml
id: 8472-4c
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: y7NDex|Qy7hVD
    source: |-
      "# through the enclosing property's setter (which replaces the whole bound"
      '# node), never through the composition class itself.'
  reason: Verbleibende Java/PHP Type-Check Dicts aus model_context.py entfernen
```
/answer 8472-4c allow
ReasoningToken: 74
Text:
 Nun die Verbraucher anpassen: typemap, model_context, client_context, server_context, templates.py.
Control Request:
```yaml
id: 8472-4d
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  reason: model_context.py nach Umbau validieren
```
/answer 8472-4d allow
Control Request:
```yaml
id: 8472-4e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
    pattern: READ_METHOD|FACTORY_METHOD|PRIMITIVE_TYPE|_java_string_literal|_php_string_literal|_TYPE_CHECK|language ==
    include:
    - model_context.py
  reason: "Prüfen, ob alte Referenzen in model_context.py verbleiben"
```
/answer 8472-4e allow
Control Request:
```yaml
id: 8472-4f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
      matches: []
```
/answer 8472-4f allow
Control Request:
```yaml
id: 8472-50
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    old_text: from xy.cgen.emit.io_context import request_root_node
    new_text: |-
      from xy.cgen.emit.io_context import request_root_node
      from xy.cgen.lang import get_language
  reason: Import in client_context.py hinzufügen
```
/answer 8472-50 allow
Control Request:
```yaml
id: 8472-51
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: z0x58G|COEEvN
    source: "_PATH_PARAM = re.compile('\\\\{([^}]+)\\\\}')"
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: parameter_type
    source: |-
      def _parameter_type(schema: dict | None, language: str) -> str:
          """Path/query parameters never enter the body tree; map their
          raw JSON-Schema type directly to a scalar type (string is the default)."""
          lang = get_language(language)
          return lang.parameter_type.get((schema or {}).get('type'), lang.parameter_default_type)
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: java_string_literal
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: php_string_literal
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: id98Oo|sRRjxF
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: php_type_hint
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: signature
    source: |-
      def _signature(parameters: tuple, language: str) -> str:
          lang = get_language(language)
          return ', '.join(lang.parameter_declaration(lang.type_hint(p.java_type), p.name) for p in parameters)
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    id: path_url_expression
    source: |-
      def _path_url_expression(path: str, path_params: tuple, language: str) -> str:
          """A string-concatenation expression rebuilding the URL path, with
          every '{param}' token replaced by its URL-encoded argument value."""
          return get_language(language).path_url_expression(path, path_params)
  reason: client_context.py Funktionen auf lang-Delegation umstellen
```
/answer 8472-51 allow
ReasoningToken: 100
Text:
 Jetzt model_context.py komplett überarbeiten.
Control Request:
```yaml
id: 8472-52
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
  reason: client_context.py validieren
```
/answer 8472-52 allow
Control Request:
```yaml
id: 8472-53
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
    ids:
    - parameter_type
    - MethodParameter
    - build_parameter
    - signature
    - path_url_expression
  reason: Aktuellen Zustand nach Edits prüfen
```
/answer 8472-53 allow
Control Request:
```yaml
id: 8472-54
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
      nodes:
      - id: parameter_type
        type: FunctionDef
        code: |-
          def _parameter_type(schema: dict | None, language: str) -> str:
              """Path/query parameters never enter the body tree; map their
              raw JSON-Schema type directly to a scalar type (string is the default)."""
              lang = get_language(language)
              return lang.parameter_type.get((schema or {}).get('type'), lang.parameter_default_type)
      - id: MethodParameter
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class MethodParameter:
              """One path/query parameter, or the synthetic body parameter."""
              '# Java identifier used in the method signature'
              name: str
              java_type: str
              '# original OpenAPI parameter name (the actual URL token/query key)'
              raw_name: str
              "# 'path' | 'query' | 'body'"
              kind: str
      - id: build_parameter
        type: FunctionDef
        code: |-
          def _build_parameter(param, kind: str, language: str) -> MethodParameter:
              return MethodParameter(
                  name=property_accessor_name(
                      param.name), java_type=_parameter_type(
                          param.schema, language), raw_name=param.name, kind=kind)
      - id: signature
        type: FunctionDef
        code: |-
          def _signature(parameters: tuple, language: str) -> str:
              lang = get_language(language)
              return ', '.join((lang.parameter_declaration(lang.type_hint(p.java_type), p.name) for p in parameters))
      - id: path_url_expression
        type: FunctionDef
        code: |-
          def _path_url_expression(path: str, path_params: tuple, language: str) -> str:
              """A string-concatenation expression rebuilding the URL path, with
              every '{param}' token replaced by its URL-encoded argument value."""
              return get_language(language).path_url_expression(path, path_params)
```
/answer 8472-54 allow
Control Request:
```yaml
id: 8472-55
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    old_text: "from xy.cgen.emit.client_context import MethodParameter, _php_type_hint, _top_path_segment, build_client_methods"
    new_text: |-
      from xy.cgen.emit.client_context import MethodParameter, _top_path_segment, build_client_methods
      from xy.cgen.lang import get_language
  reason: Import in server_context.py aktualisieren
```
/answer 8472-55 allow
Control Request:
```yaml
id: 8472-56
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: mm8SYx|RWeTpl
    source: "_PATH_PARAM = re.compile('\\\\{([^}]+)\\\\}')"
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: QLdKL0|JYEPq1
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: binding_lines
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: binding_lines_php
    source: ""
  reason: Sprachspezifische Parse/Binding-Logik aus server_context.py entfernen
```
/answer 8472-56 allow
ReasoningToken: 79
Control Request:
```yaml
id: 8472-57
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      result: success
      id: mm8SYx|RWeTpl
      new_id: mm8SYx|RWeTpl
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      result: success
      id: binding_lines
      new_id: binding_lines
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      result: success
      id: binding_lines_php
      new_id: binding_lines_php
    errors:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      error: No node matched the selector.
      id: QLdKL0|JYEPq1
```
/answer 8472-57 allow
ReasoningToken: 27
Control Request:
```yaml
id: 8472-58
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    ids:
    - build_server_methods
    - server_class_name
  reason: Aktuellen Stand von server_context.py nach ersten Edits sehen
```
/answer 8472-58 allow
Control Request:
```yaml
id: 8472-59
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: build_server_methods
        type: FunctionDef
        code: |-
          def build_server_methods(named_model) -> list[ServerMethod]:
              """One ServerMethod per operation, same order as build_client_methods."""
              support_fqn = json_support_fqn(named_model.base_package)
              server_methods = []
              for client_method in build_client_methods(named_model):
                  pattern_regex, regex_order = _path_regex(client_method.path, client_method.path_params)
                  binding_lines = _binding_lines(client_method, regex_order, support_fqn, named_model.language)
                  call_args = ', '.join((f'${p.name}' if named_model.language ==
                                        'php' else p.name for p in client_method.parameters))
                  server_methods.append(
                      ServerMethod(
                          name=client_method.name,
                          http_method=client_method.http_method,
                          response_type=client_method.response_type,
                          signature=client_method.signature,
                          description=client_method.description,
                          example_repr=client_method.example_repr,
                          pattern_name=f'PATTERN_{
                              client_method.name.upper()}',
                          pattern_regex=pattern_regex,
                          binding_lines=binding_lines,
                          call_args=call_args,
                          has_body=client_method.body_param is not None))
              return server_methods
      - id: server_class_name
        type: FunctionDef
        code: |-
          def server_class_name(named_model) -> str:
              """Deterministic name mirroring client_interface_name (e.g. 'ResponsesServer')."""
              top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
              fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
              return f'{fragment}Server'
```
/answer 8472-59 allow
ReasoningToken: 20
Text:
 Jetzt build_accessor, enum_constants/literal, string-literal-Funktionen, enum_raw_type, build_branches, discriminator/applies-Funktionen anpassen.
Text:
 Ich ersetze diese Funktionen vollständig durch die delegierenden Versionen.
Control Request:
```yaml
id: 8472-5a
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: build_server_methods
    source: |-
      def build_server_methods(named_model) -> list[ServerMethod]:
          """One ServerMethod per operation, same order as build_client_methods."""
          support_fqn = json_support_fqn(named_model.base_package)
          lang = get_language(named_model.language)
          server_methods = []
          for client_method in build_client_methods(named_model):
              pattern_regex, regex_order = _path_regex(client_method.path, client_method.path_params)
              binding_lines = lang.build_binding_lines(client_method, regex_order, support_fqn)
              call_args = ', '.join(lang.reference_expr(p.name) for p in client_method.parameters)
              server_methods.append(
                  ServerMethod(
                      name=client_method.name,
                      http_method=client_method.http_method,
                      response_type=client_method.response_type,
                      signature=client_method.signature,
                      description=client_method.description,
                      example_repr=client_method.example_repr,
                      pattern_name=f'PATTERN_{client_method.name.upper()}',
                      pattern_regex=pattern_regex,
                      binding_lines=binding_lines,
                      call_args=call_args,
                      has_body=client_method.body_param is not None))
          return server_methods
  reason: build_server_methods auf lang-Delegation umstellen
```
/answer 8472-5a allow
Control Request:
```yaml
id: 8472-5b
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  reason: server_context.py validieren
```
/answer 8472-5b allow
Control Request:
```yaml
id: 8472-5c
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  reason: Vollständige Struktur zur Sichtprüfung ansehen
```
/answer 8472-5c allow
Control Request:
```yaml
id: 8472-5d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: Ddm4e1|INaxgR
        signature: '''Builds the Jinja2 template context for the transport-agnostic abstract server …'
      - id: iSfUpq|O0VYpJ
        signature: import re
      - id: DwODB3|kYBX5M
        signature: "_PATH_PARAM = re.compile('\\\\{([^}]+)\\\\}')"
      - id: ServerMethod
        signature: "@dataclass(frozen=True) class ServerMethod:"
        docstring: One operation's abstract server method plus everything the generated `handle(Ht…
      - id: path_regex
        signature: "def _path_regex(path: str, path_params: tuple) -> tuple[str, tuple]:"
        docstring: Regex pattern string plus the path parameters in regex-group (path-appearance) …
      - id: build_server_methods
        signature: "def build_server_methods(named_model) -> list[ServerMethod]:"
        docstring: "One ServerMethod per operation, same order as build_client_methods."
      - id: server_class_name
        signature: "def server_class_name(named_model) -> str:"
        docstring: Deterministic name mirroring client_interface_name (e.g. 'ResponsesServer').
```
/answer 8472-5d allow
Control Request:
```yaml
id: 8472-5e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    old_text: "from jinja2 import Environment, FileSystemLoader, StrictUndefined"
    new_text: |-
      from jinja2 import Environment, FileSystemLoader, StrictUndefined

      from xy.cgen.lang import get_language
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    old_text: |-
      TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'
      EXTENSION = {'java': 'java', 'php': 'php'}
    new_text: TEMPLATES_ROOT = Path(__file__).resolve().parent.parent / 'templates'
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    old_text: |-
      def file_extension(language: str) -> str:
          return EXTENSION[language]
    new_text: |-
      def file_extension(language: str) -> str:
          return get_language(language).extension
  reason: "templates.py auf lang-Registry umstellen (EXTENSION, file_extension)"
```
/answer 8472-5e allow
Control Request:
```yaml
id: 8472-5f
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    id: to_php_fqn
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    id: to_namespace
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
    id: get_env
    source: |-
      @lru_cache(maxsize=None)
      def get_env(language: str) -> Environment:
          """One cached Jinja2 Environment per language, rooted at templates/<language>."""
          env = Environment(loader=FileSystemLoader(str(TEMPLATES_ROOT / language)), trim_blocks=True,
                            lstrip_blocks=True, keep_trailing_newline=True, undefined=StrictUndefined)
          env.filters.update(get_language(language).jinja_filters)
          return env
  reason: Jinja Filter-Registrierung auf lang.jinja_filters umstellen
```
/answer 8472-5f allow
Control Request:
```yaml
id: 8472-60
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/templates.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/base.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/java.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/php.py
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/lang/__init__.py
  reason: Alle geänderten/neuen Dateien validieren
```
/answer 8472-60 allow
Control Request:
```yaml
id: 8472-61
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap
    pattern: language ==|'php'|'java'
    include:
    - '*.py'
  reason: Verbleibende sprachbedingte if-Zweige oder Literale in emit/typemap finden
```
/answer 8472-61 allow
Control Request:
```yaml
id: 8472-62
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap
      matches:
      - path: model_context.py
        matches:
        - lineno: 108
          match: "def enum_constants(node, language: str='java') -> list[EnumConstant]:"
        - lineno: 127
          match: "def enum_raw_type(node, language: str='java') -> str:"
      - path: server_context.py
        matches:
        - lineno: 22
          match: "_PARSE_EXPR = {'java': _JAVA_PARSE_EXPR, 'php': _PHP_PARSE_EXPR}"
```
/answer 8472-62 allow
Control Request:
```yaml
id: 8472-63
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  text: _PARSE_EXPR
  reason: Verbleibende _PARSE_EXPR Zeile in server_context.py lokalisieren
```
/answer 8472-63 allow
Control Request:
```yaml
id: 8472-64
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
      nodes:
      - id: DwODB3|kYBX5M
        code: |-
          _PATH_PARAM = re.compile('\\{([^}]+)\\}')
          _PHP_PARSE_EXPR = {
              'int': '(int) {raw}',
              'float': '(float) {raw}',
              'bool': 'filter_var({raw}, FILTER_VALIDATE_BOOLEAN)',
              'string': '{raw}'}
          _PARSE_EXPR = {'java': _JAVA_PARSE_EXPR, 'php': _PHP_PARSE_EXPR}
```
/answer 8472-64 allow
Text:
 Jetzt build_branches anpassen: benutzt `READ_METHOD[language]` und `_discriminator_literal_expr`.
Control Request:
```yaml
id: 8472-65
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
    id: DwODB3|kYBX5M
    source: "_PATH_PARAM = re.compile('\\\\{([^}]+)\\\\}')"
  reason: Verbleibende PHP-Parse-Expr-Reste aus server_context.py entfernen
```
/answer 8472-65 allow
Control Request:
```yaml
id: 8472-66
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/server_context.py
  reason: server_context.py final validieren
```
/answer 8472-66 allow
Control Request:
```yaml
id: 8472-67
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen
    pattern: \.ya?ml$
  reason: Beispiel-OpenAPI-Schema für Testlauf finden
```
/answer 8472-67 allow
Control Request:
```yaml
id: 8472-68
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen
      entries: []
```
/answer 8472-68 allow
Control Request:
```yaml
id: 8472-69
toolName: bash
arguments:
  script: "find / -iname \"*.yaml\" -o -iname \"*.yml\" 2>/dev/null | grep -iv node_modules | xargs -I{} sh -c 'grep -l \"openapi\" \"{}\" 2>/dev/null' 2>/dev/null | head -5"
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Vorhandenes OpenAPI-Schema irgendwo im System suchen
```
/answer 8472-69 deny du kannst das hier nehmen `/home/user/xyan/xy.ai.workbench/ast-engines/openapi.yaml`
Control Request:
```yaml
id: 8472-6a
toolName: bash
arguments:
  script: |
    set -e
    export PYTHONPATH=src
    rm -rf /tmp/out_java /tmp/out_php
    python3 -m xy.cgen --schema /home/user/xyan/xy.ai.workbench/ast-engines/openapi.yaml --out /tmp/out_java --base-package com.example --language java
    echo "JAVA OK"
    python3 -m xy.cgen --schema /home/user/xyan/xy.ai.workbench/ast-engines/openapi.yaml --out /tmp/out_php --base-package com.example --language php
    echo "PHP OK"
    find /tmp/out_java -name "*.java" | wc -l
    find /tmp/out_php -name "*.php" | wc -l
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Codegenerator für beide Sprachen testweise ausführen
```
/answer 8472-6a allow
Control Request:
```yaml
id: 8472-6b
result:
  structuredContent:
    exit_code: 1
    stdout: ""
    stderr: |
      Traceback (most recent call last):
        File "<frozen runpy>", line 198, in _run_module_as_main
        File "<frozen runpy>", line 88, in _run_code
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/__main__.py", line 3, in <module>
          from xy.cgen.cli import main
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/cli.py", line 5, in <module>
          from xy.cgen.pipeline import run_pipeline
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/pipeline.py", line 3, in <module>
          from xy.cgen.emit import emit_code
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/__init__.py", line 3, in <module>
          from xy.cgen.emit.client_emit import emit_client
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_emit.py", line 8, in <module>
          from xy.cgen.emit.client_context import build_client_methods, client_interface_name
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/client_context.py", line 12, in <module>
          from xy.cgen.emit.io_context import request_root_node
        File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py", line 10, in <module>
          from xy.cgen.emit.model_context import READ_METHOD, classify
      ImportError: cannot import name 'READ_METHOD' from 'xy.cgen.emit.model_context' (/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py)
```
/answer 8472-6b allow
Control Request:
```yaml
id: 8472-6c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    ids:
    - iWMxMr|uSY70X
    - build_content_type_branches
    - build_response_setters
    - W0p47r|wrwzO7
  reason: io_context.py Nutzung von READ_METHOD finden
```
/answer 8472-6c allow
Control Request:
```yaml
id: 8472-6d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
      nodes:
      - id: iWMxMr|uSY70X
        type: imports
        code: |-
          from dataclasses import dataclass
          from xy.cgen.emit.model_context import READ_METHOD, classify
          from xy.cgen.model.nodes import RefNode
          from xy.cgen.naming.identifiers import content_type_short_name, to_pascal_case
          from xy.cgen.typemap import map_type
      - id: build_content_type_branches
        type: FunctionDef
        code: |-
          def build_content_type_branches(code_node, named_model) -> list:
              """One branch per ContentTypeView, in declaration order."""
              branches = []
              for content_type_view in code_node.content_types:
                  edge = content_type_view.body
                  category, primitive_type = classify(edge.target, named_model.named_nodes)
                  if category == 'unsupported':
                      continue
                  branches.append(ContentTypeBranch(short_name=to_pascal_case(content_type_short_name(content_type_view.content_type)),
                                                    content_type=content_type_view.content_type,
                                                    java_type=map_type(edge.target,
                                                                       named_model),
                                                    category=category,
                                                    read_method=READ_METHOD[named_model.language].get(primitive_type) if category in ('primitive',
                                                                                                                                      'enum') else None,
                                                    description=edge.description))
              return branches
      - id: build_response_setters
        type: FunctionDef
        code: |-
          def build_response_setters(response_node, named_model) -> list:
              """One ResponseSetter per (status code, content type) combination. The
              content-type suffix is only appended when a status code has more than one
              content type (declaration order, same as build_content_type_branches)."""
              setters = []
              for code_node in response_node.codes:
                  branches = build_content_type_branches(code_node, named_model)
                  multiple = len(branches) > 1
                  for branch in branches:
                      suffix = branch.short_name if multiple else ''
                      setters.append(
                          ResponseSetter(
                              status_code=code_node.status_code,
                              method_name=f'setCode{
                                  code_node.status_code}{suffix}',
                              content_type=branch.content_type,
                              java_type=branch.java_type,
                              category=branch.category,
                              node_factory_method=PRIMITIVE_NODE_FACTORY.get(
                                      branch.read_method) if branch.category == 'primitive' else None))
              return setters
      - id: W0p47r|wrwzO7
        type: statements
        code: |-
          '# read_method -> JsonNodeFactory factory-method name, for constructing a raw'
          '# JsonNode from a primitive Java value (setCode<NNN>... on the server side).'
          PRIMITIVE_NODE_FACTORY = {
              'asText': 'textNode',
              'asLong': 'numberNode',
              'asDouble': 'numberNode',
              'asBoolean': 'booleanNode'}
```
/answer 8472-6d allow
Text:
 Jetzt noch die verbleibenden `_TYPE_CHECK_METHOD`/`_PHP_TYPE_CHECK` Konstanten-Blöcke entfernen (dokumentiert bei node y7NDex|Qy7hVD) sowie überprüfen, was von `_discriminator_value_type`/`_const_value_for_branch` benutzt wird (die riefen `_literal` intern? nein).
Control Request:
```yaml
id: 8472-6e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    ids:
    - ContentTypeBranch
  reason: ContentTypeBranch Dataclass-Definition lesen
```
/answer 8472-6e allow
Control Request:
```yaml
id: 8472-6f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
      nodes:
      - id: ContentTypeBranch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ContentTypeBranch:
              """One content-type view of a CodeNode: is<Ct>()/get<Ct>(), discriminated by header."""
              short_name: str
              content_type: str
              java_type: str
              category: str
              read_method: str | None
              description: str | None
```
/answer 8472-6f allow
ReasoningToken: 107
Text:
 model_context.py fertig. Jetzt client_context.py.
Control Request:
```yaml
id: 8472-70
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    old_text: "from xy.cgen.emit.model_context import READ_METHOD, classify"
    new_text: |-
      from xy.cgen.emit.model_context import classify
      from xy.cgen.lang import get_language
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    old_text: |2-
          category: str
          read_method: str | None
          description: str | None
    new_text: |2-
          category: str
          read_method: str | None
          node_factory_method: str | None
          description: str | None
  reason: "ContentTypeBranch um node_factory_method erweitern, READ_METHOD Import entfernen"
```
/answer 8472-70 allow
Control Request:
```yaml
id: 8472-71
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    id: build_content_type_branches
    source: |-
      def build_content_type_branches(code_node, named_model) -> list:
          """One branch per ContentTypeView, in declaration order."""
          lang = get_language(named_model.language)
          branches = []
          for content_type_view in code_node.content_types:
              edge = content_type_view.body
              category, primitive_type = classify(edge.target, named_model.named_nodes)
              if category == 'unsupported':
                  continue
              has_read = category in ('primitive', 'enum')
              branches.append(ContentTypeBranch(
                  short_name=to_pascal_case(content_type_short_name(content_type_view.content_type)),
                  content_type=content_type_view.content_type,
                  java_type=map_type(edge.target, named_model),
                  category=category,
                  read_method=lang.read_method.get(primitive_type) if has_read else None,
                  node_factory_method=lang.factory_method.get(primitive_type) if category == 'primitive' else None,
                  description=edge.description))
          return branches
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    id: W0p47r|wrwzO7
    source: ""
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
    id: build_response_setters
    source: |-
      def build_response_setters(response_node, named_model) -> list:
          """One ResponseSetter per (status code, content type) combination. The
          content-type suffix is only appended when a status code has more than one
          content type (declaration order, same as build_content_type_branches)."""
          setters = []
          for code_node in response_node.codes:
              branches = build_content_type_branches(code_node, named_model)
              multiple = len(branches) > 1
              for branch in branches:
                  suffix = branch.short_name if multiple else ''
                  setters.append(
                      ResponseSetter(
                          status_code=code_node.status_code,
                          method_name=f'setCode{code_node.status_code}{suffix}',
                          content_type=branch.content_type,
                          java_type=branch.java_type,
                          category=branch.category,
                          node_factory_method=branch.node_factory_method))
          return setters
  reason: io_context.py auf lang-Delegation umstellen (READ_METHOD/PRIMITIVE_NODE_FACTORY entfernen)
```
/answer 8472-71 allow
Control Request:
```yaml
id: 8472-72
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/io_context.py
  reason: io_context.py validieren
```
/answer 8472-72 allow
Control Request:
```yaml
id: 8472-73
toolName: bash
arguments:
  script: |
    set -e
    export PYTHONPATH=src
    rm -rf /tmp/out_java /tmp/out_php
    python3 -m xy.cgen --schema /home/user/xyan/xy.ai.workbench/ast-engines/openapi.yaml --out /tmp/out_java --base-package com.example --language java
    echo "JAVA OK"
    python3 -m xy.cgen --schema /home/user/xyan/xy.ai.workbench/ast-engines/openapi.yaml --out /tmp/out_php --base-package com.example --language php
    echo "PHP OK"
    find /tmp/out_java -name "*.java" | wc -l
    find /tmp/out_php -name "*.php" | wc -l
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Codegenerator für beide Sprachen erneut testen
```
/answer 8472-73 allow
Control Request:
```yaml
id: 8472-74
result:
  structuredContent:
    stdout: |
      JAVA OK
      PHP OK
      49
      49
```
/answer 8472-74 allow
Control Request:
```yaml
id: 8472-75
toolName: bash
arguments:
  script: cd /home/user/xyan/xy.ai.workbench/codegen && git status --short 2>/dev/null | head -30; echo "---diff check via stash not needed---"
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: "Git-Status prüfen, ob Backup für Vergleich existiert"
```
/answer 8472-75 deny Was möchtest du vergleichen?
ReasoningToken: 182
Text:
 Gut. Jetzt server_context.py umbauen.
Control Request:
```yaml
id: 8472-76
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
    pattern: language ==|'php'|'java'
    include:
    - '*.py'
    exclude:
    - cli.py
    - '*/lang/*'
  reason: Finale Prüfung auf verbliebene Sprachbedingungen/-literale außerhalb lang/ und cli.py
```
/answer 8472-76 allow
Control Request:
```yaml
id: 8472-77
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
      matches:
      - path: naming/__init__.py
        matches:
        - lineno: 68
          match: "def assign_names(identified_model, base_package: str, language: str='java') -> NamedModel:"
      - path: emit/model_context.py
        matches:
        - lineno: 108
          match: "def enum_constants(node, language: str='java') -> list[EnumConstant]:"
        - lineno: 127
          match: "def enum_raw_type(node, language: str='java') -> str:"
      - path: config.py
        matches:
        - lineno: 11
          match: "language: str = 'java'"
      - path: lang/base.py
        matches:
        - lineno: 4
          match: to a hardcoded language name or an `if language == ...` branch. Adding a
      - path: lang/php.py
        matches:
        - lineno: 27
          match: name = 'php'
        - lineno: 28
          match: extension = 'php'
      - path: lang/__init__.py
        matches:
        - lineno: 9
          match: "LANGUAGES = {'java': JAVA, 'php': PHP}"
      - path: lang/java.py
        matches:
        - lineno: 26
          match: name = 'java'
        - lineno: 27
          match: extension = 'java'
```
/answer 8472-77 allow
Control Request:
```yaml
id: 8472-78
toolName: bash
arguments:
  script: rm -rf /tmp/out_java /tmp/out_php
  cwd: /home/user/xyan/xy.ai.workbench/codegen
  reason: Temporäre Testausgaben aufräumen
```
/answer 8472-78 allow
Control Request:
```yaml
id: 8472-79
result:
  structuredContent:
    result: success
```
/answer 8472-79 allow
Result Stats: id=c37f561e-5bde-4ca8-8db7-a039432c33f8, total: 155813, in: 249, out: 46653, reason: 820, read: 4247775, write: 108911
Text:
 QLdKL0|JYEPq1 wurde wohl schon durch den vorherigen Import-Edit-Block entfernt/gemerged. Jetzt build_server_methods anpassen.
Text:
 Sehr gut. Jetzt templates.py.
Text:
 Jetzt `to_php_fqn`/`to_namespace`/`get_env` entfernen bzw. anpassen.
Text:
 Jetzt alles validieren und nach verbliebenen `if language ==` / Sprachliteralen suchen.
ReasoningToken: 114
Text:
 enum_constants/enum_raw_type Defaults `'java'` sind ok (Default-Werte, keine Bedingungen). Jetzt validieren und CLI-Test durchführen.
Text:
 Jetzt teste ich den Codegenerator mit einem Beispiel-Schema für beide Sprachen.
Text:
 io_context.py hat noch eine READ_METHOD-Nutzung, die ich übersehen hatte.
ReasoningToken: 569
Text:
 Beide Sprachen generieren erfolgreich (je 49 Dateien). Jetzt vergleiche ich das Ergebnis mit einem Git-Diff-Backup-Check, falls vorhanden, und suche final nach übrig gebliebenen Sprachfragmenten im gesamten emit/typemap-Baum.
ReasoningToken: 137
Text:
 Alle verbleibenden Treffer sind nur Default-Parameterwerte (`'java'`) oder Registry-Definitionen im `lang`-Package – keine `if`-Sprachbedingungen mehr außerhalb von `lang/`. Cache im pipeline.py testen (dass `get_env` lru_cache nicht das Filter-Update mit sich selbst kollidieren lässt) ist bereits durch den erfolgreichen Testlauf bestätigt.
Fertig. Zusammenfassung der Refaktorierung:

- Neues Package `lang/`: `base.py` definiert die ABC `LanguageSupport` (Typtabellen + Methoden wie `literal`, `type_hint`, `path_url_expression`, `discriminator_literal_expr`, `applies_*`, `build_binding_lines`, `jinja_filters`, ...). `java.py`/`php.py` implementieren je eine Singleton-Instanz (`JAVA`/`PHP`) mit allen bisher verstreuten Konstanten und `if language == 'php'`-Zweigen. `__init__.py` stellt `get_language(name)` als Registry bereit.
- `emit/typemap`, `emit/model_context.py`, `emit/client_context.py`, `emit/server_context.py`, `emit/io_context.py`, `emit/templates.py` rufen jetzt ausschließlich `get_language(...)` auf; alle `if language == 'php'/'java'`-Verzweigungen sowie doppelten Literal-/Typ-Tabellen wurden entfernt.
- CLI-Testlauf für `--language java` und `--language php` erfolgreich (je 49 generierte Dateien).
