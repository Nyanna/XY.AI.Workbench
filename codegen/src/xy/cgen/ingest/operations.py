"""Extracts the operation list from an OpenAPI document's 'paths' section."""
from dataclasses import dataclass
from xy.cgen.ingest.refindex import RefIndex
HTTP_METHODS = ('get', 'put', 'post', 'delete', 'options', 'head', 'patch', 'trace')
PARAMETER_LOCATIONS = ('path', 'query')
JSON_CONTENT_TYPE = 'application/json'
JSONL_CONTENT_TYPE = 'application/x-jsonl'
RESPONSE_CONTENT_TYPES = (JSON_CONTENT_TYPE, JSONL_CONTENT_TYPE)

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

    request_body holds a raw schema node: either an inline schema dict or a
    {'$ref': ...} node. responses maps status code -> content type -> such a
    node (for JSONL the node describes one line). Refs are never expanded here.
    """
    path: str
    method: str
    operation_id: str | None
    description: str | None
    request_body: dict | None
    responses: dict[str, dict[str, dict]]
    parameters: list[Parameter]

def _json_schema(content: dict | None) -> dict | None:
    """Pick the application/json schema from a content map; other content types are dropped."""
    media = (content or {}).get(JSON_CONTENT_TYPE)
    return media.get('schema') if media else None

def _resolve_response_schemas(response_node: dict, ref_index: RefIndex) -> dict[str, dict]:
    """Reduce a responses-map entry to its schema node per supported content type.

    A $ref into components.responses is followed once to the reusable response
    object, then each supported content.<type>.schema is taken in declaration
    order. headers/examples are dropped.
    """
    node = response_node
    if '$ref' in node:
        node = ref_index.get(node['$ref'])
    schemas = {}
    for content_type, media in (node.get('content') or {}).items():
        schema = media.get('schema') if media else None
        if content_type in RESPONSE_CONTENT_TYPES and schema is not None:
            schemas[content_type] = schema
    return schemas

def _extract_parameters(raw_parameters: list | None, ref_index: RefIndex) -> list[Parameter]:
    parameters = []
    for param in raw_parameters or []:
        if '$ref' in param:
            param = ref_index.get(param['$ref'])
        location = param.get('in')
        if location not in PARAMETER_LOCATIONS:
            continue
        parameters.append(
            Parameter(
                name=param['name'],
                location=location,
                required=bool(
                    param.get(
                        'required',
                        False)),
                schema=param.get('schema')))
    return parameters

def extract_operations(document: dict, ref_index: RefIndex) -> list[Operation]:
    """Build the operation list by walking 'paths' in document order."""
    operations = []
    for path, path_item in (document.get('paths') or {}).items():
        for method in HTTP_METHODS:
            raw_operation = path_item.get(method)
            if raw_operation is None:
                continue
            request_body = _json_schema((raw_operation.get('requestBody') or {}).get('content'))
            responses = {}
            for status_code, response_node in (raw_operation.get('responses') or {}).items():
                schemas = _resolve_response_schemas(response_node, ref_index)
                if schemas:
                    responses[status_code] = schemas
            operations.append(
                Operation(
                    path=path,
                    method=method,
                    operation_id=raw_operation.get('operationId'),
                    description=raw_operation.get('description') or raw_operation.get('summary'),
                    request_body=request_body,
                    responses=responses,
                    parameters=_extract_parameters(
                        raw_operation.get('parameters'),
                        ref_index)))
    return operations