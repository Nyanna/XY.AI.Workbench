# Escape and Parsing Problems

Ich werde im MCP Server getrennt das Escaping kontrollieren lassen. Dein Fokus liegt in der Workbench.

## 1. Im Eclipse Control werden lange Web-Recherchen nicht angezeigt.

Möglicherweise hat das mit Escaping zu tun oder mit Limits des Transports oder des Web Sockets.
In der Schleife für die Behandlung von Kontrollnachrichten muss entsprechendes Logging und Fehlerbehandlung eingebaut werden. Speziell auch Parsing Fehler dürfen nicht verschluckt werden.

## 2. Diese Exception habe ich im Log gefunden

Der Connector hat folgende Exception verursacht.

Exception:
```
	java.lang.NullPointerException: Cannot invoke "java.io.BufferedReader.readLine()" because "reader" is null
	at xy.ai.workbench.connectors.claudecode.ClaudeCodeSession.readLine(ClaudeCodeSession.java:168)
	at xy.ai.workbench.connectors.claudecode.ClaudeCodeSession.readLine(ClaudeCodeSession.java:145)
	at xy.ai.workbench.connectors.claudecode.ClaudeCodeConnector.readUntilResult(ClaudeCodeConnector.java:159)
	at xy.ai.workbench.connectors.claudecode.ClaudeCodeConnector.executeRequest(ClaudeCodeConnector.java:128)
	at xy.ai.workbench.connectors.AdaptingConnector.executeRequest(AdaptingConnector.java:167)
	at xy.ai.workbench.AISessionManager.executeInner(AISessionManager.java:410)
	at xy.ai.workbench.AISessionManager.lambda$11(AISessionManager.java:295)
	at org.eclipse.core.runtime.jobs.Job$1.run(Job.java:166)
	at org.eclipse.core.internal.jobs.Worker.run(Worker.java:63)
```

Die folgenden Klassen müssen mit erweiterten Asserts und Exceptions  ausgestattet werde, um Fehler im Streamhandling frühzeitig zu erkennen.

* Controller: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java`
* Session: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeSession.java`
* Parameter: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/SessionParameters.java`

## 3. Session ID ist null

Es wurde die Datei `/home/user/xyan/xy.ai.workbench/.claude/logs/null.json` erstellt. Auch hier fehlen Assertions und Exception. Der Code muss abbrechen bevor er einen Dateinamen mit Null erzeugt.
Ich habe hier bereits etwas vorgearbeitet.

## 4. Allow ezeugt session

Kontrollkommandos wie "/exit", "/allow", "/deny" mit ausnahme "/resume" dürfen keine neue Session erzeugen. Wenn die Session nicht existiert muss es einen Fehler geben. "/allow" und "/deny" laufen über die Control und brauchen keine aktive Session.

## 5. Escaping

Ich habe Ausgaben wie die folgenden gesehen.
Durch eine zentrale Utility-Klasse muss sichergestellt seine dass alle Ausgangs und eingangströmge, sowie JSON Behandlung und Building. Korrekt escapen und wieder zurück umwandeln können.
Speziell beim Umverpacken von JSON in oder von Containern muss sichergestellt sein, dass das korrekte Escaping angewendet wird,

Log1
```log
     "structuredContent" : {
       "error" : "Start marker not found in file: '    // Inline markup regions: fenced code, inline code, URLs, file paths, @mentions.\\n    private static final Pattern MARKUP_RE = Pattern.compile(\\n            \"```[\\\\\\\\\\\\\\\\s\\\\\\\\\\\\\\\\S]*?```|`[^`]+`|https?://\\\\\\\\\\\\\\\\S+|/\\\\\\\\\\\\\\\\S+|@\\\\\\\\\\\\\\\\S+\");'"
     },
```

Log 2
```log
**Was war das Problem:**
- Die Regex hatte zu viele Backslashes: `[\\\\s\\\\S]`
- Dies wurde zu `[\\s\\S]` kompiliert statt zu `[\s\S]`

**Die Korrektur:**
Das Regex-Pattern wurde von:
"```[\\\\s\\\\S]*?```|`[^`]+`|https?://\\\\S+|/\\\\S+|@\\\\S+"

zu:
"```[\\s\\S]*?```|`[^`]+`|https?://\\S+|/\\S+|@\\S+"
```

Log 3
```log
 Thinking:
 Jetzt sehe ich das Problem! Schauen wir uns die Regex auf Linie 47 an:
 private static final Pattern MARKUP_RE = Pattern.compile(
         "```[\\\\s\\\\S]*?```|`[^`]+`|https?://\\\\S+|/\\\\S+|@\\\\S+");
 Das Problem ist hier: ````[\\\\s\\\\S]*?```
 In Java-Strings werden Backslashes verdoppelt. Also `\\\\s` ist tatsächlich `\\s` in der Regex, was für "Whitespace" steht. Und `\\\\S` ist `\\S` in der Regex, was für "Non-Whitespace" steht.
 Aber das ist korrekt für die Regex-Syntax!
 Moment, lass mich das nochmal überprüfen. In Java:
 - Ein einzelner Backslash muss als `\\` geschrieben werden
 - Also `\\s` in Java ist `\s` in der Regex
 - Und `\\\\s` in Java ist `\\s` in der Regex (das ist ein Backslash gefolgt von "s")
 So in der Zeile:
 "```[\\\\s\\\\S]*?```|..."
 Das wird zu:
 ```[\\s\\S]*?```|...
 in der Regex. Das ist falsch! Es sollte sein:
 ```[\s\S]*?```|...
 Der Regex-Pattern sollte sein:
 - ``` (drei Backticks)
 - [\s\S]*? (jedes Zeichen, einschließlich Newlines, nicht-greedy)
 - ``` (drei Backticks)
 Das bedeutet, wir brauchen in Java:
 "```[\\s\\S]*?```|..."
 Statt:
 "```[\\\\s\\\\S]*?```|..."
 Es gibt zu viele Backslashes im aktuellen Code. Die Regex versucht zu matchen: ``` gefolgt von [\\s\\S] (ein Backslash gefolgt von s oder S), aber das ist nicht das, was wir wollen. Das ist der Grund, warum Fenced Code Blocks nicht korrekt erkannt werden.
```