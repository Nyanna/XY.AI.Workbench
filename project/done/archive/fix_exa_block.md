Ich er halte bei der Verwendung von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py` folgenden Fehler:

```
"'web_search_exa' failed: HTTP 403 from https://mcp.exa.ai/mcp: {\"type\":\"https://developers.cloudflare.com/support/troubleshooting/http-status-codes/cloudflare-1xxx-errors/error-1010/\",\"title\":\"Error 1010: Access denied\",\"status\":403,\"detail\":\"The site owner has blocked access based on your browser's signature.\",\"instance\":\"a23a85014b03e0d3\",\"error_code\":1010,\"error_name\":\"browser_signature_banned\",\"error_category\":\"access_denied\",\"ray_id\":\"a23a85014b03e0d3\",\"timestamp\":\"2026-07-31T06:24:20Z\",\"zone\":\"mcp.exa.ai\",\"cloudflare_error\":true,\"retryable\":false,\"own"
``` 

Prüfe die Implementierung auf fehlende Header oder ein unspezifischer User_agent und behebe dies.
Das betrifft auch "Accept-Header" und dergleichen.
SystemInit:  id=26c9004f-926c-4fbe-874f-718640fe24f8, model=/home/user/xyan/xy.ai.workbench, cwd=claude-sonnet-5
Control Request:
```yaml
id: 4fce1668-2afb-4979-adc8-de3053c32060
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: Inspect implementation for HTTP headers/user-agent
```
/answer 4fce1668-2afb-4979-adc8-de3053c32060 allow
ReasoningToken: 9
Control Request:
```yaml
id: 5f5492b6-33d2-4384-aaae-9f6d08a6821f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/client.py
  reason: Inspect McpClient HTTP request headers
```
/answer 5f5492b6-33d2-4384-aaae-9f6d08a6821f allow
ReasoningToken: 16
ReasoningToken: 138
Control Request:
```yaml
id: 60efbcc7-da6f-4c96-b238-61f885ec6dd2
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/client.py
  old_text: |-
    #: Protocol revision advertised on ``initialize`` (server may negotiate down).
    DEFAULT_PROTOCOL_VERSION = "2025-06-18"
  new_text: |-
    #: Protocol revision advertised on ``initialize`` (server may negotiate down).
    DEFAULT_PROTOCOL_VERSION = "2025-06-18"

    #: ``urllib``'s default User-Agent (``Python-urllib/x.y``) is trivially
    #: fingerprinted and blocked by Cloudflare (error 1010, "browser_signature
    #: _banned") on some remote MCP endpoints (e.g. mcp.exa.ai). Present a
    #: realistic, modern browser signature instead so requests are not rejected
    #: purely on transport fingerprinting grounds.
    DEFAULT_USER_AGENT = (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
        "(KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
    )
  reason: Add default User-Agent constant to fix Cloudflare 403
```
/answer 60efbcc7-da6f-4c96-b238-61f885ec6dd2 allow
Control Request:
```yaml
id: 252b8972-642e-4065-93bb-a54dbcafdf46
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/client.py
  old_text: |2-
        def _headers(self) -> dict[str, str]:
            headers = {
                "Content-Type": "application/json",
                "Accept": "application/json, text/event-stream",
            }
            headers.update(self._static_headers)
  new_text: |2-
        def _headers(self) -> dict[str, str]:
            headers = {
                "Content-Type": "application/json",
                "Accept": "application/json, text/event-stream",
                "Accept-Language": "en-US,en;q=0.9",
                "User-Agent": DEFAULT_USER_AGENT,
            }
            # Static (per-server) headers may override the defaults above, e.g. to
            # supply an API key or a custom User-Agent.
            headers.update(self._static_headers)
  reason: "Add Accept-Language and User-Agent defaults, letting static headers override"
```
/answer 252b8972-642e-4065-93bb-a54dbcafdf46 allow
Control Request:
```yaml
id: c8bc2700-a14d-48cb-ad01-61579f267560
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    python3 -c \"
    import ast
    ast.parse(open('src/xy/ai/mcpc/tools/mcp/client.py').read())
    print('OK: syntax valid')
    \"
  reason: Syntax-Check der geänderten Datei
```
/answer c8bc2700-a14d-48cb-ad01-61579f267560 allow
Text:
 Nun teste ich, dass die Datei weiterhin syntaktisch valide ist und der Aufbau der Header stimmt.
Control Request:
```yaml
id: db0f8a9c-7f67-43c5-9440-ae5c309cbb32
result:
  structuredContent:
    exit_code: 2
    stdout: ""
    stderr: |2
        File "<string>", line 1
          "
          ^
      SyntaxError: unterminated string literal (detected at line 1)
      bash: -c: line 3: syntax error near unexpected token `open'
      bash: -c: line 3: `ast.parse(open('src/xy/ai/mcpc/tools/mcp/client.py').read())'
  isError: true
```
/answer db0f8a9c-7f67-43c5-9440-ae5c309cbb32 allow
Control Request:
```yaml
id: a171d446-790e-420c-943b-22213daa3105
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    python3 - <<'EOF'
    import ast
    ast.parse(open('src/xy/ai/mcpc/tools/mcp/client.py').read())
    print('OK: syntax valid')
    EOF
  reason: Syntax-Check der geänderten Datei
```
/answer a171d446-790e-420c-943b-22213daa3105 allow
Control Request:
```yaml
id: b476dbcb-e800-4cd6-84e2-38ad52d8dc92
result:
  structuredContent:
    stdout: |
      OK: syntax valid
```
/answer b476dbcb-e800-4cd6-84e2-38ad52d8dc92 allow
Control Request:
```yaml
id: 3d007420-4e58-42ea-9d4d-f0ca2bb789dd
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/client.py
  reason: Finale Sichtprüfung der Header-Logik
  start: def _headers
  max_line: 125
```
/answer 3d007420-4e58-42ea-9d4d-f0ca2bb789dd allow
Control Request:
```yaml
id: 593ca61c-b8c4-40b3-86d3-d27185f35f88
result:
  structuredContent:
    checksum: c88cbe501286bb3a4d17db4c3fcd0e8bd4b0e6a4b552157a5de7c2e506db6071
    content: |
      def _headers(self) -> dict[str, str]:
              headers = {
                  "Content-Type": "application/json",
                  "Accept": "application/json, text/event-stream",
                  "Accept-Language": "en-US,en;q=0.9",
                  "User-Agent": DEFAULT_USER_AGENT,
```
/answer 593ca61c-b8c4-40b3-86d3-d27185f35f88 allow
## Ursache gefunden

Der Fehler `Error 1010: browser_signature_banned` kommt von Cloudflare vor `mcp.exa.ai` und wird ausgelöst, weil `urllib.request` standardmäßig den User-Agent `Python-urllib/3.x` sendet – ein bekanntes, oft geblocktes Bot-Signatur-Muster. Zusätzlich fehlten Header wie `Accept-Language`, die viele WAFs zur Fingerprinting-Erkennung heranziehen.

## Fix in `client.py`

1. **Neue Konstante `DEFAULT_USER_AGENT`** – ein realistischer Chrome/Windows-User-Agent-String, der statt des generischen `Python-urllib/…` gesendet wird.
2. **`_headers()` erweitert** um:
   - `"User-Agent": DEFAULT_USER_AGENT`
   - `"Accept-Language": "en-US,en;q=0.9"`
   
   Der bestehende `Accept: application/json, text/event-stream` blieb unverändert (der war schon korrekt für Streamable-HTTP-MCP).
3. Die `_static_headers` (z. B. `x-api-key` aus `exa.py`) überschreiben weiterhin bei Bedarf die Defaults – das Verhalten ist also abwärtskompatibel.

Damit sollte der Request nicht mehr als Bot-Traffic erkannt und die 403/Error-1010-Antwort vermieden werden. Ein erneuter Aufruf von `web_search_exa` sollte jetzt funktionieren. Falls Cloudflare weiterhin blockt (z. B. wegen IP-Reputation statt Signatur), müsste man zusätzlich prüfen, ob vom gleichen Host/IP kürzlich viele automatisierte Requests kamen – das lässt sich aber nicht code-seitig lösen.
