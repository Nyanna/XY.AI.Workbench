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



Result Stats: id=1906c37f-76fe-475f-b7f2-00a46a1e1327, total: 48515, in: 17226, out: 31289, reason: 14940, read: 0, write: 0, start: 1793395763345, end: 1793395929233, time: 165739, total: 165739, io: 19, control: 149, sent: 1, recv: 2257
SystemInit:  id=1906c37f-76fe-475f-b7f2-00a46a1e1327, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 14940

