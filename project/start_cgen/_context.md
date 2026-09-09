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
  model/__init__.py       # build_model(ingested) -> NotImplementedError (Segment 03)
  identity/__init__.py    # compute_identity(model) -> NotImplementedError (Segment 04)
  naming/__init__.py      # assign_names(model, base_package) -> NotImplementedError (Segment 05)
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
