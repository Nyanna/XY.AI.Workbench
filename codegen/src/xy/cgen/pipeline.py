"""Wires the generator steps together: ingest -> model -> identity -> naming -> emit."""

from xy.cgen.config import Config
from xy.cgen.emit import emit_code
from xy.cgen.identity import compute_identity
from xy.cgen.ingest import ingest_schema
from xy.cgen.model import build_model
from xy.cgen.naming import assign_names


def run_pipeline(config: Config) -> None:
    """Run all steps in order. Each step is currently a stub."""
    ingested = ingest_schema(config)
    model = build_model(ingested)
    identified_model = compute_identity(model)
    named_model = assign_names(identified_model, config.base_package)
    emit_code(named_model, config.output_dir)
