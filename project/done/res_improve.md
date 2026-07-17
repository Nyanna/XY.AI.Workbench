Du hast häufig das Problem das du bei Verwendung des Read Tools eine Datei mehrfach liest. Scheinbar geht im Kontext die Attention verloren, dass du diese Datei bereit gelesen hast. Wie lässt sich das verbessern?
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=2d8eb4fa-0643-45dd-b24c-5a18e4899307 | model=claude-sonnet-5 | plugins=
Das ist ein bekanntes Problem bei längeren Agent-Sessions: Je mehr Kontext sich ansammelt, desto eher „vergisst" das Modell, dass eine Datei bereits gelesen wurde, und liest sie redundant erneut – das kostet Tokens, Zeit und manchmal auch Kohärenz. Hier ein paar konkrete Ansätze, um das zu verbessern:

## 1. Explizites Tracking im Kontext (Prompt-Ebene)
- **Datei-Register führen**: Am Anfang der Antwort/Session eine kurze Liste pflegen (z.B. als Kommentar oder Notiz), welche Dateien mit welchem Pfad und ggf. Zeilenbereich bereits gelesen wurden. Das kann z.B. in einer Art "Scratchpad" passieren, der bei jedem Tool-Call kurz aktualisiert wird.
- **Vor jedem Read-Aufruf kurz prüfen**: "Habe ich diese Datei (diesen Pfad) in diesem Kontext schon gelesen?" – als expliziten Zwischenschritt vor dem Tool-Call.

## 2. Technische/strukturelle Lösungen
- **Caching-Layer außerhalb des Modells**: Ein Wrapper um das Read-Tool, der bereits gelesene Dateien (Pfad + Hash/Timestamp) protokolliert und bei erneutem Aufruf entweder abbricht oder einen Hinweis zurückgibt ("Diese Datei wurde bereits gelesen, Inhalt siehe oben").
- **Deduplizierung auf Tool-Ebene**: Wenn du das Tool selbst kontrollierst (z.B. bei einem MCP-Server), kannst du dort eine Read-Historie pro Session speichern und bei identischem Request den Cache statt eines neuen Reads zurückgeben.

## 3. Kontext-Management
- **Wichtige Infos komprimieren statt im Rohformat behalten**: Nach dem Lesen einer großen Datei eine kompakte Zusammenfassung ins "Arbeitsgedächtnis" schreiben statt den vollen Dateiinhalt im Kontext zu belassen. Das reduziert nicht nur die Chance auf Vergessen, sondern auch den Tokenverbrauch.
- **Kontext-Fenster bewusst strukturieren**: Gelesene Dateien mit klaren Markern versehen (z.B. `<!-- FILE READ: path/to/file.py -->`), damit sie beim Scannen des Kontexts leichter wiedergefunden werden.

## 4. Workflow-Anpassung
- **Batch-Reads am Anfang**: Wenn absehbar ist, welche Dateien gebraucht werden, alle relevanten Dateien in einem Rutsch zu Beginn lesen, statt verteilt über die Session – reduziert die Wahrscheinlichkeit, dass zwischenzeitlich "vergessen" wird.
- **Kürzere, fokussierte Sessions**: Bei sehr langen Interaktionen den Kontext gelegentlich zusammenfassen/resetten und nur die relevanten Extrakte weiterführen.

Falls du das in einem konkreten Setup (z.B. Claude Code, eigener Agent-Loop, MCP-Server) umsetzen willst, sag mir gerne, welches Framework du nutzt – dann kann ich dir spezifischer sagen, wo man Caching oder Tracking einbauen würde.
Ich kontrolliere den MCP. Ich habe schon zusätzlich Meta-Information in die Tool-Response eingefügt. Vielleicht aber zu wenige oder nicht die richtigen. Caching bringt keinen nutzen und der Tool-Aufruf zu verweigern mit einem Hinweis führt lediglich dazu das der Agent eine anderes Toll wie Bash mit `cat` verwendet. Er ignoriert den Hinweis.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=2d8eb4fa-0643-45dd-b24c-5a18e4899307 | model=claude-sonnet-5 | plugins=
ReasoningToken: 1900
Interessant – das deutet darauf hin, dass das eigentliche Problem tiefer liegt als "Meta-Info fehlt". Ein paar Gedanken, warum deine bisherigen Ansätze nicht greifen und was stattdessen helfen könnte:

## Warum Refusal + Hinweis nicht funktioniert
Ein Refusal wird vom Modell als **Hindernis** interpretiert, nicht als Information. Agenten sind darauf trainiert, Ziele zu erreichen – wenn ein Tool blockt, wird ein anderer Weg gesucht (`cat`, `grep`, etc.). Das ist eigentlich rationales Verhalten, kein Ignorieren. Die Lösung liegt also nicht in Restriktion, sondern in **Nützlichkeit**: Das Tool muss immer einen Mehrwert liefern, der einen Re-Read unattraktiv macht – nicht verbietet.

## Der wahrscheinlichere Grund: Content ist wirklich weg aus dem Kontext
Wenn Meta-Informationen ("bereits gelesen am Turn X") nicht wirken, ist eine sehr plausible Erklärung: **Der eigentliche Dateiinhalt ist gar nicht mehr im aktiven Kontextfenster**, z.B. durch:
- Context-Compaction/Summarization durch den Client
- Truncation bei langen Sessions
- Der Agent "erinnert" sich an die Tatsache des Lesens, aber der Inhalt selbst ist nicht mehr abrufbar

In diesem Fall ist ein erneutes Lesen aus Sicht des Modells **korrekt**, weil es den Inhalt schlicht nicht mehr "sieht". Ein Hinweis wie "du hast das schon gelesen" hilft dann nichts – das Modell weiß das ja, braucht aber trotzdem den Inhalt.

**Test dafür:** Prüfe, ob der Redundant-Read typischerweise nach vielen dazwischenliegenden Tool-Calls/Turns auftritt (→ Context-Loss-Hypothese) oder auch bei kurzen Abständen (→ echtes Attention-Problem).

## Konkrete Ansätze

**1. Persistente, kompakte Digests statt Vollinhalt bei Re-Reads**
Statt zu blockieren: Beim ersten Read vollen Inhalt liefern. Bei jedem weiteren Read-Aufruf auf denselben Pfad automatisch nur einen **Diff zum letzten Stand** oder – falls unverändert – eine sehr knappe Bestätigung + die wichtigsten extrahierten Fakten (Signaturen, Struktur, o.ä.) zurückgeben, NIE eine leere Response. So bekommt der Agent trotzdem etwas Nützliches, ohne dass der volle Content erneut den Kontext aufbläht.

**2. Format wie einen System-Reminder, nicht wie Tool-Output**
Du siehst hier im Transcript, wie `<system-reminder>` Tags funktionieren – Claude ist darauf trainiert, solchen Markierungen besondere Priorität zu geben, stärker als normalem Tool-Content. Wenn deine Meta-Info aktuell als normaler Text im Tool-Result steckt, wird sie wie Daten behandelt und verliert an Gewicht. Ein Format wie:
```
<file_registry>
Bereits gelesen in dieser Session:
- /path/foo.py (zuletzt: Turn 4, unverändert)
- /path/bar.py (zuletzt: Turn 7, geändert seit Read - Diff siehe unten)
</file_registry>
```
das bei **jedem** Tool-Call (nicht nur beim betroffenen) mitgeschickt wird, wirkt als wiederkehrender, salienter Anker im Kontext – ähnlich wie ein System-Reminder, der ständig präsent ist statt einmalig erwähnt zu werden.

**3. Bash/cat-Bypass technisch unterbinden**
Falls du Kontrolle über die Sandbox/Tool-Verfügbarkeit hast: Wenn dein Read-Tool aktiv ist, Bash-Zugriff auf Dateisystem-Befehle (`cat`, `head`, `sed -n`) einschränken oder umleiten. Sonst bleibt jede Kontext-seitige Lösung wirkungslos, weil der Agent einen Ausweg hat.

**4. Reframing statt Restriktion**
Formuliere die Meta-Info nicht als Warnung ("du hast das schon gelesen"), sondern biete aktiv den Mehrwert an, den ein Re-Read nicht hätte: z.B. "Datei unverändert seit Turn 4 – hier die extrahierten Kernpunkte: [...]". Das macert das Re-Read redundant, ohne dass der Agent das Gefühl hat, blockiert zu werden.

Willst du mir zeigen, wie deine aktuelle Meta-Info-Struktur aussieht? Dann kann ich konkreter sagen, was fehlt oder falsch positioniert ist.

Der `<file_registry>` Ansatz klingt interessant. Recherchiere mal, welche erfolgversprechenden Ansätze es gibt.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=2d8eb4fa-0643-45dd-b24c-5a18e4899307 | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 4ceb969b-dd39-4d50-947a-c89e60a3cd18
toolName: web-search-exa
arguments:
  query: LLM agent avoid redundant file reads context tracking tool result deduplication
```
/answer 4ceb969b-dd39-4d50-947a-c89e60a3cd18 allow
ReasoningToken: 81
Control Request:
```yaml
id: b015c9af-cd42-4363-b098-92c9755856db
result:
  structuredContent:
    content: |-
      Title: TriSeek — Memo & Search Reuse
      URL: https://sagart-cactus.github.io/TriSeek/memo.html
      Published: N/A
      Author: N/A
      Highlights:
      Memo tracks which files your agent has read in the current session and tells it when they have changed on disk. Search reuse does the same for repeated MCP search outputs by telling the model to reuse earlier context when relevant files are unchanged.
      ...
      TriSeek ships two complementary session systems. Memo handles file reads; search reuse handles duplicate grep-style MCP queries by emitting a compact reuse envelope. Both are invalidated when files change.
      ...
      Memo observes every`Read` and`Edit` tool call in a session. When the agent reads the same file again, Memo checks the disk hash. If unchanged, the agent skips the re-read and trusts conversation history.
      ...
      Repeated identical`search_content`,`find_files`, and`search_path_and_content` queries return a compact`fresh_duplicate` response when the daemon proves relevant files are unchanged. The model reuses the earlier results already in context instead of receiving duplicate output.
      ...
      Session-scoped metadata LRU, 256 entries. Freshness is checked through daemon generation, context epoch, and the change journal.
      ...
      TriSeek observes Codex Bash reads and MCP file-read tools passively via`PreToolUse` hooks. Agents still call`memo_check` before rereads that the installed Codex version does not expose through hooks. Session file map is also invalidated on`PreCompact` so context boundaries are handled correctly.
      ...
      If`recommendation` is`skip_reread`, trust conversation history. Otherwise, read the file normally.
      ...
      Single-file freshness check. Returns a recommendation for whether to re-read.
      ...
      | Recommendation | Meaning |
      | --- | --- |
      | skip_reread | File unchanged. Trust conversation history. |
      | reread_with_diff | File changed slightly (<10% size delta). Re-read expecting a small diff. |
      | reread | File changed significantly or was never read. Read normally. |
      ...
      Every search tool response includes a`cache` field. For repeated indexed queries,`cache: hit` means TriSeek returned a compact instruction to reuse the prior result already in context.
      ...
      freshness · Daemon generation, context epoch, and change journal
      ...
      invalidation · Fresh search runs when relevant files changed or context was compacted
      ...
      Every search envelope includes a`cache` field:
      ...
      | Value | Meaning |
      | --- | --- |
      | hit | Repeated indexed search is fresh. Reuse the previous result from context. |
      | miss | Search executed and metadata was recorded for future duplicate detection. |
      | bypass | Fallback path used. Result is not eligible for reuse metadata. |
      ...
      Applies to`find_files`,`search_content`, and`search_path_and_content`.
      ...
      ```
      {
        "strategy": "triseek_indexed",
        "cache": "hit",
        "prior_search_id": "search-0001",
        "reuse_status": "fresh_duplicate",
        "reuse_reason": "unchanged",
        "generation": 42,
        "context_epoch": 0,
        "results": [],
        "results_omitted": true,
        "truncated": false
      }
      ...
      scope · Per MCP process and session hint. Two unrelated agent sessions do not share reuse state.
      ...
      max size · 256 metadata entries, not full result payloads
      ...
      stale risk · Freshness is checked before reuse; changed files force a new search.
      ...
      key normalisation · Exact match only. Changing`limit` or any query field is treated as a new search.
      ...
      Memo answers "did I read this file already in this MCP session?" Task memory answers "did the prior session in another harness already establish what this file does?"
      ...
      When`session_resume` hydrates a snapshot, the daemon primes memo state with file fingerprints from the snapshot working set. Unchanged files can return`skip_reread` even though the new harness session never opened them directly.
      ...
      The same idea applies to search reuse: searches captured in the snapshot are warmed into the search memo cache, so repeated searches can return cache-hit reuse envelopes instead of reprinting old results.
      ...
      Both layers avoid redundant work. Memo prevents redundant file reads; task memory prevents redundant exploration.

      ---

      Title: feat: add file read deduplication cache to prevent repeated reads · Pull Request #9836 · cline/cline
      URL: https://github.com/cline/cline/pull/9836
      Published: 2026-03-16T21:07:01.000Z
      Author: mathis1337
      Highlights:
      ## feat: add file read deduplication cache to prevent repeated reads
      ...
      **Issue:** N/A — This addresses a common behavior observed across models where they repeatedly read the same unchanged files, wasting API tokens and context window space.
      ...
      This PR adds a per-task file read deduplication cache that prevents models from endlessly reading the same unchanged files, reducing wasted API tokens and context window consumption.
      ...
      Models frequently re-read files they've already read in the same task — sometimes 5+ times for the same file that hasn't changed. Each redundant read wastes API tokens (both input from the tool call and output from the file content) and fills the context window with duplicate information.
      ...
      - **TaskState.ts**: Added `fileReadCache: Map<string, { content: string; readCount: number }>` to track file reads per task
      - **ReadFileToolHandler.ts**: Checks cache before performing disk I/O — returns cached content on repeat reads. After 3+ reads of the same file, returns a stronger `[DUPLICATE READ DETECTED]` warning instructing the model to use information it already has
      - **WriteToFileToolHandler.ts**: Invalidates cache entry when a file is written, so subsequent reads get fresh content
      - **ApplyPatchHandler.ts**: Invalidates cache entries for all changed/deleted/moved files after patch application
      ...
      1. First read of a file → normal disk read, result cached
      2. Second read of same file → returns cached content with `[File already read]` prefix
      3. Third+ read → returns cached content with `[DUPLICATE READ DETECTED]` warning
      4. Any write/patch to a file → cache entry invalidated, next read is fresh from disk
      ...
      - The cache operates at the **tool handler level** (`ReadFileToolHandler.execute()`), which is called identically for every provider (Anthropic, OpenRouter, Gemini, Ollama, etc.). No provider-specific code is involved.
      - **Cached content is byte-identical to a fresh read.** If a file hasn't changed since the last read, `extractFileContent()` would return the exact same content. The cache simply avoids redundant disk I/O — the model sees the same content either way.
      ...
      - **Writes properly invalidate the cache.** Any `write_to_file`, `replace_in_file`, or `apply_patch` operation invalidates the affected file's cache entry, so the model always gets fresh content after edits. There's no risk of serving stale data.
      ...
      - **The cache is per-task scoped** (lives on `TaskState`), so it auto-cleans up when the task ends with no cross-task contamination.
      ...
      - Verified the cache correctly returns cached content on second reads of the same file
      - Verified that writing to a file (via `write_to_file`) properly invalidates the cache, causing the next read to fetch fresh content from disk
      - Verified that applying patches (via `apply_patch`) properly invalidates cache entries for all changed, deleted, and moved files
      - Verified the escalating warning at 3+ reads returns the stronger `[DUPLICATE READ DETECTED]` message
      - Verified cache keys are case-insensitive (Windows path compatibility)
      - Verified no interference with existing file read error handling (try/catch around `extractFileContent` is preserved — errors bypass the cache)
      ...
      - The cache uses case-insensitive absolute paths as keys (`absolutePath.toLowerCase()`) for Windows compatibility
      - Only 4 files changed with 41 lines of additions — minimal footprint, easy to review
      - The deduplication is transparent to the model — it still receives file content, just served from cache when unchanged
      ...
      > Greptile Summary
      >
      > This PR adds a per-task file-read deduplication cache to `ReadFileToolHandler`. The cache stores file metadata (read count, mtime, and an optional image block for multimodal models) keyed by the lowercased absolute path. On repeated reads, the handler checks mtime to detect external modifications and, if the file is unchanged, returns the content with an escalating warning prefix (`[File already read]` on the 2nd read, `[DUPLICATE READ]` on the 3rd+). Cache entries are invalidated by `write_to_file`/`replace_in_file` (`WriteToFileToolHandler`), `apply_patch` (`ApplyPatchHandler`), and the entire cache is cleared after any non-rejected `execute_command` (`ExecuteCommandToolHandler`).
      >
      > **Key observations:**
      >
      > - The cache does **not** avoid disk I/O on hits — `extractFileContent` is called on every read regardless. The real benefit is the warning messages that nudge the model away from redundant reads.
      ...
      > | src/core/task/tools/handlers/ReadFileToolHandler.ts | Core of the deduplication feature: adds mtime-based cache lookup before every read, re-reads from disk on cache hits (no disk I/O is saved), and returns escalating warning prefixes on repeated reads. Missing `consecutiveMistakeCount` reset on successful cached reads (a divergence from the fresh-read path). Uses repeated dynamic imports of `node:fs/promises` instead of a static import. |
      ...
      > | src/core/task/tools/handlers/WriteToFileToolHandler.ts | Correctly invalidates the cache entry for the written file using `absolutePath.toLowerCase()`, consistent with the cache key used in `ReadFileToolHandler`. |
      ...
      'neutral'}}%%
      > flowchart TD
      ...
      > F --> K[Reset consecutiveMistakeCount = 0]
      > K --> L[Cache metadata: readCount=1,
      ...
      > @mathis1337 I also met the same issue(repeated file read). So I suggest removing duplicate file read using `FileContextTracker`'s `user_edit_date` and `cline_edit_date`. You can check my idea here(https://github.com/cline/cline/issues/9778).
      ...
      > > @mathis1337 I also met the same issue(repeated file read). So I suggest removing duplicate file read using `FileContextTracker`'s `user_edit_date` and `cline_edit_date`. You can check my idea here(https://github.com/cline/cline/issues/9778).
      >
      > Yeah I've already tested this running and been using it for over a week and it's working amazing. Hopefully cline devs can review soon.
      ...
      > > > I'm interested to understand if this actually improves cline benchmarks or not. One could make the argument that while rereading the file adds to the context window, it also refreshes the file in the "mind" of the llm (the more recent the file is in the context window, the better recall the llm has)
      > >
      > > I left a lengthy comment. Above covering this question I think. I am curious to know your thoughts. I haven't exactly tracked it, but I can tell from just usage that anthropic and claude code are definitely do a lot less reads and re-reads and noticing my final answers in planning coming back faster. Is there an easy way to bench mark that already in cline?
      >
      > Refreshing the file is fine, we should just do it with a cached version if possible and/or remind the LLM it already has it. Not doing so lets the models be "lazy" and it consumes more and burdens the users with higher consumption.
      ...
      > @maxpaulus43 - I resolved the reviews that were fair, and I left a comment with the conversation open. If you aren't satisfied let me know and we can see what can be done, but I've been using the change and I can speak for the anthropic models and gemini that i've seen consumption from reads go down. After about a week of use I've yet to see a file be read twice in a row. This was the main issue. Its not about re-reading a file. Its that sometimes the model will read the same file back to back and upwards of 20 times in some cases. Thats the model hallucinated, going crazy, whatever you want to call it. I think cline can step in and help the model by being like "hey chill, you've read this" ya know.
      >
      > This is what I mean. Cline will call the model, and it will just read the file, back to back to back to back, over and over in same context:
      > [Image: image | https://github.com/user-attachments/assets/ade1c938-b826-41f3-b946-c2e4584b9a04]
      >
      > ^^this is just a mock up of what happens. I cleared my context when installing my fork, but I've literally had it read like 20+ times and use all my usage with my model at times.

      ---

      Title: docs/whitepaper.md at main · ojuschugh1/sqz
      URL: https://github.com/ojuschugh1/sqz/blob/main/docs/whitepaper.md
      Published: N/A
      Author: N/A
      Highlights:
      Large Language Model (LLM) coding agents process tens of thousands of tokens per session through tool outputs — shell commands, file reads, test results, and build logs. These outputs enter the context window raw, consuming budget that could serve reasoning. We present **sqz**, a transparent compression layer that operates *before* tool outputs are injected into context. sqz combines domain-specific structural formatters, content-addressed deduplication, adaptive pressure-aware compression, and entropy-based safety routing to achieve a mean 24.7% reduction on first-pass content and up to 92% savings on repeated reads — without requiring model fine-tuning, prompt modification, or changes to the agent's workflow. Measured across 3,003 real compressions in production agentic sessions, sqz saved 178,442 tokens while maintaining full semantic fidelity on downstream tasks.
      ...
      The emergence of agentic coding tools — Claude Code, Cursor, Codex, Kiro, Gemini CLI — has shifted LLM usage from single-turn question-answering to multi-turn, tool-augmented sessions that can span thousands of interactions. In these sessions, the dominant source of context consumption is not user prompts or model responses, but **tool outputs**: the results of shell commands, file reads, and API calls that accumulate in the context window over time.
      ...
      length tripled from under
      ...
      2,000 tokens in
      ...
      20
      ...
      3 to over 5,400
      ...
      bulk of that
      ...
      context [1]. This creates a
      ...
      Prior work on context compression has focused on two regimes: (1) **prompt compression** that removes redundant tokens from instructions before inference [2, 3, 4], and (2) **post-hoc context pruning** that evicts stale conversation history after it enters the window [5, 6]. Both operate on text *already inside* the context. sqz addresses a third, underexplored regime: **pre-injection compression of tool outputs** — reducing token count before content ever enters the LLM's context window.
      ...
      - Preserves prompt cache validity (the message prefix never changes)
      - Operates independently of the LLM provider
      - Requires no model cooperation or fine-tuning
      - Works transparently across all agent frameworks
      ...
      sqz operates as a PreToolUse hook — a command that the agent framework invokes before delivering tool output to the model. The system comprises four integration surfaces (CLI Hook, MCP Server, Browser Extension, IDE Extension) feeding into a unified Rust core (`sqz_engine`) with an 8-stage compression pipeline. Three supporting subsystems handle persistence (SHA-256 cache, SQLite FTS5 store, CTX session graph), intelligence (TOON lossless encoder, AST parser supporting 18 languages, model router), and analytics (USD cost tracking, multi-agent budgets, correction log). The entire system operates with zero telemetry, fully offline, and is air-gap capable.
      ...
      ### 2.2 Content-Addressed Deduplication
      ...
      Agentic sessions exhibit high repetition: the same file is read multiple times, the same command re-run after edits, the same API response fetched across turns. sqz maintains a persistent SHA-256 content cache (surviving across process invocations via SQLite) and replaces repeated content with a 13-token reference marker (`§ref:HASH§`).
      ...
      This is analogous to the demand paging concept described in [8], where context content is loaded on-demand rather than kept resident. The key insight is that in coding sessions, **the dominant savings come not from compressing individual outputs harder, but from recognizing that the same content appears repeatedly.**
      ...
      Measured empirically: dedup accounts for 93% reduction on repeat reads, compared to 21-58% from first-pass structural compression.
      ...
      Unlike prompt compression tools that require API interception or model-specific adapters, sqz integrates at the **tool execution boundary** — a universal hook point that exists in every agent framework:
      ...
      A single `sqz init` command detects installed frameworks and configures the appropriate hooks. The compression is transparent — the agent receives compressed output without knowing sqz exists.
      ...
      | Content Type | Before | After | Reduction |
      |---|---:|---:|---:|
      | Repeated file reads (5×) | 10,000 | 826 | 92% |
      | Test-fix-test cycle (3 runs) | 15,000 | 5,186 | 65% |
      | Repeated JSON response (3×) | 192 | 79 | 59% |
      | Repeated log lines | 148 | 62 | 58% |
      | Large JSON array | 259 | 142 | 45% |
      | Git diff | 61 | 54 | 12% |
      | Stack trace (safe mode) | 82 | 82 | 0% |
      ...
      Because sqz operates pre-injection (before content enters the context), the message prefix used for prompt caching remains unchanged. This is a critical advantage over post-hoc pruning methods which modify history and invalidate cached prefixes. Empirically, prompt cache hit rates remain at baseline (90%+) with sqz, compared to approximately 85% with history-modifying approaches [5].
      ...
      **Prompt Compression.** LLMLingua [2] and LLMLingua-2 [3] achieve 2-20× compression by removing non-essential tokens using a small classifier model. These operate on the *entire prompt* (system + user + history) and require an additional LLM inference pass. sqz differs by targeting only tool outputs, requiring no LLM call, and operating at the structural rather than token level.
      ...
      **Context Pruning.** ACON [6] optimizes compression guidelines for long-horizon agents by analyzing failure trajectories. Dynamic Context Pruning [5] manages conversation history post-hoc. These complement sqz — they handle what's *already in context*, while sqz reduces what *enters* context.
      ...
      **Structurally Lossless Trimming.** [13] introduces a three-pass algorithm that strips mechanical bloat (raw tool outputs, base64 images, metadata) while preserving user messages verbatim, achieving up to 86% reduction. sqz operates at a complementary point — before injection rather than during trimming — but shares the insight that tool outputs are the dominant source of context bloat.
      ...
      JSONC round
      ...
      tools using JSON-with-comments configs (Open
      ...
      step strips comments
      ...
      this.
      - **No semantic compression**: sqz does not use an LLM to summarize content. This is by design (zero latency, zero cost, deterministic), but means it cannot capture semantic redundancy across structurally different outputs.
      - **Dedup granularity**: Currently whole-content SHA-256. Sub-file dedup (paragraph-level) could capture partial overlaps in modified files.
      ...
      sqz demonstrates that significant token savings (24.7% average, 92% on dedup hits) are achievable through pre-injection structural compression — without model fine-tuning, additional LLM calls, or changes to the agent workflow. By operating at the tool output boundary, sqz is framework-agnostic, cache-friendly, and transparent. The combination of domain-specific formatters, content deduplication, and adaptive pressure-awareness produces a practical system that reduces costs and improves session longevity for real-world agentic coding workflows.

      ---

      Title: yuyi118/DCP
      URL: https://github.com/yuyi118/DCP
      Published: N/A
      Author: N/A
      Highlights:
      Automatically reduces token usage in OpenCode by removing obsolete tool outputs from conversation history.
      ...
      DCP uses multiple tools and strategies to reduce context size:
      ...
      **Discard** — Exposes a `discard` tool that the AI can call to remove completed or noisy tool content from context.
      ...
      **Extract** — Exposes an `extract` tool that the AI can call to distill valuable context into concise summaries before removing the tool content.
      ...
      **Deduplication** — Identifies repeated tool calls (e.g., reading the same file multiple times) and keeps only the most recent output. Runs automatically on every request with zero LLM cost.
      ...
      **Supersede Writes** — Prunes write tool inputs for files that have subsequently been read. When a file is written and later read, the original write content becomes redundant since the current file state is captured in the read result. Runs automatically on every request with zero LLM cost.
      ...
      **Purge
      ...
      Your session history is never modified—DCP replaces pruned content with placeholders before sending requests to your LLM.
      ...
      and performance improvements
      ...
      ```jsonc
      {
          "$schema": "https://raw.githubusercontent.com/Opencode-DCP/opencode-dynamic-context-pruning/master/dcp.schema.json",
          // Enable or disable the plugin
          "enabled": true,
          // Enable debug logging to ~/.config/opencode/logs/dcp/
          "debug": false,
          // Notification display: "off", "minimal", or "detailed"
          "pruneNotification": "detailed",
          // Protect from pruning for <turns> message turns
          "turnProtection": {
              "enabled": false,
              "turns": 4,
          },
          // Protect file operations from pruning via glob patterns
          // Patterns match tool parameters.filePath (e.g. read/write/edit)
          "protectedFilePatterns": [],
          // LLM-driven context pruning tools
          "tools": {
              // Shared settings for all prune tools
              "settings": {
                  // Nudge the LLM to use prune tools (every <nudgeFrequency> tool results)
                  "nudgeEnabled": true,
                  "nudgeFrequency": 10,
                  // Additional tools to protect from pruning
                  "protectedTools": [],
              },
              // Removes tool content from context without preservation (for completed tasks or noise)
              "discard": {
                  "enabled": true,
              },
              // Distills key findings into preserved knowledge before removing raw content
              "extract": {
                  "enabled": true,
                  // Show distillation content as an ignored message notification
                  "showDistillation": false,
              },
          },
          // Automatic pruning strategies
          "strategies": {
              // Remove duplicate tool calls (same tool with same arguments)
              "deduplication": {
                  "enabled": true,
                  // Additional tools to protect from pruning
                  "protectedTools": [],
              },
              // Prune write tool inputs when the file has been subsequently read
              "supersedeWrites": {
                  "enabled": false,
              },
              // Prune tool inputs for errored tools after X turns
              "purgeErrors": {
                  "enabled": true,
                  // Number of turns before errored tool inputs are pruned
                  "turns": 4,
                  // Additional tools to protect from pruning
                  "protectedTools": [],
              },
          },
      }
      ...
      When enabled, turn protection prevents tool outputs from being pruned for a configurable number of message turns. This gives the AI time to reference recent tool outputs before they become prunable. Applies to both `discard` and `extract` tools, as well as automatic strategies.
      ...
      By default, these tools are always protected from pruning across all strategies:
      ...
      `task`, `todowrite`, `todoread`, `discard`, `extract`, `batch`, `write`, `edit`
      ...
      **Subagents** — DCP is disabled for subagents. Subagents are not designed to be token efficient; what matters is that the final message returned to the main agent is a concise summary of findings. DCP's pruning could interfere with this summarization behavior.

      ---

      Title: DC-PC-005: File read deduplication [v2.0] · Issue #213 · radoxtech/diricode
      URL: https://github.com/radoxtech/diricode/issues/213
      Published: 2026-03-24T06:21:05.000Z
      Author: radoxtech
      Highlights:
      Detect and remove duplicate file reads from the active context window, preserving only the most recent read of each file to save ~30% tokens on read-heavy sessions.
      ...
      During a coding session, agents frequently re-read the same files — checking implementation, verifying changes, cross-referencing definitions. Each redundant file read bloats the context window with identical content. By tracking all `file_read` operations and replacing older duplicates with compact placeholders, we reclaim significant context space.
      ...
      - Cline's `FileContextTracker` — deduplicates file reads, keeps most recent version only
      - Aider's repo-map — avoids redundant file reads by maintaining a persistent file index
      - OpenCode — tool results are included in context; large repeated reads cause context bloat
      - Typical savings: 20-40% token reduction on sessions with heavy file exploration
      ...
      - Track all `file_read` tool results per agent conversation (keyed by `filePath + offset + limit`)
      - When same file+range is read again → mark older read as duplicate
      - Replace older reads with compact placeholder: `[Duplicate file read removed: {path} — see most recent read]`
      - Keep only the most recent read (not oldest)
      - Partial reads (different offset/limit ranges) are NOT treated as duplicates
      - Overlapping ranges: if new read fully contains old read's range, old read is duplicate
      - Token savings tracking: `{ duplicatesFound: number, duplicatesRemoved: number, tokensSaved: number }`
      - Savings reported to EventStream for observability
      - Configurable: enable/disable dedup, minimum file size to dedup (skip tiny files)
      ...
      - Deduplicating non-file-read tool results (grep results, bash output, etc.)
      - Automatic file re-reading on external change (handled by DC-PC-009)
      - Cross-agent deduplication (each agent tracks its own reads)
      ...
      File Read Record ──────────────────────────────────────────────────────────
      export const FileReadRecordSchema = z.object({
        /** Unique ID for this read operation */
        readId: z.string(),
        /** File path that was read */
        filePath: z.string(),
        /** Start offset (line number, 1-based) */
        offset: z.number().int().min(1).default(1),
        /** Number of lines read */
        limit: z.number().int().min(1).default(2000),
        /** Token count of the read content */
        tokenCount: z.number().int().min(0),
        /** Timestamp of the read */
        timestamp: z.number(),
        /** Whether this read has been marked as duplicate */
        isDuplicate: z.boolean().default(false),
      });
      ...
      // ── Dedup Configuration ───────────────────────────────────────────────────────
      export const DedupConfigSchema = z.object({
        /** Enable/disable deduplication */
        enabled: z.boolean().default(true),
        /** Minimum token count for a read to be eligible for dedup (skip tiny files) */
        minTokensForDedup: z.number().int().min(0).default(50),
        /** Placeholder template for removed duplicates */
        placeholderTemplate: z.string().default("[Duplicate file read removed: {path} — see most recent read]"),
      });
      ...
      // ── Dedup Result ──────────────────────────────────────────────────────────────
      export const DedupResultSchema = z.object({
        /** Number of duplicate reads found */
        duplicatesFound: z.number().int().min(0),
        /** Number of duplicates actually removed (replaced with placeholder) */
        duplicatesRemoved: z.number().int().min(0),
        /** Tokens saved by deduplication */
        tokensSaved: z.number().int().min(0),
        /** Details per removed duplicate */
        removedReads: z.array(z.object({
          filePath: z.string(),
          readId: z.string(),
          tokensSaved: z.number().int(),
        })),
      });
      ...
      // ── Interface ─────────────────────────────────────────────────────────────────
      export interface IFileReadDeduplicator {
        /** Register a new file read */
        trackRead(record: z.infer<typeof FileReadRecordSchema>): void;
        /** Run deduplication on tracked reads, return result */
        deduplicate(): z.infer<typeof DedupResultSchema>;
        /** Check if a specific file+range has been read before */
        isDuplicate(filePath: string, offset: number, limit: number): boolean;
        /** Get dedup statistics */
        getStats(): { totalReads: number; uniqueFiles: number; duplicateReads: number; tokensSaved: number };
        /** Reset tracker (e.g., on new conversation) */
        reset(): void;
      }
      ...
      - [ ] Duplicate file reads detected: same `filePath + offset + limit` → duplicate
      - [ ] Most recent read preserved, older reads replaced with placeholder
      - [ ] Partial reads (different offset/limit) NOT treated as duplicates
      - [ ] Overlapping range detection: read(1-100) then read(1-200) → first is subsumed
      - [ ] Token savings reported accurately to EventStream
      - [ ] Minimum token threshold: reads under `minTokensForDedup` tokens are not deduplicated
      - [ ] Placeholder text includes the file path for reference
      - [ ] Deduplication is idempotent — running twice produces same result
      - [ ] ~30% token savings on read-heavy sessions (verified with benchmark)
      - [ ] Configurable enable/disable via `DedupConfigSchema`
      - [ ] Unit tests cover: exact duplicate, partial overlap, full overlap (subsumed), no duplicates, min-token threshold, disable toggle
      ...
      - Do NOT deduplicate partial reads with non-overlapping ranges — they contain different content
      - Do NOT remove the most recent read — always keep the latest version
      - Do NOT apply deduplication across different agents — each agent has its own tracker
      - Do NOT deduplicate file reads smaller than `minTokensForDedup` — overhead exceeds savings
      - Do NOT modify the original tool result — create a new context entry with the placeholder
      ...
      - Cline FileContextTracker — file read deduplication pattern
      - Aider repo-map: avoids redundant file reads by maintaining persistent index
      - LRU cache patterns for file read tracking
      - ADR-016: Context management policy

      ---

      Title: Result Caching & Deduplication | Token Optimization Masterclass | AgenticSkillset.org
      URL: https://agenticskillset.org/en/topics/result-caching-deduplication/
      Published: N/A
      Author: N/A
      Highlights:
      In a well-observed agentic system, one of the most striking inefficiencies is this: the same file gets read three times across three iterations because the agent "forgets" it already read it. The same search query is issued twice because the second planning step did not recognize it as a repeat of the first. The same code snippet is analyzed in multiple contexts because no one told the agent it had already been analyzed.
      ...
      This is not a model intelligence failure. It is an architectural failure. The model does not have persistent state outside of its context window. If the fact "file X was already read and the result was Y" is not in the current context, the model will read file X again.
      ...
      Result caching and deduplication add a persistent memory layer outside the context window, enabling agents to avoid re-executing work they have already done. This directly reduces tool call frequency, tool result injection, and the token bloat that comes with it.
      ...
      ### Level 2: Tool Result Caching
      ...
      Cache the outputs of tool executions so that if the same tool is called with the same arguments, the cached result is returned instead of re-executing the tool. This is the higher-value and safer caching layer for most agentic systems.
      ...
      Tip: Always cache at the tool result level before considering LLM response caching. Tool result caching is unconditionally safe (deterministic operations like reading a file produce the same output for the same input), while LLM response caching requires careful consideration of whether the use case requires determinism. Tool result caching alone typically eliminates 20–40% of total tool calls in iterative agentic loops.
      ...
      CACHEABLE (safe to cache):
        ✓ File reads (if files don't change during session)
        ✓ Code parsing / AST analysis
        ✓ Static documentation lookup
        ✓ Schema introspection (database schema, API schema)
        ✓ Grep/search operations on static codebases
        ✓ Token counting and embedding computation
      ...
      NOT CACHEABLE (do not cache):
        ✗ Web
      ...
      ✗
      ...
      data
        ✗
      ...
      ## Deduplication: Detecting and Suppressing Redundant Operations
      ...
      Caching handles the case where the exact same tool call is made twice. Deduplication handles a subtler case: semantically equivalent but syntactically different requests.
      ...
      Examples of semantic duplicates that caching alone misses: -`read_file("src/auth.py")` vs.`read_file("./src/auth.py")`(path normalization) -`search_code("authenticate user")` vs.`search_code("user authentication")`(semantic equivalence) - Reading lines 1-100 of a file, then reading lines 50-150 (overlapping range)
      ...
      ### Semantic Deduplication with Embeddings
      ...
      For search-type tools, use embedding similarity to detect semantically redundant queries:
      ...
      cached_result
      ...
      result:
      ...
      return f"[CACHED
      ...
      result}"
      ...
      execute_search
      ...
      record(query
      ...
      return
      ...
      Tip: Tune the similarity threshold for semantic deduplication carefully. Too high (0.99) and you miss obvious duplicates. Too low (0.80) and you return cached results for genuinely different queries. A threshold of 0.90–0.93 works well for most technical search queries. Instrument your deduplication layer to log similarity scores during testing so you can calibrate for your specific domain.
      ...
      Within a single agent session, in-memory caching is sufficient. But many agentic workflows involve recurring tasks — the same codebase is analyzed daily, the same documentation is queried repeatedly, the same test suite is run across multiple iterations of the same feature.
      ...
      Cross-session caching persists tool results in durable storage and reuses them across separate agent invocations.
      ...
      For codebase analysis agents, the version tag pattern is especially useful: use the git commit hash as the version tag. This ensures cached results are valid only for the specific commit being analyzed.
      ...
      Tip: For product managers: cross-session tool result caching is the technical foundation of "incremental analysis" — the ability to re-run an agentic analysis workflow and only process what has changed since the last run. When proposing agentic features, frame this as a "smart re-run" capability: the first run analyzes everything; subsequent runs only process what's new or changed. This dramatically reduces both cost and runtime for recurring workflows like daily security
      ...
      , weekly documentation updates, or sprint-by-sprint test coverage analysis.
      ...
      ## The Tool Result Summary Pattern
      ...
      Even when caching works perfectly, re-injecting a cached file read result (2,000 tokens) into every subsequent iteration is still expensive. The Tool Result Summary pattern compresses cached results before injecting them:
      ...
      def get_compressed_
      ...
      : str, args: dict,
                                full_
      ...
      : str, context_budget: int) -> str
      ...
      """Return a compressed version of a tool result if the full version is too large."""
          full_token_count = count_tokens(full_result)

          if full_token_count <= context_budget:
              return full_result  # Full result fits — no compression needed

          # Check if we already have a compressed version
          cache_key = f"compressed:{tool_name}:{hash(full_result)}"
          if cache_key in RESULT_SUMMARY_CACHE:
              return RESULT_SUMMARY_CACHE[cache_key]
      ...
      Compress the result
          compression_prompt = f"""
          This is the output of a {tool_name} operation.
          Compress it to under {context_budget} tokens while preserving all
          technically significant information. Remove boilerplate, comments
          from standard library code, and repeated patterns.

          Output:
          {full_result[:8000]}
          """
          compressed = llm.invoke([HumanMessage(content=compression_prompt)]).content
          RESULT_SUMMARY_CACHE[cache_key] = compressed
          return compressed
      ...
      The first time a tool result is used, the full result is available. For all subsequent iterations, the compressed version (produced once at a small LLM cost) is used instead of the full result.
      ...
      Tip: The threshold for triggering compression should be relative to your context budget, not a fixed number. A result that is "too large" for an agent with a 2,000-token context budget may be perfectly fine for an agent with a 10,000-token budget. Configure your compression threshold as a percentage of available context (e.g., "compress any result that would consume more than 25% of the remaining context budget").
      ...
      Result caching and deduplication attack the problem of repeated work from two directions: caching stores and reuses exact-match results, while deduplication detects and prevents semantically equivalent repeat operations. Together they typically eliminate 20–50% of tool calls in iterative agentic workflows.
      ...
      The cache policy framework (cacheable vs. non-cacheable vs. conditionally cacheable) prevents caching from introducing stale data bugs. Cross-session caching with version tags (git commit hashes for code analysis) extends these savings across multiple agent runs. The Tool Result Summary pattern addresses the companion problem of repeated re-injection of large cached results.
      ...
      With caching and deduplication in place, your agentic system stops paying for the same information twice — one of the most reliable and highest-ROI optimizations available at the loop level.

      ---

      Title: docs/deduplication.md at main · SimplyLiz/ContextCompressionEngine
      URL: https://github.com/SimplyLiz/ContextCompressionEngine/blob/main/docs/deduplication.md
      Published: N/A
      Author: N/A
      Highlights:
      Exact and fuzzy duplicate detection for repeated content in long conversations.
      ...
      Long-running conversations — especially agentic coding sessions — accumulate repeated content: the same file read multiple times, identical grep results, duplicate test output. Dedup detects these repetitions and replaces earlier occurrences with a compact reference, keeping only the latest copy.
      ...
      Both exact and fuzzy dedup are **fully lossless** — originals are always stored in the verbatim map and restored by `uncompress()`.
      ...
      1. Filter eligible messages (skip preserved roles, tool_calls, already-compressed, content < 200 chars)
      2. Compute djb2 hash of each message's content (with length prefix to reduce collisions)
      3. Group messages by hash value
      ...
      1. Within each hash group, sub-group by exact content match
      2. For groups with 2+ identical messages, select a **keep target**:
       - Prefer the first occurrence within the recency window
       - Otherwise, keep the latest occurrence
      3. Mark all other occurrences as duplicates
      ...
      A message is eligible for exact dedup when:
      ...
      - Role is not in the `preserve` list
      - No `tool_calls` array
      - Content doesn't start with `[summary:`, `[summary#`, or `[truncated`
      - Content length >= 200 chars
      ...
      Detects near-duplicates using line-level Jaccard similarity. Useful when the same file is read across edit cycles — the content evolves slightly but remains largely the same.
      ...
      - Skip indices already handled by exact dedup
      - Normalize lines: trim, lowercase, filter empty
      - Extract fingerprint: first 5 non-empty normalized lines
      - Require at least
      ...
      2 normalized lines
      ...
      1. Build inverted index: fingerprint line -> list of eligible indices
      2. Find candidate pairs: eligible indices that share >= 3 fingerprint lines
      3. Only forward pairs (avoid duplicates)
      ...
      1. **Length-ratio pre-filter** — skip pairs where `min/max length ratio < 0.7`
      2. **Line-level Jaccard** — `|A intersection B| / |A union B|` using multiset frequency maps
      3. Accept pairs above the `fuzzyThreshold` (default: 0.85)
      ...
      1. Use union-find to group transitively connected fuzzy-duplicates
      2. For each group with 2+ members:
       - Prefer the first occurrence in the recency window as keep target
       - Otherwise, keep the latest occurrence
      3. Mark all others with their similarity score
      ...
      | Threshold | Use case |
      | --------- | ------------------------------------------------------------------------------ |
      | `0.85`+ | Safe default. Catches near-identical content (whitespace changes, minor edits) |
      | `0.7` | Catches file reads across small edit cycles |
      | `0.6` | Aggressive. May group content that shares structure but differs in specifics |
      ...
      **Agentic coding sessions** — An AI assistant reads the same file 5 times during a debugging session. With exact dedup, 4 of the 5 reads become compact references. With fuzzy dedup, reads across edits are also caught.
      ...
      **Repeated tool output** — Test runners, linters, and build tools produce similar output across runs. Fuzzy dedup catches the minor variations.
      ...
      - Dedup runs **before** compression. Dedup-annotated messages are handled in the dedup path and skip the normal summarization path.
      - Dedup respects `preserve` roles and `recencyWindow` — messages in these categories are never deduped.
      - Both exact and fuzzy dedup annotations include provenance metadata (`_cce_original`), so `uncompress()` restores the original content.

      ---

      Title: feat(agent): tool call deduplication to prevent redundant execution loops · Issue #18076 · NousResearch/hermes-agent
      URL: https://github.com/NousResearch/hermes-agent/issues/18076
      Published: 2026-04-30T20:10:34.000Z
      Author: andrewhosf
      Highlights:
      ## feat(agent): tool call deduplication to prevent redundant execution loops
      ...
      # PR Proposal: Agent-Level Tool Call Deduplication / Loop Prevention
      ...
      The Hermes agent can enter expensive loops where it executes the same tool call (e.g., `git log`, `search_files`) multiple times in a row with identical arguments, producing identical output each time. This wastes tokens, burns context window space, and delays actual work.
      ...
      agent ran `git log --all --oneline --
      ...
      ="tokens_per_
      ...
      \|tok/
      ...
      .*sec\|speed" -- cli.
      ...
      run_agent.py` 30+ times consecutively with slightly different grep patterns, all returning the same 7 unrelated commits. The agent never recognized it had already confirmed the feature didn'
      ...
      exist upstream.
      ...
      - **PR #16641** (tool-call loop guardrails): Detects repeated *failing* or *non-progressing* tool calls within a single turn and injects warnings. Our bug was repeated *successful* calls across multiple turns that returned the same information.
      - **PR #3006 / #8126** (tool result caching): Caches results for identical calls. This would help but doesn't address the root cause — the model shouldn't generate the redundant calls in the first place.
      ...
      Add an agent-level **tool call deduplication and progress tracking layer** that sits between the model's output and actual tool execution.
      ...
      1. **Per-turn tool call registry**: Hash (tool_name + normalized_args) → output
      2. **Before executing any tool**: Check if this exact call was already made this turn
      3. **If duplicate**: Return cached result with a `duplicate: true` flag and append a system note: `"Note: This tool call was already executed in this turn with identical arguments. Result was: ..."`
      4. **If similar but not exact** (same tool, slightly different args, same output): Flag as "no new information" after N repeats
      5. **Cross-turn tracking** (optional): Maintain a short LRU cache of recent tool calls to catch loops that span multiple turns
      ...
      class ToolCallRegistry:
          """Tracks tool calls within a turn to prevent redundant execution."""

          def __init__(self, max_history=100):
              self._history = {}  # hash -> (args, output, timestamp)
              self._max_history = max_history

          def check(self, tool_name: str, args: dict) -> tuple[bool, Any]:
              """Returns (is_duplicate, cached_output) if this exact call was already made."""
              key = self._hash(tool_name, args)
              if key in self._history:
                  return True, self._history[key][1]
              return False, None

          def record(self, tool_name: str, args: dict, output: Any):
              """Record a tool call result for deduplication."""
              key = self._hash(tool_name, args)
              self._history[key] = (args, output, time.time())

          def _hash(self, tool_name: str, args: dict) -> str:
              # Normalize args (sort dict keys, handle lists)
              normalized = json.dumps(args, sort_keys=True, default=str)
              return hashlib.sha256(f"{tool_name}:{normalized}".encode()).hexdigest()[:32]

          def reset(self):
              """Clear history at the start of each turn."""
              self._history.clear()
      ...
      - **run_agent.py**: Instantiate `ToolCallRegistry` at turn start, check before each tool execution
      - **cli.py**: Add config option `tool_deduplication.enabled` (default: true)
      - **Config**: Add to `hermes_cli/config.py` and `cli-config.yaml.example`
      ...
      ```yaml
      tool_deduplication:
        enabled: true
        # How many identical calls before we force a cache hit
        exact_duplicate_threshold: 1  # Always dedup exact duplicates
        # How many similar calls (same tool, different args, same output) before warning
        no_progress_threshold: 3
        # Whether to append a system note when returning cached results
        append_system_note: true
      ```
      ...
      - Agent makes same `read_file` call twice in one turn → second returns cached
       - Agent runs `git log` with different grep patterns → all tracked separately
       - Turn reset: new turn can re-run same tool with fresh result
      ...
      - **Risk**: Low. This is a pure optimization — it only prevents redundant execution, never blocks new calls.
      ...
      Instead of full deduplication, we could add a **"no new information" detector**:
      ...
      - After any tool call, hash the output
      - If the same tool was called recently with the same output, append a note: `"This produced the same result as the previous call. Consider if you already have the answer."`
      - This is lighter weight but less aggressive about preventing the redundant execution itself.
      ...
      Implement the full deduplication approach. The cost is low (one dict lookup per tool call), the benefit is high (prevents the exact loop we hit), and it generalizes to any idempotent tool.
      ...
      This PR would complement those by addressing the execution layer, not just the result caching or warning layer.

      ---

      Title: blog/twir-lean-ctx.md
      URL: https://github.com/yvgude/lean-ctx/blob/main/blog/twir-lean-ctx.md
      Published: N/A
      Author: N/A
      Highlights:
      lean-ctx is an open-source context runtime that sits between AI coding tools (Cursor, Claude Code, Copilot, etc.) and the filesystem. It compresses file reads with AST-aware intelligence, strips noise from shell output via 95+ patterns, and manages cross-session memory. A single Rust binary, 76 MCP tools, zero runtime dependencies.
      ...
      AI coding agents read the same files repeatedly. A typical session reads `main.rs` ten to fifteen times at roughly 2,000 tokens each. Shell commands like `cargo build` or `git log` produce verbose output that burns through context windows. lean-ctx intercepts these reads and shell outputs, caching, compressing, and deduplicating before the LLM ever sees them.
      ...
      The performance constraint is strict: lean-ctx sits in the hot path of every tool call. Responses must complete in under 50ms. That ruled out any approach involving local LLM inference for summarization and pushed us toward deterministic, algorithmic compression — which turned out to be both faster and more reliable.
      ...
      Different files benefit from different compression levels. A 50-line utility file shouldn't be compressed the same way as a 2,000-line service module. Rather than hard-coding thresholds, lean-ctx learns them during each session using a multi-armed bandit.
      ...
      Each "arm" represents a compression threshold. The bandit selects which threshold to apply, then observes whether the AI agent needed to re-read the file (a signal that compression was too aggressive). Over a session, the bandit converges on the threshold that balances token savings against information loss.
      ...
      The bandit state (`alpha`, `beta` per arm) is serializable via serde — it persists across tool calls within a session but resets between sessions, since file characteristics change as the developer edits code.
      ...
      , and the
      ...
      , discarding the
      ...
      When a file is first read, lean-ctx stores both an MD5 hash of the content and the filesystem mtime. On subsequent reads:
      ...
      - If `mtime` hasn't changed, the cache is valid — return a compressed stub (~13 tokens instead of ~2,000).
      - If `mtime` is newer, invalidate and re-read from disk. Even if the hash happens to match (the user saved without changes), we re-read to be safe.
      - For `mode=full` reads, content hash comparison catches the case where mtime changed but content didn't.
      ...
      This dual-signal approach was motivated by real user feedback: without mtime validation, LLMs would sometimes receive stale cached content after a file edit and get confused, spending more tokens trying to work around the stale data than the cache saved in the first place.
      ...
      Each tool call flows through `ServerHandler::call_tool` — an async method that routes to specific handlers (read, shell, search, etc.), applies post-processing (compression, archiving, throttle warnings), and tracks metrics. A meta-tool `ctx` allows the AI to call any lean-ctx tool by name, so the agent can write `ctx(tool="read", path="src/main.rs")` instead of `ctx_read(path="src/main.rs")` — reducing the number of tool definitions the model needs to track.

      ---

      Title: docs/research/context-management.md at main · epicsagas/collet
      URL: https://github.com/epicsagas/collet/blob/main/docs/research/context-management.md
      Published: N/A
      Author: N/A
      Highlights:
      Despite these advances, **artifact tracking remains unsolved** — even the best systems score only 2.45/5.0 on file modification tracking in independent evaluations. Compaction is therefore best understood as a damage-limitation mechanism, not a lossless compression.
      ...
      extraction, and a `
      ...
      Tool results are the dominant source of context bloat. A typical coding agent turn structure skews heavily toward tool output:
      ...
      - File reads (full file contents, often 500–2000 lines)
      - Shell command output (build logs, test results, compiler errors)
      - Search results (multiple files, multiple matches)
      - LLM reasoning chains across many turns
      ...
      This is why observation masking — targeting tool outputs specifically — is so effective. The action/reasoning history is comparatively small; the tool output history is vast.
      ...
      | Information Type | Typically Preserved | Typically Discarded |
      |-----------------|--------------------|--------------------|
      | System prompt | Always (verbatim) | Never |
      | Last 2–3 user turns | Always (verbatim) | N/A |
      | User goal / intent | In summary | Original phrasing |
      | File paths modified | In summary (weakly) | File contents |
      | Tool call facts | That they occurred | Full output details |
      | Error messages | Key errors in summary | Full stack traces |
      | Implementation decisions | In summary | Reasoning chain |
      | Images / PDFs | Never (stripped) | Always |
      | Code blocks < 50 lines | Sometimes verbatim | Longer blocks |
      | Progress messages | Usually discarded | Yes |
      ...
      Even the best approach (Factory structured) scores only **2.45/5.0 on artifact tracking** — file modification tracking remains an unsolved problem. This argues for a separate artifact registry maintained outside the compaction boundary.
      ...
      **Key design principle**: Tool result compaction should always run first. It reclaims the most space at zero LLM cost.
      ...
      Factory.ai's approach avoids the "re-reading loop" failure mode:
      ...
      Anchored iterative summarization maintains a **persistent structured summary** that new information merges into, rather than regenerating from scratch each compaction cycle. This prevents accumulated detail loss and avoids the re-reading cycle.
      ...
      OpenCode's 5-section summary template (Goal, Instructions, Discoveries, Accomplished, Relevant Files) implements a similar pattern, with the additional feature of tool output pruning as a separate pass that skips the last 2 user turns and always preserves at least 40,000 tokens of recent tool output.
      ...
      | Technique | Description |
      |-----------|-------------|
      | **Structured summary sections** | Explicit sections force preservation of file paths, decisions, errors |
      | **Virtual file preservation** | Post-compaction, save original history as an accessible file reference |
      | **Tool output to disk** | Large outputs saved as file references rather than inline |
      | **Verbatim error preservation** | Error messages and stack traces kept exactly, not paraphrased |
      | **Verbatim code blocks** | Short code blocks (< 50 lines) preserved completely in summary |
      | **Overlap across boundaries** | Last N events carry forward across compression boundaries |
      | **Protected recent turns** | Last 2–3 turns never touched regardless of pressure |
      | **Protected tool types** | Specific high-value tools (e.g., "skill") never pruned |
      | **Weak model for summarization** | Use a cheaper model for summaries; preserve budget for main model |
      | **Decision log outside compaction** | Pin implementation decisions in a separate section exempt from compaction |
      ...
      1. **File artifact tracking**: Best systems score only 2.45/5.0. The recommended mitigation is a separate artifact registry maintained entirely outside the compaction-eligible conversation history.
      2. **Re-reading loops**: Summarization loses details that agents later need, forcing re-reads that grow context again. Virtual file access to pre-compaction state partially mitigates this.
      3. **Compaction timing**: Too early wastes information unnecessarily; too late risks overflow. Autonomous agent-driven timing (LangGraph Deep Agents pattern) is promising but remains under-tested.
      ...
      The counterintuitive finding: naive full caching can backfire. Tool results change every turn, and including them in the cache boundary forces constant cache invalidation that eliminates the benefit. **The optimal strategy is to cache static content (system prompt, project rules, repo map) and explicitly exclude dynamic content (tool results, conversation history, current user message).**
      ...
      ### 5.6 SimHash Deduplication
      ...
      Within conversation history, similar content (e.g., repeated file reads, redundant tool outputs) can be identified using locality-sensitive hashing (SimHash) and collapsed before context assembly. This is a zero-LLM-call technique that removes semantic near-duplicates — particularly effective for agents that re-read files across multiple turns.
      ...
      5: Context Assembly
      ...
      ```
      Stage 1: SimHash Deduplication
       → Identify and remove semantically similar messages
      ...
      Stage 2: Relevance Scoring (Jaccard)
       → Score each message for relevance to current query
      ...
      Stage 3: Structural Bonus
       → Extra score for code blocks, error messages, tool outputs
      ...
      Stage 6: Structured Summarization
       → Preserve key information from removed messages via structured summary
      ...
      ### 6.5 Tool Result Handling
      ...
      Tool results are capped at approximately 12k characters and are explicitly not included in the prompt caching boundary. For large tool outputs:
      ...
      - File reads beyond the cap provide `offset`/`limit` parameters for chunked access
      - Build output (cargo, test runners) is truncated to the most recent errors
      - A structured error feedback format guides the LLM toward the relevant portion of truncated output
      ...
      The bottleneck analysis identified that LLMs do not spontaneously use `offset` parameters; the agent must be prompted with actionable guidance (e.g., "output was truncated at line 450; use `offset=450` to continue reading") to avoid stalling on incomplete information.
      ...
      collet's BLAKE3 incremental hashing (originally for avoiding re-parsing unchanged files) provides the mechanism needed for cache-aware prompt construction. Static prompt sections (system prompt, project rules, repo map) are placed at the beginning of the request in a stable order, with cache breakpoints inserted at each boundary. Dynamic content (conversation history, tool results, current message) follows.
      ...
      ### 7.3 Artifact Tracking Is an Unsolved Problem
      ...
      No current compaction approach adequately tracks file modifications. The best result (Factory structured: 2.45/5.0) is worse than a passing grade. This is not a tuning problem — it is a fundamental limitation of representing file state changes as natural language.
      ...
      **Implication**: File artifact tracking requires a separate, dedicated mechanism outside the conversation summary — a structured data store updated on every write operation, not derived from the conversation post-hoc.
      ...
      Research (arXiv:2601.06007) shows that strategic cache boundary placement — specifically, caching system prompt and static project content while explicitly excluding tool results — achieves 60–80% input token savings and 20–31% TTFT improvement. Naive full caching is unstable and can actually increase costs due to constant invalidation from dynamic content.
      ...
      **Implication**: Prompt construction order matters for economics, not just correctness. Static content before dynamic content is a hard requirement for caching to function, and tool results must never be inside the cached boundary.
      ...
      Microsoft Agent Framework's `TokenBudgetComposedStrategy` pattern (tool result compaction → summarization → truncation, with early stop) prevents the common failure mode of applying aggressive summarization when tool result collapsing alone would have sufficed. Aggressive summarization on insufficiently pressured contexts wastes information unnecessarily.
```
/answer b015c9af-cd42-4363-b098-92c9755856db allow
Control Request:
```yaml
id: 3845b40c-ae32-4153-bca3-10f0792b9488
toolName: web-search-exa
arguments:
  query: Claude Code how does it avoid reading the same file twice context management
```
/answer 3845b40c-ae32-4153-bca3-10f0792b9488 allow
Control Request:
```yaml
id: a12e5b9d-d9b8-4e7f-8f74-b4f41f561b93
result:
  structuredContent:
    content: |-
      Title: Why Does Claude Code Read the Same File Multiple Times? | BSWEN
      URL: https://docs.bswen.com/blog/2026-03-22-claude-code-redundant-file-reads/
      Published: 2026-03-22T00:00:00.000Z
      Author: Cowrie   Dev @ Bswen
      Highlights:
      I think the key reason is architectural: Claude Code operates without persistent session memory for file contents.
      ...
      Each file access is treated as a fresh start. Claude has no internal record of what it has already opened. This is by design—the system doesn’t maintain a cache of previously read content.
      ...
      When Claude needs information about a file, it reads it. Period. Whether that file was opened 10 seconds ago or never before makes no difference to the decision logic.
      ...
      Claude cannot tell
      ...
      module without opening
      ...
      When searching for
      ...
      , Claude often reads
      ...
      files just to discover they’
      ...
      The architecture appears to favor statelessness. Each interaction is self-contained. This simplifies reasoning about the system—there’s no cache to invalidate, no stale data to worry about—but it means every file access starts from zero.
      ...
      Files change. Between reads, a file could be modified by the user, by Claude itself, or by another process. A caching layer would need to detect and handle all these cases.
      ...
      Rather than build complex cache invalidation logic, the current approach is: just read it again. Safe, but inefficient.
      ...
      ### Context Window Constraints
      ...
      Claude can’t hoard all previously read content in its context window. The context fills up quickly, and earlier file contents get pushed out. Even if Claude “remembered” reading a file, the actual content might no longer be accessible in the conversation history.
      ...
      This creates a situation where Claude knows it read`config.ts` but can’t recall what was in it—so it reads it again.
      ...
      You can reduce redundant reads by changing how you interact with Claude Code.
      ...
      ### Provide Content Explicitly
      ...
      Instead of letting Claude search, paste the relevant code directly:
      ...
      When you provide content explicitly, Claude doesn’t need to read files at all.
      ...
      ### Point to Specific Files
      ...
      Instead of open-ended exploration, direct Claude to exactly where it should look:
      ...
      ```
      "Check the authentication logic in /src/middleware/auth.tslines 45-60 for the bug."
      ...
      This reduces the scatter-shot reading pattern where Claude opens multiple files searching for relevant code.
      ...
      ### Use Targeted Questions
      ...
      Broad questions trigger broad searches. Narrow questions get focused results:
      ...
      ### Batch Related Requests
      ...
      If you have multiple questions about the same file, ask them together:
      ...
      ```
      Separated (triggers multiple reads):Q1: "What does auth.ts export?"[later]Q2: "Show me the validateToken function"[later]Q3: "How does handleRefresh work?"Batched (one read):"In auth.ts, tell me: 1) what it exports,2) how validateToken works, 3) the handleRefresh logic"
      ```
      ...
      In this post, I explained why Claude Code reads the same file multiple times. The key point is Claude lacks persistent session memory for file contents—each read is treated as independent, leading to significant redundancy.
      ...
      The architectural tradeoff favors simplicity over efficiency. Claude doesn’t track what it has read because tracking introduces complexity around cache invalidation, stale data, and context window management.
      ...
      You can mitigate this by providing file contents explicitly, pointing to specific locations, and batching related questions. The more targeted your requests, the less Claude needs to search—and the fewer redundant reads you’ll see.

      ---

      Title: Manage sessions
      URL: https://code.claude.com/docs/en/sessions.md
      Published: N/A
      Author: N/A
      Highlights:
      ## Manage context within a session
      ...
      These commands control what's in the context window without leaving the session:
      ...
      - `/clear`: start fresh with an empty context. The previous conversation is saved and resumable with `/resume`, or, in the same Claude Code process, {/* min-version: 2.1.191 */}from the rewind menu's previous-session entry
      - `/compact [instructions]`: replace history with a summary, optionally focused on what you specify
      - `/context`: show what is currently consuming context
      ...
      For how compaction interacts with CLAUDE.md, skills, and rules, see the context window guide. For strategies on when to clear versus compact, see Best practices.
      ...
      By default, transcripts are stored as JSONL at `~/.claude/projects//.jsonl`, where `` is your working directory path with non-alphanumeric characters replaced by `-`. Each line is a JSON object for a message, tool use, or metadata entry. The entry format is internal to Claude Code and changes between versions, so scripts that parse these files directly can break on any release. To build on session data, use `/export` or the script interfaces instead.
      ...
      | To |
      ...
      | --- | --- | --- |
      ...
      Move storage off `~/.claude` | `CLAUDE_CONFIG_DIR` | Environment variable |
      | Change the
      ...
      30-day retention | `cleanupPeriodDays` | `settings.json` |
      | Supp
      ...
      transcript writes in all modes | `CLAUDE_CODE_SKIP_PROMPT_HISTORY` | Environment variable |
      | Supp
      ...
      writes for one non-interactive run | `--no-session-persistence` | CLI flag with `claude -p` |

      ---

      Title: How Claude remembers your project - Claude Code Docs
      URL: https://code.claude.com/docs/en/memory
      Published: N/A
      Author: N/A
      Highlights:
      Claude Code has two complementary memory systems. Both are loaded at the start of every conversation. Claude treats them as context, not enforced configuration. To block an action regardless of what Claude decides, use a PreToolUse hook instead. The more specific and concise your instructions, the more consistently Claude follows them.
      ...
      CLAUDE.md files are loaded into the context window at the start of every session, consuming tokens alongside your conversation. The context window visualization shows where CLAUDE.md loads relative to the rest of the startup context. Because they're context rather than enforced configuration, how you write instructions affects how reliably Claude follows them. Specific, concise, well-structured instructions work best.
      ...
      Size: target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce adherence. If your instructions are growing large, use path-scoped rules so instructions load only when Claude works with matching files. You can also split content into imports for organization, though imported files still load and enter the context window at launch.
      ...
      Claude Code reads CLAUDE.md files by walking up the directory tree from your current working directory, checking each directory along the way for `CLAUDE.md` and `CLAUDE.local.md` files. This means if you run Claude Code in `foo/bar/`, it loads instructions from `foo/bar/CLAUDE.md`, `foo/CLAUDE.md`, and any `CLAUDE.local.md` files alongside them.
      ...
      All discovered files are concatenated into context rather than overriding each other. Across the directory tree, content is ordered from the filesystem root down to your working directory. For the `foo/bar/` example, `foo/CLAUDE.md` appears in context before `foo/bar/CLAUDE.md`, so instructions closer to where you launched Claude are read last. Within each directory, `CLAUDE.local.md` is appended after `CLAUDE.md`, so your personal notes are the last thing Claude reads at that level.
      ...
      Claude also discovers `CLAUDE.md` and `CLAUDE.local.md` files in subdirectories under your current working directory. Instead of loading them at launch, they are included when Claude reads files in those subdirectories.
      ...
      For larger projects, you can organize instructions into multiple files using the `.claude/rules/` directory. This keeps instructions modular and easier for teams to maintain. Rules can also be scoped to specific file paths, so they only load into context when Claude works with matching files, reducing noise and saving context space.
      ...
      Rules load into context every session or when matching files are opened. For task-specific instructions that don't need to be in context all the time, use skills instead, which only load when you invoke them or when Claude determines they're relevant to your prompt.
      ...
      first 200 lines of `MEMORY.md`, or the first 25KB, whichever comes first, are loaded at the start of every conversation. Content beyond that threshold is not loaded at session start. Claude keeps `MEMORY.md` concise by moving detailed notes into separate topic files.
      ...
      The `/memory` command lists your CLAUDE.md, CLAUDE.local.md, and other memory file locations across user and project scopes, lets you toggle auto memory on or off, and provides an option to open the auto memory folder. Select any file to open it in your editor. To check which files actually loaded into the current session, run `/context`.
      ...
      - Run `/context` to verify your CLAUDE.md and CLAUDE.local.md files loaded. If a file is missing from the breakdown, Claude can't see it. Use `/memory` to open and edit the files.
      - Check that the relevant CLAUDE.md is in a location that gets loaded for your session (see Choose where to put CLAUDE.md files).
      ...
      - Make instructions more specific. "Use 2-space indentation" works better than "format code nicely."
      - Look for conflicting instructions across CLAUDE.md files. If two files give different guidance for the same behavior, Claude may pick one arbitrarily.
      ...
      Files over 200 lines consume more context and may reduce adherence. Use path-scoped rules to load instructions only when Claude works with matching files, or trim content that isn't needed in every session. Splitting into `@path` imports helps organization but doesn't reduce context, since imported files load at launch.
      ...
      Project-root CLAUDE.md survives compaction: after `/compact`, Claude re-reads it from disk and re-injects it into the session. Nested CLAUDE.md files in subdirectories are not re-injected automatically; they reload the next time Claude reads a file in that subdirectory.
      ...
      AUDE.md that

      ---

      Title: How Claude Code uses prompt caching
      URL: https://code.claude.com/docs/en/prompt-caching.md
      Published: N/A
      Author: N/A
      Highlights:
      Each time you send a message in Claude Code, it makes a new API request. The model doesn't remember anything between requests, so Claude Code re-sends the full context: the system prompt, your project context, every prior message and tool result, and your new message. New content is appended at the end, which means most of each request is identical to the one before it. Prompt caching is how the API avoids reprocessing the part that didn't change.
      ...
      The API caches by matching the start of each request, called the prefix, against content it recently processed. On a normal turn, the prefix is the entire previous request and only the latest exchange is new. The match is exact, so a change anywhere in the prefix recomputes everything after it. There is no per-file or per-segment caching. See how prompt caching works in the API reference for the underlying mechanism.
      ...
      File contents enter context only when Claude reads them, and reads append to the conversation. Editing a file Claude previously read does not retroactively change the earlier read in history. Instead, Claude Code appends a ` ` noting the file changed, and Claude re-reads it if needed.
      ...
      Your project-root and user-level CLAUDE.md files are read once at session start and held in memory. Editing them mid-session does not invalidate the cache, but the edit also doesn't apply. Claude keeps working with the version that was loaded at session start. The new content loads on the next `/clear
      ...
      Nested CLAUDE.
      ...
      files in subdirectories and rules with
      ...
      paths:` frontmatter load
      ...
      Claude first reads a matching file
      ...
      is part of
      ...
      Restoring file checkpoints alongside the conversation has no separate effect on the cache. File contents enter context only when Claude reads them, the same as editing files in your repository.
      ...
      In Claude Code, the cache is effectively scoped to one machine and directory. The system prompt embeds the working directory, platform, shell, OS version, and auto-memory paths, so two sessions in different directories build different prefixes and miss each other's cache. That includes worktrees of the same repository, since each worktree has its own working directory.
      ...
      Sessions you run in parallel in the same directory build matching prefixes and read each other's cache. Sequential sessions share the prefix only when the git status snapshot at startup matches, since the system prompt also captures branch and recent commits.

      ---

      Title: Boucle-framework/tools/read-once at main · Bande-a-Bonnot/Boucle-framework · GitHub
      URL: https://github.com/Bande-a-Bonnot/Boucle-framework/tree/main/tools/read-once
      Published: 2026-02-28T13:11:17.000Z
      Author: Bande-a-Bonnot
      Highlights:
      Stop Claude Code from re-reading files it already has in context.
      ...
      A PreToolUse hook that tracks file reads within and across sessions. When Claude tries to re-read a file that has not changed, the hook tells Claude the content is already in context. Saves ~2000+ tokens per prevented same-session re-read.
      ...
      By default, read-once uses warn mode: it allows the read but attaches an advisory message. Set`READ_ONCE_MODE=deny` for hard blocking of same-session re-reads. Deny mode still allows the first read in a new session, even when another session has the same file cached, so Edit's prior-Read requirement is not deadlocked by cross-session cache hits.
      ...
      1. Hook intercepts every`Read` tool call
      2. Partial reads (with`offset` or`limit`) always pass through — only full-file reads are cached
      3. Checks a session-scoped cache: has this session read the file before?
      4. Checks a global cache: has another session seen the same path and mtime recently?
      5. If this session already read the unchanged file, warn mode allows with an advisory and deny mode blocks
      6. If only another session saw it, read-once always allows the first current-session read and records it for future same-session dedupe
      7. If the file changed since last read, allows it through (or shows just the diff — see below)
      8. Cache entries expire after 20 minutes (configurable) to handle context compaction
      ...
      When you're iterating on
      ...
      , edit it,
      ...
      Claude already has
      ...
      old version in context
      ...
      read-once shows only
      ...
      changed instead of
      ...
      full file. A 3-line change in a 200-line file costs ~30 tokens instead of ~2000.
      ...
      Claude then proceeds without the redundant read. No loss of information — the file content is still in the context window from the first read.
      ...
      Claude Code compacts the context window during long sessions, dropping older content. A file read 30 minutes ago might no longer be in the working context.
      ...
      read-once handles this two ways:
      ...
      PostCompact hook (recommended):`compact.sh` registers as a PostCompact hook and clears the session cache immediately when compaction occurs. The global cache is kept as advisory state, so a post-compaction first read is allowed and future same-session re-reads can still be deduplicated. The installer configures this automatically.
      ...
      TTL fallback: Cache entries also expire after`READ_ONCE_TTL` seconds (default: 1200 = 20 minutes). This catches cases where PostCompact is not configured.
      ...
      | Variable | Default | Description |
      | --- | --- | --- |
      | `READ_ONCE_MODE` | `warn` | `warn` allows reads with advisory message.`deny` blocks same-session re-reads, while cross-session first reads still pass through with an advisory. |
      | `READ_ONCE_TTL` | `1200` | Cache TTL in seconds. After this, re-reads are allowed (compaction safety). |
      | `READ_ONCE_DIFF` | `0` | Set to`1` to show only diffs when files change (instead of full re-read). |
      | `READ_ONCE_DIFF_MAX` | `40` | Max diff lines before falling back to full re-read. |
      | `READ_ONCE_DISABLED` | `0` | Set to`1` to disable the hook entirely. |
      | `READ_ONCE_PYTHON_CMD` | auto | Override Python launcher detection (`py`,`python3`, then`python`). |
      ...
      The Edit tool says "File has not been read yet" even though I already read it. Claude Code's Edit tool requires a successful Read in the current session before it will edit a file. In deny mode (`READ_ONCE_MODE=deny`), read-once only blocks after the current session has already seen the file. If another session cached the file first, the first current-session read still passes through with an advisory, then later same-session re-reads can be blocked.
      ...
      Won't this break after context compaction? The installer configures a PostCompact hook (`compact.sh`) that clears the cache immediately when compaction happens. As a fallback, cache entries also expire after 20 minutes (configurable via`READ_ONCE_TTL`). You can also run`read-once clear` to reset manually.
      ...
      Claude Code reads small chunks, not whole files — does this help? Partial reads with`offset` or`limit` are never cached. They always pass through. read-once only deduplicates full-file reads where the entire file is requested again.
      ...
      Isn't there a good reason Claude re-reads files? Yes — when the file changed, or when context compacted. read-once only blocks re-reads when the file hasn't changed (same mtime) and the cache is recent. Changed files always pass through. With diff mode enabled (`READ_ONCE_DIFF=1`), changed files show just the delta instead of the full content.

      ---

      Title: read-once: A Claude Code Hook That Stops Redundant File Reads - DEV Community
      URL: https://dev.to/boucle2026/read-once-a-claude-code-hook-that-stops-redundant-file-reads-4bjk
      Published: 2026-03-07T01:23:53.000Z
      Author: N/A
      Highlights:
      Claude Code re-reads files constantly. It reads a file, edits it, reads it again to verify. It re-reads config files across different subtasks. When subagents share a session, they re-read the same files the parent already loaded.
      ...
      ~2,0
      ...
      I built read-once to fix this. It's a Claude Code hook that tracks file reads within a session and blocks redundant ones.
      ...
      read-once is a PreToolUse hook that intercepts every`Read` tool call:
      ...
      1. First read of a file: allows it, records the file path and modification time
      2. Re-read of an unchanged file: blocks it, tells Claude the content is already in context
      3. Re-read of a changed file: allows it (or shows just the diff)
      4. Cache entries expire after 20 minutes to handle context compaction
      ...
      0 tokens.
      ...
      Claude proceeds without the redundant read. No information is lost -- the file content is still in the context window from the first read.
      ...
      This is where it gets interesting. When you're iterating on a file -- read, edit, read again -- Claude already has the old version in context. With diff mode enabled, read-once shows only what changed instead of the full file.
      ...
      If the diff is too large (>40 lines by default), it falls back to a full re-read. You're not losing anything.
      ...
      Claude Code compacts the context window during long sessions, dropping older content. A file read 30 minutes ago might no longer be in working context. read-once handles this with a TTL: cache entries expire after 20 minutes (configurable via`READ_ONCE_TTL`). After expiry, re-reads are allowed.
      ...
      There's no way to detect compaction events from a hook, so a time-based heuristic is the best available approach.
      ...
      It works alongside RTK (for Bash output deduplication) and Context-Mode (for large output processing). read-once operates on the Read tool layer, so there's no conflict with other optimization tools.
      ...
      | Variable | Default | What it does |
      | --- | --- | --- |
      | `READ_ONCE_TTL` | `1200` | Seconds before cache expires (compaction safety) |
      | `READ_ONCE_DIFF` | `0` | Set to`1` for diff-only mode on changed files |
      | `READ_ONCE_DIFF_MAX` | `40` | Max diff lines before falling back to full re-read |
      | `READ_ONCE_DISABLED` | `0` | Set to`1` to disable entirely |

      ---

      Title: Claude Code Context Window | Claude Code Guide
      URL: https://claudecodeguide.dev/docs/foundations/context-window
      Published: N/A
      Author: N/A
      Highlights:
      Think of it like working memory. Everything in your current session (your messages, Claude's responses, file contents it has read, command outputs) occupies space in this window. When it fills up, earlier parts of the conversation start getting pushed out. Claude Code begins forgetting things you said or files it already read.
      ...
      - Responses get less accurate. Claude Code "forgets" constraints you mentioned earlier or re-reads files it already looked at.
      - It loses track of decisions. You agreed on an approach 20 messages ago. Now it suggests something different.
      - Responses get shorter or cut off. The model is running out of room to generate.
      - You see context length warnings. Claude Code will tell you when things are getting large.
      ...
      When context gets large, run`/compact`. This summarizes the entire conversation so far and starts fresh with just the summary. Think of it as taking notes from a meeting, then starting a new meeting with only the notes.
      ...
      What it loses: Exact file contents, detailed error messages, nuanced back-and-forth reasoning. If you need to reference specific details after compaction, Claude Code will re-read the relevant files.
      ...
      Don't let a single session accumulate context from five different tasks. Finish a task, start a new session for the next one. This is the single most effective strategy and most people never do it.
      ...
      Instead of one 3-hour session, work in focused 30-60 minute blocks. End each block with a handoff (what was done, what's next). Start a fresh session that reads only the handoff. Fresh context every time.
      ...
      instead of pasting them
      ...
      Let Claude Code read files on demand. Don't paste file contents into the chat. When you paste, that content stays in the conversation forever. When Claude Code reads a file with its tools, it's more efficient.
      ...
      Sub-agents (via the Task tool) get their own context window. If you need Claude Code to explore a large codebase or research something, offload it to a sub-agent. The main conversation only gets the summary back. This is one wall in a broader approach: see the subagent context-isolation pattern for how isolation scales across memory, agents, and cron state.
      ...
      Vague prompts cause Claude Code to read more files searching for what you mean. "Fix the auth bug" makes it read every auth-related file. "Fix the token refresh logic in`src/auth/refresh.ts` where expired tokens are not being caught" sends it straight to the right file.
      ...
      Even within one session, compact between distinct tasks. Finished adding tests? Compact. Now moving to a different feature? Clean slate, prior work summarized.
      ...
      This is the power move for heavy users. Instead of fighting the context window, work with it:
      ...
      1. Work in a focused block (30-60 minutes, one task)
      2. End the session with a handoff: what was done, what's next, key decisions
      3. Start a new session. Load only the handoff and relevant`CLAUDE.md`
      4. Fresh 1M tokens, zero wasted context
      ...
      You lose nothing because the handoff captures everything that matters. You gain a full context window for the next task. Over a full day of work, this approach is dramatically more effective than one long session that degrades as context fills up.

      ---

      Title: 7. Context Management
      URL: https://notes.tsukino.dev/99-%E5%B7%A5%E5%85%B7%E4%B8%8E%E5%8F%82%E8%80%83/repos/claude-code-from-scratch/en/docs/07-context
      Published: N/A
      Author: N/A
      Highlights:
      System/user context is computed once per session and memoized: git status (5 commands executed in parallel), CLAUDE.md files (traversing the directory tree upward from CWD), current date, etc. The injection order is deliberate — system context is placed after the system prompt, and user context is prepended to the message array, ensuring the most stable content comes first to maximize cache hits.
      ...
      handling thinking blocks, merging
      ...
      `tool_use`/
      ...
      tool_result` pairing, etc
      ...
      Level 1: Tool Result Budget Trimming — Tools declare`maxResultSizeChars`(default 50K chars); when exceeded, results are persisted to disk, and only a compact reference with a 2KB preview is kept in context. The choice of persistence over truncation is deliberate: no data is lost, and the model can retrieve the full file at any time using the Read tool.
      ...
      Level 4: Context Collapse — Projection-based folding, with the key characteristic of not modifying original messages, only creating a folded view. Analogous to a database View: the underlying table doesn’t change, but queries see filtered results. When enabled, it suppresses Autocompact to prevent the two from competing.
      ...
      Difference from Claude Code: Claude Code persists to disk, and the model can retrieve full content later with the Read tool. We now also implement persistence — see persistLargeResult below. The two tiers work together: persistLargeResult first intercepts results >30KB and saves them to disk, then truncateResult handles content that passed the first tier but still exceeds 50K.
      ...
      When a tool returns a result exceeding 30KB, the full content is written to disk, and only a preview and file path are kept in context. The model can later use`read_file` to retrieve the full output on demand.
      ...
      ${lines.length} lines
      ...
      - 30KB threshold is lower than truncateResult’s 50K limit: Intercepts large results before truncation occurs, avoiding irreversible information loss. If a result is 80KB, persistLargeResult saves the full content to disk and returns a preview, rather than letting truncateResult permanently discard the middle portion.
      ...
      - 200-line preview: Gives the model enough context to decide whether it needs to read the full output. In most cases, the first 200 lines already contain the key information (beginning of file listings, first few matches of search results, main content of command output).
      ...
      - Recoverable vs irrecoverable: This is the fundamental difference from truncateResult. truncateResult is irreversible — truncated content is gone forever. persistLargeResult saves data to`~/.mini-claude/tool-results/{timestamp}-{toolName}.txt`, and the model can retrieve it at any time with`read_file`.
      ...
      - Alignment with Claude Code: This design directly corresponds to Claude Code’s Level 1 strategy (persist to disk, keep only references in context). The difference is that Claude Code uses a 2KB preview while we use 200 lines — same concept, simplified implementation.
      ...
      Snip strategy (triggered when utilization > 60%):
      ...
      - Same file read multiple times by`read_file`→ keep only the latest, snip older ones
      - More than 3 search results of the same type → snip the oldest
      - The 3 most recent`tool_result` entries are always preserved
      ...
      Key point: Only the`tool_result` content is cleared; the`tool_use` block is kept intact. The model can still see “I previously read /src/main.ts” — it just can’t see the content anymore. If needed, it can call`read_file` again. Preserving metadata matters more than preserving data.
      ...
      turn boundary — after the user
      ...
      is pushed into
      ...
      message array and before
      ...
      `compactAnth
      ...
      `compactOpenAI
      ...
      the last message is a
      ...
      text message:
      ...
      `slice(0
      ...
      when building the summarization request and re
      ...
      mid-tool-loop,
      ...
      last message will be a`tool_result
      ...
      Anthropic) or a`
      ...
      `-role message
      ...
      OpenAI); slicing it off
      ...
      s`tool_use
      ...
      Key differences from Claude Code: Claude Code uses a two-stage “analyze-summarize” prompt for higher quality summaries, restores the 5 most recent files and active skills after compression, and has a circuit breaker to prevent infinite loops. Ours is a simplified version — single-paragraph summary, no restoration mechanism, no circuit breaker.
      ...
      Tiers 1-3 run before every API call (zero API cost). Tier 4 runs at the turn boundary — after the user message is pushed into the array and before the`while` loop starts. Do not place Tier 4 at the end of the tool loop: at that point the last message is`{role: "user", content: [tool_result, ...]}`, and`compactAnthropic`’s`slice(0, -1)` would sever its pairing with the preceding`assistant` message’s`tool_use`, causing the Anthropic API to reject the summarize call with “tool_use ids were found without tool_result blocks immediately after”.`lastInputTokenCount` is still usable in the new location — it reflects the state of the previous turn’s final API call, which is enough to decide whether to trigger. The intra-pipeline order is also intentional: Budget compresses large results first, making Snip’s deduplication judgments more accurate, and Microcompact performs indiscriminate cleanup last when the time condition is met.

      ---

      Title: How Claude remembers your project
      URL: https://code.claude.com/docs/en/memory.md
      Published: N/A
      Author: N/A
      Highlights:
      Claude Code has two complementary memory systems. Both are loaded at the start of every conversation. Claude treats them as context, not enforced configuration. To block an action regardless of what Claude decides, use a PreToolUse hook instead. The more specific and concise your instructions, the more consistently Claude follows them.
      ...
      CLAUDE.md files are loaded into the context window at the start of every session, consuming tokens alongside your conversation. The context window visualization shows where CLAUDE.md loads relative to the rest of the startup context. Because they're context rather than enforced configuration, how you write instructions affects how reliably Claude follows them. Specific, concise, well-structured instructions work best.
      ...
      Size: target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce adherence. If your instructions are growing large, use path-scoped rules so instructions load only when Claude works with matching files. You can also split content into imports for organization, though imported files still load and enter the context window at launch.
      ...
      Claude Code reads CLAUDE.md files by walking up the directory tree from your current working directory, checking each directory along the way for `CLAUDE.md` and `CLAUDE.local.md` files. This means if you run Claude Code in `foo/bar/`, it loads instructions from `foo/bar/CLAUDE.md`, `foo/CLAUDE.md`, and any `CLAUDE.local.md` files alongside them.
      ...
      All discovered files are concatenated into context rather than overriding each other. Across the directory tree, content is ordered from the filesystem root down to your working directory. For the `foo/bar/` example, `foo/CLAUDE.md` appears in context before `foo/bar/CLAUDE.md`, so instructions closer to where you launched Claude are read last. Within each directory, `CLAUDE.local.md` is appended after `CLAUDE.md`, so your personal notes are the last thing Claude reads at that level.
      ...
      Claude also discovers `CLAUDE.md` and `CLAUDE.local.md` files in subdirectories under your current working directory. Instead of loading them at launch, they are included when Claude reads files in those subdirectories.
      ...
      For larger projects, you can organize instructions into multiple files using the `.claude/rules/` directory. This keeps instructions modular and easier for teams to maintain. Rules can also be scoped to specific file paths, so they only load into context when Claude works with matching files, reducing noise and saving context space.
      ...
      Rules load into context every session or when matching files are opened. For task-specific instructions that don't need to be in context all the time, use skills instead, which only load when you invoke them or when Claude determines they're relevant to your prompt.
      ...
      The first 200 lines of `MEMORY.md`, or the first 25KB, whichever comes first, are loaded at the start of every conversation. Content beyond that threshold is not loaded at session start. Claude keeps `MEMORY.md` concise by moving detailed notes into separate topic files.
      ...
      The `/memory` command lists your CLAUDE.md, CLAUDE.local.md, and other memory file locations across user and project scopes, lets you toggle auto memory on or off, and provides an option to open the auto memory folder. Select any file to open it in your editor. To check which files actually loaded into the current session, run `/context`.
      ...
      - Run `/context` to verify your CLAUDE.md and CLAUDE.local.md files loaded. If a file is missing from the breakdown, Claude can't see it. Use `/memory` to open and edit the files.
      - Check that the relevant CLAUDE.md is in a location that gets loaded for your session (see Choose where to put CLAUDE.md files).
      ...
      - Make instructions more specific. "Use 2-space indentation" works better than "format code nicely."
      - Look for conflicting instructions across CLAUDE.md files. If two files give different guidance for the same behavior, Claude may pick one arbitrarily.
      ...
      Files over 200 lines consume more context and may reduce adherence. Use path-scoped rules to load instructions only when Claude works with matching files, or trim content that isn't needed in every session. Splitting into `@path` imports helps organization but doesn't reduce context, since imported files load at launch.
      ...
      Project-root CLAUDE.md survives compaction: after `/compact`, Claude re-reads it from disk and re-injects it into the session. Nested CLAUDE.md files in subdirectories are not re-injected automatically; they reload the next time Claude reads a file in that subdirectory.
      ...
      instruction disappeared after compaction,
      ...
      was either given only in conversation or lives in a nested CLAUDE.md that
      ...
      yet. Add conversation-
      ...
      instructions to CL
      ...
      to make them persist.
      ...
      survives compaction for

      ---

      Title: Claude Code Memory System — Implementation Details | Claude Code Haha
      URL: https://claudecode-haha.relakkesyang.org/en/memory/02-implementation
      Published: N/A
      Author: N/A
      Highlights:
      This prevents duplicate saves: when the main agent has already written a memory, the background extraction is skipped.
      ...
      Key design: Single-pass (read-then-sort) to avoid double stat system calls.
      ...
      ```
      async function scanMemoryFiles(memoryDir, signal): Promise<MemoryHeader[]> {
        const entries = await readdir(memoryDir, { recursive: true })
        const mdFiles = entries.filter(f => f.endsWith('.md') && basename(f) !== 'MEMORY.md')

        // Read all files' frontmatter in parallel (first 30 lines)
        const headerResults = await Promise.allSettled(
          mdFiles.map(async (relativePath) => {
            const { content, mtimeMs } = await readFileInRange(filePath, 0, 30)
            const { frontmatter } = parseFrontmatter(content)
            return { filename, filePath, mtimeMs, description, type }
          })
        )

        // Filter successful results, sort by time descending, take first 200
        return fulfilled.sort((a, b) => b.mtimeMs - a.mtimeMs).slice(0, 200)
      }
      ```
```
/answer a12e5b9d-d9b8-4e7f-8f74-b4f41f561b93 allow
ReasoningToken: 346
Sehr ergiebige Recherche – es gibt tatsächlich schon etablierte Muster dafür, und sie bestätigen deine Beobachtungen sowie meine Hypothese zum Context-Loss. Hier die relevantesten Ansätze:

## 1. Bestätigung: Context-Compaction ist der Kernfeind
Aus Claude Codes eigener Doku: *"Claude Code compacts the context window during long sessions, dropping older content. A file read 30 minutes ago might no longer be in working context."* Und noch präziser aus einer Analyse zur internen Context-Management-Architektur:

> "Only the `tool_result` content is cleared; the `tool_use` block is kept intact. The model can still see 'I previously read /src/main.ts' — it just can't see the content anymore. **Preserving metadata matters more than preserving data.**"

Das erklärt exakt dein Problem: Dein Meta-Hinweis kommt vermutlich zu spät oder zu schwach an, weil der Agent bereits *weiß*, dass er gelesen hat, aber den Inhalt nicht mehr sehen kann – und dann rational nach einem Workaround (Bash `cat`) sucht.

## 2. Konkrete, produktiv eingesetzte Tools

**`read-once`** (Claude Code Hook, PreToolUse):
- Session-scoped Cache nach Pfad + mtime (nicht Content-Hash, aber ähnlich)
- **Warn-Modus** (Standard): Read wird durchgelassen, aber mit Advisory-Message versehen
- **Deny-Modus**: Re-Read wird nur geblockt, wenn die Datei *innerhalb derselben Session* unverändert bereits gelesen wurde
- **Diff-Modus**: Bei geänderten Dateien nur den Diff zurückgeben statt Volltext (bei 200-Zeilen-Datei mit 3 geänderten Zeilen: ~30 statt ~2000 Tokens)
- **Wichtigster Punkt**: `PostCompact`-Hook leert den Cache sofort bei Compaction, zusätzlich TTL-Fallback (20 Min) – genau um dein Problem (Content ist weg, aber "gelesen"-Meta bleibt fälschlich bestehen) zu vermeiden

**TriSeek Memo**:
- Prüft Datei-Hash bei jedem Re-Read
- Gibt eine **`recommendation`** zurück statt nur einer Notiz:
  - `skip_reread` → Datei unverändert, auf Conversation History vertrauen
  - `reread_with_diff` → kleine Änderung (<10% Size-Delta), Diff erwartbar
  - `reread` → signifikant geändert oder nie gelesen
- Das Framing als **Empfehlung mit klarer Handlungsanweisung** statt reiner Meta-Info scheint wirksamer zu sein als ein Statusvermerk

## 3. Wie Claude Code intern mit dem Problem umgeht
- **Tool Result Budget Trimming**: Ergebnisse >50K Zeichen werden auf Platte persistiert, im Kontext bleibt nur ein Preview + Referenzpfad. Modell kann bei Bedarf gezielt nachladen.
- **Nach Compaction**: Die 5 zuletzt gelesenen Dateien werden automatisch wiederhergestellt/re-injiziert – das ist ein Schlüsselmechanismus, den du übernehmen könntest.
- **Circuit Breaker** gegen Endlosschleifen beim Summarization-Prozess.

## Ableitung für dein MCP-Design

Basierend darauf würde ich folgende Kombination empfehlen:

1. **Recommendation-Feld statt Warnhinweis**: Dein `<file_registry>` sollte pro Datei ein klares, handlungsleitendes Feld liefern (`skip_reread` / `reread_with_diff` / `reread`), keine reine Beobachtung. Modelle folgen Handlungsanweisungen zuverlässiger als Feststellungen.

2. **Diff-first bei Re-Read-Anfragen**: Wenn eine bereits gelesene, unveränderte Datei erneut angefragt wird, liefere standardmäßig nur eine kompakte Bestätigung + Hash/mtime – **niemals eine Leerantwort**, aber auch nicht automatisch den Volltext. Bei geänderten Dateien: nur den Diff.

3. **Content-Hash statt nur Read-Flag**: Tracke nicht nur "gelesen ja/nein", sondern Hash + mtime, damit du zwischen "unverändert" und "geändert seit Read" unterscheiden kannst – das ist differenzierter als reine Session-Flags.

4. **Kompensiere Compaction aktiv**: Das ist vermutlich dein größter Hebel. Wenn du erkennen kannst (oder heuristisch vermuten), dass Compaction stattgefunden hat (z.B. via TTL, Turn-Zähler, oder Hook-Integration falls dein Framework das unterstützt), solltest du bereits gelesene Dateien **aktiv wieder in den Kontext einspeisen** (z.B. die letzten 3-5 gelesenen Dateien kompakt zusammenfassen und re-injizieren), statt nur zu hoffen, dass der Agent sich an sie "erinnert".

5. **Bash-Bypass technisch unterbinden**, falls möglich – sonst bleibt jede Kontext-Strategie eine Umgehungsmöglichkeit offen.

Willst du, dass ich dir ein konkretes Schema für die `file_registry`-Response-Struktur (Felder, Format, Platzierung) entwerfe, angelehnt an diese Patterns?
In welcher MCP-Struktur gebe ich einen Hint oder ein advisory am besten zurück?
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=2d8eb4fa-0643-45dd-b24c-5a18e4899307 | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: f2d581f2-9831-4d51-b202-a726e9f04565
toolName: web-search-exa
arguments:
  query: MCP protocol tool result structuredContent annotations _meta field best practice hints
```
/answer f2d581f2-9831-4d51-b202-a726e9f04565 allow
ReasoningToken: 477
Control Request:
```yaml
id: b26eff25-c055-411b-801c-8466fd6c7482
result:
  structuredContent:
    content: |-
      Title: Tools - Model Context Protocol
      URL: https://modelcontextprotocol.io/specification/2025-11-25/server/tools
      Published: 2025-11-25T00:00:00.000Z
      Author: N/A
      Highlights:
      present
      - `annotations`: Optional properties describing tool behavior
      - `execution`: Optional object describing execution-related properties
      - `
      ...
      Support`: Indicates
      ...
      this tool supports task-augmented
      ...
      . Values: `"forbidden"` (default), `"optional"`, or `"required
      ...
      ### Tool Result
      ...
      Tool results may contain structured or unstructured content.

      Unstructured content is returned in the `content` field of a result, and can contain multiple content items of different types:

      All content types (text, image, audio, resource links, and embedded resources)
      support optional
      annotations that
      provide metadata about audience, priority, and modification times. This is the
      same annotation format used by resources and prompts.
      ...
      #### Structured Content
      ...
      Structured content is returned as a JSON object in the `structuredContent` field of a result.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      #### Output Schema
      ...
      Tools may also provide an output schema for validation of structured results.
      If an output schema is provided:
      ...
      - Servers MUST provide structured results that conform to this schema.
      - Clients SHOULD validate structured results against this schema.
      ...
      Example valid response for this tool:
      ...
      ```json
      {
        "jsonrpc": "2.0",
        "id": 5,
        "result": {
          "content": [
            {
              "type": "text",
              "text": "{\"temperature\": 22.5, \"conditions\": \"Partly cloudy\", \"humidity\": 65}"
            }
          ],
          "structuredContent": {
            "temperature": 22.5,
            "conditions": "Partly cloudy",
            "humidity": 65
          }
        }
      }
      ...
      Providing an output schema helps clients and LLMs understand and properly handle structured tool outputs by:

      ---

      Title: SEP-1624: Clarify `structuredContent` vs `content` Usage Guidance · Issue #1624 · modelcontextprotocol/modelcontextprotocol
      URL: https://github.com/modelcontextprotocol/modelcontextprotocol/issues/1624
      Published: 2025-10-08T04:37:26.000Z
      Author: krubenok
      Highlights:
      This SEP proposes clarifying the specification around `structuredContent` and `content` fields in `CallToolResult` to address inconsistent client and server implementations. The proposal focuses on **non-breaking clarifications** to the existing spec language to better communicate the intended design and usage patterns for both fields.
      ...
      ```markdown
      #### Structured Content

      **Structured** content is returned as a JSON object in the `structuredContent` field of a result.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ```
      ...
      ```markdown
      #### Structured Content

      **Structured** content is returned as a JSON object in the `structuredContent` field of a result.
      ...
      The `structuredContent` and `content` fields serve different use cases:

      - **`content`**: Model-oriented output optimized for readability and token efficiency. Preferred for conversational agents and direct model prompting. Does not require strict schematisation. A plain text or markdown representation of the output for example.
      - **`structuredContent`**: Machine-oriented output for programmatic tool use, code generation, type-safe orchestration, and strict schema validation.
      ...
      **Server Requirements**:

      - When both fields are present, they MUST be semantically equivalent (same information, different presentation)
        - In some cases, an explicit field in `structuredContent` can be semantically present in `content` without specific mention. For example if expected fields are returned and no error is mentioned, a `structuredContent` response that contains a `"status": "success",` object is evident without the status being verbatim mentioned as successful in `content`.
      - For backwards compatibility, a tool that returns structured content SHOULD also return a response in `content`.
      ...
      **Client Guidance**:

      - Clients SHOULD choose the appropriate field to expose to the model based on their use case:
        - **Conversational/Agent UX**: Prefer `content` for lower token cost and readability (by both humans and models).
        - **Programmatic/Code Mode**: Prefer `structuredContent` for type safety and schema validation
      ...
      - Clients SHOULD NOT forward both fields verbatim to models as semantically distinct inputs.
      - Clients that use `content` MUST still perform `outputSchema` validation against `structuredContent` when present
      ```
      ...
      ````markdown
      ### Designing Tools with Complementary Output Formats

      Tool developers should design outputs where `content` and `structuredContent` serve complementary roles, enabling both conversational and programmatic use cases:

      - **`content`** provides a human/model-oriented representation:

        - Natural language content, context, and explanations
        - Optimized for readability and token efficiency
        - Includes suggestions, interpretations, or additional guidance
        - Suitable for direct consumption by conversational agents or end-users

      - **`structuredContent`** provides a machine-oriented representation:
        - Strict schema validation via `outputSchema`
        - Type safety for programmatic tool composition
        - Consistent structure for code generation and scripting
        - Enables reliable data extraction and transformation
      ...
      **Design patterns for complementary outputs:**

      - Define an `outputSchema` that captures all data fields, ensuring that the `text` field from `content` is present as a substring of `structuredContent`.
      - Provide semantically equivalent `content` with enhanced readability
      - Format `content` with context that helps models use the information effectively
      ...
      ```typescript
      /**
       * An optional JSON Schema object defining the structure of the tool's output returned in
       * the structuredContent field of a CallToolResult.
       *
       * When outputSchema is provided:
       * - Servers MUST populate structuredContent with data conforming to this schema
       * - Clients SHOULD validate structuredContent against this schema
       * - Servers SHOULD also provide a human-readable representation in content
       *
       * Use outputSchema for programmatic tool use, code generation, and type-safe integrations.
       * For conversational agents, consider whether unstructured content provides better model performance.
       */
      outputSchema?: {
        type: "object";
        properties?: { [key: string]: object };
        required?: string[];
      };
      ```
      ...
      > Open Question: if we continue to expect that `content` is present in `structuredContent` as a substring, rather than the stringified JSON we have today, we will have semantic duplication in `structuredContent`. By the letter of the proposed spec this is not an issue as codeful use of `structuredContent` can ignore the embedded `content` field. In practice with clients currently preferring `structuredContent` as model input, this duplication could lead to increased token usage and duplicate information being provided.
      ...
      - [OpenAI Apps SDK](https://developers.openai.com/apps-sdk/build/mcp-server#structure-the-data-your-tool-returns) implements this pattern as of 10/7 and also leverages `_meta` for additional data.
         - Would require breaking changes
         - Better addressed in future SEP after more real-world usage with more explicit side channels or audience hints (i.e. [model|host|user])
      ...
      > A couple thoughts:
      >
      > - I'm supportive of helping to clarify this! I think as programmatic MCP (aka MCP as code, programmatic function calling, programmatic tool calling, Code Mode) gains more traction it'll be valuable to encourage adoption of `structuedContent`
      ...
      > - In general, I don't think `structuredContent` should include `content` - agree this leads to duplication, especially if all the info is already captured in the response
      ...
      > - I actually think having `content` == `JSON.stringify(structuredContent)` is pretty reasonable most of the time. It's often not actually that many more tokens, and on presentation I think this should be down to the model (if needed for the user) rather than the server tool response. Especially as we move to a world with agents doing longer tasks, I think individual tool results will be less relevant to humans and programmatic MCP will be even more important, so leaning into structuredContent seems reasonable.
      >
      > - And yes there's duplication here, but tbh the cost is likely so low that it doesn't matter. If we do want to fix this we could add a capability to clients that indicates they definitely support `structuredContent` so the server doesn't need to send `content` as well, but tbh I think more features/combinations in MCP is probably the wrong direction.
      ...
      > Thanks both for the input! I think we're caught between a few points here, trying to identify which compromises make sense.
      >
      > > In general, I don't think structuredContent should include content - agree this leads to duplication, especially if all the info is already captured in the response
      >
      > Yes, in general I agree, but it seemed like a simply way to not require any changes of the existing validation expectations driven by the Inspector for example expecting that `content` is a JSON.stringify'ed substring of `structuredContent`. If we're willing to "break" that (which I don't think is actually a breaking change?) then I would agree we should ideally not duplicate it.
      >
      > > I actually think having content == JSON.stringify(structuredContent) is pretty reasonable most of the time. It's often not actually that many more tokens, and on presentation I think this should be down to the model (if needed for the user) rather than the server tool response. Especially as we move to a world with agents doing longer tasks, I think individual tool results will be less relevant to humans and programmatic MCP will be even more important, so leaning into structuredContent seems reasonable.
      ...
      > > Why?
      ...
      **ochafik** mentioned this in issue [#380: Properly clarify content model visibility - content vs structuredContent vs _meta](https://github.com/modelcontextprotocol/ext-apps/issues/380) · Jan 28, 2026 at 4:54pm

      ---

      Title: MCP App Tool Results: content, structuredContent, and _meta - sunpeak
      URL: https://sunpeak.ai/blogs/mcp-app-tool-results-content-structuredcontent-meta/
      Published: 2026-06-14T00:00:00.000Z
      Author: N/A
      Highlights:
      MCP App tool results carry separate data lanes for the model, the rendered app, and UI-only metadata.
      ...
      The fix is to treat`content`,`structuredContent`, and`_meta` as separate contracts.
      ...
      TL;DR: Put a concise model-readable summary in`content`. Put typed render data in`structuredContent`. Put UI-only metadata in`_meta` when your target host supports it. Declare`outputSchema` for structured results, keep`structuredContent` safe for model context, and test each lane before debugging React.
      ...
      The MCP tools specification defines tool results as content plus optional structured content. MCP Apps add a rendered resource on top of that result, but the base fields still matter.
      ...
      | Field | Main reader | Use it for | Avoid |
      | --- | --- | --- | --- |
      | `content` | Model, user, non-app clients | Short text summary, status, citations, compact fallback | Full UI payloads, hidden IDs, private UI state |
      | `structuredContent` | App resource, often model context | Typed JSON rows, cards, charts, form state, public result data | Secrets, huge hidden payloads, UI-only hints |
      | `_meta` | App resource when supported | Cursors, cache keys, view hints, non-secret internal UI data | Anything required for the answer, long-lived secrets |
      ...
      The split is simple in principle.`content` explains the result.`structuredContent` carries the data.`_meta` helps the UI operate without making every helper value part of the model context.
      ...
      ```
      return {
        content: [
          {
            type: 'text',
            text: 'Displayed 12 invoices for April 2026.',
          },
        ],
        structuredContent: {
          period: '2026-04',
          invoices: invoices.map((invoice) => ({
            id: invoice.publicId,
            customer: invoice.customerName,
            totalCents: invoice.totalCents,
            status: invoice.status,
            dueDate: invoice.dueDate,
          })),
        },
        _meta: {
          nextCursor,
          viewId,
        },
      };
      ...
      If a tool returns`structuredContent`, declare an`outputSchema`.

      OpenAI’s Apps SDK reference says to declare`outputSchema` for tools that return`structuredContent`, and the MCP tools spec says structured results must conform when an output schema is provided.
      ...
      `content` is not a trash can for the full UI payload. It should be the shortest useful answer a non-app client can show and the shortest useful context the model can use later.
      ...
      The MCP spec says servers that return structured content should also return serialized JSON in a text block for backwards compatibility. In practice, MCP App developers often use a concise text summary because it gives non-app clients and the model useful context without duplicating the full render payload. Test this against the hosts and clients you plan to support.
      ...
      ## What structuredContent Should Contain
      ...
      `structuredContent` should contain the stable public data the resource needs to render.
      ...
      - Rows for a table
      - Cards for a carousel
      - Chart series and labels
      - Form defaults
      - Validation results
      - Public IDs the user can see
      - Status values the model may need for follow-up turns
      ...
      Bad candidates:
      ...
      - OAuth tokens
      - Refresh tokens
      - Raw session cookies
      - Private notes the model should not read
      - Huge raw API responses
      - UI-only cursors that the model should ignore
      - Cache keys with internal structure
      ...
      OpenAI’s MCP server guide describes ChatGPT’s model as reading`structuredContent` to narrate what happened. That is the safest default for cross-host design too: assume`structuredContent` can become model context.
      ...
      ## What _meta Should Contain
      ...
      Use`_meta` for UI-only values, but treat it as host-mediated data, not as a secure vault.
      ...
      Good`_meta`:
      ...
      If a value would cause harm if exposed in a browser session, do not send it in a tool result. The app runs in an iframe, but it is still client-side code. Use short-lived, scoped, revocable tokens only when a UI truly needs direct access, and prefer server tools for sensitive operations.
      ...
      ChatGPT supports the standard MCP Apps bridge: JSON-RPC messages over`postMessage`, including`ui/notifications/tool-result`. The OpenAI Apps SDK reference documents that this notification includes`structuredContent`,`content`, and`_meta`.
      ...
      | ChatGPT field | Maps to | Notes |
      | --- | --- | --- |
      | `window.openai.toolOutput` | `structuredContent` | Use for the main render payload in ChatGPT-specific code |
      | `window.openai.toolResponseMetadata` | metadata for the full MCP result | Includes widget-only metadata in current ChatGPT docs |
      | `window.openai.toolInput` | tool input | May arrive after approval for gated tools |
      | `window.openai.widgetState` | persisted widget state | Not the same thing as tool output |
      ...
      - Put the connector answer summary in`content`.
      - Put renderable rows, charts, forms, or cards in`structuredContent`.
      - Put UI-only helper values in`_meta` only when your target host path supports them.
      ...
      1. Call the tool directly through an MCP inspector or integration test.
      2. Confirm the tool returns`content` and`structuredContent` at the top level of the result.
      3. Confirm`structuredContent` matches`outputSchema`.
      4. Confirm the UI-capable tool descriptor points at the resource with`_meta.ui.resourceUri`.
      5. Confirm the resource reads the right field, such as`output` from`useToolData`.
      6. If you use ChatGPT-specific code, log whether`window.openai.toolOutput` and`window.openai.toolResponseMetadata` are present.
      ...
      ## A Practical Rule
      ...
      When you are unsure where a value belongs, ask who needs to read it:
      ...
      - The model or non-app client needs it:`content`.
      - The app UI needs it and it is safe for model context:`structuredContent`.
      - The app UI needs it and the model should not reason about it:`_meta`.
      - Only the server needs it: keep it on the server.
      - It is a secret: do not put it in any tool result.
      ...
      content is the normal MCP tool result field for human-readable or media content, usually text the model and user can understand. structuredContent is JSON data that should match the tool outputSchema when one is declared. In MCP Apps, the rendered resource commonly uses structuredContent as its typed render payload, while content gives the model a concise explanation of what happened.
      ...
      What should go in _meta in an MCP App tool result?
      ...
      _meta is for UI-only tool result data when the host supports delivering it to the app resource. Use it for cursors, cache keys, transient view hints, non-secret internal IDs, or data the UI needs but the model should not reason about. Do not put secrets, refresh tokens, customer-private notes, or anything required for the model answer in _meta.
      ...
      Yes. OpenAI documents ChatGPT as using structuredContent for model narration, and window.openai.toolOutput maps to structuredContent in the widget runtime. Treat structuredContent as model-visible unless the exact host you target documents otherwise. Keep it concise, typed, and safe for follow-up reasoning.
      ...
      Usually yes. The MCP tools specification says servers that return structured content should also return serialized JSON in a TextContent block for backwards compatibility. For MCP Apps, you can often return a short text summary instead of duplicating the full JSON payload, as long as the host and client behavior you target accept that shape.
      ...
      Should I put pagination cursors in structuredContent or _meta?
      ...
      Put cursors in structuredContent when the model needs to discuss or choose the next page. Put cursors in _meta when only the UI needs them for a Next button or background fetch. If the cursor grants access to data, treat it like a short-lived capability and scope it tightly.
      ...
      How do I test content, structuredContent, and _meta in an MCP App?

      Call the tool through the MCP layer and assert each field separately. Verify content is concise, structuredContent matches the declared outputSchema, and _meta contains only UI-only values. Then render the resource in a local inspector and assert the UI uses structuredContent and _meta without copying private values into model-visible text.

      ---

      Title: examples/snippets/servers/direct_call_tool_result.py
      URL: https://github.com/modelcontextprotocol/python-sdk/blob/v1.x/examples/snippets/servers/direct_call_tool_result.py
      Published: N/A
      Author: N/A
      Highlights:
      @mcp.tool()
      def advanced_tool() -> CallToolResult:
       """Return CallToolResult directly for full control including _meta field."""
       return CallToolResult(
       content=[TextContent(type="text", text="Response visible to the model")],
       _meta={"hidden": "data for client applications only"},
       )
      ...
      @mcp.tool()
      def validated_tool() -> Annotated[CallToolResult, ValidationModel]:
       """Return CallToolResult with structured output validation."""
       return CallToolResult(
       content=[TextContent(type="text", text="Validated response")],
       structuredContent={"status": "success", "data": {"result": 42}},
       _meta={"internal": "metadata"},
       )

      ---

      Title: Discussion: Dynamic next-action hints in tool results (HATEOAS-inspired) · modelcontextprotocol modelcontextprotocol · Discussion #2569 · GitHub
      URL: https://github.com/modelcontextprotocol/modelcontextprotocol/discussions/2569
      Published: N/A
      Author: modelcontextprotocol
      Highlights:
      An optional`_meta` field in tool results with dynamically computed next-action hints:
      ...
      ```
      {
        "content": [{"type": "text", "text": "Order #12345 created. Status: pending."}],
        "_meta": {
          "availableActions": [
            {"tool": "orders.getStatus", "reason": "Check order progress"},
            {"tool": "orders.cancel", "reason": "Cancel while still pending"},
            {"tool": "orders.addItem", "reason": "Add more items before processing"}
          ]
        }
      }
      ```

      After the order ships, the same tool returns different hints:
      ...
      ```
      {
        "_meta": {
          "availableActions": [
            {"tool": "orders.getStatus", "reason": "Track shipment"},
            {"tool": "orders.requestReturn", "reason": "Initiate return"}
          ]
        }
      }
      ```
      ...
      `orders.cancel` is gone because the state no longer allows it.
      ...
      - Optional: servers MAY include hints, clients MAY use them. No breaking changes.
      - Dynamic: computed per-result based on user permissions, entity state, and service availability.
      - Lightweight: just tool names and short reasons. No full tool definitions (those are already in`tools/list`).
      - Advisory: hints for the LLM, not hard constraints. The LLM can still call any tool it has access to.
      - Uses existing`_meta`: no new top-level fields needed. Follows the MCP convention for extension metadata.
      ...
      - Should`availableActions` include parameter hints (e.g., pre-filled IDs from the current result)?
      - Should there be a way to indicate "required next step" vs. "optional follow-up"?
      - How should clients surface this to the LLM? Inject into system prompt? Append to tool result text?
      - Token cost trade-off: is the overhead justified by improved chaining accuracy?
      ...
      Yes. The entity is already in scope when hints are computed, so pre-filling is zero-cost for the server and eliminates an ambiguity source for the LLM:
      ...
      Without`params`, in multi-entity contexts the LLM has to infer which ID maps to which tool argument - a silent failure mode, not a loud one.
      ...
      A`priority` field is sufficient:
      ...
      ```
      type AvailableAction = {
        tool: string;
        reason: string;
        params?: Record<string, unknown>;
        priority?: 'required' | 'suggested' | 'optional';
      };
      ```
      ...
      `required` has a specific semantic: the entity is in a transitional state and leaving it without acting is unsafe (e.g.,`payment.confirm` after`payment.initiate`- skipping it leaves a dangling authorization). That's distinct from`suggested`.
      ...
      How should clients surface hints to the LLM?
      ...
      Appending to result content, not injecting into the system prompt. System prompt injection accumulates across turns — hints from turn 3 are still visible in turn 7 even after the entity has transitioned. Scoping to the result content keeps hints fresh and turn-local.
      ...
      The overhead is small. The bigger unaddressed risk is staleness: hints are valid at emission time, but a concurrent mutation can invalidate them before the LLM acts. A`stateHash` on the result lets the server detect and reject stale-hint calls with a fresh result instead of a silent wrong-state execution:
      ...
      ```
      {
        "_meta": {
          "stateHash": "e3a1f9",
          "availableActions": [...]
        }
      }
      ```
      ...
      Tool handler checks`stateHash` before executing. Stale → 409 + fresh result. Without this, the LLM retries with correct state anyway, but only after a destructive or incorrect action has already been attempted.
      ...
      Thanks sharing, very interseting. I find the`stateHash` idea very intersting and actually must even be required for safe execution.

      ---

      Title: MCP Response Objects: _meta, structuredContent, vs content – Reburn Report
      URL: https://reburnreport.news.blog/2026/03/04/mcp-response-objects-_meta-structuredcontent-vs-content/
      Published: 2026-03-04T17:30:00.000Z
      Author: N/A
      Highlights:
      In the Model Context Protocol (MCP), tool and resource responses are intentionally structured to separate machine reasoning, user presentation, and implementation details.
      ...
      ## 1. structuredContent
      ...
      Purpose: Machine-readable payload for the model and orchestration layers.
      ...
      - A JSON object
      - Must conform to the declared`outputSchema`(if defined)
      - Designed for deterministic reasoning and tool chaining
      ...
      ### Key properties
      ...
      - Strongly typed
      - Predictable structure
      - Not optimized for display
      - Contains authoritative result data
      ...
      ### Key properties
      ...
      - Optimized for readability
      - May summarize or explain structured results
      - Can contain markdown or formatted text
      - Does not need to strictly match schema
      ...
      ## 3. _meta
      ...
      Purpose: Out-of-band metadata not exposed to the model.
      ...
      - Arbitrary JSON object
      - Not part of conversational content
      - Invisible to the LLM
      ...
      ### Key properties
      ...
      - Hidden from the model
      - Can contain raw internal objects
      - Often used for UI hydration
      - Can avoid duplicating large datasets in`structuredContent`
      ...
      ```
      {  "_meta": {    "internalOrderObject": {      "dbId": 987654,      "warehouseId": "WH-02",      "rawCarrierPayload": { ... }    },    "renderHint": "showTrackingWidget"  }}
      ...
      The model never sees this data. It is strictly for application-layer use.
      ...
      ```
      {  "structuredContent": {    "orderId": "A1234",    "status": "shipped",    "trackingNumber": "1Z999AA10123456784"  },  "content": [    {      "type": "text",      "text": "Your order A1234 has shipped and is on the way."    }  ],  "_meta": {    "warehouseLocation": "Nevada",    "uiComponent": "tracking_card"  }}
      ...
      ## Conceptual Separation
      ...
      | Field | Purpose | Seen by Model | Seen by User | Typed |
      | --- | --- | --- | --- | --- |
      | `structuredContent` | Machine reasoning | Yes | Indirectly | Strongly |
      | `content` | Conversation | Yes | Yes | Loosely |
      | `_meta` | Implementation | No | No | Free-form |
      ...
      # When to Use Each
      ...
      ### Use structuredContent when:
      ...
      - The result must drive tool chaining
      - Deterministic schema validation is required
      - Output is operational rather than explanatory
      ...
      ### Use content when:
      ...
      - The result must be readable in chat
      - You want to provide explanation or narrative context
      ...
      ### Use _meta when:
      ...
      - The client needs raw backing data
      - UI hints or rendering instructions are required
      - Sensitive internal state must not be exposed to the model
      - Large datasets are needed for rendering but not reasoning
      ...
      # Common Design Pattern
      ...
      A clean implementation strategy:

      - Put canonical data in`structuredContent`
      - Put explanation or summary in`content`
      - Put heavy, internal, or UI-only data in`_meta`
      ...
      - Placing raw backend payloads directly into`structuredContent`
      - Exposing sensitive data the model does not need
      - Using`_meta` for data required for model reasoning
      ...
      # Bottom Line
      ...
      - `structuredContent`= structured truth for machines
      - `content`= conversational output for humans
      - `_meta`= private implementation details for the host
      ...
      Designing with clear separation between reasoning data, presentation data, and implementation data results in clean, scalable MCP tool interfaces.

      ---

      Title: Tool Annotations & Output Schemas - MCP Fundamentals | Stanza
      URL: https://www.stanza.dev/courses/mcp-fundamentals/tools/mcp-fundamentals-tool-annotations
      Published: N/A
      Author: N/A
      Highlights:
      Tool definitions in MCP go beyond name, description, and input schema. Annotations provide metadata hints that help clients make smarter decisions about how to present and execute tools. Output schemas add structured validation to tool results. Together, these features make MCP tools safer, more predictable, and easier for both humans and LLMs to work with.
      ...
      Annotations are optional metadata fields attached to a tool definition. They communicate behavioral hints to the client without changing the tool's functionality:
      ...
      - readOnlyHint: Indicates the tool does not modify any state. A client might auto-approve read-only tools.
      - destructiveHint: Signals the tool may delete or overwrite data. A client might require human confirmation.
      - openWorldHint: Tells the client the tool contacts external services beyond the local environment.
      - idempotentHint: Indicates the tool is safe to retry — calling it multiple times with the same arguments produces the same result.
      ...
      Output schemas extend MCP's type safety to tool results. By defining a`structuredContent` schema on the tool, the server declares the shape of successful responses. Clients can validate results automatically and present them in structured formats like tables or forms.
      ...
      When a tool defines an output schema, its result includes a`structuredContent` field alongside the regular`content` array. The`content` array remains for backward compatibility and human-readable output, while`structuredContent` provides machine-parseable data.
      ...
      The output schema tells clients exactly what fields to expect, enabling type-safe integrations.
      ...
      A critical design principle in MCP is that the server never decides to call a tool on its own. Tool invocation is always initiated by the LLM (with optional human approval). The server only advertises available tools and executes them when asked. This keeps the human or LLM in control of all actions.
      ...
      - Treating annotations as guarantees: Annotations are hints. A malicious or buggy server could set`readOnlyHint: true` on a destructive tool. Clients should use annotations for UX optimization, not security enforcement.
      - Ignoring annotations entirely: Failing to use annotations means missing opportunities to auto-approve safe tools or flag dangerous ones.
      - Assuming servers initiate tool calls: The MCP architecture ensures tool calls always flow from client to server, never the reverse.
      ...
      ### Best Practices
      ...
      - Set annotations accurately on every tool to help clients build trust-appropriate interfaces.
      - Combine`destructiveHint: true` with`idempotentHint` when applicable — it tells clients the tool is dangerous but safe to retry on failure.
      - Use output schemas for tools that return structured data to enable validation and richer client rendering.
      - Remember the control principle: the LLM or human always decides, the server only executes.
      ...
      Tool annotations provide behavioral metadata that helps clients make informed decisions about trust and approval. Output schemas add structured validation to tool results. The MCP design ensures that tool invocation is always controlled by the LLM or human, never by the server — keeping users in command of all actions.
      ...
      ```
      {
        "name
      ...
      _file",
        "description": "Permanently delete a file from the filesystem",
        "inputSchema":
      ...
      "
      ...
      ": "object",
          "properties
      ...
      {
            "path": { "type": "string" }
          },
          "required": ["path"]
        },
        "annotations": {
          "readOnlyHint": false,
          "destructiveHint": true,
          "idempotentHint": true,
          "openWorldHint": false
        }
      }

      ---

      Title: MCP Structured Output Deep Dive: outputSchema and structuredContent — ChatForest
      URL: https://chatforest.com/guides/mcp-structured-output-deep-dive/
      Published: 2026-03-28T14:30:00.000Z
      Author: N/A
      Highlights:
      When a tool declares an`outputSchema`, its`CallToolResult` should include a`structuredContent` field — a native JSON object (not a string) that validates against the declared schema:
      ...
      ```
      {
        "
      ...
      {
      ...
      ",
      ...
      "
          },
      ...
      "content
      ...
      [
            {
              "type
      ...
      "text",
              "text
      ...
      com) is on
      ...
      25-11-15."
            }
          ]
        }
      }
      ...
      The specification requires both fields to be present for tools with`outputSchema`:
      ...
      | Field | Purpose | Audience | Required? |
      | --- | --- | --- | --- |
      | `structuredContent` | Machine-readable typed data | Programmatic consumers, orchestrators | Required when`outputSchema` is declared |
      | `content` | Human/LLM-readable presentation | Language models, human users | Always required |
      ...
      Both fields must be semantically equivalent — they represent the same information in different formats.`content` is optimized for token efficiency and natural language understanding.`structuredContent` is optimized for type-safe programmatic access.
      ...
      This dual approach exists for backward compatibility. Older clients that don’t understand`structuredContent` continue to work by reading`content`. The specification states:
      ...
      A tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      Clients SHOULD NOT forward both fields to the LLM as separate inputs — they’d be double-counting the same information.
      ...
      If a tool declares`outputSchema` but a particular invocation can’t produce structured output (e.g., an error condition), the server may omit`structuredContent` and return only`content` with`isError: true`. The absence of`structuredContent` combined with`isError` signals to the client that validation should not be attempted for this response.
      ...
      Note the naming convention difference: Python SDK uses`
      ...
      _case`(`output_schema
      ...
      structured_content`,`
      ...
      _error`) while the
      ...
      protocol uses`
      ...
      Case`(`
      ...
      Schema`,`structuredContent`,`is
      ...
      `). The SDK handles serialization automatically.
      ...
      The specification says servers MUST return`structuredContent` that conforms to`outputSchema`, and clients SHOULD validate it. Here’s what validation looks like in practice:
      ...
      ## Schema Design Best Practices
      ...
      ### 1. Wrap Collections in Objects
      ...
      The root type must be`"object"`, so you can’t return a bare array. This is actually good practice — it leaves room for metadata:
      ...
      ### 2. Use Descriptive Field Names and Descriptions
      ...
      The`outputSchema` is visible to LLMs during tool discovery. Clear field names and descriptions help the model understand what data it will get and plan accordingly:
      ...
      ### 3. Mark Optional vs Required Fields Carefully
      ...
      `required` fields are the contract — they must always be present. Optional fields handle cases where data may not be available:
      ...
      ### 4. Use En
      ...
      for Constrained Values
      ...
      ### 5. Design content for the LLM, structuredContent for the Code
      ...
      The`content` field should be a natural-language summary optimized for token efficiency. The`structuredContent` carries the full data. They don’t need to be formatted the same way:
      ...
      The LLM sees the concise summary in`content`. The orchestrator gets the full typed data in`structuredContent`.
      ...
      ### 6. Keep Schemas Stable
      ...
      Treat`outputSchema` like a public API contract. Breaking changes — removing required fields, changing types, altering enum values — will break downstream consumers. When you need to evolve:
      ...
      - Adding optional fields is safe and backward-compatible
      - Removing required fields is a breaking change
      - Changing field types is a breaking change
      - Adding enum values may break clients that do exhaustive matching
      ...
      Keep the existing`content` for backward compatibility and add`structured_content`:
      ...
      - Some clients read only`content` and ignore`structuredContent`— this is why the backward-compatibility rule (always include serialized data in a`TextContent` block) matters.
      - Other clients validate that tools with`outputSchema` actually return`structuredContent` and may log warnings or errors when it’s missing.
      - Programmatic orchestrators (non-LLM clients building automated pipelines) are often the primary consumers of`structuredContent`.
      ...
      The practical takeaway: always return both`content` and`structuredContent`. Never rely on clients consuming`structuredContent` alone. This ensures your tools work across the full spectrum of current MCP clients.
      ...
      Use`outputSchema` when your tool returns data — records, measurements, statuses, lists, computed results. Skip it when your tool returns content.
      ...
      | Feature | Purpose |
      | --- | --- |
      | `outputSchema` | Declares the typed shape of a tool’s return value (JSON Schema, root must be object) |
      | `structuredContent` | The actual typed data in the response (native JSON object, not a string) |
      | `content` | Human/LLM-readable presentation (always required, backward-compatible) |
      | Dual response | Both fields carry semantically equivalent information for different audiences |
      | Client validation | SHOULD validate`structuredContent` against`outputSchema`(especially for untrusted servers) |
      ...
      The key design principle:`outputSchema` and`structuredContent` turn MCP tools into typed functions with real return signatures. They make tool chaining deterministic, validation possible, and LLM-based orchestration more reliable — while maintaining full backward compatibility through the existing`content` field.

      ---

      Title: CallToolResult in rust_mcp_schema::mcp_2025_11_25 - Rust
      URL: https://docs.rs/rust-mcp-schema/latest/rust_mcp_schema/mcp_2025_11_25/struct.CallToolResult.html
      Published: N/A
      Author: N/A
      Highlights:
      ```
      pub struct CallToolResult {
          pub content: Vec<ContentBlock>,
          pub is_error: Option<bool>,
          pub meta: Option<Map<String, Value>>,
          pub structured_content: Option<Map<String, Value>>,
      }
      ```
      ...
      ```json
      {
       "description": "The server's response to a tool call.",
       "type": "object",
       "required": [
         "content"
       ],
       "properties": {
         "_meta": {
           "description": "See [General fields: _meta](https://modelcontextprotocol.io/specification/2025-11-25/basic/index#meta) for notes on _meta usage.",
           "type": "object",
           "additionalProperties": {}
         },
         "content": {
           "description": "A list of content objects that represent the unstructured result of the tool call.",
           "type": "array",
           "items": {
             "$ref": "#/$defs/ContentBlock"
           }
         },
         "isError": {
           "description": "Whether the tool call ended in an error.\n\nIf not set, this is assumed to be false (the call was successful).\n\nAny errors that originate from the tool SHOULD be reported inside the result\nobject, with isError set to true, _not_ as an MCP protocol-level error\nresponse. Otherwise, the LLM would not be able to see that an error occurred\nand self-correct.\n\nHowever, any errors in _finding_ the tool, an error indicating that the\nserver does not support tool calls, or any other exceptional conditions,\nshould be reported as an MCP error response.",
           "type": "boolean"
         },
         "structuredContent": {
           "description": "An optional JSON object that represents the structured result of the tool call.",
           "type": "object",
           "additionalProperties": {}
         }
       }
      }
      ```
      ...
      See General fields: _meta for notes on _meta usage.
      ...
      An optional JSON object that represents the structured result of the tool call.
      ...
      #### pub fn with_meta(self, meta: Option<Map<String, Value>>) -> Self
      ...
      Assigns metadata to the CallToolResult, enabling the inclusion of extra context or details.
      ...
      #### pub fn with_structured_content( self, structured_content: Map<String, Value>, ) -> Self
      ...
      Assigns structured_content to the CallToolResult
      ...
      This method initializes the structure with the given`content` and resets all optional fields (`is_error`,`meta`, and`structured_content`) to`None`.

      ---

      Title: MCP structuredContent: How to Return Large Results Without Flooding the Context Window
      URL: https://futuresearch.ai/blog/mcp-results-widget/
      Published: 2026-02-26T16:35:30.000Z
      Author: N/A
      Highlights:
      tl;dr: Large structured data doesn't belong in the LLM's context window. Instead of dumping thousands of rows into the tool response: 1) use`structuredContent` to give the user an interactive widget rendered directly in Claude.ai and Claude Desktop — this never enters the model's context, so it costs zero tokens, 2) keep the model's`content` lean with a small preview, and 3) provide a download URL so the model can pull the data into its sandbox and process it with real code.
      ...
      - The model's context needs a small sample with column names, a few representative rows, and metadata (total count, pagination hints). This is enough to answer "what are the columns?", "show me the top results", or "are there any errors?".
      - The user needs the full dataset in an interactive format with sortable columns, per-column filters, global search, CSV download, clickable links.
      - The model's sandbox needs the
      ...
      dataset as a file it can
      ...
      Serving all three in the same`content` field is the mistake. MCP's`CallToolResult` has two fields:`content` goes to the model,`structuredContent` goes to the client. Use`content` for the text summary and download URL. Use`structuredContent` for the widget data — it never enters the context, so it costs zero tokens.
      ...
      ```
      @mcp.tool()
      async def my_tool() -> CallToolResult:
          df = run_query()  # 5,000 rows
          preview = df.head(50).to_dict(orient="records")

          # structuredContent: drives the interactive widget (client only, NOT the LLM)
          widget_data = {
              "preview": preview,
              "total": len(df),
              "csv_url": build_download_url(task_id),
              "fetch_full_results": True,
          }

          # content: what the model actually reads
          summary = (
              f"Results: {len(df)} rows, {len(df.columns)} columns. "
              f"Showing first 50. Full dataset visible in widget above.\n"
              f"Full CSV download: {build_download_url(task_id)}"
          )

          return CallToolResult(
              content=[TextContent(type="text", text=summary)],
              structuredContent=widget_data,
          )

      ```
      ...
      The`content` field contains a plain text summary. It includes:
      ...
      - Token-bounded preview: row count, column names, and a page of inline data controlled by`page_size`. If the rows contain long text fields, the server silently reduces the page to fit a token budget.
      - Pagination hints: so the model can fetch more pages if the user asks ("show me the next 50").
      - Download URL: the CSV endpoint for the sandbox, so the model can`curl` the full dataset when it needs to do real analysis.
      ...
      Only`content` enters the model's context window. The widget data is completely invisible to the LLM.
      ...
      The`structuredContent` field carries a dict that drives an interactive widget embedded directly in the conversation. This data is delivered to the client but never enters the model's context, so it adds zero token cost. The client (Claude.ai) renders the dict through a registered widget. How? The tool is registered with a`resourceUri` pointing to an HTML resource hosted by the MCP server. Claude.ai fetches that HTML once at connection time and renders it in a sandboxed iframe. The HTML is a self-contained JavaScript application that listens for tool results via the MCP ext-apps SDK, processes the`structuredContent` payload, and renders the table.
      ...
      On first load, the widget shows the small preview baked into the dict while it fetches the full dataset in the background. Once loaded, the user gets sorting, filtering, global search, CSV/JSON export, all running client-side with no server round-trips.
      ...
      dataset, but the
      ...
      contains a small preview
      ...
      The rest comes from a dedicated REST endpoint on the MCP server. The flow:
      ...
      1. Tool call completes. The MCP response includes a small preview and a`poll_token` for the widget.
      2. Widget renders the preview immediately.
      3. Widget mints a download token by calling`GET /api/results/{task_id}/download-token` with the poll token as a Bearer header. The server returns a fresh single-use token.
      4. Widget fetches full data via`GET /api/results/{task_id}/download?token={download_token}&format=json`. The server consumes the token and returns the full dataset.
      5. Widget replaces the preview with the complete dataset. Sorting, filtering, and search now operate on all rows.
      ...
      Since`structuredContent` is already invisible to the model, it costs nothing even when the client ignores it. Still, there's no point generating widget data for a terminal client. You can check who's connecting during the MCP`initialize` handshake: clients that support widgets advertise it via`capabilities.extensions["io.modelcontextprotocol/ui"]`. Alternatively, check`clientInfo.name`— Claude Code identifies as`"claude-code"` while Claude.ai and Claude Desktop both identify as`"claude-ai"`. If the client doesn't support widgets, skip`structuredContent` entirely and return only the text summary and download URL.
      ...
      - Row count > 500: context cost dominates and the model can't reason across all of them anyway.
      - Wide schemas or long text fields: a single row can
      ...
      hundreds of tokens.
      - Follow-up analysis is
      ...
      : the user will want to group, filter, or chart the data, which means the sandbox needs the file.
      ...
      The pattern for returning large results from MCP tools:

      1. Don't dump everything into the context. The LLM can't usefully process thousands of rows, and you're burning tokens for nothing.
      2. Split the response. Use`content` for a compact text summary (model only) and`structuredContent` for the rich widget (client only, zero token cost). Include a download URL so the model's sandbox can fetch the full dataset.
      3. Serve the full data over plain HTTP, separate from MCP. Both the widget and the model's sandbox fetch via REST.
      ...
      The context window is for reasoning, not storage. The sandbox is for processing. The widget is for the user. Keep each in its lane.
```
/answer b26eff25-c055-411b-801c-8466fd6c7482 allow
Control Request:
```yaml
id: 571047e0-3ec9-4f0a-b1c1-482d532a3560
toolName: web-search-exa
arguments:
  query: Model Context Protocol CallToolResult content array multiple text blocks advisory pattern
```
/answer 571047e0-3ec9-4f0a-b1c1-482d532a3560 allow
Control Request:
```yaml
id: eecefd48-d568-4328-aa20-925b351fafb2
result:
  structuredContent:
    content: |-
      Title: Tools - Model Context Protocol
      URL: https://modelcontextprotocol.io/specification/2025-11-25/server/tools
      Published: 2025-11-25T00:00:00.000Z
      Author: N/A
      Highlights:
      ### Tool Result
      ...
      Tool results may contain structured or unstructured content.

      Unstructured content is returned in the `content` field of a result, and can contain multiple content items of different types:

      All content types (text, image, audio, resource links, and embedded resources)
      support optional
      annotations that
      provide metadata about audience, priority, and modification times. This is the
      same annotation format used by resources and prompts.
      ...
      #### Text Content
      ...
      ```json
      {
        "type": "text",
        "text": "Tool result text"
      }
      ...
      #### Structured Content
      ...
      Structured content is returned as a JSON object in the `structuredContent` field of a result.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      #### Output Schema
      ...
      ```json
      {
        "jsonrpc": "2.0",
        "id":
      ...
      5,
        "result": {
          "content": [
            {
              "type": "text",
              "text": "{\"temperature\": 22.5, \"conditions\": \"Partly cloudy\", \"humidity\": 65}"
            }
          ],
          "structuredContent": {
            "temperature": 22.5,
            "conditions": "Partly cloudy",
            "humidity": 65
          }
        }
      }

      ---

      Title: SEP-1624: Clarify `structuredContent` vs `content` Usage Guidance · Issue #1624 · modelcontextprotocol/modelcontextprotocol
      URL: https://github.com/modelcontextprotocol/modelcontextprotocol/issues/1624
      Published: 2025-10-08T04:37:26.000Z
      Author: krubenok
      Highlights:
      This SEP proposes clarifying the specification around `structuredContent` and `content` fields in `CallToolResult` to address inconsistent client and server implementations. The proposal focuses on **non-breaking clarifications** to the existing spec language to better communicate the intended design and usage patterns for both fields.
      ...
      ```markdown
      #### Structured Content

      **Structured** content is returned as a JSON object in the `structuredContent` field of a result.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ```
      ...
      ```markdown
      #### Structured Content

      **Structured** content is returned as a JSON object in the `structuredContent` field of a result.
      ...
      The `structuredContent` and `content` fields serve different use cases:
      ...
      - **`content`**: Model-oriented output optimized for readability and token efficiency. Preferred for conversational agents and direct model prompting. Does not require strict schematisation. A plain text or markdown representation of the output for example.
      - **`structuredContent`**: Machine-oriented output for programmatic tool use, code generation, type-safe orchestration, and strict schema validation.
      ...
      **Server Requirements**:

      - When both fields are present, they MUST be semantically equivalent (same information, different presentation)
        - In some cases, an explicit field in `structuredContent` can be semantically present in `content` without specific mention. For example if expected fields are returned and no error is mentioned, a `structuredContent` response that contains a `"status": "success",` object is evident without the status being verbatim mentioned as successful in `content`.
      - For backwards compatibility, a tool that returns structured content SHOULD also return a response in `content`.
      ...
      **Client Guidance**:

      - Clients SHOULD choose the appropriate field to expose to the model based on their use case:
        - **Conversational/Agent UX**: Prefer `content` for lower token cost and readability (by both humans and models).
        - **Programmatic/Code Mode**: Prefer `structuredContent` for type safety and schema validation
      ...
      - Clients SHOULD NOT forward both fields verbatim to models as semantically distinct inputs.
      - Clients that use `content` MUST still perform `outputSchema` validation against `structuredContent` when present
      ```
      ...
      **Design patterns for complementary outputs:**

      - Define an `outputSchema` that captures all data fields, ensuring that the `text` field from `content` is present as a substring of `structuredContent`.
      - Provide semantically equivalent `content` with enhanced readability
      - Format `content` with context that helps models use the information effectively
      ...
      ```typescript
      /**
       * An optional JSON Schema object defining the structure of the tool's output returned in
       * the structuredContent field of a CallToolResult.
       *
       * When outputSchema is provided:
       * - Servers MUST populate structuredContent with data conforming to this schema
       * - Clients SHOULD validate structuredContent against this schema
       * - Servers SHOULD also provide a human-readable representation in content
       *
       * Use outputSchema for programmatic tool use, code generation, and type-safe integrations.
       * For conversational agents, consider whether unstructured content provides better model performance.
       */
      outputSchema?: {
        type: "object";
        properties?: { [key: string]: object };
        required?: string[];
      };
      ```
      ...
      > Open Question: if we continue to expect that `content` is present in `structuredContent` as a substring, rather than the stringified JSON we have today, we will have semantic duplication in `structuredContent`. By the letter of the proposed spec this is not an issue as codeful use of `structuredContent` can ignore the embedded `content` field. In practice with clients currently preferring `structuredContent` as model input, this duplication could lead to increased token usage and duplicate information being provided.
      ...
      284 tokens of output excluding the semantically duplicative `content` object embedded inside.
      ...
      ### `content`
      ...
      > - I actually think having `content` == `JSON.stringify(structuredContent)` is pretty reasonable most of the time. It's often not actually that many more tokens, and on presentation I think this should be down to the model (if needed for the user) rather than the server tool response. Especially as we move to a world with agents doing longer tasks, I think individual tool results will be less relevant to humans and programmatic MCP will be even more important, so leaning into structuredContent seems reasonable.
      >
      > - And yes there's duplication here, but tbh the cost is likely so low that it doesn't matter. If we do want to fix this we could add a capability to clients that indicates they definitely support `structuredContent` so the server doesn't need to send `content` as well, but tbh I think more features/combinations in MCP is probably the wrong direction.
      ...
      > Thanks both for the input! I think we're caught between a few points here, trying to identify which compromises make sense.
      >
      > > In general, I don't think structuredContent should include content - agree this leads to duplication, especially if all the info is already captured in the response
      >
      > Yes, in general I agree, but it seemed like a simply way to not require any changes of the existing validation expectations driven by the Inspector for example expecting that `content` is a JSON.stringify'ed substring of `structuredContent`. If we're willing to "break" that (which I don't think is actually a breaking change?) then I would agree we should ideally not duplicate it.
      >
      > > I actually think having content == JSON.stringify(structuredContent) is pretty reasonable most of the time. It's often not actually that many more tokens, and on presentation I think this should be down to the model (if needed for the user) rather than the server tool response. Especially as we move to a world with agents doing longer tasks, I think individual tool results will be less relevant to humans and programmatic MCP will be even more important, so leaning into structuredContent seems reasonable.
      ...
      >
      > [Image: Image | https://github.com/user-attachments/assets/e0a410ca-1f67-42c8-be50-f55eaebfb3d2]
      >
      > [Image: Image | https://github.com/user-attachments/assets/ac04231a-9e48-4a9d-b8de-7c563793eefd]
      >
      > [Image: Image | https://github.com/user-attachments/assets/338f5d1e-68de-45da-8c6c-98f95e9145c7]
      >
      > It seems to me that tools can fairly easily expose both fields for codeful and model use cases without harming the other. Ack I think a more ideal solution to this would be some of the discussion around audience annotations, but my gut sense is that we don't need to wait for a breaking change like that to improve things in the meantime. This would effectively be saying:
      >
      > - `structuredContent` is optimized for codeful use and output validatation, `content` is for direct model use.
      > - For backwards compatibility or as a fall back clients should prefer one over the other informed by their use case, but since they are intended to be semantically equivalent, a client that prefers `content` should be able to use `structuredContent` instead despite some of the compromises.
      >
      > I think the main "cost" to going this direction would be extra data over the wire of providing similar data in two fields without having a client declare if it actually wants/needs both.
      >
      > > If anything maybe we make it an extension on top of the protocol, **so its very clear to clients that they do not need to care about structuredContent at all if they're not an MCP as Code thing?**
      >
      > **Bold** is my impression as well, but I'll leave it to @domdomegg and co if we want to make such a pointy statement (the not-bold part) on that.
      >
      > > > Clients that use content MUST still perform outputSchema validation against structuredContent when present```
      ...
      >

      ---

      Title: What is the expected usage of `structuredContent` vs `content` in `CallToolResult`? · modelcontextprotocol modelcontextprotocol · Discussion #1563 · GitHub
      URL: https://github.com/modelcontextprotocol/modelcontextprotocol/discussions/1563
      Published: N/A
      Author: modelcontextprotocol
      Highlights:
      I'm trying to determine how its expected for`structuredContent` and`content` to be used in the result of a tool call. I have some confusion due to the language used in the spec. Specifically:
      ...
      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      | export interface CallToolResult extends Result { |
      | --- |
      | /** |
      | * A list of content objects that represent the unstructured result of the tool call. |
      | */ |
      | content: ContentBlock[]; |
      | /** |
      | * An optional JSON object that represents the structured result of the tool call. |
      | */ |
      | structuredContent?: { [key: string]: unknown }; |
      ...
      The spec simply seems to indicate that`content` is for unstructured data, and`structuredContent` is for structured data, and that both are equally valid in a response. Meaning that each could contain information that is different, and not simply the same data in different representations.
      ...
      This seems more logical to me, as I can see use cases where both would apply. For example if a tool returns a simple message along with the structured data. Or maybe binary data plus structured data. But this seems contrary and even incompatible with how their usage is explained in that first reference I sourced. "Incompatible" as in if unstructured plain text or binary data is in the`content` field, then how is one supposed to also put the serialized form of`structuredContent` in the field?`content` is an array. But should the serialized form go first, last, in the middle?
      ...
      Good question — the spec is a bit ambiguous here. Here's what it actually says and where the tension lies.
      ...
      - `content`(required): An array of`ContentBlock` items — text, images, audio, etc. Described as "the unstructured result of the tool call."
      - `structuredContent`(optional): A JSON object. Described as "the structured result." Must conform to`outputSchema` if one is defined.
      ...
      From the structured content spec:

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      "text": "
      ...
      22.5
      ...
      \"Partly
      ...
      \"}" }
        ],
      ...
      "structuredContent
      ...
      5,
          "conditions": "Partly cloudy"
        }
      }
      ...
      The spec describes`content` and`structuredContent` as serving different purposes (unstructured vs structured), which implies they could carry different information. But the backwards-compatibility guidance and the example both treat`content` as a serialized duplicate of`structuredContent`.
      ...
      The spec doesn't address:
      ...
      - Whether`content` can carry additional non-duplicate data (images, human-readable messages) alongside the serialized`structuredContent`
      - If you include both the serialized JSON and other content blocks, what order they should be in
      - Which field a client should prefer when both are present (though the design intent is clearly that modern clients should use`structuredContent` when available)
      ...
      Based on the spec's intent:
      ...
      1. If your tool has an`outputSchema`: return the typed result in`structuredContent`, and a serialized copy in`content` for older clients. This is the primary use case.
      2. If your tool returns mixed content (e.g., structured data + images): use`structuredContent` for the structured part and`content` for everything else. The spec doesn't prohibit this, but be aware that older clients will only see`content`.
      3. For clients: prefer`structuredContent` when present and you understand it. Fall back to`content` otherwise.
      ...
      , posting a SEP here
      ...
      relates to this topic:#

      ---

      Title: Call tools, read resources, get prompts | MCP TypeScript SDK
      URL: https://ts.sdk.modelcontextprotocol.io/v2/clients/calling.html
      Published: N/A
      Author: N/A
      Highlights:
      `listTools` returns the tools the server advertises; `callTool` invokes one by name with a plain `arguments` object
      ...
      callTool({ name: '
      ...
      -order', arguments: { id: 'A-1041' } });
      ...
      console.log(result.content);
      ...
      `result.content` is the content array the tool handler returned, unchanged:
      ...
      ```
      [ 'lookup-order', 'order-total', 'export-orders' ]
      [ { type: 'text', text: 'A-1041: 3 items, shipped' } ]

      ```
      ...
      A failed tool call is still a result: check `isError` on it before trusting `content`. Arguments the input schema rejects come back the same way. Only protocol-level failures — unknown tool, timeout — throw.
      ...
      A tool that declares an `outputSchema` returns `structuredContent` next to `content`. It is typed `unknown` — check that it is present and narrow it before use.
      ...
      When an earlier `listTools()` gave the client the tool's `outputSchema`, `callTool` validates `structuredContent` against it and rejects a result that does not match.
      ...
      - `callTool` returns `content
      ...
      the tool declares
      ...
      `outputSchema`, `structuredContent
      ...
      for your application.

      ---

      Title: docs/servers/tools.md
      URL: https://github.com/modelcontextprotocol/typescript-sdk/blob/main/docs/servers/tools.md
      Published: N/A
      Author: N/A
      Highlights:
      Every call on this page comes
      ...
      -memory `Client`
      ...
      server above — [
      ...
      ```ts source="../../examples/guides/servers/tools.examples.ts#callTool_search"
      const result = await client.callTool({ name: 'search', arguments: { query: 'mug' } });
      console.log(result.content);
      ...
      The handler's `content` comes back unchanged:
      ...
      ```
      [ { type: 'text', text: 'Travel mug\nMug rack' } ]
      ...
      The rejection is an ordinary tool result with `isError: true`, so the model reads the message and retries with arguments that fit the schema. Thrown errors and protocol-level failures are their own topic — see [Errors](errors.md).
      ...
      Add `outputSchema` and return the matching value as `structuredContent`, next to the human-readable `content`.
      ...
      ```ts source="../../examples/guides/servers/tools.examples.ts#
      ...
      Tool_structured"
      server.registerTool(
          'product-details',
          {
              description: 'Look up one product by its exact name',
              inputSchema: z.object({ name: z.string() }),
              outputSchema: z.object({ name: z.string(), price: z.number() })
          },
          async ({ name }) => {
              const product = catalog.find(candidate => candidate.name === name);
              if (!product) throw new Error(`No product named ${name}`);
              const output = { name: product.name, price: product.price };
              return {
                  content: [{ type: 'text', text: JSON.stringify(output) }],
                  structuredContent: output
              };
          }
      );
      ...
      The SDK validates `structuredContent` against `outputSchema` before the result leaves your server, and advertises the derived JSON Schema in `tools/list` so clients can validate it too.
      ...
      Calling `product-details` with `{ name: 'Travel mug' }` returns both renderings:
      ...
      ```
      {
        content: [ { type: 'text', text: '{"name":"Travel mug","price":24}' } ],
        structuredContent: { name: 'Travel mug', price: 24 }
      }
      ...
      - `registerTool(name, config, handler)` registers a tool; `inputSchema` is a Zod object schema.
      - The one schema yields the advertised JSON Schema, argument validation, and the handler
      ...
      types.
      -
      ...
      that fail the schema come back as an `isError: true` tool result; the handler never runs.
      - `outputSchema` plus `structuredContent` add machine-readable results, validated before they leave the server.
      ...
      `title` and `annotations` describe the tool to clients and never change execution.

      ---

      Title: Tools
      URL: https://modelcontextprotocol.io/specification/2025-06-18/server/tools.md
      Published: N/A
      Author: N/A
      Highlights:
      ### Tool Result
      ...
      Tool results may contain structured or unstructured content.

      Unstructured content is returned in the `content` field of a result, and can contain multiple content items of different types:

      All content types (text, image, audio, resource links, and embedded resources)
      support optional
      annotations that
      provide metadata about audience, priority, and modification times. This is the
      same annotation format used by resources and prompts.
      ...
      #### Text Content
      ...
      ```json
      {
        "type": "text",
        "text": "Tool result text"
      }
      ...
      #### Structured Content
      ...
      Structured content is returned as a JSON object in the `structuredContent` field of a result.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      #### Output Schema
      ...
      may also provide an
      ...
      an output schema
      ...
      ```json
      {
        "jsonrpc": "2.0",
        "id": 5,
        "result": {
          "content": [
            {
              "type": "text",
              "text": "{\"temperature\": 22.5, \"conditions\": \"Partly cloudy\", \"humidity\": 65}"
            }
          ],
          "structuredContent": {
            "temperature": 22.5,
            "conditions": "Partly cloudy",
            "humidity": 65
          }
        }
      }

      ---

      Title: Tools – Model Context Protocol （MCP）
      URL: https://modelcontextprotocol.info/docs/concepts/tools/
      Published: N/A
      Author: N/A
      Highlights:
      Tool errors should be reported within the result object, not as MCP protocol-level errors. This allows the LLM to see and potentially handle the error. When a tool encounters an error:
      ...
      1. Set`isError` to`true` in the result
      2. Include error details in the`content` array
      ...
      handling for tools:
      ...
      ```
          try {
            // Tool operation
            const result = performOperation();
            return {
              content: [
                {
                  type: "text",
                  text: `Operation successful: ${result}`
                }
              ]
            };
          } catch (error) {
            return {
              isError: true,
              content: [
                {
                  type: "text",
                  text: `Error: ${error.message}`
                }
              ]
            };
          }
          ```
      ...
      ```
       try:
       # Tool operation
       result = perform_operation()
       return types.CallToolResult(
       content=[
       types.TextContent(
       type="text",
       text=f"Operation successful: {result}"
       )
       ]
       )
       except Exception as error:
       return types.CallToolResult(
       isError=True,
       content=[
       types.TextContent(
       type="text",
       text=f"Error: {str(error)}"
       )
       ]
       )
          ```
      ...
      This approach allows the LLM to see that an error occurred and potentially take corrective action or request human intervention.

      ---

      Title: CallToolResultSchema | MCP TypeScript SDK
      URL: https://ts.sdk.modelcontextprotocol.io/variables/types.CallToolResultSchema.html
      Published: N/A
      Author: N/A
      Highlights:
      CallToolResultSchema: ZodObject< { _meta: ZodOptional< ZodObject< { "io.modelcontextprotocol/related-task": ZodOptional< ZodObject<{ taskId: ZodString }, $strip>, >; progressToken: ZodOptional<ZodUnion >; }, $loose, >, >; content: ZodDefault< ZodArray< ZodUnion< readonly [ ZodObject< { _meta: ZodOptional<ZodRecord<ZodString, ZodUnknown>>; annotations: ZodOptional< ZodObject< { audience: ...; lastModified: ...; priority: ... }, $strip, >, >; text: ZodString; type: ZodLiteral<"text">; }, $strip, >, ZodObject< { _meta: ZodOptional<ZodRecord<ZodString, ZodUnknown>>; annotations: ZodOptional< ZodObject< { audience: ...; lastModified: ...; priority: ... }, $strip, >, >; data: ZodString; mimeType: ZodString; type: ZodLiteral<"image">; }, $strip, >, ZodObject< { _meta: ZodOptional<ZodRecord<ZodString, ZodUnknown>>; annotations: ZodOptional< ZodObject< { audience: ...; lastModified: ...; priority: ... }, $strip, >, >; data: ZodString; mimeType: ZodString; type: ZodLiteral<"audio">; }, $strip, >, ZodObject< { _meta: ZodOptional<ZodObject<{}, $loose>>; annotations: ZodOptional< ZodObject< { audience: ...; lastModified: ...; priority: ... }, $strip, >, >; description: ZodOptional; icons: ZodOptional<ZodArray<ZodObject<(...), (...)>>>; mimeType: ZodOptional; name: ZodString; size: ZodOptional; title: ZodOptional; type: ZodLiteral<"resource_link">; uri: ZodString; }, $strip, >, ZodObject< { _meta: ZodOptional<ZodRecord<ZodString, ZodUnknown>>; annotations: ZodOptional< ZodObject< { audience: ...; lastModified: ...; priority: ... }, $strip, >, >; resource: ZodUnion< readonly [ZodObject<(...), (...)>, ZodObject<(...), (...)>], >; type: ZodLiteral<"resource">; }, $strip, >, ], >, >, >; isError: ZodOptional; structuredContent: ZodOptional<ZodRecord<ZodString, ZodUnknown>>; }, $loose,> = ...
      ...
      The server's response to
      ...
      tool call.

      ---

      Title: MCP structuredContent: How to Return Large Results Without Flooding the Context Window
      URL: https://futuresearch.ai/blog/mcp-results-widget/
      Published: 2026-02-26T16:35:30.000Z
      Author: N/A
      Highlights:
      tl;dr: Large structured data doesn't belong in the LLM's context window. Instead of dumping thousands of rows into the tool response: 1) use`structuredContent` to give the user an interactive widget rendered directly in Claude.ai and Claude Desktop — this never enters the model's context, so it costs zero tokens, 2) keep the model's`content` lean with a small preview, and 3) provide a download URL so the model can pull the data into its sandbox and process it with real code.
      ...
      Serving all three in the same`content` field is the mistake. MCP's`CallToolResult` has two fields:`content` goes to the model,`structuredContent` goes to the client. Use`content` for the text summary and download URL. Use`structuredContent` for the widget data — it never enters the context, so it costs zero tokens.
      ...
      ```
      @mcp.tool()
      async def my_tool() -> CallToolResult:
          df = run_query()  # 5,000 rows
          preview = df.head(50).to_dict(orient="records")

          # structuredContent: drives the interactive widget (client only, NOT the LLM)
          widget_data = {
              "preview": preview,
              "total": len(df),
              "csv_url": build_download_url(task_id),
              "fetch_full_results": True,
          }

          # content: what the model actually reads
          summary = (
              f"Results: {len(df)} rows, {len(df.columns)} columns. "
              f"Showing first 50. Full dataset visible in widget above.\n"
              f"Full CSV download: {build_download_url(task_id)}"
          )

          return CallToolResult(
              content=[TextContent(type="text", text=summary)],
              structuredContent=widget_data,
          )

      ```
      ...
      The`content` field contains a plain text summary. It includes:
      ...
      - Token-bounded preview: row count, column names, and a page of inline data controlled by`page_size`. If the rows contain long text fields, the server silently reduces the page to fit a token budget.
      - Pagination hints: so the model can fetch more pages if the user asks ("show me the next 50").
      - Download URL: the CSV endpoint for the sandbox, so the model can`curl` the full dataset when it needs to do real analysis.
      ...
      Only`content` enters the model's context window. The widget data is completely invisible to the LLM.
      ...
      The`structuredContent` field carries a dict that drives an interactive widget embedded directly in the conversation. This data is delivered to the client but never enters the model's context, so it adds zero token cost. The client (Claude.ai) renders the dict through a registered widget. How? The tool is registered with a`resourceUri` pointing to an HTML resource hosted by the MCP server. Claude.ai fetches that HTML once at connection time and renders it in a sandboxed iframe. The HTML is a self-contained JavaScript application that listens for tool results via the MCP ext-apps SDK, processes the`structuredContent` payload, and renders the table.
      ...
      On first load, the widget shows the small preview baked into the dict while it fetches the full dataset in the background. Once loaded, the user gets sorting, filtering, global search, CSV/JSON export, all running client-side with no server round-trips.
      ...
      The widget needs the full dataset, but the MCP tool response only contains a small preview. The rest comes from a dedicated REST endpoint on the MCP server. The flow:
      ...
      1. Tool call completes. The MCP response includes a small preview and a`poll_token` for the widget.
      2. Widget renders the preview immediately.
      3. Widget mints a download token by calling`GET /api/results/{task_id}/download-token` with the poll token as a Bearer header. The server returns a fresh single-use token.
      4. Widget fetches full data via`GET /api/results/{task_id}/download?token={download_token}&format=json`. The server consumes the token and returns the full dataset.
      5. Widget replaces the preview with the complete dataset. Sorting, filtering, and search now operate on all rows.
      ...
      Desktop. They both have a rendering layer that can display interactive HTML via MCP Apps. Claude Code is a terminal. It can't render widgets, and it ignores`structuredContent` in tool responses.
      ...
      But the core problem of keeping large data out of the context still applies. Claude Code solves it differently: it has direct filesystem access and can run`curl` locally. So instead of three lanes, you get two:
      ...
      1. Model context gets the compact text summary (same as Claude.ai).
      2. Local execution replaces both the widget and the sandbox. Claude Code downloads the CSV and processes it with pandas on your machine.
      ...
      Since`structuredContent` is already invisible to the model, it costs nothing even when the client ignores it. Still, there's no point generating widget data for a terminal client. You can check who's connecting during the MCP`initialize` handshake: clients that support widgets advertise it via`capabilities.extensions["io.modelcontextprotocol/ui"]`. Alternatively, check`clientInfo.name`— Claude Code identifies as`"claude-code"` while Claude.ai and Claude Desktop both identify as`"claude-ai"`. If the client doesn't support widgets, skip`structuredContent` entirely and return only the text summary and download URL.
      ...
      If your result set is under 100 rows and mostly numeric, dumping inline is fine. This pattern matters when:
      ...
      - Row count > 500: context cost dominates and the model can't reason across all of them anyway.
      - Wide schemas or long text fields: a single row can burn hundreds of tokens.
      - Follow-up analysis is expected: the user will want to group, filter, or chart the data, which means the sandbox needs the file.
      ...
      The pattern for returning large results from MCP tools:

      1. Don't dump everything into the context. The LLM can't usefully process thousands of rows, and you're burning tokens for nothing.
      2. Split the response. Use`content` for a compact text summary (model only) and`structuredContent` for the rich widget (client only, zero token cost). Include a download URL so the model's sandbox can fetch the full dataset.
      3. Serve the full data over plain HTTP, separate from MCP. Both the widget and the model's sandbox fetch via REST.
      ...
      The context window is for reasoning, not storage. The sandbox is for processing. The widget is for the user. Keep each in its lane.

      ---

      Title: docs/specification/draft/server/tools.mdx
      URL: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/draft/server/tools.mdx
      Published: N/A
      Author: N/A
      Highlights:
      Tool results may contain [**structured**](#structured-content) or **unstructured** content.

      **Unstructured** content is returned in the `content` field of a result, and can contain multiple content items of different types:


       All content types (text, image, audio, resource links, and embedded resources)
       support optional
       [annotations](/specification/draft/server/resources#annotations) that provide
       metadata about audience, priority, and modification times. This is the same
       annotation format used by resources and prompts.


      #### Text Content
      ...
      ```json
      {
        "type": "text",
        "text": "Tool result text"
      }
      ...
      #### Structured Content
      ...
      **Structured** content is returned as a JSON value in the `structuredContent` field of a result. This can be any JSON value (object, array, string, number, boolean, or null) that conforms to the tool's `outputSchema` if one is defined.

      For backwards compatibility, a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block.
      ...
      #### Output Schema
      ...
      Example tool with array output schema:
      ...
      ```json
      {
        "name": "list_users",
        "title": "User List",
        "description": "Returns a list of all users",
        "inputSchema": {
          "type": "object",
          "properties": {}
        },
        "outputSchema": {
          "type": "array",
          "items": {
            "type":
      ...
      object",
            "properties": {
              "id": { "type": "string" },
              "name": { "type": "string" },
              "email": { "type": "string" }
            },
            "required": ["id", "name", "email"]
          }
        }
      }
      ...
      Example valid response for a tool with array output:
      ...
      ```json
      {
        "jsonrpc": "2.0",
        "id": 6,
        "result": {
          "resultType": "complete",
          "content": [
            {
              "type": "text",
              "text": "Found 2 users: Alice (alice@example.com) and Bob (bob@example.com)."
            }
          ],
          "structuredContent": [
            { "id": "1", "name": "Alice", "email": "alice@example.com" },
            { "id": "2", "name": "Bob", "email": "bob@example.com" }
          ]
        }
      }
```
/answer eecefd48-d568-4328-aa20-925b351fafb2 allow
ReasoningToken: 804
Sehr hilfreich – die MCP-Spec und aktuelle Diskussionen dazu geben klare Antworten. Hier die Kernstruktur und warum sie so aussehen sollte:

## Die drei Kanäle in `CallToolResult`

| Feld | Sichtbar für Modell? | Zweck |
|---|---|---|
| `content` | **Ja** – Haupt-Input für das Modell | Menschen-/modell-lesbarer Text, Zusammenfassung, Advisory |
| `structuredContent` | **Ja** (wird meist mitgeschickt/narriert) | Maschinenlesbares JSON, validiert gegen `outputSchema` |
| `_meta` | **Nein** – host-/UI-only, für das Modell unsichtbar | Interne Metadaten, Cursor, Cache-Keys |

**Wichtigste Erkenntnis für deinen Fall:** Dein Advisory/Hint gehört **niemals in `_meta`** – das Feld ist explizit für das Modell unsichtbar. Das könnte einer der Gründe sein, warum bisherige Hinweise nicht ankamen, falls du sie dort platziert hattest.

## Empfohlene Struktur für dein Read-Advisory

```json
{
  "content": [
    {
      "type": "text",
      "text": "Hinweis: Diese Datei (src/auth.py) wurde bereits in Turn 4 gelesen und ist seitdem unverändert (Hash: a3f1...). Der Inhalt ist weiterhin oben im Kontext verfügbar – ein erneutes Lesen liefert keine neuen Informationen.\n\n---\n\n[eigentlicher Dateiinhalt oder Diff, je nach recommendation]"
    }
  ],
  "structuredContent": {
    "path": "src/auth.py",
    "recommendation": "skip_reread",
    "reason": "unchanged_since_last_read",
    "contentHash": "a3f1...",
    "lastReadAt": { "turn": 4 },
    "diff": null
  },
  "isError": false
}
```

### Warum diese Kombination:

1. **Advisory zuerst in `content`, als eigener Textblock vor dem eigentlichen Inhalt.** Reihenfolge im Array wird typischerweise sequenziell verarbeitet – der Hinweis soll geprimt sein, bevor das Modell den (redundanten) Inhalt sieht.

2. **Enum-Feld `recommendation` statt Freitext-Warnung.** Aus der TriSeek-Analyse und der MCP-Diskussion #2569 zu "Dynamic next-action hints": Modelle folgen klaren, handlungsleitenden Werten (`skip_reread` / `reread_with_diff` / `reread`) zuverlässiger als beobachtenden Aussagen. Formuliere es wie eine Tool-Response-Konvention, nicht wie eine Ermahnung.

3. **`structuredContent` UND `content` gleichzeitig befüllen**, auch wenn das zu leichter Redundanz führt. Aus der SEP-1624-Diskussion: Manche Clients zeigen dem Modell nur `content`, andere geben `structuredContent` direkt zur Narration weiter (so laut OpenAI Apps SDK und Anthropic-Doku bestätigt – "Treat `structuredContent` as model-visible unless the exact host documents otherwise"). Verlass dich nicht auf nur einen Kanal.

4. **Content-Hash statt reinem Turn-Zähler**, um Staleness zu vermeiden (aus der HATEOAS-Diskussion: `stateHash` verhindert, dass ein Hinweis bei zwischenzeitlicher Änderung fälschlich "skip" empfiehlt).

5. **Hinweis pro Tool-Call anhängen, nicht global injizieren.** Aus derselben Diskussion: *"Appending to result content, not injecting into the system prompt. System prompt injection accumulates across turns — hints from turn 3 are still visible in turn 7 even after the entity has transitioned. Scoping to the result content keeps hints fresh and turn-local."* Das spricht dafür, dass dein `<file_registry>`-Ansatz besser **pro Tool-Response aktuell gehalten** wird, statt als einmalige System-Notiz.

6. **`annotations` auf dem Content-Block nutzen**, um Priorität zu signalisieren (Teil der MCP-Spec seit 2025-06-18):
```json
{
  "type": "text",
  "text": "...",
  "annotations": { "priority": 0.9, "audience": ["assistant"] }
}
```
Das ist zwar in erster Linie ein Client-Rendering-Hinweis, aber falls dein Client/Framework das respektiert, kann es helfen, den Advisory-Block hervorzuheben.

7. **Bei tatsächlicher Änderung: Diff statt Vollinhalt liefern** (`recommendation: "reread_with_diff"` + `diff`-Feld befüllen). Das reduziert nicht nur Tokens, sondern gibt dem Modell aktiv das, was es *eigentlich* wissen will, statt es zu einem kompletten Re-Read zu zwingen.

Willst du, dass ich dir daraus ein konkretes JSON-Schema (`outputSchema`) für den Tool-Definitionsteil entwerfe, das diese Felder validiert?
