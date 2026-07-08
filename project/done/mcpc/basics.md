# MCP Controller

Erstelle auf Basis von Python, das Grundgerüst eines MCP-Servers ohne die Verwendung eines SDK.

* Der Server soll kompatibel zum aktuellen MCP-Standard sein
* Der Server bietet streamable HTTP Endpunkte mit JSON-RPC 2.0
* Für JSON-RPC 2.0 kann eine Bibliothek oder Standard-Package verwendet werden.

## Referenzen

* Projektverzeichnis: `/home/user/xyan/xy.ai.workbench/mcpc`
* Initiale MCP-Endpunkt: `http://localhost:9093/mpc`

## Architektur

* Tools werden zentral in einer Registry verwaltet aber Client spezifisch konfiguriert und zur Verfügung gestellt
	* Die Registry wird dafür mit dem Sessionkontext abgeglichen
* Der Primäre Key für alle Operation ist die vom Client über den Header "X-MCPC-SESSION-ID" immer übermittelte Session-ID (UUID).
* Alle Kommunikation, typischerweise in JSON, wird zeilenweise in einem zentralen Logverzeichnis unter dem Dateinamen der Session-ID gespeichert.
* Eine Serverseitige im Speicher gehaltene Session persistiert Zustände und Konfiguration für eine Session-ID
* Der Server ist Statefull
* Elicitation wird nicht unterstützt
* Notifications werden nicht unterstützt