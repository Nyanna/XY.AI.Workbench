## prettyplease 0.3.0 – öffentliche API

Minimaler Pretty-Printer, der einen `syn`-Syntaxbaum in einen formatierten Rust-Quelltext-`String` umwandelt. Gedacht für generierten Code (z. B. bindgen- oder cargo-expand-Ausgaben), nicht für handgepflegten Code.

### Crate-Metadaten

| Eigenschaft             | Wert                                                                                                  |
| ----------------------- | ----------------------------------------------------------------------------------------------------- |
| Version                 | 0.3.0 (veröffentlicht 29.09.2026)                                                                     |
| Lizenz                  | MIT OR Apache-2.0                                                                                     |
| Abhängigkeiten (normal) | `proc-macro2 ^1.0.80`, `syn ^3`                                                                       |
| Feature-Flags           | keine dokumentiert; siehe [docs.rs-Features-Seite](https://docs.rs/crate/prettyplease/0.3.0/features) |

### Öffentliche Items

Die Crate exportiert genau **eine** Funktion. Es gibt keine öffentlichen Module, Typen, Traits oder Makros.

#### Funktionen



rust

```rust
pub fn unparse(file: &syn::File) -> String
```

* **Eingabe:** `&syn::File` (aus `syn` 3.x)
* **Ausgabe:** formatierter Quelltext als `String`
* **Konfiguration:** keine (Zeilenbreite und Stil sind fest)

### Verwendung



toml

```toml
[dependencies]
prettyplease = "0.3"
syn = { version = "3", default-features = false, features = ["full", "parsing"] }
```



rust

```rust
fn main() {
    let syntax_tree = syn::parse_file("fn f(){let x=1;}").unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    print!("{}", formatted);
}
```

### Hinweise

* **Breaking Change gegenüber 0.2.x:** Die Abhängigkeit wechselt auf `syn ^3`. `syn::File` muss daher aus `syn` 3 stammen, sonst kommt es zu Typkonflikten.
* Der Output ist nicht konfigurierbar.
* Der Printer bricht nie ab. Er liefert immer eine "gut genug" formatierte Ausgabe, anders als rustfmt bei schwer formatierbarem Code.
