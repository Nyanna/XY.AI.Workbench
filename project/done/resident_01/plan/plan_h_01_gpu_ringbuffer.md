# Umsetzungsplan H1 · GPU-Ringbuffer-Modus (persistenter Kernel)

**Gruppe H — Ausblick/zurückgestellt. Status A** (Thesen-§3: die eigentliche „zentrale Antwort" der These; in Phase 1 bewusst durch das Pull-Task-Queue-Modell ersetzt).

**Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein persistenter Kernel, kein SPSC-Ring wird jetzt gebaut. Ziel ist, dass der spätere Ringbuffer-Modus **ohne API-Bruch** nachgerüstet werden kann (Ausbaupfad `resident_subengine.md` §9, Phase 4).

## 1. Was später kommt (Kurzriss, nicht umsetzen)

- Residenter Kernel mit begrenzter Lebensdauer (`idle_spin`, `max_run_time`), Neustart durch den Host; Modul, Gewichte, Index und Ring bleiben im VRAM.
- **Getrennte SPSC-Ringe** für Embedding (+ Results) und MaxSim (+ Results), je ein Schreiber, zwei monotone Zeiger `w`/`r`. CAS nur für den Exit: `CAS(w, r, r | EXIT)`.
- Manager kennt die eingereihten Requests und damit die **blockierten Segmente**; Freigabe erst, wenn kein vor dem Pointer-Swap eingereihter Request das Segment mehr referenziert.
- Quellen: `resident.md` (Ausblick, GPU-Ringbuffer-Modus), `resident_index.md` (Ringbuffer-Modus; Manager-Disziplin, Pointer-Lesezeitpunkt), `resident_subengine.md` §4.6 (Persistent-Kernel/Ringbuffer), `resident_processor.md` (Nicht umgesetzt / Erweiterungen).

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H1)

- **Persistenter Kernel:** Grid = `SMs × cudaOccupancyMaxActiveBlocksPerMultiprocessor(...)` (22 SMs auf der 1660), oder `cudaLaunchCooperativeKernel` (garantiert Co-Residenz, schlägt sonst fehl statt zu deadlocken). Fallstricke: Block-zu-Block-Wartebeziehungen → Deadlock bei nicht-residentem Block; Display-Watchdog (Linux/X11, Windows TDR ~2 s); belegt alle SMs → blockiert nachfolgende Launches.
- **SPSC-Ring Host↔Kernel:** gemapptes Pinned-Memory (`cudaHostAlloc(..., cudaHostAllocMapped)` + `cudaHostGetDevicePointer`). `head`/`tail` auf getrennten Cachelines (≥128 B), Kapazität Zweierpotenz. **`cudaHostAllocWriteCombined` NICHT setzen** (macht Host-Reads extrem langsam). Device→Host sind posted writes → erst nach `__threadfence_system()` geordnet.
- **Sichtbarkeit:** Device Release = Daten schreiben → `__threadfence_system()` → Index schreiben (oder `cuda::atomic_ref<T, thread_scope_system>::store(release)`). Device Acquire = `ld.acquire.sys` bzw. `load(acquire)`. `volatile` allein gibt **keine** Ordnung.
- **Exit-CAS host-seitig** auf dem gemappten Speicher (`AtomicU64::compare_exchange`); GPU liest `w`/Flag nur per `ld.acquire.sys`. Vermeidet GPU-System-Atomics über PCIe (`cudaDevAttrHostNativeAtomicSupported` ist plattformabhängig **[prüfen]**).
- **`ld.acquire.sys`** seit PTX 6.0 / sm_70 → auf Turing (sm75) verfügbar. Scope-Suffix `.sys` für Host↔Device nötig. Beim Austausch von Device-Tabellen zusätzlich Event/Stream-Sync.
- **Zeitscheibe:** `clock64()`/`%globaltimer`, Idle via `__nanosleep(n)` (sm70+). Stop-Flag/Ring nur an Scheibengrenzen prüfen. Scheibe deutlich < Watchdog-Limit (z. B. ≤ 100 ms) **[Empfehlung]**.
- **Crates:** Für langlaufende persistente Kernel **`cudarc` direkt** auf eigenem Stream nutzen (Driver-API, Cooperative-Launch über `result::*`), `candle` nur für Tensor-Ops. `candle` ist dafür nicht gedacht. Genaue API der aktuellen `cudarc`-Version **[prüfen]**.

## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

1. **Lane/ExecContext kennt eine persistente Variante.** `ExecContext`/Lane (`resident_subengine.md` §4.1) darf nicht auf „ein Launch = ein Batch = ein Event" festverdrahtet werden. Die Lane-Klasse muss um einen Typ erweiterbar sein, dessen Lebensdauer über viele Work-Items reicht (Fortschrittszähler statt Batch-Event). → Lane-Klasse als offenes Enum/Trait, nicht als festes `{Query,Index,Copy}`-Match ohne Default.
2. **MemoryManager kann eine langlebige Ring-/State-Region in gemapptem Pinned-Host-Speicher vergeben.** Die Arena-Klassen (`resident_subengine.md` §4.2: ParamBlock/Staging) müssen eine zusätzliche, langlebige Klasse „Ring-State (host-mapped)" aufnehmen können, ohne die `Region{arena,offset,len,gen}`-Signatur zu ändern. Kein Pointer in Signaturen.
3. **Pointer-Lesezeitpunkt ist abstrahiert.** Die Manager-Disziplin (E4, `resident_index.md`) muss beide Modi tragen: (a) Pointer als Launch-Argument (Swap wirkt ab nächstem Launch) und (b) Acquire-Load im persistenten Modus. Die Swap-/CoW-Logik (atomarer 8-Byte-Pointer-Swap, Root→Segmenttabelle→Segment) darf keine Annahme „Pointer wird nur zwischen Launches gelesen" hartkodieren.
4. **Residency-Tracker kann Segmente als „blockiert" markieren.** Der Residency-/Mapping-Zustand (E7/E5) muss ein Feld/Prädikat „durch eingereihte Ring-Requests gesperrt" aufnehmen können (Sperrmenge = nur Requests vor dem Swap; task-gebundene temporäre Segmente gelten als blockiert bis Auflösung). Eviction muss dieses Prädikat respektieren.
5. **Launcher ist dekorierbar / austauschbar.** Der Op-Launch geht über eine Launcher-Abstraktion (siehe H4), sodass neben `Direct`/`Capture` später ein Persistent-Ring-Launcher als weitere Implementierung hinter demselben Trait eintritt. Der Scheduler kennt weder Ring noch Kernel (`resident_subengine.md` §2, Invarianten).
6. **Getrennte Ringe Embedding/MaxSim als Konzept vorgesehen.** Dort, wo Op-/Ergebnis-Pfade definiert werden, keine Annahme „genau ein Ergebniskanal" treffen; Embedding-Ergebnisse (neue Segmente) und MaxSim-Ergebnisse (Hits) sind bereits getrennte Pfade (`resident_index.md`: „Kernel schreibt Segmente").

## 4. Jetzt NICHT umsetzen

Kein persistenter Kernel, keine SPSC-Ring-Implementierung, kein `__threadfence_system`/`ld.acquire.sys`-Code, keine Cooperative-Launch-Konfiguration, keine Zeitscheiben-Logik. Diese gehören in Phase 4 des Ausbaupfads.

## 5. Offene Verifikationen ([prüfen])

- `cudarc`-Cooperative-Launch-/Host-Alloc-Wrapper der aktuellen Version.
- `cudaDevAttrHostNativeAtomicSupported` auf sm75 (entscheidet, dass Exit-CAS host-seitig bleiben muss).
- Watchdog-Verhalten auf der Zielplattform (Linux/X11) → Zeitscheibengröße.

## 6. Definition of Done

- Die sechs Erweiterungspunkte aus §3 sind in den jeweiligen Kern-Paketen (B2, D1/D2, E4, E5/E7, §4.6-Launcher) als Anforderung dokumentiert bzw. im Code als offene Abstraktion vorhanden (Trait/offenes Enum/Default-Zweig).
- Ein kurzer ADR-Eintrag hält die Umsetzungsentscheidung fest: „Thesen-§3 Ringbuffer in Phase 1 durch Pull-Queue ersetzt; Ringbuffer = späterer Persistent-Scorer-Modus" (Nachvollziehbarkeit der Thesen-Treue).
- Ein No-GPU-Test belegt, dass Lane-Klasse, Arena-Klasse und Pointer-Lese-Abstraktion erweiterbar sind (Mock), ohne reale CUDA-Pfade.
