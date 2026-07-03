Ja, das geht – über zwei Mechanismen in `StyleRange` bzw. `TextStyle`:

---

## 1. `rise` – Vertikaler Offset (Baseline Shift)

Das `rise`-Property verschiebt einzelne Zeichen vertikal relativ zur Grundlinie:

- **Positiver Wert** → Text nach **oben** (Hochstellung / Superscript)
- **Negativer Wert** → Text nach **unten** (Tiefstellung / Subscript)

```java
StyleRange superscript = new StyleRange();
superscript.start = 5;
superscript.length = 2;
superscript.rise = font.getFontData()[0].getHeight() / 2; // z.B. halbe Zeichenhöhe nach oben
styledText.setStyleRange(superscript);

StyleRange subscript = new StyleRange();
subscript.start = 10;
subscript.length = 2;
subscript.rise = -(font.getFontData()[0].getHeight() / 2); // nach unten
styledText.setStyleRange(subscript);
```

---

## 2. `GlyphMetrics` – Zeichenbreite & Ausrichtung

`GlyphMetrics` erlaubt, einem einzelnen Zeichen **benutzerdefinierte Abmessungen** zu geben (Höhe über Grundlinie, Tiefe, Breite). Das wird vor allem genutzt, um **Objekte (z. B. Bilder)** in den Textfluss einzubetten:

```java
StyleRange style = new StyleRange();
style.start = offset;
style.length = 1;
style.metrics = new GlyphMetrics(
    imageHeight,  // ascent (über Grundlinie)
    0,            // descent (unter Grundlinie)
    imageWidth    // Breite des "Zeichens"
);
styledText.setStyleRange(style);

// Beim Zeichnen das Bild an der korrekten Position platzieren:
styledText.addPaintObjectListener(event -> {
    Image img = (Image) event.style.data;
    int x = event.x;
    int y = event.y + event.ascent - event.style.metrics.ascent;
    event.gc.drawImage(img, x, y);
});
```

---

## Auf Zeilenebene: Einrückung & Abstände

| API                                    | Beschreibung                                                |
| -------------------------------------- | ----------------------------------------------------------- |
| `setLineVerticalIndent(line, px)`      | Vertikalen Abstand **vor** einer Zeile setzen               |
| `setLineAlignment(line, count, align)` | Horizontale Ausrichtung pro Zeile (`SWT.LEFT/CENTER/RIGHT`) |
| `TextLayout.setSpacing(px)`            | Zeilenabstand                                               |
| `TextLayout.setIndent(px)`             | Einzug der ersten Zeile                                     |

---

## Einschränkungen

- **Kein horizontaler Offset pro Zeichen** – `rise` ist rein vertikal; horizontale Verschiebung einzelner Zeichen ist nicht vorgesehen.
- Kein Drehen oder Schrägstellen von Zeichen.
- Horizontale Ausrichtung funktioniert immer nur auf Zeilenebene.