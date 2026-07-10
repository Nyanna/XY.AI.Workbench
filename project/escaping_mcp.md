Ich habe Ausgaben wie die folgenden gesehen.
Durch eine zentrale Utility-Klasse muss sichergestellt seine dass alle Ausgangs und Eingangströme, sowie JSON Behandlung und Building. Korrekt escapen und wieder zurück umwandeln können.
Speziell beim Umverpacken von JSON in oder von Containern muss sichergestellt sein, dass das korrekte Escaping angewendet wird. Das betrifft den HTTP-Server, die Transports, den Web-Socket Adapter und die Control-Endpunkte.

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