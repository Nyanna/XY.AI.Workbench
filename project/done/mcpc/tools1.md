# Tool Stubs

Entferne das Beispielset an MCP-Tool aus `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/builtin.py` und füge anstelle, ein Set neuer Tools hinzu.
Implementiere die Tools den Anforderungen entsprechend.

* Jedes Tool bekommt ein eigenes Unterpaket und wird in der Registry registriert.
* Passe die Tool-Beschreibungen und Eingabeparameter entsprechend an.

## Referenzen

* Python Package: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools`

## Tools

### Read

Liest Dateien und gibt deren Inhalt zurück.

* Parameter sind der absolute Dateiname sowie ein optionaler Min/Max-Zeilen-Offset für eine Bereichseinschränkung.
* Cached Dateien in der Session und ermittelt eine Inhalts-Hash.
* Die Session erhält hierfür eine Datenstruktur in der Tool spezifische Daten gehalten werden können.
* Fordert der Client dieselbe Datei wiederholt an ohne das sich die Hash verändert hat gibt das Tool einen entsprechenden Fehler zurück mit dem Verweis, das sich die Datei nicht geändert hat.
* Zurückgegeben wird der angeforderte Inhalt

### Write

Schreibt Dateien vollständig neu oder hängt neue Zeilen an.

* Parameter sind der absolute Dateiname, der Modus Append oder Replace und die neuen Zeilen.
* Zurückgegeben wird nur der Erfolg oder eine Fehlermeldung.

### Insert

Fügt Bereiche in existierenden Dateien ein.

* Parameter sind der absolute Dateiname, ein Zeichenoffset und der neue Inhalt.
* Zurückgegeben wird nur der Erfolg oder eine Fehlermeldung.

### Replace

Ersetzt Bereiche in existierenden Dateien.

* Parameter sind der absolute Dateiname, ein Zeichenoffset für die Einfügemarke, eine Länge für den zu ersetzenden Bereich und der neue Inhalt.
* Zurückgegeben wird nur der Erfolg oder eine Fehlermeldung.

### Bash

Erlaubt den Aufruf der Bash in einem Zielverzeichnis.

* Parameter sind das absolute Arbeitsverzeichnis und der Skriptinhalt.
* Zurückgegeben wird der Exitcode, die Ausgabe des STDOUT und falls vorhanden die Ausgabe des STDERR.
