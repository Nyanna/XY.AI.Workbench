
Zielverzeichnis: `/home/user/xyan/xy.ai.workbench/ast-engines/rustir`
Hygiene/Arch Tests: `/home/user/xyan/xy.ai.workbench/ast-engines/rustir/tests`

Regeln:
- Projektdokumente und Code-Kommentare ausschließlich in Englisch.
- Kommentare minimal halten; keine Referenzen auf Planelemente und Plannummern oder konkrete Anforderungen in Kommentaren.
- Direkte Kindbeziehung von Modulen sind in Unterverzeichnissen zu erstellen
- Abhängigkeiten nach oben sind nicht erlaubt
- Tests dürfen Contracts niemals über Loging verifizieren
- Direkte `Use` Pfade keine unnötigen Alias Ketten
- Non-leaf modules sollen nur Moduldeklarationen und Re-Exports enthalten, keine Implementierungen
- Die Tests eines Moduls liegen in `<ebene>/test/<name>.rs`. Am Ende der Quelldatei steht:
```rust
#[cfg(test)]
#[path = "test/<name>.rs"]
mod tests;
```

Vordefinierte Tool Kommandos ("run_command")
- "runTests": Führt einen Cargo Testlauf durch.
- "read.Cargo.toml": Gibt den Inhalt der `Cargo.toml` wieder.
- "renderResident": Führt das Rendering durch den IR durch
