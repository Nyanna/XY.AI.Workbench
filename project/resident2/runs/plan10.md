Aufgabe: Leite aus dem Material ein hierarchisches Komponentenmodell ab, das alle darin enthaltenen Anforderungen aufnehmen kann.

- Eine Komponente ist ein Verantwortungsbereich, kein Datentyp. Benenne nach dem Zweck, nicht nach im Text genannten Objekten. Keine Typ- oder Musternamen.
- Name: Nominalphrase der Fachsprache, eindeutig, in der Sprache des Materials.
- Responsibility: nur aus dem Material ableiten.
- Jede Anforderung muss sich mindestens einer Komponente zuordnen lassen. Lege Komponenten an, bis das gilt, und fasse zusammen, was dieselbe Verantwortung hat.
- Hierarchie: ein Baum mit einer Wurzel, jede Komponente hat genau einen Elternknoten. Untergliedere nur, wenn eine Komponente mehrere trennbare Verantwortungen hat.

Ausgabe: genau ein YAML-Block, kein Text davor oder danach, keine YAML-Kommentare.
Struktur: Die Ausgabe ist eine Liste mit genau einem Eintrag, der Wurzelkomponente. Jede Komponente hat genau die Felder Name, Responsibility, Children.
- Name: Zeichenkette.
- Responsibility: Liste von Verbphrasen (1-2), kompakt.
- Children: Liste von Komponenten derselben Struktur (wenn vorhanden).

Form:
```yaml
- Name: Wurzel
  Responsibility:
    - ...
  Interfaces:
    - ...
  Children:
    - Name: Kind A
      Responsibility:
        - ...
      Interfaces:
        - ...
      Children: []
    - Name: Kind B
      Responsibility:
        - ...
      Interfaces:
        - ...
      Children: []
```

Material:
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/plan7.md)

Result Stats: id=bb745bd2-41d6-4ff9-8efd-c877d034612b, total: 137075, in: 65729, out: 71346, reason: 47822, read: 0, write: 0, start: 1793390317603, end: 1793390744885, time: 426530, total: 426530, io: 323, control: 752, sent: 1, recv: 3763
SystemInit:  id=bb745bd2-41d6-4ff9-8efd-c877d034612b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 47822
