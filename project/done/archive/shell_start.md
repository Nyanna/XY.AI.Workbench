Ich brauche für `/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/Main.java` ein Bash Script zum starten in `/home/user/xyan/xy.ai.workbench/ast-engines/java`.

Relevant: `/home/user/xyan/xy.ai.workbench/ast-engines/java/.classpath`
SystemInit:  id=41d3499e-7d30-4484-8484-4ea0975db173, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 8d71-131
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/ast-engines/java
  script: cat .classpath; echo "---"; find . -iname "*.jar" | head -50; echo "---"; find . -maxdepth 3 -iname "Main.java"
  reason: ".classpath und Projektstruktur prüfen, um Klassenpfad und Kompilierziel für das Start-Script zu bestimmen"
```
/answer 8d71-131 allow
Control Request:
```yaml
id: 8d71-132
result:
  structuredContent:
    stdout: "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<classpath>\n\t<classpathentry kind=\"con\" path=\"org.eclipse.jdt.launching.JRE_CONTAINER/org.eclipse.jdt.internal.debug.ui.launcher.StandardVMType/JavaSE-25\">\n\t\t<attributes>\n\t\t\t<attribute name=\"module\" value=\"true\"/>\n\t\t</attributes>\n\t</classpathentry>\n\t<classpathentry kind=\"src\" path=\"src\"/>\n\t<classpathentry kind=\"lib\" path=\"libs/jackson-core-2.20.0.jar\"/>\n\t<classpathentry kind=\"lib\" path=\"libs/jackson-databind-2.20.0.jar\"/>\n\t<classpathentry kind=\"lib\" path=\"libs/javaparser-core-3.28.2.jar\" sourcepath=\"libs/javaparser-core-3.28.2-sources.jar\">\n\t\t<attributes>\n\t\t\t<attribute name=\"javadoc_location\" value=\"jar:platform:/resource/XY.AI.ast-java/libs/javaparser-core-3.28.2-javadoc.jar!/\"/>\n\t\t</attributes>\n\t</classpathentry>\n\t<classpathentry kind=\"lib\" path=\"libs/jackson-annotations-3.0-rc5.jar\"/>\n\t<classpathentry kind=\"output\" path=\".bin\"/>\n</classpath>\n---\n./libs/javaparser-core-3.28.2.jar\n./libs/javaparser-core-3.28.2-javadoc.jar\n./libs/jackson-core-2.20.0.jar\n./libs/jackson-annotations-3.0-rc5.jar\n./libs/jackson-databind-2.20.0.jar\n./libs/javaparser-core-3.28.2-sources.jar\n---\n"
```
/answer 8d71-132 allow
ReasoningToken: 23
Control Request:
```yaml
id: 8d71-133
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/run.sh
    source: |
      #!/usr/bin/env bash
      set -euo pipefail

      SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
      cd "$SCRIPT_DIR"

      SRC_DIR="src"
      BIN_DIR=".bin"
      LIBS_DIR="libs"
      MAIN_CLASS="xy.ai.mcpc.ast.engine.Main"

      CP="$BIN_DIR:$(find "$LIBS_DIR" -maxdepth 1 -name '*.jar' ! -name '*-sources.jar' ! -name '*-javadoc.jar' | paste -sd:)"

      mkdir -p "$BIN_DIR"

      if [[ "${1:-}" == "--build" ]]; then
        find "$SRC_DIR" -name '*.java' > /tmp/ast-engine-sources.txt
        javac -encoding UTF-8 -d "$BIN_DIR" -cp "$CP" @/tmp/ast-engine-sources.txt
        shift
      fi

      exec java -cp "$CP" "$MAIN_CLASS" "$@"
  reason: "Start-Script für Main.java erstellen basierend auf .classpath (src, libs, .bin)"
```
/answer 8d71-133 allow
Control Request:
```yaml
id: 8d71-134
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/ast-engines/java
  script: chmod +x run.sh && ./run.sh --build --help 2>&1 | head -30
  reason: Script ausführbar machen und testen (Build + Start)
```
/answer 8d71-134 allow
Control Request:
```yaml
id: 8d71-135
result:
  structuredContent:
    stdout: "Exception in thread \"main\" java.lang.NumberFormatException: For input string: \"--help\"\n\tat java.base/java.lang.NumberFormatException.forInputString(NumberFormatException.java:67)\n\tat java.base/java.lang.Integer.parseInt(Integer.java:662)\n\tat java.base/java.lang.Integer.parseInt(Integer.java:778)\n\tat xy.ai.mcpc.ast.engine.Main.main(Main.java:28)\n"
```
/answer 8d71-135 allow
Result Stats: id=41d3499e-7d30-4484-8484-4ea0975db173, total: 3816, in: 139, out: 1204, reason: 0, read: 25937, write: 2473
Build und Start funktionieren – `run.sh` kompiliert (`--build`) und startet `Main` mit dem korrekten Classpath. `--help` ist kein gültiges Argument des Programms (erwartet z.B. einen Port), aber das Script selbst läuft korrekt.

Nutzung:
- `./run.sh --build <args>` – kompiliert nach `.bin` und startet
- `./run.sh <args>` – startet ohne Neu-Kompilierung
