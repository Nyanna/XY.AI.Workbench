Kann ich in einem Eclipse Plugin in einem `SourceViewer` oder `TextEditor` die Anzeige der Zeilennummer ausblenden bei Überschreiten einer bestimmten Dateigröße?
#:SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=2044c85a-bace-4591-bb8a-a0b85d11c91f | model=claude-sonnet-5 | plugins=
# Zeilennummern bei großen Dateien ausblenden

Ja, das ist möglich. Es gibt mehrere Ansätze, je nachdem ob du `SourceViewer` direkt verwendest oder den Standard-`TextEditor` erweiterst.

## 1. Bei `SourceViewer` direkt

Der `SourceViewer` selbst zeigt keine Zeilennummern an – das übernimmt normalerweise ein `VerticalRuler` mit einer `LineNumberRulerColumn`. Du kannst diese Column einfach nicht hinzufügen bzw. wieder entfernen, wenn die Datei zu groß ist:

```java
public class MySourceViewer extends SourceViewer {

    private CompositeRuler fRuler;
    private LineNumberRulerColumn fLineNumberColumn;
    private static final long MAX_SIZE_FOR_LINE_NUMBERS = 500_000; // Bytes

    public MySourceViewer(Composite parent, IVerticalRuler ruler,
            int styles, IDocument document) {
        super(parent, ruler, null, false, styles);
        setDocument(document);

        if (ruler instanceof CompositeRuler) {
            fRuler = (CompositeRuler) ruler;
        }

        updateLineNumbers(document);
    }

    private void updateLineNumbers(IDocument document) {
        if (fRuler == null) {
            return;
        }

        long size = document.getLength();

        if (size > MAX_SIZE_FOR_LINE_NUMBERS) {
            if (fLineNumberColumn != null) {
                fRuler.removeDecorator(fLineNumberColumn);
                fLineNumberColumn = null;
            }
        } else {
            if (fLineNumberColumn == null) {
                fLineNumberColumn = new LineNumberRulerColumn();
                fRuler.addDecorator(0, fLineNumberColumn);
            }
        }
    }
}
```

## 2. Bei `AbstractTextEditor` / `TextEditor` (Standard-Editor erweitern)

Wenn du von `TextEditor` erbst, gibt es die Methode `createVerticalRuler()` bzw. `initializeLineNumberRulerColumn()`. Der übliche Trick ist, `isLineNumberRulerVisible()` zu überschreiben und den Ruler bei Bedarf neu zu erzwingen:

```java
public class MyTextEditor extends TextEditor {

    private static final long MAX_SIZE = 500_000;

    @Override
    protected void initializeEditor() {
        super.initializeEditor();
    }

    @Override
    public void doSetInput(IEditorInput input) throws CoreException {
        super.doSetInput(input);
        updateLineNumberVisibility(input);
    }

    private void updateLineNumberVisibility(IEditorInput input) {
        long size = getFileSize(input);
        boolean showLineNumbers = size <= MAX_SIZE;

        IPreferenceStore store = getPreferenceStore();
        // Diese Konstante steuert die Anzeige der Line Number Ruler Column
        store.setValue(
            AbstractDecoratedTextEditorPreferenceConstants.EDITOR_LINE_NUMBER_RULER,
            showLineNumbers);
    }

    private long getFileSize(IEditorInput input) {
        if (input instanceof IFileEditorInput) {
            IFile file = ((IFileEditorInput) input).getFile();
            IPath location = file.getLocation();
            if (location != null) {
                return location.toFile().length();
            }
        }
        return 0;
    }
}
```

⚠️ **Wichtig:** Das Setzen von `store.setValue(...)` auf `EDITOR_LINE_NUMBER_RULER` ändert i. d. R. die **globale** Preference (auch für andere offene Editoren), da der `PreferenceStore` meist geteilt wird. Willst du das nur pro Editor-Instanz steuern, solltest du stattdessen den `IVerticalRulerColumn` direkt manipulieren:

### Sauberere Variante: eigenen Composite-Ruler verwenden

Überschreibe `createCompositeRuler()` in deinem `TextEditor`:

```java
@Override
protected CompositeRuler createCompositeRuler() {
    CompositeRuler ruler = new CompositeRuler();
    ruler.addDecorator(0, createAnnotationRulerColumn(ruler));

    if (shouldShowLineNumbers()) {
        LineNumberRulerColumn lineNumberColumn = new LineNumberRulerColumn();
        initializeLineNumberRulerColumn(lineNumberColumn);
        ruler.addDecorator(1, lineNumberColumn);
    }

    return ruler;
}

private boolean shouldShowLineNumbers() {
    IEditorInput input = getEditorInput();
    long size = getFileSize(input);
    return size <= MAX_SIZE;
}
```

`shouldShowLineNumbers()` musst du natürlich vor dem Aufruf von `createCompositeRuler()` bestimmen können (z. B. Input bereits über den Konstruktor oder ein zuvor gesetztes Feld verfügbar machen).

## 3. Dynamisch zur Laufzeit umschalten (z.B. wenn Datei größer wird)

Falls sich die Dateigröße während der Bearbeitung ändern kann (Nutzer tippt viel Text), horche auf `IDocumentListener`:

```java
getSourceViewer().getDocument().addDocumentListener(new IDocumentListener() {
    @Override
    public void documentChanged(DocumentEvent event) {
        updateLineNumberVisibility();
    }

    @Override
    public void documentAboutToBeChanged(DocumentEvent event) {}
});
```

und rufe darin `fRuler.addDecorator(...)`/`removeDecorator(...)` wie in Beispiel 1 auf – ggf. mit `Display.asyncExec` verpacken, um UI-Thread-Konflikte zu vermeiden.

## Zusammenfassung

| Ansatz | Wann verwenden |
|---|---|
| `SourceViewer` mit eigenem `CompositeRuler` | Volle Kontrolle, eigenständige Viewer-Implementierung |
| `TextEditor.createCompositeRuler()` überschreiben | Sauberste Lösung bei Editor-Erweiterung, keine globalen Preferences |
| `PreferenceStore.setValue(EDITOR_LINE_NUMBER_RULER, ...)` | Schnelle Lösung, aber Vorsicht: globaler Effekt möglich |

Am saubersten und am wenigsten fehleranfällig ist die Variante mit `createCompositeRuler()` in Kombination mit einer Größenprüfung beim Öffnen der Datei – ergänzt um einen `IDocumentListener`, falls sich die Größe zur Laufzeit ändern kann.