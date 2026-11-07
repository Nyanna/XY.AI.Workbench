Implementiere beim Einlesen für `/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java` einen PreProcessor.
Dieser soll alle kompatiblen Blöcke in Statements umwandeln.

Beispiel:
```java
if (ifStmt.getThenStmt() instanceof BlockStmt block 
        && block.getStatements().size() == 1) {
    ifStmt.setThenStmt(block.getStatement(0));
}
```
SystemInit:  id=41adf13e-6b92-4186-b91d-21fbfa663bce, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 8d71-136
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: Überblick über JavaAstEngine.java zur Ermittlung des Einlesepunkts für einen PreProcessor
```
/answer 8d71-136 allow
Control Request:
```yaml
id: 8d71-137
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: Lj9iLm|A5A1hY
        signature: package xy.ai.mcpc.ast.engine;
      - id: lzazYM|5l3RiL
        signature: import com.github.javaparser.ast.body.EnumConstantDeclaration;
      - id: fVjiDW|MrVYLR
        signature: import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
      - id: YftzGA|Li9D6O
        signature: import java.util.stream.Stream;
      - id: bhtE8s|XLe1AK
        signature: /**
      - id: JavaAstEngine
        signature: "public final class JavaAstEngine {"
        children:
        - id: JavaAstEngine.CYgqlK|8W8fvh
          signature: private final JavaParser parser;
        - id: JavaAstEngine.JavaAstEngine
          signature: "public JavaAstEngine() {"
        - id: JavaAstEngine.parseCompilationUnit
          signature: "public CompilationUnit parseCompilationUnit(String source) {"
        - id: JavaAstEngine.emptyCompilationUnit
          signature: "public CompilationUnit emptyCompilationUnit() {"
        - id: JavaAstEngine.print
          signature: "public String print(Node node) {"
        - id: JavaAstEngine.CBTNnm|2Rono2
          signature: /**
        - id: JavaAstEngine.validate
          signature: "public String validate(String source) {"
        - id: JavaAstEngine.gNyqku|z31nVP
          signature: "// -- fragment parsing, one entry point per node category --"
        - id: JavaAstEngine.parseImportFragment
          signature: "public ImportDeclaration parseImportFragment(String code) {"
        - id: JavaAstEngine.parseTypeFragment
          signature: "public TypeDeclaration<?> parseTypeFragment(String code) {"
        - id: JavaAstEngine.parseBodyFragment
          signature: "public BodyDeclaration<?> parseBodyFragment(String code) {"
        - id: JavaAstEngine.parseEnumConstantFragment
          signature: "public EnumConstantDeclaration parseEnumConstantFragment(String code) {"
        - id: JavaAstEngine.egr9Ri|JgFEmQ
          signature: /**
        - id: JavaAstEngine.parseReplacementFor
          signature: "public Node parseReplacementFor(Node sample, String code) {"
        - id: JavaAstEngine.ovqh7Y|d7rpKC
          signature: // -- CU-level append --
        - id: JavaAstEngine.AppendResult
          signature: "public record AppendResult(CompilationUnit cu, int units) {"
        - id: JavaAstEngine.append
          signature: "public AppendResult append(CompilationUnit cu, String code) {"
        - id: JavaAstEngine.tCUKXg|xxdtM3
          signature: // -- mutation on a resolved node --
        - id: JavaAstEngine.replace
          signature: "public void replace(AddressableNode target, String code) {"
        - id: JavaAstEngine.insert
          signature: "public int insert(AddressableNode target, String code, String position) {"
        - id: JavaAstEngine.delete
          signature: "public void delete(AddressableNode target) {"
        - id: JavaAstEngine.jsuf7R|BVBqZw
          signature: // -- signature / docstring --
        - id: JavaAstEngine.signature
          signature: "public String signature(Node node, int limit) {"
        - id: JavaAstEngine.header
          signature: "private static String header(String printed) {"
        - id: JavaAstEngine.firstLine
          signature: "private static String firstLine(String printed) {"
        - id: JavaAstEngine.rERvp0|C68UnQ
          signature: /**
        - id: JavaAstEngine.docstring
          signature: "public String docstring(Node node, int limit) {"
        - id: JavaAstEngine.xNiO2a|xmWpd8
          signature: // -- helpers --
        - id: JavaAstEngine.unwrap
          signature: "private static <T> T unwrap(ParseResult<T> result, String what) {"
        - id: JavaAstEngine.problemsMessage
          signature: "private static String problemsMessage(List<Problem> problems) {"
```
/answer 8d71-137 allow
ReasoningToken: 13
Control Request:
```yaml
id: 8d71-138
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.parseCompilationUnit
    - JavaAstEngine.unwrap
    - JavaAstEngine.CYgqlK|8W8fvh
    - JavaAstEngine.JavaAstEngine
  reason: "Details zur Methode parseCompilationUnit und unwrap ansehen, um PreProcessor beim Einlesen einzubauen"
```
/answer 8d71-138 allow
Control Request:
```yaml
id: 8d71-139
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.parseCompilationUnit
        type: method_declaration
        code: |-
          public CompilationUnit parseCompilationUnit(String source) {
              return unwrap(parser.parse(source), "compilation unit");
            }
      - id: JavaAstEngine.unwrap
        type: method_declaration
        code: |-
          private static <T> T unwrap(ParseResult<T> result, String what) {
              if (result.isSuccessful() && result.getResult().isPresent()) {
                return result.getResult().get();
              }
              throw new AstEngineException(AstEngineException.Kind.SYNTAX, "invalid " + what + ": " + problemsMessage(result.getProblems()), result.getProblems().stream().map(Problem::toString).collect(Collectors.toList()));
            }
      - id: JavaAstEngine.CYgqlK|8W8fvh
        type: statements
        code: private final JavaParser parser;
      - id: JavaAstEngine.JavaAstEngine
        type: constructor_declaration
        code: |-
          public JavaAstEngine() {
              ParserConfiguration cfg = new ParserConfiguration().setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
              cfg.setTabSize(2);
              cfg.setDetectOriginalLineSeparator(false);
              cfg.setIgnoreAnnotationsWhenAttributingComments(true);
              this.parser = new JavaParser(cfg);
            }
```
/answer 8d71-139 allow
Control Request:
```yaml
id: 8d71-13a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - lzazYM|5l3RiL
    - fVjiDW|MrVYLR
    - YftzGA|Li9D6O
    - bhtE8s|XLe1AK
  reason: "Alle Imports der Datei einsehen, um zu prüfen welche javaparser Klassen bereits verfügbar sind"
```
/answer 8d71-13a allow
Control Request:
```yaml
id: 8d71-13b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: lzazYM|5l3RiL
        type: imports
        code: |-
          import com.github.javaparser.ast.body.EnumConstantDeclaration;
          import com.github.javaparser.ast.body.EnumDeclaration;
          import com.github.javaparser.ast.body.TypeDeclaration;
          import com.github.javaparser.ast.comments.Comment;
          import com.github.javaparser.ast.comments.JavadocComment;
          import com.github.javaparser.printer.Printer;
          import com.github.javaparser.printer.DefaultPrettyPrinter;
          import com.github.javaparser.printer.configuration.DefaultConfigurationOption;
      - id: fVjiDW|MrVYLR
        type: imports
        code: |-
          import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
          import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration.ConfigOption;
          import com.github.javaparser.printer.configuration.Indentation;
          import com.github.javaparser.printer.configuration.Indentation.IndentType;
          import com.github.javaparser.printer.configuration.PrinterConfiguration;
          import java.util.Arrays;
          import java.util.List;
          import java.util.Optional;
          import java.util.stream.Collectors;
      - id: YftzGA|Li9D6O
        type: imports
        code: import java.util.stream.Stream;
      - id: bhtE8s|XLe1AK
        type: statements
        code: |-
          /**
           * JavaParser mechanics: parse/print, syntax validation, fragment parsing keyed
           * by the target node's category, and the four mutation primitives. Uses the
           * plain {@code PrettyPrinter} (via {@code Node#toString()}, which is not
           * affected unless {@code LexicalPreservingPrinter.setup} is called — which it
           * never is here) so every write normalises formatting, rather than
           * {@code LexicalPreservingPrinter}.
           */
```
/answer 8d71-13b allow
ReasoningToken: 520
Control Request:
```yaml
id: 8d71-13c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.parseImportFragment
    - JavaAstEngine.parseTypeFragment
    - JavaAstEngine.parseBodyFragment
    - JavaAstEngine.append
    - JavaAstEngine.parseReplacementFor
  reason: "Prüfen, welche weiteren Parse-Einstiegspunkte existieren, um zu entscheiden wo der PreProcessor greifen soll"
```
/answer 8d71-13c allow
Control Request:
```yaml
id: 8d71-13d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.parseImportFragment
        type: method_declaration
        code: |-
          public ImportDeclaration parseImportFragment(String code) {
              String stripped = code.strip();
              return unwrap(parser.parseImport(stripped.endsWith(";") ? stripped : stripped + ";"), "import");
            }
      - id: JavaAstEngine.parseTypeFragment
        type: method_declaration
        code: |-
          public TypeDeclaration<?> parseTypeFragment(String code) {
              return unwrap(parser.parseTypeDeclaration(code), "type declaration");
            }
      - id: JavaAstEngine.parseBodyFragment
        type: method_declaration
        code: |-
          public BodyDeclaration<?> parseBodyFragment(String code) {
              return unwrap(parser.parseBodyDeclaration(code), "member declaration");
            }
      - id: JavaAstEngine.append
        type: method_declaration
        code: |-
          public AppendResult append(CompilationUnit cu, String code) {
              boolean empty = cu.getTypes().isEmpty() && cu.getImports().isEmpty() && cu.getPackageDeclaration().isEmpty();
              if (empty) {
                return new AppendResult(parseCompilationUnit(code), 1);
              }
              String trimmed = code.strip();
              if (trimmed.startsWith("import ")) {
                cu.getImports().add(parseImportFragment(trimmed));
              } else {
                cu.addType(parseTypeFragment(trimmed));
              }
              return new AppendResult(cu, 1);
            }
      - id: JavaAstEngine.parseReplacementFor
        type: method_declaration
        code: |-
          public Node parseReplacementFor(Node sample, String code) {
              if (sample instanceof ImportDeclaration) {
                return parseImportFragment(code);
              }
              if (sample instanceof TypeDeclaration<?>) {
                return parseTypeFragment(code);
              }
              if (sample instanceof EnumConstantDeclaration) {
                return parseEnumConstantFragment(code);
              }
              return parseBodyFragment(code);
            }
```
/answer 8d71-13d allow
ReasoningToken: 2375
Control Request:
```yaml
id: 8d71-13e
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  node_type: imports
  reason: "Alle Import-Blöcke der Datei sehen, um doppelte Imports zu vermeiden"
```
/answer 8d71-13e allow
Control Request:
```yaml
id: 8d71-13f
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: Lj9iLm|A5A1hY
        type: imports
        code: |-
          package xy.ai.mcpc.ast.engine;

          import com.github.javaparser.JavaParser;
          import com.github.javaparser.ParseResult;
          import com.github.javaparser.ParserConfiguration;
          import com.github.javaparser.Problem;
          import com.github.javaparser.ast.CompilationUnit;
          import com.github.javaparser.ast.ImportDeclaration;
          import com.github.javaparser.ast.Node;
          import com.github.javaparser.ast.NodeList;
          import com.github.javaparser.ast.body.BodyDeclaration;
          import com.github.javaparser.ast.body.CallableDeclaration;
      - id: lzazYM|5l3RiL
        type: imports
        code: |-
          import com.github.javaparser.ast.body.EnumConstantDeclaration;
          import com.github.javaparser.ast.body.EnumDeclaration;
          import com.github.javaparser.ast.body.TypeDeclaration;
          import com.github.javaparser.ast.comments.Comment;
          import com.github.javaparser.ast.comments.JavadocComment;
          import com.github.javaparser.printer.Printer;
          import com.github.javaparser.printer.DefaultPrettyPrinter;
          import com.github.javaparser.printer.configuration.DefaultConfigurationOption;
      - id: fVjiDW|MrVYLR
        type: imports
        code: |-
          import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
          import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration.ConfigOption;
          import com.github.javaparser.printer.configuration.Indentation;
          import com.github.javaparser.printer.configuration.Indentation.IndentType;
          import com.github.javaparser.printer.configuration.PrinterConfiguration;
          import java.util.Arrays;
          import java.util.List;
          import java.util.Optional;
          import java.util.stream.Collectors;
      - id: YftzGA|Li9D6O
        type: imports
        code: import java.util.stream.Stream;
```
/answer 8d71-13f allow
ReasoningToken: 657
Text:
 No `stmt` imports yet. Add them and implement the preprocessor.
Control Request:
```yaml
id: 8d71-140
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: lzazYM|5l3RiL
    position: after
    source: |-
      import com.github.javaparser.ast.stmt.BlockStmt;
      import com.github.javaparser.ast.stmt.DoStmt;
      import com.github.javaparser.ast.stmt.ForEachStmt;
      import com.github.javaparser.ast.stmt.ForStmt;
      import com.github.javaparser.ast.stmt.IfStmt;
      import com.github.javaparser.ast.stmt.LabeledStmt;
      import com.github.javaparser.ast.stmt.Statement;
      import com.github.javaparser.ast.stmt.WhileStmt;
  reason: Benötigte javaparser-Statement-Klassen für den Block-zu-Statement PreProcessor importieren
```
/answer 8d71-140 allow
Text:
 Now update `unwrap` to apply the preprocessor and add the preprocessing method.
Control Request:
```yaml
id: 8d71-141
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.unwrap
    old_text: |-
      if (result.isSuccessful() && result.getResult().isPresent()) {
            return result.getResult().get();
          }
    new_text: |-
      if (result.isSuccessful() && result.getResult().isPresent()) {
            T node = result.getResult().get();
            if (node instanceof Node astNode) {
              unwrapSingleStatementBlocks(astNode);
            }
            return node;
          }
  reason: Preprocessing beim Einlesen an zentraler Stelle (unwrap) für alle Parse-Erfolge aktivieren
```
/answer 8d71-141 allow
Control Request:
```yaml
id: 8d71-142
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    id: JavaAstEngine.unwrap
    position: after
    source: |2-

        /**
         * Collapses single-statement {@code BlockStmt} bodies (if/while/do/for/foreach/labeled) into
         * their bare statement, wherever that is syntactically equivalent. Skips an if's then-block when
         * unwrapping would create dangling-else ambiguity (the if has an else, and the sole statement is
         * an else-less if).
         */
        private static void unwrapSingleStatementBlocks(Node root) {
          for (BlockStmt block : root.findAll(BlockStmt.class)) {
            if (block.getStatements().size() != 1) {
              continue;
            }
            Statement single = block.getStatement(0);
            Node parent = block.getParentNode().orElse(null);
            if (parent instanceof IfStmt ifStmt) {
              if (ifStmt.getThenStmt() == block) {
                boolean danglingElse = ifStmt.getElseStmt().isPresent()
                    && single instanceof IfStmt innerIf
                    && innerIf.getElseStmt().isEmpty();
                if (!danglingElse) {
                  ifStmt.setThenStmt(single);
                }
              } else if (ifStmt.getElseStmt().orElse(null) == block) {
                ifStmt.setElseStmt(single);
              }
            } else if (parent instanceof WhileStmt whileStmt && whileStmt.getBody() == block) {
              whileStmt.setBody(single);
            } else if (parent instanceof DoStmt doStmt && doStmt.getBody() == block) {
              doStmt.setBody(single);
            } else if (parent instanceof ForStmt forStmt && forStmt.getBody() == block) {
              forStmt.setBody(single);
            } else if (parent instanceof ForEachStmt forEachStmt && forEachStmt.getBody() == block) {
              forEachStmt.setBody(single);
            } else if (parent instanceof LabeledStmt labeledStmt && labeledStmt.getStatement() == block) {
              labeledStmt.setStatement(single);
            }
          }
        }
  reason: "PreProcessor-Methode einfügen, die kompatible Blöcke in Statements umwandelt"
```
/answer 8d71-142 allow
Control Request:
```yaml
id: 8d71-143
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: "Sicherstellen, dass die Datei nach den Änderungen syntaktisch korrekt kompiliert"
```
/answer 8d71-143 allow
Control Request:
```yaml
id: 8d71-144
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/ast-engines/java
  script: ls; cat pom.xml 2>/dev/null | head -5; cat build.gradle* 2>/dev/null | head -5
  reason: "Build-System der Java-Engine ermitteln, um die Änderung tatsächlich zu kompilieren"
```
/answer 8d71-144 deny ich kompiliere und teste das
Result Stats: id=41adf13e-6b92-4186-b91d-21fbfa663bce, total: 25479, in: 153, out: 6999, reason: 70, read: 130823, write: 18327
Text:
 Nun validieren.
ReasoningToken: 70
Verstanden, ich überlasse Kompilieren/Testen dir.
