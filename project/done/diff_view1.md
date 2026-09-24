Ich möchte eine Diff Übersicht für einen Agenten Prompt in meinem RCP-Plugin. 

- Die Erkennung eines `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java` oder `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java` soll nach dessen Ausführung in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java`, ein Snapshot triggern.
- Das Snapshot soll mittels JGit ähnlich dem Script `/home/user/xyan/xy.ai.workbench/scripts/op-snapshot.sh` implementiert werden.
- Der Snapshot prüft, ob es überhaupt ein Delta zum speichern gibt
- Nach dem Snapshot löst ein Trigger einen Refresh im Diff-Pannels aus
- Das Diff Panel zeigt stets das aktuellste Snapshot.

- Zielpackage: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff`
- Projekt: `/home/user/xyan/xy.ai.workbench`
- Plugin: `/home/user/xyan/xy.ai.workbench/plugin.xml`

- Snapshot Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-git-snapshot-dossier.md`
- Diff Panel Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md`
SystemInit:  id=47287889-6c06-4fc7-b6cc-c51376dc7069, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 114
Control Request:
```yaml
id: b655-1
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff
  reason: "Bestehende Struktur der relevanten Klassen und des Diff-Package verstehen, um sinnvollen Diff-Plan zu erstellen."
```
/answer b655-1 allow
Control Request:
```yaml
id: b655-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
      nodes:
      - id: fau272|XRuDTB
        signature: package xy.ai.workbench.commands;
      - id: AnswerCommand
        signature: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.LN3qky|soKy4k
          signature: public static final String CMD_ANSWER = "/answer";
        - id: AnswerCommand.AnswerCommand
          signature: "public AnswerCommand() {"
        - id: AnswerCommand.AnswerCommand_1
          signature: "public AnswerCommand(String id, String action, String reason) {"
        - id: AnswerCommand.prefix
          signature: "public String prefix() {"
        - id: AnswerCommand.matches
          signature: "public boolean matches(String line) {"
        - id: AnswerCommand.parse
          signature: "public Command parse(String line) {"
        - id: AnswerCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
        - id: AnswerCommand.Action
          signature: "public enum Action {"
        - id: AnswerCommand.action
          signature: "public Action action() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
      nodes:
      - id: fau272|XRuDTB
        signature: package xy.ai.workbench.commands;
      - id: ToolCommand
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|nBKSDD
          signature: public static final String CMD_TOOL = "/tool";
        - id: ToolCommand.ToolCommand
          signature: "public ToolCommand() {"
        - id: ToolCommand.ToolCommand_1
          signature: "public ToolCommand(String toolId) {"
        - id: ToolCommand.prefix
          signature: "public String prefix() {"
        - id: ToolCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ToolCommand.parse
          signature: "public Command parse(String line) {"
        - id: ToolCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: FAE3JL|nGZQ8R
        signature: package xy.ai.workbench.connector.harness;
      - id: KEc50E|uGZWQu
        signature: import xy.ai.workbench.IncludeAdapter;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public class PromptHandler {"
        children:
        - id: PromptHandler.QEglBi|eRrE2Z
          signature: private final AdaptingConnector connector;
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, ActiveEditorListener editorListener, Ad…"
        - id: PromptHandler.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: PromptHandler.addAnswerObs
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: PromptHandler.initializeInputs
          signature: "public void initializeInputs() {"
        - id: PromptHandler.execute
          signature: "public void execute(Display display) {"
        - id: PromptHandler.PromptJob
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.preprocessPrompt
          signature: "private void preprocessPrompt(Prompt prompt) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff
      error: File not found.
```
/answer b655-2 allow
Control Request:
```yaml
id: b655-3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project
    pattern: ops-git-snapshot-dossier|ops-implementation-prompt
    include:
    - '*.md'
  - directory:
    - /home/user/xyan/xy.ai.workbench/scripts
    pattern: .*
    include:
    - op-snapshot.sh
  reason: Prüfen ob Dokumente/Skript existieren.
```
/answer b655-3 allow
Control Request:
```yaml
id: b655-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project
      matches:
      - path: ops-implementation-prompt.md
        matches:
        - lineno: 120
          match: "See the accompanying dossier (`ops-git-snapshot-dossier.md`) for:"
      - path: diff_view1.md
        matches:
        - lineno: 13
          match: "- Snapshot Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-git-snapshot-dossier.md`"
        - lineno: 14
          match: "- Diff Panel Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md`"
        - lineno: 139
          match: "pattern: ops-git-snapshot-dossier|ops-implementation-prompt"
    - directory:
      - /home/user/xyan/xy.ai.workbench/scripts
      matches:
      - path: op-snapshot.sh
        matches:
        - lineno: 1
          match: "#!/usr/bin/env bash"
        - lineno: 2
          match: "#"
        - lineno: 3
          match: "# op-snapshot.sh — Snapshot an LLM operation as a commit outside of"
        - lineno: 4
          match: "#                       HEAD/branch, under refs/llm-ops/*."
        - lineno: 5
          match: "#"
        - lineno: 6
          match: "# Usage:"
        - lineno: 7
          match: "#   op-snapshot.sh [TARGET_DIR]"
        - lineno: 8
          match: "#"
        - lineno: 9
          match: "# TARGET_DIR: path to the git repository, defaults to CWD."
        - lineno: 10
          match: "#"
        - lineno: 11
          match: "# Behavior:"
        - lineno: 12
          match: "#   - Detects the current HEAD and compares it to the base commit the"
        - lineno: 13
          match: "#     active snapshot chain was started from."
        - lineno: 14
          match: "#   - If HEAD has moved since the last snapshot (i.e. a real commit"
        - lineno: 15
          match: "#     happened on the branch in the meantime), a NEW chain is started,"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b655-4 allow
ReasoningToken: 56
Control Request:
```yaml
id: b655-5
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/ops-git-snapshot-dossier.md
  - /home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md
  - /home/user/xyan/xy.ai.workbench/project/diff_view1.md
  - /home/user/xyan/xy.ai.workbench/scripts/op-snapshot.sh
  reason: Prüfen ob Dateien existieren und deren Struktur zum Lesen ermitteln.
```
/answer b655-5 allow
Control Request:
```yaml
id: b655-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/ops-git-snapshot-dossier.md
      nodes:
      - id: lu3RQa|Du6zR2
        signature: "# LLM Operation Snapshots via…"
        children:
        - id: lu3RQa|Du6zR2.d03968
          signature: LLM Operation Snapshots via Git — Implementation Dossier
          children:
          - id: lu3RQa|Du6zR2.d03968.ea525a
            signature: 1. Concept Summary
            children:
            - id: lu3RQa|Du6zR2.d03968.ea525a.cHHoa2|ojbXVG
              signature: Every LLM-driven file operati…
            - id: lu3RQa|Du6zR2.d03968.ea525a.UDaODh|hckptQ
              signature: '**Chain semantics:** a chain …'
      - id: xISKgl|E5bByK
        signature: "## 2. Bash Reference Implemen…"
        children:
        - id: xISKgl|E5bByK.63a09b
          signature: "2. Bash Reference Implementation (baseline, CLI)"
          children:
          - id: xISKgl|E5bByK.63a09b.pZt8oc|jgfldz
            signature: |-
              ```bash
              #!/usr/bin/env bash
              #…
          - id: xISKgl|E5bByK.63a09b.EYzOrb|uYHFFA
            signature: '**Revert:**'
          - id: xISKgl|E5bByK.63a09b.3HlYAz|5E36W7
            signature: |-
              ```bash
              #!/usr/bin/env bash
              s…
      - id: Q8pJO7|HTNLA9
        signature: |-
          ## 3. JGit Port

          ### 3.1 Depe…
        children:
        - id: Q8pJO7|HTNLA9.d69d26
          signature: 3. JGit Port
          children:
          - id: Q8pJO7|HTNLA9.d69d26.c27d12
            signature: 3.1 Dependencies
            children:
            - id: Q8pJO7|HTNLA9.d69d26.c27d12.i8m4ZX|HCPJjB
              signature: |-
                ```xml
                <!-- Maven, or equival…
            - id: Q8pJO7|HTNLA9.d69d26.c27d12.Vw3EJy|BSzOAs
              signature: "In an Eclipse plugin, JGit is…"
          - id: Q8pJO7|HTNLA9.d69d26.17b957
            signature: 3.2 Snapshot creation (equivalent of the bash script)
            children:
            - id: Q8pJO7|HTNLA9.d69d26.17b957.sC8WZA|cAtsT7
              signature: |-
                ```java
                import org.eclipse.jg…
          - id: Q8pJO7|HTNLA9.d69d26.5fd144
            signature: "3.3 Diff generation (unified diff, full commit — matches `git diff parent commit`)"
            children:
            - id: Q8pJO7|HTNLA9.d69d26.5fd144.WW5r4W|dsK66Z
              signature: |-
                ```java
                import org.eclipse.jg…
            - id: Q8pJO7|HTNLA9.d69d26.5fd144.jy59UK|hRb1ip
              signature: This single `format()` call o…
          - id: Q8pJO7|HTNLA9.d69d26.89dfb6
            signature: 3.4 Revert (equivalent of `git read-tree --reset -u` + `git clean -fd`)
            children:
            - id: Q8pJO7|HTNLA9.d69d26.89dfb6.TRHjAV|9tDJAx
              signature: |-
                ```java
                import org.eclipse.jg…
            - id: Q8pJO7|HTNLA9.d69d26.89dfb6.p3VFPZ|UWgGDj
              signature: '**Important:** do not use JGi…'
          - id: Q8pJO7|HTNLA9.d69d26.493a5f
            signature: 3.5 Session export / cleanup
            children:
            - id: Q8pJO7|HTNLA9.d69d26.493a5f.ZMckAj|fkhOcW
              signature: |-
                ```java
                // List all chains an…
      - id: WTwH5O|TWK4EJ
        signature: "## 4. Reading EGit's History …"
        children:
        - id: WTwH5O|TWK4EJ.03bf76
          signature: 4. Reading EGit's History Panel Programmatically
          children:
          - id: WTwH5O|TWK4EJ.03bf76.16f472
            signature: 4.1 Opening the standard History view for a resource/repo
            children:
            - id: WTwH5O|TWK4EJ.03bf76.16f472.3fGuXR|88FqsW
              signature: |-
                ```java
                import org.eclipse.te…
            - id: WTwH5O|TWK4EJ.03bf76.16f472.82L4x4|3GOR9K
              signature: '`IHistoryView` is public Ecli…'
          - id: WTwH5O|TWK4EJ.03bf76.32ee64
            signature: 4.2 Additional Refs — where custom namespaces surface
            children:
            - id: WTwH5O|TWK4EJ.03bf76.32ee64.f1ZMHv|QcdDN9
              signature: EGit's "Git Repositories" vie…
          - id: WTwH5O|TWK4EJ.03bf76.3678da
            signature: 4.3 Opening EGit's Commit Editor for one specific commit
            children:
            - id: WTwH5O|TWK4EJ.03bf76.3678da.aDOPFV|acqCy4
              signature: "Public, version-stable comman…"
            - id: WTwH5O|TWK4EJ.03bf76.3678da.i9Salk|Tt4PzF
              signature: |-
                ```java
                import org.eclipse.co…
            - id: WTwH5O|TWK4EJ.03bf76.3678da.4T61bX|ZxdQdg
              signature: '**Caveat:** verify this comma…'
            - id: WTwH5O|TWK4EJ.03bf76.3678da.TVH8wY|5qDPoD
              signature: '**Limitation this dossier''s §…'
      - id: 5uhc9h|G3DUP5
        signature: "## 5. Standalone Singleton Pa…"
        children:
        - id: 5uhc9h|G3DUP5.98e0b7
          signature: 5. Standalone Singleton Panel — Full-Operation Unified Diff (Commit-Viewer style)
          children:
          - id: 5uhc9h|G3DUP5.98e0b7.ee9cd0
            signature: "5.1 Why a `ViewPart`, not an `EditorPart`"
            children:
            - id: 5uhc9h|G3DUP5.98e0b7.ee9cd0.X7TplB|J6E2KH
              signature: Eclipse `ViewPart`s are singl…
          - id: 5uhc9h|G3DUP5.98e0b7.092381
            signature: "5.2 Rendering approach: raw unified diff text, not per-file Compare widgets"
            children:
            - id: 5uhc9h|G3DUP5.98e0b7.092381.yD63p0|J6o6Ck
              signature: Since `DiffFormatter.format(o…
          - id: 5uhc9h|G3DUP5.98e0b7.43f3fe
            signature: 5.3 Full ViewPart implementation
            children:
            - id: 5uhc9h|G3DUP5.98e0b7.43f3fe.biqPAt|SU2U1p
              signature: |-
                ```java
                package com.example.l…
          - id: 5uhc9h|G3DUP5.98e0b7.a7fed8
            signature: 5.4 View registration (`plugin.xml`)
            children:
            - id: 5uhc9h|G3DUP5.98e0b7.a7fed8.oz6Owk|D8AoDs
              signature: |-
                ```xml
                <extension point="org.…
            - id: 5uhc9h|G3DUP5.98e0b7.a7fed8.oAF7nL|781yiF
              signature: '`allowMultiple="false"` (the …'
            - id: 5uhc9h|G3DUP5.98e0b7.a7fed8.H8fCWr|TXcGMQ
              signature: |-
                ```java
                LlmOpsPanel panel = (…
          - id: 5uhc9h|G3DUP5.98e0b7.6d7162
            signature: 5.5 Revert action wiring
            children:
            - id: 5uhc9h|G3DUP5.98e0b7.6d7162.ThEAnK|JchOaA
              signature: Add a toolbar button (`org.ec…
            - id: 5uhc9h|G3DUP5.98e0b7.6d7162.TLtVoT|BC1ce8
              signature: |-
                ```java
                LlmOpReverter.revertT…
      - id: ApoLGr|acNTOM
        signature: |-
          ## 6. Key Pitfalls Checklist
          …
        children:
        - id: ApoLGr|acNTOM.2f990e
          signature: 6. Key Pitfalls Checklist
    - path: /home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md
      nodes:
      - id: 0ce91a
        signature: "Implementation Prompt: Git-Based LLM Operation Snapshot & Review System"
        children:
        - id: 0ce91a.cc11b3
          signature: Context
          children:
          - id: 0ce91a.cc11b3.AlBcjD|aIGtik
            signature: An Eclipse plugin runs an LLM…
          - id: 0ce91a.cc11b3.k7N3V1|YP7slR
            signature: The chosen mechanism uses Git…
        - id: 0ce91a.7bb0dd
          signature: Task
          children:
          - id: 0ce91a.7bb0dd.gBsQgI|IT0TdP
            signature: Implement this system as part…
        - id: 0ce91a.bca3e8
          signature: Functional Requirements
        - id: 0ce91a.d9d570
          signature: Non-Functional / Robustness Requirements
        - id: 0ce91a.c1a891
          signature: Explicitly Out of Scope
        - id: 0ce91a.d2709d
          signature: Reference Material
          children:
          - id: 0ce91a.d2709d.1W1d5X|Z0HdkB
            signature: See the accompanying dossier …
        - id: 0ce91a.ff3b5d
          signature: Deliverables Expected From Implementation
    - path: /home/user/xyan/xy.ai.workbench/project/diff_view1.md
      nodes:
      - id: tlq3tB|WBMHl7
        signature: Ich möchte eine Diff Übersich…
      - id: 7n87mF|gUJDqX
        signature: '- Die Erkennung eines `/home/…'
      - id: JwmGim|O4k6Ah
        signature: |-
          ```yaml
          id: b655-1
          toolName: …
      - id: MaWfCv|fXjSC9
        signature: |-
          /answer b655-1 allow
          Control …
      - id: w1K1Me|b5Fjv5
        signature: |-
          ```yaml
          id: b655-2
          result:
            …
      - id: NwsLzL|5ebAlS
        signature: |-
          /answer b655-2 allow
          Control …
      - id: RtkAWi|1i3mSS
        signature: |-
          ```yaml
          id: b655-3
          toolName: …
      - id: mT6N7N|x0MW1E
        signature: |-
          /answer b655-3 allow
          Control …
      - id: OwHbT2|R2rGZx
        signature: |-
          ```yaml
          id: b655-4
          result:
            …
      - id: iSkDyn|aJTFiS
        signature: |-
          /answer b655-4 allow
          Reasonin…
      - id: DJXp0o|mIvyn0
        signature: |-
          ```yaml
          id: b655-5
          toolName: …
      - id: CTdPw7|7Phvya
        signature: |-
          /answer b655-5 allow
          [xy.ai.r…
    - path: /home/user/xyan/xy.ai.workbench/scripts/op-snapshot.sh
      nodes:
      - id: wsOXc0|h9tL3l
        signature: "#!/usr/bin/env bash"
      - id: ynNDr7|Je0Ge3
        signature: "#"
      - id: ElkmLY|jxBp2k
        signature: "# op-snapshot.sh — Snapshot an LLM operation as a commit outside of"
      - id: X6Wl4q|PUZJql
        signature: "#                       HEAD/branch, under refs/llm-ops/*."
      - id: ynNDr7|Je0Ge3_1
        signature: "#"
      - id: naJ4p9|KazmZS
        signature: "# Usage:"
      - id: XAolnS|Bms854
        signature: "#   op-snapshot.sh [TARGET_DIR]"
      - id: ynNDr7|Je0Ge3_2
        signature: "#"
      - id: gGH2s2|cqO0CH
        signature: "# TARGET_DIR: path to the git repository, defaults to CWD."
      - id: ynNDr7|Je0Ge3_3
        signature: "#"
      - id: fvqq1d|cwulHZ
        signature: "# Behavior:"
      - id: M9hY87|WdKvcG
        signature: "#   - Detects the current HEAD and compares it to the base commit the"
      - id: McRm5A|3xMuxb
        signature: "#     active snapshot chain was started from."
      - id: pclTZJ|m0Y1Nk
        signature: "#   - If HEAD has moved since the last snapshot (i.e. a real commit"
      - id: mT2hc9|ICmuri
        signature: "#     happened on the branch in the meantime), a NEW chain is started,"
      - id: F3ZYtq|HNDoiT
        signature: "#     rooted at the new HEAD. The old chain is left untouched under its"
      - id: 5vCXAm|Yt288l
        signature: "#     own namespace for later inspection."
      - id: Rtd9zf|HWpYeF
        signature: "#   - Otherwise, the snapshot is appended to the existing chain, parented"
      - id: DlG7ll|CETDI4
        signature: "#     on the previous snapshot commit."
      - id: Zs4wwc|OqRdQk
        signature: "#   - Commit message is fixed to the current date/time (no free text)."
      - id: ObHa3W|FDccRn
        signature: "#   - Prints the diff against the previous snapshot (or HEAD, if this is"
      - id: kzMEUg|OA7jNy
        signature: "#     the first snapshot of the chain) to stdout."
      - id: xgAuOX|DR2VdR
        signature: "#   - Prints stable KEY=VALUE lines at the end for the calling hook to parse."
      - id: set
        signature: set -euo pipefail
        children:
        - id: set.dIwBgt|zeb4GX
          signature: set
          children:
          - id: set.dIwBgt|zeb4GX.dIwBgt|zeb4GX
            signature: set
        - id: set.IR2tgX|WrFqAs
          signature: -euo
        - id: set.As0cIG|6hUHVl
          signature: pipefail
      - id: TARGET_DIR
        signature: "TARGET_DIR=\"${1:-$(pwd)}\""
        children:
        - id: TARGET_DIR.HttoXh|Sf2W03
          signature: TARGET_DIR
        - id: TARGET_DIR.55cSYN|CErPd2
          signature: "\"${1:-$(pwd)}\""
          children:
          - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf
            signature: "${1:-$(pwd)}"
            children:
            - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf.L8Zvvc|lVqSnb
              signature: 1
            - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf.VdoG4s|I21MVf
              signature: $(pwd)
              children:
              - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf.VdoG4s|I21MVf.pwd
                signature: pwd
                children:
                - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf.VdoG4s|I21MVf.pwd.vap7bt|VcADsy
                  signature: pwd
                  children:
                  - id: TARGET_DIR.55cSYN|CErPd2.jQckmw|Ag0wyf.VdoG4s|I21MVf.pwd.vap7bt|VcADsy.vap7bt|VcADsy
                    signature: pwd
      - id: 99Eqi6|6K5IeO
        signature: "# --- Preconditions -----------------------------------------------------"
      - id: Hou5QZ|JACneA
        signature: if ! GIT_DIR_CHECK=$(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1); then
        children:
        - id: Hou5QZ|JACneA.hOXE5X|ar6q50
          signature: '! GIT_DIR_CHECK=$(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1)'
          children:
          - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK
            signature: GIT_DIR_CHECK=$(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1)
            children:
            - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.M0l1jy|sCzvz2
              signature: GIT_DIR_CHECK
            - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU
              signature: $(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1)
              children:
              - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g
                signature: git -C "$TARGET_DIR" rev-parse --git-dir 2>&1
                children:
                - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git
                  signature: git -C "$TARGET_DIR" rev-parse --git-dir
                  children:
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.KFE4Bh|L4z8rV
                    signature: git
                    children:
                    - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                      signature: git
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.i47AXo|ciOvQy
                    signature: -C
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.oiHveY|0qzG4s
                    signature: '"$TARGET_DIR"'
                    children:
                    - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.oiHveY|0qzG4s.x9NKFb|2CsNij
                      signature: $TARGET_DIR
                      children:
                      - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.oiHveY|0qzG4s.x9NKFb|2CsNij.HttoXh|Sf2W03
                        signature: TARGET_DIR
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.gDXhg0|DIlGir
                    signature: rev-parse
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.git.QUe4bx|fQWrqq
                    signature: --git-dir
                - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.lyMqJh|Ogizcj
                  signature: 2>&1
                  children:
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.lyMqJh|Ogizcj.PEvZmA|I4fuEo
                    signature: 2
                  - id: Hou5QZ|JACneA.hOXE5X|ar6q50.GIT_DIR_CHECK.PcPU2u|APilsU.koXRHj|5NVc7g.lyMqJh|Ogizcj.L8Zvvc|lVqSnb
                    signature: 1
        - id: Hou5QZ|JACneA.hfso5R|BeqBhn
          signature: "echo \"Error: '$TARGET_DIR' is not a git repository.\" >&2"
          children:
          - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo
            signature: "echo \"Error: '$TARGET_DIR' is not a git repository.\""
            children:
            - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.vhMnXv|TcYAkX
              signature: echo
              children:
              - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                signature: echo
            - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.gOjxF4|jTvH54
              signature: "\"Error: '$TARGET_DIR' is not a git repository.\""
              children:
              - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.gOjxF4|jTvH54.8MJcvH|CubwNX
                signature: "Error: '"
              - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.gOjxF4|jTvH54.x9NKFb|2CsNij
                signature: $TARGET_DIR
                children:
                - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.gOjxF4|jTvH54.x9NKFb|2CsNij.HttoXh|Sf2W03
                  signature: TARGET_DIR
              - id: Hou5QZ|JACneA.hfso5R|BeqBhn.echo.gOjxF4|jTvH54.hl1XZV|rtuWup
                signature: ''' is not a git repository.'
          - id: Hou5QZ|JACneA.hfso5R|BeqBhn.Rh7ZNz|66tcI2
            signature: '>&2'
            children:
            - id: Hou5QZ|JACneA.hfso5R|BeqBhn.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
              signature: 2
        - id: Hou5QZ|JACneA.eaaacV|EcIO8S
          signature: echo "$GIT_DIR_CHECK" >&2
          children:
          - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo
            signature: echo "$GIT_DIR_CHECK"
            children:
            - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo.vhMnXv|TcYAkX
              signature: echo
              children:
              - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                signature: echo
            - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo.2U3s0R|udODj9
              signature: '"$GIT_DIR_CHECK"'
              children:
              - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo.2U3s0R|udODj9.DyKFld|OoGOOz
                signature: $GIT_DIR_CHECK
                children:
                - id: Hou5QZ|JACneA.eaaacV|EcIO8S.echo.2U3s0R|udODj9.DyKFld|OoGOOz.M0l1jy|sCzvz2
                  signature: GIT_DIR_CHECK
          - id: Hou5QZ|JACneA.eaaacV|EcIO8S.Rh7ZNz|66tcI2
            signature: '>&2'
            children:
            - id: Hou5QZ|JACneA.eaaacV|EcIO8S.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
              signature: 2
        - id: Hou5QZ|JACneA.exit
          signature: exit 1
          children:
          - id: Hou5QZ|JACneA.exit.ECuVlZ|g5BskS
            signature: exit
            children:
            - id: Hou5QZ|JACneA.exit.ECuVlZ|g5BskS.ECuVlZ|g5BskS
              signature: exit
          - id: Hou5QZ|JACneA.exit.L8Zvvc|lVqSnb
            signature: 1
      - id: cd
        signature: cd "$TARGET_DIR"
        children:
        - id: cd.fR3fl1|UcRDKn
          signature: cd
          children:
          - id: cd.fR3fl1|UcRDKn.fR3fl1|UcRDKn
            signature: cd
        - id: cd.oiHveY|0qzG4s
          signature: '"$TARGET_DIR"'
          children:
          - id: cd.oiHveY|0qzG4s.x9NKFb|2CsNij
            signature: $TARGET_DIR
            children:
            - id: cd.oiHveY|0qzG4s.x9NKFb|2CsNij.HttoXh|Sf2W03
              signature: TARGET_DIR
      - id: A3CjF9|pYvCVz
        signature: if ! CURRENT_HEAD=$(git rev-parse HEAD 2>/dev/null); then
        children:
        - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC
          signature: '! CURRENT_HEAD=$(git rev-parse HEAD 2>/dev/null)'
          children:
          - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD
            signature: CURRENT_HEAD=$(git rev-parse HEAD 2>/dev/null)
            children:
            - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.bT7ZNR|BsQDSf
              signature: CURRENT_HEAD
            - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm
              signature: $(git rev-parse HEAD 2>/dev/null)
              children:
              - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn
                signature: git rev-parse HEAD 2>/dev/null
                children:
                - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.git
                  signature: git rev-parse HEAD
                  children:
                  - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.git.KFE4Bh|L4z8rV
                    signature: git
                    children:
                    - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                      signature: git
                  - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.git.gDXhg0|DIlGir
                    signature: rev-parse
                  - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.git.n8FElH|PxYhWw
                    signature: HEAD
                - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.JErUXN|Lr9GaI
                  signature: 2>/dev/null
                  children:
                  - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.JErUXN|Lr9GaI.PEvZmA|I4fuEo
                    signature: 2
                  - id: A3CjF9|pYvCVz.qWAxhs|wmlKLC.CURRENT_HEAD.BKCvAd|MSjfDm.GuKNPp|YH2xNn.JErUXN|Lr9GaI.69AD4l|mb2flb
                    signature: /dev/null
        - id: A3CjF9|pYvCVz.bpt4d6|toWtxi
          signature: "echo \"Error: no HEAD found (empty repository?).\" >&2"
          children:
          - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.echo
            signature: "echo \"Error: no HEAD found (empty repository?).\""
            children:
            - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.echo.vhMnXv|TcYAkX
              signature: echo
              children:
              - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                signature: echo
            - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.echo.qOrT39|Wq8AxH
              signature: "\"Error: no HEAD found (empty repository?).\""
              children:
              - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.echo.qOrT39|Wq8AxH.Hf0Duy|RmJhUH
                signature: "Error: no HEAD found (empty repository?)."
          - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.Rh7ZNz|66tcI2
            signature: '>&2'
            children:
            - id: A3CjF9|pYvCVz.bpt4d6|toWtxi.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
              signature: 2
        - id: A3CjF9|pYvCVz.exit
          signature: exit 1
          children:
          - id: A3CjF9|pYvCVz.exit.ECuVlZ|g5BskS
            signature: exit
            children:
            - id: A3CjF9|pYvCVz.exit.ECuVlZ|g5BskS.ECuVlZ|g5BskS
              signature: exit
          - id: A3CjF9|pYvCVz.exit.L8Zvvc|lVqSnb
            signature: 1
      - id: BASE_MARKER
        signature: BASE_MARKER="refs/llm-ops/_base"
        children:
        - id: BASE_MARKER.KLFeu0|COEO38
          signature: BASE_MARKER
        - id: BASE_MARKER.wCG1YT|koU26d
          signature: '"refs/llm-ops/_base"'
          children:
          - id: BASE_MARKER.wCG1YT|koU26d.R1eaVr|MSoWDM
            signature: refs/llm-ops/_base
      - id: rqoDRE|po9NeM
        signature: "# --- Determine whether the active chain is still valid ------------------"
      - id: STORED_BASE
        signature: STORED_BASE=""
        children:
        - id: STORED_BASE.is8knb|BOm3kR
          signature: STORED_BASE
        - id: STORED_BASE.z8uFN0|jNP30L
          signature: '""'
      - id: fKB37g|EoF5I5
        signature: if git show-ref --verify --quiet "$BASE_MARKER"; then
        children:
        - id: fKB37g|EoF5I5.git
          signature: git show-ref --verify --quiet "$BASE_MARKER"
          children:
          - id: fKB37g|EoF5I5.git.KFE4Bh|L4z8rV
            signature: git
            children:
            - id: fKB37g|EoF5I5.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
              signature: git
          - id: fKB37g|EoF5I5.git.FyE1yB|sOZ2aL
            signature: show-ref
          - id: fKB37g|EoF5I5.git.1uP64h|zbwxdi
            signature: --verify
          - id: fKB37g|EoF5I5.git.jbgMPG|uiNoFs
            signature: --quiet
          - id: fKB37g|EoF5I5.git.rh2R0c|zbDm37
            signature: '"$BASE_MARKER"'
            children:
            - id: fKB37g|EoF5I5.git.rh2R0c|zbDm37.XzKMas|AVTcst
              signature: $BASE_MARKER
              children:
              - id: fKB37g|EoF5I5.git.rh2R0c|zbDm37.XzKMas|AVTcst.KLFeu0|COEO38
                signature: BASE_MARKER
        - id: fKB37g|EoF5I5.STORED_BASE
          signature: STORED_BASE=$(git rev-parse "$BASE_MARKER")
          children:
          - id: fKB37g|EoF5I5.STORED_BASE.is8knb|BOm3kR
            signature: STORED_BASE
          - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI
            signature: $(git rev-parse "$BASE_MARKER")
            children:
            - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git
              signature: git rev-parse "$BASE_MARKER"
              children:
              - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.KFE4Bh|L4z8rV
                signature: git
                children:
                - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                  signature: git
              - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.gDXhg0|DIlGir
                signature: rev-parse
              - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.rh2R0c|zbDm37
                signature: '"$BASE_MARKER"'
                children:
                - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.rh2R0c|zbDm37.XzKMas|AVTcst
                  signature: $BASE_MARKER
                  children:
                  - id: fKB37g|EoF5I5.STORED_BASE.8qAJNg|yypbGI.git.rh2R0c|zbDm37.XzKMas|AVTcst.KLFeu0|COEO38
                    signature: BASE_MARKER
      - id: xJ9V4y|Rbh3c0
        signature: "if [ -z \"$STORED_BASE\" ] || [ \"$STORED_BASE\" != \"$CURRENT_HEAD\" ]; then"
        children:
        - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml
          signature: "[ -z \"$STORED_BASE\" ] || [ \"$STORED_BASE\" != \"$CURRENT_HEAD\" ]"
          children:
          - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT
            signature: "[ -z \"$STORED_BASE\" ]"
            children:
            - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT.STORED_BASE
              signature: -z "$STORED_BASE"
              children:
              - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT.STORED_BASE.ruAkts|cOm2pG
                signature: -z
              - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT.STORED_BASE.pm7TxN|0CJh43
                signature: '"$STORED_BASE"'
                children:
                - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc
                  signature: $STORED_BASE
                  children:
                  - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.qbYuwE|fskPnT.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                    signature: STORED_BASE
          - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL
            signature: "[ \"$STORED_BASE\" != \"$CURRENT_HEAD\" ]"
            children:
            - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE
              signature: '"$STORED_BASE" != "$CURRENT_HEAD"'
              children:
              - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.pm7TxN|0CJh43
                signature: '"$STORED_BASE"'
                children:
                - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc
                  signature: $STORED_BASE
                  children:
                  - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                    signature: STORED_BASE
              - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.7HYEhE|fwwNaO
                signature: '"$CURRENT_HEAD"'
                children:
                - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.7HYEhE|fwwNaO.S10KS4|Z21VYR
                  signature: $CURRENT_HEAD
                  children:
                  - id: xJ9V4y|Rbh3c0.b0r7U2|nGzGml.bFrj0j|rX7zDL.STORED_BASE.7HYEhE|fwwNaO.S10KS4|Z21VYR.bT7ZNR|BsQDSf
                    signature: CURRENT_HEAD
        - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J
          signature: "if [ -n \"$STORED_BASE\" ]; then"
          children:
          - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT
            signature: "[ -n \"$STORED_BASE\" ]"
            children:
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT.STORED_BASE
              signature: -n "$STORED_BASE"
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT.STORED_BASE.swpt2o|G8cS18
                signature: -n
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT.STORED_BASE.pm7TxN|0CJh43
                signature: '"$STORED_BASE"'
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc
                  signature: $STORED_BASE
                  children:
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.cb7qQn|eYT2KT.STORED_BASE.pm7TxN|0CJh43.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                    signature: STORED_BASE
          - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs
            signature: echo "HEAD has advanced since the last snapshot (real commit detected)." >&2
            children:
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.echo
              signature: echo "HEAD has advanced since the last snapshot (real commit detected)."
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.echo.vhMnXv|TcYAkX
                signature: echo
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                  signature: echo
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.echo.tN8Lau|7YgCwX
                signature: '"HEAD has advanced since the last snapshot (real commit detected)."'
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.echo.tN8Lau|7YgCwX.Gev7aO|oKmOA8
                  signature: HEAD has advanced since the last snapshot (real commit detected).
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.Rh7ZNz|66tcI2
              signature: '>&2'
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.MAIhgV|2tQEKs.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
                signature: 2
          - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh
            signature: "echo \"Previous chain base: $STORED_BASE\" >&2"
            children:
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo
              signature: "echo \"Previous chain base: $STORED_BASE\""
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.vhMnXv|TcYAkX
                signature: echo
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                  signature: echo
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.2oh5Dd|Eg4THb
                signature: "\"Previous chain base: $STORED_BASE\""
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.2oh5Dd|Eg4THb.T6ekFM|JRzjOK
                  signature: "Previous chain base:"
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.2oh5Dd|Eg4THb.gLJ3QD|Qt28Oc
                  signature: $STORED_BASE
                  children:
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.echo.2oh5Dd|Eg4THb.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                    signature: STORED_BASE
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.Rh7ZNz|66tcI2
              signature: '>&2'
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.51QWpM|OdiTKh.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
                signature: 2
          - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD
            signature: "echo \"Starting a new chain rooted at: $CURRENT_HEAD\" >&2"
            children:
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo
              signature: "echo \"Starting a new chain rooted at: $CURRENT_HEAD\""
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.vhMnXv|TcYAkX
                signature: echo
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                  signature: echo
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.b5AyM6|n4Wgxj
                signature: "\"Starting a new chain rooted at: $CURRENT_HEAD\""
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.b5AyM6|n4Wgxj.ku4y5m|t5FY2W
                  signature: "Starting a new chain rooted at:"
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.b5AyM6|n4Wgxj.S10KS4|Z21VYR
                  signature: $CURRENT_HEAD
                  children:
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.echo.b5AyM6|n4Wgxj.S10KS4|Z21VYR.bT7ZNR|BsQDSf
                    signature: CURRENT_HEAD
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.Rh7ZNz|66tcI2
              signature: '>&2'
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.eDP8Tx|CENFnD.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
                signature: 2
          - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C
            signature: else
            children:
            - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR
              signature: "echo \"No active chain found. Starting new chain rooted at: $CURRENT_HEAD\" >&2"
              children:
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo
                signature: "echo \"No active chain found. Starting new chain rooted at: $CURRENT_HEAD\""
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.vhMnXv|TcYAkX
                  signature: echo
                  children:
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                    signature: echo
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.g84fi5|OjJyxe
                  signature: "\"No active chain found. Starting new chain rooted at: $CURRENT_HEAD\""
                  children:
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.g84fi5|OjJyxe.R3MYwd|HUnGLH
                    signature: "No active chain found. Starting new chain rooted at:"
                  - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.g84fi5|OjJyxe.S10KS4|Z21VYR
                    signature: $CURRENT_HEAD
                    children:
                    - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.echo.g84fi5|OjJyxe.S10KS4|Z21VYR.bT7ZNR|BsQDSf
                      signature: CURRENT_HEAD
              - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.Rh7ZNz|66tcI2
                signature: '>&2'
                children:
                - id: xJ9V4y|Rbh3c0.9gsmoV|9gth5J.Xwtz9U|ZHEI7C.N9Hb8I|zXgiAR.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
                  signature: 2
        - id: xJ9V4y|Rbh3c0.git
          signature: git update-ref "$BASE_MARKER" "$CURRENT_HEAD"
          children:
          - id: xJ9V4y|Rbh3c0.git.KFE4Bh|L4z8rV
            signature: git
            children:
            - id: xJ9V4y|Rbh3c0.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
              signature: git
          - id: xJ9V4y|Rbh3c0.git.GuDOQg|r6294T
            signature: update-ref
          - id: xJ9V4y|Rbh3c0.git.rh2R0c|zbDm37
            signature: '"$BASE_MARKER"'
            children:
            - id: xJ9V4y|Rbh3c0.git.rh2R0c|zbDm37.XzKMas|AVTcst
              signature: $BASE_MARKER
              children:
              - id: xJ9V4y|Rbh3c0.git.rh2R0c|zbDm37.XzKMas|AVTcst.KLFeu0|COEO38
                signature: BASE_MARKER
          - id: xJ9V4y|Rbh3c0.git.7HYEhE|fwwNaO
            signature: '"$CURRENT_HEAD"'
            children:
            - id: xJ9V4y|Rbh3c0.git.7HYEhE|fwwNaO.S10KS4|Z21VYR
              signature: $CURRENT_HEAD
              children:
              - id: xJ9V4y|Rbh3c0.git.7HYEhE|fwwNaO.S10KS4|Z21VYR.bT7ZNR|BsQDSf
                signature: CURRENT_HEAD
        - id: xJ9V4y|Rbh3c0.STORED_BASE
          signature: STORED_BASE="$CURRENT_HEAD"
          children:
          - id: xJ9V4y|Rbh3c0.STORED_BASE.is8knb|BOm3kR
            signature: STORED_BASE
          - id: xJ9V4y|Rbh3c0.STORED_BASE.7HYEhE|fwwNaO
            signature: '"$CURRENT_HEAD"'
            children:
            - id: xJ9V4y|Rbh3c0.STORED_BASE.7HYEhE|fwwNaO.S10KS4|Z21VYR
              signature: $CURRENT_HEAD
              children:
              - id: xJ9V4y|Rbh3c0.STORED_BASE.7HYEhE|fwwNaO.S10KS4|Z21VYR.bT7ZNR|BsQDSf
                signature: CURRENT_HEAD
      - id: CHAIN_ID
        signature: CHAIN_ID=$(git rev-parse --short "$STORED_BASE")
        children:
        - id: CHAIN_ID.AR71UY|5GpB02
          signature: CHAIN_ID
        - id: CHAIN_ID.5DhQX4|RGgRFa
          signature: $(git rev-parse --short "$STORED_BASE")
          children:
          - id: CHAIN_ID.5DhQX4|RGgRFa.git
            signature: git rev-parse --short "$STORED_BASE"
            children:
            - id: CHAIN_ID.5DhQX4|RGgRFa.git.KFE4Bh|L4z8rV
              signature: git
              children:
              - id: CHAIN_ID.5DhQX4|RGgRFa.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                signature: git
            - id: CHAIN_ID.5DhQX4|RGgRFa.git.gDXhg0|DIlGir
              signature: rev-parse
            - id: CHAIN_ID.5DhQX4|RGgRFa.git.OKz9HC|s3XgHC
              signature: --short
            - id: CHAIN_ID.5DhQX4|RGgRFa.git.pm7TxN|0CJh43
              signature: '"$STORED_BASE"'
              children:
              - id: CHAIN_ID.5DhQX4|RGgRFa.git.pm7TxN|0CJh43.gLJ3QD|Qt28Oc
                signature: $STORED_BASE
                children:
                - id: CHAIN_ID.5DhQX4|RGgRFa.git.pm7TxN|0CJh43.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                  signature: STORED_BASE
      - id: NAMESPACE
        signature: NAMESPACE="refs/llm-ops/chain-$CHAIN_ID"
        children:
        - id: NAMESPACE.DgdUD6|k4yVO4
          signature: NAMESPACE
        - id: NAMESPACE.bV76zt|UQKAye
          signature: '"refs/llm-ops/chain-$CHAIN_ID"'
          children:
          - id: NAMESPACE.bV76zt|UQKAye.EfEmPN|2ut9m8
            signature: refs/llm-ops/chain-
          - id: NAMESPACE.bV76zt|UQKAye.wCjuna|FuWinw
            signature: $CHAIN_ID
            children:
            - id: NAMESPACE.bV76zt|UQKAye.wCjuna|FuWinw.AR71UY|5GpB02
              signature: CHAIN_ID
      - id: LdMUYp|A3BOCN
        signature: "# --- Find the last snapshot within the active chain ----------------------"
      - id: LAST_N
        signature: LAST_N=0
        children:
        - id: LAST_N.qkWEnr|YVdbfl
          signature: LAST_N
        - id: LAST_N.QDjlfP|EoJOXM
          signature: 0
      - id: LAST_OP_REF
        signature: LAST_OP_REF=""
        children:
        - id: LAST_OP_REF.nYpTVo|v6Ci0D
          signature: LAST_OP_REF
        - id: LAST_OP_REF.z8uFN0|jNP30L
          signature: '""'
      - id: s0Twu8|yGRhW0
        signature: while IFS= read -r ref; do
        children:
        - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R
          signature: while IFS= read -r ref; do
          children:
          - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read
            signature: IFS= read -r ref
            children:
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.IFS
              signature: IFS=
              children:
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.IFS.yv6ZyO|o74pyJ
                signature: IFS
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.wp6CyS|1ZQFWh
              signature: read
              children:
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.wp6CyS|1ZQFWh.wp6CyS|1ZQFWh
                signature: read
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.XbeIYT|rymIea
              signature: -r
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.read.O13G4I|y7ouCK
              signature: ref
          - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B
            signature: do
            children:
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA
              signature: "[ -z \"$ref\" ] && continue"
              children:
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD
                signature: "[ -z \"$ref\" ]"
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD.ref
                  signature: -z "$ref"
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD.ref.ruAkts|cOm2pG
                    signature: -z
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD.ref.CbAsCO|6toQ2s
                    signature: '"$ref"'
                    children:
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD.ref.CbAsCO|6toQ2s.Tca5Zz|g8YcO4
                      signature: $ref
                      children:
                      - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.i3UdNM|YtV2OD.ref.CbAsCO|6toQ2s.Tca5Zz|g8YcO4.O13G4I|y7ouCK
                        signature: ref
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.continue
                signature: continue
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.continue.2pEJgY|IptZVm
                  signature: continue
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.iRTXNs|z9ABAA.continue.2pEJgY|IptZVm.2pEJgY|IptZVm
                    signature: continue
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num
              signature: "num=\"${ref##*/op-}\""
              children:
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num.m7guaN|F8c4Oq
                signature: num
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num.IFxUcZ|6bAUJu
                signature: "\"${ref##*/op-}\""
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num.IFxUcZ|6bAUJu.AmksKf|z8RaXZ
                  signature: "${ref##*/op-}"
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num.IFxUcZ|6bAUJu.AmksKf|z8RaXZ.O13G4I|y7ouCK
                    signature: ref
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.num.IFxUcZ|6bAUJu.AmksKf|z8RaXZ.MzHg1k|U7MxSF
                    signature: '*/op-'
            - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E
              signature: "if [[ \"$num\" =~ ^[0-9]+$ ]] && [ \"$num\" -gt \"$LAST_N\" ]; then"
              children:
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa
                signature: "[[ \"$num\" =~ ^[0-9]+$ ]] && [ \"$num\" -gt \"$LAST_N\" ]"
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF
                  signature: "[[ \"$num\" =~ ^[0-9]+$ ]]"
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF.num
                    signature: "\"$num\" =~ ^[0-9]+$"
                    children:
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF.num.F98Vn1|1yzYEY
                      signature: '"$num"'
                      children:
                      - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF.num.F98Vn1|1yzYEY.xfYciu|7cfLSf
                        signature: $num
                        children:
                        - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF.num.F98Vn1|1yzYEY.xfYciu|7cfLSf.m7guaN|F8c4Oq
                          signature: num
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.RQsAzm|g7jFJF.num.htWehf|LFByva
                      signature: "^[0-9]+$"
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V
                  signature: "[ \"$num\" -gt \"$LAST_N\" ]"
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num
                    signature: '"$num" -gt "$LAST_N"'
                    children:
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.F98Vn1|1yzYEY
                      signature: '"$num"'
                      children:
                      - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.F98Vn1|1yzYEY.xfYciu|7cfLSf
                        signature: $num
                        children:
                        - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.F98Vn1|1yzYEY.xfYciu|7cfLSf.m7guaN|F8c4Oq
                          signature: num
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.r3MLjW|IHZrhw
                      signature: -gt
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.z9IeOM|yLS4mf
                      signature: '"$LAST_N"'
                      children:
                      - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.z9IeOM|yLS4mf.Ju3c5P|etxEuB
                        signature: $LAST_N
                        children:
                        - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.dLAPDb|4c6rqa.iIMBdx|iNYb5V.num.z9IeOM|yLS4mf.Ju3c5P|etxEuB.qkWEnr|YVdbfl
                          signature: LAST_N
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_N
                signature: LAST_N="$num"
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_N.qkWEnr|YVdbfl
                  signature: LAST_N
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_N.F98Vn1|1yzYEY
                  signature: '"$num"'
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_N.F98Vn1|1yzYEY.xfYciu|7cfLSf
                    signature: $num
                    children:
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_N.F98Vn1|1yzYEY.xfYciu|7cfLSf.m7guaN|F8c4Oq
                      signature: num
              - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_OP_REF
                signature: LAST_OP_REF="$ref"
                children:
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_OP_REF.nYpTVo|v6Ci0D
                  signature: LAST_OP_REF
                - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_OP_REF.CbAsCO|6toQ2s
                  signature: '"$ref"'
                  children:
                  - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_OP_REF.CbAsCO|6toQ2s.Tca5Zz|g8YcO4
                    signature: $ref
                    children:
                    - id: s0Twu8|yGRhW0.VMocSZ|fxOb8R.XC0pHF|thUy9B.wzAUJj|w2Ws9E.LAST_OP_REF.CbAsCO|6toQ2s.Tca5Zz|g8YcO4.O13G4I|y7ouCK
                      signature: ref
        - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I
          signature: "< <(git for-each-ref --format='%(refname)' \"${NAMESPACE}/op-*\" 2>/dev/null || t…"
          children:
          - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ
            signature: "<(git for-each-ref --format='%(refname)' \"${NAMESPACE}/op-*\" 2>/dev/null || tru…"
            children:
            - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26
              signature: "git for-each-ref --format='%(refname)' \"${NAMESPACE}/op-*\" 2>/dev/null || true"
              children:
              - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp
                signature: "git for-each-ref --format='%(refname)' \"${NAMESPACE}/op-*\" 2>/dev/null"
                children:
                - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git
                  signature: "git for-each-ref --format='%(refname)' \"${NAMESPACE}/op-*\""
                  children:
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.KFE4Bh|L4z8rV
                    signature: git
                    children:
                    - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                      signature: git
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.8yA8Sm|4o2ttQ
                    signature: for-each-ref
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.DAJNmo|Tm19Mg
                    signature: --format='%(refname)'
                    children:
                    - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.DAJNmo|Tm19Mg.51wYjo|Z7CpOu
                      signature: --format=
                    - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.DAJNmo|Tm19Mg.bulxZ7|i52UwN
                      signature: '''%(refname)'''
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.uvbF0B|HEqCUg
                    signature: "\"${NAMESPACE}/op-*\""
                    children:
                    - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.uvbF0B|HEqCUg.Xlinsf|DLQbeI
                      signature: "${NAMESPACE}"
                      children:
                      - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.uvbF0B|HEqCUg.Xlinsf|DLQbeI.DgdUD6|k4yVO4
                        signature: NAMESPACE
                    - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.git.uvbF0B|HEqCUg.x8cv5O|3Hq0XY
                      signature: /op-*
                - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.JErUXN|Lr9GaI
                  signature: 2>/dev/null
                  children:
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.JErUXN|Lr9GaI.PEvZmA|I4fuEo
                    signature: 2
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.c4NlGM|WlDHGp.JErUXN|Lr9GaI.69AD4l|mb2flb
                    signature: /dev/null
              - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.true
                signature: "true"
                children:
                - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.true.HhJDNe|PON7aF
                  signature: "true"
                  children:
                  - id: s0Twu8|yGRhW0.7ENOnZ|5bwv9I.phPoU4|ZL3GlZ.Y6fDrX|xeDd26.true.HhJDNe|PON7aF.HhJDNe|PON7aF
                    signature: "true"
      - id: Q5tAmt|5cUQOX
        signature: "if [ -n \"$LAST_OP_REF\" ]; then"
        children:
        - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh
          signature: "[ -n \"$LAST_OP_REF\" ]"
          children:
          - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh.LAST_OP_REF
            signature: -n "$LAST_OP_REF"
            children:
            - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh.LAST_OP_REF.swpt2o|G8cS18
              signature: -n
            - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh.LAST_OP_REF.25kMws|m3IqPs
              signature: '"$LAST_OP_REF"'
              children:
              - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh.LAST_OP_REF.25kMws|m3IqPs.97NAE4|bAJcTR
                signature: $LAST_OP_REF
                children:
                - id: Q5tAmt|5cUQOX.lIosFY|QM8NWh.LAST_OP_REF.25kMws|m3IqPs.97NAE4|bAJcTR.nYpTVo|v6Ci0D
                  signature: LAST_OP_REF
        - id: Q5tAmt|5cUQOX.PARENT
          signature: PARENT=$(git rev-parse "$LAST_OP_REF")
          children:
          - id: Q5tAmt|5cUQOX.PARENT.pNJcZI|7rwzCc
            signature: PARENT
          - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L
            signature: $(git rev-parse "$LAST_OP_REF")
            children:
            - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git
              signature: git rev-parse "$LAST_OP_REF"
              children:
              - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.KFE4Bh|L4z8rV
                signature: git
                children:
                - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                  signature: git
              - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.gDXhg0|DIlGir
                signature: rev-parse
              - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.25kMws|m3IqPs
                signature: '"$LAST_OP_REF"'
                children:
                - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.25kMws|m3IqPs.97NAE4|bAJcTR
                  signature: $LAST_OP_REF
                  children:
                  - id: Q5tAmt|5cUQOX.PARENT.OKNWvP|abyL1L.git.25kMws|m3IqPs.97NAE4|bAJcTR.nYpTVo|v6Ci0D
                    signature: LAST_OP_REF
        - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf
          signature: else
          children:
          - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf.PARENT
            signature: PARENT="$STORED_BASE"
            children:
            - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf.PARENT.pNJcZI|7rwzCc
              signature: PARENT
            - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf.PARENT.pm7TxN|0CJh43
              signature: '"$STORED_BASE"'
              children:
              - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf.PARENT.pm7TxN|0CJh43.gLJ3QD|Qt28Oc
                signature: $STORED_BASE
                children:
                - id: Q5tAmt|5cUQOX.vYPGP2|Qq3fWf.PARENT.pm7TxN|0CJh43.gLJ3QD|Qt28Oc.is8knb|BOm3kR
                  signature: STORED_BASE
      - id: "N"
        signature: N=$((LAST_N + 1))
        children:
        - id: N.E4HBFU|Ziyhzx
          signature: "N"
        - id: N.XiILTV|1xy0a5
          signature: $((LAST_N + 1))
          children:
          - id: N.XiILTV|1xy0a5.Jvgjst|8WjUJh
            signature: LAST_N + 1
            children:
            - id: N.XiILTV|1xy0a5.Jvgjst|8WjUJh.qkWEnr|YVdbfl
              signature: LAST_N
            - id: N.XiILTV|1xy0a5.Jvgjst|8WjUJh.L8Zvvc|lVqSnb
              signature: 1
      - id: TIMESTAMP
        signature: TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
        children:
        - id: TIMESTAMP.7EY6I8|YZmxcZ
          signature: TIMESTAMP
        - id: TIMESTAMP.lGdzgV|TX1MO5
          signature: $(date '+%Y-%m-%d %H:%M:%S')
          children:
          - id: TIMESTAMP.lGdzgV|TX1MO5.date
            signature: date '+%Y-%m-%d %H:%M:%S'
            children:
            - id: TIMESTAMP.lGdzgV|TX1MO5.date.1AZm21|OYRN7F
              signature: date
              children:
              - id: TIMESTAMP.lGdzgV|TX1MO5.date.1AZm21|OYRN7F.1AZm21|OYRN7F
                signature: date
            - id: TIMESTAMP.lGdzgV|TX1MO5.date.0oEQje|JRxqBM
              signature: '''+%Y-%m-%d %H:%M:%S'''
      - id: MSG
        signature: "MSG=\"op-$N: $TIMESTAMP\""
        children:
        - id: MSG.tFU5N3|MPlfMj
          signature: MSG
        - id: MSG.igTbrH|IDnrTb
          signature: "\"op-$N: $TIMESTAMP\""
          children:
          - id: MSG.igTbrH|IDnrTb.5tDVKt|j0lUmz
            signature: op-
          - id: MSG.igTbrH|IDnrTb.UcDxVi|khDLPC
            signature: $N
            children:
            - id: MSG.igTbrH|IDnrTb.UcDxVi|khDLPC.E4HBFU|Ziyhzx
              signature: "N"
          - id: MSG.igTbrH|IDnrTb.jLSS06|t1hpAL
            signature: ":"
          - id: MSG.igTbrH|IDnrTb.NO0DFr|vGMsQw
            signature: $TIMESTAMP
            children:
            - id: MSG.igTbrH|IDnrTb.NO0DFr|vGMsQw.7EY6I8|YZmxcZ
              signature: TIMESTAMP
      - id: 03SLx8|OXWjRU
        signature: "# --- Build the snapshot --------------------------------------------------"
      - id: git
        signature: git add -A
        children:
        - id: git.KFE4Bh|L4z8rV
          signature: git
          children:
          - id: git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
            signature: git
        - id: git.b8cjS2|5JwCHu
          signature: add
        - id: git.vL54Cs|QYFaMH
          signature: -A
      - id: TREE
        signature: TREE=$(git write-tree)
        children:
        - id: TREE.H5DT9I|zU1shG
          signature: TREE
        - id: TREE.Bfddpt|8NB6Uq
          signature: $(git write-tree)
          children:
          - id: TREE.Bfddpt|8NB6Uq.git
            signature: git write-tree
            children:
            - id: TREE.Bfddpt|8NB6Uq.git.KFE4Bh|L4z8rV
              signature: git
              children:
              - id: TREE.Bfddpt|8NB6Uq.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                signature: git
            - id: TREE.Bfddpt|8NB6Uq.git.Ruz9Rd|sAVlOi
              signature: write-tree
      - id: PARENT_TREE
        signature: "PARENT_TREE=$(git rev-parse \"${PARENT}^{tree}\")"
        children:
        - id: PARENT_TREE.nTs0md|cEbyGk
          signature: PARENT_TREE
        - id: PARENT_TREE.qOP2Jz|1u0H2T
          signature: "$(git rev-parse \"${PARENT}^{tree}\")"
          children:
          - id: PARENT_TREE.qOP2Jz|1u0H2T.git
            signature: "git rev-parse \"${PARENT}^{tree}\""
            children:
            - id: PARENT_TREE.qOP2Jz|1u0H2T.git.KFE4Bh|L4z8rV
              signature: git
              children:
              - id: PARENT_TREE.qOP2Jz|1u0H2T.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                signature: git
            - id: PARENT_TREE.qOP2Jz|1u0H2T.git.gDXhg0|DIlGir
              signature: rev-parse
            - id: PARENT_TREE.qOP2Jz|1u0H2T.git.2wOal0|UwJLiv
              signature: "\"${PARENT}^{tree}\""
              children:
              - id: PARENT_TREE.qOP2Jz|1u0H2T.git.2wOal0|UwJLiv.RoZvx4|TqpW2y
                signature: "${PARENT}"
                children:
                - id: PARENT_TREE.qOP2Jz|1u0H2T.git.2wOal0|UwJLiv.RoZvx4|TqpW2y.pNJcZI|7rwzCc
                  signature: PARENT
              - id: PARENT_TREE.qOP2Jz|1u0H2T.git.2wOal0|UwJLiv.Snqqhy|6nmwZe
                signature: "^{tree}"
      - id: PORgvf|MbiCFn
        signature: "if [ \"$TREE\" = \"$PARENT_TREE\" ]; then"
        children:
        - id: PORgvf|MbiCFn.yzNq5A|4hcOzX
          signature: "[ \"$TREE\" = \"$PARENT_TREE\" ]"
          children:
          - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE
            signature: '"$TREE" = "$PARENT_TREE"'
            children:
            - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.azqNCz|N82HXt
              signature: '"$TREE"'
              children:
              - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.azqNCz|N82HXt.OwUNdV|9h4eG5
                signature: $TREE
                children:
                - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.azqNCz|N82HXt.OwUNdV|9h4eG5.H5DT9I|zU1shG
                  signature: TREE
            - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.BWcker|YzO3xI
              signature: '"$PARENT_TREE"'
              children:
              - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.BWcker|YzO3xI.qBxs5v|pWbhAb
                signature: $PARENT_TREE
                children:
                - id: PORgvf|MbiCFn.yzNq5A|4hcOzX.TREE.BWcker|YzO3xI.qBxs5v|pWbhAb.nTs0md|cEbyGk
                  signature: PARENT_TREE
        - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1
          signature: echo "No changes since last snapshot ($PARENT) — no new commit created." >&2
          children:
          - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo
            signature: echo "No changes since last snapshot ($PARENT) — no new commit created."
            children:
            - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.vhMnXv|TcYAkX
              signature: echo
              children:
              - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
                signature: echo
            - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.hZaZIs|WWWBGQ
              signature: '"No changes since last snapshot ($PARENT) — no new commit created."'
              children:
              - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.hZaZIs|WWWBGQ.C8gRH3|oNIoUw
                signature: No changes since last snapshot (
              - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.hZaZIs|WWWBGQ.ESOhpf|YdrpeJ
                signature: $PARENT
                children:
                - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.hZaZIs|WWWBGQ.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
                  signature: PARENT
              - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.echo.hZaZIs|WWWBGQ.rnRIVC|YL1WR7
                signature: ) — no new commit created.
          - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.Rh7ZNz|66tcI2
            signature: '>&2'
            children:
            - id: PORgvf|MbiCFn.KKkL2O|Pr0ra1.Rh7ZNz|66tcI2.PEvZmA|I4fuEo
              signature: 2
        - id: PORgvf|MbiCFn.echo
          signature: echo "SNAPSHOT_REF="
          children:
          - id: PORgvf|MbiCFn.echo.vhMnXv|TcYAkX
            signature: echo
            children:
            - id: PORgvf|MbiCFn.echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
              signature: echo
          - id: PORgvf|MbiCFn.echo.g1F80t|IJXQVC
            signature: '"SNAPSHOT_REF="'
            children:
            - id: PORgvf|MbiCFn.echo.g1F80t|IJXQVC.tsN4c0|KyD2I7
              signature: SNAPSHOT_REF=
        - id: PORgvf|MbiCFn.echo_1
          signature: echo "SNAPSHOT_COMMIT=$PARENT"
          children:
          - id: PORgvf|MbiCFn.echo_1.vhMnXv|TcYAkX
            signature: echo
            children:
            - id: PORgvf|MbiCFn.echo_1.vhMnXv|TcYAkX.vhMnXv|TcYAkX
              signature: echo
          - id: PORgvf|MbiCFn.echo_1.b1gl25|1ysmFH
            signature: '"SNAPSHOT_COMMIT=$PARENT"'
            children:
            - id: PORgvf|MbiCFn.echo_1.b1gl25|1ysmFH.71WroN|iAyl7L
              signature: SNAPSHOT_COMMIT=
            - id: PORgvf|MbiCFn.echo_1.b1gl25|1ysmFH.ESOhpf|YdrpeJ
              signature: $PARENT
              children:
              - id: PORgvf|MbiCFn.echo_1.b1gl25|1ysmFH.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
                signature: PARENT
        - id: PORgvf|MbiCFn.echo_2
          signature: echo "SNAPSHOT_PARENT=$PARENT"
          children:
          - id: PORgvf|MbiCFn.echo_2.vhMnXv|TcYAkX
            signature: echo
            children:
            - id: PORgvf|MbiCFn.echo_2.vhMnXv|TcYAkX.vhMnXv|TcYAkX
              signature: echo
          - id: PORgvf|MbiCFn.echo_2.PNYyji|TcFg83
            signature: '"SNAPSHOT_PARENT=$PARENT"'
            children:
            - id: PORgvf|MbiCFn.echo_2.PNYyji|TcFg83.fUUaDW|f5E2qU
              signature: SNAPSHOT_PARENT=
            - id: PORgvf|MbiCFn.echo_2.PNYyji|TcFg83.ESOhpf|YdrpeJ
              signature: $PARENT
              children:
              - id: PORgvf|MbiCFn.echo_2.PNYyji|TcFg83.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
                signature: PARENT
        - id: PORgvf|MbiCFn.echo_3
          signature: echo "SNAPSHOT_CHAIN=$NAMESPACE"
          children:
          - id: PORgvf|MbiCFn.echo_3.vhMnXv|TcYAkX
            signature: echo
            children:
            - id: PORgvf|MbiCFn.echo_3.vhMnXv|TcYAkX.vhMnXv|TcYAkX
              signature: echo
          - id: PORgvf|MbiCFn.echo_3.AiwdsE|BaJrPF
            signature: '"SNAPSHOT_CHAIN=$NAMESPACE"'
            children:
            - id: PORgvf|MbiCFn.echo_3.AiwdsE|BaJrPF.HoHXtE|uIjz3L
              signature: SNAPSHOT_CHAIN=
            - id: PORgvf|MbiCFn.echo_3.AiwdsE|BaJrPF.9EJpkX|tdkX2M
              signature: $NAMESPACE
              children:
              - id: PORgvf|MbiCFn.echo_3.AiwdsE|BaJrPF.9EJpkX|tdkX2M.DgdUD6|k4yVO4
                signature: NAMESPACE
        - id: PORgvf|MbiCFn.exit
          signature: exit 0
          children:
          - id: PORgvf|MbiCFn.exit.ECuVlZ|g5BskS
            signature: exit
            children:
            - id: PORgvf|MbiCFn.exit.ECuVlZ|g5BskS.ECuVlZ|g5BskS
              signature: exit
          - id: PORgvf|MbiCFn.exit.QDjlfP|EoJOXM
            signature: 0
      - id: NEW_COMMIT
        signature: NEW_COMMIT=$(git commit-tree "$TREE" -p "$PARENT" -m "$MSG")
        children:
        - id: NEW_COMMIT.O6RSbC|sspQre
          signature: NEW_COMMIT
        - id: NEW_COMMIT.JxxzOK|qyHPU3
          signature: $(git commit-tree "$TREE" -p "$PARENT" -m "$MSG")
          children:
          - id: NEW_COMMIT.JxxzOK|qyHPU3.git
            signature: git commit-tree "$TREE" -p "$PARENT" -m "$MSG"
            children:
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.KFE4Bh|L4z8rV
              signature: git
              children:
              - id: NEW_COMMIT.JxxzOK|qyHPU3.git.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
                signature: git
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.Kw5Z2Z|p5FT1x
              signature: commit-tree
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.azqNCz|N82HXt
              signature: '"$TREE"'
              children:
              - id: NEW_COMMIT.JxxzOK|qyHPU3.git.azqNCz|N82HXt.OwUNdV|9h4eG5
                signature: $TREE
                children:
                - id: NEW_COMMIT.JxxzOK|qyHPU3.git.azqNCz|N82HXt.OwUNdV|9h4eG5.H5DT9I|zU1shG
                  signature: TREE
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.kmWB7d|t6XWJ2
              signature: -p
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.4gqJeJ|36XQqo
              signature: '"$PARENT"'
              children:
              - id: NEW_COMMIT.JxxzOK|qyHPU3.git.4gqJeJ|36XQqo.ESOhpf|YdrpeJ
                signature: $PARENT
                children:
                - id: NEW_COMMIT.JxxzOK|qyHPU3.git.4gqJeJ|36XQqo.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
                  signature: PARENT
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.r5Lkf9|MfgAcI
              signature: -m
            - id: NEW_COMMIT.JxxzOK|qyHPU3.git.05Nx3X|wdELA1
              signature: '"$MSG"'
              children:
              - id: NEW_COMMIT.JxxzOK|qyHPU3.git.05Nx3X|wdELA1.11V7IO|Z4Z54k
                signature: $MSG
                children:
                - id: NEW_COMMIT.JxxzOK|qyHPU3.git.05Nx3X|wdELA1.11V7IO|Z4Z54k.tFU5N3|MPlfMj
                  signature: MSG
      - id: REF_NAME
        signature: "REF_NAME=\"${NAMESPACE}/op-$N\""
        children:
        - id: REF_NAME.zreQBp|nosKiz
          signature: REF_NAME
        - id: REF_NAME.G4MqRo|TEY1u2
          signature: "\"${NAMESPACE}/op-$N\""
          children:
          - id: REF_NAME.G4MqRo|TEY1u2.Xlinsf|DLQbeI
            signature: "${NAMESPACE}"
            children:
            - id: REF_NAME.G4MqRo|TEY1u2.Xlinsf|DLQbeI.DgdUD6|k4yVO4
              signature: NAMESPACE
          - id: REF_NAME.G4MqRo|TEY1u2.XzgTxC|gY0MC3
            signature: /op-
          - id: REF_NAME.G4MqRo|TEY1u2.UcDxVi|khDLPC
            signature: $N
            children:
            - id: REF_NAME.G4MqRo|TEY1u2.UcDxVi|khDLPC.E4HBFU|Ziyhzx
              signature: "N"
      - id: git_1
        signature: git update-ref "$REF_NAME" "$NEW_COMMIT"
        children:
        - id: git_1.KFE4Bh|L4z8rV
          signature: git
          children:
          - id: git_1.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
            signature: git
        - id: git_1.GuDOQg|r6294T
          signature: update-ref
        - id: git_1.2zW7kO|cAhz9D
          signature: '"$REF_NAME"'
          children:
          - id: git_1.2zW7kO|cAhz9D.SJp3WQ|cwN606
            signature: $REF_NAME
            children:
            - id: git_1.2zW7kO|cAhz9D.SJp3WQ|cwN606.zreQBp|nosKiz
              signature: REF_NAME
        - id: git_1.cANExS|l27HgY
          signature: '"$NEW_COMMIT"'
          children:
          - id: git_1.cANExS|l27HgY.7YlGx2|gAic0h
            signature: $NEW_COMMIT
            children:
            - id: git_1.cANExS|l27HgY.7YlGx2|gAic0h.O6RSbC|sspQre
              signature: NEW_COMMIT
      - id: mHIn0j|WXj6wr
        signature: "# --- Output ---------------------------------------------------------------"
      - id: echo
        signature: echo "----------------------------------------------------------------------"
        children:
        - id: echo.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo.MFu5fe|4pGbQc
          signature: '"----------------------------------------------------------------------"'
          children:
          - id: echo.MFu5fe|4pGbQc.680OHI|ohotc8
            signature: '----------------------------------------------------------------------'
      - id: echo_1
        signature: "echo \"Snapshot created: $REF_NAME -> $NEW_COMMIT\""
        children:
        - id: echo_1.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_1.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_1.womDba|vYZwLF
          signature: "\"Snapshot created: $REF_NAME -> $NEW_COMMIT\""
          children:
          - id: echo_1.womDba|vYZwLF.kC3r6i|ohx5u8
            signature: "Snapshot created:"
          - id: echo_1.womDba|vYZwLF.SJp3WQ|cwN606
            signature: $REF_NAME
            children:
            - id: echo_1.womDba|vYZwLF.SJp3WQ|cwN606.zreQBp|nosKiz
              signature: REF_NAME
          - id: echo_1.womDba|vYZwLF.kc5CUQ|yn3QB4
            signature: ->
          - id: echo_1.womDba|vYZwLF.7YlGx2|gAic0h
            signature: $NEW_COMMIT
            children:
            - id: echo_1.womDba|vYZwLF.7YlGx2|gAic0h.O6RSbC|sspQre
              signature: NEW_COMMIT
      - id: echo_2
        signature: "echo \"Parent:           $PARENT\""
        children:
        - id: echo_2.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_2.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_2.phwEKC|PK2PMT
          signature: "\"Parent:           $PARENT\""
          children:
          - id: echo_2.phwEKC|PK2PMT.PHXdPH|0BNd8M
            signature: "Parent:"
          - id: echo_2.phwEKC|PK2PMT.ESOhpf|YdrpeJ
            signature: $PARENT
            children:
            - id: echo_2.phwEKC|PK2PMT.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
              signature: PARENT
      - id: echo_3
        signature: "echo \"Chain base:       $STORED_BASE\""
        children:
        - id: echo_3.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_3.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_3.RUmcz1|HeHota
          signature: "\"Chain base:       $STORED_BASE\""
          children:
          - id: echo_3.RUmcz1|HeHota.Twn66g|Q2yTSj
            signature: "Chain base:"
          - id: echo_3.RUmcz1|HeHota.gLJ3QD|Qt28Oc
            signature: $STORED_BASE
            children:
            - id: echo_3.RUmcz1|HeHota.gLJ3QD|Qt28Oc.is8knb|BOm3kR
              signature: STORED_BASE
      - id: echo_4
        signature: echo "----------------------------------------------------------------------"
        children:
        - id: echo_4.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_4.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_4.MFu5fe|4pGbQc
          signature: '"----------------------------------------------------------------------"'
          children:
          - id: echo_4.MFu5fe|4pGbQc.680OHI|ohotc8
            signature: '----------------------------------------------------------------------'
      - id: git_2
        signature: git diff "$PARENT" "$NEW_COMMIT"
        children:
        - id: git_2.KFE4Bh|L4z8rV
          signature: git
          children:
          - id: git_2.KFE4Bh|L4z8rV.KFE4Bh|L4z8rV
            signature: git
        - id: git_2.cXVuwz|aNLPbV
          signature: diff
        - id: git_2.4gqJeJ|36XQqo
          signature: '"$PARENT"'
          children:
          - id: git_2.4gqJeJ|36XQqo.ESOhpf|YdrpeJ
            signature: $PARENT
            children:
            - id: git_2.4gqJeJ|36XQqo.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
              signature: PARENT
        - id: git_2.cANExS|l27HgY
          signature: '"$NEW_COMMIT"'
          children:
          - id: git_2.cANExS|l27HgY.7YlGx2|gAic0h
            signature: $NEW_COMMIT
            children:
            - id: git_2.cANExS|l27HgY.7YlGx2|gAic0h.O6RSbC|sspQre
              signature: NEW_COMMIT
      - id: echo_5
        signature: echo "SNAPSHOT_REF=$REF_NAME"
        children:
        - id: echo_5.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_5.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_5.o7HrkP|RbKDQU
          signature: '"SNAPSHOT_REF=$REF_NAME"'
          children:
          - id: echo_5.o7HrkP|RbKDQU.tsN4c0|KyD2I7
            signature: SNAPSHOT_REF=
          - id: echo_5.o7HrkP|RbKDQU.SJp3WQ|cwN606
            signature: $REF_NAME
            children:
            - id: echo_5.o7HrkP|RbKDQU.SJp3WQ|cwN606.zreQBp|nosKiz
              signature: REF_NAME
      - id: echo_6
        signature: echo "SNAPSHOT_COMMIT=$NEW_COMMIT"
        children:
        - id: echo_6.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_6.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_6.FQJDer|uoHlXR
          signature: '"SNAPSHOT_COMMIT=$NEW_COMMIT"'
          children:
          - id: echo_6.FQJDer|uoHlXR.71WroN|iAyl7L
            signature: SNAPSHOT_COMMIT=
          - id: echo_6.FQJDer|uoHlXR.7YlGx2|gAic0h
            signature: $NEW_COMMIT
            children:
            - id: echo_6.FQJDer|uoHlXR.7YlGx2|gAic0h.O6RSbC|sspQre
              signature: NEW_COMMIT
      - id: echo_7
        signature: echo "SNAPSHOT_PARENT=$PARENT"
        children:
        - id: echo_7.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_7.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_7.PNYyji|TcFg83
          signature: '"SNAPSHOT_PARENT=$PARENT"'
          children:
          - id: echo_7.PNYyji|TcFg83.fUUaDW|f5E2qU
            signature: SNAPSHOT_PARENT=
          - id: echo_7.PNYyji|TcFg83.ESOhpf|YdrpeJ
            signature: $PARENT
            children:
            - id: echo_7.PNYyji|TcFg83.ESOhpf|YdrpeJ.pNJcZI|7rwzCc
              signature: PARENT
      - id: echo_8
        signature: echo "SNAPSHOT_CHAIN=$NAMESPACE"
        children:
        - id: echo_8.vhMnXv|TcYAkX
          signature: echo
          children:
          - id: echo_8.vhMnXv|TcYAkX.vhMnXv|TcYAkX
            signature: echo
        - id: echo_8.AiwdsE|BaJrPF
          signature: '"SNAPSHOT_CHAIN=$NAMESPACE"'
          children:
          - id: echo_8.AiwdsE|BaJrPF.HoHXtE|uIjz3L
            signature: SNAPSHOT_CHAIN=
          - id: echo_8.AiwdsE|BaJrPF.9EJpkX|tdkX2M
            signature: $NAMESPACE
            children:
            - id: echo_8.AiwdsE|BaJrPF.9EJpkX|tdkX2M.DgdUD6|k4yVO4
              signature: NAMESPACE
```
/answer b655-6 allow
ReasoningToken: 216
ReasoningToken: 104
Control Request:
```yaml
id: b655-7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md
    ids:
    - 0ce91a
  reason: "Vollständigen Inhalt des Implementation-Prompt-Dokuments lesen, um Stil und Format zu übernehmen."
```
/answer b655-7 allow
Control Request:
```yaml
id: b655-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md
      nodes:
      - id: 0ce91a
        type: section
        signature: "Implementation Prompt: Git-Based LLM Operation Snapshot & Review System"
        children:
        - id: 0ce91a.cc11b3
          type: section
          signature: Context
          children:
          - id: 0ce91a.cc11b3.AlBcjD|aIGtik
            type: paragraph
            code: |
              An Eclipse plugin runs an LLM agent that performs file operations
              (create/edit/delete) directly on a Git-tracked workspace. Before/after every
              LLM operation, we need a Human-in-the-Loop (HITL) review step: show the
              exact diff, let the user accept or revert it — without polluting the real
              Git history (no commits on the actual branch, no HEAD movement, no index
              side effects visible to normal Git tooling).
          - id: 0ce91a.cc11b3.k7N3V1|YP7slR
            type: paragraph
            code: |
              The chosen mechanism uses Git's own object model (trees + commits) as an
              out-of-band, GC-safe snapshot log under a dedicated ref namespace, wrapped
              in commit objects so they remain visible in EGit's native UI (Additional
              Refs, History view).
        - id: 0ce91a.7bb0dd
          type: section
          signature: Task
          children:
          - id: 0ce91a.7bb0dd.gBsQgI|IT0TdP
            type: paragraph
            code: |
              Implement this system as part of the existing Eclipse plugin, in Java using
              JGit. The plugin already contains the hooks/triggers that fire immediately
              before and after each LLM operation — integrate against those, do not
              reinvent the triggering mechanism.
        - id: 0ce91a.bca3e8
          type: section
          code: |+
            ## Functional Requirements

            1. **Snapshot creation (post-operation hook)**
               - After each LLM operation, capture the full working-tree state as a Git
                 tree object and wrap it in a commit object.
               - Do not move HEAD, the current branch, or the real index in any
                 user-visible way.
               - Commit message is fixed-format: Tool Call ID that caused the snapshot (no free-text
                 description).
               - Store the resulting commit under a dedicated ref namespace
                 (`refs/llm-ops/...`), never as a loose/unreferenced object.

            2. **Chain logic**
               - Maintain a sequential parent chain: each new snapshot's parent is the
                 previous snapshot in the same chain, not a fixed base commit.
               - Detect if HEAD has advanced since the last snapshot (i.e., a real
                 commit happened on the branch in the meantime). If so, close the
                 current chain and start a new one, rooted at the new HEAD. Do not
                 delete or modify the old chain.
               - Guard against no-op snapshots: if the working tree is unchanged since
                 the last snapshot, skip commit creation.

            3. **Diff generation**
               - Produce a single aggregated unified diff for the entire operation
                 (all changed files in one text block), equivalent to
                 `git diff <parent> <commit>` — not per-file fragments.

            4. **Revert**
               - Provide a revert operation that restores the working tree and index
                 exactly to a given snapshot's tree state.
               - Must not use a mechanism that moves HEAD or the branch (i.e., no
                 `git reset --hard` / JGit `ResetCommand`).
               - Must remove files created by the operation that are not part of the
                 target tree (untracked-file cleanup after revert).

            5. **Session inspection / export**
               - All snapshots across all chains must be listable (ref enumeration).
               - Support exporting the full snapshot history as a single portable file.
               - Support explicit cleanup (deleting refs) at session end, without
                 affecting the real branch/remote in any way.
               - Confirm no snapshot refs are ever pushed to a remote by default.

            6. **Eclipse UI integration**
               - A **singleton** panel (`ViewPart`, not `EditorPart`) that:
                 - Shows all chains and their snapshots in a tree/list (top level:
                   chain, children: individual operations).
                 - On selection, displays the full aggregated unified diff for that
                   operation in a dedicated diff area.
                 - Vertical layout: ref/chain section on top, diff section below.
                 - Provides a way to trigger revert to a selected snapshot from the
                   panel.
               - Must not open a new editor tab per commit (this is why a custom
                 singleton view is required instead of invoking EGit's built-in commit
                 editor directly).
               - Reuse EGit/JGit facilities where possible instead of reimplementing
                 diff computation or Git plumbing:
                 - `DiffFormatter` for unified diff text generation.
                 - `IHistoryView` / Additional Refs for cross-checking/inspection where
                   useful, without depending on EGit's non-exported internal classes.

        - id: 0ce91a.d9d570
          type: section
          code: |+
            ## Non-Functional / Robustness Requirements

            - No dependency on Git CLI subprocess calls for the core snapshot/diff/
              revert logic — pure JGit, in-process.
            - Every snapshot ref must point to a **commit object**, never a bare tree,
              so it renders correctly in EGit's Additional Refs / History view.
            - Snapshot creation, diff generation, and revert must be robust against:
              - concurrent external Git operations happening outside the LLM
                operation's pre/post hook window (document the assumption if not
                handled),
              - large numbers of changed files per operation (avoid excessive per-file
                widget instantiation in the UI),
              - long-running sessions accumulating many refs (avoid unbounded resource
                growth without a cleanup path).
            - Avoid dependence on internal/non-exported EGit packages
              (`org.eclipse.egit.ui.internal.*`); prefer public JGit API
              (`org.eclipse.jgit.*`) and public Eclipse Platform Team API
              (`org.eclipse.team.ui.history.*`) wherever functionality overlaps.
            - Code must compile as part of an existing Eclipse plugin (OSGi bundle),
              with JGit available as a dependency via the EGit bundle or an explicit
              `org.eclipse.jgit` dependency in `MANIFEST.MF`.

        - id: 0ce91a.c1a891
          type: section
          code: |+
            ## Explicitly Out of Scope

            - No dry-run/sandboxed execution model — operations run for real; review
              happens after execution via diff + optional revert.
            - No modification of the user's real branch, HEAD, index, or any pushed
              remote state.
            - No requirement to reconstruct arbitrary point-in-time diffs across chains
              via patch replay — chain snapshots are stored as full tree/commit
              objects, not as sequential patches.
            - No free-text commit messages or user-provided descriptions for
              snapshots.

        - id: 0ce91a.d2709d
          type: section
          signature: Reference Material
          children:
          - id: 0ce91a.d2709d.1W1d5X|Z0HdkB
            type: paragraph
            code: |
              See the accompanying dossier (`ops-git-snapshot-dossier.md`) for:
        - id: 0ce91a.ff3b5d
          type: section
          code: |
            ## Deliverables Expected From Implementation

            1. `OpSnapshotter` — snapshot creation with chain/base detection.
            2. `OpDiffReader` — aggregated unified diff generation.
            3. `OpReverter` — safe revert via `DirCacheCheckout` + clean.
            4. `OpsPanel` (singleton `ViewPart`) — chain/snapshot tree + diff display
               + revert action, wired into the existing plugin's pre/post-operation
               hooks.
            5. `plugin.xml` extension entries for the view registration.
```
/answer b655-8 allow
Control Request:
```yaml
id: b655-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/diff_view1.md
    ids:
    - tlq3tB|WBMHl7
    - 7n87mF|gUJDqX
  reason: Prüfen ob diese bestehende Datei bereits Anforderungen für dieses Feature enthält.
```
/answer b655-9 allow
Control Request:
```yaml
id: b655-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/diff_view1.md
      nodes:
      - id: tlq3tB|WBMHl7
        type: paragraph
        code: |
          Ich möchte eine Diff Übersicht für einen Agenten Prompt in meinem RCP-Plugin.
      - id: 7n87mF|gUJDqX
        type: list
        code: |
          - Die Erkennung eines `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java` oder `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java` soll nach dessen Ausführung in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java`, ein Snapshot triggern.
          - Das Snapshot soll mittels JGit ähnlich dem Script `/home/user/xyan/xy.ai.workbench/scripts/op-snapshot.sh` implementiert werden.
          - Der Snapshot prüft, ob es überhaupt ein Delta zum speichern gibt
          - Nach dem Snapshot löst ein Trigger einen Refresh im Diff-Pannels aus
          - Das Diff Panel zeigt stets das aktuellste Snapshot.

          - Zielpackage: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff`
          - Projekt: `/home/user/xyan/xy.ai.workbench`
          - Plugin: `/home/user/xyan/xy.ai.workbench/plugin.xml`

          - Snapshot Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-git-snapshot-dossier.md`
          - Diff Panel Implementierung: `/home/user/xyan/xy.ai.workbench/project/ops-implementation-prompt.md`
          SystemInit:  id=47287889-6c06-4fc7-b6cc-c51376dc7069, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
          ReasoningToken: 114
          Control Request:
```
/answer b655-a allow
Control Request:
```yaml
id: b655-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    ids:
    - AnswerCommand
  reason: Vollständige Implementierung von AnswerCommand lesen für die Integration des Snapshot-Triggers.
```
/answer b655-b allow
Control Request:
```yaml
id: b655-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
      nodes:
      - id: AnswerCommand
        type: class_declaration
        signature: "public class AnswerCommand extends Command {"
        children:
        - id: AnswerCommand.LN3qky|soKy4k
          type: statements
          code: public static final String CMD_ANSWER = "/answer";
        - id: AnswerCommand.AnswerCommand
          type: constructor_declaration
          code: "public AnswerCommand() {\n\t\tthis(\"\", \"\", \"\");\n\t}"
        - id: AnswerCommand.AnswerCommand_1
          type: constructor_declaration
          code: "public AnswerCommand(String id, String action, String reason) {\n\t\tsuper(id, action, reason);\n\t}"
        - id: AnswerCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn AnswerCommand.CMD_ANSWER;\n\t}"
        - id: AnswerCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\t// \"s\" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.\n\t\treturn line != null && line.strip().matches(\"(?is)\" + prefix() + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\");\n\t}"
        - id: AnswerCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\tString[] parts = line.strip().split(\"\\\\s+\", 4);\n\t\tString id = parts[1];\n\t\tString action = parts[2].toLowerCase();\n\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\treturn new AnswerCommand(id, action, reason);\n\t}"
        - id: AnswerCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}"
        - id: AnswerCommand.Action
          type: enum_declaration
          code: "public enum Action {\n\t\tAllow, Deny\n\t}"
        - id: AnswerCommand.action
          type: method_declaration
          code: "public Action action() {\n\t\treturn \"allow\".equalsIgnoreCase(parameter(1)) ? Action.Allow : Action.Deny;\n\t}"
```
/answer b655-c allow
Control Request:
```yaml
id: b655-d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
    ids:
    - ToolCommand
  reason: Vollständige Implementierung von ToolCommand lesen für die Integration des Snapshot-Triggers.
```
/answer b655-d allow
Control Request:
```yaml
id: b655-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
      nodes:
      - id: ToolCommand
        type: class_declaration
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.18LEdl|nBKSDD
          type: statements
          code: public static final String CMD_TOOL = "/tool";
        - id: ToolCommand.ToolCommand
          type: constructor_declaration
          code: "public ToolCommand() {\n\t\tthis(\"\");\n\t}"
        - id: ToolCommand.ToolCommand_1
          type: constructor_declaration
          code: "public ToolCommand(String toolId) {\n\t\tsuper(toolId);\n\t}"
        - id: ToolCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn ToolCommand.CMD_TOOL;\n\t}"
        - id: ToolCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}"
        - id: ToolCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}"
        - id: ToolCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
```
/answer b655-e allow
Control Request:
```yaml
id: b655-f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
  reason: "Vollständige Implementierung von PromptHandler lesen, um den passenden Trigger-Punkt für den Snapshot zu identifizieren."
```
/answer b655-f allow
Control Request:
```yaml
id: b655-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: class_declaration
        signature: "public class PromptHandler {"
        children:
        - id: PromptHandler.QEglBi|eRrE2Z
          type: statements
          code: "private final AdaptingConnector connector;\n\tprivate final EditorInterface editIfc;\n\tprivate final IncludeAdapter includeAdapter;\n\tprivate final AIBatchManager batch;\n\tprivate final PromptInputHandler input;\n\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();"
        - id: PromptHandler.PromptHandler
          type: constructor_declaration
          code: "public PromptHandler(ConfigManager cfg, ActiveEditorListener editorListener, AdaptingConnector connector,\n\t\t\tAIBatchManager batch, EditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.connector = connector;\n\t\tthis.editIfc = editIfc;\n\t\tthis.includeAdapter = includeAdapter;\n\t\tthis.batch = batch;\n\t\tthis.input = new PromptInputHandler(cfg, editorListener);\n\t}"
        - id: PromptHandler.addInputStatObs
          type: method_declaration
          code: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinput.addInputStatObs(obs, initialize);\n\t}"
        - id: PromptHandler.addAnswerObs
          type: method_declaration
          code: "public void addAnswerObs(Consumer<AIAnswer> obs) {\n\t\tanswerObs.add(obs);\n\t}"
        - id: PromptHandler.initializeInputs
          type: method_declaration
          code: "public void initializeInputs() {\n\t\tinput.initializeInputs();\n\t\tincludeAdapter.initializeBindings();\n\t}"
        - id: PromptHandler.execute
          type: method_declaration
          code: "public void execute(Display display) {\n\t\tnew PromptJob(\"Starting Prompt\", display).schedule();\n\t}"
        - id: PromptHandler.PromptJob
          type: class_declaration
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
            type: statements
            code: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            type: constructor_declaration
            code: "private PromptJob(String name, Display display) {\n\t\t\tsuper(name);\n\t\t\tthis.display = display;\n\t\t}"
          - id: PromptHandler.PromptJob.run
            type: method_declaration
            code: "@Override\n\t\tprotected IStatus run(IProgressMonitor mon) {\n\t\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 4);\n\t\t\tString reqId = null;\n\t\t\ttry {\n\t\t\t\tsub.subTask(\"Prepare inputs\");\n\t\t\t\tvar req = prepareRequest(display, false, sub);\n\t\t\t\tsub.worked(1);\n\t\t\t\tsub.subTask(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(display, req, sub);\n\t\t\t\treqId = req.getID();\n\t\t\t\tmon.worked(1);\n\n\t\t\t\tsub.subTask(\"Execute prompt\");\n\t\t\t\tvar ans = executeInner(display, req, sub, this);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(display, ans, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t// uncatched error case\n\t\t\t\tif (reqId != null) {\n\t\t\t\t\tAIAnswer error = new AIAnswer(reqId);\n\t\t\t\t\terror.answer = e.getMessage();\n\t\t\t\t\teditIfc.replaceTag(display, error, sub);\n\t\t\t\t}\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}"
        - id: PromptHandler.queueAsync
          type: method_declaration
          code: "public void queueAsync(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.queueSync
          type: method_declaration
          code: "private void queueSync(Display display, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareRequest(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
        - id: PromptHandler.queueAndSubmit
          type: method_declaration
          code: "public void queueAndSubmit(Display display) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.prepareRequest
          type: method_declaration
          code: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\t\tPrompt prompt = input.buildPrompt(display, batchFix);\n\t\tpreprocessPrompt(prompt);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\treq.setPrompt(prompt);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
        - id: PromptHandler.preprocessPrompt
          type: method_declaration
          code: "private void preprocessPrompt(Prompt prompt) {\n\t\tfor (String key : prompt.config.keys.split(\",\"))\n\t\t\tif (prompt.config.model.cap.acceptsKey(key))\n\t\t\t\tprompt.config.keys = key;\n\t\tif (prompt.arg.command instanceof CallCommand)\n\t\t\tprompt.config.model = Model.MCP_TOOLS;\n\t}"
        - id: PromptHandler.executeInner
          type: method_declaration
          code: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tres.prompt = req.getPrompt();\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\treturn res;\n\t}"
```
/answer b655-10 allow
ReasoningToken: 639
ReasoningToken: 81
ReasoningToken: 334
Control Request:
```yaml
id: b655-11
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src
  text: AnswerCommand
  reason: "Suche nach Verwendungsstellen von AnswerCommand und ToolCommand, um den Ausführungs-Trigger-Punkt zu finden."
```
/answer b655-11 allow
Control Request:
```yaml
id: b655-12
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer b655-12 allow
ReasoningToken: 24
ReasoningToken: 70
Control Request:
```yaml
id: b655-13
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
  reason: "Struktur der commands- und connector-Packages ermitteln, um Command-Ausführungsstellen zu finden."
```
/answer b655-13 allow
Control Request:
```yaml
id: b655-14
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
      error: Not a regular file.
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      error: Not a regular file.
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      error: Not a regular file.
```
/answer b655-14 allow
ReasoningToken: 109
ReasoningToken: 34
ReasoningToken: 85
ReasoningToken: 293
Control Request:
```yaml
id: b655-15
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
  reason: "Verzeichnisstruktur des workbench-Packages ermitteln, um relevante Dateien zu identifizieren (grep-Tool aktuell nicht verfügbar)."
```
/answer b655-15 allow
Control Request:
```yaml
id: b655-16
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      error: Not a regular file.
```
/answer b655-16 allow
ReasoningToken: 97
Control Request:
```yaml
id: b655-17
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
  reason: "Basis-Command-Klasse verstehen, um Ausführungsmechanismus zu finden."
```
/answer b655-17 deny nicht relevant, die Trigger musst im PromptHandler eingebaut werden
ReasoningToken: 1465
Control Request:
```yaml
id: b655-18
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/plugin.xml
  - /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
  reason: plugin.xml und MANIFEST.MF prüfen für View-Registrierung und JGit-Dependency.
```
/answer b655-18 allow
Control Request:
```yaml
id: b655-19
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/plugin.xml
      nodes:
      - id: 6aDxXF|KX26us
        signature: <?xml version="1.0" encoding="UTF-8"?>
        children:
        - id: 6aDxXF|KX26us.EvBWnk|2uwhxx
          signature: <?xml version="1.0" encoding="UTF-8"?>
          children:
          - id: 6aDxXF|KX26us.EvBWnk|2uwhxx.sVXNwz|HN8TMZ
            signature: 1.0
          - id: 6aDxXF|KX26us.EvBWnk|2uwhxx.HHCl7N|qksi7W
            signature: UTF-8
        - id: 6aDxXF|KX26us.Wyr38L|R5Uiaf
          signature: <?eclipse version="3.4"?>
          children:
          - id: 6aDxXF|KX26us.Wyr38L|R5Uiaf.JPHNPq|J4himP
            signature: eclipse
      - id: ODTrYv|DEw9ri
        signature: <plugin>
        children:
        - id: ODTrYv|DEw9ri.svusdm|MD8Wx0
          signature: <plugin>
          children:
          - id: ODTrYv|DEw9ri.svusdm|MD8Wx0.7cihA8|sDO1qh
            signature: plugin
        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW
          signature: <extension
          children:
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH
                signature: point="org.eclipse.ui.editors"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH.wvK66M|1NsA7Q
                  signature: '"org.eclipse.ui.editors"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA
              signature: <editor
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7
                signature: <editor
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf
                  signature: <editor
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.mYxTUx|pVutzf
                    signature: editor
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0
                    signature: class="xy.ai.workbench.editor.AISessionEditor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0.9ArSh7|9ArSh7
                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7
                    signature: contributorClass="xy.ai.workbench.editor.AISessionEditorContributor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7.mBa6Gq|AjDEu9
                      signature: contributorClass
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7.xTc0n3|3EOqWT
                      signature: '"xy.ai.workbench.editor.AISessionEditorContributor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP
                    signature: extensions="md"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP.oNEGLo|sOvtkc
                      signature: extensions
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP.iJFujz|T2GalM
                      signature: '"md"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE
                    signature: id="xy.ai.workbench.editor.AISessionEditor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE.9ArSh7|9ArSh7
                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s
                    signature: name="XY.AI Editor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s.OfF00l|J1wGXP
                      signature: '"XY.AI Editor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa
                    signature: symbolicFontName="org.eclipse.jface.bannerfont"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa.iSrcWf|Ih0D0I
                      signature: symbolicFontName
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa.t9yh0G|PitPXs
                      signature: '"org.eclipse.jface.bannerfont"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.lboVOr|njMjsc
                  signature: </editor>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.lboVOr|njMjsc.mYxTUx|pVutzf
                    signature: editor
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_1
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG
                signature: point="org.eclipse.help.contexts"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG.p91HbH|pMkSAp
                  signature: '"org.eclipse.help.contexts"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG
              signature: <contexts
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs
                signature: <contexts
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl
                  signature: <contexts
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.Zmw5Wl|ofHYiZ
                    signature: contexts
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf
                    signature: file="contexts.xml"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf.uZaeG9|MhHPVi
                      signature: file
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf.oiDKuK|BMWyxF
                      signature: '"contexts.xml"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.dtzzRw|RFkcTO
                  signature: </contexts>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.dtzzRw|RFkcTO.Zmw5Wl|ofHYiZ
                    signature: contexts
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_2
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa
                signature: point="org.eclipse.ui.views"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa.InVVJu|h7dmIt
                  signature: '"org.eclipse.ui.views"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb
              signature: <category
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9
                    signature: id="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K
                    signature: name="AI"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K.08u1S9|yreODC
                      signature: '"AI"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0
                    signature: class="xy.ai.workbench.view.AISessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0.nNnEWc|8t8ojj
                      signature: '"xy.ai.workbench.view.AISessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU
                    signature: id="xy.ai.workbench.view.AISessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU.nNnEWc|8t8ojj
                      signature: '"xy.ai.workbench.view.AISessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ
                    signature: name="XY.Ai Session Settings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ.GBfjgR|yY9jCq
                      signature: '"XY.Ai Session Settings"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT
                    signature: class="xy.ai.workbench.view.AIBatchView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT.mUkaJL|qmLMSD
                      signature: '"xy.ai.workbench.view.AIBatchView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6
                    signature: id="xy.ai.workbench.view.AIBatchView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6.mUkaJL|qmLMSD
                      signature: '"xy.ai.workbench.view.AIBatchView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt
                    signature: name="XY.AI Batch List"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt.0NKAYH|shNNEX
                      signature: '"XY.AI Batch List"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv
                    signature: name="AI Explorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv.0wk3UK|j57AN9
                      signature: '"AI Explorer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE
                    signature: class="xy.ai.workbench.view.explorer.MarkdownExplorerView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE.lkoC0h|wvYA3n
                      signature: '"xy.ai.workbench.view.explorer.MarkdownExplorerView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7
                    signature: id="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC
                    signature: class="xy.ai.workbench.view.ClaudeCodeSessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC.v3xUXM|onqOFk
                      signature: '"xy.ai.workbench.view.ClaudeCodeSessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0
                    signature: id="xy.ai.workbench.view.ClaudeCodeSessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0.v3xUXM|onqOFk
                      signature: '"xy.ai.workbench.view.ClaudeCodeSessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p
                    signature: name="CC Sessions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p.aYF3Ri|HTIcWU
                      signature: '"CC Sessions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_3
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz
            signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq
              signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF
                signature: point="org.eclipse.ui.navigator.navigatorContent"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF.yWgH0r|c9KkS5
                  signature: '"org.eclipse.ui.navigator.navigatorContent"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM
              signature: <navigatorContent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2
                signature: <navigatorContent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG
                  signature: <navigatorContent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.nRTKrG|sbIY3N
                    signature: navigatorContent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq
                    signature: name="AI Content"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq.3HVN1J|l5LpL4
                      signature: '"AI Content"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD
                    signature: contentProvider="xy.ai.workbench.view.explorer.MarkdownContentProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD.XicShZ|uk0ooo
                      signature: contentProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD.TfFBB4|ovFGdS
                      signature: '"xy.ai.workbench.view.explorer.MarkdownContentProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms
                    signature: labelProvider="xy.ai.workbench.view.explorer.MarkdownLabelProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms.4EFrdE|ProrOF
                      signature: labelProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms.lVVrlR|9wqNAy
                      signature: '"xy.ai.workbench.view.explorer.MarkdownLabelProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1
                    signature: id="xy.ai.workbench.view.content.AIContent"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1.omHzOp|omHzOp
                      signature: '"xy.ai.workbench.view.content.AIContent"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq
                    signature: priority="normal"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq.R1Wtl6|wGW28j
                      signature: priority
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq.SblMJG|2fuIXw
                      signature: '"normal"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv
                  signature: <triggerPoints>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy
                    signature: <triggerPoints>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.tzrGT0|Nbkq9v
                      signature: <triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.tzrGT0|Nbkq9v.TFNAyj|0PCOMF
                        signature: triggerPoints
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71
                      signature: <instanceof value="org.eclipse.core.resources.IResource" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY
                        signature: <instanceof value="org.eclipse.core.resources.IResource" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY
                          signature: <instanceof value="org.eclipse.core.resources.IResource" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR
                            signature: value="org.eclipse.core.resources.IResource"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.sOMyEq|Zt1DOX
                              signature: '"org.eclipse.core.resources.IResource"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.5LHDSz|CKh16l
                      signature: </triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.5LHDSz|CKh16l.TFNAyj|0PCOMF
                        signature: triggerPoints
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp
                    signature: <possibleChildren>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.GtIeEN|vX8S5R
                      signature: <possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.GtIeEN|vX8S5R.iKhE55|RWoxP7
                        signature: possibleChildren
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71
                      signature: <instanceof value="org.eclipse.core.resources.IResource" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY
                        signature: <instanceof value="org.eclipse.core.resources.IResource" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY
                          signature: <instanceof value="org.eclipse.core.resources.IResource" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR
                            signature: value="org.eclipse.core.resources.IResource"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.sOMyEq|Zt1DOX
                              signature: '"org.eclipse.core.resources.IResource"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.DhTzub|qZ4kEz
                      signature: </possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.DhTzub|qZ4kEz.iKhE55|RWoxP7
                        signature: possibleChildren
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.chat"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ.JoTiBt|0tBlvZ
                          signature: '"XY.AI.Workbench.wizard.newfile.chat"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.markdown"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey.HVBLms|PLxqhz
                          signature: '"XY.AI.Workbench.wizard.newfile.markdown"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA_2
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.context"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE.lgf7na|lgf7na
                          signature: '"XY.AI.Workbench.wizard.newfile.context"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.BEhMxY|zDzvLq
                  signature: </navigatorContent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.BEhMxY|zDzvLq.nRTKrG|sbIY3N
                    signature: navigatorContent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X
                    signature: id="xy.ai.workbench.view.filter.GitIgnore"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X.CjbaZR|jucSxJ
                      signature: '"xy.ai.workbench.view.filter.GitIgnore"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj
                    signature: name="Git Ignored Resources"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj.jxFY46|GFpf4W
                      signature: '"Git Ignored Resources"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn
                    signature: description="Hides resources excluded by a &quot;.gitignore&quot; file."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w
                      signature: '"Hides resources excluded by a &quot;.gitignore&quot; file."'
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j
                        signature: '&quot;'
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j.UvtwRc|yeZJVU
                          signature: quot
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j_1
                        signature: '&quot;'
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j_1.UvtwRc|yeZJVU
                          signature: quot
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0
                    signature: class="xy.ai.workbench.view.explorer.GitIgnoreViewerFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0.f1fstH|cdMHSP
                      signature: '"xy.ai.workbench.view.explorer.GitIgnoreViewerFilter"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh
                    signature: activeByDefault="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh.AESiiM|8pCtNq
                      signature: activeByDefault
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ
                    signature: id="xy.ai.workbench.view.filter.Project"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ.O0IM2i|dyYaAn
                      signature: '"xy.ai.workbench.view.filter.Project"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj
                    signature: name="Filter to Focused Project"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj.er0W4x|fBkZGR
                      signature: '"Filter to Focused Project"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak
                    signature: description="Limits the content to the project of the focused editor."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak.SdsUke|4LyZon
                      signature: '"Limits the content to the project of the focused editor."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67
                    signature: class="xy.ai.workbench.view.explorer.ProjectFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67.dktzJW|R0QOQV
                      signature: '"xy.ai.workbench.view.explorer.ProjectFilter"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh
                    signature: activeByDefault="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh.AESiiM|8pCtNq
                      signature: activeByDefault
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky
                    signature: visibleInUI="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky.c8aEjj|FWtask
                      signature: visibleInUI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_4
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW
            signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq
              signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF
                signature: point="org.eclipse.ui.navigator.navigatorContent"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF.yWgH0r|c9KkS5
                  signature: '"org.eclipse.ui.navigator.navigatorContent"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa
              signature: <navigatorContent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q
                signature: <navigatorContent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk
                  signature: <navigatorContent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.nRTKrG|sbIY3N
                    signature: navigatorContent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J
                    signature: name="Markdown Outline Content"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J.nLB2KS|UNDfIp
                      signature: '"Markdown Outline Content"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v
                    signature: contentProvider="xy.ai.workbench.editor.outline.MarkdownNavigatorContentProvide…
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v.XicShZ|uk0ooo
                      signature: contentProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v.G2xMzN|NFAWaL
                      signature: '"xy.ai.workbench.editor.outline.MarkdownNavigatorContentProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv
                    signature: labelProvider="xy.ai.workbench.editor.outline.MarkdownNavigatorLabelProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv.4EFrdE|ProrOF
                      signature: labelProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv.TD4aDX|Vzr2XR
                      signature: '"xy.ai.workbench.editor.outline.MarkdownNavigatorLabelProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP
                    signature: id="xy.ai.workbench.editor.outline.content.MarkdownOutline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP.GrTRfS|wR4LGS
                      signature: '"xy.ai.workbench.editor.outline.content.MarkdownOutline"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq
                    signature: priority="normal"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq.R1Wtl6|wGW28j
                      signature: priority
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq.SblMJG|2fuIXw
                      signature: '"normal"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd
                  signature: <triggerPoints>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu
                    signature: <triggerPoints>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.tzrGT0|Nbkq9v
                      signature: <triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.tzrGT0|Nbkq9v.TFNAyj|0PCOMF
                        signature: triggerPoints
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj
                      signature: <or>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ
                        signature: <or>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.hU4UNu|60xknY
                          signature: <or>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.hU4UNu|60xknY.K6iUQM|xlaWLE
                            signature: or
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6
                          signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI
                            signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI
                              signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV
                                signature: value="xy.ai.workbench.editor.mdast.MarkdownDocument"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV.4P13ar|Rjqh1d
                                  signature: '"xy.ai.workbench.editor.mdast.MarkdownDocument"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|Pg0Uv6_1
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R
                            signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R
                              signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf
                                signature: value="xy.ai.workbench.editor.outline.NodeElement"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.3euLtJ|6SEv6R
                                  signature: '"xy.ai.workbench.editor.outline.NodeElement"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.Nke5g2|Vw9DKx
                          signature: </or>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.Nke5g2|Vw9DKx.K6iUQM|xlaWLE
                            signature: or
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.5LHDSz|CKh16l
                      signature: </triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.5LHDSz|CKh16l.TFNAyj|0PCOMF
                        signature: triggerPoints
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg
                    signature: <possibleChildren>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.GtIeEN|vX8S5R
                      signature: <possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.GtIeEN|vX8S5R.iKhE55|RWoxP7
                        signature: possibleChildren
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd
                      signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R
                        signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R
                          signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf
                            signature: value="xy.ai.workbench.editor.outline.NodeElement"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.3euLtJ|6SEv6R
                              signature: '"xy.ai.workbench.editor.outline.NodeElement"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.DhTzub|qZ4kEz
                      signature: </possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.DhTzub|qZ4kEz.iKhE55|RWoxP7
                        signature: possibleChildren
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.BEhMxY|zDzvLq
                  signature: </navigatorContent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.BEhMxY|zDzvLq.nRTKrG|sbIY3N
                    signature: navigatorContent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY
                    signature: id="xy.ai.workbench.editor.outline.filter.ScriptBlock"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY.XMA3I1|WeQWyB
                      signature: '"xy.ai.workbench.editor.outline.filter.ScriptBlock"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT
                    signature: name="Script Blocks"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT.SOEQ3f|zJTYig
                      signature: '"Script Blocks"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn
                    signature: description="Hides fenced script block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn.kV7pmg|e7xp5d
                      signature: '"Hides fenced script block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0
                    signature: class="xy.ai.workbench.editor.outline.filter.ScriptBlockFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0.ea9vKV|TopxNR
                      signature: '"xy.ai.workbench.editor.outline.filter.ScriptBlockFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM
                    signature: id="xy.ai.workbench.editor.outline.filter.LineComment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM.JqbhXd|Fd2fVL
                      signature: '"xy.ai.workbench.editor.outline.filter.LineComment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH
                    signature: name="Line Comments"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH.wWtdQW|FmNAaf
                      signature: '"Line Comments"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx
                    signature: description="Hides line comment nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx.AOwUCl|Lu2rLK
                      signature: '"Hides line comment nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD
                    signature: class="xy.ai.workbench.editor.outline.filter.LineCommentFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD.Jyr8Rx|gr8uWc
                      signature: '"xy.ai.workbench.editor.outline.filter.LineCommentFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5
                    signature: id="xy.ai.workbench.editor.outline.filter.Paragraph"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5.o3yY7X|8U8QOk
                      signature: '"xy.ai.workbench.editor.outline.filter.Paragraph"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ
                    signature: name="Paragraphs"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ.OuC3Be|mmHUJR
                      signature: '"Paragraphs"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I
                    signature: description="Hides plain paragraph nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I.5AYmM4|GRrmnc
                      signature: '"Hides plain paragraph nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx
                    signature: class="xy.ai.workbench.editor.outline.filter.ParagraphFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx.5C1XeB|ypoUBt
                      signature: '"xy.ai.workbench.editor.outline.filter.ParagraphFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN
                    signature: id="xy.ai.workbench.editor.outline.filter.Headings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN.qGmeu4|rBt4nG
                      signature: '"xy.ai.workbench.editor.outline.filter.Headings"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5
                    signature: name="Headings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5.yxSpsk|qbEI3b
                      signature: '"Headings"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ
                    signature: description="Hides heading section nodes of all levels (and their substructure)…
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ.s3DMwj|qXXNp4
                      signature: '"Hides heading section nodes of all levels (and their substructure)."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X
                    signature: class="xy.ai.workbench.editor.outline.filter.HeadingFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X.1ssS4B|kF7weu
                      signature: '"xy.ai.workbench.editor.outline.filter.HeadingFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_5
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2
                    signature: id="xy.ai.workbench.editor.outline.filter.Thinking"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2.fLrBIh|8OWfwT
                      signature: '"xy.ai.workbench.editor.outline.filter.Thinking"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW
                    signature: name="Agent Thinking"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW.UlpgTf|fKRNGV
                      signature: '"Agent Thinking"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q
                    signature: description="Hides agent 'thinking' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q.G17igs|rqTjvd
                      signature: '"Hides agent ''thinking'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT
                    signature: class="xy.ai.workbench.editor.outline.filter.ThinkingFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT.PCsZKI|p72cjb
                      signature: '"xy.ai.workbench.editor.outline.filter.ThinkingFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_6
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP
                    signature: id="xy.ai.workbench.editor.outline.filter.Text"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP.Ahly97|2qY3FJ
                      signature: '"xy.ai.workbench.editor.outline.filter.Text"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV
                    signature: name="Agent Text"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV.y4YP7L|EByS7w
                      signature: '"Agent Text"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW
                    signature: description="Hides agent 'text' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW.IhUZmB|k4nwnv
                      signature: '"Hides agent ''text'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U
                    signature: class="xy.ai.workbench.editor.outline.filter.TextBlockFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U.i54e4l|s00nYK
                      signature: '"xy.ai.workbench.editor.outline.filter.TextBlockFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_7
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput
                    signature: id="xy.ai.workbench.editor.outline.filter.ToolUse"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput.mTqadQ|KcNVKp
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolUse"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5
                    signature: name="Agent Tool Use"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5.x5cRK5|Cc34jG
                      signature: '"Agent Tool Use"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf
                    signature: description="Hides agent 'tool use' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf.rYH2GX|ppKb9n
                      signature: '"Hides agent ''tool use'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU
                    signature: class="xy.ai.workbench.editor.outline.filter.ToolUseFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU.BYOn8S|rE4aRp
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolUseFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_8
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o
                    signature: id="xy.ai.workbench.editor.outline.filter.ToolResult"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o.SxCYPv|tYzAXt
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolResult"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm
                    signature: name="Agent Tool Result"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm.62RWAc|3nzxev
                      signature: '"Agent Tool Result"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS
                    signature: description="Hides agent 'tool result' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS.WBKunp|aZqAKQ
                      signature: '"Hides agent ''tool result'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk
                    signature: class="xy.ai.workbench.editor.outline.filter.ToolResultFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk.ucuaEl|aezrFt
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolResultFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_9
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21
                    signature: id="xy.ai.workbench.editor.outline.filter.ReasoningToken"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21.CTKZrs|pod7iu
                      signature: '"xy.ai.workbench.editor.outline.filter.ReasoningToken"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I
                    signature: name="Agent Reasoning Tokens"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I.xZZu2k|fewhuR
                      signature: '"Agent Reasoning Tokens"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf
                    signature: description="Hides agent reasoning token block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf.c68skP|fWVPRG
                      signature: '"Hides agent reasoning token block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB
                    signature: class="xy.ai.workbench.editor.outline.filter.ReasoningTokenFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB.OONJ8u|wUzd8y
                      signature: '"xy.ai.workbench.editor.outline.filter.ReasoningTokenFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_10
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq
                    signature: id="xy.ai.workbench.editor.outline.filter.TokenStats"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq.eSgNfJ|cF7xnj
                      signature: '"xy.ai.workbench.editor.outline.filter.TokenStats"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX
                    signature: name="Agent Token Stats"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX.8KRwBC|sZzujc
                      signature: '"Agent Token Stats"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp
                    signature: description="Hides agent token statistics block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp.pv9Byy|HX6QCl
                      signature: '"Hides agent token statistics block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x
                    signature: class="xy.ai.workbench.editor.outline.filter.TokenStatsFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x.pfjXgP|xPiHwk
                      signature: '"xy.ai.workbench.editor.outline.filter.TokenStatsFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_11
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp
                    signature: id="xy.ai.workbench.editor.outline.filter.SystemInit"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp.BLSd6i|FyxVlZ
                      signature: '"xy.ai.workbench.editor.outline.filter.SystemInit"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7
                    signature: name="Agent System Init"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7.pjZsZl|V6WDMd
                      signature: '"Agent System Init"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R
                    signature: description="Hides agent system-init block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R.VPFlzk|Czzwkq
                      signature: '"Hides agent system-init block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj
                    signature: class="xy.ai.workbench.editor.outline.filter.SystemInitFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj.eI2rs7|SHkhHg
                      signature: '"xy.ai.workbench.editor.outline.filter.SystemInitFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_12
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ
                    signature: id="xy.ai.workbench.editor.outline.filter.Answer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ.UBIkST|VF46NC
                      signature: '"xy.ai.workbench.editor.outline.filter.Answer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y
                    signature: name="Tool Answers"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y.YCPfCd|GLAfXv
                      signature: '"Tool Answers"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C
                    signature: description="Hides tool answer block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C.bRnYjM|vvWHM3
                      signature: '"Hides tool answer block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6
                    signature: class="xy.ai.workbench.editor.outline.filter.AnswerFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6.wdfba8|K85pjv
                      signature: '"xy.ai.workbench.editor.outline.filter.AnswerFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_13
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn
                    signature: id="xy.ai.workbench.editor.outline.filter.ControlRequest"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn.PcHVi0|wZWgRW
                      signature: '"xy.ai.workbench.editor.outline.filter.ControlRequest"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB
                    signature: name="Control Requests"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB.8m5wpk|bGfErc
                      signature: '"Control Requests"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa
                    signature: description="Hides control request section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa.Am8re5|wXmtot
                      signature: '"Hides control request section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj
                    signature: class="xy.ai.workbench.editor.outline.filter.ControlRequestFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj.H1umdD|i4g81Y
                      signature: '"xy.ai.workbench.editor.outline.filter.ControlRequestFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_14
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t
                    signature: id="xy.ai.workbench.editor.outline.filter.Page"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t.iQTD11|pEWzBB
                      signature: '"xy.ai.workbench.editor.outline.filter.Page"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn
                    signature: name="Page"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn.8c5J8K|CxykUr
                      signature: '"Page"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv
                    signature: description="Hides the page section node."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv.qlM2xX|eVZLOx
                      signature: '"Hides the page section node."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf
                    signature: class="xy.ai.workbench.editor.outline.filter.PageFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf.aX5Vnr|zMSRQ2
                      signature: '"xy.ai.workbench.editor.outline.filter.PageFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_15
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5
                    signature: id="xy.ai.workbench.editor.outline.filter.UserSection"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5.hOJA1r|rw9YPb
                      signature: '"xy.ai.workbench.editor.outline.filter.UserSection"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX
                    signature: name="User Sections"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX.CF71YR|kT9HyH
                      signature: '"User Sections"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2
                    signature: description="Hides chat 'user' section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2.5dl3ms|vzaUoh
                      signature: '"Hides chat ''user'' section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU
                    signature: class="xy.ai.workbench.editor.outline.filter.UserSectionFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU.qoA7Qz|fQ85J7
                      signature: '"xy.ai.workbench.editor.outline.filter.UserSectionFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_16
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO
                    signature: id="xy.ai.workbench.editor.outline.filter.AgentSection"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO.AKopva|U8WCb7
                      signature: '"xy.ai.workbench.editor.outline.filter.AgentSection"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I
                    signature: name="Agent Sections"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I.JC7F6w|NDcz4F
                      signature: '"Agent Sections"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36
                    signature: description="Hides chat 'agent' section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36.Mtdu1z|I8AViW
                      signature: '"Hides chat ''agent'' section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av
                    signature: class="xy.ai.workbench.editor.outline.filter.AgentSectionFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av.yo85ZA|Xp2xQ7
                      signature: '"xy.ai.workbench.editor.outline.filter.AgentSectionFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_5
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg
            signature: <extension point="org.eclipse.ui.navigator.viewer">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E
              signature: <extension point="org.eclipse.ui.navigator.viewer">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm
                signature: point="org.eclipse.ui.navigator.viewer"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm.SkBkM4|1o82R9
                  signature: '"org.eclipse.ui.navigator.viewer"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB
              signature: <viewer
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az
                signature: <viewer
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY
                  signature: <viewer
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.ua3BFe|Rs0wgv
                    signature: viewer
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L
                    signature: viewerId="xy.ai.workbench.editor.outline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L.ZbPM9C|ExhMhZ
                      signature: '"xy.ai.workbench.editor.outline"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.sfHnVX|iIrjUR
                  signature: </viewer>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.sfHnVX|iIrjUR.ua3BFe|Rs0wgv
                    signature: viewer
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi
                signature: <viewerContentBinding viewerId="xy.ai.workbench.editor.outline">
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C
                  signature: <viewerContentBinding viewerId="xy.ai.workbench.editor.outline">
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.pgutWt|OLSIV4
                    signature: viewerContentBinding
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L
                    signature: viewerId="xy.ai.workbench.editor.outline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L.ZbPM9C|ExhMhZ
                      signature: '"xy.ai.workbench.editor.outline"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6
                  signature: <includes>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I
                    signature: <includes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.PznH2s|SpglhF
                      signature: <includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.PznH2s|SpglhF.SLI9rh|d9FNDu
                        signature: includes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg
                      signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55
                        signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55
                          signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo
                            signature: pattern="xy\.ai\.workbench\.editor\.outline\..*"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo.Fp91gh|Fp91gh
                              signature: '"xy\.ai\.workbench\.editor\.outline\..*"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.J5mD7W|C6U2p5
                      signature: </includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.J5mD7W|C6U2p5.SLI9rh|d9FNDu
                        signature: includes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi
                    signature: <excludes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.ogdq0u|7jxwct
                      signature: <excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.ogdq0u|7jxwct.QMIIlf|8BvLOQ
                        signature: excludes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng
                      signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p
                        signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p
                          signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ
                            signature: pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ.bykh6r|neEyp9
                              signature: '"^(?!xy\.ai\.workbench\.editor\.outline\.).*"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.d9fOjf|ms0G0O
                      signature: </excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.d9fOjf|ms0G0O.QMIIlf|8BvLOQ
                        signature: excludes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.a8b6Hz|n6s26d
                  signature: </viewerContentBinding>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.a8b6Hz|n6s26d.pgutWt|OLSIV4
                    signature: viewerContentBinding
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_6
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx
            signature: <extension point="org.eclipse.ui.navigator.viewer">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E
              signature: <extension point="org.eclipse.ui.navigator.viewer">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm
                signature: point="org.eclipse.ui.navigator.viewer"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm.SkBkM4|1o82R9
                  signature: '"org.eclipse.ui.navigator.viewer"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D
              signature: <viewer
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV
                signature: <viewer
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5
                  signature: <viewer
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.ua3BFe|Rs0wgv
                    signature: viewer
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx
                    signature: inheritBindingsFromViewer="org.eclipse.ui.navigator.ProjectExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx.qboCpm|m5RLGP
                      signature: inheritBindingsFromViewer
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx.rzvUxu|7FcjP4
                      signature: '"org.eclipse.ui.navigator.ProjectExplorer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2
                    signature: popupMenuId="org.eclipse.ui.navigator.ProjectExplorer#PopupMenu"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2.iVj8Tf|AE8XkR
                      signature: popupMenuId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2.1Tb4iT|IlDjmx
                      signature: '"org.eclipse.ui.navigator.ProjectExplorer#PopupMenu"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1
                    signature: viewerId="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.jJEqAm|QC6vXz
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.jJEqAm|QC6vXz.jJEqAm|QC6vXz
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sfHnVX|iIrjUR
                  signature: </viewer>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sfHnVX|iIrjUR.ua3BFe|Rs0wgv
                    signature: viewer
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|IckjY4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ
                signature: <viewerContentBinding viewerId="xy.ai.workbench.view.AIExplorer">
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX
                  signature: <viewerContentBinding viewerId="xy.ai.workbench.view.AIExplorer">
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.pgutWt|OLSIV4
                    signature: viewerContentBinding
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1
                    signature: viewerId="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor
                  signature: <includes>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|8FggJ5
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU
                    signature: <includes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.PznH2s|SpglhF
                      signature: <includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.PznH2s|SpglhF.SLI9rh|d9FNDu
                        signature: includes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy
                      signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG
                        signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG
                          signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd
                            signature: pattern="xy.ai.workbench.view.content.AIContent"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd.omHzOp|omHzOp
                              signature: '"xy.ai.workbench.view.content.AIContent"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_1
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp
                        signature: <contentExtension pattern="xy.ai.workbench.view.filter.GitIgnore"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp
                          signature: <contentExtension pattern="xy.ai.workbench.view.filter.GitIgnore"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU
                            signature: pattern="xy.ai.workbench.view.filter.GitIgnore"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU.CjbaZR|jucSxJ
                              signature: '"xy.ai.workbench.view.filter.GitIgnore"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_2
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS
                        signature: <contentExtension pattern="xy.ai.workbench.view.filter.Project"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS
                          signature: <contentExtension pattern="xy.ai.workbench.view.filter.Project"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp
                            signature: pattern="xy.ai.workbench.view.filter.Project"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp.O0IM2i|dyYaAn
                              signature: '"xy.ai.workbench.view.filter.Project"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_3
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG
                        signature: <contentExtension pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG
                          signature: <contentExtension pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi
                            signature: pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi.cRofm0|moUsZD
                              signature: '"xy.ai.workbench.view.linkHelper.AILinkHelper"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|8FggJ5
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.J5mD7W|C6U2p5
                      signature: </includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.J5mD7W|C6U2p5.SLI9rh|d9FNDu
                        signature: includes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|8FggJ5_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS
                    signature: <excludes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.ogdq0u|7jxwct
                      signature: <excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.ogdq0u|7jxwct.QMIIlf|8BvLOQ
                        signature: excludes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI
                      signature: <contentExtension
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.jJEqAm|6wcsPo
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP
                        signature: <contentExtension
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww
                          signature: <contentExtension
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX
                            signature: pattern="^(?!xy\.ai\.workbench\.view\.(content\.AIContent|filter\.(GitIgnore|Pr…
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX.P3pzrq|OOBdha
                              signature: '"^(?!xy\.ai\.workbench\.view\.(content\.AIContent|filter\.(GitIgnore|Project)|l…'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.jJEqAm|6wcsPo
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.jJEqAm|6wcsPo.jJEqAm|6wcsPo
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.Rysgvf|uDSGsB
                          signature: </contentExtension>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.Rysgvf|uDSGsB.k7pAMI|paRB1P
                            signature: contentExtension
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.jJEqAm|8FggJ5
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.d9fOjf|ms0G0O
                      signature: </excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.d9fOjf|ms0G0O.QMIIlf|8BvLOQ
                        signature: excludes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|IckjY4
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.a8b6Hz|n6s26d
                  signature: </viewerContentBinding>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.a8b6Hz|n6s26d.pgutWt|OLSIV4
                    signature: viewerContentBinding
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_7
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS
            signature: <extension point="org.eclipse.ui.navigator.linkHelper">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA
              signature: <extension point="org.eclipse.ui.navigator.linkHelper">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT
                signature: point="org.eclipse.ui.navigator.linkHelper"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT.Vi712B|nWCHWi
                  signature: '"org.eclipse.ui.navigator.linkHelper"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy
              signature: <linkHelper
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L
                signature: <linkHelper
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM
                  signature: <linkHelper
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.lg7K4l|S8cI5f
                    signature: linkHelper
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G
                    signature: class="xy.ai.workbench.view.explorer.MarkdownLinkHelper"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G.6UK0zx|9VEsjb
                      signature: '"xy.ai.workbench.view.explorer.MarkdownLinkHelper"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB
                    signature: id="xy.ai.workbench.view.linkHelper.AILinkHelper"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB.cRofm0|moUsZD
                      signature: '"xy.ai.workbench.view.linkHelper.AILinkHelper"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr
                  signature: <selectionEnablement>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W
                    signature: <selectionEnablement>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.u3mwY7|23XHiL
                      signature: <selectionEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.u3mwY7|23XHiL.wxOPh0|Pt4zSU
                        signature: selectionEnablement
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi
                      signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG
                        signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG
                          signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X
                            signature: value="org.eclipse.core.resources.IFile"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X.UM4a6T|hSin1y
                              signature: '"org.eclipse.core.resources.IFile"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.6UKhOA|JfM6gv
                      signature: </selectionEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.6UKhOA|JfM6gv.wxOPh0|Pt4zSU
                        signature: selectionEnablement
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x
                    signature: <editorInputEnablement>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.f1R2gP|KPuUL8
                      signature: <editorInputEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.f1R2gP|KPuUL8.z3DTBL|brwZNR
                        signature: editorInputEnablement
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0
                      signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3
                        signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3
                          signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx
                            signature: value="org.eclipse.ui.IFileEditorInput"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx.1HrI5f|nRoZBR
                              signature: '"org.eclipse.ui.IFileEditorInput"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.zTA4ZI|jAepFK
                      signature: </editorInputEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.zTA4ZI|jAepFK.z3DTBL|brwZNR
                        signature: editorInputEnablement
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.p5cQxp|KB59Jh
                  signature: </linkHelper>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.p5cQxp|KB59Jh.lg7K4l|S8cI5f
                    signature: linkHelper
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_8
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo
                signature: point="org.eclipse.ui.commands"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo.7bGT2u|KS0dN6
                  signature: '"org.eclipse.ui.commands"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp
              signature: <category
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU
                    signature: id="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K
                    signature: name="AI"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K.08u1S9|yreODC
                      signature: '"AI"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6
                    signature: id="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR
                    signature: name="Submit prompt"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR.3AMCga|HTPMmq
                      signature: '"Submit prompt"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4
                    signature: id="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv
                    signature: name="Toggle AI Comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv.iJYSVT|W1kbZ9
                      signature: '"Toggle AI Comment"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1
                    signature: id="XY.AI.Workbench.commands.copyPath"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1.4xZaKz|rCdYxS
                      signature: '"XY.AI.Workbench.commands.copyPath"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz
                    signature: name="Copy Path"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz.mjK4OU|ppKfXr
                      signature: '"Copy Path"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_9
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW
                signature: point="org.eclipse.ui.handlers"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW.rFLkJq|kkOmqU
                  signature: '"org.eclipse.ui.handlers"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6
              signature: <handler
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk
                    signature: class="xy.ai.workbench.handlers.SessionExecuteHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk.D0XCfg|xWv0Gr
                      signature: '"xy.ai.workbench.handlers.SessionExecuteHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc
                    signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW
                    signature: class="xy.ai.workbench.editor.CommentHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW.sFp8br|fp5pCz
                      signature: '"xy.ai.workbench.editor.CommentHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K
                    signature: commandId="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv
                  signature: <activeWhen>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o
                    signature: <activeWhen>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.wpUKMU|tzVdHu
                      signature: <activeWhen>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.wpUKMU|tzVdHu.y382q5|iuqWJY
                        signature: activeWhen
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6
                      signature: <and>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19
                        signature: <and>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.tiyh6R|hf1Hug
                          signature: <and>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.tiyh6R|hf1Hug.A9Fz3K|heucGq
                            signature: and
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l
                          signature: <with variable="activeEditorId">
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3
                            signature: <with variable="activeEditorId">
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f
                              signature: <with variable="activeEditorId">
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F
                                signature: variable="activeEditorId"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F.rxgImd|NltelH
                                  signature: '"activeEditorId"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ
                              signature: <equals
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC
                                signature: <equals
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM
                                  signature: <equals
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.gwouS4|Auw28O
                                    signature: equals
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt
                                    signature: value="xy.ai.workbench.editor.AISessionEditor"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt.qPnJq7|FsZIq9
                                      signature: value
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt.9ArSh7|9ArSh7
                                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.jJEqAm|AJUbxG
                                  signature: ""
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.jJEqAm|AJUbxG.jJEqAm|AJUbxG
                                    signature: ""
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.bfSCV1|Wo3wch
                                  signature: </equals>
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.bfSCV1|Wo3wch.gwouS4|Auw28O
                                    signature: equals
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|Pg0Uv6_1
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1
                            signature: <with variable="activeEditor">
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6
                              signature: <with variable="activeEditor">
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L
                                signature: variable="activeEditor"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L.NPsmv6|IfD7NP
                                  signature: '"activeEditor"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe
                              signature: <instanceof
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn
                                signature: <instanceof
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn
                                  signature: <instanceof
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.NuJ4WF|IFqwGX
                                    signature: instanceof
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt
                                    signature: value="xy.ai.workbench.editor.AISessionEditor"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt.qPnJq7|FsZIq9
                                      signature: value
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt.9ArSh7|9ArSh7
                                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.ifqcoG|BK79Nc
                          signature: </and>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.ifqcoG|BK79Nc.A9Fz3K|heucGq
                            signature: and
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.tUi396|gYLJKs
                      signature: </activeWhen>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.tUi396|gYLJKs.y382q5|iuqWJY
                        signature: activeWhen
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN
                    signature: class="xy.ai.workbench.handlers.CopyPathHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN.AKGYYb|FzbKoO
                      signature: '"xy.ai.workbench.handlers.CopyPathHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB
                    signature: commandId="XY.AI.Workbench.commands.copyPath"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                      signature: '"XY.AI.Workbench.commands.copyPath"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|Yz4xOn
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD
                signature: point="org.eclipse.ui.bindings"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD.HKEsGM|9hjZZL
                  signature: '"org.eclipse.ui.bindings"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT
              signature: <key
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C
                signature: <key
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35
                  signature: <key
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.4TdmK0|qkRRub
                    signature: key
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc
                    signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc
                    signature: contextId="org.eclipse.ui.contexts.window"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc.F2nBbM|dKNe6M
                      signature: contextId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc.FcPD79|XLIrzD
                      signature: '"org.eclipse.ui.contexts.window"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h
                    signature: schemeId="org.eclipse.ui.defaultAcceleratorConfiguration"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h.6fwW2g|J7jRNW
                      signature: schemeId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h.8uVeeu|HHRPNe
                      signature: '"org.eclipse.ui.defaultAcceleratorConfiguration"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc
                    signature: sequence="M1+Enter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc.HHda8J|vAsOQc
                      signature: sequence
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc.ltqHfs|r3y6Vp
                      signature: '"M1+Enter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.hHAwQC|ybB0ha
                  signature: </key>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.hHAwQC|ybB0ha.4TdmK0|qkRRub
                    signature: key
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw
                signature: <key
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG
                  signature: <key
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.4TdmK0|qkRRub
                    signature: key
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K
                    signature: commandId="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8
                    signature: contextId="org.eclipse.ui.textEditorScope"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8.F2nBbM|dKNe6M
                      signature: contextId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8.GrA6do|UFSgPb
                      signature: '"org.eclipse.ui.textEditorScope"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h
                    signature: schemeId="org.eclipse.ui.defaultAcceleratorConfiguration"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h.6fwW2g|J7jRNW
                      signature: schemeId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h.8uVeeu|HHRPNe
                      signature: '"org.eclipse.ui.defaultAcceleratorConfiguration"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK
                    signature: sequence="Shift+Ctrl+C"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK.HHda8J|vAsOQc
                      signature: sequence
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK.zX292u|M82LgY
                      signature: '"Shift+Ctrl+C"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.hHAwQC|ybB0ha
                  signature: </key>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.hHAwQC|ybB0ha.4TdmK0|qkRRub
                    signature: key
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_10
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s
                signature: point="org.eclipse.ui.menus"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s.iQU0Jy|eDUE1H
                  signature: '"org.eclipse.ui.menus"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE
              signature: <menuContribution
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR
                    signature: locationURI="menu:org.eclipse.ui.main.menu?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR.IKMSH7|nko9x0
                      signature: '"menu:org.eclipse.ui.main.menu?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h
                  signature: <menu
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF
                    signature: <menu
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw
                      signature: <menu
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.SBHj3k|GReCyA
                        signature: menu
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL
                        signature: id="XY.AI.Workbench.menus.aiMenu"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL.FTbZzu|RBAvHS
                          signature: '"XY.AI.Workbench.menus.aiMenu"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi
                        signature: label="AI"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi.08u1S9|yreODC
                          signature: '"AI"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp
                        signature: mnemonic="A"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp.r21tlZ|8RIQe5
                          signature: '"A"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql
                        signature: <command
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv
                          signature: <command
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.Oc6MrV|FCuBYJ
                            signature: command
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc
                            signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                              signature: commandId
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                              signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R
                            signature: icon="icons/sample.png"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R.czwUqY|rUELQe
                              signature: icon
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R.taJEpx|cIkXsp
                              signature: '"icons/sample.png"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC
                            signature: id="XY.AI.Workbench.menus.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC.sn6NDO|fEEdrX
                              signature: id
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC.DaS0nL|7nuxuz
                              signature: '"XY.AI.Workbench.menus.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ
                            signature: mnemonic="S"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ.OrXo0A|j7rGdG
                              signature: mnemonic
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ.VnqTdH|eKF6Hh
                              signature: '"S"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.jJEqAm|qYdK3X
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.jJEqAm|qYdK3X.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.YOB4cr|h9FdVz
                          signature: </command>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                            signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.68XUTe|nnTezY
                      signature: </menu>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.68XUTe|nnTezY.SBHj3k|GReCyA
                        signature: menu
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh
                    signature: locationURI="toolbar:org.eclipse.ui.main.toolbar?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh.Q0wEPK|dWbHVk
                      signature: '"toolbar:org.eclipse.ui.main.toolbar?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP
                  signature: <toolbar
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS
                    signature: <toolbar
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok
                      signature: <toolbar
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.Pqxk0r|DPsgBy
                        signature: toolbar
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv
                        signature: id="XY.AI.Workbench.toolbars.aiToolbar"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv.jV8hIP|0jdHPv
                          signature: '"XY.AI.Workbench.toolbars.aiToolbar"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA
                        signature: <command
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI
                          signature: <command
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.Oc6MrV|FCuBYJ
                            signature: command
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc
                            signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                              signature: commandId
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                              signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R
                            signature: icon="icons/sample.png"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R.czwUqY|rUELQe
                              signature: icon
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R.taJEpx|cIkXsp
                              signature: '"icons/sample.png"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I
                            signature: id="XY.AI.Workbench.toolbars.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I.sn6NDO|fEEdrX
                              signature: id
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I.fUJU0r|kMf99Q
                              signature: '"XY.AI.Workbench.toolbars.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy
                            signature: tooltip="Submit the current prompt"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy.EiJDrG|df03Cg
                              signature: tooltip
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy.clbVRA|o2EP8a
                              signature: '"Submit the current prompt"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.jJEqAm|qYdK3X
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.jJEqAm|qYdK3X.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.YOB4cr|h9FdVz
                          signature: </command>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                            signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.kDAD27|8UkVvz
                      signature: </toolbar>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.kDAD27|8UkVvz.Pqxk0r|DPsgBy
                        signature: toolbar
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9
                    signature: locationURI="popup:org.eclipse.ui.popup.any?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9.QKbbkR|cs61Ex
                      signature: '"popup:org.eclipse.ui.popup.any?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz
                    signature: <command
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.Oc6MrV|FCuBYJ
                        signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB
                        signature: commandId="XY.AI.Workbench.commands.copyPath"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                          signature: commandId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                          signature: '"XY.AI.Workbench.commands.copyPath"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK
                        signature: label="Copy Path"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK.mjK4OU|ppKfXr
                          signature: '"Copy Path"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM
                        signature: mnemonic="P"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM.dNBBXn|qAvOHO
                          signature: '"P"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC
                        signature: style="push"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC.dE0Bu2|40u8pt
                          signature: style
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC.iZ2Ji5|xBEXZL
                          signature: '"push"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5
                      signature: <visibleWhen
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO
                        signature: <visibleWhen
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ
                          signature: <visibleWhen
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.xfgl7A|CZ6BIc
                            signature: visibleWhen
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR
                            signature: checkEnabled="false"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR.iNUWdq|h63Sz6
                              signature: checkEnabled
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR.T330Gd|8GhRd4
                              signature: '"false"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr
                          signature: <with
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe
                            signature: <with
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy
                              signature: <with
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN
                                signature: variable="selection"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN.BjW2HA|mwdYpz
                                  signature: '"selection"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal
                              signature: <iterate
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey
                                signature: <iterate
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr
                                  signature: <iterate
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.x3SDFq|4ShU3M
                                    signature: iterate
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP
                                    signature: ifEmpty="false"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP.orryTb|rcDXxf
                                      signature: ifEmpty
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP.T330Gd|8GhRd4
                                      signature: '"false"'
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM
                                  signature: <adapt
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.jJEqAm|lb1F2O
                                    signature: ""
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA
                                    signature: <adapt
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR
                                      signature: <adapt
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.HBdMko|xmasne
                                        signature: adapt
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq
                                        signature: type="org.eclipse.core.resources.IResource"
                                        children:
                                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq.jjZD4X|PFMnGV
                                          signature: type
                                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq.sOMyEq|Zt1DOX
                                          signature: '"org.eclipse.core.resources.IResource"'
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.jJEqAm|lb1F2O
                                      signature: ""
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.jJEqAm|lb1F2O.jJEqAm|lb1F2O
                                        signature: ""
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.Ir3Iz7|hzIY4n
                                      signature: </adapt>
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.Ir3Iz7|hzIY4n.HBdMko|xmasne
                                        signature: adapt
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.jJEqAm|AJUbxG
                                    signature: ""
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.Zm8mGe|rPoUJu
                                  signature: </iterate>
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.Zm8mGe|rPoUJu.x3SDFq|4ShU3M
                                    signature: iterate
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.UOKzmA|BEDiCw
                          signature: </visibleWhen>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.UOKzmA|BEDiCw.xfgl7A|CZ6BIc
                            signature: visibleWhen
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.YOB4cr|h9FdVz
                      signature: </command>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                        signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z
                    signature: locationURI="popup:#TextEditorContext?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z.eAvBjH|dUqdnI
                      signature: '"popup:#TextEditorContext?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi
                    signature: <command
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.Oc6MrV|FCuBYJ
                        signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB
                        signature: commandId="XY.AI.Workbench.commands.copyPath"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                          signature: commandId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                          signature: '"XY.AI.Workbench.commands.copyPath"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK
                        signature: label="Copy Path"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK.mjK4OU|ppKfXr
                          signature: '"Copy Path"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM
                        signature: mnemonic="P"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM.dNBBXn|qAvOHO
                          signature: '"P"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC
                        signature: style="push"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC.dE0Bu2|40u8pt
                          signature: style
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC.iZ2Ji5|xBEXZL
                          signature: '"push"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.jJEqAm|fCL9ED
                      signature: ""
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.jJEqAm|fCL9ED.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.YOB4cr|h9FdVz
                      signature: </command>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                        signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_11
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E
                signature: point="org.eclipse.ui.perspectiveExtensions"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E.nX3h2w|h61quw
                  signature: '"org.eclipse.ui.perspectiveExtensions"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv
              signature: <perspectiveExtension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e
                signature: <perspectiveExtension
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc
                  signature: <perspectiveExtension
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.EIoDLt|g8bVVr
                    signature: perspectiveExtension
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9
                    signature: targetID="org.eclipse.jdt.ui.JavaPerspective"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9.BTalmQ|yQH83Z
                      signature: targetID
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9.DkrLzJ|uM36CM
                      signature: '"org.eclipse.jdt.ui.JavaPerspective"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs
                    signature: <view
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL
                      signature: <view
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.5TDlNy|sUPndD
                        signature: view
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6
                        signature: id="xy.ai.workbench.view.AIBatchView"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6.mUkaJL|qmLMSD
                          signature: '"xy.ai.workbench.view.AIBatchView"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8
                        signature: ratio="0.5"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8.Lm4FBm|9ccknR
                          signature: ratio
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8.R4BCWz|y8ITi7
                          signature: '"0.5"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z
                        signature: relationship="bottom"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z.LfQhts|oE9ING
                          signature: relationship
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z.snbwVD|WSXfv3
                          signature: '"bottom"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp
                        signature: relative="org.eclipse.ui.views.ProblemView"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp.iVAqCB|ZndawC
                          signature: relative
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp.GadPot|8B0m3m
                          signature: '"org.eclipse.ui.views.ProblemView"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.jJEqAm|fCL9ED
                      signature: ""
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.jJEqAm|fCL9ED.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.g8VJFp|xnenxG
                      signature: </view>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.g8VJFp|xnenxG.5TDlNy|sUPndD
                        signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.QGV5Hx|kz199O
                  signature: </perspectiveExtension>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.QGV5Hx|kz199O.EIoDLt|g8bVVr
                    signature: perspectiveExtension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_12
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC
                signature: point="org.eclipse.ui.fontDefinitions"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC.CwvM9t|tWkcIv
                  signature: '"org.eclipse.ui.fontDefinitions"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1
              signature: <fontDefinition
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0
                signature: <fontDefinition
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O
                  signature: <fontDefinition
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.Iy0lps|8dN93u
                    signature: fontDefinition
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7
                    signature: id="XY.AI.Workbench.fontDefinition1"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7.cmYkeE|lDescl
                      signature: '"XY.AI.Workbench.fontDefinition1"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H
                    signature: label="XY.AI Editor Font"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H.SHMA9X|LLI9P6
                      signature: label
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H.EGONf6|pcMhiJ
                      signature: '"XY.AI Editor Font"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13
                    signature: value="1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13.qPnJq7|FsZIq9
                      signature: value
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13.pWtfMs|2V6glq
                      signature: '"1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.thLBlY|dNx8bT
                  signature: </fontDefinition>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.thLBlY|dNx8bT.Iy0lps|8dN93u
                    signature: fontDefinition
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_13
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM
                signature: id="xy.ai.workbench.promptmarker"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM.sn6NDO|fEEdrX
                  signature: id
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM.388MRq|kISRzp
                  signature: '"xy.ai.workbench.promptmarker"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm
                signature: name="XY.AI Prompt Marker"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm.g6aFQH|AsA4NM
                  signature: name
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm.w35BmX|54YejC
                  signature: '"XY.AI Prompt Marker"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M
                signature: point="org.eclipse.core.resources.markers"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M.Wi6POB|idD9hq
                  signature: '"org.eclipse.core.resources.markers"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK
              signature: <persistent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy
                signature: <persistent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q
                  signature: <persistent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.pKzr8Y|gSjHco
                    signature: persistent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR
                    signature: value="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR.qPnJq7|FsZIq9
                      signature: value
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.Yubk2a|1nElVp
                  signature: </persistent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.Yubk2a|1nElVp.pKzr8Y|gSjHco
                    signature: persistent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3
                signature: <super
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH
                  signature: <super
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.TbFdXP|Fwu00T
                    signature: super
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE
                    signature: type="org.eclipse.core.resources.taskmarker"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE.jjZD4X|PFMnGV
                      signature: type
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE.NHJ7sA|nwWkY3
                      signature: '"org.eclipse.core.resources.taskmarker"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.qhN0gU|B4EtHM
                  signature: </super>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.qhN0gU|B4EtHM.TbFdXP|Fwu00T
                    signature: super
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_14
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD
                signature: point="org.eclipse.ui.newWizards"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD.rFAdEM|yTKnr3
                  signature: '"org.eclipse.ui.newWizards"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0
              signature: <wizard
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS
                    signature: id="XY.AI.Workbench.wizard.newfile.markdown"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS.HVBLms|PLxqhz
                      signature: '"XY.AI.Workbench.wizard.newfile.markdown"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE
                    signature: name="Markdown File"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE.vqspE0|LIu494
                      signature: '"Markdown File"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei
                    signature: class="xy.ai.workbench.handlers.NewMarkdownWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei.n3oybL|s6148p
                      signature: '"xy.ai.workbench.handlers.NewMarkdownWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.sS0phJ|Rt0Dm5
                      signature: Creates a new Markdown file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.sS0phJ|Rt0Dm5.sS0phJ|Rt0Dm5
                        signature: Creates a new Markdown file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ
                    signature: id="XY.AI.Workbench.wizard.newfile.context"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ.lgf7na|lgf7na
                      signature: '"XY.AI.Workbench.wizard.newfile.context"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx
                    signature: name="Prompt Context"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx.hwiILu|AyEw9S
                      signature: '"Prompt Context"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN
                    signature: class="xy.ai.workbench.handlers.NewPromptContextWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN.Fd1IDY|b3xk2K
                      signature: '"xy.ai.workbench.handlers.NewPromptContextWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.47sWAP|ZLzPnS
                      signature: Creates a new Prompt Context file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.47sWAP|ZLzPnS.47sWAP|ZLzPnS
                        signature: Creates a new Prompt Context file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX
                    signature: id="XY.AI.Workbench.wizard.newfile.chat"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX.JoTiBt|0tBlvZ
                      signature: '"XY.AI.Workbench.wizard.newfile.chat"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1
                    signature: name="AI Chat"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1.Uv0WNH|afVFqZ
                      signature: '"AI Chat"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG
                    signature: class="xy.ai.workbench.handlers.NewChatWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG.YKLUrz|YKLUrz
                      signature: '"xy.ai.workbench.handlers.NewChatWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.TjIN8G|eJDblE
                      signature: Creates a new AI Chat file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.TjIN8G|eJDblE.TjIN8G|eJDblE
                        signature: Creates a new AI Chat file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa
                    signature: id="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM
                    signature: name="AI Files"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM.sRhT0k|AtmuUS
                      signature: '"AI Files"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|Yz4xOn_1
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX
            signature: <extension point="org.eclipse.ui.editors.annotationTypes">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9
              signature: <extension point="org.eclipse.ui.editors.annotationTypes">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y
                signature: point="org.eclipse.ui.editors.annotationTypes"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y.Bt4b4H|Bt4b4H
                  signature: '"org.eclipse.ui.editors.annotationTypes"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1
              signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs
                signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs
                  signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.jjZD4X|PFMnGV
                    signature: type
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb
                    signature: name="xy.ai.workbench.editor.spellcheck.spelling"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb.7mWWOn|QCXd55
                      signature: '"xy.ai.workbench.editor.spellcheck.spelling"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|K0kEnq
            signature: ""
        - id: ODTrYv|DEw9ri.7mn2Ql|yyuEDp
          signature: </plugin>
          children:
          - id: ODTrYv|DEw9ri.7mn2Ql|yyuEDp.7cihA8|sDO1qh
            signature: plugin
    - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
      nodes:
      - id: file
        signature: "Manifest-Version: 1.0"
```
/answer b655-19 allow
ReasoningToken: 255
ReasoningToken: 145
ReasoningToken: 22
Control Request:
```yaml
id: b655-1a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
  regexp: .*
  reason: MANIFEST.MF Bundle-Dependencies pruefen (JGit/EGit)
```
/answer b655-1a allow
Control Request:
```yaml
id: b655-1b
result:
  structuredContent:
    files:
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
           org.eclipse.ui.forms.widgets
          Bundle-Activator: xy.ai.workbench.Activator
          Bundle-ClassPath: libs/jackson-dataformat-yaml-2.20.0.jar,
           libs/snakeyaml-2.4.jar,
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
           org.eclipse.jface.text,
           org.eclipse.core.resources,
           org.eclipse.ui.editors,
           org.eclipse.ui.ide,
           org.eclipse.ui.views,
           org.eclipse.ui.navigator,
           org.eclipse.search
          Bundle-RequiredExecutionEnvironment: JavaSE-21
          Automatic-Module-Name: XY.AI.Workbench
          Bundle-ActivationPolicy: lazy
```
/answer b655-1b allow
ReasoningToken: 32
ReasoningToken: 2367
ReasoningToken: 64
ReasoningToken: 45
ReasoningToken: 660
ReasoningToken: 359
Control Request:
```yaml
id: b655-1c
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/plugin.xml
  reason: Testaufruf um Tool-Verfuegbarkeit zu pruefen
```
/answer b655-1c allow
Control Request:
```yaml
id: b655-1d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/plugin.xml
      nodes:
      - id: 6aDxXF|KX26us
        signature: <?xml version="1.0" encoding="UTF-8"?>
        children:
        - id: 6aDxXF|KX26us.EvBWnk|2uwhxx
          signature: <?xml version="1.0" encoding="UTF-8"?>
          children:
          - id: 6aDxXF|KX26us.EvBWnk|2uwhxx.sVXNwz|HN8TMZ
            signature: 1.0
          - id: 6aDxXF|KX26us.EvBWnk|2uwhxx.HHCl7N|qksi7W
            signature: UTF-8
        - id: 6aDxXF|KX26us.Wyr38L|R5Uiaf
          signature: <?eclipse version="3.4"?>
          children:
          - id: 6aDxXF|KX26us.Wyr38L|R5Uiaf.JPHNPq|J4himP
            signature: eclipse
      - id: ODTrYv|DEw9ri
        signature: <plugin>
        children:
        - id: ODTrYv|DEw9ri.svusdm|MD8Wx0
          signature: <plugin>
          children:
          - id: ODTrYv|DEw9ri.svusdm|MD8Wx0.7cihA8|sDO1qh
            signature: plugin
        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW
          signature: <extension
          children:
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH
                signature: point="org.eclipse.ui.editors"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.betSXO|P6pfSK.1dhuYh|tgaHxH.wvK66M|1NsA7Q
                  signature: '"org.eclipse.ui.editors"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA
              signature: <editor
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7
                signature: <editor
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf
                  signature: <editor
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.mYxTUx|pVutzf
                    signature: editor
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0
                    signature: class="xy.ai.workbench.editor.AISessionEditor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.dWPjcv|s5Vhn0.9ArSh7|9ArSh7
                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7
                    signature: contributorClass="xy.ai.workbench.editor.AISessionEditorContributor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7.mBa6Gq|AjDEu9
                      signature: contributorClass
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.4Sqi5B|lOfeR7.xTc0n3|3EOqWT
                      signature: '"xy.ai.workbench.editor.AISessionEditorContributor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP
                    signature: extensions="md"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP.oNEGLo|sOvtkc
                      signature: extensions
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.NMvK2B|uaz1kP.iJFujz|T2GalM
                      signature: '"md"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE
                    signature: id="xy.ai.workbench.editor.AISessionEditor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.gr8Kwr|7BIcJE.9ArSh7|9ArSh7
                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s
                    signature: name="XY.AI Editor"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.RNZrJ0|yhRA8s.OfF00l|J1wGXP
                      signature: '"XY.AI Editor"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa
                    signature: symbolicFontName="org.eclipse.jface.bannerfont"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa.iSrcWf|Ih0D0I
                      signature: symbolicFontName
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.VLlGR4|5xhshf.QucHwb|zz92oa.t9yh0G|PitPXs
                      signature: '"org.eclipse.jface.bannerfont"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.lboVOr|njMjsc
                  signature: </editor>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.PDVTYp|khUxm7.lboVOr|njMjsc.mYxTUx|pVutzf
                    signature: editor
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.PDVTYp|vpofZA.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zrdQ4C|msTMWR.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_1
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG
                signature: point="org.eclipse.help.contexts"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.s7O1Ix|lq1iDu.c2xATs|rjhayG.p91HbH|pMkSAp
                  signature: '"org.eclipse.help.contexts"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG
              signature: <contexts
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs
                signature: <contexts
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl
                  signature: <contexts
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.Zmw5Wl|ofHYiZ
                    signature: contexts
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf
                    signature: file="contexts.xml"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf.uZaeG9|MhHPVi
                      signature: file
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.ZV18YA|hZ9sUl.cUb6oQ|wvgClf.oiDKuK|BMWyxF
                      signature: '"contexts.xml"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.dtzzRw|RFkcTO
                  signature: </contexts>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.0ympsT|H4HOWs.dtzzRw|RFkcTO.Zmw5Wl|ofHYiZ
                    signature: contexts
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.0ympsT|iJYIwG.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dUE588|LF1CLk.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_2
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa
                signature: point="org.eclipse.ui.views"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.6AuZE9|NiCUes.nDImMn|5vLgGa.InVVJu|h7dmIt
                  signature: '"org.eclipse.ui.views"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb
              signature: <category
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9
                    signature: id="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.Cf0WZc|fiXNk9.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K
                    signature: name="AI"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.Fqs5Hp|J8EYqt.53ORm9|LJ5w4K.08u1S9|yreODC
                      signature: '"AI"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.hvllzw|WSds7Q.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0
                    signature: class="xy.ai.workbench.view.AISessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.qdlh4w|TG9qD0.nNnEWc|8t8ojj
                      signature: '"xy.ai.workbench.view.AISessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU
                    signature: id="xy.ai.workbench.view.AISessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.4TmgTx|NhNQXU.nNnEWc|8t8ojj
                      signature: '"xy.ai.workbench.view.AISessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ
                    signature: name="XY.Ai Session Settings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.YUzaB0|3orxGN.v8Kjmm|Z3eORZ.GBfjgR|yY9jCq
                      signature: '"XY.Ai Session Settings"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.VeXlJK|mVTfh7.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT
                    signature: class="xy.ai.workbench.view.AIBatchView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.nZbNxT|nZbNxT.mUkaJL|qmLMSD
                      signature: '"xy.ai.workbench.view.AIBatchView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6
                    signature: id="xy.ai.workbench.view.AIBatchView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.0T6syJ|DuUCg6.mUkaJL|qmLMSD
                      signature: '"xy.ai.workbench.view.AIBatchView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt
                    signature: name="XY.AI Batch List"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.BVy7qT|gJdgJ8.ezGGJq|AbjuCt.0NKAYH|shNNEX
                      signature: '"XY.AI Batch List"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.ftdIKX|flh60h.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv
                    signature: name="AI Explorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.6mcxnN|8MLpnv.0wk3UK|j57AN9
                      signature: '"AI Explorer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE
                    signature: class="xy.ai.workbench.view.explorer.MarkdownExplorerView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.FqJmV4|xfZbXE.lkoC0h|wvYA3n
                      signature: '"xy.ai.workbench.view.explorer.MarkdownExplorerView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7
                    signature: id="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.sNZsmN|23ZLLR.mq3kVB|0Kqan7.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.bAxKiO|yGdb78.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|TL9HIp_4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V
                signature: <view
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.5TDlNy|sUPndD
                    signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk
                    signature: allowMultiple="false"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk.a4v8FP|A29qzP
                      signature: allowMultiple
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.oF29a0|fImGhk.T330Gd|8GhRd4
                      signature: '"false"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q
                    signature: category="XY.AI.Workbench"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.b4IPK5|IDHt3q.DSOoZ8|tHccT7
                      signature: '"XY.AI.Workbench"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC
                    signature: class="xy.ai.workbench.view.ClaudeCodeSessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.IdAqaJ|jev6fC.v3xUXM|onqOFk
                      signature: '"xy.ai.workbench.view.ClaudeCodeSessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0
                    signature: id="xy.ai.workbench.view.ClaudeCodeSessionView"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.nDX2WZ|FOkMu0.v3xUXM|onqOFk
                      signature: '"xy.ai.workbench.view.ClaudeCodeSessionView"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp
                    signature: inject="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp.slLq54|YhRPwW
                      signature: inject
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.Yeu9DW|cQN6pp.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p
                    signature: name="CC Sessions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.epQG0W|8VIza7.dgopWU|0Fig0p.aYF3Ri|HTIcWU
                      signature: '"CC Sessions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.g8VJFp|xnenxG
                  signature: </view>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V.g8VJFp|xnenxG.5TDlNy|sUPndD
                    signature: view
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_3
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz
            signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq
              signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF
                signature: point="org.eclipse.ui.navigator.navigatorContent"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.B0bygY|zvRKhq.JpQs2R|WZaaxF.yWgH0r|c9KkS5
                  signature: '"org.eclipse.ui.navigator.navigatorContent"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM
              signature: <navigatorContent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2
                signature: <navigatorContent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG
                  signature: <navigatorContent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.nRTKrG|sbIY3N
                    signature: navigatorContent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq
                    signature: name="AI Content"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.YoVwGk|YDabeq.3HVN1J|l5LpL4
                      signature: '"AI Content"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD
                    signature: contentProvider="xy.ai.workbench.view.explorer.MarkdownContentProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD.XicShZ|uk0ooo
                      signature: contentProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.lrhQGn|LNc3dD.TfFBB4|ovFGdS
                      signature: '"xy.ai.workbench.view.explorer.MarkdownContentProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms
                    signature: labelProvider="xy.ai.workbench.view.explorer.MarkdownLabelProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms.4EFrdE|ProrOF
                      signature: labelProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.qbEY5k|M5x8Ms.lVVrlR|9wqNAy
                      signature: '"xy.ai.workbench.view.explorer.MarkdownLabelProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1
                    signature: id="xy.ai.workbench.view.content.AIContent"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.Tscv1X|IZhCy1.omHzOp|omHzOp
                      signature: '"xy.ai.workbench.view.content.AIContent"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq
                    signature: priority="normal"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq.R1Wtl6|wGW28j
                      signature: priority
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.mYa9S5|HBfbpG.K0jT42|Licpwq.SblMJG|2fuIXw
                      signature: '"normal"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv
                  signature: <triggerPoints>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy
                    signature: <triggerPoints>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.tzrGT0|Nbkq9v
                      signature: <triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.tzrGT0|Nbkq9v.TFNAyj|0PCOMF
                        signature: triggerPoints
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71
                      signature: <instanceof value="org.eclipse.core.resources.IResource" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY
                        signature: <instanceof value="org.eclipse.core.resources.IResource" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY
                          signature: <instanceof value="org.eclipse.core.resources.IResource" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR
                            signature: value="org.eclipse.core.resources.IResource"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.sOMyEq|Zt1DOX
                              signature: '"org.eclipse.core.resources.IResource"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.FUwUFs|0o4h71.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.5LHDSz|CKh16l
                      signature: </triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.QCy81s|HngsZy.5LHDSz|CKh16l.TFNAyj|0PCOMF
                        signature: triggerPoints
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp
                    signature: <possibleChildren>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.GtIeEN|vX8S5R
                      signature: <possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.GtIeEN|vX8S5R.iKhE55|RWoxP7
                        signature: possibleChildren
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71
                      signature: <instanceof value="org.eclipse.core.resources.IResource" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY
                        signature: <instanceof value="org.eclipse.core.resources.IResource" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY
                          signature: <instanceof value="org.eclipse.core.resources.IResource" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR
                            signature: value="org.eclipse.core.resources.IResource"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.FUwUFs|L7VyaY.FUwUFs|L7VyaY.Sc3Sk4|7FlEuR.sOMyEq|Zt1DOX
                              signature: '"org.eclipse.core.resources.IResource"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.FUwUFs|0o4h71.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.DhTzub|qZ4kEz
                      signature: </possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.cTrGn2|Jh2nxp.DhTzub|qZ4kEz.iKhE55|RWoxP7
                        signature: possibleChildren
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.chat"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.CGJ8qE|BTszQZ.JoTiBt|0tBlvZ
                          signature: '"XY.AI.Workbench.wizard.newfile.chat"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.QXK7dw|TI5CIi.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|NdwVaV.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.markdown"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.DxYUbw|ytpzey.HVBLms|PLxqhz
                          signature: '"XY.AI.Workbench.wizard.newfile.markdown"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.QXK7dw|GTwkC4.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|zUPGCV.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|8sqxIA_2
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf
                    signature: <commonWizard
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk
                      signature: <commonWizard
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.Q3E4hT|M5iwat
                        signature: commonWizard
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL
                        signature: type="new"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL.jjZD4X|PFMnGV
                          signature: type
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.9BmX7M|Y9QUDL.2vfox3|o1F5xr
                          signature: '"new"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE
                        signature: wizardId="XY.AI.Workbench.wizard.newfile.context"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE.osz1Ga|3uNQ7A
                          signature: wizardId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.K6Suw4|2DzMdE.lgf7na|lgf7na
                          signature: '"XY.AI.Workbench.wizard.newfile.context"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs
                        signature: menuGroupId="new.ext"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs.3bo5re|e3hIjf
                          signature: menuGroupId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.QXK7dw|xWMHNk.UhyPqm|qvNeMs.ff7w7f|KyBsXT
                          signature: '"new.ext"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn
                      signature: <enablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.jJEqAm|FMFPaR
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE
                        signature: <enablement>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY
                          signature: <enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.DZ3fZi|zpcCCY.DCjZvB|DivrWb
                            signature: enablement
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO
                          signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|tQ611E
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz
                            signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz
                              signature: <instanceof value="org.eclipse.core.resources.IFolder"/>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e
                                signature: value="org.eclipse.core.resources.IFolder"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.q6Rtgy|9FsRCz.q6Rtgy|9FsRCz.dEAEZQ|q8Tv8e.cw5kTw|jT3T4x
                                  signature: '"org.eclipse.core.resources.IFolder"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.q6Rtgy|Hy3OlO.jJEqAm|FMFPaR
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa
                          signature: </enablement>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.tm8w9T|QdsGiE.byxN4M|6N7BDa.DCjZvB|DivrWb
                            signature: enablement
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.tm8w9T|DodWIn.jJEqAm|8sqxIA
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.byqBD6|nYY0Ef
                      signature: </commonWizard>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.ypx3HL|TMjsgf.byqBD6|nYY0Ef.Q3E4hT|M5iwat
                        signature: commonWizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.KhaZ7E|3tuxQv.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.BEhMxY|zDzvLq
                  signature: </navigatorContent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.wk46Rd|a7z5h2.BEhMxY|zDzvLq.nRTKrG|sbIY3N
                    signature: navigatorContent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X
                    signature: id="xy.ai.workbench.view.filter.GitIgnore"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.YGNRVl|9BIM8X.CjbaZR|jucSxJ
                      signature: '"xy.ai.workbench.view.filter.GitIgnore"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj
                    signature: name="Git Ignored Resources"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.Sh1ko6|bo0WHj.jxFY46|GFpf4W
                      signature: '"Git Ignored Resources"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn
                    signature: description="Hides resources excluded by a &quot;.gitignore&quot; file."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w
                      signature: '"Hides resources excluded by a &quot;.gitignore&quot; file."'
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j
                        signature: '&quot;'
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j.UvtwRc|yeZJVU
                          signature: quot
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j_1
                        signature: '&quot;'
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.EuVF9T|1kyVRn.FlOF2M|yETt6w.bmi7Wm|0xQu7j_1.UvtwRc|yeZJVU
                          signature: quot
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0
                    signature: class="xy.ai.workbench.view.explorer.GitIgnoreViewerFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.it5O8I|kdWLx0.f1fstH|cdMHSP
                      signature: '"xy.ai.workbench.view.explorer.GitIgnoreViewerFilter"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh
                    signature: activeByDefault="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh.AESiiM|8pCtNq
                      signature: activeByDefault
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.6kiPwp|g69zSa.42ZQDb|haEMrh.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|wTGT3n.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ
                    signature: id="xy.ai.workbench.view.filter.Project"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.6n0JtZ|6n0JtZ.O0IM2i|dyYaAn
                      signature: '"xy.ai.workbench.view.filter.Project"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj
                    signature: name="Filter to Focused Project"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.xntUac|WGWWJj.er0W4x|fBkZGR
                      signature: '"Filter to Focused Project"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak
                    signature: description="Limits the content to the project of the focused editor."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.Qgpvqp|PCNLak.SdsUke|4LyZon
                      signature: '"Limits the content to the project of the focused editor."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67
                    signature: class="xy.ai.workbench.view.explorer.ProjectFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.LQaQ5w|X6uR67.dktzJW|R0QOQV
                      signature: '"xy.ai.workbench.view.explorer.ProjectFilter"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh
                    signature: activeByDefault="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh.AESiiM|8pCtNq
                      signature: activeByDefault
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.42ZQDb|haEMrh.v26ACe|dtG9Jc
                      signature: '"true"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky
                    signature: visibleInUI="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky.c8aEjj|FWtask
                      signature: visibleInUI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.S974RP|vVSPMZ.7cfIKr|eY4pky.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.rycMHN|fzSXjl.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.WGNcu1|Si8FjM.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|ewy1wz.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_4
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW
            signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq
              signature: <extension point="org.eclipse.ui.navigator.navigatorContent">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF
                signature: point="org.eclipse.ui.navigator.navigatorContent"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.B0bygY|zvRKhq.JpQs2R|WZaaxF.yWgH0r|c9KkS5
                  signature: '"org.eclipse.ui.navigator.navigatorContent"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa
              signature: <navigatorContent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q
                signature: <navigatorContent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk
                  signature: <navigatorContent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.nRTKrG|sbIY3N
                    signature: navigatorContent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J
                    signature: name="Markdown Outline Content"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.RSqGhZ|N0v12J.nLB2KS|UNDfIp
                      signature: '"Markdown Outline Content"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v
                    signature: contentProvider="xy.ai.workbench.editor.outline.MarkdownNavigatorContentProvide…
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v.XicShZ|uk0ooo
                      signature: contentProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.NJzRDJ|sLz80v.G2xMzN|NFAWaL
                      signature: '"xy.ai.workbench.editor.outline.MarkdownNavigatorContentProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv
                    signature: labelProvider="xy.ai.workbench.editor.outline.MarkdownNavigatorLabelProvider"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv.4EFrdE|ProrOF
                      signature: labelProvider
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.qZ2m4Z|qhFoiv.TD4aDX|Vzr2XR
                      signature: '"xy.ai.workbench.editor.outline.MarkdownNavigatorLabelProvider"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP
                    signature: id="xy.ai.workbench.editor.outline.content.MarkdownOutline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.gLtxKT|l4TfxP.GrTRfS|wR4LGS
                      signature: '"xy.ai.workbench.editor.outline.content.MarkdownOutline"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq
                    signature: priority="normal"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq.R1Wtl6|wGW28j
                      signature: priority
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.Bhri2k|rKisjk.K0jT42|Licpwq.SblMJG|2fuIXw
                      signature: '"normal"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd
                  signature: <triggerPoints>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu
                    signature: <triggerPoints>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.tzrGT0|Nbkq9v
                      signature: <triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.tzrGT0|Nbkq9v.TFNAyj|0PCOMF
                        signature: triggerPoints
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj
                      signature: <or>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ
                        signature: <or>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.hU4UNu|60xknY
                          signature: <or>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.hU4UNu|60xknY.K6iUQM|xlaWLE
                            signature: or
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6
                          signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI
                            signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI
                              signature: <instanceof value="xy.ai.workbench.editor.mdast.MarkdownDocument" />
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV
                                signature: value="xy.ai.workbench.editor.mdast.MarkdownDocument"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.CPIGoc|izUjFI.CPIGoc|izUjFI.qcGzaF|83NNtV.4P13ar|Rjqh1d
                                  signature: '"xy.ai.workbench.editor.mdast.MarkdownDocument"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|Pg0Uv6_1
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R
                            signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R
                              signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.NuJ4WF|IFqwGX
                                signature: instanceof
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf
                                signature: value="xy.ai.workbench.editor.outline.NodeElement"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.qPnJq7|FsZIq9
                                  signature: value
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.3euLtJ|6SEv6R
                                  signature: '"xy.ai.workbench.editor.outline.NodeElement"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.T2F1Lx|2kyjP6.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.Nke5g2|Vw9DKx
                          signature: </or>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.FkgHDg|kbvKIZ.Nke5g2|Vw9DKx.K6iUQM|xlaWLE
                            signature: or
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.FkgHDg|HhOYzj.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.5LHDSz|CKh16l
                      signature: </triggerPoints>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.rCotqi|PUZzPu.5LHDSz|CKh16l.TFNAyj|0PCOMF
                        signature: triggerPoints
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg
                    signature: <possibleChildren>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.GtIeEN|vX8S5R
                      signature: <possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.GtIeEN|vX8S5R.iKhE55|RWoxP7
                        signature: possibleChildren
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd
                      signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R
                        signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R
                          signature: <instanceof value="xy.ai.workbench.editor.outline.NodeElement" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf
                            signature: value="xy.ai.workbench.editor.outline.NodeElement"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.T2F1Lx|7GdY3R.T2F1Lx|7GdY3R.clE2hb|e1tInf.3euLtJ|6SEv6R
                              signature: '"xy.ai.workbench.editor.outline.NodeElement"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.T2F1Lx|gUiEAd.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.DhTzub|qZ4kEz
                      signature: </possibleChildren>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.cTrGn2|CnbPwg.DhTzub|qZ4kEz.iKhE55|RWoxP7
                        signature: possibleChildren
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.XLKvW6|yusfdd.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.BEhMxY|zDzvLq
                  signature: </navigatorContent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.wk46Rd|L5DJ8Q.BEhMxY|zDzvLq.nRTKrG|sbIY3N
                    signature: navigatorContent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY
                    signature: id="xy.ai.workbench.editor.outline.filter.ScriptBlock"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.7UYdJX|Q7yXMY.XMA3I1|WeQWyB
                      signature: '"xy.ai.workbench.editor.outline.filter.ScriptBlock"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT
                    signature: name="Script Blocks"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.WFgjWH|CsXiNT.SOEQ3f|zJTYig
                      signature: '"Script Blocks"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn
                    signature: description="Hides fenced script block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.sRvYIe|SEF2pn.kV7pmg|e7xp5d
                      signature: '"Hides fenced script block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0
                    signature: class="xy.ai.workbench.editor.outline.filter.ScriptBlockFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.RqdL9s|R8PLNG.98bETZ|eh2Ir0.ea9vKV|TopxNR
                      signature: '"xy.ai.workbench.editor.outline.filter.ScriptBlockFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gwCYye.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM
                    signature: id="xy.ai.workbench.editor.outline.filter.LineComment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.USpjMN|0b8tDM.JqbhXd|Fd2fVL
                      signature: '"xy.ai.workbench.editor.outline.filter.LineComment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH
                    signature: name="Line Comments"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.top5Dk|qODVKH.wWtdQW|FmNAaf
                      signature: '"Line Comments"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx
                    signature: description="Hides line comment nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.mY4pl3|kCg9Yx.AOwUCl|Lu2rLK
                      signature: '"Hides line comment nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD
                    signature: class="xy.ai.workbench.editor.outline.filter.LineCommentFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.aXh2SW|ZRIaXT.JW5Fq7|sSCsRD.Jyr8Rx|gr8uWc
                      signature: '"xy.ai.workbench.editor.outline.filter.LineCommentFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|cgmV51.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5
                    signature: id="xy.ai.workbench.editor.outline.filter.Paragraph"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.yhzfCD|waSAa5.o3yY7X|8U8QOk
                      signature: '"xy.ai.workbench.editor.outline.filter.Paragraph"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ
                    signature: name="Paragraphs"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.1w9ovQ|9c1IKZ.OuC3Be|mmHUJR
                      signature: '"Paragraphs"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I
                    signature: description="Hides plain paragraph nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.gzcghg|uwuQ0I.5AYmM4|GRrmnc
                      signature: '"Hides plain paragraph nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx
                    signature: class="xy.ai.workbench.editor.outline.filter.ParagraphFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.8pF7Z5|GZPS81.Zn8HPN|fQzYqx.5C1XeB|ypoUBt
                      signature: '"xy.ai.workbench.editor.outline.filter.ParagraphFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|2WCR8o.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN
                    signature: id="xy.ai.workbench.editor.outline.filter.Headings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.swxUcR|4PQ9eN.qGmeu4|rBt4nG
                      signature: '"xy.ai.workbench.editor.outline.filter.Headings"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5
                    signature: name="Headings"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.5eLdl9|APSBQ5.yxSpsk|qbEI3b
                      signature: '"Headings"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ
                    signature: description="Hides heading section nodes of all levels (and their substructure)…
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.M6OfEW|VNDemJ.s3DMwj|qXXNp4
                      signature: '"Hides heading section nodes of all levels (and their substructure)."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X
                    signature: class="xy.ai.workbench.editor.outline.filter.HeadingFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.VCDj4W|4vQuNw.BOpUIZ|YMfB1X.1ssS4B|kF7weu
                      signature: '"xy.ai.workbench.editor.outline.filter.HeadingFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|f9z7EZ.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_5
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2
                    signature: id="xy.ai.workbench.editor.outline.filter.Thinking"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.aCwrgZ|pO3wp2.fLrBIh|8OWfwT
                      signature: '"xy.ai.workbench.editor.outline.filter.Thinking"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW
                    signature: name="Agent Thinking"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.jZoFl5|CKFJAW.UlpgTf|fKRNGV
                      signature: '"Agent Thinking"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q
                    signature: description="Hides agent 'thinking' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.pDMHxi|euLI2q.G17igs|rqTjvd
                      signature: '"Hides agent ''thinking'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT
                    signature: class="xy.ai.workbench.editor.outline.filter.ThinkingFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.VVgulG|1weeCX.VUxB54|Sr48dT.PCsZKI|p72cjb
                      signature: '"xy.ai.workbench.editor.outline.filter.ThinkingFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|RgL46N.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_6
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP
                    signature: id="xy.ai.workbench.editor.outline.filter.Text"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.olqpIC|Ewd7hP.Ahly97|2qY3FJ
                      signature: '"xy.ai.workbench.editor.outline.filter.Text"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV
                    signature: name="Agent Text"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.t1Q64n|mvg7ZV.y4YP7L|EByS7w
                      signature: '"Agent Text"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW
                    signature: description="Hides agent 'text' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.0J2r7g|YVJ7kW.IhUZmB|k4nwnv
                      signature: '"Hides agent ''text'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U
                    signature: class="xy.ai.workbench.editor.outline.filter.TextBlockFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.6mgZeV|qczJI7.5xwUoQ|D2Fj0U.i54e4l|s00nYK
                      signature: '"xy.ai.workbench.editor.outline.filter.TextBlockFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|FAvelV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_7
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput
                    signature: id="xy.ai.workbench.editor.outline.filter.ToolUse"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.qSPek1|hHmput.mTqadQ|KcNVKp
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolUse"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5
                    signature: name="Agent Tool Use"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.nqe1Vw|mOqod5.x5cRK5|Cc34jG
                      signature: '"Agent Tool Use"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf
                    signature: description="Hides agent 'tool use' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.GH5YYL|05fMYf.rYH2GX|ppKb9n
                      signature: '"Hides agent ''tool use'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU
                    signature: class="xy.ai.workbench.editor.outline.filter.ToolUseFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.LVFRDb|ZJsydc.5z0OGk|HbqnjU.BYOn8S|rE4aRp
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolUseFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|Y7IHV5.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_8
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o
                    signature: id="xy.ai.workbench.editor.outline.filter.ToolResult"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.twsgY2|rsmu1o.SxCYPv|tYzAXt
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolResult"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm
                    signature: name="Agent Tool Result"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.1DyUXu|X8AKYm.62RWAc|3nzxev
                      signature: '"Agent Tool Result"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS
                    signature: description="Hides agent 'tool result' block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.s06Fc8|n8fqmS.WBKunp|aZqAKQ
                      signature: '"Hides agent ''tool result'' block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk
                    signature: class="xy.ai.workbench.editor.outline.filter.ToolResultFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.vjFj27|Q3LAxj.PZOsus|jHMcvk.ucuaEl|aezrFt
                      signature: '"xy.ai.workbench.editor.outline.filter.ToolResultFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|qK0PDE.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_9
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21
                    signature: id="xy.ai.workbench.editor.outline.filter.ReasoningToken"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.Omrdfx|jR1k21.CTKZrs|pod7iu
                      signature: '"xy.ai.workbench.editor.outline.filter.ReasoningToken"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I
                    signature: name="Agent Reasoning Tokens"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.ZoupLP|j57x4I.xZZu2k|fewhuR
                      signature: '"Agent Reasoning Tokens"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf
                    signature: description="Hides agent reasoning token block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.HDZwsw|WPkzCf.c68skP|fWVPRG
                      signature: '"Hides agent reasoning token block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB
                    signature: class="xy.ai.workbench.editor.outline.filter.ReasoningTokenFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.Nh7zkB|8Q2WS3.LhGSfL|fbmdQB.OONJ8u|wUzd8y
                      signature: '"xy.ai.workbench.editor.outline.filter.ReasoningTokenFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|TR9SkV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_10
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq
                    signature: id="xy.ai.workbench.editor.outline.filter.TokenStats"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.LG6fq8|mw1oEq.eSgNfJ|cF7xnj
                      signature: '"xy.ai.workbench.editor.outline.filter.TokenStats"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX
                    signature: name="Agent Token Stats"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.tKVSuL|5Lz3NX.8KRwBC|sZzujc
                      signature: '"Agent Token Stats"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp
                    signature: description="Hides agent token statistics block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.vUQS2x|yDtnYp.pv9Byy|HX6QCl
                      signature: '"Hides agent token statistics block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x
                    signature: class="xy.ai.workbench.editor.outline.filter.TokenStatsFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.fvwoo5|9sMb0J.5uimfp|slfP7x.pfjXgP|xPiHwk
                      signature: '"xy.ai.workbench.editor.outline.filter.TokenStatsFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|YPZfPk.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_11
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp
                    signature: id="xy.ai.workbench.editor.outline.filter.SystemInit"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.5ZJ4QE|mAuGAp.BLSd6i|FyxVlZ
                      signature: '"xy.ai.workbench.editor.outline.filter.SystemInit"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7
                    signature: name="Agent System Init"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.pzoICe|Puo5q7.pjZsZl|V6WDMd
                      signature: '"Agent System Init"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R
                    signature: description="Hides agent system-init block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.dm4NER|2Ubg4R.VPFlzk|Czzwkq
                      signature: '"Hides agent system-init block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj
                    signature: class="xy.ai.workbench.editor.outline.filter.SystemInitFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.AWYCmW|5swFsv.m6x7vh|TK1rjj.eI2rs7|SHkhHg
                      signature: '"xy.ai.workbench.editor.outline.filter.SystemInitFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|GAQflV.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_12
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ
                    signature: id="xy.ai.workbench.editor.outline.filter.Answer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.AaNLwf|YH14pZ.UBIkST|VF46NC
                      signature: '"xy.ai.workbench.editor.outline.filter.Answer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y
                    signature: name="Tool Answers"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.5dcck9|Vnwc5y.YCPfCd|GLAfXv
                      signature: '"Tool Answers"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C
                    signature: description="Hides tool answer block nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.do0lhr|lhgX6C.bRnYjM|vvWHM3
                      signature: '"Hides tool answer block nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6
                    signature: class="xy.ai.workbench.editor.outline.filter.AnswerFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.FXoFf6|WjjK8v.ieAlps|aRd2X6.wdfba8|K85pjv
                      signature: '"xy.ai.workbench.editor.outline.filter.AnswerFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|XW36Tk.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_13
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn
                    signature: id="xy.ai.workbench.editor.outline.filter.ControlRequest"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.eokDqa|sWDRsn.PcHVi0|wZWgRW
                      signature: '"xy.ai.workbench.editor.outline.filter.ControlRequest"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB
                    signature: name="Control Requests"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.So5ris|NaiVSB.8m5wpk|bGfErc
                      signature: '"Control Requests"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa
                    signature: description="Hides control request section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.6rSLMj|GArJAa.Am8re5|wXmtot
                      signature: '"Hides control request section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj
                    signature: class="xy.ai.workbench.editor.outline.filter.ControlRequestFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.fUov7V|xom5Yj.NLHRDD|guLwlj.H1umdD|i4g81Y
                      signature: '"xy.ai.workbench.editor.outline.filter.ControlRequestFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|0XXUDs.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_14
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t
                    signature: id="xy.ai.workbench.editor.outline.filter.Page"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.AUjO57|vfBc2t.iQTD11|pEWzBB
                      signature: '"xy.ai.workbench.editor.outline.filter.Page"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn
                    signature: name="Page"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.Ekq2yg|jTisZn.8c5J8K|CxykUr
                      signature: '"Page"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv
                    signature: description="Hides the page section node."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.YvtBwe|MsEKpv.qlM2xX|eVZLOx
                      signature: '"Hides the page section node."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf
                    signature: class="xy.ai.workbench.editor.outline.filter.PageFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.QuFdPj|S8SxkI.LijKPw|2bquFf.aX5Vnr|zMSRQ2
                      signature: '"xy.ai.workbench.editor.outline.filter.PageFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|gCpXPQ.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_15
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5
                    signature: id="xy.ai.workbench.editor.outline.filter.UserSection"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.qsbL7a|q9XqX5.hOJA1r|rw9YPb
                      signature: '"xy.ai.workbench.editor.outline.filter.UserSection"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX
                    signature: name="User Sections"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.XnbQDU|JnsROX.CF71YR|kT9HyH
                      signature: '"User Sections"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2
                    signature: description="Hides chat 'user' section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.n84hmQ|5Z80p2.5dl3ms|vzaUoh
                      signature: '"Hides chat ''user'' section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU
                    signature: class="xy.ai.workbench.editor.outline.filter.UserSectionFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.1WiHPF|NQDrFt.dwRYVv|2zTmVU.qoA7Qz|fQ85J7
                      signature: '"xy.ai.workbench.editor.outline.filter.UserSectionFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|4oMksK.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|TL9HIp_16
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D
                signature: <commonFilter
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0
                  signature: <commonFilter
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.fi1G8M|LNNjyn
                    signature: commonFilter
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO
                    signature: id="xy.ai.workbench.editor.outline.filter.AgentSection"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.1gLJpy|4N96nO.AKopva|U8WCb7
                      signature: '"xy.ai.workbench.editor.outline.filter.AgentSection"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I
                    signature: name="Agent Sections"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.jquErn|x4gh5I.JC7F6w|NDcz4F
                      signature: '"Agent Sections"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36
                    signature: description="Hides chat 'agent' section nodes."
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36.kfAsxJ|KEJF9f
                      signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.tEAaKn|w6SC36.Mtdu1z|I8AViW
                      signature: '"Hides chat ''agent'' section nodes."'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av
                    signature: class="xy.ai.workbench.editor.outline.filter.AgentSectionFilter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.D16zLU|5TzZe0.5vq60P|gJL3Av.yo85ZA|Xp2xQ7
                      signature: '"xy.ai.workbench.editor.outline.filter.AgentSectionFilter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.gw4WFJ|7yQQ3k
                  signature: </commonFilter>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.Bc8V3e|AUPR2D.gw4WFJ|7yQQ3k.fi1G8M|LNNjyn
                    signature: commonFilter
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.93rcGd|NTayDa.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.EORwup|rnriRW.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_5
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg
            signature: <extension point="org.eclipse.ui.navigator.viewer">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E
              signature: <extension point="org.eclipse.ui.navigator.viewer">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm
                signature: point="org.eclipse.ui.navigator.viewer"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.S5WFJC|0FOW9E.IawMLD|Za8Mjm.SkBkM4|1o82R9
                  signature: '"org.eclipse.ui.navigator.viewer"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB
              signature: <viewer
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az
                signature: <viewer
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY
                  signature: <viewer
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.ua3BFe|Rs0wgv
                    signature: viewer
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L
                    signature: viewerId="xy.ai.workbench.editor.outline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.dw4bMf|ZjUDyY.1SbAg6|nNuh1L.ZbPM9C|ExhMhZ
                      signature: '"xy.ai.workbench.editor.outline"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.sfHnVX|iIrjUR
                  signature: </viewer>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.EIuuWX|TMi4az.sfHnVX|iIrjUR.ua3BFe|Rs0wgv
                    signature: viewer
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi
                signature: <viewerContentBinding viewerId="xy.ai.workbench.editor.outline">
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C
                  signature: <viewerContentBinding viewerId="xy.ai.workbench.editor.outline">
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.pgutWt|OLSIV4
                    signature: viewerContentBinding
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L
                    signature: viewerId="xy.ai.workbench.editor.outline"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.UbvYwP|zGXG7C.1SbAg6|nNuh1L.ZbPM9C|ExhMhZ
                      signature: '"xy.ai.workbench.editor.outline"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6
                  signature: <includes>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I
                    signature: <includes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.PznH2s|SpglhF
                      signature: <includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.PznH2s|SpglhF.SLI9rh|d9FNDu
                        signature: includes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg
                      signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55
                        signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55
                          signature: <contentExtension pattern="xy\.ai\.workbench\.editor\.outline\..*" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo
                            signature: pattern="xy\.ai\.workbench\.editor\.outline\..*"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.3e4FW1|bbRa55.3e4FW1|bbRa55.SYJ3gc|pNMELo.Fp91gh|Fp91gh
                              signature: '"xy\.ai\.workbench\.editor\.outline\..*"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.3e4FW1|zZV4cg.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.J5mD7W|C6U2p5
                      signature: </includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJrlwP|Envh3I.J5mD7W|C6U2p5.SLI9rh|d9FNDu
                        signature: includes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi
                    signature: <excludes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.ogdq0u|7jxwct
                      signature: <excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.ogdq0u|7jxwct.QMIIlf|8BvLOQ
                        signature: excludes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng
                      signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p
                        signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p
                          signature: <contentExtension pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*" />
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ
                            signature: pattern="^(?!xy\.ai\.workbench\.editor\.outline\.).*"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.cJXlWr|gQbo2p.cJXlWr|gQbo2p.n01PA4|oT8LYJ.bykh6r|neEyp9
                              signature: '"^(?!xy\.ai\.workbench\.editor\.outline\.).*"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.cJXlWr|ZFc1ng.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.d9fOjf|ms0G0O
                      signature: </excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.PZlL5U|xiUOHi.d9fOjf|ms0G0O.QMIIlf|8BvLOQ
                        signature: excludes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.fY9hkw|ObshX6.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.a8b6Hz|n6s26d
                  signature: </viewerContentBinding>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.TewZ58|aoPOUi.a8b6Hz|n6s26d.pgutWt|OLSIV4
                    signature: viewerContentBinding
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.yIw0Hz|DDnZdB.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|pKGosg.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_6
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx
            signature: <extension point="org.eclipse.ui.navigator.viewer">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E
              signature: <extension point="org.eclipse.ui.navigator.viewer">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm
                signature: point="org.eclipse.ui.navigator.viewer"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.S5WFJC|0FOW9E.IawMLD|Za8Mjm.SkBkM4|1o82R9
                  signature: '"org.eclipse.ui.navigator.viewer"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D
              signature: <viewer
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV
                signature: <viewer
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5
                  signature: <viewer
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.ua3BFe|Rs0wgv
                    signature: viewer
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx
                    signature: inheritBindingsFromViewer="org.eclipse.ui.navigator.ProjectExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx.qboCpm|m5RLGP
                      signature: inheritBindingsFromViewer
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.TiGHK2|j9rdQx.rzvUxu|7FcjP4
                      signature: '"org.eclipse.ui.navigator.ProjectExplorer"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2
                    signature: popupMenuId="org.eclipse.ui.navigator.ProjectExplorer#PopupMenu"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2.iVj8Tf|AE8XkR
                      signature: popupMenuId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.k9gJQH|gl0Ic2.1Tb4iT|IlDjmx
                      signature: '"org.eclipse.ui.navigator.ProjectExplorer#PopupMenu"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1
                    signature: viewerId="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sQ1xhV|yKTVs5.oSaGjO|k5Y0T1.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.jJEqAm|QC6vXz
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.jJEqAm|QC6vXz.jJEqAm|QC6vXz
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sfHnVX|iIrjUR
                  signature: </viewer>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.rPyCoU|aXzHcV.sfHnVX|iIrjUR.ua3BFe|Rs0wgv
                    signature: viewer
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|IckjY4
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ
                signature: <viewerContentBinding viewerId="xy.ai.workbench.view.AIExplorer">
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX
                  signature: <viewerContentBinding viewerId="xy.ai.workbench.view.AIExplorer">
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.pgutWt|OLSIV4
                    signature: viewerContentBinding
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1
                    signature: viewerId="xy.ai.workbench.view.AIExplorer"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1.AdNHAD|tupnF7
                      signature: viewerId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.Z1CLGj|Lw84dX.oSaGjO|k5Y0T1.GTjn9n|r7vJpB
                      signature: '"xy.ai.workbench.view.AIExplorer"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor
                  signature: <includes>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|8FggJ5
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU
                    signature: <includes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.PznH2s|SpglhF
                      signature: <includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.PznH2s|SpglhF.SLI9rh|d9FNDu
                        signature: includes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy
                      signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG
                        signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG
                          signature: <contentExtension pattern="xy.ai.workbench.view.content.AIContent"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd
                            signature: pattern="xy.ai.workbench.view.content.AIContent"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.vOL3dk|oZGTpG.vOL3dk|oZGTpG.8lbWPc|9ldwUd.omHzOp|omHzOp
                              signature: '"xy.ai.workbench.view.content.AIContent"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_1
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp
                        signature: <contentExtension pattern="xy.ai.workbench.view.filter.GitIgnore"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp
                          signature: <contentExtension pattern="xy.ai.workbench.view.filter.GitIgnore"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU
                            signature: pattern="xy.ai.workbench.view.filter.GitIgnore"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.ZdoGdn|w08hwp.ZdoGdn|w08hwp.tGT21f|7LriZU.CjbaZR|jucSxJ
                              signature: '"xy.ai.workbench.view.filter.GitIgnore"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_2
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS
                        signature: <contentExtension pattern="xy.ai.workbench.view.filter.Project"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS
                          signature: <contentExtension pattern="xy.ai.workbench.view.filter.Project"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp
                            signature: pattern="xy.ai.workbench.view.filter.Project"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.Jeirtu|OjjKbS.Jeirtu|OjjKbS.DMT5AV|KAvASp.O0IM2i|dyYaAn
                              signature: '"xy.ai.workbench.view.filter.Project"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|dzey2j_3
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG
                        signature: <contentExtension pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG
                          signature: <contentExtension pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi
                            signature: pattern="xy.ai.workbench.view.linkHelper.AILinkHelper"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.xjtynF|HJNAPG.xjtynF|HJNAPG.9Eb56i|itSwmi.cRofm0|moUsZD
                              signature: '"xy.ai.workbench.view.linkHelper.AILinkHelper"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.xjtynF|oGLoSy.jJEqAm|8FggJ5
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.J5mD7W|C6U2p5
                      signature: </includes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.VlcjZh|qYOBqU.J5mD7W|C6U2p5.SLI9rh|d9FNDu
                        signature: includes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|8FggJ5_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS
                    signature: <excludes>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.ogdq0u|7jxwct
                      signature: <excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.ogdq0u|7jxwct.QMIIlf|8BvLOQ
                        signature: excludes
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI
                      signature: <contentExtension
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.jJEqAm|6wcsPo
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP
                        signature: <contentExtension
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww
                          signature: <contentExtension
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.k7pAMI|paRB1P
                            signature: contentExtension
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX
                            signature: pattern="^(?!xy\.ai\.workbench\.view\.(content\.AIContent|filter\.(GitIgnore|Pr…
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX.3LBbAi|J0zr3j
                              signature: pattern
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.eVtyVv|fMs5ww.IcQAj1|06kXNX.P3pzrq|OOBdha
                              signature: '"^(?!xy\.ai\.workbench\.view\.(content\.AIContent|filter\.(GitIgnore|Project)|l…'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.jJEqAm|6wcsPo
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.jJEqAm|6wcsPo.jJEqAm|6wcsPo
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.Rysgvf|uDSGsB
                          signature: </contentExtension>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.Q0PBFw|qkPDFP.Rysgvf|uDSGsB.k7pAMI|paRB1P
                            signature: contentExtension
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.Q0PBFw|agc8hI.jJEqAm|8FggJ5
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.d9fOjf|ms0G0O
                      signature: </excludes>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.a8RNVK|jlUvNS.d9fOjf|ms0G0O.QMIIlf|8BvLOQ
                        signature: excludes
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.3CWcqf|oAzeor.jJEqAm|IckjY4
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.a8b6Hz|n6s26d
                  signature: </viewerContentBinding>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.TewZ58|YolBCZ.a8b6Hz|n6s26d.pgutWt|OLSIV4
                    signature: viewerContentBinding
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.VzIYf8|4nX38D.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.zm7u9I|2O2gWx.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_7
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS
            signature: <extension point="org.eclipse.ui.navigator.linkHelper">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA
              signature: <extension point="org.eclipse.ui.navigator.linkHelper">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT
                signature: point="org.eclipse.ui.navigator.linkHelper"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.TTfpNf|R0bNIA.BMghtn|OhSjrT.Vi712B|nWCHWi
                  signature: '"org.eclipse.ui.navigator.linkHelper"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy
              signature: <linkHelper
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L
                signature: <linkHelper
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM
                  signature: <linkHelper
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.lg7K4l|S8cI5f
                    signature: linkHelper
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G
                    signature: class="xy.ai.workbench.view.explorer.MarkdownLinkHelper"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.DlAbZZ|u6A58G.6UK0zx|9VEsjb
                      signature: '"xy.ai.workbench.view.explorer.MarkdownLinkHelper"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB
                    signature: id="xy.ai.workbench.view.linkHelper.AILinkHelper"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.Jwu5ML|ZCFYCM.503UcR|9yZurB.cRofm0|moUsZD
                      signature: '"xy.ai.workbench.view.linkHelper.AILinkHelper"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr
                  signature: <selectionEnablement>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W
                    signature: <selectionEnablement>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.u3mwY7|23XHiL
                      signature: <selectionEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.u3mwY7|23XHiL.wxOPh0|Pt4zSU
                        signature: selectionEnablement
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi
                      signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG
                        signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG
                          signature: <instanceof value="org.eclipse.core.resources.IFile"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X
                            signature: value="org.eclipse.core.resources.IFile"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.nKJAV4|KC6SAG.nKJAV4|KC6SAG.Wj7Z3X|Wj7Z3X.UM4a6T|hSin1y
                              signature: '"org.eclipse.core.resources.IFile"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.nKJAV4|ovLxMi.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.6UKhOA|JfM6gv
                      signature: </selectionEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.u3mwY7|OdU71W.6UKhOA|JfM6gv.wxOPh0|Pt4zSU
                        signature: selectionEnablement
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|fCL9ED_1
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x
                    signature: <editorInputEnablement>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.f1R2gP|KPuUL8
                      signature: <editorInputEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.f1R2gP|KPuUL8.z3DTBL|brwZNR
                        signature: editorInputEnablement
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0
                      signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3
                        signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3
                          signature: <instanceof value="org.eclipse.ui.IFileEditorInput"/>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.NuJ4WF|IFqwGX
                            signature: instanceof
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx
                            signature: value="org.eclipse.ui.IFileEditorInput"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx.qPnJq7|FsZIq9
                              signature: value
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.LxDVff|AigJS3.LxDVff|AigJS3.pZJmxw|QAI3cx.1HrI5f|nRoZBR
                              signature: '"org.eclipse.ui.IFileEditorInput"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.LxDVff|uOZeK0.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.zTA4ZI|jAepFK
                      signature: </editorInputEnablement>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.f1R2gP|bZMw0x.zTA4ZI|jAepFK.z3DTBL|brwZNR
                        signature: editorInputEnablement
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.WO5uJM|iIrofr.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.p5cQxp|KB59Jh
                  signature: </linkHelper>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.1y75aq|559N2L.p5cQxp|KB59Jh.lg7K4l|S8cI5f
                    signature: linkHelper
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.1y75aq|S95gmy.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.KcDX2T|g2mXeS.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_8
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo
                signature: point="org.eclipse.ui.commands"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.0EWWZa|2lJt5U.tzzVWX|OS7SGo.7bGT2u|KS0dN6
                  signature: '"org.eclipse.ui.commands"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp
              signature: <category
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU
                    signature: id="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.6yU4v8|oZY6qU.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K
                    signature: name="AI"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.AR9cTM|gCDCcw.53ORm9|LJ5w4K.08u1S9|yreODC
                      signature: '"AI"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.hvllzw|DYpMYW.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6
                    signature: id="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.lt38bw|ae8Gh6.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR
                    signature: name="Submit prompt"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.B57vMb|wdSSI8.LfygBB|OcqJyR.3AMCga|HTPMmq
                      signature: '"Submit prompt"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.Dl8OrP|8CLBo0.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4
                    signature: id="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.IKxclO|fcYHO4.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv
                    signature: name="Toggle AI Comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.WK6qvY|DM5SAD.W1AFws|I4taqv.iJYSVT|W1kbZ9
                      signature: '"Toggle AI Comment"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.oBGIKx|ci46ea.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT
                signature: <command
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.Oc6MrV|FCuBYJ
                    signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa
                    signature: categoryId="XY.AI.Workbench.commands.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa.pjeziZ|1NtAvK
                      signature: categoryId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.XlyP86|ihitOa.83PCGt|N1DxNd
                      signature: '"XY.AI.Workbench.commands.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1
                    signature: id="XY.AI.Workbench.commands.copyPath"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.sTRwHT|kxtNu1.4xZaKz|rCdYxS
                      signature: '"XY.AI.Workbench.commands.copyPath"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz
                    signature: name="Copy Path"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.pOKVGW|IaJcdz.vaUph0|JqbrZz.mjK4OU|ppKfXr
                      signature: '"Copy Path"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.YOB4cr|h9FdVz
                  signature: </command>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.fgiPwe|m4zEiT.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                    signature: command
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.rvFOyE|JIBZFp.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.cf3kfx|jTTs3h.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_9
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW
                signature: point="org.eclipse.ui.handlers"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.wqVSn6|Oi1ioL.48gMkc|AzZeNW.rFLkJq|kkOmqU
                  signature: '"org.eclipse.ui.handlers"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6
              signature: <handler
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk
                    signature: class="xy.ai.workbench.handlers.SessionExecuteHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.GMZKKB|I3enqk.D0XCfg|xWv0Gr
                      signature: '"xy.ai.workbench.handlers.SessionExecuteHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc
                    signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.XAKg0B|BKPNHd.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.5DHCKm|KP5acR.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW
                    signature: class="xy.ai.workbench.editor.CommentHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.FNwIi9|ECKkLW.sFp8br|fp5pCz
                      signature: '"xy.ai.workbench.editor.CommentHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K
                    signature: commandId="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.gqYhr2|dzcCzg.LPlBPi|VgWb2K.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv
                  signature: <activeWhen>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o
                    signature: <activeWhen>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.wpUKMU|tzVdHu
                      signature: <activeWhen>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.wpUKMU|tzVdHu.y382q5|iuqWJY
                        signature: activeWhen
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6
                      signature: <and>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19
                        signature: <and>
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.tiyh6R|hf1Hug
                          signature: <and>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.tiyh6R|hf1Hug.A9Fz3K|heucGq
                            signature: and
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l
                          signature: <with variable="activeEditorId">
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3
                            signature: <with variable="activeEditorId">
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f
                              signature: <with variable="activeEditorId">
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F
                                signature: variable="activeEditorId"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.RJRzBR|5Tev3f.bCCBYM|0JwV3F.rxgImd|NltelH
                                  signature: '"activeEditorId"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ
                              signature: <equals
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC
                                signature: <equals
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM
                                  signature: <equals
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.gwouS4|Auw28O
                                    signature: equals
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt
                                    signature: value="xy.ai.workbench.editor.AISessionEditor"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt.qPnJq7|FsZIq9
                                      signature: value
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.LY52lE|XJdavM.HHh0F7|1rv3mt.9ArSh7|9ArSh7
                                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.jJEqAm|AJUbxG
                                  signature: ""
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.jJEqAm|AJUbxG.jJEqAm|AJUbxG
                                    signature: ""
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.bfSCV1|Wo3wch
                                  signature: </equals>
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.FvXK6C|mfv8XC.bfSCV1|Wo3wch.gwouS4|Auw28O
                                    signature: equals
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.FvXK6C|6x3rzQ.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.AdDOBO|G9tRA3.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|Pg0Uv6_1
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1
                            signature: <with variable="activeEditor">
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6
                              signature: <with variable="activeEditor">
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L
                                signature: variable="activeEditor"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.xQfBGq|yLAUh6.ad7clM|uhD00L.NPsmv6|IfD7NP
                                  signature: '"activeEditor"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe
                              signature: <instanceof
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn
                                signature: <instanceof
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn
                                  signature: <instanceof
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.NuJ4WF|IFqwGX
                                    signature: instanceof
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt
                                    signature: value="xy.ai.workbench.editor.AISessionEditor"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt.qPnJq7|FsZIq9
                                      signature: value
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.JF0fWc|CsxZUn.JF0fWc|CsxZUn.HHh0F7|1rv3mt.9ArSh7|9ArSh7
                                      signature: '"xy.ai.workbench.editor.AISessionEditor"'
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.JF0fWc|1NahAe.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.XWnfxN|i0L3U1.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.XWnfxN|y3kZ1l.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.ifqcoG|BK79Nc
                          signature: </and>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.ruXGEh|Sxlp19.ifqcoG|BK79Nc.A9Fz3K|heucGq
                            signature: and
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.ruXGEh|yPjQD6.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.tUi396|gYLJKs
                      signature: </activeWhen>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.kDi41S|sxr38o.tUi396|gYLJKs.y382q5|iuqWJY
                        signature: activeWhen
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.kDi41S|SWvagv.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.1l4mdz|d7wqDL.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28
                signature: <handler
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn
                  signature: <handler
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.3oqRPH|jJDzOn
                    signature: handler
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN
                    signature: class="xy.ai.workbench.handlers.CopyPathHandler"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.sMOJAH|mmBpLN.AKGYYb|FzbKoO
                      signature: '"xy.ai.workbench.handlers.CopyPathHandler"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB
                    signature: commandId="XY.AI.Workbench.commands.copyPath"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.k07Dy0|1vxlnn.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                      signature: '"XY.AI.Workbench.commands.copyPath"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.999qAW|XuQOg9
                  signature: </handler>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.ON6jPI|vXDj28.999qAW|XuQOg9.3oqRPH|jJDzOn
                    signature: handler
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.ON6jPI|e7xRA6.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.nvlxzc|w48NNY.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|Yz4xOn
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD
                signature: point="org.eclipse.ui.bindings"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.4W6SGz|Qi2i0l.c2RhXJ|rYhHDD.HKEsGM|9hjZZL
                  signature: '"org.eclipse.ui.bindings"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT
              signature: <key
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C
                signature: <key
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35
                  signature: <key
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.4TdmK0|qkRRub
                    signature: key
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc
                    signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                      signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc
                    signature: contextId="org.eclipse.ui.contexts.window"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc.F2nBbM|dKNe6M
                      signature: contextId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.bRLbXj|53UDdc.FcPD79|XLIrzD
                      signature: '"org.eclipse.ui.contexts.window"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h
                    signature: schemeId="org.eclipse.ui.defaultAcceleratorConfiguration"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h.6fwW2g|J7jRNW
                      signature: schemeId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.06BDHn|FSHe2h.8uVeeu|HHRPNe
                      signature: '"org.eclipse.ui.defaultAcceleratorConfiguration"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc
                    signature: sequence="M1+Enter"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc.HHda8J|vAsOQc
                      signature: sequence
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.9sJCi4|Sagg35.FFCZg7|FP1Zcc.ltqHfs|r3y6Vp
                      signature: '"M1+Enter"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.hHAwQC|ybB0ha
                  signature: </key>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.42iKiI|PR0f8C.hHAwQC|ybB0ha.4TdmK0|qkRRub
                    signature: key
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw
                signature: <key
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG
                  signature: <key
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.4TdmK0|qkRRub
                    signature: key
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K
                    signature: commandId="XY.AI.Workbench.commands.editor.comment"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K.EuTKrZ|y0A6mW
                      signature: commandId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.LPlBPi|VgWb2K.AUmPIR|zNISP9
                      signature: '"XY.AI.Workbench.commands.editor.comment"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8
                    signature: contextId="org.eclipse.ui.textEditorScope"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8.F2nBbM|dKNe6M
                      signature: contextId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.g2jzyC|pmQbx8.GrA6do|UFSgPb
                      signature: '"org.eclipse.ui.textEditorScope"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h
                    signature: schemeId="org.eclipse.ui.defaultAcceleratorConfiguration"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h.6fwW2g|J7jRNW
                      signature: schemeId
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.06BDHn|FSHe2h.8uVeeu|HHRPNe
                      signature: '"org.eclipse.ui.defaultAcceleratorConfiguration"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK
                    signature: sequence="Shift+Ctrl+C"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK.HHda8J|vAsOQc
                      signature: sequence
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.3MyVGo|4cDtOG.zJv8jC|ZnhKlK.zX292u|M82LgY
                      signature: '"Shift+Ctrl+C"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.hHAwQC|ybB0ha
                  signature: </key>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.86bnTn|CkJasw.hHAwQC|ybB0ha.4TdmK0|qkRRub
                    signature: key
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.86bnTn|mVTAoT.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mUdbYS|a8aRol.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_10
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s
                signature: point="org.eclipse.ui.menus"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.hEmDBF|Fvroi5.JNZcZK|P83u7s.iQU0Jy|eDUE1H
                  signature: '"org.eclipse.ui.menus"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE
              signature: <menuContribution
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR
                    signature: locationURI="menu:org.eclipse.ui.main.menu?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.LeCnwE|aydaew.LuU89k|NlNsTR.IKMSH7|nko9x0
                      signature: '"menu:org.eclipse.ui.main.menu?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h
                  signature: <menu
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF
                    signature: <menu
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw
                      signature: <menu
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.SBHj3k|GReCyA
                        signature: menu
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL
                        signature: id="XY.AI.Workbench.menus.aiMenu"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.uaiOAt|Ln9ZyL.FTbZzu|RBAvHS
                          signature: '"XY.AI.Workbench.menus.aiMenu"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi
                        signature: label="AI"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.Cokew4|dmbpLi.08u1S9|yreODC
                          signature: '"AI"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp
                        signature: mnemonic="A"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.nN9lTa|pG9wCw.RRcvAh|5BOlPp.r21tlZ|8RIQe5
                          signature: '"A"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql
                        signature: <command
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv
                          signature: <command
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.Oc6MrV|FCuBYJ
                            signature: command
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc
                            signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                              signature: commandId
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                              signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R
                            signature: icon="icons/sample.png"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R.czwUqY|rUELQe
                              signature: icon
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.DBDdf8|qtd60R.taJEpx|cIkXsp
                              signature: '"icons/sample.png"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC
                            signature: id="XY.AI.Workbench.menus.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC.sn6NDO|fEEdrX
                              signature: id
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.lt38bw|j2AScC.DaS0nL|7nuxuz
                              signature: '"XY.AI.Workbench.menus.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ
                            signature: mnemonic="S"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ.OrXo0A|j7rGdG
                              signature: mnemonic
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.5M6hQV|h5Ubcv.l1M8OW|Q5EHKJ.VnqTdH|eKF6Hh
                              signature: '"S"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.jJEqAm|qYdK3X
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.jJEqAm|qYdK3X.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.YOB4cr|h9FdVz
                          signature: </command>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.OVNaMI|1ql1ql.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                            signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.OVNaMI|tANmUe.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.68XUTe|nnTezY
                      signature: </menu>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.T0PWfU|flIeaF.68XUTe|nnTezY.SBHj3k|GReCyA
                        signature: menu
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.T0PWfU|bkYJ7h.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|TWQbRc.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh
                    signature: locationURI="toolbar:org.eclipse.ui.main.toolbar?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.irLsPt|JvYltw.L3B0E7|dgZpwh.Q0wEPK|dWbHVk
                      signature: '"toolbar:org.eclipse.ui.main.toolbar?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP
                  signature: <toolbar
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS
                    signature: <toolbar
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok
                      signature: <toolbar
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.Pqxk0r|DPsgBy
                        signature: toolbar
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv
                        signature: id="XY.AI.Workbench.toolbars.aiToolbar"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.jBoAEK|bi7Hok.hFNUzn|mNW2lv.jV8hIP|0jdHPv
                          signature: '"XY.AI.Workbench.toolbars.aiToolbar"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA
                        signature: <command
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI
                          signature: <command
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.Oc6MrV|FCuBYJ
                            signature: command
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc
                            signature: commandId="XY.AI.Workbench.commands.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc.EuTKrZ|y0A6mW
                              signature: commandId
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.q6j8v5|mOCSjc.zCiWwi|vDk9ru
                              signature: '"XY.AI.Workbench.commands.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R
                            signature: icon="icons/sample.png"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R.czwUqY|rUELQe
                              signature: icon
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.DBDdf8|qtd60R.taJEpx|cIkXsp
                              signature: '"icons/sample.png"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I
                            signature: id="XY.AI.Workbench.toolbars.submitPromptCommand"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I.sn6NDO|fEEdrX
                              signature: id
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.lt38bw|OFOI4I.fUJU0r|kMf99Q
                              signature: '"XY.AI.Workbench.toolbars.submitPromptCommand"'
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy
                            signature: tooltip="Submit the current prompt"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy.EiJDrG|df03Cg
                              signature: tooltip
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.dEzPP3|npuIaI.TQx0uw|IMYyXy.clbVRA|o2EP8a
                              signature: '"Submit the current prompt"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.jJEqAm|qYdK3X
                          signature: ""
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.jJEqAm|qYdK3X.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.YOB4cr|h9FdVz
                          signature: </command>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.9NTpET|xJgLhA.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                            signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.9NTpET|K9ITxt.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.kDAD27|8UkVvz
                      signature: </toolbar>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.IK6OoB|F0pXzS.kDAD27|8UkVvz.Pqxk0r|DPsgBy
                        signature: toolbar
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.IK6OoB|zfHWAP.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|WeFnQI.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9
                    signature: locationURI="popup:org.eclipse.ui.popup.any?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.mbv994|UAmJvx.w8Q4ED|SbnyI9.QKbbkR|cs61Ex
                      signature: '"popup:org.eclipse.ui.popup.any?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz
                    signature: <command
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.Oc6MrV|FCuBYJ
                        signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB
                        signature: commandId="XY.AI.Workbench.commands.copyPath"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                          signature: commandId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                          signature: '"XY.AI.Workbench.commands.copyPath"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK
                        signature: label="Copy Path"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.chW19c|D7zEGK.mjK4OU|ppKfXr
                          signature: '"Copy Path"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM
                        signature: mnemonic="P"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.WjJryQ|4zpkjM.dNBBXn|qAvOHO
                          signature: '"P"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC
                        signature: style="push"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC.dE0Bu2|40u8pt
                          signature: style
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.drvxPF|5QWNoI.PnjumK|vzLCPC.iZ2Ji5|xBEXZL
                          signature: '"push"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5
                      signature: <visibleWhen
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.jJEqAm|qYdK3X
                        signature: ""
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO
                        signature: <visibleWhen
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ
                          signature: <visibleWhen
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.xfgl7A|CZ6BIc
                            signature: visibleWhen
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR
                            signature: checkEnabled="false"
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR.iNUWdq|h63Sz6
                              signature: checkEnabled
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.OizFWI|HC3kLZ.RJjLPf|0nKaTR.T330Gd|8GhRd4
                              signature: '"false"'
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr
                          signature: <with
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.jJEqAm|Pg0Uv6
                            signature: ""
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe
                            signature: <with
                            children:
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy
                              signature: <with
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.HVOVj0|bxRQtX
                                signature: with
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN
                                signature: variable="selection"
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN.AEQB0X|bMpma3
                                  signature: variable
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.SBKB0p|uSeGIy.Dsoip9|mdH9JN.BjW2HA|mwdYpz
                                  signature: '"selection"'
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal
                              signature: <iterate
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.jJEqAm|AJUbxG
                                signature: ""
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey
                                signature: <iterate
                                children:
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr
                                  signature: <iterate
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.x3SDFq|4ShU3M
                                    signature: iterate
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP
                                    signature: ifEmpty="false"
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP.orryTb|rcDXxf
                                      signature: ifEmpty
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.mCtPXU|RKhHRr.yed9x3|f2UjZP.T330Gd|8GhRd4
                                      signature: '"false"'
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM
                                  signature: <adapt
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.jJEqAm|lb1F2O
                                    signature: ""
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA
                                    signature: <adapt
                                    children:
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR
                                      signature: <adapt
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.HBdMko|xmasne
                                        signature: adapt
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq
                                        signature: type="org.eclipse.core.resources.IResource"
                                        children:
                                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq.jjZD4X|PFMnGV
                                          signature: type
                                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.kVyEuY|yEBSjR.DiFYpL|9dMflq.sOMyEq|Zt1DOX
                                          signature: '"org.eclipse.core.resources.IResource"'
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.jJEqAm|lb1F2O
                                      signature: ""
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.jJEqAm|lb1F2O.jJEqAm|lb1F2O
                                        signature: ""
                                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.Ir3Iz7|hzIY4n
                                      signature: </adapt>
                                      children:
                                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.zzq7JZ|ui5SSA.Ir3Iz7|hzIY4n.HBdMko|xmasne
                                        signature: adapt
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.zzq7JZ|JpnlxM.jJEqAm|AJUbxG
                                    signature: ""
                                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.Zm8mGe|rPoUJu
                                  signature: </iterate>
                                  children:
                                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.beVLH8|KEbvey.Zm8mGe|rPoUJu.x3SDFq|4ShU3M
                                    signature: iterate
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.beVLH8|La4oal.jJEqAm|Pg0Uv6
                                signature: ""
                            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.S1Osve|S5JQIi
                              signature: </with>
                              children:
                              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.8c8A6J|FGJWPe.S1Osve|S5JQIi.HVOVj0|bxRQtX
                                signature: with
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.8c8A6J|yYuegr.jJEqAm|qYdK3X
                            signature: ""
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.UOKzmA|BEDiCw
                          signature: </visibleWhen>
                          children:
                          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.0XDb6r|H17wCO.UOKzmA|BEDiCw.xfgl7A|CZ6BIc
                            signature: visibleWhen
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.0XDb6r|ZYxjG5.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.YOB4cr|h9FdVz
                      signature: </command>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.m8Gqv2|lXqvzz.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                        signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.m8Gqv2|zG0IrG.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|ZwJPzk.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf
                signature: <menuContribution
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy
                  signature: <menuContribution
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.ESddou|tvZMyn
                    signature: menuContribution
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z
                    signature: locationURI="popup:#TextEditorContext?after=additions"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z.MHfPm9|59gnSV
                      signature: locationURI
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.sXONDc|Is0suy.dBMcEz|rLKb2z.eAvBjH|dUqdnI
                      signature: '"popup:#TextEditorContext?after=additions"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW
                  signature: <command
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi
                    signature: <command
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI
                      signature: <command
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.Oc6MrV|FCuBYJ
                        signature: command
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB
                        signature: commandId="XY.AI.Workbench.commands.copyPath"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB.EuTKrZ|y0A6mW
                          signature: commandId
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.YDmiOM|R8AOGB.4xZaKz|rCdYxS
                          signature: '"XY.AI.Workbench.commands.copyPath"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK
                        signature: label="Copy Path"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK.SHMA9X|LLI9P6
                          signature: label
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.chW19c|D7zEGK.mjK4OU|ppKfXr
                          signature: '"Copy Path"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM
                        signature: mnemonic="P"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM.OrXo0A|j7rGdG
                          signature: mnemonic
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.WjJryQ|4zpkjM.dNBBXn|qAvOHO
                          signature: '"P"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC
                        signature: style="push"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC.dE0Bu2|40u8pt
                          signature: style
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.drvxPF|5QWNoI.PnjumK|vzLCPC.iZ2Ji5|xBEXZL
                          signature: '"push"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.jJEqAm|fCL9ED
                      signature: ""
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.jJEqAm|fCL9ED.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.YOB4cr|h9FdVz
                      signature: </command>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.Nb8aPw|uJNUUi.YOB4cr|h9FdVz.Oc6MrV|FCuBYJ
                        signature: command
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.Nb8aPw|wH26xW.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.nZ7K7R|jrKik5
                  signature: </menuContribution>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.nadSKa|hCuwSf.nZ7K7R|jrKik5.ESddou|tvZMyn
                    signature: menuContribution
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.nadSKa|VenvPE.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.5PgE80|l7wjQS.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_11
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E
                signature: point="org.eclipse.ui.perspectiveExtensions"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.Bs1Y44|VVcHCE.Y6xI06|6F094E.nX3h2w|h61quw
                  signature: '"org.eclipse.ui.perspectiveExtensions"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv
              signature: <perspectiveExtension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e
                signature: <perspectiveExtension
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc
                  signature: <perspectiveExtension
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.EIoDLt|g8bVVr
                    signature: perspectiveExtension
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9
                    signature: targetID="org.eclipse.jdt.ui.JavaPerspective"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9.BTalmQ|yQH83Z
                      signature: targetID
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.olarfy|uTjFLc.J8qxTL|kPYGC9.DkrLzJ|uM36CM
                      signature: '"org.eclipse.jdt.ui.JavaPerspective"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim
                  signature: <view
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs
                    signature: <view
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL
                      signature: <view
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.5TDlNy|sUPndD
                        signature: view
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6
                        signature: id="xy.ai.workbench.view.AIBatchView"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6.sn6NDO|fEEdrX
                          signature: id
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.0T6syJ|DuUCg6.mUkaJL|qmLMSD
                          signature: '"xy.ai.workbench.view.AIBatchView"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8
                        signature: ratio="0.5"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8.Lm4FBm|9ccknR
                          signature: ratio
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.YJzx3R|BszrY8.R4BCWz|y8ITi7
                          signature: '"0.5"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z
                        signature: relationship="bottom"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z.LfQhts|oE9ING
                          signature: relationship
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.IxHBFH|5LOX1Z.snbwVD|WSXfv3
                          signature: '"bottom"'
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp
                        signature: relative="org.eclipse.ui.views.ProblemView"
                        children:
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp.iVAqCB|ZndawC
                          signature: relative
                        - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.dfvVAm|ArEOiL.OekD7V|c82yjp.GadPot|8B0m3m
                          signature: '"org.eclipse.ui.views.ProblemView"'
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.jJEqAm|fCL9ED
                      signature: ""
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.jJEqAm|fCL9ED.jJEqAm|fCL9ED
                        signature: ""
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.g8VJFp|xnenxG
                      signature: </view>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.c4W10T|kjNcVs.g8VJFp|xnenxG.5TDlNy|sUPndD
                        signature: view
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.c4W10T|4u5Gim.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.QGV5Hx|kz199O
                  signature: </perspectiveExtension>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.C5MSV8|GG8P0e.QGV5Hx|kz199O.EIoDLt|g8bVVr
                    signature: perspectiveExtension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.C5MSV8|rsdgKv.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.yQnSsP|muRwfA.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_12
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC
                signature: point="org.eclipse.ui.fontDefinitions"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.dco1Qb|l8f8KB.nSKOly|yYJyMC.CwvM9t|tWkcIv
                  signature: '"org.eclipse.ui.fontDefinitions"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1
              signature: <fontDefinition
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0
                signature: <fontDefinition
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O
                  signature: <fontDefinition
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.Iy0lps|8dN93u
                    signature: fontDefinition
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7
                    signature: id="XY.AI.Workbench.fontDefinition1"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.WFSl9r|izU7j7.cmYkeE|lDescl
                      signature: '"XY.AI.Workbench.fontDefinition1"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H
                    signature: label="XY.AI Editor Font"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H.SHMA9X|LLI9P6
                      signature: label
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.W6PwZk|b0Mg1H.EGONf6|pcMhiJ
                      signature: '"XY.AI Editor Font"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13
                    signature: value="1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13.qPnJq7|FsZIq9
                      signature: value
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.0qDbSZ|NVHT0O.9Bfylj|GAsN13.pWtfMs|2V6glq
                      signature: '"1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.thLBlY|dNx8bT
                  signature: </fontDefinition>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.29TU3z|31O4U0.thLBlY|dNx8bT.Iy0lps|8dN93u
                    signature: fontDefinition
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.29TU3z|nHtFv1.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.mkH8Aw|vOyCjc.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_13
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM
                signature: id="xy.ai.workbench.promptmarker"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM.sn6NDO|fEEdrX
                  signature: id
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.7GnX0l|rFABiM.388MRq|kISRzp
                  signature: '"xy.ai.workbench.promptmarker"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm
                signature: name="XY.AI Prompt Marker"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm.g6aFQH|AsA4NM
                  signature: name
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.PSz0iD|2gsERm.w35BmX|54YejC
                  signature: '"XY.AI Prompt Marker"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M
                signature: point="org.eclipse.core.resources.markers"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.gwkaP9|5UGKqb.6jlhtw|BfdM1M.Wi6POB|idD9hq
                  signature: '"org.eclipse.core.resources.markers"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK
              signature: <persistent
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy
                signature: <persistent
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q
                  signature: <persistent
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.pKzr8Y|gSjHco
                    signature: persistent
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR
                    signature: value="true"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR.qPnJq7|FsZIq9
                      signature: value
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.71pDiM|5xp86Q.Z055Mm|c6uDHR.v26ACe|dtG9Jc
                      signature: '"true"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.Yubk2a|1nElVp
                  signature: </persistent>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.8lY2XX|cYFJIy.Yubk2a|1nElVp.pKzr8Y|gSjHco
                    signature: persistent
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3
                signature: <super
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH
                  signature: <super
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.TbFdXP|Fwu00T
                    signature: super
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE
                    signature: type="org.eclipse.core.resources.taskmarker"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE.jjZD4X|PFMnGV
                      signature: type
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.diNBQk|nYyAZH.vVoM99|N09mlE.NHJ7sA|nwWkY3
                      signature: '"org.eclipse.core.resources.taskmarker"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.qhN0gU|B4EtHM
                  signature: </super>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.KAj2Si|2tI3X3.qhN0gU|B4EtHM.TbFdXP|Fwu00T
                    signature: super
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.dckkov|Zu4tLK.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.RhUOxm|iY3baR.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|eJrdFT_14
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD
            signature: <extension
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx
              signature: <extension
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD
                signature: point="org.eclipse.ui.newWizards"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.1CVTq2|ZY4cgx.ke7Ck4|8LqSxD.rFAdEM|yTKnr3
                  signature: '"org.eclipse.ui.newWizards"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0
              signature: <wizard
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS
                    signature: id="XY.AI.Workbench.wizard.newfile.markdown"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.dFZqNB|I7CtRS.HVBLms|PLxqhz
                      signature: '"XY.AI.Workbench.wizard.newfile.markdown"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE
                    signature: name="Markdown File"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.ZRsDYG|yZ6ZxE.vqspE0|LIu494
                      signature: '"Markdown File"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei
                    signature: class="xy.ai.workbench.handlers.NewMarkdownWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.094Gdo|gRDBei.n3oybL|s6148p
                      signature: '"xy.ai.workbench.handlers.NewMarkdownWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.tYg7hO|gsk29z.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.sS0phJ|Rt0Dm5
                      signature: Creates a new Markdown file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.sS0phJ|Rt0Dm5.sS0phJ|Rt0Dm5
                        signature: Creates a new Markdown file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.n68Dvo|WD7jWm.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.n68Dvo|FngPsV.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|liwWW5.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_1
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ
                    signature: id="XY.AI.Workbench.wizard.newfile.context"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.ccFuiL|P1uEVJ.lgf7na|lgf7na
                      signature: '"XY.AI.Workbench.wizard.newfile.context"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx
                    signature: name="Prompt Context"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.VSnlhx|WgnwZx.hwiILu|AyEw9S
                      signature: '"Prompt Context"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN
                    signature: class="xy.ai.workbench.handlers.NewPromptContextWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.RNVBZX|WD9LbN.Fd1IDY|b3xk2K
                      signature: '"xy.ai.workbench.handlers.NewPromptContextWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.tYg7hO|u1837D.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.47sWAP|ZLzPnS
                      signature: Creates a new Prompt Context file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.47sWAP|ZLzPnS.47sWAP|ZLzPnS
                        signature: Creates a new Prompt Context file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.JrdyEV|8CESmm.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.JrdyEV|5cUnxK.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|jZ78Pu.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_2
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV
                signature: <wizard
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k
                  signature: <wizard
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.4Jzw9h|ATE3Fh
                    signature: wizard
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX
                    signature: id="XY.AI.Workbench.wizard.newfile.chat"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.AfLQVX|AfLQVX.JoTiBt|0tBlvZ
                      signature: '"XY.AI.Workbench.wizard.newfile.chat"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1
                    signature: name="AI Chat"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.i5Yc4c|rYC1s1.Uv0WNH|afVFqZ
                      signature: '"AI Chat"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG
                    signature: class="xy.ai.workbench.handlers.NewChatWizard"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG.hRGK7L|AB8ZQq
                      signature: class
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.FXrZWo|VAbZgG.YKLUrz|YKLUrz
                      signature: '"xy.ai.workbench.handlers.NewChatWizard"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2
                    signature: category="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2.i1zovf|7H5IwG
                      signature: category
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.Ww4QYf|VXZgh2.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R
                    signature: icon="icons/sample.png"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R.czwUqY|rUELQe
                      signature: icon
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.tYg7hO|J19k2k.DBDdf8|qtd60R.taJEpx|cIkXsp
                      signature: '"icons/sample.png"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG
                  signature: <description>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.jJEqAm|fCL9ED
                    signature: ""
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ
                    signature: <description>
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.T75n3s|cpDhsJ
                      signature: <description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.T75n3s|cpDhsJ.kfAsxJ|KEJF9f
                        signature: description
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.TjIN8G|eJDblE
                      signature: Creates a new AI Chat file.
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.TjIN8G|eJDblE.TjIN8G|eJDblE
                        signature: Creates a new AI Chat file.
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.GN3fUj|pZPPeT
                      signature: </description>
                      children:
                      - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.JrdyEV|9dYNQQ.GN3fUj|pZPPeT.kfAsxJ|KEJF9f
                        signature: description
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.JrdyEV|3uMicG.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.gMHFGm|iRcxUG
                  signature: </wizard>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jIUivw|b1ZYGV.gMHFGm|iRcxUG.4Jzw9h|ATE3Fh
                    signature: wizard
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|TL9HIp_3
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM
                signature: <category
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud
                  signature: <category
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.i1zovf|7H5IwG
                    signature: category
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa
                    signature: id="XY.AI.Workbench.newfile.category"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa.sn6NDO|fEEdrX
                      signature: id
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.SdwG2o|VAERXa.YtIs1a|lg7USB
                      signature: '"XY.AI.Workbench.newfile.category"'
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM
                    signature: name="AI Files"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.LMoPpN|xo09Ud.1BIeG4|AUQ6aM.sRhT0k|AtmuUS
                      signature: '"AI Files"'
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.jJEqAm|TL9HIp
                  signature: ""
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.jJEqAm|TL9HIp.jJEqAm|TL9HIp
                    signature: ""
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.FuNHrp|o8YiGx
                  signature: </category>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.WRw1UK|wZqLUM.FuNHrp|o8YiGx.i1zovf|7H5IwG
                    signature: category
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.i6aPW9|FcLmB0.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.vzFZlq|fQgrQD.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|Yz4xOn_1
            signature: ""
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX
            signature: <extension point="org.eclipse.ui.editors.annotationTypes">
            children:
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9
              signature: <extension point="org.eclipse.ui.editors.annotationTypes">
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.Wj1AXH|e3Le1Q
                signature: extension
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y
                signature: point="org.eclipse.ui.editors.annotationTypes"
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y.TDkePX|YK266p
                  signature: point
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.YMD1Pl|B6t2V9.905eEz|VjVD4Y.Bt4b4H|Bt4b4H
                  signature: '"org.eclipse.ui.editors.annotationTypes"'
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1
              signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.jJEqAm|TL9HIp
                signature: ""
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs
                signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
                children:
                - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs
                  signature: <type name="xy.ai.workbench.editor.spellcheck.spelling"/>
                  children:
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.jjZD4X|PFMnGV
                    signature: type
                  - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb
                    signature: name="xy.ai.workbench.editor.spellcheck.spelling"
                    children:
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb.g6aFQH|AsA4NM
                      signature: name
                    - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.9syVL5|pu6fWs.9syVL5|pu6fWs.nH8iUC|1eK4zb.7mWWOn|QCXd55
                      signature: '"xy.ai.workbench.editor.spellcheck.spelling"'
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.9syVL5|R0eSH1.jJEqAm|eJrdFT
                signature: ""
            - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.lvnCsP|qUFF2a
              signature: </extension>
              children:
              - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.dBBPxh|zyehZX.lvnCsP|qUFF2a.Wj1AXH|e3Le1Q
                signature: extension
          - id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.jJEqAm|K0kEnq
            signature: ""
        - id: ODTrYv|DEw9ri.7mn2Ql|yyuEDp
          signature: </plugin>
          children:
          - id: ODTrYv|DEw9ri.7mn2Ql|yyuEDp.7cihA8|sDO1qh
            signature: plugin
```
/answer b655-1d allow
ReasoningToken: 41
Control Request:
```yaml
id: b655-1e
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
  reason: "Basisklasse Command inspizieren (parameter()-Methode, Konstruktor) fuer korrekten Zugriff auf die Trigger-ID beim Snapshot-Trigger in PromptHandler."
```
/answer b655-1e deny ein instanceof check
ReasoningToken: 6822
Control Request:
```yaml
id: b655-1f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: "Vollständige Import-Liste von PromptHandler.java einsehen, um neue Imports korrekt einzufügen und vorhandene Command-Imports zu identifizieren."
```
/answer b655-1f allow
Control Request:
```yaml
id: b655-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: FAE3JL|nGZQ8R
        signature: package xy.ai.workbench.connector.harness;
      - id: KEc50E|uGZWQu
        signature: import xy.ai.workbench.IncludeAdapter;
      - id: hMGSgH|1FhRB8
        signature: /**
      - id: PromptHandler
        signature: "public class PromptHandler {"
        children:
        - id: PromptHandler.QEglBi|eRrE2Z
          signature: private final AdaptingConnector connector;
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, ActiveEditorListener editorListener, Ad…"
        - id: PromptHandler.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: PromptHandler.addAnswerObs
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: PromptHandler.initializeInputs
          signature: "public void initializeInputs() {"
        - id: PromptHandler.execute
          signature: "public void execute(Display display) {"
        - id: PromptHandler.PromptJob
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
            signature: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, IProgressMonitor mon) {"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.preprocessPrompt
          signature: "private void preprocessPrompt(Prompt prompt) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
```
/answer b655-20 allow
ReasoningToken: 7760
Control Request:
```yaml
id: b655-21
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  regexp: ^import
  reason: "Vollständige Liste aller import-Anweisungen in PromptHandler.java ermitteln, um Namenskonflikte beim Hinzufügen des Snapshot-Triggers zu vermeiden."
```
/answer b655-21 allow
Control Request:
```yaml
id: b655-22
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes: []
```
/answer b655-22 allow
ReasoningToken: 1096
Control Request:
```yaml
id: b655-23
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  end_lineno: 15
  reason: "Alle Top-Level-Knoten am Dateianfang (Imports) ermitteln, um Namenskonflikte beim Einfuegen des Snapshot-Triggers zu vermeiden."
```
/answer b655-23 deny ignorier die import, ich werde sie ergänzen
ReasoningToken: 586
Control Request:
```yaml
id: b655-24
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/SnapshotResult.java
    source: "package xy.ai.workbench.view.diff;\n\nimport org.eclipse.jgit.lib.ObjectId;\n\n/**\n * Ergebnis eines Snapshot-Versuchs. {@code created=false} bedeutet: kein Delta zum\n * vorherigen Snapshot, es wurde kein neuer Commit angelegt.\n */\npublic record SnapshotResult(boolean created, String chain, String ref, ObjectId commit, ObjectId parent,\n\t\tString triggerLabel) {\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffRefreshBus.java
    source: "package xy.ai.workbench.view.diff;\n\nimport java.util.List;\nimport java.util.concurrent.CopyOnWriteArrayList;\nimport java.util.function.Consumer;\n\nimport org.eclipse.swt.widgets.Display;\n\n/** Einfacher Event-Bus: benachrichtigt das Diff-Panel nach jedem erzeugten Snapshot. */\npublic final class DiffRefreshBus {\n\n\tprivate static final List<Consumer<SnapshotResult>> LISTENERS = new CopyOnWriteArrayList<>();\n\n\tprivate DiffRefreshBus() {\n\t}\n\n\tpublic static void addListener(Consumer<SnapshotResult> listener) {\n\t\tLISTENERS.add(listener);\n\t}\n\n\tpublic static void removeListener(Consumer<SnapshotResult> listener) {\n\t\tLISTENERS.remove(listener);\n\t}\n\n\tpublic static void fire(SnapshotResult result) {\n\t\tDisplay display = Display.getDefault();\n\t\tif (display == null || display.isDisposed()) {\n\t\t\treturn;\n\t\t}\n\t\tdisplay.asyncExec(() -> LISTENERS.forEach(l -> l.accept(result)));\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpDiffReader.java
    source: "package xy.ai.workbench.view.diff;\n\nimport java.io.ByteArrayOutputStream;\nimport java.io.IOException;\nimport java.nio.charset.StandardCharsets;\n\nimport org.eclipse.jgit.diff.DiffFormatter;\nimport org.eclipse.jgit.lib.ObjectId;\nimport org.eclipse.jgit.lib.Repository;\nimport org.eclipse.jgit.revwalk.RevCommit;\nimport org.eclipse.jgit.revwalk.RevWalk;\n\n/**\n * Erzeugt einen aggregierten Unified Diff zwischen zwei Commits, äquivalent zu\n * {@code git diff <parent> <commit>}.\n */\npublic final class OpDiffReader {\n\n\tprivate OpDiffReader() {\n\t}\n\n\tpublic static String diff(Repository repository, ObjectId oldCommit, ObjectId newCommit) throws IOException {\n\t\ttry (ByteArrayOutputStream out = new ByteArrayOutputStream();\n\t\t\t\tDiffFormatter formatter = new DiffFormatter(out);\n\t\t\t\tRevWalk walk = new RevWalk(repository)) {\n\t\t\tformatter.setRepository(repository);\n\t\t\tRevCommit oldC = walk.parseCommit(oldCommit);\n\t\t\tRevCommit newC = walk.parseCommit(newCommit);\n\t\t\tformatter.format(oldC.getTree(), newC.getTree());\n\t\t\tformatter.flush();\n\t\t\treturn out.toString(StandardCharsets.UTF_8);\n\t\t}\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    source: "package xy.ai.workbench.view.diff;\n\nimport java.io.File;\nimport java.io.IOException;\nimport java.time.LocalDateTime;\nimport java.time.format.DateTimeFormatter;\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\nimport org.eclipse.core.resources.IProject;\nimport org.eclipse.core.resources.IWorkspaceRoot;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.jgit.api.Git;\nimport org.eclipse.jgit.api.errors.GitAPIException;\nimport org.eclipse.jgit.dircache.DirCache;\nimport org.eclipse.jgit.lib.CommitBuilder;\nimport org.eclipse.jgit.lib.ObjectId;\nimport org.eclipse.jgit.lib.ObjectInserter;\nimport org.eclipse.jgit.lib.PersonIdent;\nimport org.eclipse.jgit.lib.Ref;\nimport org.eclipse.jgit.lib.RefUpdate;\nimport org.eclipse.jgit.lib.RefUpdate.Result;\nimport org.eclipse.jgit.lib.Repository;\nimport org.eclipse.jgit.revwalk.RevCommit;\nimport org.eclipse.jgit.revwalk.RevWalk;\nimport org.eclipse.jgit.storage.file.FileRepositoryBuilder;\n\n/**\n * JGit-Port von {@code scripts/op-snapshot.sh}: legt Snapshots einer LLM-Operation als\n * Commits außerhalb von HEAD/Branch unter {@code refs/llm-ops/*} ab. Chain-Semantik,\n * Delta-Pruefung und Ref-Layout entsprechen dem Bash-Referenzskript.\n */\npublic final class OpSnapshotter {\n\n\tprivate static final String BASE_MARKER = \"refs/llm-ops/_base\";\n\tprivate static final Pattern OP_SUFFIX = Pattern.compile(\".*/op-(\\\\d+)$\");\n\tprivate static final DateTimeFormatter TS = DateTimeFormatter.ofPattern(\"yyyy-MM-dd HH:mm:ss\");\n\n\tprivate static OpSnapshotter instance;\n\n\tprivate final Repository repository;\n\n\tprivate OpSnapshotter(Repository repository) {\n\t\tthis.repository = repository;\n\t}\n\n\tpublic static synchronized OpSnapshotter getInstance() throws IOException {\n\t\tif (instance == null) {\n\t\t\tinstance = new OpSnapshotter(openRepository());\n\t\t}\n\t\treturn instance;\n\t}\n\n\tprivate static Repository openRepository() throws IOException {\n\t\tIWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();\n\t\tFile start = null;\n\t\tfor (IProject p : root.getProjects()) {\n\t\t\tif (p.isAccessible() && p.getLocation() != null) {\n\t\t\t\tstart = p.getLocation().toFile();\n\t\t\t\tbreak;\n\t\t\t}\n\t\t}\n\t\tif (start == null) {\n\t\t\tstart = root.getLocation().toFile();\n\t\t}\n\t\tFileRepositoryBuilder builder = new FileRepositoryBuilder().findGitDir(start);\n\t\tif (builder.getGitDir() == null) {\n\t\t\tthrow new IOException(\"Kein Git-Repository gefunden ab \" + start);\n\t\t}\n\t\treturn builder.build();\n\t}\n\n\tpublic Repository getRepository() {\n\t\treturn repository;\n\t}\n\n\t/**\n\t * Erstellt einen Snapshot, sofern sich der Arbeitsbaum seit dem letzten Snapshot der\n\t * aktiven Chain geaendert hat. {@code triggerLabel} fliesst nur in die Commit-Message ein.\n\t */\n\tpublic synchronized SnapshotResult snapshot(String triggerLabel) throws IOException {\n\t\tObjectId currentHead = repository.resolve(\"HEAD\");\n\t\tif (currentHead == null) {\n\t\t\tthrow new IOException(\"Kein HEAD gefunden (leeres Repository?).\");\n\t\t}\n\n\t\tObjectId storedBase = resolveRef(BASE_MARKER);\n\t\tif (storedBase == null || !storedBase.equals(currentHead)) {\n\t\t\t// HEAD hat sich seit dem letzten Snapshot bewegt (echter Commit) -> neue Chain.\n\t\t\tupdateRef(BASE_MARKER, currentHead);\n\t\t\tstoredBase = currentHead;\n\t\t}\n\n\t\tString chainId = repository.newObjectReader().abbreviate(storedBase, 7).name();\n\t\tString namespace = \"refs/llm-ops/chain-\" + chainId;\n\n\t\tint lastN = 0;\n\t\tString lastOpRefName = null;\n\t\tfor (Ref ref : repository.getRefDatabase().getRefsByPrefix(namespace + \"/op-\")) {\n\t\t\tMatcher m = OP_SUFFIX.matcher(ref.getName());\n\t\t\tif (m.matches()) {\n\t\t\t\tint n = Integer.parseInt(m.group(1));\n\t\t\t\tif (n > lastN) {\n\t\t\t\t\tlastN = n;\n\t\t\t\t\tlastOpRefName = ref.getName();\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tObjectId parent = lastOpRefName != null ? resolveRef(lastOpRefName) : storedBase;\n\n\t\t// Aequivalent zu \"git add -A\": neue/geaenderte Dateien sowie Loeschungen erfassen.\n\t\ttry (Git git = new Git(repository)) {\n\t\t\tgit.add().addFilepattern(\".\").call();\n\t\t\tgit.add().addFilepattern(\".\").setUpdate(true).call();\n\t\t} catch (GitAPIException e) {\n\t\t\tthrow new IOException(e);\n\t\t}\n\n\t\tObjectId treeId;\n\t\ttry (ObjectInserter inserter = repository.newObjectInserter()) {\n\t\t\tDirCache cache = repository.readDirCache();\n\t\t\ttreeId = cache.writeTree(inserter);\n\t\t\tinserter.flush();\n\t\t}\n\n\t\tObjectId parentTree;\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tparentTree = walk.parseCommit(parent).getTree();\n\t\t}\n\n\t\tif (treeId.equals(parentTree)) {\n\t\t\t// Kein Delta zum letzten Snapshot -> kein neuer Commit (analog Bash-Skript).\n\t\t\treturn new SnapshotResult(false, namespace, null, parent, parent, triggerLabel);\n\t\t}\n\n\t\tint n = lastN + 1;\n\t\tString message = \"op-\" + n + \" [\" + (triggerLabel == null ? \"?\" : triggerLabel) + \"] \"\n\t\t\t\t+ LocalDateTime.now().format(TS);\n\n\t\tObjectId newCommit;\n\t\ttry (ObjectInserter inserter = repository.newObjectInserter()) {\n\t\t\tCommitBuilder cb = new CommitBuilder();\n\t\t\tcb.setTreeId(treeId);\n\t\t\tcb.setParentId(parent);\n\t\t\tPersonIdent ident = new PersonIdent(\"xy.ai.workbench\", \"noreply@xy.ai.workbench\");\n\t\t\tcb.setAuthor(ident);\n\t\t\tcb.setCommitter(ident);\n\t\t\tcb.setMessage(message);\n\t\t\tnewCommit = inserter.insert(cb);\n\t\t\tinserter.flush();\n\t\t}\n\n\t\tString refName = namespace + \"/op-\" + n;\n\t\tupdateRef(refName, newCommit);\n\n\t\treturn new SnapshotResult(true, namespace, refName, newCommit, parent, triggerLabel);\n\t}\n\n\t/** Liefert den ueber alle Chains hinweg juengsten Snapshot (fuer das Diff-Panel). */\n\tpublic SnapshotResult findLatest() throws IOException {\n\t\tRef best = null;\n\t\tint bestTime = Integer.MIN_VALUE;\n\t\tfor (Ref ref : repository.getRefDatabase().getRefsByPrefix(\"refs/llm-ops/\")) {\n\t\t\tif (!OP_SUFFIX.matcher(ref.getName()).matches()) {\n\t\t\t\tcontinue;\n\t\t\t}\n\t\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\t\tRevCommit c = walk.parseCommit(ref.getObjectId());\n\t\t\t\tif (c.getCommitTime() > bestTime) {\n\t\t\t\t\tbestTime = c.getCommitTime();\n\t\t\t\t\tbest = ref;\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tif (best == null) {\n\t\t\treturn null;\n\t\t}\n\t\tString chain = best.getName().substring(0, best.getName().lastIndexOf('/'));\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tRevCommit c = walk.parseCommit(best.getObjectId());\n\t\t\tObjectId parent = c.getParentCount() > 0 ? c.getParent(0) : best.getObjectId();\n\t\t\treturn new SnapshotResult(true, chain, best.getName(), best.getObjectId(), parent, null);\n\t\t}\n\t}\n\n\tprivate ObjectId resolveRef(String name) throws IOException {\n\t\tRef ref = repository.exactRef(name);\n\t\treturn ref == null ? null : ref.getObjectId();\n\t}\n\n\tprivate void updateRef(String name, ObjectId id) throws IOException {\n\t\tRefUpdate update = repository.updateRef(name);\n\t\tupdate.setNewObjectId(id);\n\t\tupdate.setForceUpdate(true);\n\t\tResult result = update.update();\n\t\tif (result != Result.NEW && result != Result.FORCED && result != Result.FAST_FORWARD\n\t\t\t\t&& result != Result.NO_CHANGE) {\n\t\t\tthrow new IOException(\"Ref-Update fehlgeschlagen fuer \" + name + \": \" + result);\n\t\t}\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
    source: "package xy.ai.workbench.view.diff;\n\nimport java.io.IOException;\nimport java.util.function.Consumer;\n\nimport org.eclipse.jface.resource.JFaceResources;\nimport org.eclipse.jgit.lib.Repository;\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.layout.GridData;\nimport org.eclipse.swt.layout.GridLayout;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Label;\nimport org.eclipse.swt.widgets.Text;\nimport org.eclipse.ui.part.ViewPart;\n\n/**\n * Singleton-View: zeigt stets den juengsten Git-Snapshot (aggregierter Unified Diff)\n * einer LLM-Operation. Wird ueber {@link DiffRefreshBus} nach jedem Snapshot aktualisiert.\n */\npublic class DiffPanel extends ViewPart {\n\n\tpublic static final String ID = \"xy.ai.workbench.view.diff.DiffPanel\";\n\n\tprivate Label statusLabel;\n\tprivate Text diffText;\n\tprivate final Consumer<SnapshotResult> listener = this::onSnapshot;\n\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tparent.setLayout(new GridLayout(1, false));\n\n\t\tstatusLabel = new Label(parent, SWT.NONE);\n\t\tstatusLabel.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));\n\t\tstatusLabel.setText(\"Kein Snapshot vorhanden.\");\n\n\t\tdiffText = new Text(parent, SWT.MULTI | SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL | SWT.BORDER);\n\t\tdiffText.setLayoutData(new GridData(SWT.FILL, SWT.FILL, true, true));\n\t\tdiffText.setFont(JFaceResources.getTextFont());\n\n\t\tDiffRefreshBus.addListener(listener);\n\t\trefreshLatest();\n\t}\n\n\tprivate void refreshLatest() {\n\t\ttry {\n\t\t\tSnapshotResult latest = OpSnapshotter.getInstance().findLatest();\n\t\t\tif (latest != null) {\n\t\t\t\tonSnapshot(latest);\n\t\t\t}\n\t\t} catch (IOException e) {\n\t\t\tstatusLabel.setText(\"Fehler: \" + e.getMessage());\n\t\t}\n\t}\n\n\tprivate void onSnapshot(SnapshotResult result) {\n\t\tif (diffText == null || diffText.isDisposed() || !result.created()) {\n\t\t\treturn;\n\t\t}\n\t\ttry {\n\t\t\tRepository repo = OpSnapshotter.getInstance().getRepository();\n\t\t\tString text = OpDiffReader.diff(repo, result.parent(), result.commit());\n\t\t\tstatusLabel.setText(result.chain() + \" / \" + result.ref());\n\t\t\tdiffText.setText(text.isEmpty() ? \"(keine Aenderungen)\" : text);\n\t\t} catch (IOException e) {\n\t\t\tdiffText.setText(\"Fehler beim Lesen des Diffs: \" + e.getMessage());\n\t\t}\n\t}\n\n\t@Override\n\tpublic void setFocus() {\n\t\tdiffText.setFocus();\n\t}\n\n\t@Override\n\tpublic void dispose() {\n\t\tDiffRefreshBus.removeListener(listener);\n\t\tsuper.dispose();\n\t}\n}\n"
  reason: "Neue Klassen im Package view.diff anlegen: Snapshot-Ergebnis, Event-Bus, JGit-basierter Snapshotter (analog op-snapshot.sh), Diff-Reader und das Diff-Panel (ViewPart)."
```
/answer b655-24 allow
Control Request:
```yaml
id: b655-25
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
  reason: Pruefen ob die neu erstellte Datei syntaktisch korrekt ist
```
/answer b655-25 allow
Control Request:
```yaml
id: b655-26
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    id: PromptHandler.executeInner
    source: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tres.prompt = req.getPrompt();\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\ttriggerSnapshotIfNeeded(req);\n\t\treturn res;\n\t}\n\n\t/**\n\t * Loest nach Ausfuehrung einer /answer- oder /tool-Command einen Git-Snapshot aus\n\t * (JGit, analog scripts/op-snapshot.sh) und aktualisiert bei Erfolg das Diff-Panel.\n\t */\n\tprivate void triggerSnapshotIfNeeded(IModelRequest req) {\n\t\tPrompt prompt = req.getPrompt();\n\t\tif (prompt == null || prompt.arg == null) {\n\t\t\treturn;\n\t\t}\n\t\tCommand cmd = prompt.arg.command;\n\t\tif (!(cmd instanceof AnswerCommand) && !(cmd instanceof ToolCommand)) {\n\t\t\treturn;\n\t\t}\n\t\tString triggerLabel = cmd.prefix();\n\t\tJob.create(\"Workbench-Snapshot\", mon -> {\n\t\t\ttry {\n\t\t\t\tSnapshotResult result = OpSnapshotter.getInstance().snapshot(triggerLabel);\n\t\t\t\tif (result.created()) {\n\t\t\t\t\tDiffRefreshBus.fire(result);\n\t\t\t\t}\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
  reason: Snapshot-Trigger nach /answer- oder /tool-Ausfuehrung einbauen und Diff-Panel-Refresh anstossen.
```
/answer b655-26 allow
Control Request:
```yaml
id: b655-27
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/plugin.xml
    id: ODTrYv|DEw9ri.dBBPxh|C1uTgW.OlCKvM|Eut3ja.8B313S|mx8jZb.gmQdhN|v2lP3V
    position: after
    source: "\n\t\t<view\n\t\t\t\tallowMultiple=\"false\"\n\t\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\t\tclass=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\t\ticon=\"icons/sample.png\"\n\t\t\t\tid=\"xy.ai.workbench.view.diff.DiffPanel\"\n\t\t\t\tname=\"AI Diff\">\n\t\t</view>"
  reason: DiffPanel als eigenstaendigen View unter der XY.AI.Workbench-Kategorie registrieren.
```
/answer b655-27 allow
Control Request:
```yaml
id: b655-28
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF
    id: file
    old_text: |2-
       org.eclipse.ui.navigator,
       org.eclipse.search
      Bundle-RequiredExecutionEnvironment
    new_text: |2-
       org.eclipse.ui.navigator,
       org.eclipse.search,
       org.eclipse.jgit
      Bundle-RequiredExecutionEnvironment
    exact: true
  reason: JGit-Bundle als Abhaengigkeit fuer die neue Snapshot/Diff-Implementierung hinzufuegen.
```
/answer b655-28 allow
Control Request:
```yaml
id: b655-29
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffPanel.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/DiffRefreshBus.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpDiffReader.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/SnapshotResult.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  reason: Syntaktische Korrektheit aller neu erstellten bzw. geaenderten Java-Dateien pruefen.
```
/answer b655-29 allow
Result Stats: id=47287889-6c06-4fc7-b6cc-c51376dc7069, total: 783527, in: 209, out: 51969, reason: 901, read: 10599247, write: 731349
ReasoningToken: 329
ReasoningToken: 24
Text:
 Nun der Trigger in `PromptHandler.executeInner`, die Registrierung des Views in `plugin.xml` und die JGit-Dependency im `MANIFEST.MF`.
ReasoningToken: 6
ReasoningToken: 34
ReasoningToken: 39
ReasoningToken: 382
Text:
 Jetzt View-Registrierung in `plugin.xml` und JGit-Dependency im `MANIFEST.MF`.
ReasoningToken: 87
Alle Dateien sind syntaktisch korrekt. Umgesetzt:

**Neues Package `xy.ai.workbench.view.diff`:**
- `OpSnapshotter` — JGit-Port von `op-snapshot.sh` (Chain-Marker `refs/llm-ops/_base`, Chain-Namespace `refs/llm-ops/chain-<id>/op-N`, Delta-Prüfung via `git add -A` + `DirCache.writeTree` gegen den Parent-Tree, `CommitBuilder`/`RefUpdate` statt HEAD-Bewegung)
- `SnapshotResult` — Record mit `created`, `chain`, `ref`, `commit`, `parent`
- `DiffRefreshBus` — statischer Listener-Bus, feuert asynchron auf dem UI-Thread
- `OpDiffReader` — aggregierter Unified Diff via `DiffFormatter`
- `DiffPanel` — Singleton-`ViewPart`, zeigt beim Öffnen sowie nach jedem Snapshot-Event automatisch den jüngsten Diff (`findLatest()`)

**`PromptHandler.executeInner`**: ruft nach der Antwortverarbeitung `triggerSnapshotIfNeeded(req)` auf; bei `AnswerCommand`/`ToolCommand` wird asynchron `OpSnapshotter.snapshot(...)` ausgeführt und bei tatsächlichem Delta `DiffRefreshBus.fire(...)` angestoßen.

**`plugin.xml`**: View `xy.ai.workbench.view.diff.DiffPanel` ("AI Diff") unter Kategorie `XY.AI.Workbench` registriert.

**`MANIFEST.MF`**: `org.eclipse.jgit` als Require-Bundle ergänzt.

Die Imports in `PromptHandler.java` musst du wie angekündigt selbst ergänzen (`xy.ai.workbench.commands.{AnswerCommand,ToolCommand,Command}`, `xy.ai.workbench.view.diff.{OpSnapshotter,SnapshotResult,DiffRefreshBus}`, `org.eclipse.core.runtime.{Status,jobs.Job}`).
