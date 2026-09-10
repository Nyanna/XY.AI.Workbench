"""Extracts the operation list from an OpenAPI document's 'paths' section."""

from dataclasses import dataclass

from xy.cgen.ingest.refindex import RefIndex

HTTP_METHODS = ("get", "put", "post", "delete", "options", "head", "patch", "trace")
PARAMETER_LOCATIONS = ("path", "query")
JSON_CONTENT_TYPE = "application/json"


@dataclass(frozen=True)
class Parameter:
    """A path or query parameter of an operation. Never part of the JSON body tree."""

    name: str
    location: str
    required: bool
    schema: dict | None


@dataclass(frozen=True)
class Operation:
    """A single (path, method) operation with its body, response, and parameter schemas.

    request_body and responses hold raw schema nodes: either an inline schema dict
    or a {'$ref': ...} node. Refs are never expanded here.
    """

    path: str
    method: str
    operation_id: str | None
    description: str | None
    request_body: dict | None
    responses: dict[str, dict]
    parameters: list[Parameter]


def _json_schema(content: dict | None) -> dict | None:
    """Pick the application/json schema from a content map; other content types are dropped."""
    media = (content or {}).get(JSON_CONTENT_TYPE)
    return media.get("schema") if media else None


def _resolve_response_schema(response_node: dict, ref_index: RefIndex) -> dict | None:
    """Reduce a responses-map entry to its schema node.

    A $ref into components.responses is followed once to the reusable response
    object, then its content.<json>.schema is taken. headers/examples are dropped.
    """
    node = response_node
    if "$ref" in node:
        node = ref_index.get(node["$ref"])
    return _json_schema(node.get("content"))


def _extract_parameters(raw_parameters: list | None) -> list[Parameter]:
    parameters = []
    for param in raw_parameters or []:
        location = param.get("in")
        if location not in PARAMETER_LOCATIONS:
            continue
        parameters.append(
            Parameter(
                name=param["name"],
                location=location,
                required=bool(param.get("required", False)),
                schema=param.get("schema"),
            )
        )
    return parameters


def extract_operations(document: dict, ref_index: RefIndex) -> list[Operation]:
    """Build the operation list by walking 'paths' in document order."""
    operations = []
    for path, path_item in (document.get("paths") or {}).items():
        for method in HTTP_METHODS:
            raw_operation = path_item.get(method)
            if raw_operation is None:
                continue
            request_body = _json_schema((raw_operation.get("requestBody") or {}).get("content"))
            responses = {}
            for status_code, response_node in (raw_operation.get("responses") or {}).items():
                schema = _resolve_response_schema(response_node, ref_index)
                if schema is not None:
                    responses[status_code] = schema
            operations.append(
                Operation(
                    path=path,
                    method=method,
                    operation_id=raw_operation.get("operationId"),
                    description=raw_operation.get("description") or raw_operation.get("summary"),
                    request_body=request_body,
                    responses=responses,
                    parameters=_extract_parameters(raw_operation.get("parameters")),
                )
            )
    return operations
