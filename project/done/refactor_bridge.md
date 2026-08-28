Refaktoriere die Bridge in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp`.

- Die Bridge ist kein Manager, sondern ein Utillity das von den Tools verwendet wird.
- Jedes Tool soll sich selbst als MCP oder Function registrieren
- Die Python Function ist das primäre Prinzip, nicht das MCP. Dementsprechend muss ein Argument Transformator im Handler stecken, nicht in der Bridge.
	- Es gibt nur eine Anwendung der Transformator als Abstraktion ist überflüssig.
- Jedes Tool kann seine eigene Bridge Instanz haben
- Dementsprechend ruft die Core Function die Bridge auf und nicht der
SystemInit:  id=c313c7f6-ec23-4b07-8728-ab82d65c8cf8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 85c0-1
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp
  reason: Explore the MCP tools directory to understand the current bridge structure before refactoring.
```
/answer 85c0-1 allow
Control Request:
```yaml
id: 85c0-2
result:
  structuredContent:
    entries:
    - __init__.py
    - bridge.py
    - client.py
    - context7.py
    - exa.py
    - github.py
```
/answer 85c0-2 allow
ReasoningToken: 14
Control Request:
```yaml
id: 85c0-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  reason: Read the current bridge implementation to understand what needs to be refactored.
```
/answer 85c0-3 allow
Control Request:
```yaml
id: 85c0-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/client.py
  reason: Read the client module to understand how it interacts with the bridge.
```
/answer 85c0-4 allow
Control Request:
```yaml
id: 85c0-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  reason: Read the package init to understand exports and public API.
```
/answer 85c0-5 allow
Control Request:
```yaml
id: 85c0-6
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: Read an existing tool implementation using the bridge to understand usage pattern.
```
/answer 85c0-6 allow
Control Request:
```yaml
id: 85c0-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: Read an existing tool implementation using the bridge to understand usage pattern.
```
/answer 85c0-7 allow
Control Request:
```yaml
id: 85c0-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  reason: Read an existing tool implementation using the bridge to understand usage pattern.
```
/answer 85c0-8 allow
Control Request:
```yaml
id: 85c0-9
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  reason: "Locate registry.py, tool_context.py, function_registry.py to understand ToolResult, ToolContext, FunctionRegistry contracts used by the bridge and tools."
```
/answer 85c0-9 deny der Kontext is beschränkt nur auf diese Aufgabe und dieses Verzeichnis
ReasoningToken: 1048
Control Request:
```yaml
id: 85c0-a
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: Understand ToolRegistry.tool decorator contract and ToolResult class to correctly refactor the bridge/handler split.
```
/answer 85c0-a deny Ignoriere Dateien ausserhalb des Verzeichnisses
Control Request:
```yaml
id: 85c0-b
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  reason: Understand ToolContext and AppEnvironment contracts used by tool handlers.
```
/answer 85c0-b deny
Control Request:
```yaml
id: 85c0-c
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  reason: Understand FunctionRegistry.register contract used by tool modules to expose core functions.
```
/answer 85c0-c deny
ReasoningToken: 123
Control Request:
```yaml
id: 85c0-d
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  mode: replace
  content: |
    """Utility for forwarding hard-coded calls of an external MCP server.

    This is not a manager: an ``McpBridge`` owns nothing but a single lazily
    created :class:`McpClient` and a ``call()`` method that turns the remote
    ``CallToolResult`` into a :class:`ToolResult`. It knows nothing about
    ``ToolRegistry`` or ``FunctionRegistry`` – each tool module (``context7``,
    ``exa``, ``github``) builds its own bridge subclass instance, registers
    itself as an MCP tool and/or a plain function, and calls ``bridge.call()``
    from its own core function, shaping the remote arguments as needed there.
    """

    from __future__ import annotations

    import threading
    from typing import Any

    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolResult, text_content
    from xy.ai.mcpc.utils.text_sanitize import sanitize_text, sanitize_value
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError


    def compact(**kwargs: Any) -> dict[str, Any]:
        """Build a remote-call argument dict, dropping keys whose value is ``None``.

        Shared helper for the core functions in ``context7``, ``exa`` and
        ``github``, which forward only the arguments the caller actually
        supplied.
        """
        return {k: v for k, v in kwargs.items() if v is not None}


    class McpBridge:
        """Lazily connects to one external MCP server and forwards ``tools/call``."""

        def __init__(self, config: ServerConfig | None = None) -> None:
            self.config = config or ServerConfig()
            self._client: McpClient | None = None
            self._lock = threading.Lock()

        def build_client(self, config: ServerConfig) -> McpClient:
            """Create the client for the target server (called once, lazily)."""
            raise NotImplementedError

        def get_client(self) -> McpClient:
            with self._lock:
                if self._client is None:
                    self._client = self.build_client(self.config)
                return self._client

        def call(self, remote_tool: str, arguments: dict[str, Any]) -> ToolResult:
            """Forward a call and translate the outcome into a :class:`ToolResult`."""
            try:
                client = self.get_client()
                result = client.call_tool(remote_tool, arguments)
            except McpClientError as exc:
                msg = f"'{remote_tool}' failed: {exc}"
                return ToolResult(
                    content=[text_content(msg)],
                    is_error=True,
                )
            return _to_tool_result(result)


    def _to_tool_result(result: dict[str, Any]) -> ToolResult:
        """Mirror a remote ``CallToolResult`` into an MCPC :class:`ToolResult`."""
        is_error = bool(result.get("isError", False))

        # Extract the text blocks from the remote content array; this is only
        # ever surfaced to the agent as a ``content`` block on error (see below) –
        # emitting it a second time on success is what used to duplicate the
        # remote's answer next to structuredContent.
        raw_blocks = result.get("content")
        if isinstance(raw_blocks, list):
            texts = [
                block.get("text", "")
                for block in raw_blocks
                if isinstance(block, dict) and block.get("type") == "text"
            ]
            text = "\n".join(texts)
        else:
            text = ""
        # Some remote servers leak raw non-printable control bytes (e.g. an
        # unescaped 0x02) into text content; strip them so downstream consumers
        # (notably YAML block-scalar rendering) never choke on them.
        text = sanitize_text(text)

        # Mirrors the tools' own convention (see tools/CHECKLIST.md): a
        # successful result relies on structuredContent alone; errors are
        # reported purely through a readable text block, since agents commonly
        # read content[0].text for the error message.
        if is_error:
            return ToolResult(
                content=[text_content(text)] if text else [],
                is_error=True,
            )

        # Use structuredContent from the remote server when present. Otherwise
        # recover it from the text: some servers only ever fill in the text
        # block, and that text is frequently a JSON document that was serialised
        # to a string rather than left as real structure. Parsing it here keeps
        # that substructure intact instead of swallowing it into a flat
        # ``{"content": text}`` string.
        structured = result.get("structuredContent")
        if isinstance(structured, dict):
            structured_content = sanitize_value(structured)
        else:
            parsed = JsonCodec.try_decode(text)
            structured_content = sanitize_value(parsed) if isinstance(parsed, dict) else {"content": text}

        return ToolResult(
            content=[],
            structured_content=structured_content,
            is_error=False,
        )
  reason: "Rewrite the bridge as a pure utility (no registration/orchestration, no ArgTransform abstraction), per the requested refactor."
```
/answer 85c0-d allow
Control Request:
```yaml
id: 85c0-e
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  mode: replace
  content: |
    """Bridges that expose external MCP servers as MCPC tools."""

    from __future__ import annotations

    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, compact
    from xy.ai.mcpc.tools.mcp.client import DEFAULT_PROTOCOL_VERSION, McpClient, McpClientError
    from xy.ai.mcpc.tools.mcp.context7 import Context7Bridge, register_context7_tools
    from xy.ai.mcpc.tools.mcp.exa import ExaBridge, register_exa_tools
    from xy.ai.mcpc.tools.mcp.github import GitHubBridge, register_github_tools

    __all__ = [
        "Context7Bridge",
        "DEFAULT_PROTOCOL_VERSION",
        "ExaBridge",
        "GitHubBridge",
        "McpBridge",
        "McpClient",
        "McpClientError",
        "compact",
        "register_context7_tools",
        "register_exa_tools",
        "register_github_tools",
    ]
  reason: Drop the removed ArgTransform export from the package's public API.
```
/answer 85c0-e allow
Control Request:
```yaml
id: 85c0-f
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  old_text: |-
    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolRegistry
    from xy.ai.mcpc.tools.tool_context import AppEnvironment
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient
  new_text: |-
    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolRegistry, ToolResult
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient
  reason: Add the ToolResult/ToolContext imports needed now that context7's own registration handlers are written in this module instead of the bridge.
```
/answer 85c0-f allow
ReasoningToken: 25763
ReasoningToken: 9
Control Request:
```yaml
id: 85c0-10
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  old_text: |-
    def register_context7_tools(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        bridge: "Context7Bridge | None" = None,
    ) -> None:
        """Register the Context7-backed ``context7_libraries`` and
        ``context7_documentation`` tools."""
        bridge = bridge or Context7Bridge(environment.config if environment is not None else None)
        functions = environment.functions if environment is not None else None

        def context7_libraries(libraryName: str, query: str) -> dict:
            """Search Context7 for a library and return its canonical library ID.

            Best for: Resolving a library name to the ID needed by
            ``context7_documentation``.

            Args:
                libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
                query: User's original question or task, used for relevance ranking.

            Returns:
                dict with ``content``: ranked list of matching libraries (ID, title,
                description, snippet count, reputation, benchmark score, versions).
            """
            return bridge.call("resolve-library-id", compact(libraryName=libraryName, query=query)).to_dict()

        def context7_documentation(libraryId: str, query: str) -> dict:
            """Fetch documentation and code examples for a library from Context7.

            Best for: Retrieving accurate API docs, usage examples, and
            configuration guides for any library or framework. Use
            ``context7_libraries`` first to obtain the correct libraryId. Keep
            each query scoped to a single concept.

            Args:
                libraryId: Context7-compatible library ID as returned by
                    ``context7_libraries`` (e.g. '/reactjs/react.dev'), optionally
                    suffixed with a version.
                query: The question or task to find documentation for, scoped to
                    a single concept.

            Returns:
                dict with ``content``: documentation snippets and code examples.
            """
            return bridge.call("query-docs", compact(libraryId=libraryId, query=query)).to_dict()

        bridge.register_tool(
            registry,
            name="context7_libraries",
            remote_tool="resolve-library-id",
            title="Context7 resolve library ID",
            description=_RESOLVE_DESCRIPTION,
            input_schema=_RESOLVE_SCHEMA,
            output_schema=_RESOLVE_OUTPUT,
            annotations=_RO,
            functions=functions,
            core=context7_libraries,
        )
        bridge.register_tool(
            registry,
            name="context7_documentation",
            remote_tool="query-docs",
            title="Context7 query docs",
            description=_QUERY_DOCS_DESCRIPTION,
            input_schema=_QUERY_DOCS_SCHEMA,
            output_schema=_QUERY_DOCS_OUTPUT,
            annotations=_RO,
            functions=functions,
            core=context7_documentation,
        )
  new_text: |-
    def register_context7_tools(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        bridge: "Context7Bridge | None" = None,
    ) -> None:
        """Register the Context7-backed ``context7_libraries`` and
        ``context7_documentation`` tools."""
        bridge = bridge or Context7Bridge(environment.config if environment is not None else None)
        functions = environment.functions if environment is not None else None

        def context7_libraries(libraryName: str, query: str) -> ToolResult:
            """Search Context7 for a library and return its canonical library ID.

            Best for: Resolving a library name to the ID needed by
            ``context7_documentation``.

            Args:
                libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
                query: User's original question or task, used for relevance ranking.

            Returns:
                Ranked list of matching libraries (ID, title, description, snippet
                count, reputation, benchmark score, versions).
            """
            return bridge.call("resolve-library-id", compact(libraryName=libraryName, query=query))

        def context7_documentation(libraryId: str, query: str) -> ToolResult:
            """Fetch documentation and code examples for a library from Context7.

            Best for: Retrieving accurate API docs, usage examples, and
            configuration guides for any library or framework. Use
            ``context7_libraries`` first to obtain the correct libraryId. Keep
            each query scoped to a single concept.

            Args:
                libraryId: Context7-compatible library ID as returned by
                    ``context7_libraries`` (e.g. '/reactjs/react.dev'), optionally
                    suffixed with a version.
                query: The question or task to find documentation for, scoped to
                    a single concept.

            Returns:
                Documentation snippets and code examples relevant to the query.
            """
            return bridge.call("query-docs", compact(libraryId=libraryId, query=query))

        @registry.tool(
            "context7_libraries",
            title="Context7 resolve library ID",
            description=_RESOLVE_DESCRIPTION,
            input_schema=_RESOLVE_SCHEMA,
            output_schema=_RESOLVE_OUTPUT,
            annotations=_RO,
        )
        def _libraries_tool(ctx: ToolContext) -> ToolResult:
            return context7_libraries(**ctx.arguments)

        @registry.tool(
            "context7_documentation",
            title="Context7 query docs",
            description=_QUERY_DOCS_DESCRIPTION,
            input_schema=_QUERY_DOCS_SCHEMA,
            output_schema=_QUERY_DOCS_OUTPUT,
            annotations=_RO,
        )
        def _documentation_tool(ctx: ToolContext) -> ToolResult:
            return context7_documentation(**ctx.arguments)

        if functions is not None:
            functions.register(context7_libraries, id="context7_libraries")
            functions.register(context7_documentation, id="context7_documentation")
  reason: "Have each tool register itself directly (MCP handler delegating to the core function, core function calling the bridge) instead of going through the removed generic McpBridge.register_tool."
```
/answer 85c0-10 allow
Control Request:
```yaml
id: 85c0-11
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  old_text: |-
    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolRegistry
    from xy.ai.mcpc.tools.tool_context import AppEnvironment
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError
  new_text: |-
    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolRegistry, ToolResult
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError
  reason: Add ToolResult/ToolContext imports needed for exa's self-registration.
```
/answer 85c0-11 deny Nutze nicht den Decorator zur Registriern sondern die ABC ToolDefinition
Control Request:
```yaml
id: 85c0-12
toolName: ask_user
arguments:
  question: "Für die Registrierung soll ich statt des `@registry.tool(...)`-Decorators die ABC `ToolDefinition` verwenden. Kannst du mir zeigen, wie `ToolDefinition` aussieht (Felder/Methoden, z.B. in `tools/registry.py`), oder mir kurz ein Beispiel geben, wie ein Tool damit registriert wird? Ich habe aktuell nur Lesezugriff auf `tools/mcp/`."
  reason: Die vorhandenen Bridge/Tool-Dateien verwenden bisher ausschließlich den registry.tool-Decorator; ohne Einsicht in registry.py kenne ich die Signatur der ToolDefinition-ABC nicht und kann die Registrierung sonst nicht korrekt umbauen.
```
/answer 85c0-12 allow Ja schau dir dieses beispiel an `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py`
Control Request:
```yaml
id: 85c0-13
result:
  content:
  - type: text
    text: "Tool 'ask_user' failed: 'str' object has no attribute 'answer'"
  isError: true
```
/answer 85c0-13 deny Ja schau dir dieses beispiel an `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py`
Control Request:
```yaml
id: 85c0-14
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  reason: "Study a reference tool implementation that uses the ToolDefinition ABC instead of the registry.tool decorator, to correctly refactor the mcp bridge tools."
```
/answer 85c0-14 allow
ReasoningToken: 17
Text:
 Jetzt exa.py:
ReasoningToken: 967
Control Request:
```yaml
id: 85c0-15
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  mode: replace
  content: |
    """Utility for forwarding hard-coded calls of an external MCP server.

    This is not a manager: an ``McpBridge`` owns nothing but a single lazily
    created :class:`McpClient` and a ``call()`` method that resolves a remote
    ``CallToolResult`` into plain structured data, raising :class:`McpBridgeError`
    on failure. It knows nothing about ``ToolRegistry``, ``ToolDefinition`` or
    ``FunctionRegistry`` – each tool module (``context7``, ``exa``, ``github``)
    builds its own bridge subclass instance, exposes its own plain ``core``
    functions that call ``bridge.call()`` (shaping arguments as needed there),
    and wraps each one in its own ``ToolDefinition`` for MCP registration.
    """

    from __future__ import annotations

    import threading
    from typing import Any

    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.utils.text_sanitize import sanitize_text, sanitize_value
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError


    def compact(**kwargs: Any) -> dict[str, Any]:
        """Build a remote-call argument dict, dropping keys whose value is ``None``.

        Shared helper for the core functions in ``context7``, ``exa`` and
        ``github``, which forward only the arguments the caller actually
        supplied.
        """
        return {k: v for k, v in kwargs.items() if v is not None}


    class McpBridgeError(RuntimeError):
        """Raised when a forwarded call fails, at transport level or because the
        remote tool itself reported ``isError``."""


    class McpBridge:
        """Lazily connects to one external MCP server and forwards ``tools/call``."""

        def __init__(self, config: ServerConfig | None = None) -> None:
            self.config = config or ServerConfig()
            self._client: McpClient | None = None
            self._lock = threading.Lock()

        def build_client(self, config: ServerConfig) -> McpClient:
            """Create the client for the target server (called once, lazily)."""
            raise NotImplementedError

        def get_client(self) -> McpClient:
            with self._lock:
                if self._client is None:
                    self._client = self.build_client(self.config)
                return self._client

        def call(self, remote_tool: str, arguments: dict[str, Any]) -> dict[str, Any]:
            """Forward a call and return the remote's structured result.

            Raises:
                McpBridgeError: if the transport/protocol fails, or the remote
                    tool call itself reports ``isError``.
            """
            try:
                client = self.get_client()
                result = client.call_tool(remote_tool, arguments)
            except McpClientError as exc:
                raise McpBridgeError(f"'{remote_tool}' failed: {exc}") from exc
            return _extract_result(remote_tool, result)


    def _extract_result(remote_tool: str, result: dict[str, Any]) -> dict[str, Any]:
        """Resolve a remote ``CallToolResult`` into structured data, or raise."""
        # Extract the text blocks from the remote content array; on error this is
        # the only material the agent gets to see, so it also becomes the
        # McpBridgeError message.
        raw_blocks = result.get("content")
        if isinstance(raw_blocks, list):
            texts = [
                block.get("text", "")
                for block in raw_blocks
                if isinstance(block, dict) and block.get("type") == "text"
            ]
            text = "\n".join(texts)
        else:
            text = ""
        # Some remote servers leak raw non-printable control bytes (e.g. an
        # unescaped 0x02) into text content; strip them so downstream consumers
        # (notably YAML block-scalar rendering) never choke on them.
        text = sanitize_text(text)

        if result.get("isError", False):
            raise McpBridgeError(text or f"'{remote_tool}' failed")

        # Use structuredContent from the remote server when present. Otherwise
        # recover it from the text: some servers only ever fill in the text
        # block, and that text is frequently a JSON document that was serialised
        # to a string rather than left as real structure. Parsing it here keeps
        # that substructure intact instead of swallowing it into a flat
        # ``{"content": text}`` string.
        structured = result.get("structuredContent")
        if isinstance(structured, dict):
            return sanitize_value(structured)
        parsed = JsonCodec.try_decode(text)
        return sanitize_value(parsed) if isinstance(parsed, dict) else {"content": text}
  reason: "Rewrite the bridge as a pure utility that returns plain structured data and raises McpBridgeError on failure, so the ToolDefinition-based handlers (which now call the core functions, not the bridge) can catch a domain exception instead of receiving a pre-built ToolResult from the bridge."
```
/answer 85c0-15 allow
Control Request:
```yaml
id: 85c0-16
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  mode: replace
  content: |
    """Bridges that expose external MCP servers as MCPC tools."""

    from __future__ import annotations

    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import DEFAULT_PROTOCOL_VERSION, McpClient, McpClientError
    from xy.ai.mcpc.tools.mcp.context7 import Context7Bridge, register_context7_tools
    from xy.ai.mcpc.tools.mcp.exa import ExaBridge, register_exa_tools
    from xy.ai.mcpc.tools.mcp.github import GitHubBridge, register_github_tools

    __all__ = [
        "Context7Bridge",
        "DEFAULT_PROTOCOL_VERSION",
        "ExaBridge",
        "GitHubBridge",
        "McpBridge",
        "McpBridgeError",
        "McpClient",
        "McpClientError",
        "compact",
        "register_context7_tools",
        "register_exa_tools",
        "register_github_tools",
    ]
  reason: "Export the new McpBridgeError alongside the pure-utility bridge, dropping the removed decorator-registration helper."
```
/answer 85c0-16 allow
Control Request:
```yaml
id: 85c0-17
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: Re-read the current state of context7.py before rewriting the registration part to use ToolDefinition instead of the decorator.
```
/answer 85c0-17 allow
ReasoningToken: 8879
Control Request:
```yaml
id: 85c0-18
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  mode: replace
  content: |
    """Context7 bridge – library documentation tools backed by the Context7 MCP server.

    Exposes two tools:
      context7_libraries      →  resolveLibraryId
      context7_documentation  →  queryDocs
    """

    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient

    _RESOLVE_DESCRIPTION = (
        "Search Context7 for a library and return its canonical library ID.\n\n"
        "Best for: Resolving a library name to the ID needed by context7_documentation.\n"
        "Returns: Ranked list of matching libraries with ID, title, description, "
        "snippet count, reputation, benchmark score, and available versions."
    )
    _RESOLVE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "libraryName": {
                "type": "string",
                "description": "Library name to search for (e.g. 'react', 'next.js', 'vue').",
            },
            "query": {
                "type": "string",
                "description": (
                    "User's original question or task – used for relevance ranking "
                    "(e.g. 'How to manage state with hooks')."
                ),
            },
        },
        "required": ["libraryName", "query"],
    }
    _RESOLVE_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": (
                    "Ranked list of matching libraries. Each entry contains "
                    "Context7-compatible library ID, title, description, code snippet count, "
                    "source reputation, benchmark score, and available versions."
                ),
            },
        },
        "required": ["content"],
    }

    _QUERY_DOCS_DESCRIPTION = (
        "Fetch documentation and code examples for a library from Context7.\n\n"
        "Best for: Retrieving accurate API docs, usage examples, and configuration guides "
        "for any library or framework.\n"
        "Use context7_libraries first to obtain the correct libraryId.\n"
        "Returns: Documentation snippets and code examples relevant to the query.\n\n"
        "Keep each query scoped to a single concept. For multi-concept questions, "
        "make separate calls per concept unless the question is about how the concepts interact.\n"
    )
    _QUERY_DOCS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "libraryId": {
                "type": "string",
                "description": (
                    "Context7-compatible library ID as returned by context7_libraries "
                    "(e.g. '/reactjs/react.dev', '/vercel/next.js'). "
                    "Optionally suffix with a version: '/vercel/next.js/v14.3.0'."
                ),
            },
            "query": {
                "type": "string",
                "description": (
                    "The question or task to find documentation for, scoped to a single concept. "
                    "Be specific and include relevant details "
                    "(e.g. 'React useEffect cleanup function examples')."
                ),
            },
        },
        "required": ["libraryId", "query"],
    }
    _QUERY_DOCS_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Documentation snippets and code examples relevant to the query.",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


    class Context7Bridge(McpBridge):
        """Bridge to the Context7 remote MCP server."""

        def build_client(self, config: ServerConfig) -> McpClient:
            headers: dict[str, str] = {}
            if config.context7_api_key:
                headers["CONTEXT7_API_KEY"] = config.context7_api_key
            return McpClient(config.context7_mcp_url, headers=headers)


    def register_context7_tools(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        bridge: "Context7Bridge | None" = None,
    ) -> None:
        """Register the Context7-backed ``context7_libraries`` and
        ``context7_documentation`` tools."""
        bridge = bridge or Context7Bridge(environment.config if environment is not None else None)
        functions = environment.functions if environment is not None else None

        def context7_libraries(libraryName: str, query: str) -> dict:
            """Search Context7 for a library and return its canonical library ID.

            Best for: Resolving a library name to the ID needed by
            ``context7_documentation``.

            Args:
                libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
                query: User's original question or task, used for relevance ranking.

            Returns:
                Ranked list of matching libraries (ID, title, description, snippet
                count, reputation, benchmark score, versions).

            Raises:
                McpBridgeError: if the Context7 call fails.
            """
            return bridge.call("resolve-library-id", compact(libraryName=libraryName, query=query))

        def context7_documentation(libraryId: str, query: str) -> dict:
            """Fetch documentation and code examples for a library from Context7.

            Best for: Retrieving accurate API docs, usage examples, and
            configuration guides for any library or framework. Use
            ``context7_libraries`` first to obtain the correct libraryId. Keep
            each query scoped to a single concept.

            Args:
                libraryId: Context7-compatible library ID as returned by
                    ``context7_libraries`` (e.g. '/reactjs/react.dev'), optionally
                    suffixed with a version.
                query: The question or task to find documentation for, scoped to
                    a single concept.

            Returns:
                Documentation snippets and code examples relevant to the query.

            Raises:
                McpBridgeError: if the Context7 call fails.
            """
            return bridge.call("query-docs", compact(libraryId=libraryId, query=query))

        class Context7LibrariesTool(ToolDefinition):
            name = "context7_libraries"
            title = "Context7 resolve library ID"
            description = _RESOLVE_DESCRIPTION
            input_schema = _RESOLVE_SCHEMA
            output_schema = _RESOLVE_OUTPUT
            annotations = _RO

            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    result = context7_libraries(libraryName=args["libraryName"], query=args["query"])
                except McpBridgeError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content=result)

        class Context7DocumentationTool(ToolDefinition):
            name = "context7_documentation"
            title = "Context7 query docs"
            description = _QUERY_DOCS_DESCRIPTION
            input_schema = _QUERY_DOCS_SCHEMA
            output_schema = _QUERY_DOCS_OUTPUT
            annotations = _RO

            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    result = context7_documentation(libraryId=args["libraryId"], query=args["query"])
                except McpBridgeError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content=result)

        registry.register(Context7LibrariesTool())
        registry.register(Context7DocumentationTool())
        if functions is not None:
            functions.register(context7_libraries)
            functions.register(context7_documentation)
  reason: "Rewrite context7 registration to use the ToolDefinition ABC instead of the registry.tool decorator: each ToolDefinition.handle() is the argument transformer (MCP args -> core call -> ToolResult/error), while the plain core function is the one that calls the bridge."
```
/answer 85c0-18 allow
ReasoningToken: 20
Text:
 Jetzt context7.py komplett neu mit `ToolDefinition`-Pattern:
Control Request:
```yaml
id: 85c0-19
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  mode: replace
  content: |
    """Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa's MCP.

    Exa is reached through its remote MCP server; the API key is taken from the
    server configuration.  MCPC advertises its own descriptions and input schemas.
    """

    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

    _WEB_SEARCH_DESCRIPTION = (
        "Search the web for any topic and get clean, ready-to-use content.\n\n"
        "Best for: Finding current information, facts, or "
        "answering questions about any topic.\n"
        "Returns: Clean text content from top search results."
    )
    _WEB_SEARCH_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "Natural language search query. Should be a semantically rich "
                    "description of the ideal page."
                ),
            },
            "numResults": {
                "type": "integer",
                "description": "Number of search results to return (default: 10).",
                "minimum": 1,
            },
        },
        "required": ["query"],
    }
    _SEARCH_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Clean text content from the top search results.",
            },
        },
        "required": ["content"],
    }

    _WEB_FETCH_DESCRIPTION = (
        "Read a webpage's full content as clean markdown. Use to read any URL.\n\n"
        "Best for: Extracting full content from known URLs. Batch multiple URLs in "
        "one call.\n"
        "Returns: Clean text content and metadata from the page(s)."
    )
    _WEB_FETCH_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "urls": {
                "type": "array",
                "items": {"type": "string"},
                "description": "URLs to read. Batch multiple URLs in one call.",
            },
            "maxCharacters": {
                "type": "integer",
                "description": "Maximum characters to extract per page (default: 3000)",
                "minimum": 1,
            },
        },
        "required": ["urls"],
    }
    _FETCH_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Clean text content extracted from the requested page(s).",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


    def _coerce_urls(urls: list[str] | str) -> list[str]:
        """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
        if isinstance(urls, str):
            # Accept a JSON-encoded list carried as a string; a plain URL (or any
            # non-list) is wrapped as a single-element list.
            parsed = JsonCodec.try_decode(urls)
            return parsed if isinstance(parsed, list) else [urls]
        return urls


    class ExaBridge(McpBridge):
        """Bridge to the Exa remote MCP server."""

        def build_client(self, config: ServerConfig) -> McpClient:
            api_key = config.exa_api_key
            if not api_key:
                raise McpClientError(
                    "Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY)."
                )
            return McpClient(config.exa_mcp_url, headers={"x-api-key": api_key})


    def register_exa_tools(
        registry: ToolRegistry,
        environment: "AppEnvironment | None" = None,
        bridge: "ExaBridge | None" = None,
    ) -> None:
        """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
        bridge = bridge or ExaBridge(environment.config if environment is not None else None)
        functions = environment.functions if environment is not None else None

        def web_search_exa(query: str, numResults: int | None = None) -> dict:
            """Search the web for any topic and get clean, ready-to-use content.

            Best for: Finding current information, facts, or answering questions
            about any topic.

            Args:
                query: Natural language search query; should be a semantically
                    rich description of the ideal page.
                numResults: Number of search results to return (default: 10).

            Returns:
                Clean text content from the top results.

            Raises:
                McpBridgeError: if the Exa call fails.
            """
            return bridge.call("web_search_exa", compact(query=query, numResults=numResults))

        def web_fetch_exa(urls: list[str] | str, maxCharacters: int | None = None) -> dict:
            """Read a webpage's full content as clean markdown.

            Best for: Extracting full content from known URLs. Batch multiple
            URLs in one call.

            Args:
                urls: URL(s) to read; a single URL or a list of URLs.
                maxCharacters: Maximum characters to extract per page (default: 3000).

            Returns:
                Clean text content and metadata from the page(s).

            Raises:
                McpBridgeError: if the Exa call fails.
            """
            arguments = compact(urls=_coerce_urls(urls), maxCharacters=maxCharacters)
            return bridge.call("web_fetch_exa", arguments)

        class WebSearchExaTool(ToolDefinition):
            name = "web_search_exa"
            title = "Exa web search"
            description = _WEB_SEARCH_DESCRIPTION
            input_schema = _WEB_SEARCH_SCHEMA
            output_schema = _SEARCH_OUTPUT_SCHEMA
            annotations = _RO

            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    result = web_search_exa(query=args["query"], numResults=args.get("numResults"))
                except McpBridgeError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content=result)

        class WebFetchExaTool(ToolDefinition):
            name = "web_fetch_exa"
            title = "Exa web fetch"
            description = _WEB_FETCH_DESCRIPTION
            input_schema = _WEB_FETCH_SCHEMA
            output_schema = _FETCH_OUTPUT_SCHEMA
            annotations = _RO

            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    result = web_fetch_exa(urls=args["urls"], maxCharacters=args.get("maxCharacters"))
                except McpBridgeError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content=result)

        registry.register(WebSearchExaTool())
        registry.register(WebFetchExaTool())
        if functions is not None:
            functions.register(web_search_exa)
            functions.register(web_fetch_exa)
  reason: "Rewrite exa registration to use ToolDefinition; the URL leniency transform now exists exactly once, inside the web_fetch_exa core function, instead of duplicated as a bridge-level ArgTransform."
```
/answer 85c0-19 allow
ReasoningToken: 37
Text:
 Jetzt exa.py komplett neu schreiben:
Control Request:
```yaml
id: 85c0-1a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  mode: replace
  content: |
    """GitHub bridge – read-only research tools backed by GitHub's remote MCP server.

    Only read-only tools are registered: file/code access, issues, discussions,
    pull requests, commits, and project information.
    """

    from __future__ import annotations

    from typing import Any, Callable

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

    _CONTENT_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Response content from the GitHub MCP server.",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}

    _GET_FILE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner (user or org)."},
            "repo": {"type": "string", "description": "Repository name."},
            "path": {
                "type": "string",
                "description": "Path to the file or directory (omit for root).",
            },
            "ref": {
                "type": "string",
                "description": (
                    "Branch, tag, or ref (e.g. refs/heads/main, refs/pull/42/head). "
                    "Ignored when sha is provided."
                ),
            },
            "sha": {
                "type": "string",
                "description": "Exact commit SHA; takes precedence over ref.",
            },
        },
        "required": ["owner", "repo"],
    }

    _GET_TREE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "tree_sha": {
                "type": "string",
                "description": "SHA, branch, or tag to read the tree from (defaults to default branch).",
            },
            "recursive": {
                "type": "boolean",
                "description": "Recurse into sub-trees (default false).",
            },
            "path_filter": {
                "type": "string",
                "description": "Optional path prefix to filter results (e.g. 'src/').",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_CODE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "GitHub code search query (max 256 chars). "
                    "Qualifiers: repo:owner/repo, org:, language:, path:, "
                    "filename:, extension:, in:file|path."
                ),
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _SEARCH_COMMITS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "GitHub commit search query. Scope with repo:owner/repo or org:. "
                    "Qualifiers: author:, committer:, author-date:, committer-date:, "
                    "merge:true|false, hash:."
                ),
            },
            "sort": {
                "type": "string",
                "description": "Sort by author-date or committer-date (defaults to best match).",
            },
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _SEARCH_REPOS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "Repository search query. Supports qualifiers: "
                    "topic:, language:, stars:>N, user:, org:, is:archived."
                ),
            },
            "sort": {
                "type": "string",
                "description": "Sort by: stars | forks | help-wanted-issues | updated.",
            },
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "minimal_output": {
                "type": "boolean",
                "description": "Return minimal repository info (default true).",
            },
        },
        "required": ["query"],
    }

    _ISSUE_READ_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "issue_number": {"type": "integer", "description": "Issue number."},
            "method": {
                "type": "string",
                "description": (
                    "Read operation to perform:\n"
                    "  get – issue body and metadata\n"
                    "  get_comments – issue comments\n"
                    "  get_sub_issues – child issues\n"
                    "  get_parent – parent issue (if this is a sub-issue)\n"
                    "  get_labels – labels assigned to the issue"
                ),
                "enum": ["get", "get_comments", "get_sub_issues", "get_parent", "get_labels"],
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
        },
        "required": ["owner", "repo", "issue_number", "method"],
    }

    _LIST_ISSUES_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "state": {
                "type": "string",
                "description": "Filter by state: open | closed (default: both).",
            },
            "labels": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Filter by label names.",
            },
            "since": {
                "type": "string",
                "description": "Only issues updated after this ISO 8601 timestamp.",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (from previous response).",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_ISSUES_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Search query using GitHub issues search syntax.",
            },
            "owner": {
                "type": "string",
                "description": "Restrict to this owner (requires repo).",
            },
            "repo": {
                "type": "string",
                "description": "Restrict to this repo (requires owner).",
            },
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _GET_DISCUSSION_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "discussionNumber": {"type": "integer", "description": "Discussion number."},
        },
        "required": ["owner", "repo", "discussionNumber"],
    }

    _GET_DISCUSSION_COMMENTS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "discussionNumber": {"type": "integer", "description": "Discussion number."},
            "includeReplies": {
                "type": "boolean",
                "description": "Include nested replies per comment (up to 100, default false).",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Cursor for pagination."},
        },
        "required": ["owner", "repo", "discussionNumber"],
    }

    _LIST_DISCUSSIONS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner or org."},
            "repo": {
                "type": "string",
                "description": "Repository name (omit for org-level discussions).",
            },
            "category": {
                "type": "string",
                "description": "Filter by discussion category ID.",
            },
            "orderBy": {
                "type": "string",
                "description": "Order by field (requires direction).",
            },
            "direction": {"type": "string", "description": "Order direction: ASC | DESC."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Cursor for pagination."},
        },
        "required": ["owner"],
    }

    _PR_READ_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "pullNumber": {"type": "integer", "description": "Pull request number."},
            "method": {
                "type": "string",
                "description": (
                    "Data to retrieve:\n"
                    "  get – PR body and metadata\n"
                    "  get_diff – unified diff\n"
                    "  get_status – combined commit status\n"
                    "  get_files – changed files\n"
                    "  get_commits – commits on the PR\n"
                    "  get_review_comments – review threads\n"
                    "  get_reviews – review summaries\n"
                    "  get_comments – general comments\n"
                    "  get_check_runs – CI check runs"
                ),
                "enum": [
                    "get",
                    "get_diff",
                    "get_status",
                    "get_files",
                    "get_commits",
                    "get_review_comments",
                    "get_reviews",
                    "get_comments",
                    "get_check_runs",
                ],
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (get_review_comments only).",
            },
        },
        "required": ["owner", "repo", "pullNumber", "method"],
    }

    _LIST_PRS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "state": {"type": "string", "description": "Filter: open | closed | all."},
            "base": {"type": "string", "description": "Filter by base branch name."},
            "sort": {
                "type": "string",
                "description": "Sort by: created | updated | popularity | long-running.",
            },
            "direction": {"type": "string", "description": "Sort direction: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_PRS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Search query using GitHub pull request search syntax.",
            },
            "owner": {
                "type": "string",
                "description": "Restrict to this owner (requires repo).",
            },
            "repo": {
                "type": "string",
                "description": "Restrict to this repo (requires owner).",
            },
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _GET_COMMIT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "sha": {"type": "string", "description": "Commit SHA, branch name, or tag name."},
            "detail": {
                "type": "string",
                "description": (
                    "File detail level:\n"
                    "  none – omit files entirely\n"
                    "  stats – per-file counts (default)\n"
                    "  full_patch – includes diff content (can be large)"
                ),
                "enum": ["none", "stats", "full_patch"],
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo", "sha"],
    }

    _LIST_COMMITS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "sha": {
                "type": "string",
                "description": "Branch, tag, or SHA to list commits from (defaults to default branch).",
            },
            "path": {
                "type": "string",
                "description": "Only commits touching this file path.",
            },
            "author": {
                "type": "string",
                "description": "Filter by author username or email.",
            },
            "since": {
                "type": "string",
                "description": "Only commits after this date (ISO 8601: YYYY-MM-DDTHH:MM:SSZ).",
            },
            "until": {
                "type": "string",
                "description": "Only commits before this date (ISO 8601).",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo"],
    }

    _PROJECTS_GET_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "method": {
                "type": "string",
                "description": (
                    "Operation:\n"
                    "  get_project – project metadata\n"
                    "  get_project_field – a single project field\n"
                    "  get_project_item – a single project item\n"
                    "  get_project_status_update – a status update"
                ),
                "enum": [
                    "get_project",
                    "get_project_field",
                    "get_project_item",
                    "get_project_status_update",
                ],
            },
            "owner": {
                "type": "string",
                "description": "Owner (user or org login).",
            },
            "owner_type": {
                "type": "string",
                "description": "Owner type: user | org (auto-detected if omitted).",
            },
            "project_number": {"type": "integer", "description": "Project number."},
            "field_id": {
                "type": "integer",
                "description": "Field ID (required for get_project_field).",
            },
            "item_id": {
                "type": "integer",
                "description": "Item ID (required for get_project_item).",
            },
            "fields": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Field IDs to include in get_project_item response.",
            },
            "status_update_id": {
                "type": "string",
                "description": "Status update node ID (required for get_project_status_update).",
            },
        },
        "required": ["method"],
    }

    _PROJECTS_LIST_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "method": {
                "type": "string",
                "description": (
                    "Operation:\n"
                    "  list_projects – projects for an owner\n"
                    "  list_project_fields – fields of a project\n"
                    "  list_project_items – items in a project\n"
                    "  list_project_status_updates – status updates"
                ),
                "enum": [
                    "list_projects",
                    "list_project_fields",
                    "list_project_items",
                    "list_project_status_updates",
                ],
            },
            "owner": {"type": "string", "description": "Owner (user or org login)."},
            "owner_type": {
                "type": "string",
                "description": "Owner type: user | org.",
            },
            "project_number": {
                "type": "integer",
                "description": "Project number (required for fields, items, and status updates).",
            },
            "query": {
                "type": "string",
                "description": (
                    "Filter string: for list_projects use title/state filters; "
                    "for list_project_items use GitHub project filter syntax."
                ),
            },
            "fields": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Field IDs to include for list_project_items.",
            },
            "per_page": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Forward pagination cursor."},
            "before": {"type": "string", "description": "Backward pagination cursor."},
        },
        "required": ["method", "owner"],
    }


    class GitHubBridge(McpBridge):
        """Bridge to the GitHub remote MCP server (read-only)."""

        def build_client(self, config: ServerConfig) -> McpClient:
            pat = config.github_api_pat
            if not pat:
                raise McpClientError(
                    "GitHub PAT is not configured (set MCPC_GITHUB_PAT)."
                )
            return McpClient(
                config.github_mcp_url,
                headers={"Authorization": f"Bearer {pat}"},
            )


    def _tool_class(
        name: str, title: str, description: str, input_schema: dict[str, Any], core: Callable[..., dict]
    ) -> ToolDefinition:
        """Build a :class:`ToolDefinition` instance that forwards ``ctx.arguments``
        straight to *core*, translating an :class:`McpBridgeError` into an error
        :class:`ToolResult`. *core* remains the only thing that talks to the bridge.
        """

        def handle(self: ToolDefinition, ctx: ToolContext) -> ToolResult:
            try:
                result = core(**ctx.arguments)
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)

        return type(
            f"_{name}_tool",
            (ToolDefinition,),
            {
                "name": name,
                "title": title,
                "description": description,
                "input_schema": input_schema,
                "output_schema": _CONTENT_OUTPUT,
                "annotations": _RO,
                "handle": handle,
            },
        )()


    def register_github_tools(
        registry: ToolRegistry,
        environment: AppEnvironment,
    ) -> None:
        """Register read-only GitHub research tools."""
        bridge = GitHubBridge(environment.config)
        functions = environment.functions

        def github_get_file(
            owner: str, repo: str, path: str | None = None, ref: str | None = None, sha: str | None = None
        ) -> dict:
            """Read a file or directory listing from a GitHub repository.

            Best for: Fetching source code, configs, and READMEs at any ref or commit.

            Args:
                owner: Repository owner (user or org).
                repo: Repository name.
                path: Path to the file or directory (omit for root).
                ref: Branch, tag, or ref (e.g. refs/heads/main); ignored when sha is given.
                sha: Exact commit SHA; takes precedence over ref.
            """
            return bridge.call("get_file_contents", compact(owner=owner, repo=repo, path=path, ref=ref, sha=sha))

        def github_get_tree(
            owner: str,
            repo: str,
            tree_sha: str | None = None,
            recursive: bool | None = None,
            path_filter: str | None = None,
        ) -> dict:
            """List the file tree of a GitHub repository at a given ref.

            Best for: Understanding project layout before reading individual files.

            Args:
                owner: Repository owner.
                repo: Repository name.
                tree_sha: SHA, branch, or tag to read the tree from (defaults to default branch).
                recursive: Recurse into sub-trees (default false).
                path_filter: Optional path prefix to filter results (e.g. 'src/').
            """
            return bridge.call(
                "get_repository_tree",
                compact(owner=owner, repo=repo, tree_sha=tree_sha, recursive=recursive, path_filter=path_filter),
            )

        def github_search_code(query: str, perPage: int | None = None, page: int | None = None) -> dict:
            """Search GitHub code across repositories.

            Best for: Finding specific functions, patterns, or usages across the
            GitHub ecosystem.

            Args:
                query: GitHub code search query (max 256 chars); qualifiers:
                    repo:owner/repo, org:, language:, path:, filename:, extension:, in:file|path.
                perPage: Results per page (max 15).
                page: Page number (min 1).
            """
            return bridge.call("search_code", compact(query=query, perPage=perPage, page=page))

        def github_search_commits(
            query: str,
            sort: str | None = None,
            order: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """Search commit messages on GitHub.

            Best for: Finding commits by message keyword, author, or date across repositories.

            Args:
                query: GitHub commit search query; scope with repo:owner/repo or org:;
                    qualifiers: author:, committer:, author-date:, committer-date:,
                    merge:true|false, hash:.
                sort: Sort by author-date or committer-date (defaults to best match).
                order: Sort order: asc | desc.
                perPage: Results per page (max 15).
                page: Page number (min 1).
            """
            return bridge.call(
                "search_commits", compact(query=query, sort=sort, order=order, perPage=perPage, page=page)
            )

        def github_search_repos(
            query: str,
            sort: str | None = None,
            order: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
            minimal_output: bool | None = None,
        ) -> dict:
            """Search GitHub for repositories matching a query.

            Best for: Discovering projects by name, topic, language, or stars.

            Args:
                query: Repository search query; qualifiers: topic:, language:,
                    stars:>N, user:, org:, is:archived.
                sort: Sort by: stars | forks | help-wanted-issues | updated.
                order: Sort order: asc | desc.
                perPage: Results per page (max 10).
                page: Page number (min 1).
                minimal_output: Return minimal repository info (default true).
            """
            return bridge.call(
                "search_repositories",
                compact(query=query, sort=sort, order=order, perPage=perPage, page=page, minimal_output=minimal_output),
            )

        def github_issue_read(
            owner: str,
            repo: str,
            issue_number: int,
            method: str,
            page: int | None = None,
            perPage: int | None = None,
        ) -> dict:
            """Read a GitHub issue: body, comments, sub-issues, labels, or parent.

            Args:
                owner: Repository owner.
                repo: Repository name.
                issue_number: Issue number.
                method: One of get | get_comments | get_sub_issues | get_parent | get_labels.
                page: Page number (min 1).
                perPage: Results per page (max 20).
            """
            return bridge.call(
                "issue_read",
                compact(owner=owner, repo=repo, issue_number=issue_number, method=method, page=page, perPage=perPage),
            )

        def github_list_issues(
            owner: str,
            repo: str,
            state: str | None = None,
            labels: list[str] | None = None,
            since: str | None = None,
            perPage: int | None = None,
            after: str | None = None,
        ) -> dict:
            """List issues in a GitHub repository with optional filters.

            Best for: Enumerating open or closed issues, filtering by label or state.

            Args:
                owner: Repository owner.
                repo: Repository name.
                state: Filter by state: open | closed (default: both).
                labels: Filter by label names.
                since: Only issues updated after this ISO 8601 timestamp.
                perPage: Results per page (max 15).
                after: Cursor for pagination (from previous response).
            """
            return bridge.call(
                "list_issues",
                compact(owner=owner, repo=repo, state=state, labels=labels, since=since, perPage=perPage, after=after),
            )

        def github_search_issues(
            query: str,
            owner: str | None = None,
            repo: str | None = None,
            sort: str | None = None,
            order: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """Search GitHub issues using GitHub's issue search syntax.

            Best for: Finding issues by keyword, author, label, or state across repositories.

            Args:
                query: Search query using GitHub issues search syntax.
                owner: Restrict to this owner (requires repo).
                repo: Restrict to this repo (requires owner).
                sort: Sort field.
                order: Sort order: asc | desc.
                perPage: Results per page (max 15).
                page: Page number (min 1).
            """
            return bridge.call(
                "search_issues",
                compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
            )

        def github_get_discussion(owner: str, repo: str, discussionNumber: int) -> dict:
            """Get the body and metadata of a single GitHub Discussion.

            Best for: Reading a specific community discussion or Q&A thread.

            Args:
                owner: Repository owner.
                repo: Repository name.
                discussionNumber: Discussion number.
            """
            return bridge.call(
                "get_discussion", compact(owner=owner, repo=repo, discussionNumber=discussionNumber)
            )

        def github_get_discussion_comments(
            owner: str,
            repo: str,
            discussionNumber: int,
            includeReplies: bool | None = None,
            perPage: int | None = None,
            after: str | None = None,
        ) -> dict:
            """Get comments for a GitHub Discussion, optionally including nested replies.

            Best for: Reading community feedback, answers, and Q&A responses.

            Args:
                owner: Repository owner.
                repo: Repository name.
                discussionNumber: Discussion number.
                includeReplies: Include nested replies per comment (up to 100, default false).
                perPage: Results per page (max 20).
                after: Cursor for pagination.
            """
            return bridge.call(
                "get_discussion_comments",
                compact(
                    owner=owner,
                    repo=repo,
                    discussionNumber=discussionNumber,
                    includeReplies=includeReplies,
                    perPage=perPage,
                    after=after,
                ),
            )

        def github_list_discussions(
            owner: str,
            repo: str | None = None,
            category: str | None = None,
            orderBy: str | None = None,
            direction: str | None = None,
            perPage: int | None = None,
            after: str | None = None,
        ) -> dict:
            """List GitHub Discussions for a repository or organisation.

            Best for: Browsing community discussions, optionally filtered by category.

            Args:
                owner: Repository owner or org.
                repo: Repository name (omit for org-level discussions).
                category: Filter by discussion category ID.
                orderBy: Order by field (requires direction).
                direction: Order direction: ASC | DESC.
                perPage: Results per page (max 20).
                after: Cursor for pagination.
            """
            return bridge.call(
                "list_discussions",
                compact(
                    owner=owner, repo=repo, category=category, orderBy=orderBy, direction=direction,
                    perPage=perPage, after=after,
                ),
            )

        def github_pr_read(
            owner: str,
            repo: str,
            pullNumber: int,
            method: str,
            page: int | None = None,
            perPage: int | None = None,
            after: str | None = None,
        ) -> dict:
            """Read details of a GitHub Pull Request: body, diff, files, commits, reviews, or comments.

            Args:
                owner: Repository owner.
                repo: Repository name.
                pullNumber: Pull request number.
                method: One of get | get_diff | get_status | get_files | get_commits |
                    get_review_comments | get_reviews | get_comments | get_check_runs.
                page: Page number (min 1).
                perPage: Results per page (max 10).
                after: Cursor for pagination (get_review_comments only).
            """
            return bridge.call(
                "pull_request_read",
                compact(owner=owner, repo=repo, pullNumber=pullNumber, method=method, page=page, perPage=perPage, after=after),
            )

        def github_list_prs(
            owner: str,
            repo: str,
            state: str | None = None,
            base: str | None = None,
            sort: str | None = None,
            direction: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """List pull requests in a GitHub repository.

            Best for: Enumerating open or merged PRs with optional state and base-branch filters.

            Args:
                owner: Repository owner.
                repo: Repository name.
                state: Filter: open | closed | all.
                base: Filter by base branch name.
                sort: Sort by: created | updated | popularity | long-running.
                direction: Sort direction: asc | desc.
                perPage: Results per page (max 10).
                page: Page number (min 1).
            """
            return bridge.call(
                "list_pull_requests",
                compact(owner=owner, repo=repo, state=state, base=base, sort=sort, direction=direction,
                        perPage=perPage, page=page),
            )

        def github_search_prs(
            query: str,
            owner: str | None = None,
            repo: str | None = None,
            sort: str | None = None,
            order: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """Search GitHub pull requests using GitHub's PR search syntax.

            Best for: Finding PRs by keyword, author, state, or label across repositories.

            Args:
                query: Search query using GitHub pull request search syntax.
                owner: Restrict to this owner (requires repo).
                repo: Restrict to this repo (requires owner).
                sort: Sort field.
                order: Sort order: asc | desc.
                perPage: Results per page (max 10).
                page: Page number (min 1).
            """
            return bridge.call(
                "search_pull_requests",
                compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
            )

        def github_get_commit(
            owner: str,
            repo: str,
            sha: str,
            detail: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """Get details of a single GitHub commit including changed files.

            Best for: Inspecting what changed in a specific commit.

            Args:
                owner: Repository owner.
                repo: Repository name.
                sha: Commit SHA, branch name, or tag name.
                detail: File detail level: none | stats (default) | full_patch.
                perPage: Results per page (max 10).
                page: Page number (min 1).
            """
            return bridge.call(
                "get_commit", compact(owner=owner, repo=repo, sha=sha, detail=detail, perPage=perPage, page=page)
            )

        def github_list_commits(
            owner: str,
            repo: str,
            sha: str | None = None,
            path: str | None = None,
            author: str | None = None,
            since: str | None = None,
            until: str | None = None,
            perPage: int | None = None,
            page: int | None = None,
        ) -> dict:
            """List commits in a GitHub repository, optionally filtered by author, path, or date.

            Best for: Reviewing recent history or changes to a specific file.

            Args:
                owner: Repository owner.
                repo: Repository name.
                sha: Branch, tag, or SHA to list commits from (defaults to default branch).
                path: Only commits touching this file path.
                author: Filter by author username or email.
                since: Only commits after this date (ISO 8601).
                until: Only commits before this date (ISO 8601).
                perPage: Results per page (max 10).
                page: Page number (min 1).
            """
            return bridge.call(
                "list_commits",
                compact(owner=owner, repo=repo, sha=sha, path=path, author=author, since=since, until=until,
                        perPage=perPage, page=page),
            )

        def github_projects_get(
            method: str,
            owner: str | None = None,
            owner_type: str | None = None,
            project_number: int | None = None,
            field_id: int | None = None,
            item_id: int | None = None,
            fields: list[str] | None = None,
            status_update_id: str | None = None,
        ) -> dict:
            """Get details of a GitHub Project or one of its fields, items, or status updates.

            Args:
                method: One of get_project | get_project_field | get_project_item |
                    get_project_status_update.
                owner: Owner (user or org login).
                owner_type: Owner type: user | org (auto-detected if omitted).
                project_number: Project number.
                field_id: Field ID (required for get_project_field).
                item_id: Item ID (required for get_project_item).
                fields: Field IDs to include in get_project_item response.
                status_update_id: Status update node ID (required for get_project_status_update).
            """
            return bridge.call(
                "projects_get",
                compact(
                    method=method, owner=owner, owner_type=owner_type, project_number=project_number,
                    field_id=field_id, item_id=item_id, fields=fields, status_update_id=status_update_id,
                ),
            )

        def github_projects_list(
            method: str,
            owner: str,
            owner_type: str | None = None,
            project_number: int | None = None,
            query: str | None = None,
            fields: list[str] | None = None,
            per_page: int | None = None,
            after: str | None = None,
            before: str | None = None,
        ) -> dict:
            """List GitHub Projects resources: projects, fields, items, or status updates.

            Args:
                method: One of list_projects | list_project_fields | list_project_items |
                    list_project_status_updates.
                owner: Owner (user or org login).
                owner_type: Owner type: user | org.
                project_number: Project number (required for fields, items, and status updates).
                query: Filter string (title/state filters for list_projects; GitHub
                    project filter syntax for list_project_items).
                fields: Field IDs to include for list_project_items.
                per_page: Results per page (max 20).
                after: Forward pagination cursor.
                before: Backward pagination cursor.
            """
            return bridge.call(
                "projects_list",
                compact(
                    method=method, owner=owner, owner_type=owner_type, project_number=project_number,
                    query=query, fields=fields, per_page=per_page, after=after, before=before,
                ),
            )

        tools: list[tuple[str, str, str, dict[str, Any], Callable[..., dict]]] = [
            (
                "github_get_file",
                "GitHub get file contents",
                "Read a file or directory listing from a GitHub repository.\n\n"
                "Best for: Fetching source code, configs, and READMEs at any ref or commit.",
                _GET_FILE_SCHEMA,
                github_get_file,
            ),
            (
                "github_get_tree",
                "GitHub get repository tree",
                "List the file tree of a GitHub repository at a given ref.\n\n"
                "Best for: Understanding project layout before reading individual files.",
                _GET_TREE_SCHEMA,
                github_get_tree,
            ),
            (
                "github_search_code",
                "GitHub search code",
                "Search GitHub code across repositories.\n\n"
                "Best for: Finding specific functions, patterns, or usages across the GitHub ecosystem.",
                _SEARCH_CODE_SCHEMA,
                github_search_code,
            ),
            (
                "github_search_commits",
                "GitHub search commits",
                "Search commit messages on GitHub.\n\n"
                "Best for: Finding commits by message keyword, author, or date across repositories.",
                _SEARCH_COMMITS_SCHEMA,
                github_search_commits,
            ),
            (
                "github_search_repos",
                "GitHub search repositories",
                "Search GitHub for repositories matching a query.\n\n"
                "Best for: Discovering projects by name, topic, language, or stars.",
                _SEARCH_REPOS_SCHEMA,
                github_search_repos,
            ),
            (
                "github_issue_read",
                "GitHub read issue",
                "Read a GitHub issue: body, comments, sub-issues, labels, or parent.\n\n"
                "method: get | get_comments | get_sub_issues | get_parent | get_labels",
                _ISSUE_READ_SCHEMA,
                github_issue_read,
            ),
            (
                "github_list_issues",
                "GitHub list issues",
                "List issues in a GitHub repository with optional filters.\n\n"
                "Best for: Enumerating open or closed issues, filtering by label or state.",
                _LIST_ISSUES_SCHEMA,
                github_list_issues,
            ),
            (
                "github_search_issues",
                "GitHub search issues",
                "Search GitHub issues using GitHub's issue search syntax.\n\n"
                "Best for: Finding issues by keyword, author, label, or state across repositories.",
                _SEARCH_ISSUES_SCHEMA,
                github_search_issues,
            ),
            (
                "github_get_discussion",
                "GitHub get discussion",
                "Get the body and metadata of a single GitHub Discussion.\n\n"
                "Best for: Reading a specific community discussion or Q&A thread.",
                _GET_DISCUSSION_SCHEMA,
                github_get_discussion,
            ),
            (
                "github_get_discussion_comments",
                "GitHub get discussion comments",
                "Get comments for a GitHub Discussion, optionally including nested replies.\n\n"
                "Best for: Reading community feedback, answers, and Q&A responses.",
                _GET_DISCUSSION_COMMENTS_SCHEMA,
                github_get_discussion_comments,
            ),
            (
                "github_list_discussions",
                "GitHub list discussions",
                "List GitHub Discussions for a repository or organisation.\n\n"
                "Best for: Browsing community discussions, optionally filtered by category.",
                _LIST_DISCUSSIONS_SCHEMA,
                github_list_discussions,
            ),
            (
                "github_pr_read",
                "GitHub read pull request",
                "Read details of a GitHub Pull Request: body, diff, files, commits, "
                "reviews, or comments.\n\n"
                "method: get | get_diff | get_status | get_files | get_commits | "
                "get_review_comments | get_reviews | get_comments | get_check_runs",
                _PR_READ_SCHEMA,
                github_pr_read,
            ),
            (
                "github_list_prs",
                "GitHub list pull requests",
                "List pull requests in a GitHub repository.\n\n"
                "Best for: Enumerating open or merged PRs with optional state and base-branch filters.",
                _LIST_PRS_SCHEMA,
                github_list_prs,
            ),
            (
                "github_search_prs",
                "GitHub search pull requests",
                "Search GitHub pull requests using GitHub's PR search syntax.\n\n"
                "Best for: Finding PRs by keyword, author, state, or label across repositories.",
                _SEARCH_PRS_SCHEMA,
                github_search_prs,
            ),
            (
                "github_get_commit",
                "GitHub get commit",
                "Get details of a single GitHub commit including changed files.\n\n"
                "Best for: Inspecting what changed in a specific commit.",
                _GET_COMMIT_SCHEMA,
                github_get_commit,
            ),
            (
                "github_list_commits",
                "GitHub list commits",
                "List commits in a GitHub repository, optionally filtered by author, path, or date.\n\n"
                "Best for: Reviewing recent history or changes to a specific file.",
                _LIST_COMMITS_SCHEMA,
                github_list_commits,
            ),
            (
                "github_projects_get",
                "GitHub get project",
                "Get details of a GitHub Project or one of its fields, items, or status updates.\n\n"
                "method: get_project | get_project_field | get_project_item | get_project_status_update",
                _PROJECTS_GET_SCHEMA,
                github_projects_get,
            ),
            (
                "github_projects_list",
                "GitHub list projects",
                "List GitHub Projects resources: projects, fields, items, or status updates.\n\n"
                "method: list_projects | list_project_fields | list_project_items | "
                "list_project_status_updates",
                _PROJECTS_LIST_SCHEMA,
                github_projects_list,
            ),
        ]

        for name, title, description, input_schema, core in tools:
            registry.register(_tool_class(name, title, description, input_schema, core))
            functions.register(core)
  reason: "Rewrite the GitHub bridge module: core functions now call the bridge and raise McpBridgeError on failure (no more .to_dict()); each is wrapped in a dynamically built ToolDefinition (via the local _tool_class factory) whose handle() is the argument transformer forwarding ctx.arguments to the core function, replacing the removed bridge.register_tool orchestration."
```
/answer 85c0-1a allow
Control Request:
```yaml
id: 85c0-1b
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp
  pattern: .*\.py$
  reason: Confirm final file set in the mcp tools directory after the refactor.
```
/answer 85c0-1b allow
Control Request:
```yaml
id: 85c0-1c
result:
  structuredContent:
    entries:
    - __init__.py
    - bridge.py
    - client.py
    - context7.py
    - exa.py
    - github.py
```
/answer 85c0-1c allow
Control Request:
```yaml
id: 85c0-1d
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  start: class GitHubBridge
  end: def register_github_tools(
  reason: Verify the GitHubBridge class and the new _tool_class factory were written correctly.
```
/answer 85c0-1d deny Ich prüfe deine Arbeit selbst wenn du fertig bist
Cannot read STDOUT: the CLI process is not started (uuid=c313c7f6-ec23-4b07-8728-ab82d65c8cf8, processAlive=false). A prompt must be sent (writeLine) before reading.