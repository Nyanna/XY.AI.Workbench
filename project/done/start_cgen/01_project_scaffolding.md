# 01 — Projektgerüst

**Ziel:** Python-Projektstruktur, Abhängigkeiten, Modulgrenzen, CLI-Skelett, Konfiguration. Noch keine Generierungslogik.
**Abhängig von:** —

## Deliverables
- Python-Projekt in `/home/user/xyan/xy.ai.workbench/codegen`:
  ```
  codegen/
    pyproject.toml        # deps: pyyaml, jinja2
    cgen/
      __main__.py         # ruft cli
      cli.py              # Args: --schema, --out, --base-package
      config.py           # Config-Objekt (input_schema, output_dir, base_package)
      ingest/             # Segment 02
      model/              # Segment 03
      identity/           # Segment 04
      naming/             # Segment 05
      typemap/            # Segment 06
      emit/               # Segment 07/09/10 (model, io, client) + FileWriter
      templates/          # Jinja2-Templates
      pipeline.py         # verdrahtet die Schritte (Segment 11), Schritte hier als leere Stubs
  ```
- CLI `python -m cgen --schema <yaml> --out <dir> --base-package xy.api.codegen` läuft (ruft leere Pipeline).

## Verbindliche Entscheidungen
- Template-Engine: **Jinja2**.
- Abhängigkeitsrichtung (streng): `ingest → model → identity → naming → emit`. `typemap` wird von `naming`/`emit` genutzt. `cli` verdrahtet alles.
- Keine Tests (Vorgabe). Ein Kompilat-Check (Segment 11) ist kein Test.
- `base_package` Default `xy.api.codegen`.

## Interpretations-Leitplanken
- **Model-Emit darf keine Client-Abhängigkeit haben** (I2). Client ist ein separates Emit-Submodul.
- In diesem Segment **keine** Schema-/Generierungslogik — nur Gerüst und benannte, leere Pipeline-Schritte.

## Acceptance
- `python -m cgen --help` funktioniert.
- Pipeline-Schritte existieren als benannte, leere Funktionen in der korrekten Abhängigkeitsrichtung.
- Keine Modulgrenzenverletzung (z.B. `model` importiert nicht aus `emit`).

## Nicht in diesem Segment
Jegliche Schema-Verarbeitung, IR, Namen, Templates-Inhalt.
