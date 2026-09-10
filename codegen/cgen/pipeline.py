"""Wires the generator steps together: ingest -> model -> identity -> naming -> emit."""

from cgen.config import Config
from cgen.emit import emit_code
from cgen.identity import compute_identity
from cgen.ingest import ingest_schema
from cgen.model import build_model
from cgen.naming import assign_names


def run_pipeline(config: Config) -> None:
    """Run all steps in order. Each step is currently a stub."""
    ingested = ingest_schema(config)
    model = build_model(ingested)
    identified_model = compute_identity(model)
    named_model = assign_names(identified_model, config.base_package)
    emit_code(named_model, config.output_dir)
