Optimiere die Komponentenliste (YAML): deduplizieren und zusammenführen, wo sinnvoll. Die Ausgabe wird von einem Agenten weiterverarbeitet, daher auf Salienz und Kompaktheit optimieren, und zwar nur durch Wegfall von Redundanz und Füllwörtern, nie durch Wegfall von Aussagen.

Maßstab für Formulierung und Zusammenführung (nicht zum Verschieben von Anforderungen):
- Jedes Modul ist unabhängig entwickelbar, auf Basis definierter Schnittstellen und Verantwortlichkeiten.
- Eine Anforderung gehört in das Modul, das sie ohne Kenntnis fremder Interna umsetzen kann.

Regeln:
- Anforderungen bleiben in ihrer Komponente. Dedupliziert und zusammengeführt wird nur innerhalb einer Komponente, nie komponentenübergreifend.
- Keine erfundenen Aussagen, Verantwortlichkeiten oder Begriffe.
- Zusammenführen oder Umformulieren ist erlaubt, wenn Redundanz entfällt und jede ursprüngliche Aussage erhalten bleibt.
- Verschiedene Verantwortungen nicht in einem Punkt mischen (z. B. Lebenszyklus, Konfigurationsvalidierung, Ressourcenverwaltung).
- Komponentennamen, Reihenfolge und YAML-Struktur bleiben unverändert.

Ausgabe: nur die optimierte YAML-Liste.

Anforderungen:
```yaml
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/split_merged/05.yml)
```
