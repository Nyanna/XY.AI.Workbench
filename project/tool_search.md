Ich möchte die Architektur der Toolnutzung ändern. Implementiere Folgendes.
Tools werden in Zukunft generell als Python Functions bei Bedarf bereitgestellt.
Der alte klassische MCP Mechanismus über die Registry soll erhalten belieben.

## tool_search

Ein neues Tool "tool_search" in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools`.
- Als Eingabe eine Space separierte liste von Keywords (nur englisch), die mit dem Tools (zuerst Function Name, dann Docstring) abgeglichen wird.
- Als Ausgabe eine Liste von Functions; Name und Docstring (keine Signatur, nur auf die erste Zeile gekürzter Docstring); Alphabetisch sortiert.
- Ein Result wird in der Session geflaggt; Pro Session wird jede Function nur einmal als Ergebnis ausgegeben; Im Worst Case hat der Agent also einmal jede Toolbeschreibung erhalten.

## tool_schema

Ein neues Tool "tool_schema". Dieses gibt Nutzung und Schema-Informationen zu einem Tool aus.
- Erhält als Eingabe einen Function Name
- Liefert die Signatur und den vollen Docstring zurück, sowie eine Liste des Quelltexts aller nicht primitiven Typen und Objekte, die vom Tool selbst deklariert werden und nicht Teil der Standardbibliothek sind. Auch enthalten verschachtelte selbst deklarierte Objekte;  Der Agent soll imstande sein die Schnittstelle vollständig und Typsicher zu nutzen.