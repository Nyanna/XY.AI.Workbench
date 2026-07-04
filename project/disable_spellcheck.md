# Spellcheck Deaktivieren

Der Spellcheck ist nun in Eclipse integriert daher braucht der Prompt-Hook der Claude-Code-CLI nicht erneut prüfen.

## Connector

Baue in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java` ein das Grundsätzliche eine Umgebungsvariable für die Deaktivierung des Spellchecks in dem Prozessaufruf gesetzt wird.

## Skript

Ändere in `/home/user/xyan/xy.ai.workbench/claude-code/default/scripts/spell-check.sh` das bei gesetzter Umgebungsvariable der Prompt nicht aufgehalten wird.
Spell Check Errors 5:
• "Spellcheck"  →  Stellheck
• "Spellcheck"  →  Stellheck
• "Prompt-Hook"  →  Prompt-Koog | Prompt-Hooge | Prompt-Look
• "Connector"  →  Konnektor | Sonnentor | Konrektor
• "Spellchecks"  →  Stellhecks
#: Thinking:
#: The user is asking me to:
#: 1. Modify the ClaudeCodeConnector.java file to set an environment variable that disables spell check in the process call
#: 2. Modify the spell-check.sh script so that when this environment variable is set, the prompt is not held up (i.e., the script doesn't wait/block)
#: This is in German, so let me understand it better:
#: - "Spellcheck Deaktivieren" = Disable Spellcheck
#: - "Der Spellcheck ist nun in Eclipse integriert daher braucht der Prompt-Hook der Claude-Code-CLI nicht erneut prüfen" = Spellcheck is now integrated in Eclipse, so the prompt hook of Claude-Code-CLI doesn't need to check again
#: - "Connector" = Connector section
#: - "Baue in ... ein das Grundsätzliche eine Umgebungsvariable für die Deaktivierung des Spellchecks in dem Prozessaufruf gesetzt wird" = Build into [...] the fundamental setting of an environment variable for disabling spell check in the process call
#: - "Skript" = Script section
#: - "Ändere in ... das bei gesetzter Umgebungsvariable der Prompt nicht aufgehalten wird" = Change in [...] so that when the environment variable is set, the prompt is not held up/blocked
#: So the task is:
#: 1. Add environment variable setup in ClaudeCodeConnector.java to disable spell check
#: 2. Modify spell-check.sh to check for this environment variable and skip the prompt blocking when it's set
#: Let me start by reading both files to understand their current structure.
#: Tool: Read
#: file_path: "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java"
#: Tool: Read
#: file_path: "/home/user/xyan/xy.ai.workbench/claude-code/default/scripts/spell-check.sh"
#: ReasoningToken: 388
#: Thinking:
#: Now I understand both files. Let me implement the changes:
#: 1. In `ClaudeCodeConnector.java`:
#:    - I need to set an environment variable to disable spell check in the ProcessBuilder when creating the process
#:    - I should add this environment variable in the `ensureProcess` method where the ProcessBuilder is configured
#: 2. In `spell-check.sh`:
#:    - I need to check for an environment variable and skip the spell check prompt when it's set
#:    - I should add this check early in the script, similar to the AGENT_ID check
#: Let me implement:
#: - Environment variable name: `DISABLE_SPELLCHECK` or similar (I'll use `CLAUDE_CODE_DISABLE_SPELLCHECK` to be more specific)
#: For the ClaudeCodeConnector.java - I need to modify the `ensureProcess` method to set the environment variable in the ProcessBuilder before starting the process.
#: For the spell-check.sh - I need to add an early exit when the environment variable is set.
#: Tool: Edit
#: replace_all: false
#: file_path: "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java"
#: old_string: "\tprivate synchronized void ensureProcess(ClaudeCodeRequest req) throws IOException {\n\t\tif (process != null && process.isAlive())\n\t\t\treturn;\n\n\t\t// Determine working directory: fixed on first start, preserved across restarts\n\t\tif (processWorkDir == null)\n\t\t\tif (req.workDir != null)\n\t\t\t\tprocessWorkDir = req.workDir;\n\t\t\telse\n\t\t\t\tthrow new IllegalStateException(\n\t\t\t\t\t\t\"No active editor to determine working directory for Claude Code process\");\n\n\t\tList<String> cmd = requestBuilder.buildCommand(profile);\n\t\tProcessBuilder pb = new ProcessBuilder(cmd);\n\t\tpb.directory(processWorkDir.toFile());\n\t\tpb.redirectErrorStream(false);\n\n\t\tprocess = pb.start();\n\t\tstdin = new PrintWriter(process.getOutputStream());\n\t\tstdout = new BufferedReader(new InputStreamReader(process.getInputStream()));\n\n\t\tLOG.info(\"Claude Code process started in: \" + processWorkDir);\n\t\tLOG.info(\"Claude-CLI command: \" + String.join(\" \", cmd));\n\t}"
#: new_string: "\tprivate synchronized void ensureProcess(ClaudeCodeRequest req) throws IOException {\n\t\tif (process != null && process.isAlive())\n\t\t\treturn;\n\n\t\t// Determine working directory: fixed on first start, preserved across restarts\n\t\tif (processWorkDir == null)\n\t\t\tif (req.workDir != null)\n\t\t\t\tprocessWorkDir = req.workDir;\n\t\t\telse\n\t\t\t\tthrow new IllegalStateException(\n\t\t\t\t\t\t\"No active editor to determine working directory for Claude Code process\");\n\n\t\tList<String> cmd = requestBuilder.buildCommand(profile);\n\t\tProcessBuilder pb = new ProcessBuilder(cmd);\n\t\tpb.directory(processWorkDir.toFile());\n\t\tpb.redirectErrorStream(false);\n\n\t\t// Disable spell check: set environment variable for prompt hook\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_SPELLCHECK\", \"true\");\n\n\t\tprocess = pb.start();\n\t\tstdin = new PrintWriter(process.getOutputStream());\n\t\tstdout = new BufferedReader(new InputStreamReader(process.getInputStream()));\n\n\t\tLOG.info(\"Claude Code process started in: \" + processWorkDir);\n\t\tLOG.info(\"Claude-CLI command: \" + String.join(\" \", cmd));\n\t}"
#: ReasoningToken: 234
#: Thinking:
#: Good, now I need to modify the spell-check.sh script to check for the CLAUDE_CODE_DISABLE_SPELLCHECK environment variable and exit early if it's set.
#: Tool: Edit
#: replace_all: false
#: file_path: "/home/user/xyan/xy.ai.workbench/claude-code/default/scripts/spell-check.sh"
#: old_string: "#!/bin/bash\n\nINPUT=$(cat)\n\n# Skip check when hook fires inside a subagent\nAGENT_ID=$(echo \"$INPUT\" | jq -r '.agent_id // empty')\nif [ -n \"$AGENT_ID\" ]; then\n  exit 0\nfi"
#: new_string: "#!/bin/bash\n\nINPUT=$(cat)\n\n# Skip check when spell check is disabled via environment variable\nif [ -n \"$CLAUDE_CODE_DISABLE_SPELLCHECK\" ]; then\n  exit 0\nfi\n\n# Skip check when hook fires inside a subagent\nAGENT_ID=$(echo \"$INPUT\" | jq -r '.agent_id // empty')\nif [ -n \"$AGENT_ID\" ]; then\n  exit 0\nfi"
#: ReasoningToken: 45
#: Thinking:
#: Perfect! Now I've made both changes:
#: 1. In ClaudeCodeConnector.java - Added a line to set the environment variable `CLAUDE_CODE_DISABLE_SPELLCHECK` to "true" in the ProcessBuilder before starting the process
#: 2. In spell-check.sh - Added an early exit check at the top of the script (right after reading INPUT) that checks if the environment variable is set, and if so, exits immediately without blocking the prompt
#: This should allow the spell check to be cleanly disabled when the Claude Code process is launched from Eclipse, since the spell check is now integrated there.
#: ReasoningToken: 138
Perfect! ✅ Ich habe die erforderlichen Änderungen durchgeführt:

## 1. **ClaudeCodeConnector.java** (Zeile 191)
Umgebungsvariable `CLAUDE_CODE_DISABLE_SPELLCHECK` in den Prozessaufruf eingefügt:
```java
// Disable spell check: set environment variable for prompt hook
pb.environment().put("CLAUDE_CODE_DISABLE_SPELLCHECK", "true");
```

Diese Variable wird jedes Mal gesetzt, wenn der Claude Code Prozess gestartet wird.

## 2. **spell-check.sh** (Zeilen 5-7)
Frühe Prüfung der Umgebungsvariable hinzugefügt:
```bash
# Skip check when spell check is disabled via environment variable
if [ -n "$CLAUDE_CODE_DISABLE_SPELLCHECK" ]; then
  exit 0
fi
```

Wenn die Variable gesetzt ist, wird das Script sofort beendet (exit 0), ohne den Prompt zu blockieren.

## Funktionsweise
- Der Eclipse Connector setzt die Variable beim Prozessstart
- Das Spell-Check-Hook Script erkennt die Variable und überspringt alle Prüfungen
- Der Prompt wird nicht aufgehalten, da die Spellcheck-Validierung nun in Eclipse erfolgt