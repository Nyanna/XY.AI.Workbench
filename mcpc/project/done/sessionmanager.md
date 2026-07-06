# CLI-Session Manager

Implementiere einen CLI-Session-Manager. Die Zielsprache von Code-Kommentaren und Texten ist englisch.  

## Kontext

Ein separater CLI-Session-Manager soll parallel eine Vielzahl von CLI-Prozessen verwalten, bei Bedarf Starten oder Beenden.

## Referenzen

* Python Projekt-Root: `/home/user/xyan/xy.ai.workbench/mcpc`

## Agenten Tool

* Das Starten der CLI wird vom Manager übernommen.
* Das Agenten-Tool stellt beim Manager eine Anforderung und erhält ein CLI-Session-Objekt.
* Die Anforderung stellt er auf Basis der Promptinformation wie Modell, Tools und Effort oder einer per Resume gegebenen CLI-Session-ID
* Wenn Resume nicht angegeben ist wird stets eine neue CLI-Session erzeugt.

## Manager

Der CLI-Session-Manager ist zentrale Verwaltung der CLI-Sessions. Er hält einen Index, erzeugt Sessions oder fährt diese Herunter.

* Der Manager startet oder beendet selbst keine CLI-Session, sondern verwaltet die CLI-Sessions.
* Bei einer Anforderung prüft der Manager die Gültigkeit aller aktiven CLI-Session und fordert abgelaufene CLI-Sessions auf die CLI zu terminieren.
* Abgelaufene terminierte Sessions werden entfernt. Beendete gültige Session können noch fortgesetzt werden.
* Eine CLI-Session kann anhand der CLI-Session-UUID aufgelöst werden. 

### Bestehende Session-Anforderung

Wird eine bestehende CLI-Session mittels per "resume" übergebener UUID angefordert so Prüft der Manager auf die bestehende Session und deren Gültigkeit. Eine CLI-Session wird 1 Stunde nach der letzten Verwendung ungültig.

### Neue Session-Anforderung

Wird eine neue Session angefordert so erstellt der Manager ein neues CLI-Session-Objekt, trägt dieses ein und gibt es zurück.


### CLI-Session-Objekt

Die CLI-Session sichert die optimale Nutzung des CLI-Session-Caches.

* Die Session stellt ein Container-Objekt dar und enthält Referenzen auf den CLI-Prozess, STDIN, STDOUT, STDERR und den CLI-Manager.
* Die CLI-Session-UUID wird der CLI mittels `--session-id` übergeben.
* Ein eingehender Prompt startet eine noch nicht gestartete oder gültige Session.
* Die Session speichert den Startzeitpunkt, den Zeitstempel der letzten gesendeten und empfangenen Nachricht.
* Der Zeitstempel der letzten gesendeten Nachricht bestimmt die TTL/Gültigkeit
* Eine Session verliert nach 1 Stunde ohne Nutzung ihr Gültigkeit
* Ein Promptversuch in einer ungültigen Session, beendet eine noch laufende CLI und meldet dann einen Fehler.
* Die Session startet und beendet die CLI und kann `--resume <SessionID>` verwenden um eine bestehende CLI-Session wieder aufnehmen.
* Ein Prompt, der an die CLI gesendet wird, setzt die verbleibende Lebenszeit wieder auf 1 Stunde zurück.
* Die Session ist für die Replikation des Ausgabestroms verantwortlich.
* Das Session-Objekt delegiert die Aufrufe des Agenten-Tools auf STDIN und STDOUT.

#### Replikation

Das  CLI-Session-Objekt speichert eine Kopie der Ein- und Ausgabeströme zeilenweise in einer JSON-Datei ab.

* Die JSON-Datei hat den Namen der CLI-Session-ID und wird in einem gemeinsamen Logverzeichnis abgelegt.

#### CLI-Prozess

Nachfolgend die Prozessinstanzierung in Java.

```java

	List<String> cmd = parameters.buildBaseCommand();
	if (uuid == null) { // initial start
		uuid = UUID.randomUUID().toString();
		cmd.add("--session-id");
	} else //resume existing session
		cmd.add("--resume");
	cmd.add(uuid);
		
		
	public List<String> buildBaseCommand() {
		List<String> cmd = new ArrayList<>();
		cmd.add("claude");
		cmd.add("--system-prompt");
		cmd.add(systemPrompt);
		cmd.add("--tools");
		cmd.add("\"\""); // no builtin tools
		cmd.add("--settings");
		cmd.add("{\"hooks\":{\"PreToolUse\":[{\"type\":\"http\",\"url\":\"http://localhost:9093/hooks/tool\",\"headers\":{\"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"},\"allowedEnvVars\":[\"MCPC_SESSION_ID\"],\"timeout\":86400}]}}"); // self reference MCPC
		cmd.add("--mcp-config");
		cmd.add("{\"mcpServers\": {\"mcp\": {\"type\": \"http\",\"url\": \"http://localhost:9093/mcp\", \"timeout\": 86400000, \"headers\": {\"X-MCPC-SESSION-ID\": \"$MCPC_SESSION_ID\"}}}}"); // self reference MCPC
		cmd.add("--verbose");
		cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		cmd.add("--replay-user-messages");
		cmd.add("--model");
		cmd.add(model);
		if (Reasoning.Disabled != reasoning) {
			cmd.add("--effort");
			cmd.add(reasoning.name().toLowerCase());
		}
		cmd.add("--dangerously-skip-permissions"); // as long there is no permission prompt handling implemented
		return cmd;
	}

	public void buildEvironment(ProcessBuilder pb) {
		pb.environment().put("CLAUDE_CONFIG_DIR", System.getProperty("user.home") + "/.claude-" + cliProfile);
		pb.environment().put("CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_SPELLCHECK", "true");
		pb.environment().put("CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING", "1");
		pb.environment().put("MAX_THINKING_TOKENS", "0");
		pb.environment().put("CLAUDE_CODE_DISABLE_AGENT_VIEW", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_BACKGROUND_TASKS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_BUNDLED_SKILLS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_CLAUDE_MDS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_CRON", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_POLICY_SKILLS", "1");
		pb.environment().put("CLAUDE_CODE_DISABLE_WORKFLOWS", "1");
		pb.environment().put("CLAUDE_CODE_ENABLE_AWAY_SUMMARY", "0");
		pb.environment().put("CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH", "1");
		pb.environment().put("CLAUDE_CODE_FORK_SUBAGENT", "0");
		pb.environment().put("CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY", "1"); // number of parralel read tools
		pb.environment().put("ENABLE_TOOL_SEARCH", "false");
		pb.environment().put("CLAUDE_CODE_DISABLE_ADVISOR_TOOL", "1");
	}
```