"""JSON-RPC 2.0 / MCP error definitions.

The MCP wire protocol is JSON-RPC 2.0, so the standard JSON-RPC error codes
apply.  Anything in the implementation-defined range ``[-32000, -32099]`` is
reserved for MCP/server specific errors.
"""

from __future__ import annotations

from typing import Any

# --- Standard JSON-RPC 2.0 error codes ------------------------------------
PARSE_ERROR = -32700
INVALID_REQUEST = -32600
METHOD_NOT_FOUND = -32601
INVALID_PARAMS = -32602
INTERNAL_ERROR = -32603

# --- Server/MCP specific error codes (implementation-defined range) -------
#: The client sent a request before completing the ``initialize`` handshake.
NOT_INITIALIZED = -32002
#: The referenced session id is unknown / has been terminated.
SESSION_NOT_FOUND = -32001


class JsonRpcError(Exception):
    """An error that can be serialised into a JSON-RPC ``error`` object.

    Handlers raise this to abort processing of a single request; the transport
    layer turns it into a well-formed JSON-RPC error response.
    """

    def __init__(self, code: int, message: str, data: Any | None = None) -> None:
        super().__init__(message)
        self.code = code
        self.message = message
        self.data = data

    def to_object(self) -> dict[str, Any]:
        """Return the JSON-RPC ``error`` member for this exception."""
        error: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.data is not None:
            error["data"] = self.data
        return error

    def __repr__(self) -> str:  # pragma: no cover - debugging aid
        return f"JsonRpcError(code={self.code!r}, message={self.message!r})"


# Convenience constructors -------------------------------------------------

def parse_error(data: Any | None = None) -> JsonRpcError:
    return JsonRpcError(PARSE_ERROR, "Parse error", data)


def invalid_request(message: str = "Invalid Request", data: Any | None = None) -> JsonRpcError:
    return JsonRpcError(INVALID_REQUEST, message, data)


def method_not_found(method: str) -> JsonRpcError:
    return JsonRpcError(METHOD_NOT_FOUND, f"Method not found: {method}", {"method": method})


def invalid_params(message: str = "Invalid params", data: Any | None = None) -> JsonRpcError:
    return JsonRpcError(INVALID_PARAMS, message, data)


def internal_error(message: str = "Internal error", data: Any | None = None) -> JsonRpcError:
    return JsonRpcError(INTERNAL_ERROR, message, data)
