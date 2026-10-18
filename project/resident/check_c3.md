Prüfe die Abweichungen und hole die Ausführungen nach.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_c_00_overview.md)

# Abweichungen
C3 ist umgesetzt, aber ich konnte nichts kompilieren oder testen. Mir stand nur die AST-Werkzeugschiene zur Verfügung, ohne Shell. Die fünf neuen Dateien bestehen die Syntaxprüfung, und die Import-Pfade habe ich gegen die Re-Exports geprüft. Bitte lauf `cargo test -p` auf dem Crate, bevor du dich darauf verlässt.

**Was von deinem Plan abweicht**
- **Hub-Anbindung fehlt noch.** Der Hub-`Expander` verlangt fertige `OperationNode`s mit festem Eingabewert. Folge-Ops sollen aber erst beim Abschluss entstehen, und diese Engine-Seite gibt es noch nicht (nur `NoEngine`). Der Planner liefert deshalb `Vec<PlannedOp>` (`OpDesc` plus `materialize`). Ob der Adapter alle Ops auf einmal einreiht oder nur die erste und den Rest über die Engine, musst du entscheiden.
- **`EngineOffers` statt `CapabilitySet`.** Das vorhandene `CapabilitySet` bildet Präzisionsachsen ab, nicht "Engine X bietet Ressource Y". Für die Device-Wahl habe ich deshalb einen eigenen Typ angelegt.
- **Kein `notify`-Watcher.** Hot-Reload läuft nur über `Planner::reload`, den Datei-Watcher habe ich weggelassen.
- **Neue Abhängigkeit `toml = "0.9"`** in `Cargo.toml`. `Cargo.lock` muss einmal aktualisiert werden.