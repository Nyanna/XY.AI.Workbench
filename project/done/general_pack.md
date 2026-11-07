Generalisiere `/home/user/xyan/xy.ai.workbench/codegen/pack-jar.sh` über Argumente und füge die Packing Funktionen den Builds an.

Builds:
`/home/user/xyan/xy.ai.workbench/codegen/anthropic.sh`
`/home/user/xyan/xy.ai.workbench/codegen/deepseek.sh`
SystemInit:  id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 0ee4-101
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/pack-jar.sh
  - path: /home/user/xyan/xy.ai.workbench/codegen/anthropic.sh
  - path: /home/user/xyan/xy.ai.workbench/codegen/deepseek.sh
  reason: Read the packing script and both build scripts to generalize and integrate.
```
/answer 0ee4-101 allow
ReasoningToken: 364
Control Request:
```yaml
id: 0ee4-102
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/pack-jar.sh
    mode: replace
    content: |
      #!/bin/bash
      set -euo pipefail

      if [[ $# -lt 2 || $# -gt 3 ]]; then
        echo "Usage: $0 <package> <jar-file> [base-dir=.bin]" >&2
        exit 2
      fi

      PKG_PATH="${1//.//}"
      JAR_FILE="$2"
      BASE_DIR="${3:-.bin}"

      SRC_DIR="$BASE_DIR/$PKG_PATH"

      [[ -d "$SRC_DIR" ]] || { echo "Dir not found: $SRC_DIR" >&2; exit 1; }
      [[ -n "$(ls -A "$SRC_DIR")" ]] || { echo "Dir is empty: $SRC_DIR" >&2; exit 1; }
      command -v jar >/dev/null || { echo "'jar' not found" >&2; exit 1; }

      jar --create --file "$JAR_FILE" -C "$BASE_DIR" "$PKG_PATH"

      if ! jar --list --file "$JAR_FILE" | grep -q "^$PKG_PATH/"; then
        echo "JAR check failed" >&2
        exit 1
      fi

      find "$SRC_DIR" -mindepth 1 -delete

      echo "Created: $JAR_FILE ($(jar --list --file "$JAR_FILE" | grep -vc '/$') Files), $SRC_DIR cleared."
  - path: /home/user/xyan/xy.ai.workbench/codegen/anthropic.sh
    mode: replace
    content: |
      #!/bin/bash
      set -euo pipefail

      export PYTHONDONTWRITEBYTECODE=1

      PKG="xy.ai.conscious.connector.openapi.anthropic"

      clear
      PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/anthropic.filtered.yaml --out ../connector/src --base-package "$PKG"

      "$(dirname "$0")/pack-jar.sh" "$PKG" anthropic-connector.jar
  - path: /home/user/xyan/xy.ai.workbench/codegen/deepseek.sh
    mode: replace
    content: |
      #!/bin/bash
      set -euo pipefail

      export PYTHONDONTWRITEBYTECODE=1

      PKG="xy.ai.conscious.connector.openapi.deepseek"

      clear
      PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/deepseek.filtered.yaml --out ../connector/src --base-package "$PKG"

      "$(dirname "$0")/pack-jar.sh" "$PKG" deepseek-connector.jar
  reason: Generalize pack-jar.sh via arguments and call it from both build scripts.
```
/answer 0ee4-102 allow
Result Stats: id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, total: 7957, in: 6, out: 2599, reason: 0, read: 5908, write: 5352, start: 1794046152643, end: 1794046159738, time: 7033, total: 20621, io: 0, control: 62, sent: 0, recv: 172
Ich habe `pack-jar.sh` über Argumente verallgemeinert und beide Build-Skripte rufen es jetzt nach der Generierung auf. Ausgeführt oder getestet habe ich nichts.

**`pack-jar.sh`** hat jetzt die Syntax `pack-jar.sh <package> <jar-file> [base-dir=.bin]`.
- Der Package-Punktpfad wird in einen Verzeichnispfad umgewandelt, aus `a.b.c` wird also `a/b/c`.
- Ohne die zwei Pflichtargumente bricht das Skript mit einer Usage-Meldung ab.
- Die Prüfungen, das Anlegen der JAR und das Leeren des Quellverzeichnisses sind unverändert.

**`anthropic.sh` und `deepseek.sh`**
- Jedes Skript hat jetzt eine Variable `PKG`, die sowohl `--base-package` als auch der Packing-Aufruf nutzen.
- Danach rufen sie `pack-jar.sh "$PKG" <name>-connector.jar` auf, also `anthropic-connector.jar` bzw. `deepseek-connector.jar`.
- Ich habe `set -euo pipefail` ergänzt, damit das Packing nur nach einer erfolgreichen Generierung läuft.
- Das `clear` steht jetzt in einer eigenen Zeile statt in der `&&`-Kette.

**Mögliche Lücke bei den Pfaden:**
- Der alte Pack-Default zeigte auf `xy/ai/workbench/...`, die Generatoren nutzen aber `xy.ai.conscious...`. Ich habe das Package der Generatoren übernommen.
- Außerdem erzeugt der Generator Quellen in `../connector/src`, während `pack-jar.sh` standardmäßig aus `.bin` packt. Dort müssten kompilierte Klassen liegen, und ein Compile-Schritt fehlt in den Builds.
- Wenn die JAR aus dem Generator-Output entstehen soll, übergib als drittes Argument den passenden Ordner. Falls `.bin` aus einem separaten Compile-Schritt kommt, muss dieser vor dem Packing laufen.

Wie sollen die Skripte hier vorgehen?
Gib mir einen Moment
Result Stats: id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, total: 8833, in: 8, out: 2631, reason: 0, read: 11260, write: 6194, start: 1794046346787, end: 1794046350754, time: 3913, total: 24534, io: 0, control: 54, sent: 1, recv: 18
SystemInit:  id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Klar, nimm dir die Zeit. Ich warte, bis du dich meldest.
richtiges bin ist connector/.bin
SystemInit:  id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 58
Control Request:
```yaml
id: 0ee4-103
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/codegen/anthropic.sh
    mode: replace
    content: |
      #!/bin/bash
      set -euo pipefail

      export PYTHONDONTWRITEBYTECODE=1

      PKG="xy.ai.conscious.connector.openapi.anthropic"

      clear
      PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/anthropic.filtered.yaml --out ../connector/src --base-package "$PKG"

      "$(dirname "$0")/pack-jar.sh" "$PKG" anthropic-connector.jar ../connector/.bin
  - path: /home/user/xyan/xy.ai.workbench/codegen/deepseek.sh
    mode: replace
    content: |
      #!/bin/bash
      set -euo pipefail

      export PYTHONDONTWRITEBYTECODE=1

      PKG="xy.ai.conscious.connector.openapi.deepseek"

      clear
      PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/deepseek.filtered.yaml --out ../connector/src --base-package "$PKG"

      "$(dirname "$0")/pack-jar.sh" "$PKG" deepseek-connector.jar ../connector/.bin
  reason: Pass connector/.bin as base dir to pack-jar.sh in both builds.
```
/answer 0ee4-103 allow
Result Stats: id=2e18b97f-3dfb-48d8-9791-ee9c254c0baa, total: 10601, in: 12, out: 3521, reason: 0, read: 23721, write: 7068, start: 1794046469310, end: 1794046472270, time: 2913, total: 32573, io: 0, control: 47, sent: 0, recv: 52
Beide Builds übergeben jetzt `../connector/.bin` als drittes Argument an `pack-jar.sh`. Der Pfad ist relativ zu `codegen/` gedacht, wie auch `--out ../connector/src`. Getestet habe ich es nicht.

Die JAR-Dateien (`anthropic-connector.jar`, `deepseek-connector.jar`) landen weiterhin im aktuellen Arbeitsverzeichnis. Sie sollen eventuell nach `../connector/` oder in einen anderen Ordner. Dann ändere ich den JAR-Pfad.
