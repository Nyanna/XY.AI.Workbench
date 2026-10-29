Lies die These und arbeite dann das Concept zu einem englischsprachigen Entwurf auf. Das Concept stellt den geplanten konkreten Umsetzungsversuch dar.

# These:

[include](/home/user/xyan/xy.ai.workbench/docs/dev/bottom_up.md)

# Concept:

### You said:

Ich möchte via Top-to-Bottom-to-Top ein großes Projekt agentisch entwickeln.\
Wie splitte ich ein sehr großes Anforderungsdokument in zusammenpassende, aber kongruente Teilaspekte? Recherchiere das.\
Kann ein Agent so ein großes Dokument sinnvoll in einem Pass verarbeiten? Was ist mit Ausgabetokenlimit.

### Other responded:

**Aufteilung (Top-down → Bottom-up)**

1. **Stufe 0, Kerndokument (klein, \~2–5 Seiten):** Vision, Glossar, Architekturprinzipien, Tech-Stack, Querschnittsregeln (Naming, Fehlerbehandlung, Security) und eine Liste aller Module. Es ist die Single Source of Truth und wird jedem Agenten mitgegeben.
2. **Stufe 1, Zerlegung nach Domänen/Bounded Contexts**, nicht nach Seitenzahl. Jede Teilspezifikation enthält Ziel, Akzeptanzkriterien, Abhängigkeiten und Schnittstellen.
3. **Verträge zuerst:** API-Schemas, Datenmodell und Events werden als eigene Artefakte vor den Modulen festgelegt. Sie sind das, was die Teile “kongruent” hält. Bei Multi-Agent-Setups verstärkt sich Interpretationsdrift sonst von Agent zu Agent.
4. **Abhängigkeitsgraph:** Eine Reihenfolge der Teilspezifikationen und ein Trace-Index (Requirement-ID → Teilspezifikation → Task → Test). Tools wie ralph-specum nennen das “Epic-Triage” in abhängigkeitsbewusste Specs.
5. **Konsistenzprüfung:** Ein separater Review-Agent prüft die Teilspezifikationen auf Lücken, Überlappungen und Widersprüche gegen das Kerndokument, bevor Code entsteht.
6. **Bottom-up:** Pro Task läuft ein frischer Agent mit Kerndokument, seinem Teilspec und den Verträgen. Tests und Akzeptanzkriterien dienen als ausführbares Orakel, und Ergebnisse fließen in den Trace-Index zurück.

**Ein Pass?**

Technisch passt es: Fable 5, Opus 4.8 und Sonnet 4.6 haben bis zu 1M Token Kontext. Sinnvoll ist es aber nur eingeschränkt:

* **Context Rot:** Größere Fenster verzögern die Degradation, verhindern sie aber nicht. Genauigkeit sinkt kontinuierlich mit der Länge, und Informationen in der Mitte werden schlechter genutzt (U-förmige Kurve). Je nach Messung treten Einbrüche schon ab ca. 32–64K Token auf. Das sind Messungen der Quellen, keine Garantien für dein Dokument.
* **Lesen ≠ Schlussfolgern:** Etwas in 1M Token wiederzufinden heißt nicht, zuverlässig darüber zu schlussfolgern.
* **Empfehlung:** Einmal komplett lesen lassen, um Kerndokument, Zerlegung und Index zu erzeugen. Danach arbeitet jeder Agent nur noch mit seinem Ausschnitt. Ein praktischer Richtwert aus den Quellen ist, Kontext dicht und klein zu halten (z. B. Spec-Partitionen unter \~400 Zeilen).

**Ausgabelimit**

Es ist vom Kontextfenster getrennt:

* Max. Output laut Quellen: **128K Token** pro Antwort bei Fable 5 und Opus 4.8. Die Angaben zu Sonnet 4.6/Haiku 4.5 widersprechen sich (64K vs. 8K), prüfe das in der offiziellen Doku.
* Batch-API: bis zu 300K Output mit Beta-Header (`output-300k-2026-03-24`).
* Output zählt gegen das Kontextfenster, Thinking-Tokens ebenfalls.
* Hohe `max_tokens` erfordern Streaming im SDK. Bei `stop_reason: max_tokens` ist die Ausgabe abgeschnitten.
* Praktisch: Lass nie das ganze Zerlegungsergebnis in einem Output erzeugen, sondern Datei für Datei bzw. Teilspec für Teilspec.

Sources:

* [How to write a good spec for AI agents (Addy Osmani)](https://addyosmani.com/blog/good-spec/)
* [The right amount of spec for agentic development (O’Reilly)](https://oreilly.com/radar/the-right-amount-of-spec-for-agentic-development)
* [ralph-specum spec-workflow](https://www.claudepluginhub.com/skills/tzachbon-ralph-specum-plugins-ralph-specum-2/spec-workflow)
* [Claude Context Window Guide (fast.io)](https://fast.io/resources/claude-context-window-guide/)
* [Fable 5 API production patterns](https://developersdigest.tech/blog/fable-5-api-production-patterns-rate-limits)
* [Context Rot (Lyzr)](https://lyzr.ai/blog/context-rot/)
* [Context rot fixes (LogRocket)](https://blog.logrocket.com/context-rot-slowing-down-your-ai-agent-how-fix/)
* [LeanSpec: Large specs and agent performance](https://lean-spec.vercel.app/blog/ai-agent-performance)

### You said:

Du scheinst das Grundproblem nicht zu verstehen und die Antwort müsste algorithmisch sein.\
Selbst die abstrakte Beschreibung ist 10 k Zeilen lang. Daraus lässt sich bereits in einem Pass kein gutes Ergebnis ableiten. Ich rede allein von Aspekttrennung nicht von Spezifikation.

### Other responded:

Entwurf eines Algorithmus für die konsistente Aufteilung in Aspekte

Entwurf eines Algorithmus für die konsistente Aufteilung in Aspekte

Du hast recht: Das ist ein Partitionierungsproblem, kein Spezifikationsproblem. Der Algorithmus unten hält nie mehr als einen Chunk im Kontext, und der Text wird nie vom LLM reproduziert.

**Prinzip:** Das LLM erzeugt nur kleine Metadaten (Tags, Entitäten, Referenzen) pro Einheit. Das Gruppieren übernimmt deterministischer Code über einen Graphen.

**Algorithmus**

1. **Atomisieren (kein LLM):** Zerlege das Dokument an Überschriften, Listen und Absätzen in Einheiten `u_i` mit stabiler ID und Zeilenbereich. Der Originaltext bleibt unverändert im Speicher.
2. **Map (pro Chunk, ca. 300–500 Zeilen, nur 1 Chunk im Kontext):** Das LLM liefert pro Einheit JSON:

   * `aspects`: freie Tags, 1–3 Stück
   * `defines`: Begriffe/Entitäten, die hier definiert werden
   * `uses`: Begriffe/Entitäten, die hier verwendet werden
   * `refs`: explizite Verweise auf andere Einheiten

   Die Ausgabe enthält nur IDs und Labels, nie Text. Dadurch greift das Ausgabelimit nicht, und es gibt keine Umformulierungsdrift.

3. **Vokabular normalisieren:** Die Tag- und Entitätslisten (einige hundert Einträge) passen in einen Pass. Das LLM oder Embedding-Clustering bildet Synonyme auf kanonische Labels ab. Das ist die einzige globale LLM-Operation, und sie arbeitet nur auf Labels.
4. **Graph bauen (deterministisch):** Knoten sind die Einheiten. Kanten gibt es bei:

   * `uses → defines` (Abhängigkeit)
   * `refs`
   * gemeinsamen Tags/Entitäten (gewichtet, z. B. TF-IDF-artig, damit Allerweltsbegriffe wenig Gewicht haben)

5. **Partitionieren:** Community-Detection (Leiden/Louvain), rekursiv. Ein Cluster, der das Größenbudget überschreitet (z. B. 1500 Zeilen), wird erneut auf seinem Teilgraphen partitioniert. Kleine Cluster mit hoher Kopplung werden verschmolzen.
6. **Querschnitt behandeln:** Einheiten mit hohem Grad über mehrere Cluster (hohe Betweenness) sind Cross-Cutting-Concerns. Sie werden aus den Domänen-Clustern herausgelöst und bilden einen eigenen Querschnittsaspekt, der überall referenziert wird.
7. **Kongruenz durch Schnittkanten:** Jede Kante, die zwei Cluster verbindet, wird zu einem Interface-Eintrag `(Entität, produzierender Aspekt, konsumierender Aspekt, Unit-IDs)`. Die Interface-Tabelle ist der Vertrag zwischen den Teilen. Jeder Aspekt bekommt sein Glossar-Subset (`defines` + `uses`) und seine Ein- und Ausgangs-Interfaces.
8. **Invarianten (Mengenprüfung, kein LLM):**

   * Jede Unit-ID liegt in ≥1 Aspekt (Coverage), Waisen werden dem Nachbarcluster mit dem höchsten Kantengewicht zugeordnet.
   * Jeder `uses`-Begriff hat ein `defines` (sonst Lückenmeldung).
   * Jeder Begriff hat genau eine definierende Einheit (sonst Konfliktmeldung).
   * Clustergröße liegt innerhalb des Budgets.

9. **Widersprüche lokal prüfen:** Das LLM vergleicht nur Einheitenpaare entlang der Schnittkanten und innerhalb eines Clusters. Das ist begrenzt, kein globaler Quadratvergleich.
10. **Iterieren:** Fallen Modularität und Schnittkantenanteil schlecht aus, werden Tags oder Cluster nachjustiert (Schritt 3 oder 5) und neu berechnet. Die Map-Ergebnisse bleiben dabei gecacht.

**Ergebnis pro Aspekt:** Eine Liste von Zeilenbereichen (der Text wird per Slicing geholt), ein Glossar-Subset und Interfaces. Zusammen ist das genau die disjunkte, aber über die Interface-Tabelle verbundene Aufteilung, die du suchst.

Die Skalierung ist linear in der Dokumentgröße: n/Chunk Map-Aufrufe plus ein Label-Pass plus lokale Prüfungen.

### You said:

Ah, du redest von Kategorisierung. Der erste Pass wird in jedem Fall reichen, um ein Tag Vokabular abzuleiten (vielleicht Glossar). Dann können parallele Pässe die Anforderungen abschnittsweise 1 zu N taggen. So lässt sich verlustfrei scopen. Weitere Pässe können dann jeweils weiter schneiden, oder Kanten von Aspekten bilden. Ist das richtig?

### Other responded:

Ja, das ist richtig, mit fünf Präzisierungen:

1. **Vokabular-Pass:** Auch der erste Pass muss nicht das ganze Dokument auf einmal sehen. Pro Chunk werden Kandidaten-Tags vorgeschlagen, danach werden sie in einem zweiten Schritt auf Labels-Ebene zusammengeführt. Das Ergebnis ist ein geschlossenes Vokabular, jeweils mit Definition, Abgrenzung und 1–2 Beispielen, damit parallele Tagger konsistent bleiben.
2. **Verlustfreiheit:** Sie entsteht nicht durch das Tagging selbst, sondern durch Invarianten, die du deterministisch prüfst:

   * Das Dokument wird vorab in Einheiten mit stabiler ID zerlegt.
   * Jede Einheit hat ≥1 Tag, es gibt ein explizites `UNASSIGNED`/`CROSS-CUTTING`, und Rückläufer gehen in eine Nachbearbeitung.
   * Die Vereinigung aller Scopes ergibt wieder die Menge aller Einheiten.

3. **Tagging:** Tagger dürfen nur Tags aus dem Vokabular nutzen und geben nur IDs und Tags aus, nie Text. Neue Tags sind als Vorschlag erlaubt, werden aber erst nach einem Merge-Schritt aufgenommen. So driftet das Vokabular nicht.
4. **Weiteres Schneiden:** Das geht rekursiv. Ein Scope, der zu groß ist, bekommt ein eigenes Sub-Vokabular, und der gleiche Tagging-Pass läuft nur über seine Einheiten.
5. **Kanten:** Es gibt zwei Quellen, die du kombinieren kannst:

   * **Strukturell:** Ko-Tagging (eine Einheit trägt Tags A und B → Kante A–B, gewichtet nach Häufigkeit).
   * **Semantisch:** ein weiterer Pass, der pro Einheit `defines`/`uses`/explizite Referenzen extrahiert. Daraus entstehen gerichtete Abhängigkeiten zwischen Aspekten.

   Die Kanten-Tabelle (Aspekt A → Aspekt B, belegt durch Einheiten-IDs) ist der Kongruenz-Vertrag.

Das Hauptrisiko ist Tag-Granularität: Zu grob ergibt riesige Scopes, zu fein ergibt viel Kopplung. Die Kennzahlen dafür sind Scopegröße und Anteil der Einheiten mit vielen Tags. Wenn diese schlecht sind, gehören das Vokabular und damit die Tagging-Pässe überarbeitet. Die gecachten Pässe bleiben davon unberührt, nur die betroffenen Teile müssen neu laufen.

### You said:

Kannst du das in Operationen in einer Liste beschreiben (Pseudoalgorithmus)?

### Other responded:

```
EINGABE: Dokument D
AUSGABE: Scopes S (Tag → Einheiten-IDs), Kanten E (Aspekt → Aspekt, belegt durch IDs)

── Phase 0: Atomisieren (deterministisch)
1.  U ← split(D) an Überschriften/Absätzen/Listenpunkten
2.  für jedes u in U: u.id ← stabile ID, u.range ← Zeilenbereich
3.  C ← gruppiere U zu Chunks (Budget z. B. 300–500 Zeilen, an Einheitsgrenzen)

── Phase 1: Vokabular (parallel über C, dann Merge)
4.  für jeden Chunk c in C (parallel):
        cand[c] ← LLM: schlage Kandidaten-Tags + Kurzdefinition vor
5.  V ← LLM: merge(alle cand) → geschlossenes Vokabular
        je Tag: Name, Definition, Abgrenzung, 1–2 Beispiel-IDs
6.  (optional) G ← Glossar der definierten Begriffe aus cand

── Phase 2: Tagging 1:N (parallel)
7.  für jeden Chunk c in C (parallel):
        für jede Einheit u in c:
            u.tags ← LLM: wähle 1..N Tags aus V  (nur Tags aus V, nur IDs ausgeben)
                        sonst "CROSS-CUTTING" oder "UNASSIGNED"
            u.proposals ← LLM: ggf. Vorschlag neuer Tag (nicht aktiv)
8.  falls Vorschläge vorhanden: V ← merge(V, Vorschläge); nur betroffene Einheiten neu taggen

── Phase 3: Invarianten (deterministisch)
9.  prüfe: jede Einheit hat ≥1 Tag
10. prüfe: ⋃ Scopes == alle IDs  (verlustfrei)
11. UNASSIGNED ≠ ∅ → nachtaggen oder V erweitern, zurück zu 7
12. Kennzahlen: Scopegröße, Tags pro Einheit
        zu groß → Schritt 13; zu viele Multi-Tags → V gröber/feiner nachziehen, zu 5

── Phase 4: Scopes bilden und rekursiv schneiden
13. S[t] ← {u.id | t ∈ u.tags} für alle t in V
14. für jeden Scope s mit Größe > Budget:
        V_s ← Phase 1 nur auf s
        Phase 2 und 3 nur auf s  (Sub-Scopes)
        S ← ersetze s durch Sub-Scopes

── Phase 5: Kanten
15. Ko-Tagging: für jede Einheit u, für jedes Tagpaar (a,b) in u.tags:
        E[a,b].gewicht += 1,  E[a,b].belege += u.id
16. Semantisch (parallel über C):
        für jede Einheit u: u.defines, u.uses, u.refs ← LLM (Begriffe/IDs, kein Text)
17. für jede Einheit u, jedes x in u.uses, y = definierende Einheit von x:
        wenn Aspekt(u) ≠ Aspekt(y): E[Aspekt(u) → Aspekt(y)].belege += (u.id, x, y.id)
18. Prüfung: jeder Begriff in uses hat genau eine definierende Einheit
        (0 → Lückenmeldung, >1 → Konfliktmeldung)

── Phase 6: Ausgabe
19. je Scope: ID-Liste → Text per Slicing aus D, Glossar-Subset, ein-/ausgehende Kanten
20. Vertragstabelle = E (Aspekt A → Aspekt B, Begriff, Beleg-IDs)
```

Die Pässe in den Schritten 4, 7 und 16 sind pro Chunk gecacht, bei einer Vokabularänderung laufen nur die betroffenen Teile neu.

### You said:

Was ist dann die Ausgabe? EIne flache Liste der Atome?

### Other responded:

Fast: Die Grundausgabe ist eine flache Tabelle der Atome mit Annotationen. Alles andere wird daraus abgeleitet.

**1. Atomtabelle (Ground Truth)**

| id    | range     | tags\[]        | defines\[] | uses\[]        | refs\[]  |
| ----- | --------- | -------------- | ---------- | -------------- | -------- |
| u0412 | 2210–2218 | \[auth, audit] | \[Session] | \[User, Rolle] | \[u0033] |

Der Text steht nicht in der Tabelle. Er bleibt unverändert in D und wird über `range` geholt.

**2. Vokabular V** (Tag, Definition, Abgrenzung, Beispiele, ggf. Eltern-Tag für die Hierarchie aus den rekursiven Schnitten)

**3. Abgeleitete Sichten (aus 1+2 berechnet, nicht separat gepflegt):**

* **Scope-Index:** Tag → Liste der IDs (invertierter Index). Daraus entsteht pro Aspekt die Slice-Datei: die Einheiten in Dokumentreihenfolge, plus Glossar-Subset.
* **Kantentabelle:** (Aspekt A → Aspekt B, Begriff, Beleg-IDs, Gewicht)
* **Glossar:** Begriff → definierende Einheit

**4. Berichte** (Invarianten, Lücken und Konflikte, Scopegrößen)

Die Atomtabelle ist verlustfrei und nach dem Tagging und dem Extraktionspass unveränderlich, außer bei Re-Tagging. Scopes und Kanten sind Views. Ein Schnitt anders zu legen heißt damit nur, die Views neu zu berechnen. Materialisiert werden Slices erst, wenn ein Agent sie braucht. Das Ergebnis ist also nicht eine Liste von Teilen, sondern ein annotierter Index, aus dem beliebig viele Zerlegungen abfragbar sind.

### You said:

Ich hätte das gern besser agentisch verwendbar. Die Atome können in einzelne Markdown-Dokumente mit ihren Metadaten Frontatmatter.\
Darauf aufbauend kann eine weitere Phase Agenten direkt auf Basis der Metadaten und einer Übersichtsdatei alle Atome neu schneiden und gruppieren.\
Also Beispielweise:

* Anforderungen trennen und verschieben
* Neue Atome extrapulieren und einfügen
* Cluster und Komponenten bilden
* Hierarchienbilden auf basis der Metadaten doer Cluster

Was denkst du?

### Other responded:

Die Idee ist gut, weil sie die Kernoperationen als Dateisystemarbeit abbildet, womit Agenten gut umgehen können (grep, Edit, Git-Diff, Parallelisierung, menschlich prüfbar). Entscheidend sind fünf Punkte:

1. **Atome nicht umschreiben, sondern ableiten**

   * Original-Atome sind im Body unveränderlich (`origin: source`, `src_range`, `text_hash`).
   * Split/Merge erzeugen neue Atome mit `derived_from: [ids]`, die alten werden per Tombstone (`status: superseded`, `superseded_by`) markiert, nie gelöscht.
   * Extrapolierte Atome bekommen `origin: inferred` und `basis: [ids]`, damit sie klar von verbatim Anforderungen trennbar sind.

2. **Geschlossener Operationssatz statt freier Edits**

   * `split`, `merge`, `move`, `retag`, `create_inferred`, `cluster`, `set_parent`.
   * Agenten schreiben Operationen in ein Ops-Log (oder direkt in Frontmatter über ein Skript), und ein deterministischer Applier führt sie aus.
   * Das ergibt Nachvollziehbarkeit, Wiederholbarkeit und kaum Schreibkonflikte zwischen parallelen Agenten (jeder besitzt disjunkte Atom-Mengen oder eigene Branches).

3. **Validator nach jeder Phase (deterministisch)**

   * Coverage: Die Original-Zeilenbereiche sind vollständig und überlappungsfrei durch aktive Atome oder deren Nachfolger abgedeckt.
   * IDs eindeutig, alle `refs`/`uses`/`parent` lösen auf, keine verwaisten Tombstones.
   * Verbatim-Check per Hash, Zyklen in der Hierarchie, Clustergrößen.
   * Das ersetzt die Verlustfreiheit, die sonst nur Vertrauen wäre.

4. **Übersichtsdatei generieren, nicht pflegen**

   * Bei Tausenden Atomen sprengt eine flache Übersicht den Kontext. Daher mehrstufig: Index → Cluster-Summaries → Atome.
   * Alles wird per Skript aus dem Frontmatter erzeugt, Agenten lesen es und ändern es nie von Hand.
   * Der Agent lädt nur die Ebene, die er braucht (z. B. ein Cluster plus dessen Schnittkanten).

5. **Hierarchie in Metadaten, nicht in Ordnern**

   * Dateien bleiben flach (`atoms/u0412.md`). `cluster`, `parent` und `component` stehen im Frontmatter, denn Verschieben im Dateisystem bricht Referenzen und Diffs.
   * Ordneransichten sind generierte Views.

Das Minimalschema könnte so aussehen:

```
id, origin(source|inferred), src_range, text_hash, status,
derived_from[], superseded_by, basis[],
tags[], cluster, parent, component,
defines[], uses[], refs[]
```

Offen bleibt die Reihenfolge der Phasen. Cluster und Hierarchie sollten erst nach Abschluss von Split/Merge entstehen, sonst ändern sich IDs unter den Gruppen. Bei jeder Änderung des Atomschnitts müssen `uses`/`defines` für die betroffenen Atome neu extrahiert werden.



### You said:

Die Atome müssen umgeschrieben werden. Das Ziel ist agentische Umsetzung. Am Ende kommen im besten Fall isolierte Arbeitspakete heraus und separate Arbeitspakete für die Integration von Artefakten.\
Rückverfolgung sollte nur für den Operator möglich sein. Nicht den Agenten, sonst erzeugt man ein inverses Problem und der Agent stolpert bei der Umsetzung den Anforderungsbaum hoch und runter. Genau das, was den Kontext und die Performance massiv degradieren lässt.

### Other responded:

Dann trenne ich zwei Ebenen, und das ändert das Design an drei Stellen.

**1. Operator-Ebene (Herkunft) und Agent-Ebene (Arbeitspakete) getrennt**

* **Operator-Ebene:** Atome mit Frontmatter, Tombstones, `derived_from`, Ops-Log, Validatoren und das Trace-Ledger (Paket → Atome → Quellbereiche). Das liegt in einem separaten Verzeichnis oder einer separaten Datenbank, das Agenten nie sehen.
* **Agent-Ebene:** Ein generierter Export, der nur Pakettext, Verträge und Glossar enthält. Es gibt keine Quell-IDs, keine `parent`-/`derived_from`-Felder und keine Verweise auf andere Pakete außer über Verträge. Paket-IDs sind opak und lokal.

**2. Atome werden zu eigenständigen Aussagen umgeschrieben**

Ein Atom ist fertig, wenn es ohne Kontext verständlich ist:

* Pronomen und Verweise (“wie oben”, “siehe 3.2”) sind aufgelöst.
* Verwendete Begriffe sind inline definiert oder per Glossar-Subset im Paket enthalten.
* Implizites Kontextwissen aus dem Eltern-Abschnitt ist ausgeschrieben.
* Es enthält genau eine prüfbare Aussage mit Akzeptanzkriterium, wo möglich.

Verlustfreiheit kommt dadurch nicht mehr aus Hash-Vergleich, sondern aus Prüfungen auf der Operator-Seite:

* **Abdeckung:** Jedes Quellatom hat ≥1 umgeschriebenes Nachfolgeatom (Ledger-Check), und jedes neue Atom hat Quelle oder ist als `inferred` markiert.
* **Treue:** Ein unabhängiger Prüf-Agent bekommt Original und Umschreibung und meldet Hinzufügungen, Auslassungen und Bedeutungsverschiebungen.
* **Round-Trip:** Aus den Umschreibungen wird die Aussage des Originals rekonstruiert und verglichen.
* Inferierte Atome gehen vor der Übernahme in ein Paket durch den Operator.

**3. Pakete statt Baum**

Aus Clustern entstehen Pakete mit fester Form:

* **Fachliches Paket:** Zielbeschreibung, umgeschriebene Anforderungen (flach, nach Priorität oder Abhängigkeit geordnet), Glossar-Subset, Eingangs- und Ausgangsverträge, Akzeptanzkriterien, Definition of Done.
* **Vertrags-Artefakte:** Schnittstellen, Datenmodelle und Events werden vor den Paketen aus den Schnittkanten erzeugt. Beide Seiten bekommen sie als fix vorgegeben. Ändern darf sie nur der Operator.
* **Integrationspaket:** Pro Schnittkante oder Cluster-Gruppe. Eingabe sind die Verträge und die Ergebnisse der beteiligten Pakete, Aufgabe ist Verdrahtung und Vertragstests. Es kennt die Pakete nur über deren Artefakte, nicht über die Anforderungen.

**Ablauf**

```
1. Atomisieren (Operator, deterministisch)
2. Umschreiben zu eigenständigen Atomen (Agenten, parallel, Ops-Log)
3. Treue- und Abdeckungsprüfung (Operator)
4. Cluster/Komponenten bilden (Metadaten-basiert)
5. Verträge aus Schnittkanten ableiten und freigeben
6. Pakete + Integrationspakete kompilieren (Export ohne Herkunft)
7. Ausführung: nur der Agent-Export wird übergeben
8. Ergebnisse zurück ins Ledger (Operator)
```

**Folge für Änderungen:** Ändert sich eine Anforderung, läuft die Wirkungsanalyse über das Ledger (betroffene Atome → Pakete → Verträge). Betroffene Pakete werden neu kompiliert und neu ausgeführt. Der Agent erfährt nur die neue Paketfassung, nicht die Historie.

**Hauptrisiko:** Beim Umschreiben driftet die Bedeutung, und das fällt später nicht mehr auf, weil Agenten nicht zurückverfolgen können. Deshalb gehören die Treueprüfung (Schritt 3) und Akzeptanzkriterien als ausführbare Tests ins Paket. Sie sind das einzige Gegengewicht auf Agentenseite.

### You said:

Gibt es für diesen Bereich Papers?\
Ich kenne Dinge wie die agentische Neufassung des GCC. Was ich hier beschreibe, ist jedoch eine andere Zielsetzung. Es geht nicht darum, mit möglichst wenig Regeln und Harness, Agenten vollständig autonom entwickeln zu lassen, sondern um eine möglichst sichere und effiziente Entwicklung. Der Harness kann in meinem Fall ausgeprägt und algorithmisch sein mit deterministischen Komponenten, Gates und Boundaries.

### Other responded:

Ich habe keine Arbeit gefunden, die deine Pipeline als Ganzes beschreibt (Atomisierung, Aspekt-Tagging, Umschreiben zu kontextfreien Atomen, Arbeitspakete, Trace nur für den Operator). Es gibt aber Arbeiten zu den einzelnen Bausteinen. Ich habe nur Abstracts und Snippets gesehen, nicht die Volltexte.

**Harness, Gates, Deterministik (deine Zielsetzung)**

* [A Deterministic Control Plane for LLM Coding Agents](https://arxiv.org/html/2606.26924v1) (2606.26924): phasengesteuerter Zustandsautomat, Requirements hash-adressiert bei Freigabe, Trace Requirement→Datei→Test, Governance bewusst nicht an LLMs delegiert. Das ist inhaltlich am nächsten an deinem Ansatz.
* [Verification as an Architectural Layer for LLM Agents](https://arxiv.org/abs/2609.31937) (2609.31937): V-Modell, Gate (deterministisch) und Judge (LLM) getrennt. Im Pilot kamen 8 von 9 Korrekturen von deterministischen Gates, und Planung verschlechterte das Ergebnis, sobald Verifikation vorhanden war.
* [Deterministic Envelope / Controlled Variation Space](https://arxiv.org/html/2610.06255v1) (2610.06255): LLM-Variation wird in einem begrenzten Raum gehalten, Schema-Gates dienen als Ausgangsbedingung.
* Reddy et al., *Reason less, verify more* (2607.07405): deterministische Gates gegen stille Policy-Verletzungen (nur als Referenz in einer Literaturliste gesehen, nicht selbst gelesen).

**Anforderungen → Cluster → Architektur/Pakete**

* [ARLO](https://arxiv.org/pdf/2504.06143) (2504.06143): Embeddings und hierarchisches Clustering der Requirements, um LLM-Aufrufe zu sparen, dann Architekturableitung pro Cluster.
* [Cluster-Aware Dual-Level Test Specification](https://arxiv.org/html/2606.17197) (2606.17197): UMAP + HDBSCAN auf Requirements. Clustering plus Summarization gilt dort als Stand der Technik für große Requirement-Korpora.
* [iReFeed](https://arxiv.org/html/2603.28677v1) (2603.28677): Innerhalb eines Themen-Clusters fand das LLM mehr “requires”-Beziehungen, als wenn man alles auf einmal gibt. Das stützt die clusterlokale Abhängigkeitsextraktion.
* [RADIANT](https://arxiv.org/html/2607.16708v1) (2607.16708): Requirement-zu-Verifikation mit Req2X-Trace. Multi-Agent-Zerlegung verbessert die syntaktische Validität, der semantische Gewinn ist modellabhängig.
* [Contract-Coding](https://arxiv.org/html/2604.13100) (2604.13100): “Language Contract” als symbolische Zwischenschicht für Repo-Generierung.
* [GraphRAG](https://emergentmind.com/papers/2404.16130) (Edge et al., 2404.16130): hierarchische Leiden-Partitionierung mit vorberechneten Summaries. [k-core-Variante](https://arxiv.org/html/2603.05207) (2603.05207): Leiden ist auf dünnen Graphen instabil, k-core liefert deterministische, ausgewogenere Hierarchien. Das ist relevant, wenn deine Partitionierung reproduzierbar sein soll.
* [MetaGPT](https://ar5iv.labs.arxiv.org/html/2308.00352): strukturierte Zwischenartefakte (PRD, Interfaces) erhöhen die Erfolgsrate.

**Evidenz für deine Designentscheidungen**

* [Citation Discipline in Spec-Driven Development](https://arxiv.org/pdf/2606.30689) (2606.30689): Inline-Zitate (Rückverweise) im Kontext senken die Ausgabe-Determinismus signifikant (Cohen’s d ≈ −0.7 bei Claude und GLM). Das stützt direkt, den Trace vom Agenten fernzuhalten.
* [Adaptive Task Graphs](https://arxiv.org/pdf/2605.06320) (2605.06320): Wird die Zerlegung vor der Ausführung festgeschrieben, ist sie bei Aufgaben mit erst später sichtbarer Struktur systematisch falsch, und der SOP-Ansatz kennt keine Revision. Das ist das Risiko deiner festen Pakete, daher braucht der Operator einen Replanning-Pfad.
* SAFEdit (2604.25737): Zerlegung in Planner/Coder/Tester löst Zuverlässigkeitsprobleme nicht automatisch (nur über eine Sekundärquelle gesehen).

**Überblick:** [LLM-Based Multi-Agent Systems for SE](https://arxiv.org/pdf/2404.04834) (2404.04834), [NLP4RE-Survey](https://arxiv.org/html/2401.01508v1) (2401.01508, Klassifikation/Clustering von Requirements).

**Lücken in der Literatur:** Ich habe nichts gefunden zum Umschreiben von Requirements in kontextfreie Atome mit Treueprüfung und nichts zur Aufteilung in Operator- und Agentenebene für Traceability. Letzteres stützt nur indirekt das Citation-Discipline-Paper.

Result Stats: id=ee75d982-bdae-4822-b272-2c098ea082d6, total: 251160, in: 60804, out: 190356, reason: 174474, read: 0, write: 0, start: 1793310245519, end: 1793311986125, time: 1738911, total: 1738911, io: 835, control: 1695, sent: 1, recv: 4851
SystemInit:  id=ee75d982-bdae-4822-b272-2c098ea082d6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-5-5
ReasoningToken: 128000
ReasoningToken: 46474

---

## References

References already listed in the position paper are not repeated. The 2026 preprints in Table 5 are omitted until they are verified.

- Baniassad, E., & Clarke, S. (2004). Theme: An approach for aspect-oriented analysis and design. *Proceedings of ICSE 2004*, 158–167.
- Blondel, V. D., Guillaume, J.-L., Lambiotte, R., & Lefebvre, E. (2008). Fast unfolding of communities in large networks. *Journal of Statistical Mechanics*, P10008.
- Edge, D., et al. (2024). From local to global: A graph RAG approach to query-focused summarization. arXiv:2404.16130.
- Gotel, O., & Finkelstein, A. (1994). An analysis of the requirements traceability problem. *Proceedings of ICRE 1994*, 94–101.
- He, J., Treude, C., & Lo, D. (2024). LLM-based multi-agent systems for software engineering: Literature review, vision and the road ahead. arXiv:2404.04834.
- Hong, S., et al. (2024). MetaGPT: Meta programming for a multi-agent collaborative framework. *ICLR 2024*.
- Hubert, L., & Arabie, P. (1985). Comparing partitions. *Journal of Classification*, 2, 193–218.
- ISO/IEC/IEEE 29148:2018. *Systems and software engineering — Life cycle processes — Requirements engineering.*
- Krippendorff, K. (2004). *Content analysis: An introduction to its methodology* (2nd ed.). Sage.
- Marin, M., van Deursen, A., & Moonen, L. (2007). Identifying crosscutting concerns using fan-in analysis. *ACM Transactions on Software Engineering and Methodology*, 17(1).
- Mavin, A., Wilkinson, P., Harwood, A., & Novak, M. (2009). Easy approach to requirements syntax (EARS). *Proceedings of RE 2009*, 317–322.
- Rashid, A., Moreira, A., & Araújo, J. (2003). Modularisation and composition of aspectual requirements. *Proceedings of AOSD 2003*, 11–20.
- Traag, V. A., Waltman, L., & van Eck, N. J. (2019). From Louvain to Leiden: Guaranteeing well-connected communities. *Scientific Reports*, 9, 5233.

---

**Anmerkungen zum Entwurf**

- **Einordnung:** Der Entwurf ist als Begleitpapier zur These angelegt. Er konkretisiert §6 (Schritte 1–4) und §10 der These, und H8–H12 setzen deren Hypothesennummerierung fort.
- **Angleichungen an die These:**
  - `CROSS-CUTTING` ist kein eigener Querschnittsaspekt mehr, sondern eine Queue, die in Kanten aufgelöst wird (P7).
  - Hierarchie gibt es nur als Kontraktion bzw. View (§5.1, §6.1).
  - Provenienz steht nicht im Frontmatter, sondern im Ledger. Sie ist damit strukturell verborgen und nicht nur weggefiltert.
- **Ergänzungen:** Kongruenz ist operational als C1–C4 definiert. Dazu kommen Reason Codes für Blocker-Reports, ein lokaler Widerspruchscheck (aus dem früheren Concept-Stand), ein durchgehendes Beispiel sowie Fault Injection für das Fidelity-Gate.
- **Modellangaben:** Konkrete Kontext- und Output-Limits habe ich bewusst weggelassen und als Parameter behandelt.
- **Literatur:** Die 2026-Preprints aus dem Concept konnte ich nicht prüfen, sie liegen teils nach meinem Wissensstand. Sie sind deshalb als „to verify“ markiert und stehen nicht in den Referenzen. Vorrangig zu prüfen ist 2606.30689 (Citation Discipline), weil sich H10 und §2.4 darauf stützen.
- **Artefakt in der These:** In §10 steht mitten im Satz „ReasoningToken: 112490 / ReasoningToken: 10787“. Das ist vermutlich ein Copy-Paste-Rest.
