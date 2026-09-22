# Eclipse Diff-Panel: Implementierungsdetails

## Architekturüberblick

```
AST-Knoten A, B
      │
      ▼
Tokenisierung (bestehender Lexer/Parser)
      │
      ▼
Stufe 1: Token-Diff (RangeDifferencer)
      │
      ├─ CHANGE-Bereich kurz? ──► Stufe 2: Zeichen-Diff (RangeDifferencer)
      │
      ▼
StyleRange-Erzeugung (verschachtelt: Token-Level + Char-Level)
      │
      ▼
StyledText in ViewPart (read-only)
```

Kein `TextMergeViewer`, kein `SourceViewer`, kein `IDocument` – reines `StyledText`
in einem `ViewPart`. Kein Reconciler, keine Annotationen, keine Sprachunterstützung.

---

## 1. IRangeComparator-Implementierungen

```java
class TokenComparator implements IRangeComparator {
    private final List<Token> tokens;

    TokenComparator(List<Token> tokens) {
        this.tokens = tokens;
    }

    @Override
    public int getRangeCount() {
        return tokens.size();
    }

    @Override
    public boolean rangesEqual(int thisIndex, IRangeComparator other, int otherIndex) {
        return tokens.get(thisIndex).equals(((TokenComparator) other).tokens.get(otherIndex));
    }

    @Override
    public boolean skipRangeComparison(int length, int maxLength, IRangeComparator other) {
        return false; // bei kleinen AST-Knoten irrelevant, kein Performance-Skip nötig
    }

    Token get(int index) {
        return tokens.get(index);
    }
}
```

```java
class CharComparator implements IRangeComparator {
    private final char[] chars;

    CharComparator(String text) {
        this.chars = text.toCharArray();
    }

    @Override
    public int getRangeCount() {
        return chars.length;
    }

    @Override
    public boolean rangesEqual(int thisIndex, IRangeComparator other, int otherIndex) {
        return chars[thisIndex] == ((CharComparator) other).chars[otherIndex];
    }

    @Override
    public boolean skipRangeComparison(int length, int maxLength, IRangeComparator other) {
        return false;
    }
}
```

---

## 2. Stufe 1: Token-Diff

```java
RangeDifference[] tokenDiffs = RangeDifferencer.findDifferences(
    new TokenComparator(tokensA),
    new TokenComparator(tokensB)
);
```

`RangeDifference` liefert pro Änderungsblock:
- `kind` (`RangeDifference.NOCHANGE`, `CHANGE`, `LEFT`, `RIGHT`)
- `leftStart` / `leftLength`
- `rightStart` / `rightLength`

Die eigentlichen Token-Objekte werden über die Indizes aus `tokensA`/`tokensB` nachgeschlagen
(im Gegensatz zu `java-diff-utils`, das die Elemente direkt im Delta mitführt).

---

## 3. Stufe 2: Zeichen-Diff

```java
static final int SHORT_TOKEN_THRESHOLD = 3;

boolean isShortChange(RangeDifference diff) {
    return diff.kind() == RangeDifference.CHANGE
        && diff.leftLength() <= SHORT_TOKEN_THRESHOLD
        && diff.rightLength() <= SHORT_TOKEN_THRESHOLD;
}

for (RangeDifference diff : tokenDiffs) {
    if (isShortChange(diff)) {
        String oldText = joinTokens(tokensA, diff.leftStart(), diff.leftLength());
        String newText = joinTokens(tokensB, diff.rightStart(), diff.rightLength());

        RangeDifference[] charDiffs = RangeDifferencer.findDifferences(
            new CharComparator(oldText),
            new CharComparator(newText)
        );
        // charDiffs -> feinere StyleRanges innerhalb dieses Token-Blocks
    }
}
```

Rekursionstiefe bewusst auf zwei Ebenen begrenzt (Token → Zeichen). Eine dritte Ebene
bringt bei kleinen AST-Knoteninhalten keinen Mehrwert.

---

## 4. StyledText-Rendering (ViewPart, read-only)

```java
public class DiffPanelView extends ViewPart {

    private StyledText styledText;

    @Override
    public void createPartControl(Composite parent) {
        styledText = new StyledText(parent, SWT.V_SCROLL | SWT.H_SCROLL | SWT.READ_ONLY);
        styledText.setFont(JFaceResources.getTextFont());
        renderDiff();
    }

    private void renderDiff() {
        String content = buildDisplayText(); // z.B. beide Seiten kompakt interleaved
        styledText.setText(content);
        styledText.setStyleRanges(buildStyleRanges());
    }

    @Override
    public void setFocus() {
        styledText.setFocus();
    }

    @Override
    public void dispose() {
        // eigene Color-Objekte hier disposen, falls nicht über JFaceResources/ColorRegistry verwaltet
        super.dispose();
    }
}
```

### Verschachtelte StyleRanges (Token-Hintergrund + Zeichen-Hintergrund)

```java
List<StyleRange> ranges = new ArrayList<>();

// äußerer Hintergrund: gesamter geänderter Token-Block
StyleRange tokenBg = new StyleRange();
tokenBg.start = tokenBlockOffset;
tokenBg.length = tokenBlockLength;
tokenBg.background = colorRegistry.get("diff.token.changed");
ranges.add(tokenBg);

// innerer, satterer Hintergrund: exakte Zeichenänderung innerhalb des Blocks
StyleRange charBg = new StyleRange();
charBg.start = tokenBlockOffset + charDiffOffset;
charBg.length = charDiffLength;
charBg.background = colorRegistry.get("diff.char.changed");
ranges.add(charBg);
```

**Wichtig:** `setStyleRanges(StyleRange[])` verlangt sortierte, nicht überlappende Ranges.
Für echte Verschachtelung (äußerer + innerer Range am selben Offset-Bereich) den Bereich
vorab in nicht überlappende Segmente auflösen:

```
[unverändert] [char-diff-Segment] [unverändert]
```

Alternativ `replaceStyleRanges(int start, int length, StyleRange[] ranges)` verwenden,
das inkrementelles Setzen ohne globale Sortierpflicht über den gesamten Text erlaubt.

### Farbverwaltung

- Eigene `Color`-Objekte immer über `ColorRegistry` (JFace) verwalten lassen –
  kein manuelles `dispose()` in der View nötig.
- Falls direkt `new Color(display, r, g, b)`: zwingend in `ViewPart.dispose()` freigeben.

---

## 5. Zusammenfassung der Bibliothekswahl

| Ebene | Werkzeug | Algorithmus |
|---|---|---|
| Token-Diff | `RangeDifferencer` (`org.eclipse.compare.rangedifferencer`) | LCS-Variante mit Präfix/Suffix-Optimierung |
| Zeichen-Diff | `RangeDifferencer` (gleiche API, `CharComparator`) | dieselbe Engine |
| Rendering | `StyledText` (SWT) | – |
| Panel-Host | `ViewPart` | – |

**Keine Zusatzdependency** (kein `java-diff-utils`, kein JGit) nötig, da `RangeDifferencer`
bereits Teil des Eclipse Compare Frameworks ist und über `IRangeComparator` generisch auf
beliebigen Elementtypen (Zeilen, Tokens, Zeichen) arbeitet.

**Bewusst nicht verwendet:** GumTree (Tree-Edit-Distance, Refactoring-Erkennung) –
nicht erforderlich, da AST-Matching bereits sprachagnostisch vorgelagert erfolgt und
nur die visuelle Wort-/Teilausdrucks-Diff-Qualität gefordert ist.
