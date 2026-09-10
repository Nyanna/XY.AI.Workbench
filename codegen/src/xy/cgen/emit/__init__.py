"""Orchestrates rendering of model, io, and client code to the output directory."""

from pathlib import Path

from xy.cgen.emit.client_emit import emit_client
from xy.cgen.emit.io_emit import emit_io
from xy.cgen.emit.model_emit import emit_model
from xy.cgen.emit.writer import FileWriter


def emit_code(model, output_dir: Path):
    """Run all emit steps against a shared FileWriter.

    Model emission is implemented; io/client emission still raise
    NotImplementedError until their segments land.
    """
    writer = FileWriter(output_dir)
    emit_model(model, writer)
    emit_io(model, writer)
    emit_client(model, writer)
