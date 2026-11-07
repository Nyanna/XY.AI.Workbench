Es gib in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` Methode `executeRequest` einen Check für das MCPC Agent Profile "MCPC not reachable for AgentProfile".
Ist "MCPC" nicht verfügbar soll auf eine vorhandene Run Configuration geprüft werden.
	- Ist diese nicht vorhanden, soll sie angelegt werden
	- Ist sie nicht bereits gestartet, soll sie gestartet werden
	- Muss sie gestartet werden so wird 3 Sekunden auf die Verfügbarkeit gewartet
	- Ist sie vorhanden und bereits gestartet oder die zeit abgelaufen wird wie bisher die Exception geworfen.

- Run configuration "MCPC" (`/home/user/xyan/xy.ai.workbench/project/MCPC.launch`)
- `/home/user/xyan/xy.ai.workbench/plugin.xml`
- `/home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF`

## Hilfe und Beispiele

**Prüfen, ob eine Launch-Konfiguration läuft** (über `ILaunchManager`):

```java
ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();

boolean running = false;
for (ILaunch launch : lm.getLaunches()) {
    ILaunchConfiguration cfg = launch.getLaunchConfiguration();
    if (cfg != null && "MeineConfig".equals(cfg.getName()) && !launch.isTerminated()) {
        running = true;
        break;
    }
}
```

**Neue Konfiguration anlegen und starten:**

```java
ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();

// Typ-ID z.B. Java Application, Eclipse Application, JUnit Plug-in Test etc.
ILaunchConfigurationType type = lm.getLaunchConfigurationType(
        IJavaLaunchConfigurationConstants.ID_JAVA_APPLICATION);
        // alternativ: "org.eclipse.pde.ui.RuntimeWorkbench" für Eclipse Application

ILaunchConfigurationWorkingCopy wc =
        type.newInstance(null, lm.generateLaunchConfigurationName("MeineConfig"));

wc.setAttribute(IJavaLaunchConfigurationConstants.ATTR_PROJECT_NAME, "mein.projekt");
wc.setAttribute(IJavaLaunchConfigurationConstants.ATTR_MAIN_TYPE_NAME, "pkg.Main");
wc.setAttribute(IJavaLaunchConfigurationConstants.ATTR_PROGRAM_ARGUMENTS, "-arg1");
wc.setAttribute(IJavaLaunchConfigurationConstants.ATTR_VM_ARGUMENTS, "-Xmx512m");

ILaunchConfiguration cfg = wc.doSave();   // persistiert; weglassen für temporäre Config
// Start (run oder debug), async mit Progress-Monitor:
DebugUITools.launch(cfg, ILaunchManager.RUN_MODE);
// oder ohne UI: cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
```

**Bestehende Konfiguration suchen und wiederverwenden:**

```java
for (ILaunchConfiguration c : lm.getLaunchConfigurations(type)) {
    if (c.getName().equals("MeineConfig")) { /* nutzen */ }
}
```

**Auf Ende warten/reagieren:** `lm.addLaunchListener(ILaunchesListener2)` bzw. `DebugPlugin.getDefault().addDebugEventListener(...)` und auf `DebugEvent.TERMINATE` prüfen.

Benötigte Plug-in-Abhängigkeiten: `org.eclipse.debug.core`, `org.eclipse.debug.ui`, `org.eclipse.jdt.launching` (und `org.eclipse.jdt.debug.ui` o. ä. je nach Typ).

SystemInit:  id=2b32623a-9d8d-4f0c-ba3d-3167da55d15b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 9b8e-1
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: MCPC not reachable
    intent: ast_navigate
  reason: Locate the check in CCConnector
```
/answer 9b8e-1 deny Ziel datei ist angegeben
Control Request:
```yaml
id: 9b8e-2
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/MCPC.launch
  - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
  reason: Read launch config and manifest
```
/answer 9b8e-2 allow
Control Request:
```yaml
id: 9b8e-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/MCPC.launch
      nodes:
      - id: file
        signature: <?xml version="1.0" encoding="UTF-8" standalone="no"?>
    - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
      nodes:
      - id: file
        signature: "Manifest-Version: 1.0"
```
/answer 9b8e-3 allow
Control Request:
```yaml
id: 9b8e-4
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  text: MCPC not reachable
  reason: Locate the check
```
/answer 9b8e-4 allow
ReasoningToken: 22
Control Request:
```yaml
id: 9b8e-5
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.executeRequest
        code: |2
            @Override
            public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {
              SubMonitor sub = SubMonitor.convert(mon, "Executing prompt", 2);
              CCSession session = null;
              TimeStats ts = new TimeStats();
              ts.start();
              String relativeFilePath = req.arg.project.relativize(Paths.get(req.arg.absoluteFilePath)).toString();
              FrozenConfig fc = req.config;
              ClaudeSessionParameters params = new ClaudeSessionParameters(req.arg.project, fc.systemPrompt, fc.tools, fc.model, fc.reasoning, fc.profile, fc.keys, fc.cacheMode, relativeFilePath);
              params.setTitle(req.title);
              Command command = req.arg.command;
              if (command instanceof ResumeCommand rc) {
                sub.subTask("Importing session");
                sessionManager.importSession(rc.parameter(0), params);
                return new CCResponse(req.id, "Session created");
              }
              if (command instanceof ExitCommand) {
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
                sub.subTask("Terminating CLI process");
                session.terminate();
                return new CCResponse(req.id, "Session closed!");
              }
              if (command instanceof AnswerCommand ac) {
                if (ac.action() == AnswerCommand.Action.Allow)
                  ts.control(() -> controlClient.approve(ac.parameter(0), ac.parameter(2)));
                else if (ac.action() == AnswerCommand.Action.Exec)
                  ts.control(() -> controlClient.exec(ac.parameter(0), ac.parameter(2)));
                else
                  ts.control(() -> controlClient.deny(ac.parameter(0), ac.parameter(2)));
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallEditCommand cec) {
                if (!ts.control(() -> controlClient.submitEdit(cec.yaml())))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command instanceof CallCommand) {
                if (req.arg.yaml == null || req.arg.yaml.isBlank())
                  throw new IllegalArgumentException("No preceding ```yaml block found before /call");
                if (!ts.control(() -> controlClient.submitEdit(req.arg.yaml)))
                  throw new IllegalArgumentException("Invalid or incomplete edit YAML block");
                session = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);
              } else if (command == null) {
                sub.subTask("Acquiring session");
                session = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);
              } else
                throw new UnsupportedOperationException("Unsupported command: " + command.prefix());
              try {
                session.setInPrompt(true);
                if (command == null) {
                  if (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !ts.control(() -> controlClient.isMCPCAvailable()))
                    throw new IllegalStateException("MCPC not reachable for AgentProfile");
                  sub.subTask("Sending prompt");
                  String promptJson = requestBuilder.buildPromptJson(req.promptText);
                  long t0 = System.nanoTime();
                  session.writeLine(promptJson);
                  ts.ioMs += (System.nanoTime() - t0) / 1_000_000;
                  ts.sent++;
                }
                sub.subTask("Waiting for answer");
                return readUntilResult(req, session, sub.split(1), job, ts);
              } catch (IOException e) {
                throw new IllegalStateException("Claude Code CLI error", e);
              } finally {
                session.setInPrompt(false);
              }
            }
```
/answer 9b8e-5 allow
Control Request:
```yaml
id: 9b8e-6
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/MCPC.launch
  - /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
  regexp: .
  reason: Read file contents
```
/answer 9b8e-6 allow
Control Request:
```yaml
id: 9b8e-7
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/MCPC.launch
      nodes:
      - id: file
        code: |
          <?xml version="1.0" encoding="UTF-8" standalone="no"?>
          <launchConfiguration type="org.eclipse.ui.externaltools.ProgramLaunchConfigurationType">
              <booleanAttribute key="org.eclipse.debug.core.ATTR_FORCE_SYSTEM_CONSOLE_ENCODING" value="false"/>
              <mapAttribute key="org.eclipse.debug.core.environmentVariables">
                  <mapEntry key="TERM" value="xterm-256color"/>
              </mapAttribute>
              <stringAttribute key="org.eclipse.debug.core.terminate.timestamp" value="1792775899156"/>
              <listAttribute key="org.eclipse.debug.ui.favoriteGroups">
                  <listEntry value="org.eclipse.ui.externaltools.launchGroup"/>
              </listAttribute>
              <stringAttribute key="org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE" value="${none}"/>
              <stringAttribute key="org.eclipse.ui.externaltools.ATTR_LOCATION" value="/usr/bin/bash"/>
              <stringAttribute key="org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS" value="-lc &quot;/home/user/xyan/xy.ai.workbench/mcpc/start.sh&quot;"/>
              <stringAttribute key="org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY" value="/home/user/xyan/xy.ai.workbench/mcpc"/>
          </launchConfiguration>
    - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
      nodes:
      - id: file
        code: |
          Manifest-Version: 1.0
          Bundle-ManifestVersion: 2
          Bundle-Name: XY.AI.Workbench
          Bundle-SymbolicName: XY.AI.Workbench;singleton:=true
          Bundle-Version: 260713.1.0
          Import-Package: jakarta.inject;version="[2.0.0,3.0.0)",
           org.eclipse.ui.forms.widgets, org.eclipse.egit.ui.internal.commit,
           org.osgi.service.event
          Bundle-Activator: xy.ai.workbench.Activator
          Bundle-ClassPath: libs/jackson-dataformat-yaml-2.20.0.jar,
           libs/snakeyaml-2.4.jar,
           libs/deepseek-connector.jar,
           libs/openai/jackson-annotations-3.0-rc5.jar,
           libs/openai/jackson-core-2.20.0.jar,
           libs/openai/jackson-databind-2.20.0.jar,
           libs/openai/jackson-datatype-jdk8-2.22.2.jar,
           libs/openai/jackson-datatype-jsr310-2.22.2.jar,
           libs/openai/jackson-module-kotlin-2.22.2.jar,
           libs/openai/jackson-module-parameter-names-2.20.0.jar,
           libs/openai/kotlin-reflect-2.4.20.jar,
           libs/openai/kotlin-stdlib-2.4.20.jar,
           libs/openai/kotlin-stdlib-jdk8-2.4.20.jar,
           libs/openai/okhttp-5.5.0.jar,
           libs/openai/okio-3.18.2.jar,
           libs/openai/okio-jvm-3.18.2.jar,
           .,
           libs/openai/slf4j-api-2.1.0-alpha1.jar,
           libs/google/api-common-2.68.0.jar,
           libs/google/error_prone_annotations-2.50.0.jar,
           libs/google/google-auth-library-credentials-1.52.0.jar,
           libs/google/google-auth-library-oauth2-http-1.52.0.jar,
           libs/google/google-genai-1.70.0.jar,
           libs/google/google-http-client-2.2.0.jar,
           libs/google/google-http-client-gson-2.2.0.jar,
           libs/google/gson-2.14.0.jar,
           libs/google/guava-33.7.1-jre.jar,
           libs/anthropic/anthropic-java-2.61.0.jar,
           libs/anthropic/anthropic-java-client-okhttp-2.61.0.jar,
           libs/anthropic/anthropic-java-core-2.61.0.jar,
           libs/google/auto-value-annotations-1.11.0.jar,
           libs/openai/openai-java-4.58.0.jar,
           libs/openai/openai-java-client-okhttp-4.58.0.jar,
           libs/openai/openai-java-core-4.58.0.jar
          Bundle-Vendor: Xyan Kruse
          Require-Bundle: org.eclipse.ui,
           org.eclipse.core.runtime,
           org.eclipse.e4.core.di,
           org.eclipse.e4.core.contexts,
           org.eclipse.e4.ui.di,
           org.eclipse.e4.ui.workbench,
           org.eclipse.e4.ui.model.workbench,
           org.eclipse.jface.text,
           org.eclipse.core.resources,
           org.eclipse.ui.editors,
           org.eclipse.ui.ide,
           org.eclipse.ui.views,
           org.eclipse.ui.navigator,
           org.eclipse.search,
           org.eclipse.jgit;bundle-version="7.8.0"
          Bundle-RequiredExecutionEnvironment: JavaSE-25
          Automatic-Module-Name: XY.AI.Workbench
          Bundle-ActivationPolicy: lazy
```
/answer 9b8e-7 allow
ReasoningToken: 145
Control Request:
```yaml
id: 9b8e-8
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  node_type: Import|Package
  reason: Check imports
```
/answer 9b8e-8 allow
Control Request:
```yaml
id: 9b8e-9
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: j77HfY|S3QA7e
        type: ImportGroup
        code: |
          import xy.ai.workbench.models.TimeStats;
          import java.io.IOException;
          import java.nio.file.Paths;
          import java.util.ArrayList;
          import java.util.UUID;
          import java.util.stream.Collectors;
          import java.util.zip.CRC32;
          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.commands.AnswerCommand;
          import xy.ai.workbench.commands.CallCommand;
          import xy.ai.workbench.commands.CallEditCommand;
          import xy.ai.workbench.commands.Command;
          import xy.ai.workbench.commands.ExitCommand;
          import xy.ai.workbench.commands.ResumeCommand;
          import xy.ai.workbench.connector.IAIConnector;
          import xy.ai.workbench.connector.harness.FrozenConfig;
          import xy.ai.workbench.connector.harness.Prompt;
          import xy.ai.workbench.connector.harness.Role;
          import xy.ai.workbench.connector.harness.SessionCallbacks;
      - id: gqhGpR|GrHQlC
        type: ImportGroup
        code: |
          import xy.ai.workbench.connector.harness.IIncludeAdapter;
          import xy.ai.workbench.connector.harness.IncludeProcessor;
          import xy.ai.workbench.models.AIAnswer;
          import java.util.List;
```
/answer 9b8e-9 allow
Control Request:
```yaml
id: 9b8e-a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    old_text: |-
      !ts.control(() -> controlClient.isMCPCAvailable()))
                throw
    new_text: |-
      !ts.control(() -> controlClient.isMCPCAvailable()) && !startMcpcLaunch(ts))
                throw
  reason: Use launch fallback
```
/answer 9b8e-a allow
ReasoningToken: 167
Control Request:
```yaml
id: 9b8e-b
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: CCConnector.executeRequest
    position: before
    source: |2+
        private static final String MCPC_LAUNCH_NAME = "MCPC";

        /** Starts the MCPC run configuration if not already running; returns true if MCPC became available within 3s. */
        private boolean startMcpcLaunch(TimeStats ts) {
          try {
            ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
            for (ILaunch l : lm.getLaunches()) {
              ILaunchConfiguration c = l.getLaunchConfiguration();
              if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
                return false;
            }
            ILaunchConfiguration cfg = null;
            for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
              if (MCPC_LAUNCH_NAME.equals(c.getName())) {
                cfg = c;
                break;
              }
            }
            if (cfg == null) {
              ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
              ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
              String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
              wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
              wc.setAttribute("org.eclipse.debug.core.environmentVariables", java.util.Map.of("TERM", "xterm-256color"));
              cfg = wc.doSave();
            }
            cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
            long end = System.currentTimeMillis() + 3000;
            while (System.currentTimeMillis() < end) {
              if (ts.control(() -> controlClient.isMCPCAvailable()))
                return true;
              Thread.sleep(250);
            }
          } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
          } catch (CoreException e) {
            LOG.error("Failed to start MCPC run configuration", e);
          }
          return false;
        }

  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: j77HfY|S3QA7e
    position: after
    source: |
      import org.eclipse.core.runtime.CoreException;
      import org.eclipse.core.runtime.NullProgressMonitor;
      import org.eclipse.debug.core.DebugPlugin;
      import org.eclipse.debug.core.ILaunch;
      import org.eclipse.debug.core.ILaunchConfiguration;
      import org.eclipse.debug.core.ILaunchConfigurationType;
      import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
      import org.eclipse.debug.core.ILaunchManager;
  - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
    id: file
    position: after
    source: ""
  reason: "Add helper, imports"
```
/answer 9b8e-b allow
Control Request:
```yaml
id: 9b8e-c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: static .* (error|warn)\(
    include:
    - LOG.java
  reason: Check LOG API
```
/answer 9b8e-c allow
Control Request:
```yaml
id: 9b8e-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: LOG.java
        matches:
        - "public static void error(String message) {"
        - "public static void error(String message, Throwable throwable) {"
```
/answer 9b8e-d allow
ReasoningToken: 52
Control Request:
```yaml
id: 9b8e-e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
    id: file
    old_text: |2
       org.eclipse.search,
    new_text: |2
       org.eclipse.search,
       org.eclipse.debug.core,
  reason: Add debug.core dependency
```
/answer 9b8e-e allow
Result Stats: id=2b32623a-9d8d-4f0c-ba3d-3167da55d15b, total: 20322, in: 12, out: 3998, reason: 0, read: 56367, write: 16312, start: 1792776874065, end: 1792776879251, time: 5126, total: 25057, io: 0, control: 60, sent: 0, recv: 132
`executeRequest` in `CCConnector.java` startet jetzt die Run Configuration "MCPC", wenn MCPC nicht erreichbar ist. Kompiliert oder getestet habe ich nichts, und ich habe `ast_validate` nicht laufen lassen.

So läuft es ab:
- **Läuft "MCPC" schon:** Es wird nichts gestartet, und die bisherige `IllegalStateException("MCPC not reachable for AgentProfile")` wird geworfen.
- **Config fehlt:** Sie wird als Program-Launch angelegt, mit denselben Werten wie in `MCPC.launch` (`/usr/bin/bash -lc ".../mcpc/start.sh"`, Working Directory, `TERM`). Die Pfade sind im Code fest eingetragen.
- **Config vorhanden, aber nicht gestartet:** Sie wird im Run-Modus gestartet. Danach wird bis zu 3 Sekunden alle 250 ms geprüft, ob MCPC erreichbar ist.
- **Ablauf der 3 Sekunden oder Fehler beim Anlegen/Starten:** Die Exception wird wie bisher geworfen. Ein Fehler beim Anlegen oder Starten wird zusätzlich über `LOG.error` protokolliert.

Die Logik steckt in der neuen Methode `startMcpcLaunch` direkt vor `executeRequest`. Im `MANIFEST.MF` habe ich `org.eclipse.debug.core` zu `Require-Bundle` ergänzt. `plugin.xml` musste ich nicht ändern.

Die Config wird über `doSave()` im Workspace gespeichert. Die Datei `project/MCPC.launch` wird dabei nicht verwendet. Sie wird auch nicht ausgewertet, wenn sie im Workspace noch nicht als Launch Configuration bekannt ist.
