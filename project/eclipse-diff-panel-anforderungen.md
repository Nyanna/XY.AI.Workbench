# Anforderungen: Kompaktes Diff-Panel für Eclipse

## Kontext

Es wird bereits ein sprachagnostisches AST-Matching auf zwei Versionen eines
Codeartefakts durchgeführt. Für Paare geänderter AST-Knoten (meist kurze Inhalte:
einzelne Ausdrücke, wenige Tokens bis wenige Zeilen) soll eine visuelle Diff-Anzeige
innerhalb eines beliebigen Eclipse Tool-Panels (`ViewPart`) erfolgen.

## Ziel

Kompakte, read-only Diff-Darstellung mit Wort-/Teilausdrucks-Genauigkeit,
vergleichbar mit der visuellen Qualität von IntelliJs Inline-Diff-Highlighting –
**ohne** Refactoring- oder Verschiebungserkennung, **ohne** Side-by-Side-Layout.

## Funktionale Anforderungen

1. **Eingabe:** zwei beliebige Textinhalte (Content zweier AST-Knoten), meist kurz
   (einzelne Zeile bis wenige Zeilen).
2. **Diff-Berechnung zweistufig:**
   - Stufe 1: Diff auf Token-Ebene (Tokenisierung über bestehenden
     Lexer/Parser, keine Neuimplementierung).
   - Stufe 2: bei geänderten Token-Blöcken (`CHANGE`) mit geringer Länge
     zusätzlich Diff auf Zeichenebene, um Teiländerungen innerhalb eines Tokens
     sichtbar zu machen (z. B. `getUserName` → `getUserNames`).
   - Rekursionstiefe fix auf zwei Ebenen (Token, Zeichen) begrenzt.
3. **Diff-Engine:** `org.eclipse.compare.rangedifferencer.RangeDifferencer`,
   generisch über eigene `IRangeComparator`-Implementierungen für Token- und
   Zeichenlisten. Keine zusätzliche Diff-Bibliothek (kein `java-diff-utils`,
   kein JGit) einführen.
4. **Darstellung:**
   - Ein einzelnes `StyledText`-Widget (SWT), kein `SourceViewer`, kein
     `IDocument`, kein `TextMergeViewer`, kein Side-by-Side.
   - Read-only (`SWT.READ_ONLY`), kein Editieren, kein Undo/Dirty-State.
   - Kein Reconciler, keine Annotationen, keine Vertical-Ruler-Marker.
   - Keine sprachspezifische Syntaxhervorhebung (kein JDT-Highlighting).
   - Verschachtelte Hervorhebung: äußerer Hintergrund für geänderte Token-Blöcke,
     zusätzlicher, satterer innerer Hintergrund für die exakte Zeichenänderung
     innerhalb dieses Blocks.
5. **Panel-Integration:** Anzeige erfolgt in einem beliebigen `ViewPart`
   (nicht an einen Editor-Lifecycle gebunden), reine Anzeigekomponente,
   wiederverwendbar für wechselnde Diff-Inhalte (`setText()` +
   `setStyleRanges()` bei jedem neuen Vergleich).

## Nicht-Ziele (explizit ausgeschlossen)

- Keine Refactoring- oder Verschiebungserkennung (kein GumTree, keine
  Tree-Edit-Distance).
- Kein 3-Wege-Merge, keine Merge-Funktionalität.
- Kein Side-by-Side- oder Multi-Pane-Layout.
- Keine Undo/Redo- oder Editierfunktion.

## Technische Vorgaben

- Zielplattform: Eclipse RCP/SWT, Java.
- Farbverwaltung über JFace `ColorRegistry`, kein manuelles `Color`-Handling
  ohne Dispose-Absicherung.
- `StyleRange`-Erzeugung muss sortierte, nicht überlappende Ranges liefern
  (ggf. Segmentierung überlappender Bereiche) oder `replaceStyleRanges`
  verwenden.
- Bestehende Tokenisierung (aus dem vorhandenen AST-Parser/Lexer) ist
  wiederzuverwenden, keine separate Tokenisierung für die Diff-Anzeige
  implementieren.

## Akzeptanzkriterien

- Zwei kurze Textinhalte mit einer Teiländerung (z. B. geänderter
  Bezeichner, geänderter Literalwert) werden so dargestellt, dass sowohl der
  betroffene Token als auch die exakte geänderte Teilzeichenkette darin optisch
  unterscheidbar hervorgehoben sind.
- Unveränderte Bereiche bleiben ohne Hervorhebung.
- Reine Einfügungen/Löschungen (kein `REPLACE`) werden ohne unnötigen
  Zeichen-Diff-Durchlauf dargestellt.
- Die View funktioniert unabhängig von der Herkunft der AST-Knoten
  (sprachagnostisch, keine Java-spezifische Anzeige-Logik).
