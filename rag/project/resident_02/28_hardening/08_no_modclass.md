Schreibe einen Archunit/Hygiene Test. Dann korrigiere Schrittweise die Fehlschläge.

- Es sollen keine `mod.rs` Dateien verwendet werden deren Inhalt nicht klar ist. 
- Non-leaf modules sollen nur Moduldeklarationen und Re-Exports enthalten, keine Implementierungen.
- Außerdem sollen sie einen beschreibenden Namen haben und nicht generisch `mod.rs`/`deps.rs` heißen ohne das der Inhalt zumindest entfernt abgeleitet werden kann.

[include](/home/user/xyan/xy.ai.workbench/rag/project/resident_02/28_hardening/index.md)