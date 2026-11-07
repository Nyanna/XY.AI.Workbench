In `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` muss die Methode `readUntilResult` verbessert werden.

- Auch ohne Exception soll STDERR in der Schleife ausgelesen, dem Output angefügt und zusätzlich gelogged werden
- Nach 30 Sekunden in der schleife ohne jegliche Änderung, soll eine Stuck-Detection die Schleife mit einem Fehler beenden (Line ist für STDOUT/STDERR ununterbrochen Leer)
SystemInit:  id=8ed7b5e8-6862-4ab9-9f56-f8d5e28aa96b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: f447-1
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  name: readUntilResult
  reason: Locate method to improve stderr handling and add stuck-detection
```
/answer f447-1 allow
ReasoningToken: 12
Control Request:
```yaml
id: f447-2
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.readUntilResult
        code: |2
            private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {
              SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", 10000);
              CCResponse resp = new CCResponse(req.id);
              String line;
              while (true) {
                // alternate read sources undtil answer
                try {
                  // wait 300 ms
                  if ((line = session.readLine()) != null) {
                    session.setLastRawLine(line);
                    job.setName(abbreviateForStatus(line));
                    mon.worked(1);
                    jsonParser.parseLine(resp, session, sub, line);
                  }
                } catch (Exception ex) {
                  LOG.error("Error reading line: ", ex);
                  try {
                    while ((line = session.readError()) != null) LOG.error("CLI stderr: " + line);
                  } catch (Exception ex2) {
                    LOG.error("Error reading stderr: ", ex2);
                  }
                  resp.resultText = ex.getMessage();
                }
                if (!resp.isReady())
                  controlClient.checkControlEndpoint(resp, session.getID());
                if (resp.isReady()) {
                  resp.sessionId = session.getID();
                  session.stats.add(resp.stats);
                  session.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;
                  return resp;
                }
                sessionManager.onSessionChanged(session);
              }
            }
```
/answer f447-2 allow
Control Request:
```yaml
id: f447-3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: readError|readLine|resultText|class CCResponse|output
    include:
    - '*.java'
  reason: Understand session read methods and CCResponse fields for stderr append and stuck detection
```
/answer f447-3 allow
Control Request:
```yaml
id: f447-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: TimedLineReader.java
        matches:
        - lineno: 26
          match: "public String readLine() throws IOException {"
      - path: HookParser.java
        matches:
        - lineno: 8
          match: "* blocks an operation, reducing the output to the bare hook message:"
        - lineno: 14
          match: '*   &lt;hook output&gt;'
        - lineno: 20
          match: '*   &lt;hook output&gt;'
        - lineno: 43
          match: '* only the hook output section.'
        - lineno: 50
          match: '* <li>The actual hook output (may span multiple lines)</li>'
        - lineno: 55
          match: "* @return the hook output, or the original {@code text} if the pattern is not"
        - lineno: 86
          match: // Hook output starts on the line after the bracket line
        - lineno: 89
          match: // Find the "Original prompt:" line — hook output ends before it
        - lineno: 99
          match: "// Collect hook output lines, stripping leading/trailing blank lines"
        - lineno: 111
          match: return text; // no hook output found — keep original
      - path: CCSession.java
        matches:
        - lineno: 211
          match: "public String readLine() {"
        - lineno: 212
          match: "return readLine(stdout, \"STDOUT\");"
        - lineno: 215
          match: "public String readError() {"
        - lineno: 216
          match: "return readLine(stderr, \"STDERR\");"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer f447-4 allow
ReasoningToken: 77
Control Request:
```yaml
id: f447-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  regexp: readLine
  reason: See readLine/readError implementation and timeout behavior
```
/answer f447-5 allow
Control Request:
```yaml
id: f447-6
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
      nodes:
      - id: CCSession.readLine
        code: "\tpublic String readLine() {\n\t\treturn readLine(stdout, \"STDOUT\");\n\t}\n"
      - id: CCSession.readError
        code: "\tpublic String readError() {\n\t\treturn readLine(stderr, \"STDERR\");\n\t}\n"
      - id: CCSession.readLine_2
        code: "\tprivate String readLine(TimedLineReader reader, String channel) {\n\t\tif (reader == null)\n\t\t\tthrow new IllegalStateException(\n\t\t\t\t\t\"Cannot read \" + channel + \": the CLI process is not started\" + \" (uuid=\" + uuid + \", processAlive=\"\n\t\t\t\t\t\t\t+ isProcessAlive() + \").\" + \" A prompt must be sent (writeLine) before reading.\");\n\n\t\ttry {\n\t\t\tvar line = reader.readLine();\n\t\t\tmirrorLine(line);\n\t\t\tlastSentAt = Instant.now();\n\n\t\t\treturn line;\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(\"Failed to read \" + channel + \" for session \" + uuid, e);\n\t\t}\n\t}\n"
```
/answer f447-6 allow
Control Request:
```yaml
id: f447-7
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
  name: CCResponse
  reason: Check CCResponse fields for appending stderr output
```
/answer f447-7 allow
Control Request:
```yaml
id: f447-8
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer f447-8 allow
ReasoningToken: 183
Control Request:
```yaml
id: f447-9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: class CCResponse
    include:
    - '*.java'
  reason: Find CCResponse class file
```
/answer f447-9 allow
Control Request:
```yaml
id: f447-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCResponse.java
        matches:
        - lineno: 8
          match: "public class CCResponse implements IModelResponse {"
```
/answer f447-a allow
Control Request:
```yaml
id: f447-b
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
  reason: See fields available for appending stderr output
```
/answer f447-b allow
Control Request:
```yaml
id: f447-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
      nodes:
      - id: GmYxyr|se6MkO
        signature: import java.util.LinkedHashMap;
      - id: CCResponse
        signature: "public class CCResponse implements IModelResponse {"
        children:
        - id: CCResponse.B0vmpT|2GN6aA
          signature: public final String id;
        - id: CCResponse.CCResponse
          signature: public CCResponse(String id)
        - id: CCResponse.CCResponse_2
          signature: "public CCResponse(String id, String resultText)"
        - id: CCResponse.setToolUse
          signature: "public void setToolUse(String resultText, String toolUseId)"
        - id: CCResponse.isReady
          signature: public boolean isReady()
```
/answer f447-c allow
Control Request:
```yaml
id: f447-d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCResponse.java
    ids:
    - CCResponse
  reason: Full view of CCResponse fields to decide where to append stderr
```
/answer f447-d deny resp.resultText wie bereits der Fall
ReasoningToken: 16
Control Request:
```yaml
id: f447-e
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.readUntilResult
    source: |2
        private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressMonitor mon, Job job) throws IOException {
          SubMonitor sub = SubMonitor.convert(mon, "Reading Claude output", 10000);
          CCResponse resp = new CCResponse(req.id);
          String line;
          long lastChangeAt = System.currentTimeMillis();
          while (true) {
            boolean changed = false;
            // alternate read sources undtil answer
            try {
              // wait 300 ms
              if ((line = session.readLine()) != null) {
                session.setLastRawLine(line);
                job.setName(abbreviateForStatus(line));
                mon.worked(1);
                jsonParser.parseLine(resp, session, sub, line);
                changed = true;
              }
            } catch (Exception ex) {
              LOG.error("Error reading line: ", ex);
              resp.resultText = ex.getMessage();
              changed = true;
            }
            try {
              String errLine = session.readError();
              if (errLine != null) {
                LOG.error("CLI stderr: " + errLine);
                resp.resultText = (resp.resultText == null ? "" : resp.resultText + "\n") + errLine;
                changed = true;
              }
            } catch (Exception ex2) {
              LOG.error("Error reading stderr: ", ex2);
            }
            if (changed) {
              lastChangeAt = System.currentTimeMillis();
            } else if (System.currentTimeMillis() - lastChangeAt > 30000) {
              // Stuck detection: neither STDOUT nor STDERR produced anything for 30s
              throw new IOException(
                  "Stuck detection: no STDOUT/STDERR output for 30 seconds (session " + session.getID() + ")");
            }
            if (!resp.isReady())
              controlClient.checkControlEndpoint(resp, session.getID());
            if (resp.isReady()) {
              resp.sessionId = session.getID();
              session.stats.add(resp.stats);
              session.stats.totalToken = session.stats.inputToken + session.stats.cacheCreate;
              return resp;
            }
            sessionManager.onSessionChanged(session);
          }
        }
  reason: "Improve readUntilResult: always drain+log stderr, append to resultText, add 30s stuck detection"
```
/answer f447-e allow
Control Request:
```yaml
id: f447-f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: Verify the file still compiles after edit
```
/answer f447-f allow
Result Stats: id=8ed7b5e8-6862-4ab9-9f56-f8d5e28aa96b, total: 10958, in: 151, out: 3936, reason: 1259, read: 80135, write: 6871
ReasoningToken: 1259
Done. `readUntilResult` now always drains and logs STDERR each iteration (appending it to `resp.resultText`), and throws an `IOException` if 30 seconds pass with no change on either STDOUT or STDERR (stuck detection).
