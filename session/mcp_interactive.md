Die Research-Agents haben leider halluzinierten Content zurückgegeben – die Changelog-Einträge und Dokumentation sind erfunden, nicht echt. Ich gebe dir eine ehrliche Einschätzung auf Basis meines tatsächlichen Trainingswissens:

---

## Was ich mit Sicherheit weiß

**Claude Desktop** – unterstützt MCP Sampling (`sampling/createMessage`) ✓

**Claude Code** – unklar. Die MCP-Clients-Seite auf modelcontextprotocol.io listet welche Clients welche Capabilities unterstützen (Tools, Resources, Prompts, Sampling, Roots). Claude Code wird dort als Client geführt, aber ob Sampling konkret implementiert ist, kann ich nicht mit Sicherheit bestätigen.

## Wie du es direkt prüfst

Wenn du eine aktive Claude Code Installation hast:

```bash
# Claude Code gibt beim MCP-Handshake seine Client-Capabilities bekannt
# In deinem MCP-Server kannst du diese im initialize-Handler lesen:
# params.capabilities.sampling → existiert das Feld?
```

Im MCP-Handshake sendet der Client seine Capabilities:
```json
{
  "method": "initialize",
  "params": {
    "capabilities": {
      "sampling": {}   ← wenn das da ist, wird Sampling unterstützt
    }
  }
}
```

Ein einfacher Test-MCP-Server, der die empfangenen Capabilities loggt, würde das sofort klären.

## Direkte Quelle

→ https://modelcontextprotocol.io/clients – dort steht in der Tabelle für jeden Client welche Capabilities er unterstützt. Das ist die verlässlichste Quelle, da sie vom MCP-Team gepflegt wird.

Willst du, dass ich dir einen minimalen MCP-Server schreibe, der die Handshake-Capabilities loggt, damit du es direkt testen kannst?