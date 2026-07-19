"""MCP protocol logic (lifecycle + tools feature).

This module is transport-agnostic: it takes a parsed JSON-RPC request plus the
owning :class:`Session` and returns a result payload (or raises
:class:`JsonRpcError`).  The Streamable HTTP transport wraps the return value
in a JSON-RPC envelope.
"""

from __future__ import annotations

import logging
import base64
from typing import TYPE_CHECKING, Any

from . import errors
from .config import ServerConfig
from .jsonrpc import JsonRpcRequest
from .registry import ToolContext, ToolRegistry, normalize_result
from .session import Session

logger = logging.getLogger("xy.ai.mcpc.protocol")

if TYPE_CHECKING:
    from .context import AppServices

# Methods a client may call before the initialize handshake has completed.
_PRE_INIT_METHODS = {"initialize", "ping"}


def _encode_cursor(offset: int) -> str:
    return base64.urlsafe_b64encode(str(offset).encode("ascii")).decode("ascii")


def _decode_cursor(cursor: str) -> int:
    try:
        offset = int(base64.urlsafe_b64decode(cursor.encode("ascii")).decode("ascii"))
    except (ValueError, TypeError):
        raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
    if offset < 0:
        raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
    return offset


class McpProtocol:
    """Dispatches MCP methods against a session."""

    def __init__(
        self,
        config: ServerConfig,
        registry: ToolRegistry,
        services: "AppServices | None" = None,
    ) -> None:
        self.config = config
        self.registry = registry
        self.services = services
        self._handlers = {
            "initialize": self._handle_initialize,
            "ping": self._handle_ping,
            "tools/list": self._handle_tools_list,
            "tools/call": self._handle_tools_call,
        }

    # -- Request handling ---------------------------------------------------
    def handle_request(
        self,
        session: Session,
        request: JsonRpcRequest,
        *,
        skip_control: bool = False,
    ) -> Any:
        """Handle a JSON-RPC *request* and return its ``result`` payload.

        ``skip_control`` suppresses tool interception for this request,
        regardless of whether a :class:`ToolControlManager` is configured.
        It is set when the caller sends ``X-MCPC-CONTROL: off``.
        """
        handler = self._handlers.get(request.method)
        if handler is None:
            raise errors.method_not_found(request.method)

        if request.method not in _PRE_INIT_METHODS and not session.handshake_complete:
            raise errors.JsonRpcError(
                errors.NOT_INITIALIZED,
                "Session is not initialized; send an 'initialize' request first",
            )
        if request.method == "tools/call":
            return self._handle_tools_call(session, request.params, skip_control=skip_control)
        return handler(session, request.params)

    def handle_notification(self, session: Session, request: JsonRpcRequest) -> None:
        """Handle a JSON-RPC *notification*.

        Notifications are not "supported" in the sense that the server never
        acts on arbitrary ones and never emits any; the lifecycle
        ``notifications/initialized`` is accepted to complete the handshake.
        """
        if request.method == "notifications/initialized":
            with session.lock:
                session.initialized = True
        # All other notifications are silently accepted and ignored.

    # -- Lifecycle ----------------------------------------------------------
    def _handle_initialize(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
        requested = params.get("protocolVersion")
        if not isinstance(requested, str):
            raise errors.invalid_params('"protocolVersion" is required')

        if requested in self.config.supported_protocol_versions:
            negotiated = requested
        else:
            negotiated = self.config.preferred_protocol_version

        with session.lock:
            session.protocol_version = negotiated
            session.client_info = params.get("clientInfo")
            session.client_capabilities = params.get("capabilities")
            session.touch()

        return {
            "protocolVersion": negotiated,
            "capabilities": {
                # Only the tools feature is offered; listChanged is false since
                # notifications are unsupported.
                "tools": {"listChanged": False},
            },
            "serverInfo": {
                "name": self.config.server_name,
                "title": self.config.server_title,
                "version": self.config.server_version,
            },
            "instructions": self.config.instructions,
        }

    def _handle_ping(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
        return {}

    # -- Tools --------------------------------------------------------------
    def _handle_tools_list(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
        tools = self.registry.list_for_session(session)

        cursor = params.get("cursor")
        start = _decode_cursor(cursor) if cursor is not None else 0
        page_size = self.config.tools_page_size
        page = tools[start : start + page_size]

        result: dict[str, Any] = {"tools": [t.to_spec() for t in page]}
        if start + page_size < len(tools):
            result["nextCursor"] = _encode_cursor(start + page_size)
        return result

    def _handle_tools_call(
        self,
        session: Session,
        params: dict[str, Any],
        *,
        skip_control: bool = False,
    ) -> dict[str, Any]:
        name = params.get("name")
        if not isinstance(name, str) or not name:
            raise errors.invalid_params('"name" is required')

        arguments = params.get("arguments", {})
        if arguments is None:
            arguments = {}
        if not isinstance(arguments, dict):
            raise errors.invalid_params('"arguments" must be an object')

        tool = self.registry.get(name)
        # "Errors in finding the tool" are protocol errors (spec, tools/call).
        if tool is None or not session.is_tool_enabled(name):
            raise errors.invalid_params(
                f"Unknown or unavailable tool: {name}", {"name": name}
            )

        _validate_arguments(tool.input_schema, arguments)

        # --- request interception -------------------------------------------
        control = self.services.control_manager if self.services else None
        if control is not None and not skip_control:
            decision = control.submit_request(session, name, arguments)
            if not decision.approved:
                from .registry import ToolResult, text_content
                reason = decision.rejection_reason or "Tool call rejected by controller"
                return ToolResult(
                    content=[text_content(f"DENIED: {reason}")],
                    is_error=True,
                ).to_dict()
            if decision.modified_arguments is not None:
                arguments = decision.modified_arguments
        # --------------------------------------------------------------------

        context = ToolContext(session=session, arguments=arguments, services=self.services)
        # Tool execution errors are reported *inside* the result (isError=true)
        # so the model can see and self-correct, not as protocol errors.
        try:
            with session.lock:
                raw = tool.handler(context)
            result = normalize_result(raw)
        except errors.JsonRpcError:
            raise
        except Exception as exc:  # noqa: BLE001 - surface as tool error result
            from .registry import ToolResult, text_content

            result = ToolResult(
                content=[text_content(f"Tool '{name}' failed: {exc}")],
                is_error=True,
            )

        # --- result interception --------------------------------------------
        if control is not None and not skip_control:
            decision = control.submit_result(
                session, name, result.to_dict(), auto_approve=result.auto_approve
            )
            if not decision.approved:
                from .registry import ToolResult, text_content
                reason = decision.rejection_reason or "Tool result rejected by controller"
                return ToolResult(
                    content=[text_content(f"DENIED: {reason}")],
                    is_error=True,
                ).to_dict()
            if decision.modified_result is not None:
                return decision.modified_result
        # --------------------------------------------------------------------

        return result.to_dict()


def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
    """Minimal validation of *arguments* against an input JSON Schema.

    Only the top-level ``required`` list and primitive ``type`` of declared
    properties are checked — enough to give clients meaningful ``INVALID_PARAMS``
    errors without pulling in a full JSON Schema implementation.
    """
    required = schema.get("required", [])
    missing = [key for key in required if key not in arguments]
    if missing:
        raise errors.invalid_params(
            f"Missing required argument(s): {', '.join(missing)}",
            {"missing": missing},
        )

    properties = schema.get("properties", {})
    type_checks = {
        "string": str,
        "number": (int, float),
        "integer": int,
        "boolean": bool,
        "object": dict,
        "array": list,
    }
    for key, value in arguments.items():
        prop = properties.get(key)
        if not isinstance(prop, dict):
            continue
        expected = prop.get("type")
        py_type = type_checks.get(expected) if isinstance(expected, str) else None
        if py_type is None:
            continue
        # bool is a subclass of int; guard the integer/number cases explicitly.
        if expected in ("number", "integer") and isinstance(value, bool):
            ok = False
        else:
            ok = isinstance(value, py_type)
        if not ok:
            raise errors.invalid_params(
                f"Argument '{key}' must be of type {expected}",
                {"argument": key, "expectedType": expected},
            )
