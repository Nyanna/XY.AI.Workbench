# Kontext (discovered) — nicht erneut ermitteln

Diese Datei bündelt Umgebungs-/Repo-Fakten, die pro Segment sonst neu discovered würden.
Bei Widerspruch gilt `00_overview.md`.

## Pfade
- Codegen-Projekt: `/home/user/xyan/xy.ai.workbench/codegen`
- Package-Root: `/home/user/xyan/xy.ai.workbench/codegen/cgen`
- Beispielschema (Acceptance-Fixture): `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
  - 1 Operation: `POST /responses`, `operationId=createResponse`
  - `components.schemas`: 85 Einträge
  - `components.responses`: `InferenceRateLimited`, `InferenceServiceUnavailable` (je → `ErrorResponse`)

## Projekt-Setup
- `pyproject.toml`: `name=cgen`, `requires-python>=3.10`, deps `pyyaml>=6.0`, `jinja2>=3.1`; setuptools, Packages via `include=["cgen*"]`.
- Kein Testframework eingebunden. **Vorerst keine Tests** (siehe `00_overview.md`) — Acceptance manuell/ad-hoc verifizieren, keine Testdateien committen.
- pytest ist in der Umgebung zwar installierbar/vorhanden, aber nicht Teil des Plans.
- `PYTHONDONTWRITEBYTECODE=1` ist in `~/.bashrc` gesetzt. **Kein `__pycache__` im Repo ablegen** — falls doch entstanden, vor Abschluss löschen (`find <projekt> -name __pycache__ -exec rm -rf {} +`).

## Package-Layout (Stand nach Segment 01+02)
```
cgen/
  __init__.py            # leer
  __main__.py            # `python -m cgen` -> cli.main()
  cli.py                 # argparse: --schema, --out, --base-package (default xy.api.codegen) -> Config -> run_pipeline
  config.py              # Config(frozen dataclass): input_schema: Path, output_dir: Path, base_package: str
  pipeline.py            # run_pipeline(config): ingest -> model -> identity -> naming -> emit (Reihenfolge fix)
  ingest/                # SEGMENT 02 — implementiert
    __init__.py           # IngestedSchema(ref_index, operations); ingest_schema(config)
    loader.py             # load_yaml(path) -> dict (reines yaml.safe_load)
    refindex.py           # RefIndex; build_ref_index(document); Keys: '#/components/schemas/<Name>', '#/components/responses/<Name>'
    operations.py         # Operation, Parameter (frozen dataclasses); extract_operations(document, ref_index)
  model/                  # SEGMENT 03 — implementiert (siehe Segment-03-Ergebnis unten)
  identity/               # SEGMENT 04 — implementiert (siehe Segment-04-Ergebnis unten)
  naming/                  # SEGMENT 05 — implementiert
    __init__.py            # assign_names(identified_model, base_package) -> NamedModel; NodeName
    identifiers.py          # sanitize_identifier, to_pascal_case/camel_case, class_identifier, property_accessor_name, content_type_short_name
    paths.py                 # path_to_package_segments, path_to_class_fragment, method_to_class_fragment
    traverse.py               # iter_child_edges(node) -- read-only child-edge iteration
    names.py                  # derive_class_names(identified_model) -> ClassNames (pre-package class names)
    packages.py                # collect_named_references, named_package/site_package/response_root_package/anonymous_package
  typemap/__init__.py     # map_type(node) -> NotImplementedError (Segment 06, von naming/emit genutzt)
  emit/
    __init__.py            # emit_code(model, output_dir) -> NotImplementedError (Segment 07)
    model_emit.py           # emit_model(model, writer) -> NotImplementedError
    io_emit.py               # emit_io(model, writer) -> NotImplementedError
    client_emit.py           # emit_client(model, writer) -> NotImplementedError
    writer.py                # FileWriter(output_dir).write(relative_path, content) -> NotImplementedError
  templates/.gitkeep       # Ablageort für Jinja2-Templates (noch leer)
```
Alle noch nicht umgesetzten Funktionen sind Stubs, die `NotImplementedError` werfen — das ist erwartet und kein Bug.

## Segment-02-Ergebnis (Ingest) — API für Folgesegmente
- `ingest_schema(config: Config) -> IngestedSchema`
- `IngestedSchema.ref_index: RefIndex` — `.get(ref)`, `.add(ref, node)`, `__contains__`, `.schema_refs()`
- `IngestedSchema.operations: list[Operation]`
- `Operation`: `path, method, operation_id, request_body: dict|None, responses: dict[str, dict], parameters: list[Parameter]`
  - `request_body`/`responses`-Werte sind rohe Schema-Knoten (inline dict **oder** `{'$ref': '...'}`), `$ref` bleibt atomar/unexpanded.
  - `components.responses`-Refs sind bereits auf `content.application/json.schema` reduziert (headers/examples verworfen).
- `Parameter`: `name, location('path'|'query'), required, schema` — nur path/query, gehört NICHT in den Body-Baum.
- Nur `application/json`-Content wird berücksichtigt; andere Content-Types werden verworfen (Kurzformen: Segment 09).
- `security`/`servers`/`info` werden geladen, aber nicht in `IngestedSchema` übernommen.

## Konventionen (verbindlich, aus `00_overview.md`)
- Kommentare: knapp, signifikant, **englisch**, **keine Referenzen** auf den Umsetzungsplan/Segmentnummern/„Requirements" im Code.
- `$ref` nie inline expandieren (I4); benannte `components.schemas` nie mergen/dedup (I5); Metadaten nie Teil der Identität (I3).
- Determinismus/Sortierung (I7) erst ab den Segmenten, die Reihenfolge/Namen festlegen (04/05 ff.) — in Ingest nicht relevant, da nur Rohdaten extrahiert werden.

## Segment-03-Ergebnis (IR / Node Model) — API für Folgesegmente
- `cgen/model/nodes.py`: Node-Kinds (`RefNode`, `PrimitiveNode`, `EnumNode`, `ObjectNode`, `ListNode`,
  `DictionaryNode`, `AnyDictionaryNode`, `CompositionNode`, `UnsupportedNode`) + Transport-Root-Kinds
  (`RequestNode`, `ResponseNode`, `CodeNode`, `ContentTypeView`) + `Edge` (label, target, description,
  example, default — `example`/`default` nutzen Sentinel `MISSING` statt `None`, um "fehlt" von "explizit
  null" zu unterscheiden) + `Discriminator` (property_name, mapping — roh, unresolved, Auflösung erst
  Segment 08). Alle Node-Klassen tragen ein `kind`-ClassVar; keine Metadaten am Knoten (I3).
- `cgen/model/build.py`: `build_model(ingested: IngestedSchema) -> Model`.
  - `Model(named_nodes: dict[str, Node], operations: tuple[OperationModel, ...])`.
  - `OperationModel(operation: Operation, request: RequestNode|None, response: ResponseNode)`.
  - `named_nodes`: ein Node pro `components.schemas`-Eintrag (Key = Original-Name ohne Präfix), `$ref`
    wird nie expandiert (`RefNode(name=...)`), dadurch terminiert der Aufbau auch bei zyklischen Schemas.
  - `build_node(raw: dict) -> Node` / `build_edge(label, raw) -> Edge` sind die zentralen Dispatcher —
    von Folgesegmenten nicht direkt gebraucht, aber exportiert falls nötig.
  - `type: [X, 'null', ...]`-Kurzform wird in eine äquivalente `CompositionNode(anyOf, ...)` übersetzt
    (nicht kollabiert, D3-konform).
  - `additionalProperties: true|{}` sowie bare `type: object` ohne `properties`/`additionalProperties`
    → `AnyDictionaryNode()` (D9). `additionalProperties: {schema}` (bei fehlenden `properties`) →
    `DictionaryNode(value=Edge)`.
  - `required`, das kein vorhandenes Property nennt, wird gefiltert (D8).
  - `not` → `UnsupportedNode(reason="not")` (D10); im deepseek-Fixture nicht vorhanden.
  - Response-Codes/ContentTypes behalten Dokument-Reihenfolge (kein Sortieren — das ist erst 04/05 Thema).
- `cgen/model/__init__.py` exportiert alle o.g. Klassen + `build_model`.
- Verifiziert gegen `deepseek.filtered.yaml`: 85 `named_nodes`, 1 Operation; `CreateResponse`/`Response`
  → allOf mit 2 RefNodes + inline `ObjectNode` (Part3); `InputContent` → oneOf, discriminator `type`,
  3 RefNodes; `ToolsArray` → `ListNode(element=Ref(Tool))`; Request-Root → `RefNode(CreateResponse)`
  (D4); Response-Codes `200→Response, 429→ErrorResponse, 503→ErrorResponse` (via bereits von Segment 02
  aufgelöste `components.responses`).

## Segment-04-Ergebnis (Identity/Dedup) — API für Folgesegmente
- `cgen/identity/fingerprint.py`: `fingerprint_of(node, child_fingerprints: dict[id->str]) -> str`
  (sha256 über kanonische Tupel-Form). Deckt `RefNode..UnsupportedNode` ab (Root-/Transport-Kinds nicht).
  RefNode-Fingerprint = nur `node.name` (Ziel NIE expandiert, I4). Objekt-Properties nach Label sortiert;
  Union-Branches/Listen-Elemente **positionsbelassen** (nie sortiert). Enum-Werte als Set behandelt
  (sortiert nach `(type_name, repr)`). Discriminator (propertyName+mapping) fließt in CompositionNode-FP ein.
- `cgen/identity/dedup.py`: `DedupContext` + `canonicalize_model(model) -> (named_nodes, operations, ctx)`.
  Baut den Baum bottom-up neu auf; anonyme Knoten mit gleichem Fingerprint werden zum selben Python-Objekt
  (`is`-Vergleich zeigt Sharing); benannte `components.schemas`-Knoten werden nie in die anonyme
  Canonical-Registry aufgenommen/daraus bedient — bleiben immer eigenständige Objekte, auch bei
  Fingerprint-Gleichheit mit einem anderen benannten oder anonymen Knoten (I5, z.B.
  `FunctionCallOutputStatusEnum`/`FunctionCallItemStatus`: gleicher FP, zwei Objekte).
- `cgen/identity/__init__.py`: `compute_identity(model: Model) -> IdentifiedModel`.
  - `IdentifiedModel(named_nodes: dict[str, Node], operations: tuple[OperationModel, ...], fingerprints: dict[id(node)->str])`
    + `.fingerprint_of(node) -> str|None`.
  - `fingerprints` deckt nur den geteilten Typ-Graphen ab (nicht `RequestNode`/`ResponseNode`/`CodeNode`/
    `ContentTypeView` — diese werden nie dedupliziert und tragen keinen Fingerprint).
  - Determinismus verifiziert: zwei Läufe erzeugen identische sortierte Fingerprint-Mengen.
- Verifiziert gegen `deepseek.filtered.yaml`: 85 `named_nodes` (unverändert), 259 fingerprinted Knoten;
  einzelnes inline `anyOf[string,null]` → 1 Fingerprint/Objekt; einzelnes inline Status-Enum
  `{in_progress,completed,incomplete}` → 1 Objekt (die zwei gleichwertigen *benannten* Enums bleiben
  laut I5 getrennt); `ImageDetail`/`DetailEnum` (beide benannt) → gleicher Fingerprint, zwei Objekte.

## Segment-05-Ergebnis (Naming/Packaging) — API für Folgesegmente
- `cgen/naming/identifiers.py`: `sanitize_identifier`, `to_pascal_case`, `to_camel_case`,
  `class_identifier` (PascalCase + sanitize), `property_accessor_name` (snake_case→camelCase,
  für Getter/Setter-Namen), `content_type_short_name` ('application/json'→'json').
- `cgen/naming/paths.py`: `path_to_package_segments`, `path_to_class_fragment`,
  `method_to_class_fragment` — rein strukturelle Ableitung aus `path`/`method`.
- `cgen/naming/traverse.py`: `iter_child_edges(node)` — kind-agnostisches Child-Edge-Iterieren
  (Object/List/Dictionary/Composition), von naming intern genutzt (kein Rebuild wie in dedup.py).
- `cgen/naming/names.py`: `derive_class_names(identified_model) -> ClassNames` — reine
  Namensableitung (noch ohne Package). Kontext-Threading: der "nächste einschließende
  Klassenname" wird nur bei ObjectNode/anonymen Komposition-Branches weitergereicht
  (`<Kontext>Part<n>`, n = 1-basierter Branch-/Property-Index); List/Dictionary/Enum/
  AnyDictionary sind kontextunabhängig strukturell benannt. RequestNode: `$ref`-Body →
  `force_name=None` (Name kommt vom benannten Ziel, D4); inline → `force_name=<Path>Request<Method>`.
  CodeNode/ContentTypeView: immer synthetisch `<Path>ResponseCode<code><Ct>`, Payload darunter
  wird unabhängig (mit diesem Namen als Kontext) benannt.
- `cgen/naming/packages.py`: `collect_named_references(identified_model)` — traversiert alle
  Operationen (inkl. transitiv durch benannte Schemas, Zyklenschutz über besuchte Namen) und
  liefert `paths_by_name` (Menge referenzierender Pfade je Key), `first_site` (erster Fundort:
  Path/Seite/Methode/Code/Content-Type) und `methods_by_path`. `named_package(...)`,
  `site_package(...)`, `response_root_package(...)`, `anonymous_package(node, base)` (Kind-Bucket:
  Enum→enums, List→lists, Object→objects, Composition→operators, (Any)Dictionary→dictionaries;
  `UnsupportedNode` → `None`, kein Package/keine Klasse, D10).
- `cgen/naming/__init__.py`: `assign_names(identified_model, base_package) -> NamedModel`.
  - `NamedModel(named_nodes, operations, fingerprints, base_package, names: dict[id(node)->NodeName])`
    + `.name_of(node)`, `.name_of_ref(schema_key)`. `NodeName(package, class_name)` + `.fqn`.
  - **Package-Regel für benannte Schemas:** `ListNode`/`EnumNode`/`DictionaryNode`/
    `AnyDictionaryNode` gehen **immer** in den Kind-Bucket (`.lists`/`.enums`/`.dictionaries`),
    unabhängig von der Zahl referenzierender Pfade (Beispiel: `ToolsArray`, benannt aber
    `type: array` → `.lists`) — ihre Identität ist strukturell wie bei anonymen Pendants.
    `ObjectNode`/`CompositionNode` (inkl. `CreateResponse`, `Response`) folgen weiter der
    Pfad-Regel (`.components` bei >1 referenzierendem Pfad, sonst dessen request/response-Package;
    0 referenzierende Pfade fällt defensiv auf `.components`, im Fixture nicht vorgekommen).
  - **Kollisionsauflösung (D6):** pro `(package, class_name)`-Bucket nach Fingerprint
    (`identified_model.fingerprint_of`, für Transport-Roots ein synthetischer Sortier-Key aus
    Pfad/Methode/Code) sortiert, dann `Name`, `Name2`, `Name3` vergeben — nie nach Discovery-Reihenfolge.
  - `RequestNode` (D4, `$ref`-Body) und `ContentTypeView` (immer, da mit `CodeNode` identisch
    benannt) werden NICHT eigenständig in die Kollisionsauflösung gegeben, sondern spiegeln
    (`names[id(x)] = names[id(sibling)]`) das bereits aufgelöste Ergebnis ihres Gegenstücks
    (benanntes Schema bzw. `CodeNode`) — sie sind dieselbe Klasse, keine Kollision.
- Verifiziert gegen `deepseek.filtered.yaml`: 177 benannte Klassen-Einträge; `CreateResponse` →
  `.request.responses.post.json`; `Response`(named)/`ResponsesResponseCode200Json` →
  `.response.responses.code200.json`; `ResponsesResponse` (root) → `.response.responses`;
  `ToolsArray` → `.lists`; geteiltes `anyOf[string,null]` → `.operators`/`AnyOfStringNull`;
  `_MisalignmentErrorType` → Klasse `MisalignmentErrorType`; Kollisionen wie
  `AnyOfFunctionToolPart1Null`/`AnyOfFunctionToolPart1Null2` korrekt nummeriert; zwei Läufe
  liefern identische sortierte FQN-Mengen (177 Einträge, byte-identisch).

## Segment-06-Ergebnis (Type Mapping) — API für Folgesegmente
- `cgen/typemap/__init__.py`: `map_type(node, named_model) -> str` — liefert den Java-Typ
  für den Verwendungsort (Getter/Setter/Element/Value). Kein eigener Node-Rebuild, reine
  Dispatch-Funktion, kind-basiert:
  - `PrimitiveNode` (`string|integer|number|boolean`) → Boxed-Typ (`String`/`Long`/`Double`/
    `Boolean`, `JAVA_PRIMITIVE_TYPE`-Dict). `primitive_type == "null"` → `ValueError`
    (kein eigenständiger Typ, D3: Auflösung ist Getter-Sache der umschließenden
    Komposition-View, Segment 08).
  - `AnyDictionaryNode` → `ANY_DICTIONARY_JAVA_TYPE` = `"com.fasterxml.jackson.databind.JsonNode"`
    (FQN-String, D9) — **überschreibt** die von Segment 05 vergebene Klasse/Package
    (`.dictionaries`/`AnyDictionary`); diese Namensvergabe bleibt für Identität/Bucketing
    bestehen, wird aber für den tatsächlichen Java-Typ am Nutzungsort nicht verwendet.
  - `RefNode` → `named_model.name_of_ref(node.name).fqn` (Ziel nie expandiert, nur der
    Name des benannten Schemas wird aufgelöst, I4-konform).
  - `EnumNode`/`ListNode`/`DictionaryNode`/`ObjectNode`/`CompositionNode` → generierte
    Klasse via `named_model.name_of(node).fqn`.
  - `UnsupportedNode` → `ValueError` (D10, keine View/kein Typ).
  - Alle anderen Kinds (unbekannt/Transport-Root-Kinds ohne eigenen Typ) → `TypeError`.
  - Rückgabewert ist immer ein String: package-qualifizierter Name (FQN) für generierte
    Klassen/JsonNode, unqualifizierter `java.lang`-Name für Primitives.
- Verifiziert gegen `deepseek.filtered.yaml` (ad-hoc, über die volle Pipeline bis
  `assign_names`): `Response.created_at`(number)→`Double`, `Usage.input_tokens`(integer,
  über `Response.usage`-Ref aufgelöst)→`Long`, `Response.parallel_tool_calls`(boolean)→
  `Boolean`; `FunctionTool.parameters` ist `anyOf[additionalProperties:{}, null]` — der
  `AnyDictionaryNode`-Zweig (nicht die umschließende `AnyOf…Null`-Komposition-Property
  selbst) → `com.fasterxml.jackson.databind.JsonNode`.

## Segment-07-Ergebnis (Model-Emission) — API für Folgesegmente
- `cgen/emit/model_context.py`: `Accessor` (label, name, java_type, category, read_method,
  factory_method, description, example_repr) — der Render-Kontext pro Kind-Kante.
  `classify(node, named_nodes) -> (category, primitive_type)` folgt eine `RefNode`-Kette bis
  zum strukturellen Zielknoten (nur zur Codegen-Strategiewahl, keine IR-Mutation/-Expansion,
  I4 bleibt gewahrt) und liefert `'primitive'|'enum'|'any_dictionary'|'complex'|'unsupported'`.
  `build_accessor(label, edge, named_model) -> Accessor|None` (None bei `'unsupported'`, D10 —
  keine Accessor-Methode wird generiert). `enum_constants(node)`/`enum_raw_type(node)` für
  EnumNode-Klassen (siehe unten).
- `cgen/emit/model_emit.py`: `emit_model(named_model, writer)` — implementiert. Traversiert den
  gesamten erreichbaren Graphen (alle `named_nodes` + jede Operation Request/Response-Baum,
  `RefNode` wird zur Traversierung dereferenziert, nie aber strukturell gemerged) und rendert
  pro besuchtem `ObjectNode|ListNode|DictionaryNode|EnumNode|AnyDictionaryNode` genau eine
  `.java`-Datei unter `named_model.name_of(node)` (package→Verzeichnis, `class_name.java`).
  `CompositionNode` wird bewusst NICHT gerendert (Segment 08); `UnsupportedNode` nie (D10).
  **Wichtig:** `AnyDictionaryNode` bekommt entgegen der ersten Intuition doch eine generierte
  Klasse (Proxy mit `get/put/remove/keys/containsKey`, raw `JsonNode`-Werte) — `map_type`
  (Segment 06) kollabiert `AnyDictionaryNode` nur am **direkten** Verwendungsort zu
  `JsonNode`; ein benanntes `additionalProperties:true`-Schema, das über einen `RefNode`
  referenziert wird (z.B. `ResponseFormatJsonSchemaSchema`), liefert dagegen die FQN der
  generierten Klasse selbst (RefNode-Zweig in `map_type` dereferenziert die Zielart nicht) —
  ohne diese Klasse kompiliert der generierte Code nicht.
- Jede generierte Proxy-Klasse (Object/List/Dictionary/AnyDictionary) hält **nur** ein
  `private final JsonNode node` Feld + `public JsonNode node()`-Accessor (Konvention für
  Eltern-Setter, die ein Kind-Objekt anhängen: `((ObjectNode) node).set(label, value.node())`).
  Primitive/Enum-Felder lesen/schreiben direkt auf dem gebundenen Node (kein Kind-Objekt).
  `EnumNode` wird als **echtes Java-`enum`** gerendert (Konstanten `rawValue()`/statisches
  `fromValue(raw)`), nicht als JsonNode-Proxy — es hat keine Kind-Kanten, der Wert ist terminal.
  Getter für explizit-`null` vs. absent unterscheiden hier (noch) nicht (das ist eine
  Komposition-View-Aufgabe, Segment 08, D3); beide liefern schlicht `null`.
  Java-Typen werden immer als FQN inline verwendet (keine `import`-Verwaltung nötig/vorhanden).
- Mixed-Lists (`ListNode(mixed=True)`, Tuple-Validation) werden unterstützt
  (`list_mixed.java.jinja`, `getElementN()/setElementN()`, Schreiben über
  `JsonNodeFactory.instance.<textNode|numberNode|booleanNode>(...)`, da `ArrayNode` anders als
  `ObjectNode` keine typisierten `set(int,...)`-Overloads hat) — im deepseek-Fixture nicht
  vorhanden, aber strukturell abgedeckt.
- `cgen/emit/writer.py`: `FileWriter.write(relative_path, content)` — implementiert (legt
  Verzeichnisse an, schreibt UTF-8).
- `cgen/emit/__init__.py`: `emit_code` ruft jetzt `emit_model` (fertig), dann `emit_io`/
  `emit_client` (weiterhin `NotImplementedError`, Segmente 09/10).
- Templates: `cgen/templates/model/{object,list,list_mixed,dictionary,enum,any_dictionary}.java.jinja`
  + `macros.jinja` (Javadoc aus Edge-`description`/`example`, I3-konform: Metadaten nur an
  Getter/Setter, nie an der Klasse).
- Verifiziert gegen `deepseek.filtered.yaml`: 121 `.java`-Dateien erzeugt; `OutputMessage`
  (id/type/role/status direkt am Node, `content` als lazy `OutputMessageContentList`) entspricht
  der Acceptance. Volle Kompilierprobe (`javac` gegen Jackson 2.16): alle verbleibenden Fehler
  sind ausschließlich Referenzen auf `CompositionNode`-Klassen (Package `.operators` bzw. deren
  Klassennamen) — exakt der für Segment 08 zurückgestellte Scope, keine sonstigen Fehler.

## Segment-08-Ergebnis (Kompositionen & Proxy-Views) — API für Folgesegmente
- `cgen/emit/model_context.py`: `classify()` liefert jetzt zusätzlich die Kategorie
  `'composition'` (resolved Zielknoten ist `CompositionNode`) — Getter darf hier NICHT wie
  `'complex'` `child == null || child.isNull()` zusammenfassen, sondern nur `child == null`
  (absent) zu Java-`null` kollabieren; ein explizites JSON-`null` liefert weiterhin eine
  gebundene View-Instanz, deren eigene `isNull()`-Branch-Methode `true` meldet (D3). Bugfix
  nebenbei gefunden/behoben: `classify()` prüfte `AnyDictionaryNode` erst NACH dem Auflösen der
  `RefNode`-Kette und klassifizierte ein benanntes `additionalProperties:true`-Schema, das über
  `$ref` erreicht wird (z.B. `ResponseFormatJsonSchemaSchema`), fälschlich als `'any_dictionary'`
  (roher `JsonNode`-Getter) trotz `map_type` (Segment 06), das für `RefNode` **immer** die
  generierte Klassen-FQN liefert (nie kollabiert) — Typkonflikt, kompilierte nicht. Fix: der
  `AnyDictionaryNode`-Check läuft jetzt auf dem **unaufgelösten** `node`-Parameter (spiegelt
  `map_type`s Reihenfolge exakt); über `RefNode` erreichte `AnyDictionaryNode`-Schemas werden
  jetzt korrekt als `'complex'` klassifiziert (eigene generierte Proxy-Klasse, deren Konstruktor
  ohnehin nur `JsonNode` nimmt).
  Neu: `build_branches(node: CompositionNode, named_model) -> list[Branch]` — der Render-Kontext
  pro Zweig. `Branch.accessor_name` ist der bereits von Segment 05 vergebene Klassenname des
  Zweigs (z.B. `Cat`, `ResponseProperties`, `CreateResponsePart3`) — **kein** neuer Namensraum,
  reine Wiederverwendung über `named_model.name_of(_ref)`. `Branch.applies_expr` ist eine
  fertige Java-Boolean-Ausdrucksstring (referenziert nur `node`, keine Hilfsvariablen nötig):
  `None` bei `allOf` (immer gültig, kein `is<Branch>()`); bei `anyOf`/`oneOf` entweder
  Discriminator-Vergleich (`node.path("<prop>").<asXxx>() == ...`/`.equals(...)`, D1: `mapping`
  hat Vorrang vor Const-Lookup je Zweig, `mapping`-Werte werden auf mehrere Zweige verteilt,
  Rest fällt auf Const/Single-Value-Enum der Discriminator-Property zurück) oder — falls kein
  Discriminator-Wert auflösbar ist — ein struktureller Fallback (`ObjectNode`: Konjunktion
  `node.has("field")` über alle `required`-Felder, `"true"` falls keine; `ListNode`:
  `node.isArray()`; `Dictionary/AnyDictionary`: `node.isObject()`; `Enum/Primitive`:
  `node.isTextual()/isIntegralNumber()/isNumber()/isBoolean()`; `null`-Zweig: `node.isNull()`;
  verschachtelte Komposition/unbekannt: `"true"`, bewusst nicht weiter geprüft).
  `Branch.has_getter` ist `False` nur für den `null`-Zweig (kein sinnvoller Getter für "kein
  Wert") — der `isNull()`-Check bleibt trotzdem erhalten (einziger Fall, in dem `is<Branch>()`
  buchstäblich `isNull` heißt, das ist beabsichtigt und liefert genau die D3-Unterscheidung in
  Kombination mit dem `absent`-Check der Eltern-Property).
  `UnsupportedNode`-Zweige werden übersprungen (D10, keine View).
- `cgen/emit/model_emit.py`: `CompositionNode` ist jetzt in `_MODEL_CLASS_KINDS`; `_render_composition`
  rendert `model/composition.java.jinja`. Traversierung (`_walk`/`iter_child_edges`) brauchte
  keine Änderung — Komposition-Branches wurden schon immer als Kind-Kanten iteriert (nötig für
  Naming/Packaging seit Segment 05), sie wurden nur bisher beim Rendern übersprungen.
- `cgen/templates/model/composition.java.jinja`: EIN `JsonNode`-Feld pro Klasse (I6, kein
  Merge). Pro Zweig: optional `public boolean is<Branch>()` (wenn `applies_expr` gesetzt) und
  optional `public <JavaType> get<Branch>()` (wenn `has_getter`) — Getter-Codeform je Kategorie
  identisch zu den bestehenden Model-Templates (`primitive`/`enum`/`any_dictionary`/sonst
  `new <Type>(node)`), aber **ungefiltert auf demselben `node`**, nie auf einem Kind-Feld —
  das ist der Kern von "Proxy-View über denselben Node". Keine Setter auf Komposition-Ebene
  (das Segment beschreibt nur Zugriffs-/Prüfmethoden): Schreiben läuft entweder über die
  zurückgegebene Zweig-View selbst (die denselben Node teilt, z.B.
  `getResponseProperties().setModel(...)`) oder über den Setter der umschließenden Property
  (ersetzt den ganzen gebundenen Node, bereits generisch durch `'composition'`-Kategorie im
  Eltern-Template abgedeckt).
- `cgen/templates/model/{object,list,list_mixed,dictionary}.java.jinja`: neuer Getter-Zweig
  `{% elif a.category == 'composition' %}` (nur `child == null` kollabiert, `isNull()` nicht) —
  Setter unverändert, fällt weiterhin in den bestehenden `{% else %}`-Zweig (`value.node()`),
  das war schon vorher korrekt für jeden Proxy-artigen Kindtyp.
- Verifiziert gegen `deepseek.filtered.yaml`: 169 `.java`-Dateien erzeugt (vorher 121, +48
  Komposition-Klassen in `.operators` + zuvor unbenannte anonyme Objekt-Branches, die erst durch
  das Rendern der Kompositionen erreichbar/nötig wurden). `CreateResponse` (allOf) →
  `getCreateModelResponseProperties()`/`getResponseProperties()`/`getCreateResponsePart3()`
  (exakt Acceptance-Wortlaut). `InputContent`/`Tool` (oneOf, Discriminator `type`, kein/implizites
  `mapping`) → `is<Branch>()` via `"<const>".equals(node.path("type").asText())`. `AnyOfReasoningNull`
  → `isReasoning()` (`"true"`, kein Discriminator/kein required-Feld zum Prüfen) + `getReasoning()`
  (kollabiert nur `child==null`) + `isNull()` (`node.isNull()`); Eltern-Property `reasoning` auf
  `CreateResponsePart3` unterscheidet jetzt sichtbar absent (`null`) von gebundener View
  (`isNull()==true`) von echtem Wert. Volle Kompilierprobe (`javac` gegen Jackson 2.16.1): alle
  169 Dateien kompilieren fehlerfrei (`exit=0`), keine offenen Fehler mehr.

## Segment-09-Ergebnis (Request/Response-Serialisierung) — API für Folgesegmente
- `cgen/emit/io_context.py` (neu): `request_root_node(operation_model, named_nodes) -> Node|None`
  — löst den D4-Sonderfall auf: ist der Request-Body ein `$ref`, IST das benannte Schema (nicht
  der `RequestNode`) die Klasse, die `toString()`/`fromString()` bekommt; bei inline Body ist es
  der Body-Knoten selbst. `request_root_node_ids(model) -> set[id(...)]` sammelt das über alle
  Operationen. `json_support_fqn(base_package)` → `<base>.io.JsonSupport`. `build_code_branches`/
  `build_content_type_branches` liefern den Render-Kontext für `ResponseNode`/`CodeNode`
  (Wiederverwendung von `model_context.classify`/`map_type`/`PRIMITIVE_READ_METHOD`).
- `cgen/emit/model_emit.py`: `emit_model` berechnet jetzt vorab `request_root_node_ids(model)` und
  reicht `is_request_root: bool` + `json_support_fqn` in den Render-Kontext jeder
  Object/List/(Mixed-)List/Dictionary/AnyDictionary/Composition-Klasse durch (Enum bewusst
  ausgenommen — ein bare Top-Level-Enum-Request-Body ist außerhalb des Fixture-Scopes und ein
  Java-`enum` kann das für `fromString` nötige gebundene `JsonNode` ohnehin nicht halten). **Es
  entsteht keine separate Wrapper-Klasse** — die ohnehin generierte Klasse (z.B. `CreateResponse`,
  ein allOf-`CompositionNode`) bekommt die zwei Methoden direkt angehängt.
- `cgen/templates/model/{object,list,list_mixed,dictionary,any_dictionary,composition}.java.jinja`:
  jeweils ein `{% if is_request_root %}`-Block am Klassenende mit `toString()`
  (`{{ json_support_fqn }}.write(node)`) und statischem `fromString(String body)`
  (`new {{ class_name }}({{ json_support_fqn }}.parse(body))`).
- `cgen/templates/io/json_support.java.jinja` (neu): erzeugt einmalig `<base>.io.JsonSupport` —
  statischer `ObjectMapper` + `parse(String)->JsonNode`/`write(JsonNode)->String`, IOExceptions in
  `RuntimeException` gewrappt (keine checked Exception in der generierten API).
- `cgen/templates/io/response.java.jinja` (neu): rendert die `ResponseNode`-Wurzelklasse (z.B.
  `ResponsesResponse`) — hält `node`+`statusCode`(als `String`, robust gegen `default`/`4XX`)
  +`contentType` (reine Transport-Metadaten, D5, nie im Body). `from(String body, int statusCode,
  String contentType)` ist der einzige Konstruktionsweg (privater Konstruktor). Pro `CodeNode` ein
  `getCode<code>()`, das `null` liefert, wenn der gebundene `statusCode` nicht passt, sonst eine
  neue `CodeNode`-Instanz (Discriminator = Statuscode).
- `cgen/templates/io/code.java.jinja` (neu): rendert die `CodeNode`/`ContentTypeView`-Klasse (eine
  Klasse für beide, wie schon in Segment 05 benannt) — hält `node`+`contentType`. Pro
  `ContentTypeView` ein `is<Ct>()` (Content-Type-Header-Vergleich) + `get<Ct>()`
  (`null` falls Content-Type nicht passt, sonst dieselbe Getter-Form wie
  `composition.java.jinja`s Branches: `primitive`/`enum`/`any_dictionary`/sonst `new Type(node)`).
- `cgen/emit/io_emit.py`: `emit_io(model, writer)` implementiert — schreibt `JsonSupport` einmalig,
  dann pro Operation die `ResponseNode`- und alle `CodeNode`-Klassen (Namen/Pakete kommen bereits
  vollständig aus Segment 05, hier nur Rendering). Request-seitige Serialisierung läuft komplett
  über `model_emit.py` (kein separater Pfad).
- Verifiziert gegen `deepseek.filtered.yaml`: weiterhin 174 `.java`-Dateien (169 Modell- + 1
  `JsonSupport` + `ResponsesResponse` + 3×`ResponsesResponseCode<code>Json`); volle Kompilierprobe
  (`javac` gegen Jackson 2.17.2) `exit=0`. Ad-hoc-Laufprobe: `CreateResponse.fromString(body)` →
  `toString()` roundtrippt; `ResponsesResponse.from(body,200,"application/json").getCode200()`
  liefert die Instanz, `getCode429()`→`null` (und umgekehrt bei Status 429); `getCode200().getJson()`
  liefert die `Response`-Body-Instanz (`getResponsePart3().getId()` liest das erwartete Feld).
  `emit_code` bricht danach weiterhin (erwartet) mit `NotImplementedError` in `emit_client` ab
  (Segment 10).

## Segment-10-Ergebnis (Client) — API für Folgesegmente
- `cgen/ingest/operations.py`: `Operation` hat ein neues Feld `description: str | None`
  (`raw_operation.get('description') or raw_operation.get('summary')`) — nötig, damit das
  Interface laut Segment-10-Vorgabe die Operation-Beschreibung als Javadoc übernimmt (Segment
  02/09 unberührt, rein additiv, kein Bruch bestehender Konsumenten).
- `cgen/emit/client_context.py` (neu): `build_client_methods(named_model) -> list[ClientMethod]`
  — ein `ClientMethod` pro Operation, deterministisch nach `(path, method)` sortiert (I7, nicht
  Discovery-Reihenfolge). `MethodParameter(name, java_type, raw_name, kind)` für
  path/query/body-Parameter; Pfad-/Query-Parameter-Typen werden NICHT über den IR-Graphen
  gemappt (Parameter sind laut Segment 02 nie Teil des Body-Baums), sondern direkt aus dem
  rohen JSON-Schema-`type` auf einen skalaren Java-Typ abgebildet (`PARAMETER_JAVA_TYPE`,
  Default `String`). Body-Parameter-Typ kommt über `io_context.request_root_node` (D4-konform,
  Wiederverwendung aus Segment 09) + `named_model.name_of(...)`. `_method_name`: `operationId`
  (sanitized) gewinnt, sonst `to_camel_case("<method><PathClassFragment>")`
  (z.B. `postResponses`). `client_interface_name(named_model)`: sortierte Menge der
  Top-Level-Pfadsegmente aller Operationen, jedes `class_identifier`-konvertiert, konkateniert,
  `+"Client"` — bei genau einem Pfad `/responses` ergibt das exakt `ResponsesClient`
  (Acceptance-Wortlaut); die Impl heißt `<Interface>Impl`.
- `cgen/emit/client_emit.py`: `emit_client(model, writer)` implementiert — schreibt GENAU EIN
  Interface + EINE Impl-Klasse für die gesamte API in `<base_package>.client` (kein Package pro
  Pfad/Operation, "eine Client Facade" laut Quelldokument). No-op bei `model.operations == ()`.
- `cgen/templates/client/interface.java.jinja` (neu): ein Methodensignatur-Eintrag pro
  `ClientMethod`, Javadoc via `model/macros.jinja` aus `description`/`example_repr` der
  Operation (nicht des Schemas). Rückgabetyp/Body-Parameter-Typ werden als FQN inline verwendet
  (Projekt-Konvention, keine `import`-Verwaltung für generierte Typen).
- `cgen/templates/client/impl.java.jinja` (neu): `java.net.http.HttpClient`-basierte
  Implementierung, öffentlicher Zwei-Konstruktor (`(baseUrl)` und `(baseUrl, HttpClient)` —
  Base-URL als Konstruktor-Parameter, D11), `protected void customizeRequest(HttpRequest.Builder)`
  als No-op-Hook (D11: Auth-Injection via Subklasse, kein konkreter Auth-Code im Kern). Pro
  Methode: URL wird aus `path_url_expression` (vorgefertigter Java-Konkatenations-Ausdruck,
  Pfad-Parameter über `URLEncoder.encode`) + optionalem Query-String (Null-Check pro
  Query-Parameter, dann `?a=..&b=..`) gebaut; `.method(HTTP_METHODE, BodyPublisher)` einheitlich
  für alle Methoden (Body vorhanden -> `ofString(request.toString())`, sonst `noBody()`) statt
  spezialisierter `.GET()/.POST()`-Aufrufe (einfacher, deckt auch PUT/PATCH ohne Sonderfall ab).
  Response-Konstruktion exakt wie Segment 09 vorgesehen: `ResponseType.from(body, statusCode,
  contentType)` aus dem `HttpResponse<String>` + `Content-Type`-Header. Transportfehler
  (IOException/InterruptedException) werden in `RuntimeException` gewrappt (keine checked
  Exception in der generierten API, konsistent zu `JsonSupport`); HTTP-Fehlercodes (429/503)
  sind keine Exception, sondern laufen unverändert durch `ResponseType.from(...)`.
- Verifiziert gegen `deepseek.filtered.yaml`: `ResponsesClient.createResponse(CreateResponse) :
  ResponsesResponse` (exakter Acceptance-Wortlaut), `ResponsesClientImpl implements
  ResponsesClient`. `emit_code` läuft jetzt vollständig durch (kein `NotImplementedError` mehr,
  Pipeline-Ende erreicht). Volle Kompilierprobe (`javac` gegen Jackson 2.17.2): 176 `.java`-Dateien
  (vorher 174 + Interface + Impl), `exit=0`. Reflektions-Probe: Interface-Methode exakt
  `ResponsesResponse createResponse(CreateResponse)`, Impl instanziierbar über
  `new ResponsesClientImpl(String)`.

## Plan

/home/user/xyan/xy.ai.workbench/project/start_cgen/00_overview.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/01_project_scaffolding.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/02_schema_ingest.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/03_node_model_ir.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/04_identity_and_dedup.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/05_naming_and_packaging.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/06_type_mapping.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/07_model_emission.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/08_compositions_views.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/09_request_response_serialization.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/10_client.md                 
/home/user/xyan/xy.ai.workbench/project/start_cgen/11_end_to_end_and_verification.md
/home/user/xyan/xy.ai.workbench/project/start_cgen/_context.md