Arbeite folgendes Material zu einer These um, in Markdown und englischer Sprache:

# Trigramm-Methode & Multi-Projektions-Suchmethodologie

## 1. Ausgangspunkt: Die Trigramm-Methode

Grundverfahren: Für jede Datei wird eine Signatur gebildet, indem alle enthaltenen Trigramme gezählt werden. Die häufigsten 10 % werden entfernt, die verbleibenden 90 % bilden eine Bitmap (Signatur).

**Ursprüngliche Kritik:** Das lokale (datei-interne) Entfernen der häufigsten Trigramme erzeugt Falsch-Negative, weil ein Trigramm in Datei A entfernt, in Datei B aber erhalten sein kann — die Anfrage kann beide Fälle nicht unterscheiden (0-Bit ist überladen: "nicht enthalten" vs. "entfernt").

**Vorgeschlagene Lösungen (klassische Sicht):**
- Globale Dokumentfrequenz (df) statt lokaler Zählung — verworfen, weil ein globaler Korpus sprachliche/strukturelle Homogenität unterstellt, die nicht gegeben ist.
- Entfernungsmenge R explizit mitführen (Seiteninformation) — korrekt, aber mit Speicherkosten.
- Winnowing/Minimizer — deterministische, inhaltsunabhängige Auswahl mit Garantie.

---

## 2. Reformulierung der Verfahrenssemantik

Die entscheidende Wende der Diskussion: Das Verfahren ist **kein Containment-Test** ("ist X in Datei D enthalten?"), sondern ein **Ähnlichkeits-Schwellwert-Filter** (Overlap-Threshold):

> d ist Kandidat, wenn |T(q) ∩ S(d)| ≥ θ — ohne Ranking, ohne Sortierung.

Unter dieser Spezifikation sind "Falsch-Negative" kein Fehler, sondern eine **bewusste Skopierung** (Domänengrenze) der Methode: Anfragen, die überwiegend aus für die Zieldatei typischem "Hintergrund" (häufige Trigramme) bestehen, sind durch diese Methode prinzipiell nicht sinnvoll trennbar — analog zur Sternenhimmel-Analogie: Man kann keinen Stern in einem Himmelsausschnitt suchen, der aus lauter Sternen besteht; der Hintergrund muss subtrahiert werden, bevor eine Schwelle überhaupt trennen kann.

**Effekt des Entfernens (Reformuliert als Nutzen, nicht Verlust):**
- Es senkt den Zufallsoverlap zwischen Dateien desselben Korpus (gemeinsamer "Grundton"), wodurch echte Treffer deutlicher vom Rauschen abgehoben werden.
- Besonders wirksam bei Dateien aus ähnlichem Korpus, weil dort lokale und klassenweite Häufigkeit stark korrelieren — die Ähnlichkeit des Korpus liefert implizit die Konsistenz, die sonst eine globale Stopp-Menge liefern müsste.
- Die Entfernungsquote (10 %) hat ein Optimum: zu wenig entfernt bringt keinen Trennschärfegewinn, zu viel beginnt selektive (Zipf-Schwanz-)Trigramme zu treffen und schadet echten Treffern stärker als Rauschen.

---

## 3. Architektur: Mehrkanal-Suche ohne Filter-Kette

Das System besteht aus mehreren **unabhängigen, parallel laufenden Projektionen** (Retrieval-Kanälen), deren Ergebnisse **additiv vereinigt**, nicht gefiltert oder sortiert werden:

1. **Exakte Textsuche** — Containment, keine Toleranz.
2. **Trigramm/10%-Signatur** — Mengenüberlappung mit Schwelle θ, Frequenzunterdrückung.
3. **Multivektor Late Interaction** (ColBERT-Familie) — gelernte Token-Ähnlichkeit (MaxSim), fremdes Wissen (Encoder-Training).
4. **AST-Struktursuche** — syntaktische Baumstruktur, invariant gegenüber Bezeichnern/Layout.
5. **Korpus-trainierte Vektoren** — wie (3), aber Wissensquelle ist der Korpus selbst statt ein fremdtrainiertes Modell.

Kein Kanal filtert einen anderen vor; alle laufen unabhängig auf dem vollen Korpus. Die Ausgabe ist die Vereinigung aller Kandidatenmengen, jedes Element trägt eine **Herkunfts-Deklaration** (welcher Kanal, welche Parameter, welche Rohmerkmale — Overlap-Wert, MaxSim-Score, Strukturmatch).

**Kein Ranking — Begründung:**
- Relevanz ist eine private Funktion des Anfragenden. Jedes systemseitige Ranking ist nur ein Schätzer dieser Funktion und kann sie nicht übertreffen, wenn der Anfragende sie selbst anwenden kann.
- Ranking ist unmöglich, wenn die Relevanzfunktion dem Anfragenden selbst nicht explizit ist (Anomalous State of Knowledge / Berrypicking) — jede Antizipation einer unbekannten Dimension ist logisch unmöglich, da das Maß der Annäherung bereits die Dimension selbst wäre.
- Einzige Ausnahme: Die Anfrage liefert ihre Bewertungsfunktion selbst mit (expliziter Scoring-Mechanismus) — dann ist "Ranking" nur delegierte Auswertung, keine Antizipation.

---

## 4. Erkenntnistheoretische Grundlegung: Es gibt keinen Nullpunkt

Zentrale Einsicht der Diskussion: Nicht nur Ranking, auch **jede Repräsentation** (Trigramm, Bitmap, sogar "Rohdaten" als Bytes/Dateigrenzen) ist bereits eine Projektion/Antizipation. Es gibt keine neutrale Zwischenstufe und keinen Nullpunkt — Rohdaten sind selbst eine Kodierung.

**Konsequenzen:**
- Korrektheit eines Index lässt sich nicht absolut, sondern nur relativ zwischen Projektionsebenen definieren (Konsistenz statt Soundness im absoluten Sinn).
- Die einzig erreichbare Eigenschaft ist **Transparenz der Projektion** (Deklaration: Operator, Parameter, Reihenfolge der Schritte), nicht Neutralität.
- Da jede einzelne Repräsentation etwas verwirft, ist die einzig konsistente Strategie: **möglichst diverse Repräsentationen implementieren**, die den theoretisch möglichen Raum von Projektionen abdecken, statt eine "richtige" zu suchen.

---

## 5. Der Raum der Projektionen: Achsen und Abstand

Die implementierten Verfahren lassen sich auf Achsen abbilden. Eine erste, 1-dimensionale Achse ("Toleranz / Abstraktionsgrad von der Zeichenfolge"):

| Verfahren | Toleranz | Invariante |
|---|---|---|
| Exakt | keine | Reihenfolge & Identität der Zeichen |
| Trigramm/10% | lokal, mengenartig | Zeichennachbarschaft, nicht Position/Reihenfolge |
| Multivektor | gelernt, kontinuierlich | Token-Bedeutung, nicht Schreibweise |

Mit AST und Korpus-Vektoren ergibt sich eine mindestens 4-dimensionale Achsenübersicht:

| Achse | Exakt | Trigramm/10% | Multivektor | AST | Korpus-Vektoren |
|---|---|---|---|---|---|
| Toleranz | keine | mengenartig | gelernt | strukturell | gelernt |
| Strukturbindung | Zeichen | Zeichen | Token | Baum | Segment |
| Wissensquelle | keine | keine | fremd | Grammatik | Korpus |
| Hintergrundunterdrückung | nein | explizit | implizit | nein | implizit |

**Messbarkeit der Diversität:** "Gleichmäßige Abdeckung" braucht ein Maß. Da es keine ausgezeichnete Metrik über Projektionen gibt, wird sie aus dem **Verhalten der Verfahren selbst** konstruiert: Abstand zweier Verfahren = Jaccard-Distanz ihrer Ergebnismengen über Samples und Korpus. Ein neues Verfahren ist wertvoll in dem Maß, wie weit es von allen vorhandenen entfernt ist (Maximin-/Farthest-Point-Auswahl). Der Rang der Abstandsmatrix schätzt die effektive Dimension der Abdeckung; Redundanz zeigt sich an niedrigem Rang.

**Interpolation:**
- Innerhalb einer Parameterfamilie (Entfernungsquote, θ, n-Gramm-Länge) ist Interpolation wohldefiniert (Kontinuum).
- Zwischen Verfahrensfamilien gibt es keine kanonische Zwischenstufe; "Interpolation" bedeutet dort nur, dass Vereinigung/Kontrast der Mengen eine dazwischenliegende Projektion approximiert — nur gültig, wenn der Ergebnisraum hinreichend glatt ist (messbar: wächst Mengenabstand mit Parameterabstand?).

**Lücken im Raum** werden sichtbar als Dateipaare, die von keinem Verfahren unterschieden werden, sich im Rohinhalt aber unterscheiden. Identifizierte offene Achsen: Editierabstand/Fuzzy-Matching (zwischen Exakt und Trigramm), Token-Normalisierung/BM25-artige Überlappung (zwischen Trigramm und Multivektor), Layout/Kompressibilität (Normalized Compression Distance), Graphbeziehungen (Importe, Referenzen), Zeit/Herkunft (Metadaten, Historie).

---

## 6. Zwei-Phasen-Interaktionsmodell

**Phase 1 — Exploration (unspezifisch):**
- Client gibt Sample oder vage Anfrage, Volumen N und Verteilung w über Verfahren vor.
- System liefert pro Verfahren i höchstens n_i = w_i · N Kandidaten, Sampling-Rate = n_i / |C_i|.
- Ist |C_i| ≤ n_i, ist die Ausgabe vollständig (Rate 1) — das wird explizit gemeldet, da es qualitativ verschieden von einer Stichprobe ist (Aussagen über Abwesenheit werden möglich).
- Rückmeldung pro Verfahren: |C_i|, n_i, Rate, Seed — damit auch unvollständige Mengen interpretierbar bleiben.
- Restbudget-Regel (wenn ein Verfahren sein Kontingent nicht ausschöpft): Default = Verfall (Verteilung bleibt gewahrt), alternativ Umverteilung (Volumen bleibt gewahrt, Verteilung verzerrt) — Wahl liegt beim Client.
- Stichprobenart wählbar: gleichverteilt (zeigt dominante Cluster) vs. diversitätsorientiert / Farthest-Point über Signaturabstand (zeigt Randbereiche).
- Deterministisches Sampling (Hash mit Seed) statt Zufall — reproduzierbar, nachziehbar ("mehr" liefert Fortsetzung derselben Reihenfolge).

**Phase 2 — Spezifikation (nach erreichter Explizierbarkeit):** Der Client wählt einen von vier Pfaden:
1. Anfrage verfeinern (neue Samples als Kontrast, positiv/negativ).
2. Scoring formulieren (eigene Bewertungsfunktion über gelieferte Rohmerkmale; System wertet aus oder liefert Rohdaten zur clientseitigen Auswertung).
3. Einschränken auf Verfahren/Achsen/Parameterbereiche.
4. Weiter explorieren (andere Samples, Auflösungsstufen, Signatur-Landkarten).

**Darstellungsregel:** Feste, bedeutungslose Reihenfolge (z. B. alphabetisch nach Verfahrenskennung), gleichrangige Darstellung — jede Hervorhebung (auch durch Sortierung oder Mehrfachtreffer-Betonung) wäre bereits eine Gewichtung durch die Hintertür.

---

## 7. Cache als Inspektionsraum

Der Cache speichert Suchläufe (Kandidatenmengen, Rohmerkmale, Herkünfte) vollständig, nicht nur die ausgewählte Stichprobe. Das trennt:
- **Suchlauf** (teuer, einmalig),
- **Sampling & Scoring** (billig, beliebig oft wiederholbar auf dem Cache).

**Konsequenzen:**
- Suchläufe selbst werden zu einem durchsuchbaren Objekt — dieselben Projektionsprinzipien (Kontrast, Überschneidung, Landkarte) lassen sich auf frühere Läufe anwenden.
- Cache-Schlüssel = (Korpusstand, Verfahrensversion, Parameter) — sonst vermischen sich Projektionen unbemerkt. Veralterung wird markiert, nicht stillschweigend ersetzt.
- Aufbewahrungsregel (was bei Platzdruck verworfen wird) ist selbst eine Antizipation und muss deklariert oder dem Client überlassen werden (Pinning).
- Nutzerurteile (Bestätigung/Verwerfung) werden mit Zeitstempel und Runde geführt, revidierbar, ohne Neuberechnung.

---

## 8. Konvergenzkriterium statt Effizienzkriterium

Zentrale Verteidigungslinie des Ansatzes gegen das Probabilistic Ranking Principle (das Ranking als optimal unter Unsicherheit und begrenzter Nutzeraufmerksamkeit begründet):

> Erschöpfende Bewertung durch den Nutzer ist akzeptabel, **solange jede Iteration den Suchraum ein Stück verfeinert, statt etwas zu antizipieren, das der Nutzer nicht bestätigt hat.**

**Anforderungen:**
- Monotone Verfeinerung: Bestätigtes bleibt bestätigt, Verworfenes bleibt verworfen (Cache-Garantie).
- Bestätigung ist die einzige legitime Quelle für Verengung. Aus Nutzerurteilen abgeleitete Kontraste ("diese Merkmale trennen deine bestätigten von verworfenen Samples") sind Vorschläge, keine automatischen Anwendungen.
- System kann Stagnation erkennen und melden (verworfene und bestätigte Samples sind in allen Sichten ununterscheidbar) — das ist ein Befund über fehlende Abdeckung, keine Bewertung.

**Skalierungsmodell — zwei entscheidende Größen:**

1. **Korpusgröße N:** Untere Schranke für Rundenzahl ≈ log₂(N/k) Bit, wenn jede Runde die Kandidatenmenge um einen konstanten Faktor teilt (saubere Trennung durch Projektionen). Gelingt das nicht, wächst die Rundenzahl linear mit N.
2. **Repräsentationsdistanz d:** Abstand zwischen der vom Nutzer gemeinten Dimension und den nächstliegenden implementierten Projektionen (gemessen an Ergebnis-Differenzmengen).
 - klein d → Konvergenz in wenigen Runden,
 - mittleres d → Zielmenge nur als Schnitt/Kombination mehrerer Projektionen approximierbar (Interpolationsfall),
 - großes d → Stagnation unabhängig von N (keine Kombination trennt die Dimension).

**Wechselwirkung:** Große N verstärken kleine d-Defizite — bei großem Korpus sinkt der Treffer-Anteil im Sample, Präzision der Einzelverfahren wird wichtiger. Bei kleinem Korpus kann der Nutzer auch schwache Projektionen durch erschöpfende Sichtung kompensieren.

**Zu prüfende Kernhypothese:** Rundenzahl ~ log N bei d klein/mittel, mit klarem Stagnationssignal bei d groß.
- Fällt der erste Teil linear aus → Methode nur für kleine/mittlere Korpora tragfähig.
- Fehlt das Stagnationssignal → Nutzer kann nicht unterscheiden, ob weiter explorieren oder neues Verfahren fordern.

---

## 9. Verwandte / etablierte Ansätze (Einordnung)

| Aspekt des Konzepts | Etablierter Ansatz | Kernunterschied |
|---|---|---|
| Mehrere Repräsentationen, Kontrast statt Gewichtung | Principle of Polyrepresentation (Ingwersen) | dort: Überlappung als Relevanzindikator; hier: nur als Kontrast |
| Unsortierte Mengen mit Herkunft | Metasearch/Federated Search | dort meist Fusion zu Ranking (CombSUM, RRF) |
| Rohmerkmale statt Scores | Vespa, Solr/Elasticsearch Feature-Logging, PyTerrier | Scoring dort Teil des Systems, austauschbar statt zwingend clientseitig |
| Exploration statt Anfrage-Antwort | Exploratory Search (Marchionini), Berrypicking (Bates), ASK (Belkin), Information Foraging (Pirolli/Card), Scatter/Gather | liefert meist Cluster/Facetten, keine Verfahrensvielfalt |
| Sample als Anfrage | Query by Example, Relevance Feedback (Rocchio), AIDE | Feedback wird dort meist in Ranking überführt |
| Volumen/Verteilung, Sampling | Approximate Query Processing (BlinkDB), Online Aggregation (Hellerstein), Technology-Assisted Review | deckt Budget/Rückmeldung ab, nicht Multi-Projektions-Abdeckung |
| Diversitätsauswahl über Verfahren | Result Diversification (MMR, DPP), Quality-Diversity/MAP-Elites, Novelty Search | dort meist auf Ergebnisebene, hier auf Verfahrensebene |
| Gecachte Läufe als Korpus | TREC Session Track, Scroll/PIT (Elasticsearch), Notebook-Provenance | als durchsuchbares Objekt wenig verbreitet |

**Einzelverfahren:**
- Trigramm-Signaturen: Signature Files (Faloutsos), BitFunnel (Bing, frequenzbewusste Bitvergabe), Google Code Search / Zoekt, Winnowing/MOSS, MinHash/SimHash.
- AST-Suche: Deckard, SourcererCC, NiCad, Semgrep/ast-grep, Comby.
- Multivektor: ColBERT, ColBERTv2/PLAID.
- Strukturlos-strukturell: Normalized Compression Distance (Cilibrasi/Vitányi).

**Wo das Konzept über Etabliertes hinausgeht:**
- Verfahrensmenge selbst als vermessener, abzudeckender Raum (Ergebnismengen-Abstand als Metrik).
- Verzicht auf jede systemseitige Ordnung ist erkenntnistheoretisch begründet (Unmöglichkeit der Antizipation unbekannter Dimensionen), nicht nur pragmatisch.
- Erkenntnisgröße ist der Kontrast zwischen Projektionen, nicht die Güte einer einzelnen Projektion.
- Konvergenzkriterium (monotone, bestätigte Verfeinerung) statt Effizienzkriterium (Probabilistic Ranking Principle) als Rechtfertigung für den Verzicht auf Ranking.

Result Stats: id=df5e1695-3b13-498e-94a1-77fa16088f12, total: 19634, in: 7593, out: 12041, reason: 4241, read: 0, write: 0
SystemInit:  id=df5e1695-3b13-498e-94a1-77fa16088f12, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 4241
