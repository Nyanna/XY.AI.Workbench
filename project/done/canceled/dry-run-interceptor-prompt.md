# Implementierungsprompt: Dry-Run Interceptor für AST-Edit-Tool (MCP-Server)

## Kontext

Der bestehende MCP-Server implementiert generische, textbasierte Edit-Tools auf einer
AST-Engine (Tree-sitter). Die Engine läuft pro Request. Edit-Operationen arbeiten auf
Text-Ebene (Byte-Ranges); die Umwandlung in AST-Strukturen übernimmt die Engine selbst.

Ziel: Dry-Run-Fähigkeit mit Diff-Preview, ohne Aspekt-Code in den Tool-Schichten.

## Prinzip

Ein **Interceptor** wird als Layer zwischen Tool-Aufrufer und Engine/Storage gesetzt.
Er ist als **Read-Through LIFO-Cache, der Writes blockiert** implementiert:

- **Aufzeichnung (Log):** Jeder Read und jeder Write eines AST-Knotens/Textbereichs
  wird mit ID protokolliert. Die Aufzeichnung ist append-only – mehrfaches Erfassen
  derselben ID ist zulässig und überschreibt nichts im Log.
- **LIFO-Zugriff:** Bei Abfrage einer ID liefert der Interceptor den zuletzt
  aufgezeichneten Stand dieser ID.
- **Read-Through:** Ein Read wird zunächst gegen den Interceptor-Log aufgelöst.
  Existiert ein Eintrag für die angefragte ID, wird dieser zurückgegeben (nicht der
  reale Speicherstand). Existiert keiner, wird an die echte Quelle durchgereicht und
  das Ergebnis für künftige Reads im Log erfasst.
- **Write-Block:** Schreiboperationen werden im Log erfasst, aber nicht an die reale
  Engine/Storage persistiert. Der reale Zustand bleibt unverändert.
- **Sequenz statt Diff-Log:** Der Interceptor zeichnet Zustände (ausgehender/
  eingehender Text pro ID) auf, nicht die Edit-Operation selbst. Die Änderung ergibt
  sich aus der Sequenz der Einträge pro ID (erstes Vorkommen = Baseline, letztes
  Vorkommen = aktueller simulierter Stand).
- Der Interceptor speichert den Typ Read/Write/Replace/Delete usw.

## Abgrenzung / Zuständigkeit

- Der Interceptor liefert ausschließlich die aufgezeichnete Sequenz (ID → Liste von
  Textständen in Aufzeichnungsreihenfolge).
- **Diff-Erzeugung und -Darstellung sind nicht Teil des Interceptors.** Der Aufrufer
  liest den Log nach Abschluss der simulierten Operation(en) aus und erzeugt daraus
  den Diff (z. B. Baseline vs. letzter Stand pro ID, oder vollständiger Text-Diff).
- Operationsabhängigkeiten zwischen mehreren simulierten Schritten werden nicht
  gesondert behandelt – sie ergeben sich implizit korrekt, sofern Read-Through wie
  oben beschrieben funktioniert.
- ID-Stabilität über Reparses hinweg wird nicht erzwungen. Ist eine ID nach einer
  simulierten Änderung nicht mehr zuordenbar, gilt der zugehörige Knoten als neu.

## Implementierungsaufgabe

1. Interceptor als Layer implementieren, der sich zwischen die generischen Edit-Tools
   und die Engine/Storage schaltet (kein Aspekt-Code in den Tools selbst).
2. Log-Struktur: ID → chronologische Liste erfasster Textstände (kein Overwrite,
   kein Pop – reines Anhängen).
3. Read-Pfad: Lookup im Log (LIFO: letzter Eintrag zur ID) vor Zugriff auf reale
   Quelle; bei Treffer keinen Durchgriff auf reale Engine.
4. Write-Pfad: Eintrag im Log erzeugen, realen Persistierungsaufruf unterdrücken.
5. Auslese-Schnittstelle für den Aufrufer: vollständiger Log (ID, Sequenz der
   Textstände) nach Abschluss der Simulation abrufbar machen.
6. Sicherstellen, dass die Engine-Instanz pro Request isoliert ist (bereits gegeben),
   sodass kein Zustand zwischen Dry-Run und nachfolgenden echten Requests leakt.
