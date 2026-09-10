"""Loads the OpenAPI schema and extracts the ref index and operation list."""

from dataclasses import dataclass

from cgen.config import Config
from cgen.ingest.loader import load_yaml
from cgen.ingest.operations import Operation, Parameter, extract_operations
from cgen.ingest.refindex import RefIndex, build_ref_index

__all__ = ["IngestedSchema", "Operation", "Parameter", "RefIndex", "ingest_schema"]


@dataclass(frozen=True)
class IngestedSchema:
    """Result of ingestion: the component ref index and the flat operation list."""

    ref_index: RefIndex
    operations: list[Operation]


def ingest_schema(config: Config) -> IngestedSchema:
    """Load the YAML document, index components, and extract operations.

    info/servers/security are present in the loaded document but are not
    carried into the result.
    """
    document = load_yaml(config.input_schema)
    ref_index = build_ref_index(document)
    operations = extract_operations(document, ref_index)
    return IngestedSchema(ref_index=ref_index, operations=operations)
