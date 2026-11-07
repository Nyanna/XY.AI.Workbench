Zusammenfassung der finalen Architektur, als Implementierungs-Fahrplan:

**Einmalig, Session-Start:**



java

```java
ObjectId parentCommit = repository.resolve("HEAD");
String lastOpCommit = parentCommit.getName(); // Startpunkt der Kette
```

**Pro LLM-Operation (in deinem bestehenden pre/post-Hook):**

1. **Pre-Hook:** nichts Neues nötig – der Zustand *vor* der Operation ist bereits durch `lastOpCommit` (bzw. initial `HEAD`) abgedeckt.
2. **LLM führt Operation aus** (dein bestehender Trigger, unverändert).
3. **Post-Hook – Snapshot bauen:**



java

```java
   DirCache cache = repository.lockDirCache();
   // Working Tree in den Index einlesen (AddCommand oder DirCache-API)
   ObjectId treeId = cache.writeTree(inserter);

   // Commit-Objekt erzeugen, OHNE Branch/HEAD zu bewegen
   CommitBuilder cb = new CommitBuilder();
   cb.setTreeId(treeId);
   cb.setParentId(ObjectId.fromString(lastOpCommit));
   cb.setMessage("op-" + n + ": " + beschreibung);
   cb.setAuthor(...); cb.setCommitter(...);
   ObjectId newCommit = inserter.insert(cb);
   inserter.flush();

   // Eigener Ref-Namespace, verkettet
   RefUpdate update = repository.updateRef("refs/llm-ops/op-" + n);
   update.setNewObjectId(newCommit);
   update.update();

   lastOpCommit = newCommit.getName(); // Parent für nächste Operation
```

4. **Diff anzeigen (HITL-Gate):**



java

```java
   DiffFormatter df = new DiffFormatter(outputStream);
   df.setRepository(repository);
   df.format(parentTreeIter, newTreeIter); // oder direkt zwei Commits
```

→ Ausgabe in `org.eclipse.compare.CompareEditorInput` einspeisen, damit es wie ein natives Eclipse-Diff aussieht (kein eigenes Rendering nötig).

5. **Bei Ablehnung – Revert:**



java

```java
   DirCache dc = repository.lockDirCache();
   // Working Tree + Index exakt auf vorherigen Snapshot zurücksetzen
   new ResetCommand(repository)
       .setMode(ResetCommand.ResetType.HARD) // bezogen auf den vorherigen op-Commit, nicht HEAD!
       .setRef("refs/llm-ops/op-" + (n-1))
       .call();
   // Wichtig: ResetCommand bewegt normalerweise HEAD – hier stattdessen
   // manuell DirCache + Working Tree aus dem Tree des Ziel-Commits neu aufbauen,
   // damit dein echter Branch/HEAD unberührt bleibt (siehe Punkt unten).
```

Sauberer via `DirCacheCheckout` direkt auf den Ziel-Tree, nicht über `ResetCommand` (das ist auf HEAD-Bewegung ausgelegt).\
Danach ungetrackte neue Dateien separat entfernen (`Git.clean()`).

**Am Session-Ende:**

* `refs/llm-ops/*` bleibt als vollständige, EGit-sichtbare Commit-Kette stehen (unter "Additional Refs" in der Git Repositories View, per Rechtsklick → "Show In History" betrachtbar).
* Aufräumen falls gewünscht: `git update-ref -d refs/llm-ops/op-N` pro Eintrag, oder Namespace komplett behalten für Audit-Zwecke.
* Kein Push nötig/gewollt – Namespace bleibt lokal, solange du keine Push-Refspec dafür konfigurierst.

**Die drei kritischen Punkte, die du nicht vergessen darfst:**

1. Jeder neue Commit bekommt den *vorherigen `op-N`-Commit* als Parent, nicht immer denselben Ausgangs-Commit – sonst Fächer statt Kette in der History-View.
2. Revert läuft gegen den Tree des Ziel-Commits (`DirCacheCheckout`), nicht gegen `ResetCommand` mit HEAD-Bezug – sonst bewegst du versehentlich den echten Branch.
3. Neu angelegte, bis dahin ungetrackte Dateien werden von einem reinen Tree-Reset nicht entfernt – nach Revert zusätzlich Clean-Schritt einbauen.
