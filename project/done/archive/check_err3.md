Prüfe den Fehler:
Traceback (most recent call last):
  File "<frozen runpy>", line 198, in _run_module_as_main
  File "<frozen runpy>", line 88, in _run_code
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/__main__.py", line 6, in <module>
    main()
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/cli.py", line 25, in main
    run_pipeline(config)
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/pipeline.py", line 17, in run_pipeline
    emit_code(named_model, config.output_dir)
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/__init__.py", line 14, in emit_code
    emit_model(model, writer)
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py", line 34, in emit_model
    content = _render(node, name, model, id(node) in request_root_ids)
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py", line 40, in _render
    return _render_object(node, name, model, is_request_root)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py", line 58, in _render_object
    accessors = [a for a in (build_accessor(edge.label, edge, model) for edge in node.properties) if a is not None]
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_emit.py", line 58, in <genexpr>
    accessors = [a for a in (build_accessor(edge.label, edge, model) for edge in node.properties) if a is not None]
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py", line 91, in build_accessor
    java_type=map_type(
              ^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py", line 24, in map_type
    return _map_primitive(node, named_model.language)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py", line 37, in _map_primitive
    raise ValueError("'null' has no standalone type -- resolve via the enclosing composition view")
ValueError: 'null' has no standalone type -- resolve via the enclosing composition view
SystemInit:  id=eb15ca1a-a105-409f-a721-2c3fac127270, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den relevanten Code an.
Control Request:
```yaml
id: e715-69
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
  reason: "Typemap-Modul lesen, um die Stelle des ValueError zu verstehen"
```
/answer e715-69 allow
Control Request:
```yaml
id: e715-6a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
      nodes:
      - id: UKZFWe|iGaZar
        signature: "'Maps IR nodes to the Java type used at their use-site (getter/setter, list\\nel…"
      - id: MAsCzl|TkBLfF
        signature: from xy.cgen.lang import get_language
      - id: Wu9MuW|ZXSQiA
        signature: '''# Node kinds whose type is the generated class assigned by naming.'''
      - id: map_type
        signature: "def map_type(node, named_model) -> str:"
        docstring: Resolve the target-language type for a given IR node. `named_model` supplies th…
      - id: map_primitive
        signature: "def _map_primitive(node: PrimitiveNode, language: str) -> str:"
```
/answer e715-6a allow
Control Request:
```yaml
id: e715-6b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  name: build_accessor
  reason: "build_accessor ansehen, das map_type aufruft"
```
/answer e715-6b allow
Control Request:
```yaml
id: e715-6c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: build_accessor
        code: |-
          def build_accessor(label: str, edge, named_model) -> Accessor | None:
              """Build the accessor context for one edge, or None if it has no view."""
              category, primitive_type = classify(edge.target, named_model.named_nodes)
              if category == 'unsupported':
                  return None
              lang = get_language(named_model.language)
              has_read = category in ('primitive', 'enum')
              return Accessor(
                  label=label,
                  name=to_pascal_case(
                      property_accessor_name(label)),
                  java_type=map_type(
                      edge.target,
                      named_model),
                  category=category,
                  read_method=lang.read_method.get(primitive_type) if has_read else None,
                  factory_method=lang.factory_method.get(primitive_type) if has_read else None,
                  description=edge.description,
                  example_repr=None if edge.example is MISSING else repr(
                      edge.example))
```
/answer e715-6c allow
Control Request:
```yaml
id: e715-6d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
    ids:
    - map_type
    - map_primitive
  reason: Implementierung von map_type und _map_primitive lesen
```
/answer e715-6d allow
Control Request:
```yaml
id: e715-6e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/typemap/__init__.py
      nodes:
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
                  return get_language(named_model.language).any_dictionary_type
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
              return get_language(language).primitive_type[node.primitive_type]
```
/answer e715-6e allow
Control Request:
```yaml
id: e715-6f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
    pattern: def classify|'null'|"null"
    include:
    - '*.py'
    exclude:
    - __pycache__/**
    limit: 30
  reason: "Finden, wie classify null-Primitive behandelt"
```
/answer e715-6f allow
Control Request:
```yaml
id: e715-70
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen
      matches:
      - path: naming/names.py
        matches:
        - "'null': 'Null'}"
      - path: typemap/__init__.py
        matches:
        - "if node.primitive_type == 'null':"
        - raise ValueError("'null' has no standalone type -- resolve via the enclosing composition view")
      - path: emit/model_context.py
        matches:
        - "def classify(node, named_nodes: dict) -> tuple[str, str | None]:"
        - '"# None for the ''null'' branch (no getter)"'
        - '"# ''primitive'' | ''enum'' | ''any_dictionary'' | ''composition'' | ''complex'' | ''null''"'
        - "if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':"
        - "category='null',"
        - "if resolved.primitive_type == 'null':"
      - path: model/build.py
        matches:
        - "PRIMITIVE_TYPES = ('string', 'integer', 'number', 'boolean', 'null')"
        - return 'null'
        - "\"\"\"`type: [X, 'null', ...]` shorthand -> anyOf of single-typed variants.\"\"\""
```
/answer e715-70 allow
Control Request:
```yaml
id: e715-71
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
  name: classify
  reason: "classify lesen, um die Kategorie 'null' zu prüfen"
```
/answer e715-71 allow
Control Request:
```yaml
id: e715-72
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
      nodes:
      - id: classify
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
```
/answer e715-72 allow
ReasoningToken: 127
Control Request:
```yaml
id: e715-73
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
    pattern: '''null''|category'
    include:
    - model_context.py
    exclude:
    - __pycache__/**
    limit: 30
  intent: editing
  reason: Kontext der null-Behandlung in model_context.py
```
/answer e715-73 allow
Control Request:
```yaml
id: e715-74
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit
      matches:
      - path: model_context.py
        nodes:
        - id: Accessor.wTgFnL|8qU8F5
          matches:
          - "category: str"
        - id: classify.1En0AI|PwAYc5
          matches:
          - "\"\"\"-> (category, primitive_type). primitive_type is set for primitive/enum only."
        - id: build_accessor.WqecQH|Sx8jEp
          matches:
          - "category, primitive_type = classify(edge.target, named_model.named_nodes)"
          - "if category == 'unsupported':"
          - "has_read = category in ('primitive', 'enum')"
        - id: build_accessor.Q6OV1A|AOctA5
          matches:
          - "category=category,"
        - id: Branch.xFApAo|pIwJo0
          matches:
          - '"# None for the ''null'' branch (no getter)"'
          - '"# ''primitive'' | ''enum'' | ''any_dictionary'' | ''composition'' | ''complex'' | ''null''"'
          - "category: str"
        - id: build_branches.L9iStE|tjU0dd
          matches:
          - "if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':"
          - "category='null',"
          - "category, primitive_type = classify(target, named_nodes)"
          - "category=category,"
          - read_method=lang.read_method.get(primitive_type) if category in (
        - id: structural_applies_expr.DOR1E3|IquzeX
          matches:
          - "if resolved.primitive_type == 'null':"
```
/answer e715-74 allow
ReasoningToken: 70
Control Request:
```yaml
id: e715-75
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/src/xy/cgen/emit/model_context.py
    id: build_accessor
    old_text: |2-
          if category == 'unsupported':
              return None
    new_text: |2-
          if category == 'unsupported' or primitive_type == 'null':
              return None
  reason: "Property mit reinem null-Typ überspringen, statt map_type aufzurufen"
```
/answer e715-75 allow
Result Stats: id=eb15ca1a-a105-409f-a721-2c3fac127270, total: 8762, in: 12, out: 2009, reason: 59, read: 50963, write: 6741, start: 1793572459675, end: 1793572464909, time: 5172, total: 21766, io: 0, control: 62, sent: 0, recv: 103
ReasoningToken: 59
**Ursache:** Eine Property des Objekts hat direkt den Typ `null`, zum Beispiel `type: 'null'` oder `const: null`. `build_accessor` in `emit/model_context.py` hat dafür trotzdem `map_type` aufgerufen. `classify` liefert dafür `('primitive', 'null')`. Nur `build_branches` behandelt `null` bei Compositions (`anyOf`/`oneOf`) gesondert. `_map_primitive` lehnt `null` bewusst ab, weil `null` nur über die umgebende Composition aufgelöst werden darf. Deshalb kam der `ValueError`.

**Fix:** In `build_accessor` gebe ich jetzt `None` zurück, wenn `primitive_type == 'null'`. Die Property bekommt dann keinen Getter, so wie bei `unsupported`. Das passt zum Kommentar im Code: "None for the 'null' branch (no getter)".

Ich habe den Lauf nicht wiederholt. Bitte starte die Pipeline noch einmal.

Die Prüfung gilt auch für ein Enum, dessen `primitive_type` `null` ist. Auch das erzeugt jetzt keinen Getter, was bei einem reinen null-Enum sinnvoll ist.
