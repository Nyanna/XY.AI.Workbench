# Umsetzungsplan G2 — Index-Schema (Parquet / Arrow)

> Paket **G2** aus `resident_planplan.md`. Quellen: `resident_first.md` (Index), `resident.md`/`resident_index.md` (Persistenz außerhalb der Engine), `context/context_g.md` (§G2). Abhängt von: **B4** (Index-Trait), **G1** (Modellkonfig). Ergebnis: Index-Schema-Spezifikation + Roundtrip-Verifikation.
>
> Basis-Crate: **`parquet` 60.0.0** (gepinnt). **Legende:** ✅ verifiziert · ⚠️ zur Laufzeit prüfen.

---

## 1. Zweck

Konkretes, persistierbares Index-Schema für e5-small-Vektoren: Vektorspalte + Zeilen-/Offset-ID, Payload (Zieldatei + Zeichenoffsets + Checksum) für Rückverweis auf den Originaltext, Schema-Versionierung (Modell-ID, Dim, Pooling, Präfixe, dtype) und mmap-Lesezugriff.

**Vertragslage (✅):** Persistenz **außerhalb** der Engine. Das **Index-Objekt besitzt den Mutex/die Persistenz selbst** (`resident.md`, `resident_index.md`). G2 implementiert ein konkretes `Index` (B4-Trait), nicht den Manager. Modellwechsel = **neue Schema-Version + Re-Embedding**.

---

## 2. Zu implementierende Artefakte

### 2.1 Arrow-Schema — `src/index/schema.rs`
Spalten:
- **`vector`** — Vektordaten, 384×f16. Layout-Entscheidung siehe §3.
- **`entry_id`** — stabile Entry-Identität (B4 `EntryId`).
- **`row`/`offset_id`** — Zeilen-/Offset-ID (Zugriff per Zeilennummer).
- **Payload-Spalten** (§2.2).
- **`stamp`** — monotoner Zähler-Wert pro Entry (B4). **Eigene Spalte**, damit der Scan **keine Vektorseiten einpaged** (harte Vorgabe `resident_index.md`). ⚠️ Prüfen, dass `stamps()` ohne Vektor-Decode iterierbar ist (ggf. separates Row-Group-Layout / separate Datei je nach Reifegrad).

### 2.2 Payload — `src/index/payload.rs`
Pro Entry (für Rückverweis / späteren Cross-Encoder-Scorer):
- `target_file: String` (Zieldatei)
- `char_start: u64`, `char_end: u64` (Zeichenoffsets der Zeile/Chunk)
- `checksum: u64` (Verifizierung, siehe §4)

### 2.3 Header / Schema-Metadaten — `src/index/schema.rs`
In den Parquet-Footer via `WriterPropertiesBuilder::set_key_value_metadata(Vec<KeyValue>)` (`KeyValue { key, value }`). Lesen über `ParquetMetaData::file_metadata().key_value_metadata()`.

### 2.4 Parquet-I/O — `src/index/parquet_io.rs`
Writer + Reader inkl. mmap, implementiert den B4-`Index`-Trait (`stamp`, `stamps`, `read`, `apply`, `changed`, `capabilities`=`[Range]` initial, usw.).

---

## 3. Vektor-Layout-Entscheidung (⚠️ vor Festlegung Roundtrip-Test!)

Drei Optionen aus `context_g.md`:

| Option | Pro | Contra |
|---|---|---|
| **`FixedSizeList<Float16, 384>`** | selbstbeschreibend, interoperabel | arrow-rs schreibt als repeated group (Def/Rep-Levels) → Overhead, **kein** echtes Zero-Copy |
| **`FixedSizeBinary(768)`** (rohe f16-Bytes) | kompakt, schnell | nicht „selbstbeschreibend" → dim/dtype/Endianness in Metadaten nötig |
| **`List<Float16>`** | flexibel | variabler Overhead, langsamer |

**Entscheidungsregel:**
- Interop wichtig (pyarrow/DuckDB/Spark) → **FixedSizeList**.
- Sonst → **FixedSizeBinary(768)** + Schema-Metadaten (dim, dtype, Endianness).

⚠️ Hintergründe: `Float16` → Parquet `FIXED_LEN_BYTE_ARRAY(2)` mit Logical Type `FLOAT16` (Parquet ≥ 2.10); Reifegrad **mittel**, Interop variiert (pyarrow ≥ ~15/16, DuckDB, Spark). **Verpflichtend:** Mini-Roundtrip-Test (Write→Read) in der **gepinnten** `parquet`/`arrow`-Version, bevor das Layout final festgelegt wird.

**Brute-Force-Scan-Alternative (Architekturoption, nicht belegt):** Vektoren zusätzlich als flaches binäres Sidecar (echtes mmap, Zero-Copy, SIMD-Dot). Nur erwägen, wenn Parquet-Decode beim Scan zum Engpass wird; als Option dokumentieren, nicht initial bauen.

---

## 4. Checksums (Payload-Verifizierung) — Frage 14

- `crc32fast` ist optional im `parquet`-Crate (⚠️ dort Page-CRC, nicht Payload).
- **Empfehlung:** Für „Zieldatei + Zeichenoffsets pro Zeile" genügt **xxh3-64/128** oder **CRC32C**. Kleine Payloads → Geschwindigkeit irrelevant, **64-Bit-Hash genügt**.
- Nur bei Sicherheitsanforderung (Manipulationsschutz) → **BLAKE3** (kryptografisch, SIMD/MT) oder SHA-256 (`ring` ist bereits optional im Dependency-Baum).
- **Entscheidung:** xxh3-64 für `payload.checksum`. Bei Mismatch → Fehler `ChecksumMismatch` (Q1), kein stiller Fallback.

---

## 5. Schema-Versionierung (Header-Keys) — Frage 15

Footer-Metadaten (strikt beim Öffnen prüfen; Mismatch → `IncompatibleSchema` → Re-Embedding):

| Key | Wert (aus G1) |
|---|---|
| `resident.schema_version` | z. B. `1` |
| `embedding.model_id` | `intfloat/multilingual-e5-small` |
| `embedding.model_revision` | HF-Commit-Hash |
| `embedding.dim` | `384` |
| `embedding.pooling` | `mean_masked` |
| `embedding.normalize` | `l2` |
| `embedding.prefix_query` | `query: ` |
| `embedding.prefix_passage` | `passage: ` |
| `embedding.storage_dtype` | `f16` |
| `embedding.compute_dtype` | `f32`/`f16` (gemischt dokumentieren) |
| `embedding.max_seq_len` | `512` |
| `tokenizer.sha` | Hash von `tokenizer.json` |

⚠️ ArrowWriter schreibt auch `Schema::with_metadata` in den Footer (Verhalten aus Erfahrung, Detail nicht in Docs geprüft) — Roundtrip bestätigen.

---

## 6. Writer-Einstellungen (✅ `WriterPropertiesBuilder`)

- `set_column_dictionary_enabled(false)` für die **Vektorspalte** (Dictionary bringt bei Floats nichts).
- `set_compression` / `set_column_compression` (Default UNCOMPRESSED) — bewusst wählen; für FixedLenByteArray + PLAIN ist Decode-Aufwand gering.
- `set_max_row_group_row_count` — Row-Group-Größe nach Zugriffsmuster.
- `set_statistics_enabled` — für Range/Pruning.
- `set_offset_index_disabled(false)` (Default; Offset-Index hilft Zugriff per Zeilennummer — **nicht** deaktivieren).
- `set_data_page_row_count_limit` (Default 20 000).
- `RowNumber`-Virtual-Column (experimental) für Zeilenindex beim Lesen erwägen.
- `ARROW:schema`-Meta (`ARROW_SCHEMA_META_KEY`) stellt Arrow-Typen beim Rücklesen exakt wieder her (Reader fällt sonst auf Inferenz zurück) — sicherstellen, dass ArrowWriter es schreibt.

---

## 7. mmap-Zugriff — Frage 13 (⚠️ praktisch prüfen)

- Reader arbeitet über `ChunkReader` (impl. für `File` und `bytes::Bytes`).
- **Weg:** Datei mit `memmap2` mappen → als `Bytes` (`Bytes::from_owner(mmap)`, `bytes ≥ 1.9`) an `ParquetRecordBatchReaderBuilder::try_new`.
- **Kein echtes Zero-Copy** auf Vektoren: Parquet dekodiert, Arrow-Arrays werden allokiert/kopiert (bei UNCOMPRESSED + PLAIN FixedLenByteArray gering).
- Random-Access: Footer zuerst, Spaltenchunks nur bei Zugriff; `RowSelection`/Offset-Index lädt nur nötige Pages. `madvise(Random/WillNeed)` über `memmap2::Advice`.
- **⚠️ memmap2-Risiko:** Datei während Mapping **nicht** kürzen/überschreiben (UB/SIGBUS) → **Index immer als neue Datei schreiben + atomisch umbenennen** (passt zur Persistenz-Vorgabe; sichert auch CoW-Disziplin des Managers).

---

## 8. Fehlerbehandlung (Q1) & Logging (Q2)

- **Fehler:** `IncompatibleSchema { expected, found }` (Header-Mismatch), `ChecksumMismatch { entry }`, Lade-/I-O-Fehler auf B4/`IndexUnavailable` abbilden. Kein stilles Ausweichen.
- **Logging:** Span pro Öffnen/Schreiben mit `schema_version`, `model_id`, `model_revision`, `row_groups`, `checksum_ok`, `access=mmap|file`.

---

## 9. Teststrategie (Q3) & Verifikation (⚠️)

1. **Roundtrip** `FixedSizeList<Float16,384>` (bzw. FixedSizeBinary) Write→Read in gepinnter Version; Werte bit-/toleranzgleich. Optional pyarrow/DuckDB-Interop.
2. **Header-Roundtrip:** alle Keys (§5) zurücklesbar; Mismatch löst `IncompatibleSchema` aus.
3. **Payload/Checksum:** xxh3-64 Roundtrip; manipulierter Payload → `ChecksumMismatch`.
4. **Stamp-Spalte:** `stamps()` iterierbar **ohne** Vektor-Decode (keine Vektorseiten eingepaged).
5. **mmap-Weg:** `Bytes::from_owner(mmap)` praktisch; atomisches Rename beim Schreiben.
6. **B4-Vertrag:** `apply` gibt eigenen Stamp zurück; Schreibreihenfolge Daten→Stamp→Signal; `changed()`-Watch feuert.

---

## 10. Definition of Done

- Vektor-Layout per Roundtrip-Test final entschieden und dokumentiert (FixedSizeList **oder** FixedSizeBinary + Metadaten).
- `Index`-Trait (B4) für den Parquet-Index implementiert; `stamps()` ohne Vektor-Decode.
- Payload (Zieldatei + Zeichenoffsets + xxh3-Checksum) schreib-/prüfbar.
- Header-Keys (§5) aus G1 übernommen, strikt geprüft; Mismatch → `IncompatibleSchema`.
- mmap-Lesepfad praktisch verifiziert; Schreiben via neue Datei + atomisches Rename.
- Q1/Q2/Q3 erfüllt; No-GPU-Tests grün.
