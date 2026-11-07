Optimiere den Modulbaum (YAML) durch Umformen und Mergen.

## Operationen

Mergen: Fasse die Responsibility-Listen von Modulen zusammen, die zu einem Modul verschmolzen werden. Entferne Duplikate und formuliere die Einträge zusammenhängend.
Umformen: Hänge Module oder Teilbäume um, ohne Responsibilities inhaltlich zu verändern.

## Ziele

Erhöhe die Tiefe und reduziere die Breite. Jedes Modul soll möglichst wenige direkte Kinder haben.
Lege Module für GPU- und CPU-Aspekte in jeweils eigene Teilbäume, damit sie besser isoliert sind. Ausgenommen sind geteilte Module, die beide Seiten nutzen. Sie bleiben auf gemeinsamer Ebene.

## Randbedingungen

Erhalte alle Responsibilities vollständig. Nichts darf entfallen oder neu hinzukommen.
Behalte das YAML-Schema bei (Name, Responsibility, Children).
Gib nur den optimierten Modulbaum als YAML aus, ohne Erläuterungen.

## Eingabe
