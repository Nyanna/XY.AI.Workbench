Implementiere einen Trigramm basierten Layer.

- Beispiel: `/home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs`, `/home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs`
- Dateien über Maximalgröße 5 MB werden nicht durchsucht
- Für die rekursive Suche wird ein optionales "directory" Field aus der Query verwendet (Fallback auf Document Root bereits vorhanden)
- Der Layer verwendet das "Query" Feld der Query
	- Die Query wird normalisiert und zusätzlich in eine Trigramm Signatur umgewandelt
	- Die Signatur bestimmt die zu durchsuchenden Dateien und der Normalisierte Spiegel identifiziert die Trefferzeilen
	- Im Query result Objekt wird ein Eintrag mit dem Feld "File" (relativer Dateipfad zum Document Root) abgelegt
		- Es wird eine assoziative sortierte Liste erzeugt ("Lines") die Zeilennummer zum Text der matchenden Zeile abbildet
		- Mehrere Treffer in derselben Zeile bilden nur einen Eintrag
- Der Layer verwendet Multithreading (`/home/user/xyan/xy.ai.workbench/rag/src/core/executor.rs`), jeweils einen Task pro Dateisuche(Suche in der Datei)
	- nach 90% der Maximallaufzeit wird keine weitere Dateisuche gestartet
	- bei Erreichen der Maximallaufzeit (1 Minute) werden bestehende Dateisuchen abgebrochen
- Ein eintreffende Query startet einen Suchlauf
	- Ein Suchlauf präpariert die Query und gleicht diese auf Basis eines rekursives Laufs über `/home/user/xyan/xy.ai.workbench/rag/src/layers/dir_cache.rs` mit den gespeicherten Signaturen ab.
		- Fehlt eine Signatur oder ist sie veraltet, wird die Datei neu Indiziert
		- Trifft die Signatur ausreichend stark zu wird eine Dateisuche gestartet

# Vocabular

Das Trigram-Vokabular wird lazy generiert und gespeichert oder aus einer vorher gespeicherten Datei geladen.

- Trigramm ID zufällig verteilen und als Datei persistieren

# Normalisierung

- Zuerst unicode-normalization: NFC/NFD/NFKC/NFKD.
- alle nicht englischen alphanumerischen Zeichen werden entweder Leerzeichen (Code und Syntax) oder umgewandelt (Umlaute aus Sprachen)
- Newline bleiben erhalten und garantieren ein Mapping auf die Datei zurück
- Alles CamelCase wird gesplittet und anschließend lowercase
- Multiple Leerzeichen werden zu einem zusammen gezogen und sind der Vokabulartrenner.
	- Zeilen die nur aus Whitespace bestehen werden auf das Newline kollabiert 

Die normalisierte Form wird in einer gespiegelten Verzeichnisstruktur unterhalb des Storage Verzeichnisses gespeichert und für die Zeilenassoziation des Results verwendet..  

# Signatur

- Dann Zerlegung in Trigramme
	- Grenzmarker statt Mindestlänge. Pro Token ein Marker _ vorn und hinten, dann gleitende Trigramme: go → _go, go_; a → _a_; user → _us, use, ser, er_.
	- Ein Token der Länge L liefert genau L Gramme, auch für 1–2 Zeichen.
	- Wortanfang und -ende werden unterscheidbar (_us ≠ us_), das erhöht die Selektivität.
- übersetze die Zeile in Trigramme und entferne alle doppelten.
- dann alle trigramme in der Datei zählen und die häufigsten X Prozent (10%) entfernen, wenn sie über einem Threshold liegen (10).


# Index

Diese Signatur im Index des Layers speichern mit Dateipfad und mtime.

# Suchlauf 

- wenn datei im index gefunden aber nicht mehr auf der platte -> aus  Index entfernen
- Wenn index mit MTime veraltet, neu indizieren

# quantisierter Radix-4-Trie (Speicherform) und Bitmasken-Darstellung (Memory)

## Ziel
Implementiere eine Datenstruktur für **Mengen ganzzahliger Schlüssel** (Trigramm-IDs) mit
1. einer **kompakten, serialisierbaren Form** (Pre-Order-Radix-4-Trie, quantisiert) und
2. einer **In-Memory-Form als flache Bitmaske** (`u64`-Wörter), mit der ein Abgleich gegen eine Anfrage nur aus `AND` + Popcount besteht.

Zweck: Pro Datei wird gespeichert, welche Gramme vorkommen. Zur Anfragezeit wird gezählt, wie viele Anfragegramme in der Datei vorkommen (`≥ T` → Datei wird durchsucht).
Die Eingabe sind fertige Schlüssel (`u32`).

## Parameter (`TrieParams`)
- `key_bits` (Standard 16): Breite des Schlüssels.
- `quant_bits` q (Standard 0): Zahl der entfernten unteren Schlüsselbits. Zelle = `key >> q`. Auflösung `b = key_bits − q`.
- `root_bits` r (Standard 4): Die obersten r Bit der Zelle sind eine **direkte Wurzelmaske** mit `2^r` Flags.
- Darunter folgen `L = (b − r) / 2` Radix-4-Ebenen.
- Gültig nur wenn: `1 ≤ r ≤ min(8, b)`, `(b − r)` gerade, `b ≤ 24`. Sonst `Err(InvalidParams)`.

Quantisierung ist einseitig: Mehrere Schlüssel teilen sich eine Zelle. Anwesende Schlüssel bleiben anwesend, es entstehen nur Falsch-Positive.

## Kompaktform (Pre-Order-Bitstrom)
- Bitreihenfolge: **LSB-first**. Stromposition i = Bit `i % 8` von Byte `i / 8`. Padding-Bits am Ende sind 0.
- Eine **Gruppe** besteht aus Flags: Wurzelgruppe `2^r` Flags, alle anderen Gruppen 4 Flags. Flag j gehört zum Präfixwert j (bei 4er-Gruppen: die nächsten 2 Schlüsselbits, MSB zuerst).
- Aufbau: Wurzelgruppe. Danach für jedes gesetzte Flag in aufsteigender Reihenfolge **sofort** dessen Untergruppe (rekursiv, Pre-Order), bevor das nächste Geschwister kommt.
- Die letzte Ebene (Gruppen auf Tiefe L) trägt direkt die Zellenflags. Darunter gibt es nichts mehr.
- Gesetzte Flags haben immer eine Untergruppe mit mindestens einem gesetzten Flag (Kanonik). Die Wurzelgruppe darf nur bei leerer Menge ganz null sein.
- Die Ausgabe ist deterministisch: unabhängig von Reihenfolge und Duplikaten der Eingabe, byte-identisch.

### Container
Header (little endian): `magic "GTRI"` (4 Byte), `version u8 = 1`, `key_bits u8`, `quant_bits u8`, `root_bits u8`, `payload_bits u32`, danach `ceil(payload_bits/8)` Payload-Bytes.
Zusätzlich `to_payload()` / `from_payload(params, bytes, bits)` ohne Header, für Verwendung mit gemeinsamen Parametern.

### Testvektor (verbindlich)
`key_bits=6, q=0, r=2` (→ b=6, L=2), Schlüssel `{13, 14, 48}`:
- Gruppen in Reihenfolge: `1001 | 0001 | 0110 | 1000 | 1000` (Flag 0 zuerst), 20 Bit.
- Payload-Bytes: `0x89 0x16 0x01`.
- Zellen-Bitmaske: ein Wort `0x0001_0000_0000_6000` (Bits 13, 14, 48 gesetzt).

## In-Memory-Form
- `CellMask { words: Box<[u64]> }` mit `2^b` Bit, LSB-first: Zelle c = Bit `c % 64` in Wort `c / 64`. Für `b < 6` genau ein Wort, ungenutzte Bits 0.
- Die Struktur wird beim Laden aus der Kompaktform in einem Durchlauf aufgeklappt. Dabei werden **nur Blattzellen** gesetzt (innere Flags nicht).

## Anfrage (`QueryMask`)
- Gebaut aus den Anfrageschlüsseln mit denselben Parametern.
- Speichert: `n_keys` (verschiedene Anfrageschlüssel), `n_cells` (verschiedene gesetzte Zellen), und eine **dünne Wortliste** `Vec<(word_idx, mask)>` aufsteigend nach Index. Außerdem ein Suffix-Array der Popcounts dieser Masken.
- `corrected_threshold(T) = T.saturating_sub(n_keys − n_cells)`. Das gleicht Kollisionen innerhalb der Anfrage aus. Dateiseitige Kollisionen brauchen keine Korrektur.

## API (Vorschlag)
- `TrieParams::new(key_bits, quant_bits, root_bits) -> Result<_, Error>`
- `CompactTrie::build(params, keys: impl IntoIterator<Item=u32>) -> CompactTrie`
- `CompactTrie::{to_bytes, from_bytes, to_payload, from_payload, expand() -> CellMask}`
- `CellMask::{from_keys, contains_key, count_ones, union_with}`
- `QueryMask::{new, corrected_threshold}`
- `matched_cells(file: &CellMask, q: &QueryMask) -> u32`: Summe über die dünne Wortliste von `popcount(file.words[i] & mask)`.
- `matches_at_least(file, q, t_cells) -> bool` mit Frühabbruch: `true`, sobald die Summe ≥ t ist. `false`, sobald Summe + Rest-Popcount (Suffix-Array) < t.
- Optional (Stretch): `MaskTable` mit Wörtern aller Dateien in einem zusammenhängenden `Vec<u64>` (fester Stride) und `scan(&QueryMask, t) -> Vec<u32>`, optional parallel per `rayon` hinter einem Feature-Flag.

## Anforderungen
- Stabiles Rust, `#![forbid(unsafe_code)]`, keine Rekursion beim Decodieren (expliziter Stack), `u64::count_ones()` für Popcount.
- Fehler statt Panic bei: falscher Magic/Version, ungültigen Parametern, abgeschnittenem Strom, Überhang nach dem Strom, Padding ≠ 0, leerer Untergruppe, Schlüssel ≥ `2^key_bits`.
- Zähler laufen nie über innere Ebenen: nur Zellen werden gezählt.
- Abhängigkeiten minimal (`thiserror` optional). Dev: `proptest`, `criterion`.

## Tests
- Testvektor oben (Bytes und Maske exakt).
- Property-Tests: `expand(build(keys))` ≡ `CellMask::from_keys(keys)`; `contains_key(k)` für alle `k ∈ keys`; Gleichheit mit naivem `HashSet`-Zählen für q = 0; für q > 0 gilt `matched_cells ≥` echte Trefferzahl.
- Randfälle: leere Menge, ein Schlüssel, alle Schlüssel (volle Menge), Schlüssel 0 und `2^key_bits − 1`, Duplikate, jede gültige (r, q)-Kombination.
- Korrupte Eingaben: Abschneiden an jeder Byte-Grenze, gekippte Bits → immer `Err`, nie Panic.
- Reihenfolge-Invarianz der Bytes (Eingabe gemischt).

## Benchmarks (`criterion`)
- Aufklappen: `b = 16`, n ∈ {100, 500, 5000}.
- Abgleich: 100 000 Masken zu 8 KB (b = 16) und zu 128 B (b = 10) gegen Anfragen mit 20 bzw. 100 Schlüsseln. Ausgabe: ns pro Datei, getrennt für `matched_cells` und `matches_at_least`.