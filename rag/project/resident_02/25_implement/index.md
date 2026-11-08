
Zielverzeichnis: `/home/user/xyan/xy.ai.workbench/rag/src/resident` entsprechend dann `/home/user/xyan/xy.ai.workbench/rag/src/resident/l130...`
Modell (multilingual-e5-small): `/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small`
Wichtige Datei: `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`
Wurzel ist: `/home/user/xyan/xy.ai.workbench/rag/src/lib.rs`. Sie enthält `pub mod resident;` bereits.

Umgebung:
- nvcc: /usr/local/cuda-13.3/bin/nvcc (CUDA 13.3, V13.3.73)
- GPU: NVIDIA GeForce GTX 1660 SUPER, compute capability 7.5 (sm_75), Treiber 610.57.04
- Ziel-Architektur: sm_75
- Von nvcc unterstützte Architekturen: compute_75, 80, 86, 87, 88, 89, 90, 100, 103, 110, 120, 121

Regeln:
- Projektdokumente und Code-Kommentare ausschließlich in Englisch.
- Alle Modul-, Datei-, Typ-, Struktur-, Muster- und Funktionsnamen sind englisch und von dir gewählt.
- Kommentare minimal halten; keine Referenzen auf Planelemente und Plannummern in Kommentaren.
- Nutze Bash nicht für Retrieval Tasks
- Prüfe die tatsächliche Umsetzung referenzierter Elemente
- Die Tests eines Moduls liegen in `<ebene>/test/<name>.rs`. Am Ende der Quelldatei steht:
```rust
#[cfg(test)]
#[path = "test/<name>.rs"]
mod tests;
```

Vordefinierte Tool Kommandos ("run_command")
- "runTests": Führt einen Cargo Testlauf durch.
- "read.Cargo.toml": Gibt den Inhalt der `Cargo.toml` wieder.

Konflikte und Probleme:
- Zuerst den Kontext auf bereits erledigte Specs erweitern
- Dann den Kontext auf zukünftige Konsumenten erweitern

Festgelegte Verzeichnisstruktur für Module:
```yaml
[include](/home/user/xyan/xy.ai.workbench/rag/project/resident_02/27.tree.yml)
```
- Direkte Kindbeziehung von Modulen sind in Unterverzeichnissen zu erstellen
