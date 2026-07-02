Genau – das ist konzeptionell sauber. Die Logik wäre:

### Der Ansatz

```
Session läuft...
    Turn 3: Snapshot der JSONL machen  →  snapshot_t3.jsonl
    Turn 4, 5, 6...  (läuft schief)
    Rewind gewünscht:
        session.jsonl ← snapshot_t3.jsonl wiederherstellen
        claude --resume <session-id>
```

Da der Conversation-Prefix bis Turn 3 unverändert ist, greift der Cache – **keine Re-Processing-Kosten** für die History.

### Voraussetzungen für Cache-Hit

| Bedingung             | Detail                                          |
| --------------------- | ----------------------------------------------- |
| TTL aktiv             | 5 min (Standard) oder 1 h (höhere Tiers)        |
| Prefix identisch      | JSONL exakt wie beim letzten Cache-Write        |
| Gleiches Modell       | Kein Modellwechsel zwischen Snapshot und Resume |
| CLAUDE.md unverändert | Sonst Cache-Invalidierung                       |

### Praktische Überlegung

Die Session-JSONL liegt unter `~/.claude/projects/<pfad-hash>/`. Ein Snapshot ist ein simples `cp`. Das Restore vor `--resume` ist deterministisch – **kein interner State ausserhalb dieser Datei**, der inkonsistent werden könnte (sofern keine File-Edits rückgängig gemacht werden müssen, die Claude auf Disk gemacht hat).

**Den Haken gibt es trotzdem:** Die von Claude editierten Dateien im Projekt sind nicht Teil der JSONL – die müsstest du separat snapshotten (z. B. via `git stash` / Commit), sonst divergiert Conversation-State und Filesystem-State.