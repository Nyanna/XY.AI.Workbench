# Umsetzungsplan H4 · Graph-Capture & Persistent-Scorer

**Gruppe H — Ausblick/zurückgestellt. Status A** (Thesen-§3/§9; Ausbaupfad `resident_subengine.md` §9, Phase 4). Persistent-Scorer überlappt H1.

**Planungsauftrag:** NUR Erweiterungspunkte sichern. Kein Graph-Capture, kein Persistent-Scorer wird jetzt gebaut.

## 1. Was später kommt (Kurzriss, nicht umsetzen)

- **Graph-Decorator** auf dem Launcher: Modus `Direct` oder `Capture`, Cache-Schlüssel `(Op, Shape-Bucket, Regions)`, hält Handles (keine Besitzrechte), verwirft bei `gen`-Änderung; Ein-/Ausgaben in festen Ring-Slots des Managers (`resident_subengine.md` §4.6).
- **Persistent-Scorer** mit Ring (siehe H1). „Sinnvoll für den Scorer, nicht für den Encoder."

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H4)

- **Ablauf:** `cudaStreamBeginCapture(stream, mode)` → Ops im Stream → `cudaStreamEndCapture` → `cudaGraphInstantiate` → `cudaGraphLaunch` (Replay). Modi `Global`/`ThreadLocal`/`Relaxed`.
- **Während Capture verboten/problematisch:** synchrone Aufrufe (`cudaStreamSynchronize`, `cudaDeviceSynchronize`, `cudaMemcpy` ohne Async), Legacy-Default-Stream, klassisches `cudaMalloc` (Global-Modus), Event-Queries. **Erlaubt:** Kernel, `cudaMemcpyAsync`, Events (Record/Wait), `cudaMallocAsync`.
- **Capture über candle-Ops** nur, wenn keine Allokation/Sync im Capture ausgelöst wird; candle allokiert über cudarc — nur `cuMemAllocAsync` ist capture-fähig **[prüfen]**. Kein offizielles candle-Feature bekannt **[prüfen GitHub-Issues]**. Empfehlung: Warm-up-Lauf vor Capture, feste Buffer, Capture nur für eigene Kernel-Folgen.
- **Cache-Schlüssel** = (Shape-Bucket, dtype, Kernel-Variante, Gerät/Stream). Feste Ein-/Ausgabe-Regionen; bei Pointer-Änderung (`gen`) Instanz invalidieren oder per `cudaGraphExecKernelNodeSetParams`/`cudaGraphExecUpdate` aktualisieren. Shapes in wenige Buckets (Zweierpotenzen) quantisieren (jede Instanz kostet VRAM).
- **`cudarc`** hat Graph-Support auf Raw-API-Ebene (`result::graph`/`sys`) und ab neueren Versionen ein sicheres Wrapper-API **[prüfen Version/Beispiele]**.

## 3. Zu sichernde Erweiterungspunkte (Checkliste — darf jetzt nicht verbaut werden)

1. **Launcher ist ein Trait mit Decorator-Fähigkeit.** Jeder Op-Launch geht über eine Launcher-Abstraktion, auf die ein Decorator mit Modus `Direct | Capture` (später `PersistentRing`) gelegt werden kann. Der Scheduler kennt keine Graphs/Kernel (`resident_subengine.md` §2, Invarianten).
2. **Ein-/Ausgaben in festen Regionen / Ring-Slots.** Die Op-IO-Pfade müssen feste, vom Manager verwaltete Regionen nutzen können (keine impliziten Allokationen im Hot-Path). Das ist Voraussetzung für Capture **und** für den stream-geordneten Arena-Betrieb (D3/E3: `cudaMalloc/Free` im Betrieb vermeiden).
3. **`gen`-basierte Invalidierung ist vorhanden.** `Region{...,gen}` entwertet Handles nach Evict/Reload (`resident_subengine.md` §4.2). Sicherstellen, dass eine spätere Graph-Instanz an `gen` gebunden/invalidierbar ist (Cache-Key enthält Region-Identität + `gen`).
4. **Cache-Key-Struktur als offener Platzhalter.** Dort, wo Launches identifiziert werden, eine Key-Struktur `(Op, Shape-Bucket, Regions/gen, dtype, kernel-variant)` vorsehen (Shape-Bucket-Quantisierung als Hook), ohne jetzt einen Graph-Cache zu bauen.
5. **Warm-up-Hook.** Der Lane-/Ressourcen-Lifecycle darf einen Warm-up-Lauf vor einem späteren Capture erlauben (keine Annahme „erster Launch = produktiver Launch").

## 4. Jetzt NICHT umsetzen

Kein `cudaStreamBeginCapture`/`cudaGraphInstantiate`-Code, kein Graph-Cache, kein Persistent-Scorer, keine `cudaGraphExecUpdate`-Pfade.

## 5. Offene Verifikationen ([prüfen])

- candle-Capture über eigenen Stream ohne Alloc/Sync (Issue-Recherche `huggingface/candle`).
- `cudarc` Graph-Wrapper-API der Projektversion.
- `cuMemAllocAsync`-Pfad in candle/cudarc (capture-fähig).

## 6. Definition of Done

- Launcher-Trait + feste-Region-IO + `gen`-Invalidierung + Cache-Key-Platzhalter + Warm-up-Hook sind als Anforderung/offene Abstraktion in §4.6-nahen Paketen (D2/D6/E4) vorhanden.
- Ein No-GPU-Test belegt, dass ein Launcher-Decorator (Mock `Direct` vs. Mock-`Capture`) hinter demselben Trait austauschbar ist, ohne Scheduler-/Op-Änderung.
