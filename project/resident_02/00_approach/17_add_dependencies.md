Ergänze pro Komponente (aus der Liste)  `Dependencies`: Namen der Knoten, die fertig sein müssen, bevor diese Komponente umgesetzt werden kann.

Regeln:
- Nur Namen aus dem Komponentenbaum, exakt geschrieben.
- Aufnehmen, wenn die Komponente den Vertrag des Ziels (Typen, Schnittstellen, Verhalten) benötigt, um ihr Design festzulegen und umzusetzen. Das gilt auch, wenn das Ziel ein Bestandteil ist, aus dem die Komponente aufgebaut ist.
- Nicht aufnehmen:
  - Das Wort kommt nur als Wert, Parametername oder Bezeichnung vor.
  - Es fehlt nur eine kleine Ergänzung am Ziel (Payload-Feld, Enum-Wert, zusätzlicher Parameter). Solche Erweiterungen darf das spätere Paket im früheren Modul nachziehen, solange sie Vertrag und Design des Ziels nicht ändern.
  - Bloßes Zusammenspiel zur Laufzeit ohne Umsetzungsvoraussetzung.
- Provider ist, wer den Vertrag definiert. Wer ihn nur nutzt oder erfüllt, hängt vom Definierenden ab, nie umgekehrt.
- Immer den tiefsten Knoten nennen, den die Komponente tatsächlich voraussetzt. Nur wenn das Ziel als Ganzes gemeint ist, den übergeordneten Knoten.
- Eigene Unterknoten und Elternknoten sind keine Dependencies.
- Prüfe vor jeder Dependency: Welchen Vertrag liefert das Ziel, der das Design dieser Komponente bestimmt? Nicht benennbar: weglassen.
- Bei Unsicherheit über Richtung oder Tiefe einer echten Kante: aufnehmen. Eine fehlende Dependency ist ein Fehler, eine überzählige nicht.


Ausgabe: nur YAML, Komponente unverändert plus:
  Dependencies: [<Name>, ...]

## Komponentenbaum (nur Orientierung)
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/modules_optimized.yml)

## Komponentenliste
```yaml
[include](/home/user/xyan/xy.ai.workbench/project/resident2/runs/split_optimized/05.yml)
```