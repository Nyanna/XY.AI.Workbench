"""Builds the IR node graph from ingested schema data.

$ref is turned into a RefNode (an atomic id token) and never expanded here,
so named components.schemas form a DAG rather than being inlined -- this is
what lets fingerprint bottom-up without special-casing cycles.
"""

from dataclasses import dataclass

from xy.cgen.ingest import IngestedSchema, Operation
from xy.cgen.ingest.refindex import SCHEMA_REF_PREFIX
from xy.cgen.model.nodes import (
    AnyDictionaryNode,
    CodeNode,
    CompositionNode,
    ContentTypeView,
    Discriminator,
    DictionaryNode,
    Edge,
    EnumNode,
    ListNode,
    Node,
    ObjectNode,
    PrimitiveNode,
    RefNode,
    RequestNode,
    ResponseNode,
    UnsupportedNode,
)

COMPOSITION_KEYWORDS = ("allOf", "anyOf", "oneOf")
PRIMITIVE_TYPES = ("string", "integer", "number", "boolean", "null")
JSON_CONTENT_TYPE = "application/json"


@dataclass(frozen=True)
class OperationModel:
    """The request/response root nodes for a single (path, method) operation."""

    operation: Operation
    request: RequestNode | None
    response: ResponseNode


@dataclass(frozen=True)
class Model:
    """The full IR: every named schema plus the per-operation transport roots."""

    named_nodes: dict[str, Node]
    operations: tuple[OperationModel, ...]


def build_model(ingested: IngestedSchema) -> Model:
    """Build one node per components.schemas entry and the roots for every operation."""
    named_nodes = {
        ref.removeprefix(SCHEMA_REF_PREFIX): build_node(ingested.ref_index.get(ref))
        for ref in ingested.ref_index.schema_refs()
    }
    operations = tuple(_build_operation(operation) for operation in ingested.operations)
    return Model(named_nodes=named_nodes, operations=operations)


def _build_operation(operation: Operation) -> OperationModel:
    request = None
    if operation.request_body is not None:
        request = RequestNode(body=build_edge("body", operation.request_body))
    codes = tuple(
        CodeNode(
            status_code=status_code,
            content_types=(
                ContentTypeView(content_type=JSON_CONTENT_TYPE, body=build_edge("body", schema)),
            ),
        )
        for status_code, schema in operation.responses.items()
    )
    return OperationModel(operation=operation, request=request, response=ResponseNode(codes=codes))


def build_edge(label: str, raw: dict) -> Edge:
    """Build an edge: the target node plus this use site's metadata.

    $ref siblings (OpenAPI 3.1) land here as edge metadata overrides -- they
    never change the target's structure, only what this particular edge reports.
    """
    kwargs = {}
    if "description" in raw:
        kwargs["description"] = raw["description"]
    if "example" in raw:
        kwargs["example"] = raw["example"]
    if "default" in raw:
        kwargs["default"] = raw["default"]
    return Edge(label=label, target=build_node(raw), **kwargs)


def build_node(raw: dict) -> Node:
    """Dispatch a raw schema dict to the matching IR node kind."""
    if "$ref" in raw:
        return RefNode(name=raw["$ref"].removeprefix(SCHEMA_REF_PREFIX))
    if "not" in raw:
        return UnsupportedNode(reason="not")
    for keyword in COMPOSITION_KEYWORDS:
        if keyword in raw:
            return _build_composition(keyword, raw)
    if "enum" in raw:
        return _build_enum(raw)

    schema_type = raw.get("type")
    if isinstance(schema_type, list):
        return _build_type_union(raw, schema_type)
    if schema_type == "array":
        return _build_list(raw)
    if schema_type == "object" or "properties" in raw or "additionalProperties" in raw:
        return _build_object_or_dictionary(raw)
    if schema_type in PRIMITIVE_TYPES:
        return PrimitiveNode(primitive_type=schema_type)

    raise ValueError(f"unsupported schema shape: {raw!r}")


def _build_object_or_dictionary(raw: dict) -> Node:
    properties = raw.get("properties") or {}
    if properties:
        edges = tuple(build_edge(name, sub_schema) for name, sub_schema in properties.items())
        required = frozenset(name for name in raw.get("required") or [] if name in properties)
        return ObjectNode(properties=edges, required=required)

    additional = raw.get("additionalProperties")
    if isinstance(additional, dict) and additional:  # additionalProperties: {schema}
        return DictionaryNode(value=build_edge("value", additional))
    return AnyDictionaryNode()  # additionalProperties: true|{} or bare 'object'


def _build_list(raw: dict) -> Node:
    items = raw.get("items")
    if isinstance(items, list):  # tuple validation: one type per position
        edges = tuple(build_edge(str(index), item) for index, item in enumerate(items))
        return ListNode(elements=edges, mixed=True)
    if items is not None:
        return ListNode(elements=(build_edge("element", items),))
    raise ValueError("array schema without 'items' is unsupported")


def _build_enum(raw: dict) -> Node:
    values = tuple(raw.get("enum") or ())
    primitive_type = raw.get("type") or _infer_primitive_type(values)
    return EnumNode(primitive_type=primitive_type, values=values)


def _infer_primitive_type(values: tuple) -> str:
    if not values:
        return "string"
    sample = values[0]
    if isinstance(sample, bool):
        return "boolean"
    if isinstance(sample, int):
        return "integer"
    if isinstance(sample, float):
        return "number"
    if sample is None:
        return "null"
    return "string"


def _build_composition(keyword: str, raw: dict) -> Node:
    edges = tuple(build_edge(str(index), branch) for index, branch in enumerate(raw[keyword]))
    discriminator = None
    raw_discriminator = raw.get("discriminator")
    if raw_discriminator:
        discriminator = Discriminator(
            property_name=raw_discriminator["propertyName"],
            mapping=raw_discriminator.get("mapping"),
        )
    return CompositionNode(keyword=keyword, branches=edges, discriminator=discriminator)


def _build_type_union(raw: dict, schema_type: list) -> Node:
    """`type: [X, 'null', ...]` shorthand -> anyOf of single-typed variants."""
    branches = []
    for single_type in schema_type:
        branch = dict(raw)
        branch["type"] = single_type
        branches.append(branch)
    edges = tuple(build_edge(str(index), branch) for index, branch in enumerate(branches))
    return CompositionNode(keyword="anyOf", branches=edges, discriminator=None)
