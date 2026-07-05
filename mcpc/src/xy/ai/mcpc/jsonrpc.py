"""Minimal, dependency-free JSON-RPC 2.0 message handling.

Only the pieces required for the MCP wire protocol are implemented:

* parsing a raw HTTP body into a JSON object,
* classifying a message as *request*, *notification* or *response*,
* building well-formed success and error responses.

Batch requests were removed from the MCP spec (protocol revision 2025-06-18
onwards), therefore a message is always a single JSON object.
"""

from __future__ import annotations

import json
from dataclasses import dataclass
from enum import Enum
from typing import Any

from .errors import JsonRpcError, invalid_request, parse_error

JSONRPC_VERSION = "2.0"

# A JSON-RPC id may be a string, number or null.
RequestId = str | int | float | None


class MessageKind(Enum):
    REQUEST = "request"
    NOTIFICATION = "notification"
    RESPONSE = "response"


@dataclass(slots=True)
class JsonRpcRequest:
    """A parsed JSON-RPC *request* or *notification*.

    A notification is simply a request without an ``id`` member.
    """

    method: str
    params: dict[str, Any]
    id: RequestId = None
    is_notification: bool = False


def parse_body(raw: bytes) -> dict[str, Any]:
    """Decode a raw HTTP body into a JSON object.

    Raises :class:`JsonRpcError` with the ``PARSE_ERROR`` code on malformed
    JSON and ``INVALID_REQUEST`` when the top-level value is not an object.
    """
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as exc:  # pragma: no cover - defensive
        raise parse_error("Body is not valid UTF-8") from exc
    try:
        value = json.loads(text)
    except json.JSONDecodeError as exc:
        raise parse_error(str(exc)) from exc
    if isinstance(value, list):
        raise invalid_request("Batch requests are not supported")
    if not isinstance(value, dict):
        raise invalid_request("A JSON-RPC message must be an object")
    return value


def classify(message: dict[str, Any]) -> MessageKind:
    """Determine whether *message* is a request, notification or response."""
    if "method" in message:
        return MessageKind.NOTIFICATION if "id" not in message else MessageKind.REQUEST
    if "result" in message or "error" in message:
        return MessageKind.RESPONSE
    raise invalid_request("Message is neither a request, notification nor response")


def to_request(message: dict[str, Any]) -> JsonRpcRequest:
    """Validate and convert a raw message into a :class:`JsonRpcRequest`.

    Accepts both requests and notifications.
    """
    if message.get("jsonrpc") != JSONRPC_VERSION:
        raise invalid_request(f'"jsonrpc" must be "{JSONRPC_VERSION}"')

    method = message.get("method")
    if not isinstance(method, str) or not method:
        raise invalid_request('"method" must be a non-empty string')

    params = message.get("params", {})
    if params is None:
        params = {}
    if not isinstance(params, dict):
        # MCP only uses "by-name" params (objects); positional params are rejected.
        raise invalid_request('"params" must be an object')

    is_notification = "id" not in message
    request_id = message.get("id")
    if not is_notification and not isinstance(request_id, (str, int, float)) and request_id is not None:
        raise invalid_request('"id" must be a string, number or null')

    return JsonRpcRequest(
        method=method,
        params=params,
        id=request_id,
        is_notification=is_notification,
    )


def success_response(request_id: RequestId, result: Any) -> dict[str, Any]:
    """Build a JSON-RPC success response object."""
    return {"jsonrpc": JSONRPC_VERSION, "id": request_id, "result": result}


def error_response(request_id: RequestId, error: JsonRpcError | dict[str, Any]) -> dict[str, Any]:
    """Build a JSON-RPC error response object.

    ``request_id`` is ``None`` for protocol-level errors (e.g. a parse error)
    where the id could not be determined.
    """
    obj = error.to_object() if isinstance(error, JsonRpcError) else error
    return {"jsonrpc": JSONRPC_VERSION, "id": request_id, "error": obj}


def dumps(message: dict[str, Any]) -> bytes:
    """Serialise a JSON-RPC message to UTF-8 bytes."""
    return json.dumps(message, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
