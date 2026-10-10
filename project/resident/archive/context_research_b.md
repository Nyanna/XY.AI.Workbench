Lies zunächst alle Dokumente in `/home/user/xyan/xy.ai.workbench/project/resident` ein.

```json
{
  "id" : "call_fecd700b-dd1a-44b0-aa8a-cdcc668509d5",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
```

Erstelle eine Liste aller in `resident_planplan.md` für Gruppe B zu beantwortenden Fragen für eine Web-Recherche. Ein anderer Agent wird diese Fragen recherchieren, beantworten und ein Markdown Kontext File als Ergebnis liefern.
SystemInit:  id=42f2572a-c282-4974-a3fa-42d4eaa7d270, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich lese zunächst alle Dokumente im angegebenen Verzeichnis ein.
Control Request:
```yaml
id: 0f9c-4
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  reason: "Einlesen aller Projektdokumente im Verzeichnis 'resident', um den Kontext zu erfassen und insbesondere die für Gruppe B zu beantwortenden Fragen aus resident_planplan.md zu extrahieren."
```
/answer 0f9c-4 allow
Result Stats: id=42f2572a-c282-4974-a3fa-42d4eaa7d270, total: 62832, in: 119, out: 10031, reason: 4871, read: 1503, write: 52682
ReasoningToken: 107
ReasoningToken: 4764
Ich habe alle acht Dokumente eingelesen. Nachfolgend die Fragenliste für die **Web-Recherche zu Gruppe B** (Kern-Datentypen & Verträge, Pakete B1–B6). Sie ist aus den „Zu klären"-Punkten und den beschriebenen Datentypen/Verträgen abgeleitet und so formuliert, dass ein Recherche-Agent sie extern beantworten und als Markdown-Kontextdatei liefern kann.

---

# Web-Recherche-Fragen – Gruppe B (Kern-Datentypen & Verträge)

**Zweck der Recherche:** Externe/belegbare Grundlagen (Crate-Verhalten, nebenläufige Muster, Rust-Idiome, CUDA/candle-Fakten) für die spätere Spezifikation der Pakete B1–B6 der „Resident"-Engine sammeln. Stack-Kontext: `arc-swap`, `tokio`, `crossbeam`, `parking_lot`, `candle`/`candle-core`, `cudarc`, `safetensors`, `tokenizers`, `parquet`/Arrow, `thiserror`, `tracing`. Zielhardware: GTX 1660 (Turing, sm75), 6 GB VRAM.

**Konvention je Antwort:** Kernaussage, Quelle(n)/Links, Code-/API-Beispiel wenn möglich, Versionsstand, Risiken/Fallstricke.

---

## B1 · Queue & Task-Primitive
Quellen im Projekt: `resident.md` (Queue-Logik), `resident_processor.md` (Queues). Zu klären: Payload-Besitzübergang, Unlink-Semantik, shared-Queue.

1. **arc-swap:** Wie implementiert man mit `ArcSwapOption<Node>` eine lock-freie, einfach verkettete Liste mit Append (CAS auf `tail.next`) und Unlink (CAS auf `prev.next`)? Welche API (`compare_and_swap`, `rcu`, `store`, `load`) eignet sich, und ist der Link-CAS ohne `unsafe` realisierbar?
2. **Speicherfreigabe:** Verhindert `Arc<Node>` pro Durchläufer (statt `crossbeam-epoch`/Hazard Pointers) das Use-after-free vollständig? Welche ABA-/Reclamation-Fallstricke bleiben bei lock-freien Singly-Linked-Lists mit reiner `Arc`-Referenzzählung?
3. **tokio::sync::Notify:** Genaue Semantik von `notify_waiters()` vs. `notify_one()`; korrekte Reihenfolge `notified().enable()` → `state.load()` → warten; unter welchen Bedingungen gehen Wakeups verloren bzw. werden zusammengefasst?
4. **Set-once-Ergebnis:** Vergleich `std::sync::OnceLock` vs. `once_cell::sync::OnceCell` vs. `tokio::sync::OnceCell` für `result: OnceCell<Result<Output, Error>>` in nebenläufigem Set-once-Kontext (Blocking vs. async, Mehrfach-Set).
5. **Zustandsautomat (AtomicU8):** Empfohlene CAS-Muster und Memory-Ordering für `Queued|Taken|Done|Cancelled` (`compare_exchange`/`compare_exchange_weak`, Acquire/Release/AcqRel) für `claim` (Queued→Taken), Fertigstellung (Taken→Done) und `cancel` (Queued→Cancelled).
6. **Iterativer Drop:** Idiomatisches Rust-Muster, um Stack-Overflow beim rekursiven `Drop` langer `Arc`-Ketten zu vermeiden (`while let Some(n) = cur.take_next()`); belegte Beispiele/Blogposts.
7. **Payload-Besitzübergang:** Muster und Fallstricke für `Mutex<Option<Data>>` beim Claim (`take()` unter Lock); Eignung `parking_lot::Mutex` vs. `std::sync::Mutex` in einem lock-freien Umfeld (Poisoning, Fairness, Overhead).

## B2 · Region / Layout / Speicher-Modell
Quellen: `resident_subengine.md` §4.2. Zu klären: `gen`-Entwertung nach Evict/Reload, No-Tensor/No-Pointer-Invariante.

8. **Generational Handles:** Etablierte Rust-Ansätze/Crates für Handle-Entwertung per Generationszähler (`slotmap`, `generational-arena`, `thunderdome`); wie werden Stale-Handle-Checks (`gen`-Vergleich) performant umgesetzt?
9. **Arena-Allokator:** Bewährte Muster/Crates für Offset-basierte Arenen mit Free-List (Handles statt Roh-Pointer, Offsets als `u64`); Beispiele aus GPU-/Game-Engines in Rust.
10. **candle-DTypes:** Welche `DType`-Werte unterstützt candle aktuell (f32, f16, bf16, u8, u32, i64, …)? Wie werden `shape`/`strides` in candle modelliert — als Referenz für ein eigenes `Layout { shape, strides, dtype }`?
11. **DType-Mapping:** Wie gestaltet man ein eigenes DType-Enum, das sauber auf candle- und safetensors-DTypes abbildet (Policy GPU f16 / CPU f32), ohne Tensor-/Pointer-Typen in öffentlichen Signaturen zu leaken?

## B3 · Operation-/Op-Modell
Quellen: `resident_planner.md` (Ausgabe), `resident_subengine.md` (Task ≠ Op). Zu klären: Handle-/Lease-Repräsentation, Affinitäts-Erweiterungspunkt.

12. **Typisierte Op/IR:** Entwurfsmuster in Rust für eine Operation mit unveränderlichen `params` und einer `requires()`-Methode zum Ressourcen-/Capability-Matching (Beispiele aus Rust-Compilern/ML-Graphen/Schedulern).
13. **Host/Device-Value:** Muster für enum-basierte Datenübergabe `Value::Host(data) | Value::Device(handle)` in gemischten CPU/GPU-Pipelines — Beispiele aus candle, `burn`, `wgpu` oder vergleichbaren Rust-Stacks.
14. **Lease/Handle:** RAII-basierte Lease-Muster vs. manuelle Use-Zähler für Device-Ressourcen (`Region` mit `gen`); wie wird der Erweiterungspunkt „Affinität an ein Handle" in einem `requires()`-ähnlichen Prädikat zukunftssicher vorgesehen?

## B4 · Index-Trait & -Vertrag
Quellen: `resident_index.md` (Index-Vertrag, Scope). Zu klären: Fallback-Stamp (size+mtime→Hash, Racy), Stamp-Spalte ohne Vektorseiten, Mehrschreiber-Serialisierung.

15. **tokio::sync::watch\<u64\>:** Semantik als Level-Signal (nur letzter Wert, pro-Receiver-Zustand, Zusammenfassung mehrerer Updates); korrekte Nutzung von `borrow_and_update`/`changed()`; wann gehen Zwischenwerte verloren (für zustandsbasierten Abgleich akzeptabel)?
16. **mtime-Racy-Problem:** Wie lösen etablierte Tools (git, rsync, make, Bazel) das Problem „mtime innerhalb der Zeitauflösung = unsicher"? Welche Zeitauflösung liefern moderne Dateisysteme/`stat` (ext4, NTFS, APFS), und wann ist der Content-Hash-Fallback nötig?
17. **Content-Hash-Fallback:** Welche schnellen, nicht-kryptografischen Hashes (xxHash/xxh3, `blake3`, `ahash`) eignen sich als Fallback-Stamp für Entries; Durchsatz und Crate-Reife?
18. **Monotone Zähler/ABA:** Muster für einen globalen, monotonen Stamp-Zähler (`AtomicU64::fetch_add`), der auch über Entfernen+Wiederanlegen derselben ID eindeutig bleibt; Vergleich nur per `!=`.
19. **Parquet/Arrow Column-Layout:** Wie legt man eine Stamp-Spalte so getrennt von Vektorspalten (`FixedSizeList<f16,384>`) an, dass ein Scan der Stamps keine Vektor-Pages einpaged (Column-Pruning, Row-Groups, Page-Index, mmap-Zugriff)? Welche Rust-Crate(s) (`arrow`, `parquet`) bieten das?
20. **Mehrschreiber-Serialisierung:** Muster für einen serialisierten `apply`-Pfad (Last-Writer-Wins) mit Rückgabe des eigenen Write-Stamps; Optionen (Single-Writer-Task, `Mutex`, Command-Channel).

## B5 · Kernel-Abstraktion
Quellen: `resident_index.md` (Kernel-Schnittstelle), `resident_subengine.md` §4.5. Zu klären: Adress-/Offset-Agnostik, Rückabbildung-Zuständigkeit.

21. **cudarc-Launch-Abstraktion:** Wie launcht `cudarc` Kernel (Device-Pointer, `LaunchConfig`, Streams)? Wie entwirft man ein Rust-Trait für den Kernel (Batch-Start/-Ende, Ergebnisse, geschriebene Bereiche, Fortschrittszähler), das real (cudarc) und als Mock implementierbar ist?
22. **Offset-Tabellen/Prefix-Summen:** Effiziente Konstruktion von `Variable { offsets }`-Layouts für variabel lange Entries und Rückabbildung Adresse→Entry per Binärsuche/Intervall-Map; Rust-Crates/Muster.
23. **Mock-Kernel für Tests:** Bewährte Muster für No-GPU-Mocks hinter einem Kernel-Trait (deterministische Ergebnisse, simulierte „geschriebene Bereiche", Fortschrittszähler) zur GPU-freien Testbarkeit.

## B6 · CapabilitySet & Profile-Typen
Quellen: `resident_precision.md` (Capability-Listen, Profilbildung). Zu klären: Datentyp der Achsen, Erweiterbarkeit ohne Fassaden-/Task-Änderung.

24. **Hardware-Präzisionspfade:** Welche Rechenpfade bietet die GTX 1660 / Turing (sm75): fp16-Compute, `dp4a`/int8, fehlende Tensor Cores (TU116)? Was davon unterstützen candle und cudarc konkret?
25. **Erweiterbare Capability-Modellierung:** Rust-Datentyp-Muster für „Achsen mit Wertelisten" ohne festes Enum (z. B. `HashMap<Axis, Vec<Value>>`, typisierte Achsen, `enum_dispatch`), die ohne Fassaden-/Task-Änderung erweiterbar bleiben.
26. **Pareto-Front & Profil-Position:** Datentypen/Algorithmen zur Repräsentation einer Pareto-Front (Zeit × Abweichung) und zur Berechnung einer normierten Position `p ∈ [0,1]` per kumulierter Bogenlänge; Crates/Referenzimplementierungen.

---

## Hinweise zum Zuschnitt

- **Fokus auf extern Recherchierbares:** Rein interne Designentscheidungen (z. B. exakte Feldnamen, finale Enum-Varianten) sind nicht Teil der Recherche, sondern der anschließenden Spezifikation; die Fragen liefern dafür die *Grundlagen/Belege*.
- **Nicht in Gruppe B enthalten:** Versions-/Kompatibilitätsprüfung der Crates gehört zu Paket **A3** (nicht hier); konkrete e5-small-Modell-/Index-Verifikation zu **G1/G2**; die Kalibrierungs-*Logik* zu **F2–F5** (B6 deckt nur die *Typen* ab).
- **Erwartetes Ergebnis des Recherche-Agenten:** eine Markdown-Kontextdatei, gegliedert nach B1–B6, je Frage mit belegter Antwort, Links/Quellen, Code-/API-Beispielen und Fallstrick-Notizen.

Soll ich diese Liste zusätzlich als fertige Markdown-Datei (z. B. `resident_groupB_research.md`) im Projektordner formatieren, damit der Recherche-Agent sie direkt als Input übernehmen kann?
