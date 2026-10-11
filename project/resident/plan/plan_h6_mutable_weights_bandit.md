# Umsetzungsplan H6 · Mutable Weights / Online-Bandit

**Gruppe H — Status —** (Thesen-§9/§9.1). **Ausdrücklich NICHT Engine-Scope.**

**Planungsauftrag:** Keine Engine-Arbeit. Divergenz zur These dokumentieren und sicherstellen, dass ein späterer eigener Workstream nicht verbaut wird („nicht verbauen").

## 1. Einordnung / Divergenz

- These §9/§9.1: Parameter-/Aktivierungs-Dualität, **mutable weights**, Online-Adaption per Contextual Bandit als Meta-Router.
- Plan-Divergenz (ausdrücklich festhalten): **Gewichte sind immutabel** (Weights-Arena langlebig, nur bei Entladung frei, `resident_subengine.md` §4.2/§4.3). **Nur der Index** ist bidirektional mutabel (`resident.md`, `resident_index.md`). Das ist die spekulativste Thesen-Idee und bewusst nicht adressiert.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` H6)

- **Contextual Bandits als Meta-Router:** Arme = Retrieval-/Cascade-Strategien, Kontext = Query-Features, Reward = Klick/Zufriedenheit/latenz-adjustierte Qualität. Verfahren: LinUCB (Li et al., WWW 2010), Thompson Sampling, ε-greedy; Toolkit Vowpal Wabbit. Off-Policy-Evaluation (IPS/Doubly Robust) vor Ausrollen.
- **Online-Adaption von Gewichten:** Online-Fine-Tuning, LoRA-Adapter-Updates, OGD auf Heads, kontinuierliches Lernen. Risiken: katastrophales Vergessen, Reward-Hacking, Feedback-Loops/Bias, Nicht-Reproduzierbarkeit. Praktikabel nur für kleine Teile (Router, Score-Gewichte), **nicht** für das Basismodell.
- **Reward-Stream-Pipeline:** Inferenz loggt (Kontext, Aktion, Propensity) → Reward-Events über Join-Key → Lernkomponente (getrennter Prozess) → versionierter Snapshot → **atomarer Swap** im Serving. Inferenz und Update strikt getrennt, Rollback über Versionierung.

## 3. Zu sichernde Nicht-Ausschlüsse (Checkliste — „nicht verbauen")

1. **Gewichts-Reload/Swap-Pfad nicht ausschließen.** Gewichte sind immutabel, aber die Ressource ist **reload-/swap-fähig** über den Lifecycle `Unloaded→Loading→Resident→…` und `Region.gen`-Entwertung (`resident_subengine.md` §4.2/§6). Ein späterer versionierter Snapshot-Swap (neue Gewichte als neue Region, Pointer-Swap wie bei Index-CoW) wäre damit prinzipiell möglich — diese Reload-Fähigkeit nicht hartkodiert deaktivieren (kein `const`/statisch eingebrannte Gewichte).
2. **Parameterobjekt erweiterbar für spätere Logging-Felder.** Das Task-Params-Objekt (`#[non_exhaustive]`/Builder) darf später (Kontext, Aktion, Propensity) als Diagnose-/Log-Felder aufnehmen, ohne Bruch — jetzt nur nicht ausschließen.
3. **Addon-Pfad trägt einen Meta-Router.** Ein Bandit-Meta-Router ist konzeptionell ein **Addon** (`resident_index.md` Addons), das Folgetasks unterschiedlich parametrisiert über die normale Queue einreiht — keine Kernänderung nötig. Sicherstellen, dass der Addon-/erweiterbare-Params-Pfad das zulässt.

## 4. Jetzt NICHT umsetzen (und auch nicht als Engine-Workstream planen)

Kein Bandit, keine Reward-Queue, keine mutable Weights-Arena, kein Online-Lernen, keine Propensity-Logging-Implementierung. Falls je gewünscht: **eigener, getrennter Workstream** (zusätzliche Reward-Queue, mutable Weights-Arena, separater Lernprozess, versionierter Swap) — außerhalb dieser Engine-Phase.

## 5. Definition of Done

- Ein ADR-Eintrag dokumentiert die Divergenz: „Gewichte immutabel, nur Index bidirektional mutabel; Bandit/Online-Adaption bewusst nicht adressiert."
- Gewichts-Ressource ist reload-/swap-fähig (nicht statisch eingebrannt); Params sind erweiterbar; Addon-Pfad existiert.
- Kein weiterer Code-Workstream.
