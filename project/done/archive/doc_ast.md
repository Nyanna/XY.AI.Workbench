Ich brauche eine Ausarbeitung der These, Kernargumente und Umsetzung des Nutzers in englischer Sprache.

# Material

## You said:
Ich brauche analog Python AST etwas für Java

## Agent said:
Ja. Für Java gibt es mehrere gute AST-Bibliotheken, je nachdem, was du mit dem AST machen willst:
| Bibliothek          | Vergleich zu Python `ast` | Stärke                                                     |
| ------------------- | ------------------------- | ---------------------------------------------------------- |
| **Eclipse JDT AST** | ⭐⭐⭐⭐⭐                     | Sehr vollständiger Java-AST, Bindings/Typauflösung         |
| **JavaParser**      | ⭐⭐⭐⭐⭐                     | Einfach API, gut für Analysen und Code-Manipulation        |
| **Spoon**           | ⭐⭐⭐⭐                      | Hoch-level Java-Modell, sehr gut für Refactoring/Analyse   |
| **Tree-sitter**     | ⭐⭐⭐                       | Schneller Concrete Syntax Tree, auch aus Nicht-Java heraus |
**Wenn du etwas möchtest, das sich wie `ast` in Python anfühlt, würde ich zuerst JavaParser anschauen.**\
JDT ist mächtiger, aber deutlich komplexer. JDT kann unter anderem ASTs aus Sourcecode erzeugen, Source-Positionen erhalten und optional Bindings auflösen. [![](https://www.google.com/s2/favicons?domain=https://help.eclipse.org\&sz=128)Eclipse Help+1](https://help.eclipse.org/latest/ntopic/org.eclipse.jdt.doc.isv/reference/api/org/eclipse/jdt/core/dom/ASTParser.html?utm_source=chatgpt.com)
Beispiel mit **JavaParser**:
```
CompilationUnit cu = StaticJavaParser.parse("""
    class Foo {
        int add(int a, int b) {
            return a + b;
        }
    }
""");
cu.findAll(MethodDeclaration.class).forEach(method -> {
    System.out.println(method.getName());
    System.out.println(method.getParameters());
});
```
Das entspricht konzeptionell etwa:
```
tree = ast.parse(source)
for node in ast.walk(tree):
    if isinstance(node, ast.FunctionDef):
        print(node.name)
```
Mit **Eclipse JDT** sieht es eher so aus:
```
ASTParser parser = ASTParser.newParser(AST.getJLSLatest());
parser.setSource(source.toCharArray());
CompilationUnit cu = (CompilationUnit) parser.createAST(null);
cu.accept(new ASTVisitor() {
    @Override
    public boolean visit(MethodDeclaration node) {
        System.out.println(node.getName());
        return true;
    }
});
```
JDT unterstützt außerdem das Modifizieren und anschließende Wieder-Ausgeben des ASTs. [![](https://www.google.com/s2/favicons?domain=https://help.eclipse.org\&sz=128)Eclipse Help+1](https://help.eclipse.org/latest/rtopic/org.eclipse.jdt.doc.isv/reference/api/org/eclipse/jdt/core/dom/AST.html?utm_source=chatgpt.com)
**Spoon** ist besonders interessant, wenn du nicht nur parsen, sondern z. B. Klassen, Methoden, Typen und Aufrufbeziehungen systematisch analysieren oder transformieren möchtest. [![](https://www.google.com/s2/favicons?domain=https://spoon.gforge.inria.fr\&sz=128)Spoon](https://spoon.gforge.inria.fr/launcher.html?utm_source=chatgpt.com)
Wenn du mir sagst, ob du **Java-Code analysieren, transformieren/refactoren oder z. B. Methoden/Klassen extrahieren** willst, kann ich dir die passende Library samt Maven-Dependency und Python-`ast`-zu-Java-Äquivalenten zeigen.

## You said:
Python AST ist relativ stupide und reduziert alles auf die sematik. Die komplette formatierung geht dabei verloren. Das ist genau der zentrale aspekt den ich brauche. Java uniform auf die semantik runterbrechen, anschließend editieren und dann wieder zurückschreiben. Keine formatierungsänderungen.

## Agent said:
Ja — dann ist **JavaParser mit `LexicalPreservingPrinter`** sehr nah an genau deinem Anwendungsfall.
Der entscheidende Unterschied zu Python `ast` ist:
* **AST**: semantisch/strukturell arbeiten
* **Token-/Lexical-Layer**: Originaltext, Whitespace, Kommentare und Layout behalten
* Beim Zurückschreiben werden **unveränderte Bereiche aus dem Original übernommen**; geändert wird nur der notwendige Bereich. JavaParser beschreibt genau dieses Prinzip als „keep the source code … as close as possible to the source code that was parsed“. [![](https://www.google.com/s2/favicons?domain=https://javaparser.org\&sz=128)JavaParser+1](https://javaparser.org/making-strings-in-javaparser/?utm_source=chatgpt.com)
### Genau dein Modell
```
Java source
    │
    ▼
Parser
    │
    ├── AST  ───────────────► deine semantische Manipulation
    │
    └── Lexical information
              │
              ▼
        LexicalPreservingPrinter
              │
              ▼
        Java source
```
Beispiel:
```
class Foo{  // völlig absichtlich hässlich
    int   x=1;
    void   bar( ) {
        x++;
    }
}
```
Du machst semantisch nur:
```
x = 42;
```
Dann soll idealerweise herauskommen:
```
class Foo{  // völlig absichtlich hässlich
    int   x=42;
    void   bar( ) {
        x++;
    }
}
```
und **nicht**:
```
class Foo {
    int x = 42;
    void bar() {
        x++;
    }
}
```
Genau für diesen Unterschied ist der `LexicalPreservingPrinter` gedacht. Er speichert die ursprüngliche Textrepräsentation zusammen mit den AST-Strukturen und verfolgt Änderungen am AST. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/javaparser/javaparser/blob/master/javaparser-core/src/main/java/com/github/javaparser/printer/lexicalpreservation/LexicalPreservingPrinter.java?utm_source=chatgpt.com)
### Minimaler Aufbau
```
String source = Files.readString(path);
CompilationUnit cu =
    LexicalPreservingPrinter.setup(
        StaticJavaParser.parse(source)
    );
// Semantische Manipulation
cu.findAll(VariableDeclarator.class)
  .stream()
  .filter(v -> v.getNameAsString().equals("x"))
  .forEach(v -> v.setInitializer("42"));
String result = LexicalPreservingPrinter.print(cu);
```
Damit hast du ziemlich genau:
```
Original
   ↓
AST als abstrahierte Arbeitsrepräsentation
   ↓
AST editieren
   ↓
ursprüngliche Lexik + gezielte Änderungen
   ↓
Originalformatierung erhalten
```
### Noch interessanter: Eclipse JDT
**JDT `ASTRewrite`** verfolgt sogar explizit das Konzept von **minimalen Textänderungen**. Die API erzeugt Text-Edits auf Basis des ursprünglichen Sourcecodes und versucht dabei, vorhandene Kommentare und Einrückungen zu erhalten. [![](https://www.google.com/s2/favicons?domain=https://help.eclipse.org\&sz=128)Eclipse Help](https://help.eclipse.org/latest/topic/org.eclipse.jdt.doc.isv/reference/api/org/eclipse/jdt/core/dom/rewrite/ASTRewrite.html?utm_source=chatgpt.com)
Das ist für einen Refactoring-Engine-Ansatz ebenfalls sehr interessant.
Der wichtige Unterschied wäre:
|                          | JavaParser                   | JDT                      |
| ------------------------ | ---------------------------- | ------------------------ |
| AST angenehm zu benutzen | **sehr gut**                 | eher sperrig             |
| Originalformatierung     | **LexicalPreservingPrinter** | ASTRewrite               |
| Kommentare               | ja                           | ja                       |
| Semantische Typauflösung | Symbol Solver                | **sehr stark**, Bindings |
| Minimaler Text-Diff      | gut                          | **sehr stark**           |
| API-Ergonomie            | **besser**                   | komplexer                |
JDT hält für AST-Knoten Source-Ranges zurück und `ASTRewrite` arbeitet anschließend auf dem Originaltext, statt einfach den gesamten Baum neu zu pretty-printen. [![](https://www.google.com/s2/favicons?domain=https://help.eclipse.org\&sz=128)Eclipse Help+1](https://help.eclipse.org/latest/rtopic/org.eclipse.jdt.doc.isv/reference/api/org/eclipse/jdt/core/dom/AST.html?utm_source=chatgpt.com)
**Für das von dir beschriebene „AST als semantische Schicht über einem unveränderten Source-Text“ würde ich daher konkret zwischen `JavaParser + LexicalPreservingPrinter` und `JDT + ASTRewrite` wählen.**
Wenn das Ziel sogar **bytegenaues Round-Trip-Verhalten** ist — also auch exotische Whitespace-Varianten, Kommentare an ungewöhnlichen Stellen usw. — würde ich die Architektur noch etwas anders aufziehen: **CST/Tokenstream als Source-of-Truth + AST als semantische Ansicht**. Tree-sitter verfolgt genau dieses CST-Modell. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/tree-sitter/tree-sitter/blob/master/docs/src/index.md?utm_source=chatgpt.com)

## You said:
Du hast mich missverstanden. Die formatierung soll entfernt werden, ich will reine uniformae semantik ohne irgendwelche zeichen und formatierungen. Reiner strukturgraph.

## Agent said:
Ah, jetzt klar: **genau wie Python `ast`**. Die konkrete Darstellung des Java-Quelltexts ist irrelevant. Du willst:
```
Java-Source
   ↓
Parser
   ↓
reiner semantischer Strukturbaum/-graph
   ↓
manipulieren
   ↓
Java-Source neu erzeugen
```
Also z. B.
```
int x=1;
```
und
```
int   x = 1;
```
und
```
int x =
    1;
```
werden zu **demselben Modell**.
Dafür sind **JavaParser** und **Eclipse JDT** geeignet. Du brauchst ausdrücklich **nicht** `LexicalPreservingPrinter`.
### Am ehesten Python-`ast` → JavaParser
JavaParser bietet einen klassischen AST:
```
CompilationUnit cu = StaticJavaParser.parse(source);
cu.findAll(MethodDeclaration.class).forEach(method -> {
    System.out.println(method.getNameAsString());
});
```
Der AST enthält Dinge wie:
```
CompilationUnit
└── ClassDeclaration
    └── MethodDeclaration
        ├── Parameter
        ├── ReturnType
        └── BlockStmt
            └── ReturnStmt
                └── BinaryExpr
                    ├── NameExpr
                    └── IntegerLiteralExpr
```
Whitespace, Einrückung usw. sind für diese Struktur **nicht relevant**.
### Wenn du wirklich einen semantischen Strukturgraphen willst
Dann würde ich mir **Spoon** besonders ansehen.
Spoon geht über einen klassischen AST hinaus und stellt ein **höherleveliges, semantisches Modell von Java** bereit:
```
CtModel
 ├── CtType
 ├── CtMethod
 ├── CtParameter
 ├── CtInvocation
 ├── CtFieldAccess
 ├── CtLiteral
 └── ...
```
Dazu kommen Beziehungen wie Typen, Methoden, Referenzen, Aufrufe usw. Das ist wesentlich näher an:
> „Ich möchte Java auf eine uniforme semantische Repräsentation reduzieren.“
als an:
> „Ich möchte den konkreten Parse Tree von Java.“
### Meine Einordnung
Wenn dein Ziel ist:
```
Python ast
    ↓
Java-Äquivalent
```
→ **JavaParser**
Wenn dein Ziel ist:
```
Java
 ↓
normalisiertes semantisches Modell / Strukturgraph
 ↓
Analyse + Transformation
 ↓
Java
```
→ **Spoon**
Wenn du beispielsweise
```
foo.bar(x + 1)
```
nicht als irgendwelche konkreten Syntaxknoten betrachten willst, sondern semantisch eher als
```
Invocation
  target = foo
  method = bar
  argument =
      Add
        x
        1
```
dann bist du konzeptionell genau bei dem Modell, das du beschreibst.
Der entscheidende Punkt ist also: **Du suchst einen AST/Program-Model-Parser, nicht einen formatierungserhaltenden Parser.**

## You said:
Ich brauche dabei keine relation oder typauslösung. Die reine semantische syntaxstruktur.

## Agent said:
Dann ist **JavaParser wahrscheinlich genau das, was du suchst**.
Du brauchst nur:
* Parsing von Java
* reine Syntax-/Semantikstruktur
* keine Source-Formatierung
* keine Kommentare/Whitespace
* keine Symbolauflösung
* keine Typauflösung
* keine Referenzrelationen
Also im Grunde **„Python `ast`, nur für Java“**.
JavaParser liefert dir dafür einen normalen AST mit Knoten wie:
```
CompilationUnit
ClassDeclaration
MethodDeclaration
Parameter
BlockStmt
VariableDeclarationExpr
AssignExpr
BinaryExpr
MethodCallExpr
IfStmt
ForStmt
...
```
Beispiel:
```
CompilationUnit cu = StaticJavaParser.parse(source);
for (MethodDeclaration m : cu.findAll(MethodDeclaration.class)) {
    System.out.println(m.getNameAsString());
}
```
Du kannst den Baum auch selbst traversieren:
```
cu.accept(new VoidVisitorAdapter<Void>() {
    @Override
    public void visit(BinaryExpr n, Void arg) {
        System.out.println(n.getOperator());
        super.visit(n, arg);
    }
});
```
**Spoon wäre für deinen Fall eher unnötig**, weil es zusätzlich ein deutlich reichhaltigeres Programm- und Referenzmodell aufspannt.
Eine wichtige Nuance: JavaParser ist ein **konkreter AST mit Java-spezifischen Syntaxknoten**, nicht ein maximal kanonisiertes semantisches IR. Beispielsweise bleiben bestimmte syntaktische Unterschiede als unterschiedliche Knoten/Repräsentationen erhalten, obwohl sie dieselbe Bedeutung haben.
Wenn du wirklich **Syntax-Normalisierung** willst, also etwa mehrere Java-Schreibweisen auf exakt denselben kanonischen Strukturgraphen abbilden möchtest, wäre das ein Schritt **oberhalb von JavaParser**: JavaParser → eigener normalisierter IR.
Das klingt bei deiner Beschreibung sogar ziemlich genau nach dem, was du eigentlich suchst.

## You said:
Welche abhängigkeiten hat JavaParser?

## Agent said:
Für deinen Anwendungsfall reicht **`javaparser-core`**. Die aktuelle Version ist **3.28.2** (1. Juni 2026). [![](https://www.google.com/s2/favicons?domain=https://mvnrepository.com\&sz=128)Maven Repository+1](https://mvnrepository.com/artifact/com.github.javaparser/javaparser-core/versions?utm_source=chatgpt.com)
```
<dependency>
    <groupId>com.github.javaparser</groupId>
    <artifactId>javaparser-core</artifactId>
    <version>3.28.2</version>
</dependency>
```
**Wichtig:** `javaparser-core` hat laut Maven Repository **keine transitive Dependency**. [![](https://www.google.com/s2/favicons?domain=https://mvnrepository.com\&sz=128)Maven Repository](https://mvnrepository.com/artifact/com.github.javaparser/javaparser-core/3.26.3/used-by?utm_source=chatgpt.com) Das ist für dein Ziel ziemlich attraktiv: ein einzelnes Parser-JAR.
Du brauchst **nicht**:
```
javaparser-symbol-solver-core
```
Das ist eine zusätzliche Schicht für Symbol-/Typauflösung und Referenzen — genau das, was du explizit nicht brauchst. Das Symbol-Solver-Modul baut auf JavaParser auf. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/javaparser/javasymbolsolver-maven-sample/blob/master/pom.xml?utm_source=chatgpt.com)
Wenn du möchtest, kann ich dir als Nächstes die **komplette AST-Klassenstruktur von JavaParser für Java** gegenüber Python `ast` gegenüberstellen.

## You said:
gibt es eine CLI ?

## Agent said:
Nicht als fertiges **offizielles `javaparser`-CLI**, soweit das aktuelle Projekt zeigt. JavaParser wird primär als Library ausgeliefert; die offiziellen Beispiele starten eigene Java-Programme, die `javaparser-core` einbinden. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/javaparser/javaparser?utm_source=chatgpt.com)
Für deinen Anwendungsfall ist aber interessant: Es gibt ein separates Modul **`javaparser-core-serialization`**, mit dem sich der AST als **JSON serialisieren** lässt. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/javaparser/javaparser?utm_source=chatgpt.com)
Damit könntest du sehr einfach ein eigenes Tool bauen:
```
java2ast Foo.java
```
→
```
{
  "nodeType": "CompilationUnit",
  "types": [
    {
      "nodeType": "ClassDeclaration",
      ...
    }
  ]
}
```
Das wäre für dein ursprüngliches Ziel vermutlich sogar ideal: **Java → reiner Strukturgraph → JSON**, ohne Typauflösung oder sonstige Semantik.
Ich kann dir auch ein **standalone `java2ast`-CLI als einzelnes JAR** bauen, das `stdin`, Dateien und JSON-Ausgabe unterstützt.

## You said:
Mein MCP ist python. Wie bekomme ich effizient den Java AST so in python datenstrukturen und wieder zurück?

## Agent said:
Worked for 7s
Ja. Für einen Python-MCP würde ich **nicht** versuchen, JavaParser direkt aus Python zu bedienen. Ich würde einen kleinen **lang laufenden Java-Worker** dazwischen setzen:
```
Python MCP
   │
   │ JSON / JSONL über stdin/stdout
   ▼
JavaParser Worker
   │
   ├── parse(source) → AST-JSON
   └── AST-JSON → JavaParser AST → pretty-print → source
```
JavaParser kann ASTs direkt nach JSON serialisieren und auch wieder deserialisieren; dafür gibt es `javaparser-core-serialization`. Das Projekt dokumentiert beide Richtungen explizit. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/javaparser/javaparser?utm_source=chatgpt.com)
### Für deinen Fall würde ich es aber leicht anders machen
Du willst Python-seitig einen **sauberen semantischen Datenbaum**, nicht den kompletten JavaParser-internen AST. Also:
```
{
  "kind": "class",
  "name": "Foo",
  "members": [
    {
      "kind": "method",
      "name": "bar",
      "return_type": {"kind": "primitive", "name": "int"},
      "params": [],
      "body": [
        {
          "kind": "return",
          "value": {
            "kind": "binary",
            "op": "+",
            "left": {"kind": "name", "name": "a"},
            "right": {"kind": "int", "value": 1}
          }
        }
      ]
    }
  ]
}
```
Dann ist Python vollkommen unabhängig von JavaParser-Klassennamen und internen Details.
### Architektur
**Java-Seite**
```
CompilationUnit cu = StaticJavaParser.parse(source);
SemanticAst ast = convert(cu);
System.out.println(json.writeValueAsString(ast));
```
Beim Rückweg:
```
SemanticAst ast = json.readValue(input, SemanticAst.class);
CompilationUnit cu = convertBack(ast);
System.out.println(cu.toString());
```
Die eigentliche Konvertierung `JavaParser AST ↔ SemanticAst` ist bei dir dann die **Kanonsierungsschicht**.
Das ist m.E. besser als JavaParser-JSON 1:1 nach Python zu exponieren, weil dessen Serialisierung auch interne AST-Aspekte wie `RANGE` und `TOKEN_RANGE` kennt. [![](https://www.google.com/s2/favicons?domain=https://javadoc.io\&sz=128)javadoc.io](https://javadoc.io/static/com.github.javaparser/javaparser-core-serialization/3.25.9/index-all.html?utm_source=chatgpt.com)
### Warum ein persistenter Worker?
Nicht:
```
subprocess.run(["java", "parser.jar"], ...)
```
bei jedem MCP-Aufruf.
Sondern einmal:
```
proc = subprocess.Popen(
    ["java", "-jar", "java-parser-worker.jar"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    text=True,
)
```
und dann JSONL:
```
{"op":"parse","source":"class Foo { ... }"}
{"op":"generate","ast":{...}}
```
Antwort jeweils eine JSON-Zeile.
Damit hast du praktisch eine kleine **Java-AST-RPC-Schnittstelle**, ohne JNI, JPype oder JVM-Einbettung.
### Noch wichtiger
Weil du **keine Typauflösung/Relationen** willst, brauchst du wirklich nur:
```
<dependency>
    <groupId>com.github.javaparser</groupId>
    <artifactId>javaparser-core</artifactId>
    <version>3.28.2</version>
</dependency>
```
und für die eingebaute JSON-Roundtrip-Serialisierung zusätzlich:
```
<dependency>
    <groupId>com.github.javaparser</groupId>
    <artifactId>javaparser-core-serialization</artifactId>
    <version>3.28.2</version>
</dependency>
```
JavaParser selbst beschreibt `javaparser-core` als ausreichend für Parsing/AST-Traversierung/Manipulation und das separate Serialization-Modul für JSON. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/javaparser/javaparser?utm_source=chatgpt.com)
**Für dein MCP würde ich sogar einen Schritt weitergehen und ein kleines `java-ast-worker.jar` mit genau zwei RPCs bauen:**
```
parse   : Java source → kanonischer JSON-IR
generate: kanonischer JSON-IR → Java source
```
Dann kannst du den AST in Python ganz normal als `dict`/`list` bearbeiten.
Ich kann dir dafür ein **komplettes minimales Java-Worker-JAR + Python-Klasse** skizzieren, inklusive deskanonischer AST-Struktur.

## You said:
du meinst pipe basiert mit JSON protokoll?

## Agent said:
Ja, genau.
**Lang laufender Java-Prozess + JSONL über `stdin/stdout`**:
```
Python MCP
   │
   │ stdin:  {"op":"parse","source":"..."}\n
   ▼
Java Worker
   │
   │ stdout: {"ast":{...}}\n
   ▼
Python
```
JSONL ist hier praktischer als ein einzelnes JSON-Dokument pro Prozess, weil du den JVM-Start nur einmal bezahlst und beliebig viele Requests schicken kannst.
Wichtig wäre dabei:
```
stdin  → ausschließlich Protokoll
stdout → ausschließlich Protokoll
stderr → Logs/Fehler
```
Also etwa:
```
{"id":1,"op":"parse","source":"class Foo { int x; }"}
```
Antwort:
```
{"id":1,"ok":true,"ast":{"kind":"class","name":"Foo","fields":[...]}}
```
Und zurück:
```
{"id":2,"op":"generate","ast":{...}}
```
Antwort:
```
{"id":2,"ok":true,"source":"class Foo {\n    int x;\n}"}
```
Für einen MCP ist das attraktiv, weil Python einfach `dict`/`list` bekommt und keine Java-Objekte oder JVM-Bindings verwalten muss.
Ich würde allerdings **dein eigenes JSON-IR** übertragen und nicht blind den JavaParser-internen AST serialisieren. Dadurch bleibt die Schnittstelle stabil und du kannst die Semantikstruktur selbst definieren.

## You said:
gibt es ein effizienteres RPC zwischen Java und Python?

## Agent said:
Worked for 7s
Ja. **Für deinen Fall würde ich JSONL nicht als RPC-Protokoll nehmen.** Ich würde zwischen Transport und Serialisierung trennen:
### Meine Empfehlung
```
Python MCP
   │
   │ Unix Domain Socket
   │ length-prefixed MessagePack
   ▼
Java Worker
```
**MessagePack** ist binär, kleiner und schneller als JSON und hat sowohl für Python als auch Java etablierte Implementierungen. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/msgpack/msgpack-python?utm_source=chatgpt.com)
Java unterstützt Unix-Domain-Sockets nativ seit JDK 16. [![](https://www.google.com/s2/favicons?domain=https://docs.oracle.com\&sz=128)Oracle Docs](https://docs.oracle.com/en/java/javase/26/docs/api/java.base/java/net/UnixDomainSocketAddress.html?utm_source=chatgpt.com)
Für deinen AST passt das besonders gut, weil dein Modell ohnehin natürlich auf diese primitiven Strukturen abbildbar ist:
```
dict
list
string
integer
boolean
null
```
Also etwa:
```
{
    "kind": "binary",
    "op": "+",
    "left": {"kind": "name", "name": "x"},
    "right": {"kind": "literal", "value": 1}
}
```
wird direkt zu MessagePack und zurück.
### Warum nicht gRPC?
**Protobuf + gRPC** wäre meine Wahl, wenn du ein stark typisiertes, versioniertes RPC-Interface brauchst. Protobuf erzeugt sowohl Java- als auch Python-Code aus demselben Schema. [![](https://www.google.com/s2/favicons?domain=https://protobuf.dev\&sz=128)Protocol Buffers+1](https://protobuf.dev/programming-guides/editions/?utm_source=chatgpt.com)
Aber für deinen AST sehe ich einen Nachteil:
```
Python dict
   ↓
protobuf Message
   ↓
Java
```
Du brauchst dann eine zusätzliche Konvertierung zwischen deinem dynamischen AST und den generierten Protobuf-Klassen.
gRPC kann zwar ebenfalls Unix-Domain-Sockets verwenden, ist aber für einen lokalen, einfachen Request/Response-Worker relativ viel Infrastruktur. [![](https://www.google.com/s2/favicons?domain=https://chromium.googlesource.com\&sz=128)Chromium Git Repositories](https://chromium.googlesource.com/external/github.com/grpc/grpc/%2B/refs/heads/master/examples/python/uds/?utm_source=chatgpt.com)
### Noch schneller?
**Cap'n Proto** wäre interessant, wenn die ASTs wirklich riesig werden und du möglichst wenig Kopieren/Deserialisieren möchtest. Es ist auf sehr effizientes Encoding und RPC ausgelegt und besitzt Python-Unterstützung. [![](https://www.google.com/s2/favicons?domain=https://capnproto.github.io\&sz=128)Cap'n Proto](https://capnproto.github.io/pycapnp/quickstart.html?utm_source=chatgpt.com)
Aber ich würde nicht damit anfangen.
### Für deinen konkreten Anwendungsfall
Ich würde es so bauen:
```
                 ┌─────────────────────┐
                 │    Python MCP       │
                 │                     │
                 │ Python dict/list    │
                 └─────────┬───────────┘
                           │
                    MessagePack
                           │
                    Unix Socket
                           │
                 ┌─────────▼───────────┐
                 │    Java Worker      │
                 │                     │
                 │ JavaParser          │
                 │      ↓              │
                 │ Semantic AST        │
                 └─────────────────────┘
```
Und das Protokoll sehr klein halten:
```
request  = {
    id:    int,
    op:    "parse" | "generate",
    data:  ...
}
response = {
    id:    int,
    ok:    bool,
    data:  ...
}
```
Für **einen einzelnen langlebigen Java-Prozess** ist sogar ein Unix-Socket nicht zwingend schneller als `stdin/stdout`; der große Gewinn kommt bereits durch **binäre Serialisierung statt JSON**. Den Socket würde ich hauptsächlich wegen sauberem RPC-Design, Multiplexing und möglicher späterer Mehrprozess-Nutzung wählen.
**Meine konkrete Wahl wäre daher:**
> **JavaParser → eigenes kanonisches AST → MessagePack → Unix Domain Socket → Python `dict`**
Das ist sehr viel leichtergewichtig als gRPC und deutlich sauberer als JavaParser-Objekte via JPype/Py4J über die Sprachgrenze zu schleppen.

## You said:
Ja dann klingt das nach Protobuf. Ich könnte passend ein minimales LSP implementieren python als client. Mit protobuf kann ich spätere noch andere AST Server anschließen für rust oder node für javascript

## Agent said:
Ja — **die Richtung ist sehr sinnvoll**, mit einer kleinen begrifflichen Trennung:
**LSP selbst ist JSON-RPC**, nicht Protobuf. Der aktuelle LSP-Standard definiert die Nachrichten über JSON-RPC. [![](https://www.google.com/s2/favicons?domain=https://microsoft.github.io\&sz=128)Microsoft GitHub+1](https://microsoft.github.io/language-server-protocol/?utm_source=chatgpt.com)
Für dein Vorhaben würde ich deshalb eher ein **„LSP-artiges AST-RPC“** bauen:
```
Python MCP
    │
    │ Protobuf RPC
    ▼
Language/AST Server
    │
    ├── JavaParser
    ├── Rust parser
    ├── JS/TS parser
    └── ...
```
Das hat gegenüber echtem LSP sogar Vorteile, weil du dein **eigenes, sprachübergreifendes AST-Protokoll** definieren kannst.
### Der wichtige Teil: Protobuf als gemeinsame IR
Zum Beispiel:
```
message AstNode {
  oneof node {
    CompilationUnit compilation_unit = 1;
    ClassDecl class_decl = 2;
    FunctionDecl function_decl = 3;
    CallExpr call_expr = 4;
    BinaryExpr binary_expr = 5;
    ...
  }
}
```
Dann können verschiedene Server dasselbe Schema implementieren:
```
Java Server       → AstNode
Rust Server       → AstNode
JavaScript Server → AstNode
Python Client     ← AstNode
```
Protobuf erzeugt dafür direkt Java- und Python-Klassen aus demselben `.proto`. [![](https://www.google.com/s2/favicons?domain=https://protobuf.dev\&sz=128)Protocol Buffers+2![](https://www.google.com/s2/favicons?domain=https://protobuf.dev\&sz=128)Protocol Buffers+2](https://protobuf.dev/programming-guides/editions/?utm_source=chatgpt.com)
### Ich würde sogar zwei Schichten trennen
```
             RPC/API
                │
        ┌───────▼────────┐
        │ AstService     │
        ├────────────────┤
        │ parse          │
        │ generate       │
        │ capabilities   │
        └───────┬────────┘
                │
         protobuf messages
                │
     ┌──────────┼──────────┐
     ▼          ▼          ▼
   Java        Rust       JS/TS
```
Und `parse` sollte **nicht** einfach den nativen Parser-AST zurückgeben. Der Server übersetzt ihn in dein **kanonisches AST-Modell**.
Das ist langfristig der entscheidende Punkt: JavaParser kann intern völlig anders modellieren als dein Rust- oder JS-Parser, aber Python sieht immer dasselbe Modell.
### Ein möglicher API-Kern
```
service AstService {
  rpc Parse(ParseRequest) returns (ParseResponse);
  rpc Generate(GenerateRequest) returns (GenerateResponse);
  rpc GetCapabilities(CapabilitiesRequest)
      returns (CapabilitiesResponse);
}
```
Dann etwa:
```
message ParseRequest {
  string language = 1;
  string source = 2;
}
message ParseResponse {
  AstNode root = 1;
  repeated Diagnostic diagnostics = 2;
}
```
Damit kannst du später beispielsweise:
```
ast = client.parse("java", source)
ast = client.parse("rust", source)
ast = client.parse("javascript", source)
```
und **das gleiche Python-Transformationssystem** auf alle drei Sprachen anwenden — zumindest für den gemeinsamen Teil deines IR.
Der eigentliche Knackpunkt ist damit weniger RPC als das **Design des sprachübergreifenden AST-IR**. Genau dort würde ich zuerst sehr sorgfältig modellieren.

## You said:
Protobuf kann auch direkt Rust erzeugen oder JS

## Agent said:
Ja. **Protobuf kann Code für Rust und JavaScript/TypeScript generieren** – neben Python und Java.
Typischerweise:
```
schema.proto
   │
   ├── protoc + Python plugin  → Python
   ├── protoc + Java plugin    → Java
   ├── protoc + Rust plugin    → Rust
   └── protoc + JS/TS plugin   → JS / TS
```
Für Rust ist `prost` sehr verbreitet; für JavaScript gibt es die offiziellen Protobuf-Generatoren sowie Ökosysteme wie `ts-proto`.
Damit wird dein Ansatz ziemlich elegant:
```
                    common.proto
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Python         JavaParser      RustParser
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                    gemeinsames
                      AST-IR
```
Der wichtige Punkt ist: **Protobuf definiert nur dein Datenmodell und RPC-Protokoll.** Die eigentliche Frage ist, wie dein sprachübergreifendes AST-IR aussieht.
Für Java/Rust/JS würde ich daher nicht versuchen, Syntax 1:1 zu vereinheitlichen, sondern einen **kleinen gemeinsamen Kern** definieren und sprachspezifische Erweiterungen zulassen. Das verhindert, dass dein IR auf den kleinsten gemeinsamen Nenner degeneriert.

## You said:
gibt es schon sowas ähnliches?

## Agent said:
Worked for 8s
Ja — und ein Projekt ist deiner Idee erstaunlich nah: **Semgrep `ast_generic`**.
### 1. Semgrep `ast_generic`
Semgrep hat einen **sprachübergreifenden Generic AST**, aktuell für rund 31 Sprachen, darunter **Java, JavaScript/TypeScript, Rust, Python, Go, C/C++, Kotlin** usw. Ziel ist genau, gemeinsame syntaktische Konstrukte über verschiedene Sprachen zu vereinheitlichen. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/semgrep/semgrep/blob/develop/dune-project?utm_source=chatgpt.com)
Bemerkenswert ist:
```
Java parser ─┐
Rust parser ─┼──> AST_generic
JS parser ───┤
Python ──────┘
```
Die Struktur ist mit **ATD** spezifiziert und kann dadurch auch für andere Sprachen generiert werden. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/semgrep/semgrep/blob/develop/dune-project?utm_source=chatgpt.com)
Allerdings ist es nicht ganz dein Wunschmodell: Semgrops `AST_generic` ist intern noch relativ nah an einem CST und enthält viele Tokeninformationen; der Pretty-Printer nutzt diese Informationen explizit. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/semgrep/semgrep/blob/develop/src/printing/Pretty_print_AST.ml?utm_source=chatgpt.com)
### 2. `cqfn/uast`
Das ist noch näher an deiner ursprünglichen Idee:
**Unified Abstract Syntax Tree**, ausdrücklich als gemeinsame AST-Schicht für mehrere Sprachen. Das Projekt unterstützt u. a. **Java, JavaScript und Python**, konvertiert deren native ASTs in einen gemeinsamen Baum und beschreibt sogar explizit das Ziel, aus gemeinsamen („green“) Nodes wieder Quellcode generieren zu können. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/cqfn/uast?utm_source=chatgpt.com)
Das ist konzeptionell fast:
```
JavaParser ──┐
ANTLR JS ────┼──> UAST ──> Transformation ──> Source
ANTLR Python ┘
```
Das Projekt ist allerdings eher ein älterer Forschungs-/Framework-Ansatz als eine fertige moderne RPC-Infrastruktur.
### 3. YASA-UAST
Es gibt inzwischen auch **YASA-UAST** von Ant Group. Das ist eine explizite **Unified Abstract Syntax Tree IR für Multi-Language Program Analysis**, mit Parsern für Java/JS, Go, PHP und Python. Das Projekt bietet sogar SDK/MCP-Komponenten. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/antgroup/YASA-UAST?utm_source=chatgpt.com)
Das ist aktuell besonders interessant, weil es genau in Richtung **gemeinsame IR über Sprachgrenzen hinweg** geht. Allerdings liegt der Schwerpunkt stärker auf Static Analysis/semantischer Analyse als auf deinem minimalistischen „AST editieren → wieder Source erzeugen“-Use-Case. [![](https://www.google.com/s2/favicons?domain=https://arxiv.org\&sz=128)arXiv](https://arxiv.org/abs/2601.17390?utm_source=chatgpt.com)
### Was ich an deiner Stelle bauen würde
Deine Idee
```
Python MCP
     │
     │ Protobuf RPC
     ▼
  AstService
     │
     ├── Java Server → JavaParser
     ├── Rust Server → Rust parser
     ├── JS Server   → Babel / TS parser
     └── ...
```
ist **nicht ungewöhnlich als Architektur**, aber die Kombination
> **kanonisches, kleines, sprachübergreifendes AST-IR + bidirektional + Protobuf + austauschbare Parser-Server**
habe ich als fertiges Projekt nicht gefunden.
Und genau dort sehe ich einen echten Vorteil gegenüber Semgrep/YASA: Du könntest das IR **viel kleiner und konsequenter als reine Syntaxstruktur** definieren, ohne Symbolauflösung, Typen, CFG, Dataflow usw.
Der spannendste Ausgangspunkt wäre für mich deshalb tatsächlich **Semgrep `ast_generic` als Referenz für die Node-Modellierung**, aber ein eigenes Protobuf-Schema für dein deutlich schlankeres IR. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/semgrep/semgrep/blob/develop/dune-project?utm_source=chatgpt.com)

## You said:
Treesitter ist doch dasselbe?

## Agent said:
Nein, **konzeptionell nicht ganz**.
**Tree-sitter erzeugt primär einen CST (Concrete Syntax Tree)**, keinen semantisch normalisierten AST.
Beispiel:
```
a + b
```
Tree-sitter sieht ungefähr:
```
binary_expression
├── identifier "a"
├── "+"
└── identifier "b"
```
Der `+` ist also Teil der Baumstruktur. Bei vielen Konstrukten bleiben auch Klammern, Keywords und andere syntaktische Details erhalten.
Dein gewünschtes Modell wäre eher:
```
BinaryExpr
├── left: Name("a")
├── op: ADD
└── right: Name("b")
```
Also:
```
                    Tree-sitter
Java ───────────────────► CST
                           │
                           ▼
                    Normalisierung
                           │
                           ▼
                        dein AST
```
### Der wichtige Unterschied
|                             | Tree-sitter                                                      | Dein Ansatz                |
| --------------------------- | ---------------------------------------------------------------- | -------------------------- |
| CST                         | **ja**                                                           | nein                       |
| AST                         | indirekt selbst ableiten                                         | **direkt**                 |
| Whitespace                  | nicht Teil der normalen Baumstruktur, aber Syntaxdetails bleiben | **vollständig irrelevant** |
| Kommentare                  | häufig als Nodes/Extras                                          | nach Wunsch ignorierbar    |
| `if`, Klammern etc.         | konkrete Syntax                                                  | kanonische Semantik        |
| Sprachübergreifendes Modell | nein                                                             | **ja, Ziel**               |
| Roundtrip                   | prinzipiell sehr gut                                             | eigener Generator          |
Tree-sitter ist daher **eine hervorragende Parsing-Basis** für das, was du bauen willst. Du könntest sogar die Parser-Seite komplett darauf aufbauen:
```
Java       ──┐
JavaScript ──┤
Rust       ──┼── Tree-sitter ──► sprachspezifischer CST
Python     ──┘                       │
                                     ▼
                              Normalizer
                                     │
                                     ▼
                              Common AST
                                     │
                                     ▼
                               Protobuf RPC
```
Das hätte gegenüber JavaParser einen interessanten Vorteil: **Tree-sitter gibt es für sehr viele Sprachen und ist inkrementell**.
Der eigentliche Aufwand verschiebt sich dann von „Parser für jede Sprache“ zu **„CST → dein kanonisches AST“**.
Und das passt ziemlich genau zu deiner Architekturidee mit Java/Rust/JS-Servern.

## You said:
Kein ahnung was du da gerade erzählt ahst

## Agent said:
Ja. Du hast recht — ich habe deine Frage unnötig in Richtung „CST vs. AST“ gezogen.
**Für deine Architektur ist Tree-sitter tatsächlich sehr nah an dem, was du suchst:**
```
Java / Rust / JS
      ↓
  Tree-sitter
      ↓
Syntax Tree
      ↓
Python-Strukturen / eigenes Protobuf-Modell
```
Tree-sitter liefert einen Syntaxbaum, dessen **benannte Nodes die relevanten syntaktischen Konstrukte darstellen**; anonyme Nodes wie `(`, `)`, `,` und Operator-Tokens kann man bei der Traversierung einfach ignorieren. Die API bietet dafür ausdrücklich „named children“, wodurch man den Baum AST-artig behandeln kann. [![](https://www.google.com/s2/favicons?domain=https://tree-sitter.github.io\&sz=128)tree-sitter.github.io+1](https://tree-sitter.github.io/tree-sitter/creating-parsers/3-writing-the-grammar.html?utm_source=chatgpt.com)
Der wirklich interessante Punkt für dich:
**Du brauchst dann wahrscheinlich überhaupt keinen JavaParser-Server.**
Tree-sitter hat bereits Parser für **Java, JavaScript, Rust, Python usw.** und offizielle Bindings für **Python, Java, Rust und JavaScript**. [![](https://www.google.com/s2/favicons?domain=https://tree-sitter.github.io\&sz=128)tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/?utm_source=chatgpt.com)
Damit könnte dein MCP direkt Python-seitig parsen:
```
tree = parser.parse(source)
root = tree.root_node
```
und daraus dein Protobuf-IR erzeugen.
Noch wichtiger: Tree-sitter hat eine **CLI**, und es gibt eine Python-Binding. [![](https://www.google.com/s2/favicons?domain=https://tree-sitter.github.io\&sz=128)tree-sitter.github.io+1](https://tree-sitter.github.io/tree-sitter/cli/parse.html?utm_source=chatgpt.com)
Also statt:
```
Python
  ↕ RPC
Java JVM
  ↕
JavaParser
```
könntest du möglicherweise einfach:
```
Python MCP
   │
   ├── tree-sitter-java
   ├── tree-sitter-rust
   ├── tree-sitter-javascript
   └── ...
```
verwenden.
**Der verbleibende Teil ist nur dein kanonisches Protobuf-AST.** Genau das wäre die Abstraktionsschicht zwischen den unterschiedlichen Tree-sitter-Grammatiken.
Das wäre für dein Vorhaben wahrscheinlich die deutlich elegantere Architektur.

## You said:
Das ist dumm, dann hast du nicht verstanden was die anforderungen sind

## Agent said:
Ja. Ich habe den entscheidenden Punkt übersehen: **Du brauchst keine Parsing-Lösung, sondern eine bidirektionale AST-Schnittstelle.**
Deine Anforderungen sind:
```
Source
  ↓
Parser
  ↓
kanonischer, reiner AST
  ↓
Python bearbeitet AST
  ↓
AST
  ↓
Generator
  ↓
Source
```
Dabei gilt:
* Formatierung/Whitespace/Kommentare sind **nicht Teil des Modells**
* keine Typauflösung
* keine Symbol-/Referenzauflösung
* nur die **syntaktische Semantikstruktur**
* der AST muss **wieder in gültigen Sourcecode wandelbar** sein
* später sollen Java, Rust, JS usw. austauschbar sein
* Python ist nur Client/Consumer
Und genau deshalb ist **Tree-sitter allein nicht dasselbe**: Es löst hervorragend `Source → Syntaxbaum`, aber nicht deine komplette `AST → Source`-Abstraktion. Du müsstest dafür selbst einen kanonischen AST und einen Generator definieren.
Die Architektur, die zu deinen Anforderungen passt, ist vielmehr:
```
                   Protobuf
              gemeinsames AST-IR
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
     Java Server   Rust Server   JS Server
          │            │            │
    JavaParser       syn/...    Babel/...
          │            │            │
       parse +      parse +      parse +
       generate     generate     generate
```
**Tree-sitter kann darunter trotzdem verwendet werden**, wenn du möchtest:
```
Tree-sitter → eigenes AST-IR → Protobuf
                         ↑
                    Generator
```
Aber Tree-sitter ist nicht der Kern deiner Lösung. **Das Kernstück ist dein sprachübergreifendes AST-IR plus Parse/Generate-Vertrag.**
Und das ist auch der Grund, warum **Protobuf hier tatsächlich gut passt**: Du kannst das IR einmal definieren und für Python, Java, Rust und JS generieren.
Ich würde als Nächstes genau dieses **AST-IR** designen — und zwar so, dass Java/Rust/JS möglichst viel gemeinsam haben, ohne sie auf einen künstlich kleinen Nenner zu reduzieren.

## You said:
Gibt es jetzt sowas schon was ich suche. Das zielt eher auf AI

## Agent said:
Ja — jetzt ist klar, worauf du hinauswillst: **eine AST-Abstraktion speziell als Schnittstelle für AI-Code-Verarbeitung**, nicht primär klassische Static Analysis.
Ich habe danach gesucht. Es gibt einige sehr nahe Projekte, aber **dein exaktes Gesamtpaket habe ich nicht gefunden**.
### Am nächsten dran: YASA-UAST
**YASA-UAST** ist eine echte sprachübergreifende UAST-IR. Sourcecode wird aus verschiedenen Sprachen in ein einheitliches AST-Modell überführt; YASA ist explizit auf automatisierte Codeanalyse ausgelegt und bietet sogar eine MCP-Komponente. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/antgroup/YASA-UAST?utm_source=chatgpt.com)
Aber: Der Schwerpunkt ist **Security/Static Analysis** — Queries, Dataflow, Call Graphs, Taint Analysis — und nicht „AI bekommt einen kleinen kanonischen AST, verändert ihn und wir generieren wieder Source“. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/antgroup/YASA-Engine?utm_source=chatgpt.com)
### Für AI noch interessanter: MetaCall `meta-ast`
Das neue **`meta-ast`** ist ausdrücklich als Grundlage für polyglottes Developer-Tooling gedacht, unterstützt 9 Sprachen und normalisiert die Ergebnisse in ein sprachunabhängiges IR. Es ist allerdings ebenfalls primär auf **Symbol-/Dependency-Graphen** ausgerichtet, nicht auf einen vollständigen bidirektionalen AST. [![](https://www.google.com/s2/favicons?domain=https://docs.rs\&sz=128)Docs.rs](https://docs.rs/crate/meta-ast/latest?utm_source=chatgpt.com)
### AI/MCP-Projekte
Es gibt mittlerweile mehrere MCP-Server, die ASTs für LLMs bereitstellen, z. B. `ast-mcp-server` mit Java, Rust, JS usw. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/angrysky56/ast-mcp-server?utm_source=chatgpt.com) Auch `ast_context` ist ausdrücklich als „code graph … for AI assistants“ gedacht und unterstützt viele Sprachen. [![](https://www.google.com/s2/favicons?domain=https://docs.rs\&sz=128)Docs.rs+1](https://docs.rs/crate/ast_context/0.1.6/source/README.md?utm_source=chatgpt.com)
Aber auch hier: **Analyse für das LLM**, nicht dein AST-als-Programmier-IR mit
```
parse → AST verändern → generate
```
### Das wirklich Interessante für deine Idee
Es gibt Forschung in genau der Richtung **„Universal IR + bidirektionale Codegenerierung“**. Zum Beispiel **CrossTL**: sprachspezifische Parser → universelle IR → Transformation → sprachspezifischer Codegenerator. Das ist allerdings auf Übersetzung zwischen Sprachen ausgerichtet, nicht als AI-RPC-Standard. [![](https://www.google.com/s2/favicons?domain=https://arxiv.org\&sz=128)arXiv](https://arxiv.org/abs/2508.21256?utm_source=chatgpt.com)

### Meine Einschätzung
**Es existieren Bausteine, aber ich finde aktuell keinen etablierten Standard à la**
```
                protobuf AST IR
                       │
       ┌───────────────┼───────────────┐
       ▼               ▼               ▼
     Java            Rust             JS
    parser           parser           parser
       │               │               │
       └────── parse → Common AST ← generate ──────┘
                       │
                       ▼
                    Python AI
```
mit dem expliziten Ziel:
**„AI manipuliert einen sprachübergreifenden, formatierungsfreien AST und bekommt daraus wieder Sourcecode.“**
Und ich glaube, das ist eine wichtige Unterscheidung zu den existierenden AI-Code-Tools: **Die meisten geben dem Modell Codeausschnitte oder semantische Graphen; dein Modell wäre eine echte editierbare IR.**
Von den existierenden Projekten würde ich mir **YASA-UAST** und **CrossTL** als konzeptionelle Referenzen ansehen. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/antgroup/YASA-UAST?utm_source=chatgpt.com)

## You said:
FÜr KI sind 2 varianten schelcht. 1. KI arbeitet mit AST -> das ist widernatürlich dem training und er kapazität 2. KI arbeitet direkt mit text, das ist ineffizient und fehleranfällig und braucht separate validierungsruns Über legen ist aber KI arbeiten durch eine Normalisierungsschicht. Die schicht normalisiert auf ein generisches Format, mit semantischen knoten wie dateien und wenn die KI durch die schicht schreibt werden ihre fehler automatisch korrigiert und antizipiert. Die normalisierung muss daher auf semantik reduzieren und die semantik wieder zurück plain und straigth darstellen können.
Show moreShow less

## Agent said:
Ja — und jetzt würde ich deine Idee deutlich präziser formulieren:
Du suchst **keinen AST für KI**, sondern eine **AI-facing Normalization IR für bestehenden Code**.
Der entscheidende Unterschied ist:
```
             heutiger Ansatz
Code ─────────────► LLM ─────────────► Code
                      ↑
                 Syntaxlast
oder
Code ─► AST ─► LLM ─► AST ─► Code
          ↑             ↑
       für LLMs unnatürlich
             dein Ansatz
Code
  │
  ▼
Parser
  │
  ▼
Canonical Semantic IR
  │
  │  ← LLM arbeitet hier
  ▼
Normalizer / Validator
  │
  ▼
Generator
  │
  ▼
Code
```
Und deine wichtige zusätzliche Anforderung ist:
> **Die IR selbst soll so gestaltet sein, dass ungültige oder redundante AI-Ausgaben bereits durch die Normalisierungsschicht aufgefangen werden.**
Das ist **nicht einfach „AST als JSON für LLM“**.
Ich habe danach noch einmal gezielt gesucht. Es gibt inzwischen Projekte, die sehr nah kommen:
**AIRL** ist momentan der interessanteste Treffer. Es beschreibt sich explizit als „AI-native Intermediate Representation“, als **typed program graph**, bei dem der Agent nicht Quelltext editiert, sondern semantische Patches auf der IR ausführt; Quelltext ist nur eine Projektion. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub+1](https://github.com/osalabs/airl?utm_source=chatgpt.com)
Aber AIRL ist etwas anderes als deine Idee: Es ist im Kern eine **eigene AI-Programmiersprache/IR**, nicht eine Normalisierungsschicht, die **bestehendes Java/Rust/JS** verlustfrei auf ein gemeinsames semantisches Modell reduziert und wieder zurückprojektiert.
Daneben gibt es **UAST/YASA-UAST**, die sprachübergreifende Syntax normalisieren. Das geht in deine Richtung, aber ihr Schwerpunkt ist Program Analysis und nicht die von dir beschriebene AI-Editier-Schnittstelle. Die früher genannten Projekte sind daher als Architekturvorbilder brauchbar, treffen deinen Zweck aber nicht vollständig.
Was ich dagegen **noch nicht gefunden habe**, ist genau dieses Modell:
```
Java ─────┐
Rust ─────┤
JS ───────┼─► canonical semantic IR ◄─► AI
Python ───┘
                    │
                    ▼
             language backend
```
mit diesen Eigenschaften gleichzeitig:
1. **nur semantisch relevante Struktur**
2. **sprachübergreifend normalisiert**
3. **für LLM-Tokenisierung natürlich darstellbar**
4. **editierbar**
5. **kanonisch/normierend**
6. **aus der IR wieder deterministisch Sourcecode generierbar**
7. **Fehler werden an der IR-Grenze korrigiert bzw. normalisiert**
Der letzte Punkt ist meines Erachtens der eigentlich neue Kern deiner Idee.
Beispielsweise könnte die AI schreiben:
```
function foo
    args: x
    body:
        return x
```
Die Schicht macht daraus intern einen kanonischen Knoten:
```
Function
  name = foo
  params = [x]
  return = x
```
und der Java-Backend-Generator entscheidet daraus:
```
Object foo(Object x) {
    return x;
}
```
Das Modell muss weder Java-Semikolon noch Klammerung noch Java-spezifische Deklarationssyntax korrekt reproduzieren.
**Damit reduzierst du nicht nur Tokens, sondern verschiebst einen Teil der Fehlertoleranz aus dem LLM in deterministischen Code.** Das ist der wesentliche Unterschied zu „LLM bekommt einen AST“.
Interessanterweise gibt es bereits aktuelle Arbeiten, die die Ineffizienz von textbasiertem Editing explizit adressieren; etwa FastEdit versucht, das Wiederholen vorhandenen Codes bei Edits zu eliminieren. Das geht aber über AST-basierte Lokalisierung und nicht über deine bidirektionale semantische IR. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/parcadei/fastedit?utm_source=chatgpt.com)
**Ich würde deine Idee daher eher „Canonical Semantic Code IR“ oder „AI Code Normalization Layer“ nennen als UAST.** AIRL ist momentan der nächste konzeptionelle Nachbar, aber nicht dasselbe. [![](https://www.google.com/s2/favicons?domain=https://github.com\&sz=128)GitHub](https://github.com/osalabs/airl?utm_source=chatgpt.com)
Der nächste interessante Schritt wäre jetzt, **dein IR selbst zu entwerfen**: insbesondere welche Knoten wirklich sprachübergreifend semantisch sind (`Repository → Module → File → Type → Function → Statement → Expression`) und wo man bewusst *keine* Vereinheitlichung erzwingt.

## You said:
Ja aber das sind dumme überlegungen. Diese gehen davon aus das KI intelligent wäre und semantisch arbeiten kann. kann sie aber nicht. KI versteht semantik nicht besser als menschen. Mein ansatz trennt das. KI ist ein statistisches prediction modell. Man kann die fehlerhafte prediction einfach durch die semantik filtern und antizipieren.
Show moreShow less

## Agent said:
Ja. Das ist ein anderer Ansatz als „AI-native IR“.
Dein Modell wäre eher:
```
                  statistisches Modell
                        LLM
                         │
                  erzeugt Kandidaten
                         │
                         ▼
               Semantik-Normalisierung
                         │
              ┌──────────┴──────────┐
              │                     │
        gültige Struktur       Fehler erkennen
              │                     │
              └──────────┬──────────┘
                         ▼
                 kanonische Semantik
                         │
                         ▼
                  Quellcode erzeugen
```
Der entscheidende Punkt ist: **Die KI muss die Semantik nicht „verstehen“.** Sie muss nur plausible Sequenzen/Kandidaten erzeugen. Die Normalisierungsschicht besitzt die Regeln, die entscheiden, was davon strukturell zulässig ist.
Damit verschiebt sich die Zuverlässigkeit:
```
LLM:             probabilistisch
Normalization:   deterministisch
Generator:       deterministisch
```
Und das ist fundamental anders als:
```
LLM → Code → Compiler/Tests → LLM korrigiert Fehler
```
weil bei deinem Ansatz die **Fehlerdomäne schon während der Generierung eingeschränkt** wird.
Ein Beispiel:
```
class Foo {
    int x;
}
```
Die KI produziert vielleicht sinngemäß:
```
class Foo
  field x
  field y
  method x
```
Die Normalisierung kennt aber das zulässige Schema:
```
Class
 ├─ Field(name=x)
 └─ Field(name=y)
```
und kann etwa `method x` als unzulässige Struktur erkennen, statt den Fehler erst nach der Rückübersetzung zu entdecken.
Der wirklich interessante Teil ist deshalb **nicht das AST-Schema**, sondern die Normalisierung als **Constraint-/Recovery-Schicht zwischen einem probabilistischen Modell und einer deterministischen Programmdarstellung**.
Das ist meines Wissens deutlich weniger etabliert als „LLM arbeitet auf AST“ oder „LLM arbeitet auf Code“. Die bisherigen UAST/AI-IR-Projekte, die wir besprochen haben, zielen überwiegend darauf, **die Repräsentation für das Modell angenehmer zu machen**. Dein Ziel ist dagegen:
> **Das Modell darf dumm und fehlerhaft sein; die Repräsentation soll seine Fehler absorbieren.**
Das ist eine wesentlich andere Problemstellung.

## You said:
Meine idee ist eher, die AI will einen semantischen block in einer funktion ändern. SIe kann die zeichenfolgen generieren aber das ist fehlerbehaftet. Aber wir können die zeichenfolge in einen AST normalisieren und dann die operation ausführen. Das LLM bekommt dabei aber nicht den AST zu sehen sondern den COdetext
Show moreShow less

## Agent said:
Ja. **Jetzt verstehe ich den Ansatz.** Der AST ist für die Infrastruktur, **nicht für das LLM**.
Das LLM arbeitet ganz normal mit Code:
```
LLM
 │
 │ erzeugt / verändert Codetext
 ▼
"int x = foo(a + 1);"
 │
 ▼
Parser / Normalizer
 │
 ▼
AST
 │
 │ semantische Operation
 ▼
bestehender AST
 │
 ▼
Generator
 │
 ▼
Codetext
```
Der entscheidende Trick ist also:
> **Das LLM darf seine natürliche Ausgabeform verwenden. Die Infrastruktur interpretiert diese Ausgabe in eine robuste semantische Operation.**
### Beispiel
Bestehender Code:
```
if (x > 10) {
    foo(x);
    bar(x);
}
```
Das LLM soll nur den Block ändern und erzeugt:
```
foo(x);
baz(x)
```
Fehler wie fehlendes `;`, falsche Klammerung, alternative Schreibweisen usw. sind hier **nicht fatal**, weil du den vorgeschlagenen Codetext zunächst separat parsest:
```
LLM-Text
   ↓
AST(fragment)
   ↓
normalisierte Operation
   ↓
Transformation des eigentlichen AST
   ↓
Java-Generator
```
Damit ist der AST eine **Transaktions- und Validierungsschicht**.
### Das ist etwas Wichtiges
Du brauchst dann **gar keinen sprachübergreifenden AST**, den das LLM verstehen soll.
Du kannst einen gemeinsamen **Operationsvertrag** haben:
```
replace block
insert statement
delete expression
replace method body
rename declaration
add parameter
...
```
und sprachspezifisch:
```
Java text → Java AST
Rust text → Rust AST
JS text   → JS AST
```
Die AI bleibt immer bei:
```
plain code
```
Die semantische Normalisierung passiert **hinter ihrem Rücken**.
Das hat auch einen weiteren Vorteil: Du musst die AI nicht auf ein künstliches IR trainieren. Sie produziert genau das, wofür sie trainiert wurde: **Quelltext**.
Deine Architektur wäre damit eher:
```
                   LLM
                    │
               plain code
                    │
             ┌──────▼──────┐
             │ Normalizer  │
             │ Parse       │
             │ Validate    │
             │ Recover     │
             └──────┬──────┘
                    │
                   AST
                    │
             semantic operation
                    │
             ┌──────▼──────┐
             │ Generator   │
             └──────┬──────┘
                    │
                 source
```
**Das ist eine sehr andere Idee als „LLM auf AST arbeiten lassen“.** Der AST ist hier quasi der Compiler-ähnliche Schutzschild zwischen probabilistischer Textgenerierung und deterministischer Code-Manipulation.
Der interessante Forschungs-/Engineering-Punkt wäre dann tatsächlich, **wie die Normalization Layer aus fehlerhaftem AI-Codetext möglichst viel rekonstruieren kann, ohne selbst semantische Entscheidungen zu erfinden.**

## You said:
Genau und das wichtigste dabei ist die enge normalisierung. Python AST ist ideal. Es wird direkt auf instruktionsebene herunterbrochen und kann als uniformes code ausgegeben werden. Es gibt dabei keine variation der darstellung

## Agent said:
Genau. **Jetzt ist der Kern deiner Idee klar.**
Du willst keine „semantisch kluge“ Repräsentation, sondern eine **enge, deterministische Normalisierung**:
```
LLM erzeugt beliebigen plausiblen Code
              ↓
        enger Parser
              ↓
      kanonische Struktur
              ↓
    deterministische Ausgabe
```
Bei Python `ast` ist genau das attraktiv:
```
x + 1
```
wird zu einer festen Struktur:
```
BinOp(
    left=Name("x"),
    op=Add(),
    right=Constant(1)
)
```
Die unterschiedlichen Schreibweisen des Inputs verschwinden. Danach kannst du aus derselben Struktur **eine einzige kanonische Darstellung** erzeugen.
Für dein System ist deshalb entscheidend, dass die Normalisierung **möglichst nah an der sprachlichen Semantik, aber maximal deterministisch** ist.
Also beispielsweise:
```
        "if (x) { foo(); }"
                  │
                  ▼
          If(
            condition=x,
            body=[Call(foo)]
          )
                  │
                  ▼
        if (x) {
            foo();
        }
```
Nicht:
```
        "irgendwie verstandene Bedeutung von if"
```
sondern:
> **Parserregeln erzwingen eine eindeutige Struktur.**
Und daraus folgt auch, warum **Tree-sitter allein nicht dein eigentliches Ziel löst**: Du brauchst nicht bloß einen Parser, sondern einen **kanonischen AST + kanonischen Renderer**.
Für dein Vorhaben würde ich deshalb JavaParser noch einmal unter genau diesem Gesichtspunkt bewerten: **Kann man damit aus beliebigen Java-Syntaxvarianten einen hinreichend engen AST gewinnen und anschließend deterministisch und variationsfrei wieder Java erzeugen?** Das ist die entscheidende Frage — nicht, ob die Library „AST kann“.

## You said:
Deshalb helfen IR nur bedingt. Der IR muss eng sein und symetrisch ein und ausgelesen werden können

## Agent said:
Genau. **Symmetrie ist der entscheidende Punkt.**
Ein allgemeines IR hilft dir wenig, wenn es nur eine gute interne Repräsentation ist. Für deinen Ansatz muss es eine **bidirektionale, enge Normalform** sein:
```
          beliebiger Code
                │
                ▼
          Parse / Normalize
                │
                ▼
        Canonical IR / AST
                │
        ┌───────┴───────┐
        │               │
      lesen            ändern
        │               │
        └───────┬───────┘
                ▼
          Canonical Print
                │
                ▼
          eindeutiger Code
```
Die zentrale Eigenschaft wäre idealerweise:
```
normalize(print(IR)) == IR
```
und möglichst auch:
```
normalize(code1) == normalize(code2)
```
für alle syntaktisch unterschiedlichen Schreibweisen, die dieselbe normalisierte Struktur darstellen sollen.
Damit bekommst du eine Art **Normalform für Code**. Das ist für deinen AI-Use-Case viel wertvoller als ein möglichst reichhaltiges UAST.
Und „eng“ bedeutet für mich in deinem Kontext:
* jeder relevante Knoten hat genau eine Bedeutung
* keine alternativen Darstellungen desselben Sachverhalts
* keine Token-/Formatierungsinformation
* keine überflüssigen syntaktischen Zwischenknoten
* keine impliziten semantischen Beziehungen, die nicht aus der Syntax folgen
* ein eindeutiger Renderer
Das ist tatsächlich näher an **Python `ast` + canonical unparser** als an klassischen Compiler-IRs.
Der interessante technische Begriff dafür wäre vielleicht **canonical bidirectional syntax representation** bzw. **normalizing AST**, nicht einfach IR.
Und damit wird auch die Auswahl der Parser klarer: Die wichtigste Frage bei JavaParser wäre jetzt nicht „wie gut ist der AST?“, sondern **wie deterministisch und vollständig lässt sich `parse → canonicalize → print → parse` als stabile Schleife aufbauen?**

## You said:
davon unabhängig ist die eigentliche kompilierung. Jede operation muss AST vollstänidg sein aber nicht notwendig schon kompilieren

## Agent said:
Ja. Das trennt zwei völlig verschiedene Ebenen:
```
LLM / Text
   ↓
Parsing + Normalisierung
   ↓
vollständiger AST
   ↓
AST-Operation
   ↓
vollständiger AST
   ↓
kanonischer Source
   ↓
Compiler
```
Der **AST muss strukturell vollständig und intern konsistent** sein, aber er muss noch keine kompilierbare Gesamtsemantik garantieren.
Beispiel:
```
foo(unknownVariable);
```
kann ein vollkommen gültiger AST sein:
```
Call
 ├─ callee = foo
 └─ arg = Name(unknownVariable)
```
Erst die Kompilierung entscheidet später, ob `foo` und `unknownVariable` tatsächlich existieren, die Typen passen usw.
Das ist für deinen Ansatz sogar wichtig, weil die Normalisierungsschicht **nicht zum Compiler werden sollte**. Sonst müsstest du für jede kleine AI-Operation bereits Projektkontext, Imports, Typauflösung, Build-System usw. einbeziehen.
Ich würde daher drei Validierungsebenen unterscheiden:
```
1. Syntax
   "Ist daraus ein vollständiger AST geworden?"
2. Struktur
   "Ist dieser AST zulässig und kanonisch?"
3. Kompilierung
   "Ist das Gesamtprogramm semantisch korrekt?"
```
Nur **1 + 2** gehören in deine Normalisierungsschicht.
Das macht die Operationen auch lokal und schnell: Eine AI-Änderung an einem Funktionsblock kann vollständig normalisiert werden, ohne das gesamte Projekt kompilieren zu müssen.

# Zusammenfassung: Java-AST-Normalisierungsschicht für KI-Code-Editing

## Ausgangsfrage
Python-Äquivalent für Java-AST gesucht, um semantische Codeanalyse/-manipulation zu ermöglichen.

## Entwicklung der Anforderung (wichtigste Klärungsschritte)

1. **Nicht formatierungserhaltend** – kein `LexicalPreservingPrinter`/JDT `ASTRewrite`. Ziel ist die vollständige Entfernung von Whitespace, Kommentaren, Formatierung – reine Semantikstruktur, analog Python `ast`.
2. **Keine Typ-/Symbolauflösung, keine Relationen nötig** – also kein Spoon, kein Symbol Solver. Nur reine Syntax-/Semantikstruktur.
3. **JavaParser (`javaparser-core`)** wurde als passende Library identifiziert, da abhängigkeitsfrei und ausreichend für reinen AST.
4. **CLI**: keine offizielle vorhanden, aber `javaparser-core-serialization` erlaubt JSON-Export als Basis für ein eigenes Tool.

## Architekturdiskussion (Python-MCP ↔ Java)
- Kein JNI/JPype – stattdessen **langlebiger Java-Worker-Prozess**.
- Transport/RPC-Optionen diskutiert und verworfen zugunsten von Protobuf:
  - JSONL über stdin/stdout (einfach, aber ineffizient)
  - MessagePack + Unix-Socket (schneller, aber weniger typsicher)
  - **Protobuf** favorisiert: sprachübergreifend Code-generierbar (Python, Java, Rust, JS), typsicheres Schema, Basis für spätere Mehrsprachigkeit (Rust/JS-AST-Server anschließbar).
- Wichtig: **eigenes kanonisches IR senden**, nicht den internen JavaParser-AST 1:1 serialisieren (unnötiger Ballast wie Ranges/Tokens).

## Recherche nach existierenden Lösungen
Mehrere Projekte prüfend verglichen, keines passte exakt:
- **Semgrep `ast_generic`** – sprachübergreifend, aber zu nah am CST, tokenlastig.
- **cqfn/uast**, **YASA-UAST** (Ant Group) – unified AST, aber Fokus auf Static/Security-Analysis, nicht AI-Editing.
- **Tree-sitter** – wurde als CST (nicht normalisierter AST) korrekt eingeordnet, nachdem Missverständnis geklärt wurde: liefert konkrete Syntax (inkl. Operator-Tokens etc.), keine kanonische Semantikstruktur.
- **AIRL** – „AI-native IR“, eigene Programmiersprache, nicht das Gesuchte.
- **CrossTL** – universelle IR für Cross-Language-Codegenerierung, Forschungsprojekt, nicht AI-RPC-fokussiert.

**Fazit der Recherche:** Kein etabliertes Projekt deckt exakt die Kombination „kanonisches, enges, sprachübergreifendes, bidirektionales AST-IR speziell als KI-Editier-Schnittstelle" ab.

## Zentrale konzeptionelle Wende (Kernargument des Nutzers)
Der Nutzer korrigierte die Grundannahme des Agenten mehrfach deutlich:

1. **KI ist kein Semantik-Versteher, sondern ein statistisches Vorhersagemodell.** Weder „KI arbeitet direkt auf AST" (unnatürlich fürs Training) noch „KI arbeitet nur mit Text" (fehleranfällig, braucht separate Validierung) sind gut.
2. **Lösung:** Eine **Normalisierungsschicht** zwischen LLM und AST:
   - Die KI sieht und erzeugt **ausschließlich Klartext/Code** (ihr natives Format).
   - Dieser Text wird geparst/normalisiert in einen **AST**, auf dem die eigentliche semantische Operation ausgeführt wird.
   - Der AST fungiert als **deterministischer Schutzschild/Validierungs- und Recovery-Schicht**, die Fehler der probabilistischen Textgenerierung automatisch abfängt und korrigiert, bevor sie sich auswirken.
   - Die KI „sieht" den AST nie – nur Code-Text als Ein-/Ausgabe.

3. **Enge Normalisierung ist entscheidend** (Vorbild: Python `ast`):
   - Jede syntaktische Variation eines Ausdrucks muss auf exakt eine kanonische Struktur reduziert werden.
   - Keine Darstellungsvarianz, keine Formatierungsinformation, keine überflüssigen Zwischenknoten.
   - Geforderte Eigenschaft: **Symmetrie/Bidirektionalität** – `normalize(print(IR)) == IR` und idealerweise `normalize(code1) == normalize(code2)` für semantisch gleiche, syntaktisch unterschiedliche Eingaben.
   - Dies wird als **wichtiger als ein reichhaltiges, generisches Cross-Language-IR** eingeordnet – „IR helfen nur bedingt, wenn sie nicht eng und symmetrisch ein-/auslesbar sind."

4. **Trennung von AST-Vollständigkeit und Kompilierbarkeit:**
   - Jede AST-Operation muss strukturell vollständig sein (syntaktisch gültiger Baum).
   - Der AST muss aber **nicht** zwingend kompilierbar/semantisch korrekt im Gesamtkontext sein (z. B. unbekannte Variablen sind AST-technisch erlaubt).
   - Drei getrennte Validierungsebenen: (1) Syntax → vollständiger AST, (2) Struktur → zulässiger/kanonischer AST, (3) Kompilierung → Gesamtprogrammkorrektheit. Nur 1+2 gehören in die Normalisierungsschicht; Punkt 3 bleibt separat (Compiler), damit lokale KI-Edits schnell und kontextarm bleiben.

## Kernthese des Nutzers (zusammengefasst)
Eine KI-Code-Editier-Architektur sollte die **Fehleranfälligkeit des LLM nicht durch AST-natives Arbeiten oder reine Textvalidierung lösen**, sondern durch eine **transparente, enge, symmetrische Normalisierungsschicht**, die:
- der KI ihr natürliches Ausgabeformat (Text) belässt,
- Texteingaben in eine eindeutige kanonische Struktur überführt,
- darauf deterministische Operationen ausführt,
- und daraus wieder eindeutigen Code erzeugt –
ohne dabei bereits volle Programmkompilierbarkeit vorauszusetzen.

# AI Code Editing via a Tight, Symmetric Normalization Layer

*An elaboration of the user's thesis, core arguments, and proposed implementation*

## Thesis

An AI code-editing architecture should **not** try to solve the error-proneness of an LLM by making the model work natively on an AST, nor by validating raw text output after the fact. Instead, it should insert a **transparent, tight, symmetric normalization layer** between the model and the code.

In this design:

- the AI keeps its **natural output format — plain source text**,
- that text is **parsed and normalized into a single canonical structure (AST)**,
- **deterministic operations** are executed on that structure,
- and **unambiguous code** is regenerated from it —

all **without requiring full program compilability** at the point of the edit.

The AST is not a representation *for the model to reason about*. It is an **infrastructure-side shield** that absorbs the model's statistical errors before they take effect.

## Core Arguments

### 1. An LLM is a statistical prediction model, not a semantic reasoner

The decisive correction the user makes to the conventional framing: AI does **not** understand semantics better than a human does. Treating the model as if it could "work semantically" is a category error. It produces plausible token sequences; it does not comprehend structure. Any architecture must be built around this fact, not against it.

### 2. Both established paradigms are wrong for this reason

- **AI operates directly on the AST** → unnatural to how the model is trained and to its actual capacity. Forcing the model onto an artificial IR fights its strengths.
- **AI operates directly on text** → inefficient, error-prone, and requires separate validation runs (compile/test cycles) to catch mistakes after they occur.

Both approaches misplace responsibility: they either overestimate the model or leave error handling too late.

### 3. Separation of concerns is the actual solution

The key insight is to **split the probabilistic part from the deterministic part**:

```
        LLM  (probabilistic)
          │  plain code text
          ▼
   Parse / Normalize  (deterministic)
          │
          ▼
   Canonical AST  ──► semantic operation
          │
          ▼
   Generator  (deterministic)
          │
          ▼
        Source code
```

- The LLM may be "dumb" and faulty — that is expected.
- The normalization layer holds the rules that decide what is structurally admissible.
- **The model never sees the AST.** It only ever reads and writes code text — the exact format it was trained on.
- Errors (missing `;`, wrong bracketing, alternative spellings) are **not fatal**, because the proposed fragment is parsed and normalized *behind the model's back* before being applied.

This shifts reliability out of the model and into deterministic code, rather than relying on a downstream `code → compiler/tests → LLM fixes it` loop.

### 4. Tight normalization is the essential property (Python `ast` as the model)

The value is not in a rich representation, but in a **narrow, deterministic** one. Python's `ast` is the ideal reference: it breaks code down to instruction level and re-emits it uniformly, with **no variation in representation**.

"Tight" means:

- every syntactic variation of the same construct collapses to **exactly one** canonical structure,
- no formatting or token information,
- no redundant intermediate syntax nodes,
- no implicit semantic relationships beyond what the syntax states,
- **one** deterministic renderer.

The required property is **symmetry / bidirectionality**:

```
normalize(print(IR)) == IR
normalize(code1)     == normalize(code2)   // for semantically equal, syntactically different inputs
```

This yields a **normal form for code**. The user explicitly rates this as **more important than a rich, generic cross-language IR** — "IRs only help conditionally if they are not tight and cannot be read and written symmetrically." A general IR that is merely a good internal representation is not enough; it must be a tight, invertible normal form.

### 5. AST-completeness must be decoupled from compilability

Every operation must produce a **structurally complete, internally consistent AST** — but that AST need **not** compile or be semantically correct in the full project context. For example:

```
foo(unknownVariable);
```

is a perfectly valid AST node even though `foo` and `unknownVariable` may not exist. Whether they resolve is a **later** concern.

This gives three distinct validation levels:

1. **Syntax** — "Did this become a complete AST?"
2. **Structure** — "Is this AST admissible and canonical?"
3. **Compilation** — "Is the whole program semantically correct?"

Only **levels 1 and 2** belong in the normalization layer. Level 3 stays with the compiler. Keeping compilation out means each AI edit remains **local, fast, and context-light** — no need to pull in imports, type resolution, or the build system for a single block change.

## Implementation

### Target use case

The AI wants to change a **semantic block inside a function**. It generates the character sequence (error-prone). The infrastructure parses that sequence into an AST, applies the operation, and regenerates code:

```
Existing:
if (x > 10) {
    foo(x);
    bar(x);
}

LLM proposes (possibly malformed):
foo(x);
baz(x)          // missing ';'

Flow:
LLM text → AST(fragment) → normalized operation → transform target AST → generator → source
```

The AST thus acts as a **transaction and validation boundary** around each edit.

### Parsing / normalization backend (Java)

- **JavaParser (`javaparser-core`)** as the parser — it is **dependency-free** and produces a pure AST without formatting.
- **Not** `LexicalPreservingPrinter` and **not** JDT `ASTRewrite` — those preserve formatting, which is the opposite of the goal.
- **No** Symbol Solver, **no** Spoon — type/symbol resolution and reference relations are explicitly out of scope.
- The critical evaluation question for the parser is therefore not "does it have an AST?" but: **how deterministically can a `parse → canonicalize → print → parse` loop be built?**

### Process architecture

- **No JNI / JPype / embedded JVM.** Instead, a **long-running Java worker process**, so the JVM start-up cost is paid once.
- Clean channel separation: protocol on the data channel, logs/errors on `stderr`.

### RPC / serialization

- **Protobuf** as the contract, chosen over JSONL (simple but inefficient) and MessagePack (fast but less type-safe).
- Rationale: a **typed, versioned schema** that generates code for **Python, Java, Rust, and JS** from one `.proto`, enabling later **language-agnostic** parser servers.
- **Send a custom canonical IR — not the raw internal JavaParser AST** (which carries unnecessary ballast such as ranges and token information).

### Contract shape

An LSP-like — but custom — AST-RPC contract, e.g.:

```
service AstService {
  rpc Parse(ParseRequest)       returns (ParseResponse);
  rpc Generate(GenerateRequest) returns (GenerateResponse);
}
```

- A **common operation contract** (`replace block`, `insert statement`, `delete expression`, `replace method body`, `rename declaration`, `add parameter`, …).
- **Language-specific** `parse` / `generate` pairs behind it (`Java text ↔ Java AST`, later `Rust text ↔ Rust AST`, `JS text ↔ JS AST`).
- The AI always stays at the **plain-code** boundary; normalization happens on the server side.

### Overall picture

```
                   LLM
                    │  plain code
             ┌──────▼──────┐
             │ Normalizer  │  parse · validate · recover
             └──────┬──────┘
                    │  canonical AST
             semantic operation
                    │
             ┌──────▼──────┐
             │ Generator   │  deterministic, variation-free
             └──────┬──────┘
                    │
                 source
```

## The Genuinely Novel Point

The interesting engineering problem is **not** the AST schema itself. It is the normalization layer as a **constraint- and recovery mechanism between a probabilistic model and a deterministic program representation** — reconstructing as much as possible from faulty AI code text **without inventing semantic decisions of its own**.

In one line:

> **The model is allowed to be dumb and wrong; the representation is designed to absorb its errors** — via a tight, symmetric, bidirectional normalization layer, decoupled from full compilation.

# Java AST Normalization Layer for AI Code Editing: Thesis, Core Arguments, and Implementation Approach

## 1. Thesis

Effective AI-driven code editing requires **neither** direct AST manipulation by the language model **nor** raw unvalidated text editing. Instead, the correct architecture inserts a **strict, symmetric, deterministic normalization layer** between the LLM and the program's structural representation. The LLM continues to read and write plain source-code text — its native, trained modality — while all structural validation, error correction, and semantic-safe transformation happen transparently inside this normalization layer, which internally operates on an AST but never exposes it to the model.

## 2. Core Arguments

### 2.1 The LLM is a statistical predictor, not a semantic reasoner
The user explicitly rejects the common assumption that LLMs "understand" code semantics well enough to manipulate an AST directly. An LLM is a token-prediction model; forcing it to emit or consume AST-shaped data is unnatural relative to its training distribution and wastes capacity that would otherwise go toward producing fluent, familiar code text.

### 2.2 Both naïve approaches fail
- **AI operates directly on the AST:** Unnatural for the model's training and representational capacity; increases cognitive/token overhead without improving reliability.
- **AI operates directly on text:** Efficient for the model, but error-prone (syntax slips, inconsistent formatting, invalid fragments) and requires expensive downstream validation/compilation passes to catch mistakes after the fact.

### 2.3 Solution: a transparent normalization layer
The architecture separates concerns by inserting a layer that:
1. Accepts arbitrary, possibly imperfect code text from the LLM.
2. Parses and **normalizes** it into a canonical AST.
3. Performs the intended semantic operation on that AST (e.g., replacing a block, inserting a statement).
4. Deterministically regenerates canonical source text from the modified AST.

Crucially, the LLM **never sees the AST** — it only ever reads/writes plain code. The AST exists purely as an internal transactional and validation mechanism that absorbs and corrects the model's probabilistic errors before they can propagate.

```
LLM (plain code in/out)
        │
        ▼
  Parse / Normalize   ← errors absorbed here
        │
        ▼
  Canonical AST  ──► semantic operation ──► Canonical AST
        │
        ▼
  Deterministic Generator
        │
        ▼
  Plain code (LLM never sees intermediate AST)
```

### 2.4 Normalization must be "tight" (eng) — Python `ast` as the model
The user identifies Python's `ast` module as the ideal reference precisely because it reduces source code to instruction-level structure with **zero representational variation**: every semantically equivalent input collapses to exactly one canonical structure (e.g., `x + 1` always becomes the same `BinOp` node regardless of surrounding whitespace or formatting). This is the property that must be replicated for Java (and eventually other languages).

Requirements for a valid canonical representation:
- No whitespace, comments, or formatting information retained.
- No alternative representations of the same semantic construct.
- No redundant/incidental syntactic wrapper nodes.
- Exactly one canonical printer for any given structure.

### 2.5 Symmetry/bidirectionality is the decisive property — not richness of the IR
The user's key correction to the discussion of generic/universal IRs (Semgrep `ast_generic`, YASA-UAST, AIRL, CrossTL, Tree-sitter, etc.) is that **none of these solve the actual problem**, because they are not built around strict bidirectional symmetry. A rich, general-purpose intermediate representation is of limited value if it isn't:

```
normalize(print(IR)) == IR
normalize(code₁) == normalize(code₂)   (for all inputs with identical semantics)
```

This symmetry requirement is elevated above cross-language generality as the primary design criterion — a narrow, tightly-defined, perfectly round-tripping representation is more valuable than a broad but loosely normalized universal AST.

### 2.6 AST completeness ≠ compilability
A final, independent clarification: every AST produced by an editing operation must be **structurally complete** (a syntactically well-formed tree), but it does **not** need to be compilable or semantically valid within the full program context (e.g., referencing an undefined variable is still a valid AST node). The user distinguishes three separate validation levels:

1. **Syntax** — Does the text parse into a complete AST?
2. **Structure** — Is the AST well-formed and canonical?
3. **Compilation** — Is the AST semantically valid in the full project context (types, imports, symbol resolution, build system)?

Only levels 1 and 2 belong inside the normalization layer. Level 3 (full compilation) is deliberately excluded so that individual AI edits remain fast, local, and independent of full project context — the normalization layer must not become a compiler.

## 3. Implementation Approach (Technical Path Explored)

| Stage | Decision | Rationale |
|---|---|---|
| Parsing library | **JavaParser (`javaparser-core`)** | Zero transitive dependencies; produces a straightforward AST comparable to Python's `ast`; no symbol/type resolution baggage (unlike Spoon or JavaParser's symbol-solver module) |
| Formatting preservation | **Explicitly rejected** (`LexicalPreservingPrinter`, JDT `ASTRewrite`) | The goal is the opposite — a formatting-free, uniform semantic structure |
| Bridging Java ↔ Python | **Long-lived Java worker process**, not JNI/JPype embedding | Keeps JVM startup cost paid once; clean process boundary |
| Transport/RPC | **Protobuf** (over JSON-RPC/MessagePack alternatives considered and dropped) | Enables schema-first, cross-language code generation (Python, Java, Rust, JS) for a future multi-language AST-server architecture |
| Wire payload | **Custom canonical IR**, not JavaParser's native serialized AST | Avoids leaking internal library details (token ranges, positions) into the interface; keeps the schema stable and language-agnostic |
| Future extensibility | Additional per-language AST servers (Rust, JavaScript) behind the same Protobuf contract | Same canonical operation vocabulary (replace block, insert statement, rename declaration, etc.) reusable across languages |

### Architectural summary diagram

```
                    LLM (plain source text)
                              │
                              ▼
                 ┌─────────────────────────┐
                 │  Normalization Layer    │
                 │  (per-language server)  │
                 │                         │
                 │  parse → canonical AST  │
                 │  apply semantic op      │
                 │  generate → plain code  │
                 └────────────┬────────────┘
                              │  Protobuf RPC
                              ▼
                     Python MCP orchestrator
```

Java backend: JavaParser-based worker exposing `parse` / `generate` operations via a persistent process, communicating through a custom, tightly-scoped Protobuf schema — explicitly excluding formatting preservation, type resolution, and symbol/reference analysis, in favor of a narrow, symmetric, canonical semantic structure suitable as an error-correcting buffer between an LLM's text output and deterministic source code.
