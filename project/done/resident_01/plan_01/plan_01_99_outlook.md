# Plan 01 — Ausblick (nur umreißen, Erweiterungspunkte sichern)

Diese Erweiterungen sind nicht Teil von Plan 01. Sie sind hier festgehalten, damit die Entscheidungen der Stufen S1 bis S6 sie nicht verbauen. Zu jedem Punkt steht, welchen Haken Plan 01 dafür setzt und was ein Folgeplan klären müsste. Bezug zur Metaplanung: Gruppe H (H1 bis H4) und die Hook-Verpflichtungen in §6 der Scope-Charta (`rag/docs/architecture/scope_charta.md`); deren Liste ist in L0.1 zu lesen und gegen Plan 01 zu prüfen.

## Erweiterungen

| ID | Thema | Bezug | Haken in Plan 01 | Voraussetzung und nächster Schritt |
|---|---|---|---|---|
| A1 | GPU-Pfad (`cuda`): `GpuEngine`, `GpuIndexManager`, Residency, Präzisionsprofile | Metaplan D8, E7, Gruppe F; Phasenplan | `BackendInfo.kind`, `RuntimeConfig` mit reserviertem Gerätefeld, kein `cfg(feature)` im Layer (D17) | Engine-Gruppen D/E auf GPU abschließen; Folgeplan „GPU-Runtime“ mit Golden-Vergleich CPU gegen GPU (Toleranz 1e-3, E45) |
| A2 | Daemon- und Server-Modus (Prozess bleibt warm) | Charta §3 („kept warm“) | lazy `OnceCell`-State (D11), konfigurierbarer `background` (D9), Konfiguration als Struktur | Server-Entwurf (Socket oder stdio), Lebenszyklus, Mehrbenutzerfragen; `Engine::run_query` ist bereits als Bibliothek nutzbar |
| A3 | Rerank mit Cross-Encoder (zweiter Similarity-Task auf verengtem Scope) | H3, `ScorerKind::CrossEncoder`, Addons | `SearchOutcome` mit Statistik, `top_k_factor`, Scope-Planer (`ScopePlan`) | Engine: `ScorerUnavailable` aufheben (Inference-Ops, CrossEncoder-Ressource); Layer: Politik, wann und mit welchem k |
| A4 | Funnel mit Centroid-Prefilter | H3, `Scope::Centroid`, `Index::summary`, Addons | `ScopePlan` kapselt die Scope-Bildung; `IndexStore` erlaubt Zusatzdaten | Index mit Summary (Centroids) bauen; Politik und Recall-Messung im Layer |
| A5 | `SharedIndex` als zentrale Änderungsquelle | Layer-Vertrag (Persistence contract), E15 | `Catalog::plan` arbeitet auf einer Liste sichtbarer Dateien und ist quellenneutral | Eine zentrale Komponente muss `SharedIndex` befüllen; dann `ChangeSource`-Trait einführen und Cursor nutzen |
| A6 | Enrich-Modus als eigener Layer (zum Beispiel `resident_rerank`) | `LayerStage::Enrich`, `depends_on` | `FileHit`-Abbildung, Scope aus Dateimengen, getrennte Layer-ID möglich | Gemeinsame Runtime-Instanz zwischen zwei Layern (geteilter `Arc<ResidentRuntime>`), Entscheidung über Aktivierungsfelder |
| A7 | Hybrid-Fusion (zum Beispiel Reciprocal Rank Fusion) als Postprocess-Layer | Core rankt nicht numerisch (E1) | Objekt `Resident` mit `Score` und `Rank` im Ergebnis | Eigener Postprocess-Layer mit `ResultSet::sort_by_key`; Gewichtung von grep, trigram, resident |
| A8 | Index-Sharding oder LSM-Segmente | D6, E36 (Flush ist O(N)) | `IndexStore` kapselt den Index; Backend-Naht erlaubt mehrere `IndexHandle` | Auslöser: Flush über 2 s oder mehr als 50k Chunks (L6.6); mehrere Indizes, Top-k-Merge im Layer |
| A9 | Chunk-genaue Wiederverwendung von Embeddings | W2 (Kosten) | Chunk-Texte sind deterministisch, Katalog kennt Dateihash | Chunk-Hashes im Katalog, Cache von Vektoren je Hash; lohnt nur bei häufigen kleinen Änderungen |
| A10 | Dateiwächter für langlebige Prozesse | Layer-Vertrag (`background`) | `background`-Schleife, `maintain(Update, …)` | Neue Abhängigkeit (zum Beispiel `notify`), nur sinnvoll mit A2 |
| A11 | Hierarchische Scopes (`Scope::Subtree`, `Index::children`) für Verzeichnisse | E40 | Bereichsmodell der EntryId (D5) bleibt kompatibel | Index mit Knotenstruktur; Vergleich gegen `Scope::Set` per Messung |
| A12 | Automatisches Abschneiden (Elbow, `z_top1`, Gap) | E21, D19 | Statistik in `detail.search` | Auswertung auf dem Gold-Set, dann Option `residentCut` |
| A13 | Mehrere Modelle (Auswahl per Feld) | Charta §3 (mehrere Modelle, feste Menge vor `init`) | Index ist modellgebunden (LP12), Fingerabdruck im Katalog | Ein Index je Modell, Modellauswahlfeld, Runtime pro Modell |
| A14 | Gleitende Indizierung, MaxSim/ColBERT | H2, G3, Charta §5 | Index-Datenmodell mit variabler Länge in der Engine vorbereitet | Eigenes Datenmodell und Chunking; kein Plan |
| A15 | Kalibrierung und Präzisionsprofile | Gruppe F, D18 | `precision` wird durchgereicht | Engine liefert Profile; Layer wählt Zielpräzision je Anfrageart |

## Nicht verbauen (Prüfliste für Reviews)

- Keine Annahme im Layer, dass die Engine nur CPU kennt: Geräte- und Backendwahl bleibt in `ResidentBackendFactory` und `ResidentRuntime`.
- Keine Beschränkung der Scope-Typen auf `All` und `Set`: `ScopePlan` liefert einen `Scope`, nicht eine Liste.
- Kein festes Ein-Index-Modell in der Backend-Naht: `IndexHandle` ist undurchsichtig und mehrfach verwendbar.
- Keine Kopplung der Ergebnisfelder an einen einzigen Treffertyp: `Resident.Chunks` ist eine Liste, weitere Felder dürfen hinzukommen.
- Kein stilles Verwerfen von Engine-Statistik: Sie bleibt in `detail.search` verfügbar, bis A12 sie nutzt.
