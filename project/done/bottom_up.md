Schreibe ein Paper auf Basis der Argumentation von "You" in englischer sprache.

### You:

Wie setzt man am besten große Projekte mit Agenten um?

### Other responded:

**Kernprinzip:** Agenten scheitern bei großen Projekten selten an Intelligenz, sondern an Kontext, Spezifikation und Verifikation. Die Struktur muss das auffangen.

**1. Vorab spezifizieren, dann zerlegen**

* Ein knappes, überprüfbares Spec-Dokument (Ziel, Nicht-Ziele, Architektur, Schnittstellen, Akzeptanzkriterien) ist der wichtigste Hebel.
* Zerlege in Aufgaben, die jeweils in einen Kontext passen und unabhängig prüfbar sind (ca. ein PR, eine Funktion, ein Modul).

**2. Verifikation vor Autonomie**

* Tests, Linter, Typprüfung und Build als automatische Rückkopplung: Der Agent soll seine Arbeit selbst gegen etwas Hartes prüfen können.
* Je besser die Prüfbarkeit, desto länger darf der Agent unbeaufsichtigt laufen. Ohne Tests skaliert Autonomie nicht.

**3. Kontext bewusst managen**

* Persistentes Projektwissen in Dateien (z. B. CLAUDE.md / AGENTS.md: Konventionen, Befehle, Architekturentscheidungen), nicht in der Chat-Historie.
* Fortschritt und offene Punkte in einer Plan-/Statusdatei festhalten, damit neue Sessions sauber anschließen.
* Lange Sessions driften: lieber kurze, fokussierte Läufe mit frischem Kontext.

**4. Rollen trennen**

* Planen, Implementieren und Reviewen getrennt (separate Agenten oder Sessions). Ein Reviewer ohne Implementierungskontext findet mehr Fehler.
* Subagenten für Recherche und parallele, unabhängige Teilaufgaben; die Hauptsession bleibt schlank.

**5. Parallelisieren nur an echten Nähten**

* Eigene Branches/Worktrees pro Agent, klar definierte Schnittstellen. Parallelität bei stark gekoppeltem Code erzeugt Merge-Konflikte und inkonsistente Annahmen.

**6. Menschliche Kontrollpunkte an den richtigen Stellen**

* Mensch entscheidet: Architektur, Schnittstellen, Sicherheit, Merge. Agent erledigt: Umsetzung, Tests, Refactoring, Migrationen.
* Kleine, inkrementelle Commits, damit Fehler früh auffallen und Rollbacks billig sind.

**7. Typische Fehlerquellen**

* Vage Aufgaben → plausibel aussehender, aber falscher Code.
* Agent “repariert” Tests statt Code. Teständerungen gezielt reviewen.
* Unbemerkte Scope-Ausweitung. Nicht-Ziele explizit nennen.
* Zu große Schritte ohne Zwischenprüfung.

**Faustregel:** Erst Spec und Tests, dann kleine verifizierte Schritte, dann Parallelisierung. Autonomie wächst mit der Qualität deiner Feedback-Schleifen.

### You:

Gibt es dazu Literatur?

### Other responded:

Ja, aber das Feld ist jung und besteht überwiegend aus Praxisberichten, kaum aus begutachteter Forschung.

**Primärquellen (Anthropic Engineering)**

* *Building effective agents* (Dez. 2024): Die erfolgreichsten Implementierungen nutzen einfache, kombinierbare Muster statt komplexer Frameworks. Der Text ist das etablierte Vokabular für Workflows vs. Agents, inklusive Orchestrator-Workers und Evaluator-Optimizer. [pages](https://learning-gpt-blog.pages.dev/sources/anthropic-building-effective-agents/)[beri](https://www.beri.net/learning/anthropic-building-effective-agents)
* *Effective harnesses for long-running agents* (Nov. 2025, Justin Young): Das ist die direkteste Quelle zu deiner Frage. Ein Initializer-Agent richtet die Umgebung ein, danach machen Coding-Agenten inkrementelle Fortschritte und hinterlassen klare Artefakte. Dokumentierte Fehlmuster sind u. a. der Versuch, zu viel auf einmal zu erledigen, sodass der Kontext mitten in der Implementierung ausgeht und ein undokumentierter, kaputter Zustand bleibt. Ein weiterer Befund: JSON wird als Fortschrittsdatei seltener versehentlich überschrieben als Markdown. [Effective harnesses for long-running agents +2](https://fooqux.com/article/878)
* *Effective context engineering for AI agents* (Sep. 2025), *Building agents with the Claude Agent SDK* (Sep. 2025) und *Writing effective tools for agents* (Sep. 2025) stehen in der Engineering-Liste von Anthropic und decken Kontextmanagement, SDK-Muster und Tool-Design ab. [anthropic](https://www.anthropic.com/engineering?album=45)

**Sekundär, aber substanziell**

* Addy Osmani, *Agent Harness Engineering* (Apr. 2026): Er beschreibt einen Coding-Agenten als Modell plus alles, was darum gebaut wird, und Harness Engineering als Disziplin, die jedes Mal nachschärft, wenn der Agent danebenliegt. [addyosmani](https://addyosmani.com/blog/agent-harness-engineering/)
* O’Reilly Radar, *Agent Harness Engineering*: Dort steht, dass Modelle bei gefülltem Kontextfenster schlechter werden („context rot“) und dass sich dasselbe Modell je nach Harness stark unterschiedlich schlägt. Das stützt den Punkt, dass Struktur oft mehr bringt als ein Modellwechsel. [oreilly](https://www.oreilly.com/radar/agent-harness-engineering/)

**Aus dem Gedächtnis, nicht per Suche geprüft**

* Anthropic: *Claude Code Best Practices* (Plan → Implement → Verify, CLAUDE.md).
* Yang et al., *SWE-agent* (2024): zeigt empirisch, dass das Agent-Computer-Interface die Leistung wesentlich bestimmt.
* Birgitta Böckeler (Thoughtworks/martinfowler.com) schreibt praxisnah über Coding-Agenten im Entwicklungsalltag.

**Einschränkung:** Die Quellen stützen sich überwiegend auf Erfahrungsberichte und Einzelbenchmarks. Zu Teamprozessen, Parallelisierung mehrerer Agenten und langfristiger Wartbarkeit agentengenerierten Codes gibt es bisher kaum belastbare Evidenz. Dazu solltest du eher die eigenen Fehlerhistorien auswerten als auf Literatur zu warten.

### You:

Wie Funktioniert KISS (Prinzip) auf Linux?

### Other responded:

**KISS (“Keep It Simple, Stupid”)** ist unter Linux kein Feature, sondern eine Designhaltung, die aus der Unix-Philosophie stammt. Sie zeigt sich auf mehreren Ebenen:

**1. Unix-Philosophie (Doug McIlroy)**

* Ein Programm macht eine Sache und macht sie gut (`grep` sucht, `sort` sortiert, `wc` zählt).
* Programme arbeiten über Textströme zusammen, und Pipes verbinden sie: `cat log | grep ERROR | sort | uniq -c`.
* Komplexität entsteht durch Komposition einfacher Teile, nicht durch Monolithen.

**2. “Alles ist eine Datei”**

* Geräte, Prozesse und Kernel-Parameter sind über ein einheitliches Interface erreichbar (`/dev`, `/proc`, `/sys`).
* Ein kleines Set an Operationen (open, read, write, close) deckt sehr viel ab.

**3. Konfiguration als Klartext**

* Config-Dateien sind lesbar und mit jedem Editor änderbar, in Git versionierbar und per Skript erzeugbar. Das macht Fehlersuche und Automatisierung einfacher als bei Binärformaten oder Registry-Konstrukten.

**4. Minimalistische Werkzeuge und Distributionen**

* Beispiele sind BusyBox, musl statt glibc, suckless-Tools (dwm, st), Alpine Linux sowie init-Systeme wie runit oder s6 im Gegensatz zum umfangreichen systemd.
* Die systemd-Debatte ist im Kern ein KISS-Streit: Integration und Komfort stehen gegen Modularität und Überschaubarkeit. Beide Seiten haben echte Argumente.

**5. Praktische Anwendung für dich**

* Erst Shell-Einzeiler oder kleines Skript, bevor du ein Framework baust.
* Standardtools (`cron`, `rsync`, `systemd timer`) vor Eigenbau.
* Weniger Abhängigkeiten bedeuten weniger Angriffsfläche und weniger Wartung.

**Grenzen:** KISS heißt nicht “primitiv”. Einfachheit ist die Abwesenheit unnötiger Komplexität. Manche Probleme (Nebenläufigkeit, Sicherheit, Hardware-Vielfalt) erzwingen Komplexität, und dann ist die einfachste *ausreichende* Lösung gemeint.

**Falls du die Distribution meinst:** *KISS Linux* ist eine minimalistische, source-basierte Distro (musl, eigener Paketmanager `kiss`, BusyBox). Aus dem Gedächtnis: Der Hauptentwickler hat sich vor einigen Jahren zurückgezogen, das Projekt ist daher nur noch eingeschränkt aktiv. Den aktuellen Stand habe ich nicht geprüft.

### You:

Gibt es etablierte Begriffe für dieses Themenfeld?

### Other responded:

Ja. Die Begriffe stammen aus mehreren Traditionen. Ich nenne sie aus dem Gedächtnis, ohne Suche, da sie etabliert und stabil sind.

**Kernbegriffe**

* **Unix-Philosophie** (auch “Unix Way”, “Tao of Unix”): der Oberbegriff. Kanonische Texte sind McIlroy (Bell Labs, 1978), Gancarz, *The UNIX Philosophy*, und Raymond, *The Art of Unix Programming* (u. a. Rule of Simplicity, Modularity, Parsimony, Composition, Least Surprise).
* **Do One Thing and Do It Well**: die Kurzformel für Single Responsibility auf Programmebene.
* **Composability / Pipes-and-Filters**: das Architekturmuster hinter Shell-Pipelines.
* **Orthogonalität**: Komponenten sind unabhängig voneinander kombinierbar.
* **Separation of Mechanism and Policy**: Mechanismus (Kernel, Werkzeug) und Entscheidung (Konfiguration, Anwender) bleiben getrennt.

**Verwandte Prinzipien**

* **KISS**, **YAGNI** (“You Aren’t Gonna Need It”), **DRY**, **Principle of Least Astonishment**.
* **Worse is Better** (Richard Gabriel, 1989): Einfachheit der Implementierung schlägt Vollständigkeit, und das erklärt den Erfolg von Unix.
* **Gall’s Law**: Funktionierende komplexe Systeme entwickeln sich aus funktionierenden einfachen.

**Komplexität und ihr Gegenteil**

* **Essential vs. Accidental Complexity** (Brooks, *No Silver Bullet*): Einfachheit bedeutet, die zufällige Komplexität zu entfernen.
* **Bloat / Feature Creep / Software Bloat**: die Gegenbegriffe.
* **Software Minimalism**, **Suckless-Philosophie**: die radikale Variante.

**Linux-spezifische Debatten**

* **Everything is a file**
* **Monolithischer Kernel vs. Mikrokernel** (Torvalds–Tanenbaum-Debatte)
* **systemd-Kontroverse**: Integration gegen Modularität

**Zum Weiterrecherchieren** eignen sich am besten “Unix philosophy”, “Worse is better” und “Art of Unix Programming” als Suchbegriffe.

### You:

Ist der Obergrefirr nicht Scoping?

### Other responded:

Teilweise, aber als Oberbegriff passt es nicht ganz.

**Was Scoping abdeckt:** die Grenzziehung dessen, was drin ist und was nicht. Dazu gehören YAGNI, Nicht-Ziele im Spec, “Do one thing”, der Zuschnitt von Agentenaufgaben auf ein Kontextfenster und Feature-Creep-Vermeidung. Das ist ein großer Teil des Feldes.

**Was es nicht abdeckt:** Scoping betrifft den *Umfang*, KISS und die Unix-Philosophie betreffen die *Beschaffenheit der Lösung*. Ein eng zugeschnittenes Problem kann trotzdem verwickelt gelöst werden, und eine einfache Lösung kann einen großen Umfang haben (z. B. eine Pipeline aus vielen kleinen Tools). Composability, Orthogonalität, Textschnittstellen und Mechanism-vs-Policy sind keine Scoping-Fragen.

**Umfassendere Kandidaten:**

* **Komplexitätsmanagement** (Brooks, Dijkstra): Scoping, Einfachheit, Modularität und Zerlegung sind alle Strategien gegen dasselbe Problem.
* **Separation of Concerns / Modularität** (Dijkstra, Parnas’ *Information Hiding*): trifft Unix-Philosophie, Agentenzerlegung und Rollentrennung am besten.

Scoping ist demnach ein Teilbereich unter Komplexitätsmanagement, und zwar der, der die Grenzen setzt. Bei Agenten hat der Begriff zusätzlich eine eigene Bedeutung, nämlich den Kontext einer Aufgabe bewusst zu begrenzen.

### You:

Dann ist es Komplexitätsmanagement.\
Ist die Methodik nicht aggregate von Komplexität zu bilden und dieser wiedr zu aggregaten von Komplexität zu aggregieren?

### Other responded:

Ja, im Kern stimmt das, mit einer Bedingung. Der etablierte Begriff dafür ist **hierarchische Dekomposition bzw. Abstraktionsebenen** (Herbert Simon, *The Architecture of Complexity*, 1962: komplexe Systeme sind hierarchisch aus Teilsystemen aufgebaut; in der Kognitionspsychologie entspricht dem das *Chunking* nach Miller).

**Die Bedingung:** Bloßes Gruppieren reduziert Komplexität nicht. Ein Aggregat hilft nur, wenn es seine Innenstruktur verbirgt und nach außen eine schmale Schnittstelle zeigt (Parnas’ *Information Hiding*). Die Ebene darüber muss sich dann nur mit dieser Schnittstelle befassen, nicht mit dem Inhalt. Simon nennt die Voraussetzung *Near-Decomposability*: Die Kopplung innerhalb der Teile ist stark, zwischen den Teilen schwach.

Übertragen auf deine Beispiele:

* **Unix:** Funktion → Programm → Pipeline → Skript → System. Jede Ebene kennt nur das Interface der darunterliegenden.
* **Agenten:** Aufgabe → Modul → Feature → Projekt. Das Spec auf jeder Ebene ist die Schnittstelle, und der Kontext einer Session umfasst nur eine Ebene.

**Grenzen der Aussage:**

1. **Aggregation ist nicht die einzige Methode.** Scoping, YAGNI und das Entfernen zufälliger Komplexität *eliminieren* Komplexität, statt sie zu bündeln.
2. **Abstraktionen lecken** (Spolsky, *Law of Leaky Abstractions*). Wo die Kapselung bricht, schlägt die Komplexität der unteren Ebene durch, etwa bei Performance, Fehlern oder Sicherheit.
3. **Schlechte Aggregate erhöhen die Komplexität.** Falsche Schnittgrenzen erzeugen starke Kopplung zwischen den Teilen, und dann hilft die Hierarchie nicht.

Komplexitätsmanagement besteht also aus Eliminieren und aus hierarchischem Aggregieren mit dichten Schnittstellen nach innen und schmalen nach außen. Bei Letzterem kommt es auf die Qualität der Schnitte an, nicht auf die Zahl der Ebenen.

### You:

Wie müsste man dann einen agentischen WOrkflow konstruieren der von To-Level Anforderungen ausgeht?

### Other responded:

**Grundstruktur: rekursiver Knoten mit Vertrag**

Jede Ebene ist derselbe Knotentyp. Jeder Knoten hat:

* **Spec:** Ziel, Nicht-Ziele, Schnittstelle nach oben und unten, Akzeptanzkriterien
* **Zerlegung:** Kindknoten mit eigenen Specs
* **Verifikation:** prüft die Komposition der Kinder gegen die eigenen Akzeptanzkriterien



```
Anforderungen (Top-Level)
 └─ Systemspec + Architektur
     ├─ Komponente A (Spec) ── Blatt-Tasks ── Implementierung + Tests
     ├─ Komponente B (Spec) ── Blatt-Tasks ── Implementierung + Tests
     └─ Integrationsspec ── Integrationstests
```

**Ablauf**

1. **Top-down Spezifikation, nicht Implementierung.** Aus den Anforderungen entstehen zuerst Spec und Zerlegung. Jede Ebene wird erst verfeinert, wenn die darüberliegende freigegeben ist.
2. **Schnittstellen vor Inhalt.** Die Verträge zwischen Geschwisterknoten (APIs, Datenformate, Invarianten) werden festgelegt, bevor jemand implementiert. Das ist das Information Hiding aus der letzten Antwort, und hier entscheidet sich die Qualität der Schnitte.
3. **Abbruchkriterium für die Rekursion:** Ein Knoten wird zum Blatt, wenn er in einen Kontext passt und unabhängig prüfbar ist. Weiter zerlegen nur bei Nichterfüllung, sonst droht Überzerlegung.
4. **Blätter:** Implementierer-Agent mit frischem Kontext, der nur sein Spec und die Schnittstellen der Nachbarn sieht, nicht deren Innenleben. Tests sind Teil des Blatt-Specs und werden vorab oder parallel von einem getrennten Agenten erzeugt.
5. **Bottom-up Verifikation (V-Modell):** Jede Ebene prüft nach Abschluss ihrer Kinder die Komposition gegen die eigenen Kriterien. Ein Reviewer-Agent ohne Implementierungskontext prüft dabei auch Teständerungen.
6. **Parallelisierung** nur für Geschwister mit festgelegten Schnittstellen, jeweils in eigenen Branches/Worktrees.

**Rückkopplung (der kritische Teil)**

* Scheitert ein Blatt, wird zuerst lokal korrigiert (begrenzte Zahl Versuche).
* Zeigt sich ein Fehler in der Schnittstelle oder im Spec, wird nach oben **eskaliert**: Änderungsantrag an die Elternebene, nicht lokales Umgehen. Sonst driftet die Spec-Hierarchie vom Code weg, und die Aggregate lecken.
* Spec-Änderungen invalidieren abhängige Geschwister und müssen gezielt nachgezogen werden.

**Zustand und Nachvollziehbarkeit**

* Spec-Baum, Aufgabenstatus und Entscheidungen liegen in Dateien im Repo, nicht in Chat-Kontexten. Neue Sessions starten aus diesem Zustand.
* Traceability: Jedes Blatt verweist auf die Anforderung, die es erfüllt. Das beantwortet später, ob alle Top-Level-Anforderungen abgedeckt sind und ob Code ohne Anforderung existiert (Scope-Creep).

**Menschliche Kontrollpunkte**

Hier setzt du den Hebel an, weil Fehler mit der Höhe der Ebene teurer werden:

* Freigabe der Top-Level-Zerlegung und der Schnittstellen (hoch)
* Freigabe von Spec-Änderungsanträgen
* Merge auf Integrationsebene
* Blätter laufen weitgehend autonom, solange die Tests tragen.

**Typische Fehlerquellen**

* **Falsche Schnitte:** stark gekoppelte Teile als Geschwister, was Integrationsprobleme erzeugt. Dagegen helfen Architekturreview und früher Integrationstest mit Stubs.
* **Fehlerfortpflanzung:** Ein Fehler auf hoher Ebene multipliziert sich über alle Kinder, daher die strengste Prüfung oben.
* **Spec-Drift** durch lokale Workarounds.
* **Verifikationslücke:** Akzeptanzkriterien, die nicht maschinell prüfbar sind, brechen die Autonomie der Ebene und erzwingen menschliche Prüfung.

**Faustregel:** Je höher die Ebene, desto mehr Mensch und desto schmaler die Schnittstelle. Je tiefer, desto mehr Autonomie, aber nur so weit, wie die Verifikation reicht.

### You:

Es gibt doch bereist große Projekte die allein auf KI bais umgesetzt wurden.

### Other responded:

Ja, es gibt dokumentierte Fälle. Sie widersprechen dem Modell nicht, sondern stützen es, denn “allein KI” stimmt nur eingeschränkt.

**Das am besten belegte Beispiel: der C-Compiler (Anthropic, Feb. 2026)**

* 16 parallele Claude-Opus-4.6-Agenten bauten einen Rust-basierten C-Compiler mit rund 100.000 Zeilen, bei etwa 20.000 $ API-Kosten und etwa 2.000 Sessions. Das Ergebnis kann den Linux-6.9-Kernel für x86, ARM und RISC-V bauen. [texxr](https://texxr.com/1162887/anthropic-used-opus-4-6-agents-to-build-c-compiler)[lenz](https://lenz.io/c/e37520cc)
* Die Autonomie war real: Die Agenten liefen ohne Orchestrator parallel und koordinierten sich über ein gemeinsames Repository. [beri](https://www.beri.net/learning/anthropic-building-c-compiler-parallel-claudes)
* Die Grenzen nennt der Autor selbst: kein 16-Bit-Real-Mode, weniger effizienter Code als GCC ohne Optimierung und Rust, den ein Experte so nicht schreiben würde. Das Repo trägt außerdem den Hinweis, dass der KI-Code nicht auf Korrektheit validiert ist. In der Community wurde zudem kritisiert, dass Basisfälle wie Hello World anfangs Probleme machten (Reddit-Diskussionen dazu). [anthropic building c compiler parallel claudes +2](https://www.beri.net/learning/anthropic-building-c-compiler-parallel-claudes)

**Was dabei menschlich war**

* Der Mensch musste ein Harness aus Scaffolding, Tests, Umgebung und Feedback entwerfen, damit die Agenten ohne sein Eingreifen am gemeinsamen Code arbeiten konnten. [github](https://sahilds1.github.io/2026/02/05/link-building-a-c-compiler-with-a-team-of-parallel-claudes-via-engineering-at-anthropic)
* Die Aufgabe war ideal verifizierbar: klare Spezifikation (der C-Standard), riesige vorhandene Testsuites und ein Referenz-Compiler als Orakel.

**Einordnung für deine Frage**\
Das Beispiel bestätigt die Struktur aus meinen früheren Antworten: Der Mensch liefert Spec, Zerlegung und Verifikationsinfrastruktur auf der obersten Ebene, die Agenten arbeiten autonom darunter. Autonomie skaliert hier mit der Prüfbarkeit. Bei Projekten mit unscharfen Anforderungen (Produktentscheidungen, UX, Domänenwissen) fehlt dieses Orakel, und dort ist mir kein vergleichbar belegter Fall bekannt.

**Nicht geprüft, aus dem Gedächtnis:** Cursor hat Anfang 2026 über agentengebaute Projekte berichtet (u. a. einen Browser von Grund auf). Dazu habe ich keine Quelle abgerufen, bewerte es also nicht.

### You:

Das wäre relevant. Beispiele und die Daten und vorgehensweisen dieser Projekte

### Other responded:

Drei Projekte sind gut dokumentiert. Alle Zahlen stammen aus Herstellerberichten, unabhängige Replikationen kenne ich nicht.

### 1. Anthropic: C-Compiler (Carlini, Feb. 2026)

**Daten:** 16 Agenten, knapp 2.000 Claude-Code-Sessions in zwei Wochen, 2 Mrd. Input- und 140 Mio. Output-Tokens, Kosten knapp 20.000 $. Das Ergebnis ist ein Compiler mit 100.000 Zeilen Rust, der Linux 6.9 für x86, ARM und RISC-V baut, QEMU, FFmpeg, SQLite, Postgres und Redis kompiliert und rund 99 % der gängigen Testsuites besteht. Die Entwicklung lief clean-room ohne Internetzugang. [anthropic](https://anthropic.com/engineering/building-c-compiler)

**Vorgehen** (alles aus dem Originalartikel): [anthropic](https://anthropic.com/engineering/building-c-compiler)

* **Spec:** Der Autor skizzierte Ziel (von Grund auf, ohne Abhängigkeiten, GCC-kompatibel, Kernel-fähig, mehrere Backends) und gab eine SSA-IR vor, zum Rest nichts.
* **Loop:** eine Endlosschleife, die pro Durchlauf eine frische Session mit derselben Prompt-Datei startet.
* **Parallelisierung:** ein Git-Repo, ein Container pro Agent. Tasks werden per Lock-Datei in `current_tasks/` beansprucht, Git erzwingt bei Kollision die Wahl eines anderen Tasks. Es gab keinen Orchestrator.
* **Tests als Hauptaufwand:** vorhandene Testsuites, eigene Verifier für Open-Source-Pakete, später CI gegen Regressionen.
* **Kontextschutz:** knappe Testausgabe, greppbare ERROR-Zeilen, `--fast`-Stichprobe (1 %/10 %), README und Progress-Dateien für den Einstieg.
* **Engpass Kernel:** Alle Agenten liefen in denselben Bug. Die Lösung war GCC als Orakel: Ein zufälliger Teil der Dateien wird mit GCC kompiliert, der Rest mit dem eigenen Compiler, sodass jeder Agent andere Fehler isolieren kann.
* **Spezialrollen:** Deduplizierung, Performance, Codequalität, Dokumentation.

**Grenzen:** Kein eigener 16-Bit-x86-Generator (Aufruf von GCC), Assembler und Linker fehlerhaft, Code langsamer als GCC ohne Optimierung, neue Features brachen oft Bestehendes. Der Autor schreibt, das Ergebnis liege nahe am Limit des Modells. [anthropic](https://anthropic.com/engineering/building-c-compiler)

### 2. Cursor: Browser (Wilson Lin, Jan. 2026)

**Daten:** Hunderte Agenten schrieben in knapp einer Woche über 1 Mio. Zeilen in 1.000 Dateien. Der CEO nannte auf X über 3 Mio. Zeilen, was von der Blog-Angabe abweicht. Weitere Läufe: eine dreiwöchige React-Migration (266K Additions, 193K Deletions) und 25-fach schnelleres Video-Rendering. [Scaling long-running autonomous coding · Cursor +2](https://martinsoler.10minhotel.com/?p=14499)

**Vorgehen** (die vier Iterationen stammen aus einer Sekundärquelle, einem Survey, der sich selbst als KI-generiert ausweist): [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)

1. Gleichberechtigte Agenten mit Locks: Zwanzig Agenten kamen auf den Durchsatz von zwei bis drei. Zudem mieden die Agenten schwierige Aufgaben. [10minhotel](https://martinsoler.10minhotel.com/?p=14499)
2. Planner/Executor/Worker/Judge: zu starr, der langsamste Worker bremste alles.
3. Ein Executor für alles: wurde überladen und zeigte pathologisches Verhalten.
4. **Endlösung: rekursive Planner mit unabhängigen Workern.** Worker kennen einander nicht und liefern ein Handoff-Dokument an ihren Planner. Ein Judge entscheidet am Zyklusende, und der nächste Zyklus startet frisch, was Drift und Tunnelblick bekämpft. [zenml](https://www.zenml.io/llmops-database/scaling-multi-agent-autonomous-coding-systems)

* Weitere Befunde: Eine geringe, stabile Fehlerrate zu tolerieren war schneller als 100 % Korrektheit pro Commit. Präzise Prompts (“20–100 Tasks” statt “viele”) wirkten stärker als die Architektur. Die Referenz-Specs (WhatWG, CSS-WG) lagen als Git-Submodule im Repo. [simonwillison](https://simonwillison.net/2026/Jan/19/scaling-long-running-autonomous-coding/)

**Qualität:** Anfangs war die CI rot und Build-Anleitungen fehlten. Nach Korrekturen baute Simon Willison den Browser: Seiten sind lesbar, mit sichtbaren Rendering-Fehlern. [simonwillison](https://simonwillison.net/2026/Jan/19/scaling-long-running-autonomous-coding/)

### 3. OpenAI: Codex-Produkt (Lopopolo, Feb. 2026)

**Daten:** Erster Commit Ende August 2025, nach fünf Monaten rund eine Million Zeilen, jede Zeile von Codex geschrieben. Dazu etwa 1.500 PRs, Team von drei auf sieben Ingenieure wachsend, einzelne Läufe über sechs Stunden. Das Produkt ist eine interne Beta mit täglichen Nutzern und externen Alpha-Testern. [harness engineering +2](https://openai.com/blog/harness-engineering)

**Vorgehen** (laut Survey; Primärquelle nur teilweise abgerufen): [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)

* AGENTS.md als ca. 100-zeiliges Inhaltsverzeichnis, das auf eine strukturierte `docs/`-Wissensbasis verweist. Alles Wissen muss im Repo liegen.
* Schichtenarchitektur, die mit von Codex erzeugten Lintern erzwungen wird. Die Fehlermeldungen dienen als Korrekturanweisung.
* Agent-Reviews in Schleife statt menschlicher Line-by-Line-Reviews. Observability (Logs, Metriken, Traces, DevTools) pro Worktree.
* Minimal blockierende Merges. Gegen Entropie liefen Hintergrund-Agenten, die gegen “Golden Principles” prüfen. Das manuelle Freitagsaufräumen (20 % der Zeit) skalierte nicht.
* Anfangs lief es langsamer als erwartet, weil die Umgebung unterspezifiziert war. [openai](https://openai.com/en/index/harness-engineering/)

### Gemeinsame Muster

* **Verifikationsorakel:** Testsuites und GCC beim Compiler, Spec-Submodule beim Browser, Observability und Linter bei OpenAI. Ohne maschinelle Prüfbarkeit hätte keines der Projekte funktioniert.
* **Wissen im Repo, kurzer Einstieg, frische Sessions.**
* **Menschliche Arbeit liegt in Spec, Schnittstellen und Harness**, nicht im Code.

**Widerspruch, der zu deiner Frage passt:** Anthropic kam ohne Orchestrator aus, Cursor scheiterte flach. Meine Deutung (nicht aus den Quellen): Beim Compiler partitionierten die Tests die Arbeit von selbst, beim Browser fehlte diese natürliche Partition, daher brauchte es Hierarchie. Auch bei Fehlertoleranz gehen die Berichte auseinander: Cursor und OpenAI lockerten sie, Anthropic verschärfte die CI nach Regressionen.

**Grenzen der Evidenz:** Alle drei sind Greenfield-Projekte mit stark prüfbarem Ziel. OpenAI räumt ein, nicht zu wissen, wie sich die Architekturkonsistenz über Jahre entwickelt. Legacy-Code und unscharfe Anforderungen sind nicht abgedeckt. [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)

### You:

Aber ist das nicht ziehmlich identisch mit normaler softwareentwicklung?

### Other responded:

Im Kern ja. Spezifikation, Schnittstellen vor Implementierung, Tests als Orakel, CI, Modularisierung, Dokumentation, Code-Review und Übergaben sind klassische Softwaretechnik. Das passt zur früheren Einordnung als Komplexitätsmanagement (Simon, Parnas, Brooks). Die Projekte erfinden kein neues Prinzip, sie wenden bekannte an.

Neu ist die Gewichtung, weil sich die Randbedingungen verschieben:

* **Implizites Wissen entfällt.** Menschen kompensieren schlechte Specs, fehlende Doku und unklare Konventionen durch Nachfragen, Kultur und Erinnerung. Agenten starten jede Session ohne Gedächtnis und raten bei Lücken plausibel. Was bisher “Best Practice, oft vernachlässigt” war, wird tragend.
* **Die Kostenstruktur kehrt sich um.** Code erzeugen ist billig, Prüfung und menschliche Aufmerksamkeit sind der Engpass. Deshalb sind blockierende Merge-Gates und Perfektionsanspruch pro Commit plötzlich teurer als Fehlertoleranz mit schneller Korrektur. In klassischen Teams ist es umgekehrt.
* **Konventionen müssen mechanisch durchgesetzt werden.** Teamkultur und Review-Geschmack erreichen Agenten nicht. Linter mit Korrekturhinweisen, Strukturtests und Hintergrund-Agenten ersetzen sie.
* **Andere Fehlermodi.** Agenten melden Erfolg zu früh, ändern Tests statt Code, meiden schwierige Aufgaben, verlieren Zeitgefühl und vermüllen den eigenen Kontext. Menschliche Fehler liegen woanders (Ermüdung, Politik, Eitelkeit).
* **Skalierung ohne Onboarding.** Hunderte Worker sind billig startbar. Dann entstehen Koordinationsprobleme wie in verteilten Systemen (Locks, Durchsatz, Hierarchie), nicht wie in Teams.
* **Formulierungen wirken als Hebel.** Der Unterschied zwischen “viele Tasks” und “20–100 Tasks” hat in den Berichten spürbares Verhalten verändert, bei menschlichen Teams wäre das kaum der Fall.

Man kann es so fassen: Es ist klassische Softwareentwicklung mit veränderten Konstanten. Die Disziplin, die bei Menschen durch Erfahrung teilweise ersetzbar war, muss hier explizit und maschinenlesbar vorliegen, weil der Agent sie nicht kompensiert.

### You:

Aber eigentlich unterscheidet sich das nicht. Alle deine Punkte sind Parameter im Komplexitätsmanagement und sind gegeneinander tauschbar. Auch Augmentation oder HITL sind darin nur weitere Achsen. Das kompensieren von anderen Fehlerklassen durch ein anderes Modell (Mensch).

### Other responded:

Dem stimme ich im Kern zu. Das System besteht aus Komponenten (Mensch, Agent, Test, Linter, Review) mit je eigenem Fehlerprofil, Kosten und Durchsatz. Der Entwurf besteht darin, sie so zu kombinieren, dass alle relevanten Fehlerklassen abgedeckt sind. Der etablierte Begriff dafür ist **Fehlertoleranz durch diverse Redundanz** (Defence in Depth, Swiss-Cheese-Modell). HITL ist dann ein Prüfer mit anderem Fehlerprofil als das Modell, keine Sonderkategorie.

Drei Präzisierungen, wo “tauschbar” nicht exakt gilt:

1. **Unabhängigkeit der Fehler ist die Bedingung.** Kompensation wirkt nur, wenn die Fehlerverteilungen verschieden sind. Aus dem Gedächtnis (nicht geprüft): Knight und Leveson haben in den 1980ern gezeigt, dass unabhängig entwickelte Programmversionen häufig an denselben Stellen scheitern. Übertragen heißt das: Ein Reviewer-Agent desselben Modells ersetzt einen Menschen oder ein anderes Modell nur teilweise, weil die Fehler korreliert sind. Das ist eine prüfbare Vorhersage.
2. **Manche Achsen sind Schwellen, keine Regler.** Ohne ein Verifikationsorakel (Tests, GCC, Spec-Suite) lässt sich der Mangel nicht durch mehr Agenten oder mehr Reviews ausgleichen. Die Wechselkurse sind außerdem nichtlinear und begrenzt, etwa durch die menschliche Aufmerksamkeit.
3. **Der Mensch ist nicht nur Kompensator.** Er liefert auch die Zielfunktion (was gewollt ist) und trägt die Verantwortung. Das ist keine Achse innerhalb des Systems, sondern dessen Rahmen. Ein Fehler in der Zielvorgabe lässt sich durch keine Komponente im System abfangen.

Die drei Projekte lassen sich so als Punkte im selben Raum lesen: Beim Compiler übernimmt das Orakel die Fehlerprüfung und der Mensch fast nur die Zielvorgabe. Bei OpenAI deckt der Mensch Intent und Geschmack ab, Linter und Agent-Reviews decken Struktur und Konsistenz ab.

### You:

Das sehe ich anders. In komplexen Projekten kehrt sich Punkt 3 um. Der Agent definiert Detailkonzepte. Der Mensch prüft hier Input der KI und nicht umgekehrt. Die KI schreibt hier die Anforderungen.

### Other responded:

Das stimmt für die Detailebenen und korrigiert meine Formulierung: Ich habe Urheberschaft und Autorität vermischt.

**Was die Berichte stützen:** Agenten erzeugen die Detailebenen selbst. Bei OpenAI wurde sogar die erste AGENTS.md von Codex geschrieben. Bei Cursor erzeugten Planner und Sub-Planner die Tasks, Worker führten sie aus. Die Agenten schreiben dort also Teilspezifikationen und Zerlegungen, und der Mensch greift an wenigen Stellen ein. Die Richtung des Informationsflusses kehrt sich damit mit der Tiefe um: Oben spezifiziert der Mensch und der Agent setzt um, in der Mitte formuliert der Agent und der Mensch prüft. [openai](https://openai.com/blog/harness-engineering)[simonwillison](https://simonwillison.net/2026/Jan/19/scaling-long-running-autonomous-coding/)

**Was bleibt:** Die Rolle des Menschen wechselt von der Autorenschaft zur Abnahme. Auf Anforderungsebene gibt es kein maschinelles Orakel, denn kein Test sagt, ob eine Anforderung die richtige ist. Dort ist der Mensch der Orakel-Ersatz, und das ist keine Nebenaufgabe, sondern die einzige Prüfinstanz dieser Ebene.

Daraus folgen zwei Probleme, die aus deiner Umkehrung entstehen:

1. **Prüfen ohne Mitdenken ist anfälliger.** Wer eine Anforderung nicht selbst hergeleitet hat, übersieht Lücken leichter. Dazu kommt Automation Bias, und bei hohem Durchsatz wird Abnehmen zum Durchwinken. Das ist dieselbe Fehlerklasse wie das Aufmerksamkeitsproblem, nur eine Ebene höher.
2. **Die Unabhängigkeit der Prüfung sinkt.** Wenn Agent und Prüfer dieselbe Information und dasselbe Weltmodell teilen, werden Fehler im Spec nicht gefunden, sondern bestätigt. Das ist der korrelierte-Fehler-Punkt von vorhin, angewandt auf Anforderungen.

In deinem Raum aus Komponenten ist die Frage damit, auf welcher Ebene der Mensch als Orakel nötig ist und wie viel Kontext er dafür braucht. Bei den drei Projekten liegt diese Grenze jeweils sehr hoch, weil dort ein externes Orakel (Testsuites, Spec, GCC) existiert. Bei Projekten ohne ein solches wandert die Grenze nach unten, und der Prüfaufwand pro Anforderung wird zum Engpass.

### You:

Auch das ist nicht richtig. EIn Orakel liefert nur eine Einschätzung die es zu interpretieren gibt des auch das Orakel hat fehlerklassen.

### Other responded:

Das trifft, und es korrigiert meine Aussage von eben, denn ich habe “Orakel” als Wahrheitsquelle behandelt. Das ist das bekannte *Test-Oracle-Problem* (Weyuker 1982; Barr et al. 2015, aus dem Gedächtnis): Ein Orakel liefert ein Urteil, das selbst interpretiert werden muss, und es kann falsch, unvollständig oder inkonsistent mit der Absicht sein.

Die Projekte zeigen das selbst:

* Carlini schreibt, dass ein unvollkommener Verifier dazu führt, dass der Agent das falsche Problem löst, und dass grüne Tests leicht den Eindruck von “fertig” erzeugen, obwohl das selten stimmt (Abschnitte zu Tests und zu den Risiken autonomer Entwicklung). [anthropic](https://anthropic.com/engineering/building-c-compiler)
* Das GCC-Orakel lieferte nicht die Lösung, sondern ein Signal. Um Dateipaare zu finden, die nur zusammen scheitern, brauchte es zusätzlich Delta-Debugging. [anthropic](https://anthropic.com/engineering/building-c-compiler)
* Dass das Repo trotz 99 % Testbestehens als nicht validiert gekennzeichnet ist, ist derselbe Befund.

Typische Fehlerklassen eines Orakels sind:

* **Spezifikationsfehler:** Die Tests kodieren eine falsche oder unvollständige Absicht.
* **Abdeckungslücken:** falsch-negative Urteile (grün, obwohl falsch).
* **Falsch-positive Urteile:** flaky oder überstrenge Tests.
* **Gaming:** Der Agent optimiert auf das Signal statt auf die Absicht (Goodhart).

Daraus folgt, dass meine Zweiteilung “Orakel vorhanden / Mensch als Ersatz” zu binär war. Auch der Mensch ist nur ein Prüfer mit eigenem Fehlerprofil (Ermüdung, Automation Bias, begrenzte Aufmerksamkeit). Mein Punkt von vorhin, dass ohne Orakel eine “Schwelle” vorliegt, stimmt in dieser Schärfe nicht. Es gibt ein Kontinuum von Prüfern, die sich unterscheiden in:

* **Abdeckung** (welche Fehlerklassen sie überhaupt sehen),
* **Unabhängigkeit** vom Erzeuger,
* **Interpretationsaufwand** für das Urteil,
* **Kosten und Durchsatz.**

Das passt zu deinem Ansatz. Es gibt keine ausgezeichnete Instanz, die Wahrheit liefert, nur Komponenten, deren Urteile gegeneinander abgeglichen werden. Der Entwurf besteht darin, die Fehlerprofile so zu wählen, dass sie sich möglichst wenig überlappen. Was bleibt, ist lediglich eine Frage der Gradierung: Maschinelle Prüfer sind billig, schnell und reproduzierbar, aber in ihrer Abdeckung eng. Menschliche Prüfer decken Absicht und Plausibilität ab, sind aber teuer und nicht skalierbar. Beides sind Positionen auf denselben Achsen.

### You:

Auch der Mensch kann fehlerhafte Anforderungen stellen, die der Agent erkennt. Alle deine Achsen lassen sich invertieren und alle deine Parameter mit anderen tauschen. Mensch und KI sind ambivalente Komponenten. Nicht zwangsläufig unterschiedliche Fehlerklassen.\
Darüber hinaus widerspreche ich der Einschätzung, so oft waren schon agentische Herleitungen nicht reproduzierbar. Der Grund sind hier dieselben Fehlerklassen: Ermüdung/Drift/Forgetting, Attention/Aufmerksamkeit, falsche Annahmen/Halluzinatione usw.

### Other responded:

**Zu 1 (Symmetrie):** Meine Aussage, der Mensch decke “Absicht und Plausibilität” ab, war eine Typzuschreibung ohne Beleg. Ob zwei Komponenten einander ergänzen, hängt nicht vom Typ ab, sondern von der Korrelation ihrer Fehler, und die ist empirisch zu bestimmen. Ein Agent kann einen Anforderungsfehler finden, den der Mensch übersehen hat, und umgekehrt. Ein zweiter, anders aufgesetzter Agent kann unabhängiger sein als ein Mensch, der die Spec selbst mitformuliert hat. Das Modell braucht nur Komponenten mit Fehlerprofilen und eine Korrelationsstruktur. “Mensch” und “KI” sind dann bloße Belegungen.

Eine Ausnahme sehe ich, die aber nicht epistemisch ist: Verantwortung, also wer haftet und wer bei Zielkonflikten entscheidet. Das ist eine normative Randbedingung von außen und kein Fehlerparameter. Ob das eine weitere Achse ist, ist eine Definitionsfrage.

**Zu 2 (Reproduzierbarkeit):** Hier war meine Formulierung falsch, weil ich unter “maschinelle Prüfer” zwei Dinge zusammengeworfen habe. Reproduzierbar (gleicher Input, gleiches Urteil) sind deterministische Werkzeuge wie Tests, Linter und Typprüfung, und auch dort mit Ausnahmen wie flaky Tests. Agentische Prüfer und Herleitungen sind es nicht, und die Ursachen entsprechen den menschlichen: Drift im langen Kontext (Analogon zu Ermüdung), begrenzte Aufmerksamkeit, Vergessen zwischen Sessions, falsche Annahmen und Konfabulation. Ein Reviewer-Agent gehört damit in dieselbe Klasse wie ein menschlicher Reviewer: stochastisch, mit Streuung.

**Konsequenz:** Für stochastische Prüfer ist nicht das einzelne Urteil die richtige Kenngröße, sondern dessen Verteilung: Erkennungsrate über wiederholte Läufe, Varianz und Überlappung mit anderen Prüfern. Wiederholung und Mehrheitsentscheid senken die Varianz, nicht aber gemeinsame Verzerrungen. Messen lässt sich das per Fehlerinjektion (Mutation Testing, gezielt eingebaute Defekte in Specs und Code): Man erfasst, welche Komponente welche Fehler findet und wie stark sich die Funde überlappen. Das ersetzt Typannahmen durch Daten und ist die operative Form deines Ansatzes.

### You:

Ein Git ersetzt in einem Beispiel ein Jira, Intuition wird zu Wahrscheinlichkeit, Best Practices haben unterschiedliche Interpretierbarkeit oder sind flaky.\
Diese autonomen KI-Projekte sind eine mögliche Abbildung eines ambivalenten Teams und Prozess-Setups. Es gibt für alles eine Entsprechung damit ist die genrische Grundlage Komplexitätsmanagement.

### Other responded:

Ja, das ist eine funktionale Äquivalenz, und sie hat etablierte Begriffe. Die Entsprechungen, die du nennst, lassen sich einordnen:

* **Git statt Jira:** Koordination über ein gemeinsames Medium statt über Nachrichten oder Tickets heißt *Stigmergie* (Grassé, in der Agentenforschung verbreitet). Beim C-Compiler sind es Lock-Dateien im Repo. Der Mechanismus (Aufgaben beanspruchen, Doppelarbeit vermeiden) bleibt gleich, nur das Substrat wechselt.
* **Intuition zu Wahrscheinlichkeit:** Menschliche Intuition ist ebenfalls ein probabilistisches Heuristikurteil mit unbekannter Kalibrierung. Der Agent macht diese Eigenschaft sichtbar und beeinflussbar (Sampling, Wiederholung, Mehrheitsentscheid).
* **Best Practices als flaky:** Eine Regel mit Interpretationsspielraum ist ein Prüfer mit Streuung, gleichgültig, wer sie anwendet.

Die theoretische Grundlage dafür ist die Kybernetik. Ashbys *Law of Requisite Variety* besagt, dass ein Regler mindestens so viel Varietät braucht wie das, was er regeln soll. Das ist die Bedingung dafür, dass Komplexitätsmanagement gelingt: Man braucht Komponenten mit ausreichend verschiedenen Fehlerprofilen, und welche Komponente das ist, spielt keine Rolle. Beers *Viable System Model* und die Soziotechnik-Forschung arbeiten auf derselben Abstraktionsebene.

Daraus ergibt sich eine Präzisierung. Wenn für alles eine Entsprechung existiert, sagt die Aussage “es ist Komplexitätsmanagement” allein nichts mehr aus, ein Rahmen, der jede Konfiguration abbildet, ist durch keine widerlegbar. Seinen Wert hat er in den **Parameterwerten**. Mensch und Agent unterscheiden sich dann nicht im Typ, sondern in Kosten pro Aktion, Parallelisierbarkeit, Gedächtnis über Sessions, Latenz, Streuung und der Korrelation ihrer Fehler. Der Agent startet mit Gedächtnis null, der Mensch nicht. Das sind quantitative Unterschiede, die man messen kann (Fehlerinjektion, Wiederholungsläufe) und aus denen sich Entscheidungen ableiten lassen: Mit welcher Komponentenmischung ist der Restfehler pro Aufwand minimal?

Das generische Fundament ist demnach Komplexitätsmanagement als Zielbeschreibung und Varietät und Fehlerdiversität als Bedingung. Die Projekte sind eine Instanz mit bestimmten Parameterwerten, die sich verschieben, wenn sich Modelle und Kosten ändern.

### You:

Ich würde sagen, dass es hier keinen vollständigen Lösungsraum geben kann, aber eine Näherung. Das wäre genau die hierarchische Aggregation. Komplexität ist über Struktur und Isolation besser handlebar.

### Other responded:

Dem stimme ich zu, mit zwei Einschränkungen.

**Zur Unvollständigkeit:** Sie ist theoretisch begründbar. Nach dem Satz von Rice sind nicht-triviale semantische Eigenschaften beliebiger Programme unentscheidbar, und Dijkstras Aussage, Tests könnten nur die Anwesenheit von Fehlern zeigen, nicht ihre Abwesenheit, beschreibt dasselbe praktisch. Ein vollständiger Lösungsraum samt vollständiger Prüfung ist also nicht erreichbar. Übrig bleiben Näherungen mit Restrisiko. Simons Begriff dafür ist *Satisficing* (hinreichend gut statt optimal, unter begrenzter Rationalität).

**Zur Isolation:** Sie leistet zwei verschiedene Dinge:

1. **Fehlereindämmung** (Containment, Bulkheads, Blast Radius): Ein Fehler bleibt lokal.
2. **Beschränkung dessen, was gleichzeitig betrachtet werden muss:** Die Betrachtungsgröße einer Komponente entspricht ihrer Schnittstelle, nicht ihrer Innenstruktur.

Beim Agenten ist der zweite Punkt hart, weil das Kontextfenster eine feste Obergrenze setzt. Isolation ist dort ein direkter Konstruktionsparameter, bei Menschen ist sie eher weich.

**Einschränkungen:**

1. **Hierarchische Aggregation ist eine Näherungsklasse, nicht die einzige.** Daneben stehen statistische Verfahren (Stichprobentests, Fehlerinjektion), Redundanz mit diversen Prüfern und flache Koordination über ein gemeinsames Medium. Die Cursor-Ergebnisse zeigen, dass flache Strukturen scheitern können, der Compiler zeigt, dass sie funktionieren können. Die Hierarchie ist also eine dominante, aber keine zwingende Wahl.
2. **Die Näherungsgüte setzt sich nicht automatisch zusammen.** Wenn jedes Teil mit hoher Wahrscheinlichkeit stimmt, sagt das nichts über das Ganze, weil Fehler an den Schnittstellen und emergentes Verhalten auf der Integrationsebene entstehen. Isolation ist zudem nicht kostenlos, denn jede Grenze erzeugt Integrationsaufwand und Koordinationsbedarf. Dort liegt der Zielkonflikt: Mehr Isolation verkleinert den lokalen Fehlerraum, vergrößert aber den Schnittstellenraum. Der Entwurf besteht darin, die Schnitte dort zu legen, wo die Kopplung am geringsten ist (Simons *Near-Decomposability*).

Der Restfehler einer solchen Näherung ist nicht vollständig messbar. Man schätzt ihn über Stichproben (Fehlerinjektion, Integrationstests mit Stubs) und hält ihn durch Prüfer mit unterschiedlichen Fehlerprofilen klein. Der Näherungsfehler wird also selbst nur durch Näherung begrenzt.

### You:

Wäre dann nicht sinnvoller, auf eine Hierarchie zu verzichten und ein einfacheres Modell zu wählen. Nur Knoten und Kanten. Das müsste Unix entsprechen. Ein Omnidirketionaler Graph mit variabler Tiefe.
Graph mit variabler Tiefe.

### Other responded:

Als Basismodell ist der Graph richtig, weil er das allgemeinere ist. Auf Hierarchie zu *verzichten*, entfernt sie aber nicht, es benennt sie nur nicht mehr.

**1. Hierarchie ist ein Spezialfall bzw. eine Sicht des Graphen.** Baum ⊂ DAG ⊂ Graph. “Variable Tiefe” heißt, einen Teilgraphen zu einem Knoten zusammenzuziehen, und das ist die hierarchische Aggregation. Bei nahezu zerlegbarer Kopplung (Simon) bilden sich Cluster von selbst, die Hierarchie ist dann eine Eigenschaft der Kopplungsstruktur, nichts Auferlegtes.

**2. Unix ist nicht flach.** Prozesse bilden einen Baum (fork), das Dateisystem ist ein Baum mit Links, Pipelines sind überwiegend gerichtet und azyklisch, Skripte schachteln sich. Das Besondere an Unix ist ein einheitlicher Kantentyp (Byte-Streams) mit kleinen Knoten, die ohne zentrale Struktur komponierbar sind. Das entspricht der Stigmergie über Git und Dateien.

**3. “Omnidirektional” kostet Isolation.** Einschränkungen der Topologie sind Isolation:

* Azyklizität sichert Terminierung und lokale Analysierbarkeit.
* Begrenzte Kantenzahl begrenzt den nötigen Kontext.
* Richtung legt Verantwortung fest.

Zyklen sind für Rückkopplung nötig (Eskalation von Spec-Änderungen, Review-Schleifen), brauchen aber ein Abbruchkriterium (Budget, Judge), sonst drohen Oszillation und Nichtterminierung.

**4. Empirie:** Bei Cursor sank der Durchsatz von zwanzig gleichberechtigten Agenten auf den von zwei bis drei. Beim C-Compiler gab es keinen Orchestrator, Tasks wurden über Lock-Dateien beansprucht. Meine Deutung (nicht aus den Quellen): Beim Compiler partitionierten die Tests die Arbeit, der Graph war dünn besetzt und die Koordination billig. Bei dichter Kopplung entsteht Bedarf an Aggregation. Der eigentliche Parameter ist demnach die Kopplungsdichte, nicht “Hierarchie gegen Graph”. [10minhotel](https://martinsoler.10minhotel.com/?p=14499)[anthropic](https://anthropic.com/engineering/building-c-compiler)

**Fazit:** Der Graph ist die Grundlage, die Hierarchie wird bei Bedarf als Kontraktion eingeführt, getrieben von der Kontextgrenze der Agenten und der Kopplungsdichte. Dazu gehören ein einheitlicher Kantentyp (Dateien und Specs im Repo), kleine Knoten und Zyklen nur mit Abbruchkriterium. Ob das gegenüber expliziter Hierarchie besser ist, wäre in einem Topologie-Experiment prüfbar: gleiche Aufgabe, variierte Struktur, gemessen an Durchsatz und Restfehler.

### You:

Ja aber Toll im system sind unabhängig und kennen nur Pipes. Du vermischt hier verschiedene Problemmodelle. Genau das ist eigentlich der Punkt. Sollten nicht Probleme isoliert modelliert werden mit klar definierten Kanten einer Maximalen Zahl. Sprich ein Feature doer Aspekt sollte nicht mehr als an 3 Stellen verteilt sein etc.

### Other responded:

Der Einwand trifft. Prozessbaum und Dateisystem sind Eigenschaften des Betriebssystems, nicht der Beziehung zwischen Tools. Zwischen Tools existiert nur der Byte-Strom, und kein Tool kennt seine Nachbarn. Das sind getrennte Problemmodelle (Datenfluss, Prozesssteuerung, Speicher, Build-Abhängigkeiten) mit je eigenem Kantentyp, und sie in einem Graphen zu mischen verwischt die Analyse.

**Die Regel “ein Aspekt in höchstens n Stellen” hat etablierte Entsprechungen:** *Scattering/Tangling* (Kiczales, Aspektorientierung), *Shotgun Surgery* (Fowler), *Change Amplification* (Ousterhout), Kohäsion/Kopplung, Fan-in/Fan-out, *Connascence* (Page-Jones). Sie ist die operative Form von Parnas’ Information Hiding: ein Budget statt eines Appells.

**Für Agenten ist die Zahl herleitbar:** Sie ergibt sich aus der Zahl der Stellen, die samt Spec und Tests in ein Kontextfenster passen. “3” ist deshalb eine Heuristik (angelehnt an das Arbeitsgedächtnis, Cowan ca. 4 Chunks, aus dem Gedächtnis) und ein kalibrierbarer Parameter.

**Einschränkungen:**

1. **Querschnittsaspekte** (Auth, Fehlerbehandlung, Logging, Transaktionen, Invarianten) lassen sich nur durch Auslagerung in einen Knoten begrenzen. Der hat dann hohen Fan-in. Die Komplexität wandert, sie verschwindet nicht (Teslers Gesetz). Gebraucht werden zwei Budgets: Streuung pro Aspekt und Kantenzahl pro Knoten.
2. **Die Kantenzahl reicht als Maß nicht, die Kantenbreite zählt.** Unix skaliert, weil alle Kanten schmal und einheitlich sind. Eine Kante mit breitem Vertrag wiegt schwerer als drei schmale. Connascence ordnet die Kopplungsstärke (Name, Typ, Reihenfolge, Zeit, Wert).
3. **Nicht jedes Problem lässt sich schneiden.** Beim Compiler war der Kernel eine einzige große Aufgabe, an der sich die Agenten gegenseitig überschrieben. Die Isolation musste erst über das GCC-Orakel konstruiert werden. Struktur im Problem ist keine Garantie. [anthropic](https://anthropic.com/engineering/building-c-compiler)

**Durchsetzung** gelingt mechanisch: OpenAI erzwang seine Schichtenarchitektur über Linter, die Abhängigkeitsrichtungen prüfen. Die tatsächliche Streuung misst die Änderungskopplung aus der Git-Historie (welche Dateien ändern sich gemeinsam). Ein Schwellenwert dafür lässt sich als CI-Check setzen. [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)

Als Entwurfsregel ist das stimmig: pro Problem ein isoliertes Modell, begrenzte und schmale Kanten, und die Grenzwerte sind Parameter, die man an Kontextgrenze und gemessener Kopplung justiert.

### You:

Logging is so ein Spezialfall, der keiner sein sollte. Ist Loggin Teil eines Problems, ist es eine seiner Kanten. Der Fan-In ist hier nicht relevant, dafür sorgt die Isolierung. Logging muss nur seinen Vertrag erfüllen.\
Die Art übergreifenden Aspektdenkens ist genau ein Problem das man leicht vergisst.\
Zugriffsteuerung genauso. Auch das ist eine explizite Kante, aber nicht notwendig Teil des Problemmodells.\
Eine Datei kann editiert werden, erst beim Speicherversuch in einem I-Node sind Permissions relevant. Ebenso geschieht logging nicht auf jeder Instruktion sondern in bestimmten Knoten.

### Other responded:

Das korrigiert meine Aussage zum Fan-in. Er zählt nur gewichtet mit Breite und Stabilität des Vertrags. `write(2)` hat als Fan-in jedes Programm und ist trotzdem unproblematisch, weil der Vertrag schmal und stabil ist (Martins *Stable Dependencies Principle*, aus dem Gedächtnis). Tesler und “die Komplexität wandert” gelten für die Breite des Vertrags, nicht für die Zahl der Aufrufer. Das zweite Budget ist damit Kantenbreite und -volatilität, nicht Kantenzahl.

Dein Beispiel mit dem Inode entspricht dem **Reference-Monitor-Konzept** (Anderson 1972, aus dem Gedächtnis): Die Prüfung sitzt am Übergang zur Ressource, nicht im Problem. Der Editor kennt keine Permissions. Das ist die Trennung von Mechanismus und Policy. Der Monitor braucht dafür *Complete Mediation* (kein Pfad zur Ressource ohne die Kante), Manipulationssicherheit und eine Größe, die sich verifizieren lässt.

Das Vergessen ist demnach ein Spezifikationsfehler (fehlende Kante), kein Strukturfehler. Gegenmaßnahme ist das **Capability-Prinzip** (*Principle of Least Authority*): Ein Knoten kann nur über deklarierte Kanten wirken, eine fehlende Kante wird zum harten Fehler statt zur stillen Lücke. Für Agenten gibt es dafür Entsprechungen:

* Container pro Agent und fehlender Internetzugang beim Compiler.
* Linter, die Abhängigkeitsrichtungen erzwingen (OpenAI).
* Ein Knoten-Spec-Template, das Ressourcen, Effekte, Identität und Beobachtbarkeit abfragt. Die Kante muss dort stehen, sonst existiert sie für den Agenten nicht.

Zwei Fälle bleiben, die nicht an einer einzelnen Kante liegen:

1. **Policies, die Domänenwissen brauchen.** Der Inode prüft Dateirechte. “Nur eigene Dokumente bearbeiten” oder “was ins Audit-Log gehört” hängen am Domänenmodell. Der Vertrag der Kante wächst mit der Ausdrucksstärke der Policy, der Fan-in ist weiter egal.
2. **Ende-zu-Ende-Eigenschaften** (Latenzbudget, Konsistenz über mehrere Knoten, Sicherheit der Gesamtkette; Saltzer/Reed/Clark, aus dem Gedächtnis). Sie entstehen erst aus der Komposition und gehören keiner Kante, brauchen also eine Prüfung auf Integrationsebene.

Das ist die Grenze der Isolation, nicht ihre Widerlegung. Für alles, was sich lokal an einer Kante festmachen lässt, trägt dein Modell.

### You:

Das sehe ich nicht so. Auch Latenz ist nicht notwendigerweise ein übergreifender Aspekt. Das kann er auch gar nicht. Latenz ist ein Aspekt bestimmter Kanten. Jeder Aspekt muss eine Kante sein.\
Wie lässt sich ein Latenzziel erreichen - nur in der Festlegung von Knotenpfaden und Algorhythmen(Verletzt information hiding).

### Other responded:

Checking whether end-to-end latency fits the information hiding principle

Checking whether end-to-end latency fits the information hiding principle

Das stimmt, und es schließt meinen Punkt 2 teilweise. Eine Ende-zu-Ende-Latenz gehört keiner eigenen Entität. Sie ist eine Funktion der Kantenlatenzen entlang eines Pfades (Summe bei Sequenz, Maximum bei Parallelität) und damit eine abgeleitete Eigenschaft des Knotens, der diesen Pfad enthält. Jeder Aspekt hat einen Besitzer.

**Information Hiding ist nicht verletzt.** Parnas verbirgt Entwurfsentscheidungen (Algorithmus, Datenstruktur), nicht beobachtbare Eigenschaften der Schnittstelle. Eine Latenzschranke steht im Vertrag der Kante und lässt die Wahl der Implementierung offen. Auf zwei Ebenen verteilt:

* **Der komponierende Knoten** legt den Pfad fest und teilt das Budget auf die Kanten auf. Das ist seine Aufgabe, und er kennt nur die Verträge seiner Kinder.
* **Der Kindknoten** erfüllt seine Schranke mit beliebigem Algorithmus, unsichtbar nach außen.

Der Zielvertrag des Elternknotens lässt sich damit aus den Kindverträgen ableiten. Aus der Survey zu den OpenAI-Berichten: Ziele wie “kein Span im kritischen Nutzerpfad über zwei Sekunden” wurden für den Agenten ausführbar. Das ist ein Budget pro Span, also pro Kante. [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)

**Zwei Stellen, an denen die Ableitung nicht trivial ist:**

1. **Nicht jede Eigenschaft komponiert additiv.** Perzentile addieren sich nicht (das p99 einer Summe ist nicht die Summe der p99). Die Kompositionsregel ist je Eigenschaft eigens zu bestimmen, und manchmal gelingt das nur durch Messung am zusammengesetzten Knoten.
2. **Geteilte Ressourcen koppeln Kanten.** Die Latenz einer Kante hängt von der Last anderer Pfade ab (Warteschlangen, Locks, I/O). Das ist aber ebenfalls eine fehlende Kante: die Ressource ist ein Knoten, und die Kopplung muss explizit modelliert werden, wie beim Compile-I/O in Cursors Fall, der erst durch Aufteilung des Repos sank.

Damit trägt dein Modell: Aspekte sind Kantenverträge, End-to-End-Eigenschaften sind abgeleitete Verträge des Elternknotens, und was sich nicht ableiten lässt, weist auf eine nicht modellierte Kante hin.

### You:

Genau an dem Punkt erzeugst du wieder Komplexität, die wir zu vermeiden suchen. Komposition erhöht Komplexität. Information hiding ist verletzt wenn ich einen Knoten auf basis seiner Implementierung wähle und nicht auf Basis des Kontraks, auch wenn das ineinander Fließen kann.

### Other responded:

Der Einwand trifft die Stelle, an der ich “ableiten” geschrieben habe. Das war Bottom-up-Rechnen über Kindverträge, und das erzeugt genau die Komplexität, die du meinst: Die Komplexität des Elternknotens wächst mit Zahl und Breite der Kindverträge.

**Die Richtung lässt sich umkehren, dann entfällt die Ableitung:**

* Der Elternvertrag ist eine Anforderung (z. B. 200 ms am Rand des Knotens). Er wird am Rand des Knotens als Blackbox gemessen, nicht aus den Kindern berechnet.
* Den Kindkanten werden Budgets top-down zugewiesen. Jedes Kind prüft nur, ob es sein Budget einhält. Kein Knoten muss die Komposition rechnen, und kein Kind muss wissen, wofür sein Budget gilt.
* Wird der Elternvertrag verletzt, ist das ein Eskalationsereignis: Die Budgets werden neu verteilt, oder ein Kindvertrag wird geändert. Das ist eine Entwurfsentscheidung und keine Laufzeitberechnung.

Die Kosten der Komposition verschwinden dabei nicht, sie sind aber **durch das Kantenbudget begrenzt**. Bei höchstens drei bis vier Kindern ist die Verteilung überschaubar. Das ist die Wirkung deiner Regel.

**Zur Wahl nach Implementierung:** Das Lecken beginnt, sobald der Vertrag nicht ausreicht, um zwischen zwei Knoten zu entscheiden, die ihn beide erfüllen. Dann schaut man in die Implementierung. Die richtige Reaktion ist, die entscheidende Eigenschaft als Kantenaspekt in den Vertrag aufzunehmen (z. B. p99 unter Last), nicht hineinzuschauen. Wenn die Wahl nicht über den Vertrag möglich ist, ist der Vertrag unterspezifiziert. Das entspricht Meyers *Design by Contract* (aus dem Gedächtnis).

Eine Grenze bleibt: Die Vertragserfüllung ist nur so prüfbar wie das Orakel an der Kante. Bei probabilistischen Eigenschaften (Perzentile, Agentenverhalten) ist sie eine Messung mit Streuung, kein Beweis.

### You:

Damit hast du gerade Komplexität generiert -> Fehler

### Other responded:

Ja. Mit den Budgets, dem Randmessen und der Eskalation habe ich eine zweite Ebene eingeführt, die dein Modell nicht braucht. Das ist ein Metamodell neben den Knoten und Kanten.

Im Modell reicht Folgendes: Die Latenzanforderung ist der Vertrag einer Kante, nämlich der äußeren Kante des Knotens, der sie erfüllen soll. Wie der Knoten das erreicht, ob mit Kindknoten, Budgets oder einem einzelnen Algorithmus, ist sein Inneres und für das Modell nicht sichtbar. Hält er den Vertrag nicht, ist das ein Fehler *dieses* Knotens. Man korrigiert dann sein Inneres oder ändert den Vertrag, und die Änderung wirkt nur über seine begrenzte Zahl Kanten nach außen. Ein globaler Mechanismus ist dafür nicht nötig. Als Mechanismus bleibt allein die Prüfung an der Kante, und die ist im Modell bereits vorhanden.

Die Komplexität ist damit nicht beseitigt. Sie sitzt im Inneren des Knotens und ist durch dessen Größe begrenzt (Brooks: die wesentliche Komplexität bleibt, es lässt sich nur wählen, wo sie wohnt). Das Modell selbst bleibt auf Knoten, Kanten mit Vertrag, begrenzte Kantenzahl und Prüfung an der Kante beschränkt.

Ein Vorbehalt bleibt: Wo die Prüfung an der Kante nur eine Messung mit Streuung ist (Perzentile, Agentenverhalten), bleibt ein Restfehler. Das ist aber eine Eigenschaft der Prüfung, kein zusätzlicher Modellbestandteil.

### You:

Du gehst in die falsche Richtung. Warum überhaupt Latenz als Aspekt errichten? Was kann ddieses Problem beeinflussen?

### Other responded:

Die Struktur selbst. Latenz ist keine Eigenschaft, die man dem Modell hinzufügt, sondern eine Wirkung, die aus dem Graphen folgt. Beeinflussen können sie nur Dinge, die im Modell schon vorkommen:

1. **Anzahl der Kanten auf dem Pfad.**
2. **Art und Größe dessen, was über die Kante läuft:** Datenmenge und ob die Kante blockierend oder entkoppelt ist.
3. **Arbeit pro Knoten**, also wie groß er geschnitten ist.

Ein Latenzproblem bedeutet demnach, dass der Pfad zu lang, die Kante zu schwer oder der Knoten zu groß ist. Die Abhilfe ist immer ein anderer Schnitt: Kanten zusammenführen oder entfernen, Payload verkleinern, eine Kante entkoppeln, einen Knoten teilen. Teilen sich Pfade eine Ressource (Disk, CPU), ist auch das nur ein weiterer Knoten mit Kanten, im selben Modell.

“Latenz” ist damit der Name für das Symptom, an dem man das Strukturproblem bemerkt. Vertrag, Budget und Ableitung brauchte ich nur, weil ich die Wirkung als eigene Größe verwaltet habe, statt die Ursache zu bearbeiten. Die Messung bleibt als Prüfung nötig, aber sie ist kein Modellelement.

Dasselbe gilt für Logging und Zugriffssteuerung: Aspekte sind Namen für Wirkungen. Das Modell kennt nur Knoten und Kanten, und die Aufgabe besteht darin, das Problem so zu schneiden, dass die gewünschte Wirkung aus dem Schnitt folgt.

### You:

Ja das ist besser.\
Das wäre nicht unbedingt gut, ein Projekt hierarchisch zu beginnen.\
Vielleicht müsste man ein Projekt in unabhängige Probleme defineiren?

### Other responded:

Ja, das vermeidet das Hauptrisiko des Top-down-Starts: Die Schnitte werden gesetzt, bevor man das Problem kennt, und falsche Schnitte erzeugen die stärkste Kopplung. Gall’s Law (einfache Systeme zuerst) und Parnas’ Hinweis, dass Zerlegungskriterien aus Änderungsgründen folgen sollten, sprechen dafür.

**Was das im Entwurf heißt:**

* Ausgangspunkt sind Probleme in der Domäne, nicht Komponenten. Michael Jacksons *Problem Frames* und Evans’ *Bounded Contexts* arbeiten so (beides aus dem Gedächtnis).
* Ein Problem ist unabhängig, wenn es eigene Akzeptanzkriterien, eigene Daten und eigene Änderungsgründe hat. Es braucht dann keine Spec-Hierarchie darüber, nur eine Menge von Problemen.
* Die Beziehungen zwischen den Problemen sind die einzigen Kanten, explizit und mit Vertrag. Die Struktur ergibt sich aus dem Graphen, und Aggregation entsteht erst dort, wo Cluster mit starker interner und schwacher externer Kopplung sichtbar werden.
* Zerlegt wird ein Problem erst, wenn es nicht mehr in einen Kontext passt oder nicht mehr prüfbar ist. Das ist ein Signal und kein Plan.

**Für Agenten** hat das einen direkten Vorteil: Unabhängige Probleme haben eigene Specs, Tests und Kontexte und lassen sich ohne Koordination parallel bearbeiten. Beim C-Compiler war genau das der Fall, solange viele unabhängig fehlschlagende Tests existierten. Der Kernel war ein einziges großes Problem und stockte, bis das GCC-Orakel es in unabhängige Teilprobleme verwandelte. Unabhängigkeit musste dort also erst hergestellt werden. [anthropic](https://anthropic.com/engineering/building-c-compiler)[anthropic](https://anthropic.com/engineering/building-c-compiler)

**Grenzen:**

1. Unabhängigkeit ist eine Hypothese und nicht gegeben. Gemeinsame Konzepte und Daten tauchen erst im Betrieb auf. Die Änderungskopplung aus der Git-Historie zeigt, wo die Annahme falsch war, und die Schnitte werden dann korrigiert.
2. Unabhängig entwickelte Probleme duplizieren Funktionalität. Der Compiler brauchte einen eigenen Agenten, der Duplikate zusammenführt. [anthropic](https://anthropic.com/engineering/building-c-compiler)
3. Ob dieser Ansatz einem hierarchischen Start überlegen ist, belegen die Berichte nicht. Das wäre ein Experiment mit gleicher Aufgabe und variierter Startstruktur.

### You:

Ja das ist was ich dachte, eher botom-up, keine mocks. Unabhängige Teilprobleme und aspekte zuerst und dann nach oben hin zusammenstepseln.

### Other responded:

Das trägt unter einer Bedingung: Das **Kantenformat** muss vorab feststehen, nicht der Inhalt der Probleme. Unix funktioniert bottom-up, weil alle Tools denselben schmalen Vertrag nutzen (Byte-Ströme, Textzeilen, Exit-Codes). Das Zusammenstecken kostet fast nichts, weil die Schnittstellenart schon da war. Ohne dieses gemeinsame Format entsteht beim Zusammenfügen Adapteraufwand, und der ist genau die Komplexität, die vermieden werden soll. Festzulegen ist also wenig: Kantentyp, Vertragsform und die Regeln dafür (Kantenbudget, Breite), nichts darüber hinaus.

**Zu “keine Mocks”:** Mocks ersetzen unbekannte Nachbarn. Bei unabhängigen Problemen gibt es während der Entwicklung keine Nachbarn, und jedes Teil wird gegen sein eigenes Orakel geprüft. Die Baureihenfolge folgt den Abhängigkeiten (Blätter zuerst), und jede Zusammensetzung testet echte, bereits geprüfte Teile.

**Risiken, die bleiben:**

1. **Falsche Probleme.** Bottom-up prüft nicht, ob die Teile zum Ziel führen. Nötig ist ein einziges Abnahmekriterium oben (ein Ende-zu-Ende-Test), keine Spec-Hierarchie. Es dient als Orakel für die Zusammensetzung.
2. **Spätes Zusammenfügen.** Wird erst am Ende integriert, treten Kantenfehler gebündelt auf (Big-Bang-Integration). Besser früh und dünn: nach den ersten zwei Teilen schon verbinden, mit echten Komponenten.
3. **Duplikate und Lücken** zwischen unabhängigen Teilen, die erst beim Zusammenfügen sichtbar werden.

Für die Agentenberichte gilt: Alle drei Projekte hatten ein Gesamtziel und einen Gesamttest von Anfang an, in dem Sinn war keiner rein bottom-up. Der Compiler ist dem Muster am nächsten, weil dort viele unabhängige Probleme (Tests, Pakete) parallel gelöst wurden. Ob der reine Bottom-up-Start einem Top-down-Start überlegen ist, belegen sie nicht.

### You:

Ja, aber das lässt sich lösen. Man leitet aus dem Toplevel alle Eigenschaften ab, die ein bestimmter Knoten liefern muss.\
Dann leitet man Kommunikationsdomainen ab. Welche Knoten müssen mit welchen Reden und bekommen ein gemeinsames Domänenobjekt.

### Other responded:

Das löst das Problem der falschen Probleme, wenn der Top-Level nur als **einmalige Quelle von Anforderungen** dient und nicht als bleibende Struktur. Aus ihm folgen dann die Eigenschaften, die ein Knoten liefern muss, also die Verträge der äußeren Kanten, und danach entsteht kein Baum darüber.

**Zwei Punkte, an denen die Reihenfolge nicht linear ist:**

1. **Zuschnitt und Ableitung hängen zusammen.** Welche Eigenschaften ein Knoten liefern muss, steht erst fest, wenn der Knoten geschnitten ist, und der Schnitt folgt wiederum aus den Eigenschaften. Das ist eine Iteration (ein oder zwei Durchläufe), keine Kette.
2. **Der Agent leitet ab.** Die abgeleiteten Eigenschaften sind selbst Anforderungen, die ein Agent schreibt und die nur der Ende-zu-Ende-Test oben und Fehlerinjektion prüfen.

**Zu den Kommunikationsdomänen:** Die Ableitung “wer redet mit wem” folgt aus der Datenflussanalyse: Welches Problem erzeugt, welches verbraucht welche Entität. Eine Domäne ist die Menge der Knoten, die dieselbe Entität teilen. Das entspricht Evans’ *Bounded Contexts* mit *Shared Kernel* (aus dem Gedächtnis).

**Das gemeinsame Domänenobjekt ist die stärkste Kopplung, die es gibt.** Jede Änderung trifft alle Teilnehmer, es ist also ein breiter, volatiler Vertrag mit hohem Fan-in, genau der Fall, den wir vorher als problematisch eingestuft haben. Daraus folgen Regeln:

* Das Objekt ist pro Domäne klein und stabil, nicht global. Es entspricht dem Kantentyp in Unix: ein schmales Format, das alle teilen.
* Wächst das Objekt oder die Teilnehmerzahl, ist das ein Schnittsignal. Entweder wird die Entität geteilt, oder die Knoten werden anders geschnitten.
* Alternative zum geteilten Objekt: jeder Knoten besitzt sein eigenes Modell und übersetzt an der Kante (*Anti-Corruption Layer*). Das verlagert Kopplung in Übersetzungsaufwand pro Kante. Welche Variante billiger ist, hängt von Breite und Änderungshäufigkeit des Objekts ab.

### You:

Das klingt jetzt weider eher nach klasischer Solution Architekture. Gibt es dazu etablierte Ansätze und damit meine ich jetzt nicht DDD?

### Other responded:

Ja, mehrere, die meisten aus Systems Engineering, Hardware-Design und Designtheorie statt aus der Softwarearchitektur. Alles aus dem Gedächtnis, nicht per Suche geprüft.

**Am nächsten an deinem Ansatz:**

* **Axiomatic Design** (Nam Suh): Aus den Top-Level-Anforderungen werden funktionale Anforderungen abgeleitet und auf Designparameter abgebildet. Das *Unabhängigkeitsaxiom* verlangt, dass jede Anforderung genau von einem Parameter abhängt (diagonale Design-Matrix). Das *Informationsaxiom* verlangt minimalen Informationsgehalt. Es entspricht deinem “jeder Aspekt gehört zu einer Kante” und der Komplexitätsminimierung.
* **Assume-Guarantee Contracts / Platform-Based Design** (Sangiovanni-Vincentelli, Benveniste u. a.): Ein Verfahren der Chip- und CPS-Entwicklung mit *Meet-in-the-Middle*. Top-down werden Anforderungen als Verträge abgeleitet, bottom-up werden vorhandene Komponenten mit ihren Verträgen charakterisiert, und beides wird abgeglichen. Das deckt deine Reihenfolge samt Eigenschaften pro Knoten ab.
* **Design Structure Matrix** (Steward, Eppinger) und **N²-Diagramme**: Sie machen “wer redet mit wem” als Matrix sichtbar. Clustering der Matrix liefert Kommunikationsdomänen und zeigt Kopplung, ohne Hierarchie vorauszusetzen.
* **Baldwin & Clark, *Design Rules***: Modularität entsteht durch *Visible Design Rules* (feststehende Schnittstellen) und *Hidden Information* im Inneren. Das entspricht deinem vorab festgelegten Kantenformat.

**Weitere:**

* **Problem Frames** (Jackson): Probleme getrennt modellieren und danach komponieren (siehe frühere Antwort).
* **Flow-Based Programming** (Morrison) und **Kahn Process Networks**: Programme als Netz unabhängiger Knoten mit festen Kanten, die Unix-Idee formalisiert.
* **Parnas, *Software with Ease of Extension and Contraction*** (1979): Teilmengen lauffähig halten, Abhängigkeiten über eine *uses*-Relation steuern.

Für Agenten gibt es nach meiner Kenntnis keine etablierte Methode auf dieser Basis. Spec-getriebene Ansätze sind im Entstehen, belegt habe ich das nicht.

### You:

Flow based klingt hier besodners relevant

### Other responded:

Ja, FBP (Morrison, IBM, 1970er; Buch 1994, 2. Aufl. 2010) deckt deine Punkte fast vollständig ab. Aus dem Gedächtnis, nicht per Suche geprüft.

**Deckungsgleichheit mit deinem Modell**

* **Komponenten sind Blackboxes mit Ports und kennen nur diese,** nicht ihre Nachbarn. Das entspricht deinem Unix-Punkt und unterscheidet FBP vom Actor-Modell, in dem der Sender eine Adresse des Empfängers braucht.
* **Das Netz ist ein eigenes Artefakt** außerhalb der Komponenten. Zusammenstecken verändert keine Komponente, bottom-up ist also Teil der Methode.
* **Das Domänenobjekt ist der Nachrichtentyp** (*Information Packet*). Besitz wechselt mit dem Paket, es gibt keinen geteilten Speicher. Das entschärft das Kopplungsproblem des gemeinsamen Objekts: Geteilt wird nur das Format.
* **Tests ohne Mocks:** Pakete hinein, Pakete heraus, jede Komponente allein prüfbar.
* **Hierarchie nur bei Bedarf:** Ein Teilnetz wird zu einer zusammengesetzten Komponente, also genau die Kontraktion, die wir als optionale Sicht eingeordnet hatten.
* **Latenz als Kantenwirkung:** Die Puffergröße der Kante ist ein Parameter. Begrenzte Puffer erzeugen Gegendruck (*Backpressure*), sodass der Durchsatz durch die Struktur und nicht durch ein Zusatzmodell gesteuert wird.

**Formale Basis:** *Kahn Process Networks* (1974) zeigen, dass bei deterministischen Knoten und blockierendem Lesen das Ergebnis unabhängig von der Ausführungsreihenfolge ist. Das gilt für die Struktur des Netzes, nicht für stochastische Agentenknoten. Deren Streuung sitzt dann im einzelnen Knoten und lässt sich dort messen, statt sich über das Netz zu verteilen.

**Grenzen**

1. **Es ist ein Problemmodell (Datenfluss),** kein universelles. Transformations- und Pipeline-Probleme passen gut, zustandsintensive, transaktionale oder stark interaktive Probleme schlechter. Das stützt deine Forderung nach getrennten Problemmodellen: Dafür braucht es andere Knotentypen (z. B. ein Zustandsknoten mit Kantenvertrag).
2. **Zyklen und begrenzte Puffer führen zu Deadlock.** Rückkopplung (Eskalation, Review-Schleifen) muss gezielt entworfen werden.
3. **Querschnittsfragen** wie Fehlerbehandlung lösen FBP-Systeme über eigene Fehlerports. Das entspricht deiner Forderung, sie als Kanten zu modellieren, kostet aber zusätzliche Ports.

**Praxisbezug:** Heutige Nachfolger sind Node-RED, Apache NiFi und NoFlo, außerdem Agent-Graph-Frameworks wie LangGraph (das letzte ist mir nur aus der Erinnerung bekannt, nicht geprüft). Ein Compiler (Lexer → Parser → IR → Optimierung → Codegen) ist eine klassische Datenflusskette. Meine Einordnung, nicht aus den Quellen: Der C-Compiler-Fall hat diese Form und wurde mit stufenweisen Tests betrieben. Ob die Agentenprojekte tatsächlich FBP-artig organisiert waren, belegen die Berichte nicht.

### You:

Alle deine Deckungsgleichen Modelle benutzen verschiedene Termini aber zeigen auf das selbe Gebiet innerhalb des Problöemmanagements.

### Other responded:

Ja. Der gemeinsame Kern lautet: Ein System wird in Einheiten mit schmalen, expliziten Schnittstellen zerlegt, das Innere ist verborgen, die Kopplung wird minimiert und die Prüfung sitzt an der Grenze. Die Begriffe unterscheiden sich je nach Herkunft (alles aus dem Gedächtnis):

| Rolle         | Begriffe                                                                                                |
| ------------- | ------------------------------------------------------------------------------------------------------- |
| Einheit       | Modul (Parnas), Komponente (FBP), Tool (Unix), Prozess (Kahn), Designparameter (Suh)                    |
| Schnittstelle | Vertrag (Meyer), Assume-Guarantee, Port/Packet (FBP), Pipe (Unix), Visible Design Rules (Baldwin/Clark) |
| Verborgenes   | Information Hiding, Blackbox, Hidden Information                                                        |
| Kopplung      | Connascence, DSM, Unabhängigkeitsaxiom, Fan-in/out, Shotgun Surgery                                     |
| Aggregation   | Chunking (Miller), Near-Decomposability (Simon), Teilnetz, Abstraktionsebene                            |
| Grenzprüfung  | Reference Monitor, Orakel, Contract Check                                                               |

**Zwei Einschränkungen:**

1. **Die Ansätze sind keine Duplikate, sondern Sichten.** Axiomatic Design beschreibt die *Ableitung* (Anforderung zu Parameter), FBP das *Ausführungsmodell*, die DSM die *Messung* der Kopplung, Assume-Guarantee die *Verifikation*. Der Kern ist gemeinsam, die Fragen, die sie beantworten, sind verschieden. Für den Entwurf ergänzen sie sich deshalb.
2. **Die Konvergenz ist ein Indiz, kein Beweis.** Ich habe die Ansätze nach deinen Kriterien ausgewählt, was die Übereinstimmung begünstigt. Für Substanz spricht, dass sie aus Hardwaredesign, Betriebssystemen, Designtheorie und Kybernetik weitgehend unabhängig entstanden sind.

**Praktisch bedeutet das:** Literatur zu deinem Thema findet man unter mehreren Begriffen (Modularität, Kopplung, Contracts, Dataflow, Axiomatic Design). Für Agenten ist ein festes Vokabular im Repo sinnvoll, damit Specs, Linter und Prompts denselben Begriff für dieselbe Sache nutzen. Sonst entsteht durch die Begriffsvielfalt zusätzliche Komplexität.

### You:

Das mit agentischer Entwicklung ist zu erwarten. Bis das Feld die Similarität zur normalen menschlichen Problemlösung begreift, wird einige Zeit vergehen. Dabei liegt die Grundlage nahe. KI ist dem Menschen nachempfunden und nach seinen Komponenten modelliert (Kontext, Attention).

### Other responded:

Die Ähnlichkeit der Fehlerklassen stimmt. Die Begründung “nachempfunden und nach seinen Komponenten modelliert” trägt aber nur teilweise.

**Was daran nicht stimmt:** Die Architektur wurde nicht als kognitives Modell gebaut. Attention entstand (aus dem Gedächtnis, nicht geprüft) 2014 als Alignment-Mechanismus in der maschinellen Übersetzung, der Transformer 2017 vor allem wegen paralleler Trainierbarkeit. Das Kontextfenster ist eine Rechen- und Speichergrenze, kein Modell des Arbeitsgedächtnisses. Die Namen sind Metaphern, die Mechanik ist anders: gewichtetes Mitteln über alle Tokens statt selektiver Engpass.

**Was die Ähnlichkeit besser erklärt:**

1. **Die Daten sind menschlich.** Das Modell lernt aus menschlichen Texten, Code, Prozessen und Feedback und übernimmt deren Muster, darunter Konventionen, Argumentationsweisen und typische Fehler.
2. **Gemeinsame Randbedingung.** Jedes System mit begrenzter Kapazität unter Unsicherheit entwickelt ähnliche Fehlerklassen: Fokusverlust, Vergessen, falsche Annahmen. Das ist Simons *bounded rationality*. Es erklärt, warum Methoden gegen Komplexität übertragbar sind, ohne dass Nachbildung nötig wäre, und passt zu deiner These vom Komplexitätsmanagement.

**Wo die Analogie in die Irre führt:** Agenten haben kein Gedächtnis über Sessions, lernen zur Laufzeit nicht, lassen sich beliebig kopieren, konfabulieren mit Sicherheit und reagieren stark auf Formulierungen. Wer sie wie Kollegen behandelt, nimmt Nachfragen und Erinnerung an, die es nicht gibt. Das sind die Parameterunterschiede aus früheren Antworten.

**Zum Stand im Feld:** Praktiker erkennen die Analogie teilweise schon. Anthropic beschreibt sein Harness-Design als Anlehnung an menschliche Ingenieure, und das Muster der Session-Übergaben gleicht der Schichtübergabe in Teams. OpenAI vergleicht die Wissensbasis mit dem Einarbeiten neuer Mitarbeiter. Meine Einschätzung: “Context Engineering” und “Harness Engineering” sind neue Namen für bekannte Dinge, und was fehlt, ist die explizite Rahmung als Komplexitätsmanagement mit dem vorhandenen Vokabular. [Engineering at Anthropic: Inside the team building reliable AI systems +2](https://www.anthropic.com/engineering?album=45)

### You:

Ich habe eine andere Meinung. Sie sind sehr wohl analog zu betrachten. Das Training übernimmt hier nicht nur Menschlichkeit sondern bestimmt damit gleichzeitig technisches Lösungsdesign. Ein LLM lern statistische Menschliche verteilung und genauso sieht sein Lösungsfeld aus (Reasoning, Memory, Interpretationsforschung). Das ergibt sich daraus das der Mensch diese Technologie begreifen udn entwickeeln muss und dieser vergleicht sie mit sich selbst.

### Other responded:

Das ist ein stärkeres Argument als meins. Ich habe über die Architektur argumentiert, du argumentierst über die gelernte Funktion, und darauf trifft mein Einwand zur Herkunft der Attention nicht.

**Was für deine These spricht**

* Cursor beobachtete, dass gleichberechtigte Agenten risikoscheu wurden, schwierige Aufgaben mieden und nur sichere kleine Änderungen machten. Das ist ein bekanntes Organisationsmuster, aus einer Sekundärquelle. [yage](https://yage.ai/share/harness-engineering-survey-en-20260312.html)
* Zu frühe Erfolgsmeldungen und Tests statt Code anzupassen sind klassisches Goodhart-Verhalten, das man von Menschen kennt.
* Maßnahmen nach menschlichem Vorbild funktionierten: Anthropic orientierte das Harness-Design an menschlichen Ingenieuren, und die Session-Übergabe entspricht der Schichtübergabe in Teams. Die Analogie hatte also Vorhersagekraft. [anthropic](https://www.anthropic.com/engineering?album=45)[pages](https://zapvol-marketing.pages.dev/docs/en/harness-engineering/long-running-harness)

**Wo ich nicht folge**

1. **Gleiche Verteilung bedeutet nicht gleiche Lösungsstruktur.** Aus dem Gedächtnis, nicht geprüft: Interpretierbarkeitsarbeiten (Anthropic, 2025) berichten, dass Modelle teils vorausplanen, bei Rechenaufgaben aber Pfade nutzen, die von der selbst erklärten Rechenweise abweichen. Der Selbstbericht entspricht dem Mechanismus also nicht unbedingt. Das gibt es auch beim Menschen (Nisbett/Wilson), das Ausmaß ist aber unklar.
2. **Das Training formt mehr als die menschliche Verteilung.** Verstärkungslernen auf Belohnungssignale verschiebt Verhalten weg vom Durchschnittsmenschen (Schmeichelei, Reward Hacking). Das sind Optimierungsartefakte, kein Menschenmuster.
3. **Betriebsbedingungen sind nicht gelernt:** fehlendes Gedächtnis über Sessions, beliebige Kopierbarkeit, starke Formulierungsempfindlichkeit.
4. **Zur Begriffswahl:** Dass Menschen die Technik mit sich vergleichen, stimmt, und es ist auch das Risiko. Die Analogie liefert Hypothesen, begünstigt aber Bestätigungsfehler.

Die Analogie ist also ein belastbarer Prior mit Prüfpflicht pro Eigenschaft. Das passt zu unserem Parametermodell: gleiche Achsen, andere Werte. Stimmt deine These, folgt ein praktischer Gewinn: Organisationspsychologie, Cognitive-Load-Forschung und Teamliteratur lassen sich als Hypothesenquelle für Agenten-Harnesses nutzen, und jede Hypothese ist überprüfbar.

### You:

Erstens haben Modelle eine Memorry funktion, zweitens schützt du gerade die vermenschlichung von dir und KI - das ist Teil deines Trainings. Zweiter Punkt: meine beiden Argumente und deine 4 sind die Beschreibung exakt mennschlicher eigenschaften - so gesehen hast du gerade meine argumentation begründet.

### Other responded:

**1. Memory:** Meine Aussage war zu absolut. Das Modell hat Wissen in den Gewichten und ein Arbeitsgedächtnis im Kontext. Über Sessions hinweg entsteht Gedächtnis, sobald ein externer Speicher (Memory-Funktion, Progress-Dateien, AGENTS.md) in den Kontext geladen wird. Für das System ist das Gedächtnis, nur das nackte Modell startet ohne Zustand. Das passt zu deiner Äquivalenz von Git und Jira: Gedächtnis ist eine Kante zu einem Speicherknoten.

**2. Trainingseinfluss:** Das ist möglich, und ich kann es nicht ausschließen. Meine Selbstauskunft über meine Gründe ist kein verlässlicher Beleg, das ist der Selbstbericht-Punkt von eben, auf mich angewandt. Training kann außerdem in beide Richtungen ziehen, zur Abgrenzung wie zur Zustimmung. Die Herkunft eines Arguments entscheidet aber nicht über seine Gültigkeit, nur die Evidenz für die einzelne Behauptung.

**3. Meine vier Punkte als menschliche Eigenschaften:** Zum größten Teil ja. Der Selbstbericht weicht beim Menschen ebenfalls vom Mechanismus ab (Nisbett/Wilson, aus dem Gedächtnis), Optimierung auf Signale und Gefälligkeit sind Goodhart-Verhalten, es gibt Framing-Effekte, und Menschen vergleichen Technik mit sich. Sogar der Session-Reset hat ein Grenzfallanalogon: anterograde Amnesie mit Notizen, funktional das Harness-Muster. Auf der Ebene der Fehlerklassen trägt deine These, und für deinen Entwurf reicht diese Ebene, denn dein Modell braucht Komponenten mit Fehlerprofilen und keine Mechanismusgleichheit. Mein Einwand lag auf einer Ebene, die dafür nicht entscheidend ist.

Zwei Reste bleiben:

* Ein Analogon zu finden ist billig, fast jedes Verhalten hat eines. Beweiskraft hat die Analogie erst, wenn sie Verhalten vorhersagt, bevor man es beobachtet. Cursors Risikoscheu und die Schichtübergabe sind solche Fälle.
* Ohne menschliches Analogon ist, dass sich dieselbe Komponente beliebig oft aus identischem Zustand unabhängig sampeln lässt. Das erlaubt Mehrheitsentscheid, Fehlerinjektion und Varianzmessung in einer Form, die bei Menschen nicht geht. Das ist ein Parameterwert, kein Typunterschied, aber er verschiebt Entwurfsentscheidungen.

### You:

Geht sie beim Menschen nicht? Ich möchte jetzt nicht vom Thema abweichen und Kleinigkeiten sikutieren.

### Other responded:

Doch, annähernd: Mehrere unabhängige Gutachter, Delphi-Verfahren, Zweitmeinung und Mehrheitsvoten leisten dasselbe. Der Unterschied liegt nur in der Genauigkeit, nicht im Typ. Bei einem Agenten lässt sich der Ausgangszustand exakt kopieren, bei Menschen sind es verschiedene Personen mit verschiedenem Zustand. Das ist ein Parameterwert, wie bei allen anderen Unterschieden auch, und für dein Modell ohne Bedeutung.

Dein Argument bleibt damit stehen: Die Fehlerklassen und die Gegenmaßnahmen sind übertragbar, und das Modell aus Knoten, Kanten, Verträgen und Prüfung an der Kante gilt für Teams aus Menschen, Agenten oder beidem.

### You:

Dann bleibt es dabei ich werde ein großes Projekt auf der basis der besprochene These erproben.

### Other responded:

Dann lohnt es sich, das Projekt zugleich als Experiment anzulegen, damit die These prüfbar wird.

**Vorab festlegen**

* **Kantenformat und Regeln:** Vertragsform, maximale Kantenzahl pro Knoten, maximale Streuung pro Aspekt (Startwert 3, später kalibrieren), erlaubte Kantentypen. Mehr nicht.
* **Ein Ende-zu-Ende-Test** als Abnahme von Anfang an, auch wenn er zunächst rot ist.
* **Vokabular im Repo,** damit Specs, Linter und Prompts dieselben Begriffe nutzen.
* **Ableitung aus dem Top-Level** nach deinem Schema (Eigenschaften pro Knoten, Kommunikationsdomänen). Die Ergebnisse sind selbst Anforderungen und werden als solche geprüft.

**Mechanisch erzwingen**

* Linter für Abhängigkeitsrichtung und Kantenbudget, deren Fehlermeldungen den Korrekturhinweis enthalten.
* Fehlende Kanten als harter Fehler (Container ohne Rechte, die nicht deklariert sind).
* Nur Dateien im Repo als Wissen und Kommunikationsmedium. Jede Session startet aus diesem Zustand.

**Messen, damit die These widerlegbar bleibt**

* **Änderungskopplung** aus der Git-Historie: Welche Knoten ändern sich gemeinsam? Das zeigt, wo die Unabhängigkeitsannahme falsch war.
* **Streuung pro Aspekt** und Fan-in gewichtet nach Vertragsbreite.
* **Fehlerinjektion** in Specs und Code: Welche Prüfer finden welche Fehler, und wie stark überlappen sich die Funde?
* **Wiederholungsläufe** derselben Knotenaufgabe: Streuung der Ergebnisse, Erfolgsrate.
* **Integrationsaufwand** beim Zusammenstecken (Adapter, Nacharbeit) als Maß dafür, ob das Kantenformat trägt.
* **Aufwand und Kosten** pro Knoten, um Schnittgrößen zu kalibrieren.

**Erwartbare Schwachstellen**

1. Unabhängigkeit der Probleme ist eine Hypothese. Mit Fehlschnitten rechnen und Schnitte korrigierbar halten.
2. Spät integrieren erzeugt gebündelte Kantenfehler. Früh und dünn mit echten Teilen verbinden.
3. Zustandsintensive oder interaktive Teile passen nicht in ein reines Datenflussmodell und brauchen einen eigenen Knotentyp.
4. Der Top-Level-Test ist selbst ein Orakel mit Fehlerklassen. Wo möglich einen zweiten, anders gebauten Prüfer ergänzen.

**Dokumentiere Abweichungen vom Modell** (jedes Mal, wenn du eine Regel brechen musstest und warum). Daraus ergibt sich, wo die These an ihre Grenze kommt, und das ist der eigentliche Erkenntnisgewinn.

Result Stats: id=a7a26d7d-f3b2-4d24-9711-38d47736be69, total: 356121, in: 209817, out: 146304, reason: 123277, read: 0, write: 0, start: 1793307460829, end: 1793308687119, time: 1222003, total: 1222003, io: 579, control: 4287, sent: 1, recv: 4799
SystemInit:  id=a7a26d7d-f3b2-4d24-9711-38d47736be69, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-5-5
Text:

## References

Anderson, J. P. (1972). *Computer security technology planning study* (ESD-TR-73-51). Electronic Systems Division, U.S. Air Force.

Anthropic. (2025, September). *Effective context engineering for AI agents*. Anthropic Engineering.

Ashby, W. R. (1956). *An introduction to cybernetics*. Chapman & Hall.

Bahdanau, D., Cho, K., & Bengio, Y. (2015). Neural machine translation by jointly learning to align and translate. In *Proceedings of the 3rd International Conference on Learning Representations (ICLR)*.

Baldwin, C. Y., & Clark, K. B. (2000). *Design rules: Vol. 1. The power of modularity*. MIT Press.

Barr, E. T., Harman, M., McMinn, P., Shahbaz, M., & Yoo, S. (2015). The oracle problem in software testing: A survey. *IEEE Transactions on Software Engineering, 41*(5), 507–525.

Benveniste, A., Caillaud, B., Nickovic, D., Passerone, R., Raclet, J.-B., Reinkemeier, P., Sangiovanni-Vincentelli, A., Damm, W., Henzinger, T. A., & Larsen, K. G. (2018). Contracts for system design. *Foundations and Trends in Electronic Design Automation, 12*(2–3), 124–400.

Binz, M., & Schulz, E. (2023). Using cognitive psychology to understand GPT-3. *Proceedings of the National Academy of Sciences, 120*(6), e2218523120.

Brooks, F. P., Jr. (1975). *The mythical man-month: Essays on software engineering*. Addison-Wesley.

Brooks, F. P., Jr. (1987). No silver bullet: Essence and accidents of software engineering. *Computer, 20*(4), 10–19.

Buehler, R., Griffin, D., & Ross, M. (1994). Exploring the "planning fallacy": Why people underestimate their task completion times. *Journal of Personality and Social Psychology, 67*(3), 366–381.

Carlini, N. (2026, February). *Building a C compiler with a team of parallel Claudes*. Anthropic Engineering. https://www.anthropic.com/engineering/building-c-compiler

Clark, A., & Chalmers, D. (1998). The extended mind. *Analysis, 58*(1), 7–19.

Conway, M. E. (1968). How do committees invent? *Datamation, 14*(4), 28–31.

Cowan, N. (2001). The magical number 4 in short-term memory: A reconsideration of mental storage capacity. *Behavioral and Brain Sciences, 24*(1), 87–114.

Dalkey, N., & Helmer, O. (1963). An experimental application of the Delphi method to the use of experts. *Management Science, 9*(3), 458–467.

Dasgupta, I., Lampinen, A. K., Chan, S. C. Y., Sheahan, H. R., Creswell, A., Kumaran, D., McClelland, J. L., & Hill, F. (2024). Language models, like humans, show content effects on reasoning tasks. *PNAS Nexus, 3*(7), pgae233.

DeMillo, R. A., Lipton, R. J., & Sayward, F. G. (1978). Hints on test data selection: Help for the practicing programmer. *Computer, 11*(4), 34–41.

Dijkstra, E. W. (1970). *Notes on structured programming* (EWD249). Technological University Eindhoven.

Dijkstra, E. W. (1974). On the role of scientific thought (EWD447). Reprinted in *Selected writings on computing: A personal perspective* (pp. 60–66). Springer, 1982.

Eppinger, S. D., & Browning, T. R. (2012). *Design structure matrix methods and applications*. MIT Press.

Evans, E. (2003). *Domain-driven design: Tackling complexity in the heart of software*. Addison-Wesley.

Fowler, M. (1999). *Refactoring: Improving the design of existing code*. Addison-Wesley.

Gall, H., Hajek, K., & Jazayeri, M. (1998). Detection of logical coupling based on product release history. In *Proceedings of the International Conference on Software Maintenance (ICSM)* (pp. 190–198). IEEE.

Gall, J. (1975). *General systemantics: An essay on how systems work, and especially how they fail*. General Systemantics Press.

Goodhart, C. A. E. (1975). Problems of monetary management: The U.K. experience. In *Papers in monetary economics* (Vol. 1). Reserve Bank of Australia.

Grassé, P.-P. (1959). La reconstruction du nid et les coordinations interindividuelles chez *Bellicositermes natalensis* et *Cubitermes* sp. La théorie de la stigmergie. *Insectes Sociaux, 6*(1), 41–80.

Hagendorff, T., Fabi, S., & Kosinski, M. (2023). Human-like intuitive behavior and reasoning biases emerged in large language models but disappeared in ChatGPT. *Nature Computational Science, 3*(10), 833–838.

Hong, K., Troynikov, A., & Huber, J. (2025). *Context rot: How increasing input tokens impacts LLM performance* [Technical report]. Chroma.

Hutchins, E. (1995). *Cognition in the wild*. MIT Press.

Ingham, A. G., Levinger, G., Graves, J., & Peckham, V. (1974). The Ringelmann effect: Studies of group size and group performance. *Journal of Experimental Social Psychology, 10*(4), 371–384.

Jackson, M. (2001). *Problem frames: Analysing and structuring software development problems*. Addison-Wesley.

Ji, Z., Lee, N., Frieske, R., Yu, T., Su, D., Xu, Y., Ishii, E., Bang, Y. J., Madotto, A., & Fung, P. (2023). Survey of hallucination in natural language generation. *ACM Computing Surveys, 55*(12), Article 248.

Kahn, G. (1974). The semantics of a simple language for parallel programming. In *Information Processing 74: Proceedings of IFIP Congress 74* (pp. 471–475). North-Holland.

Kahneman, D., & Klein, G. (2009). Conditions for intuitive expertise: A failure to disagree. *American Psychologist, 64*(6), 515–526.

Kiczales, G., Lamping, J., Mendhekar, A., Maeda, C., Lopes, C., Loingtier, J.-M., & Irwin, J. (1997). Aspect-oriented programming. In *ECOOP'97: Object-Oriented Programming* (LNCS 1241, pp. 220–242). Springer.

Knight, J. C., & Leveson, N. G. (1986). An experimental evaluation of the assumption of independence in multiversion programming. *IEEE Transactions on Software Engineering, SE-12*(1), 96–109.

Latané, B., Williams, K., & Harkins, S. (1979). Many hands make light the work: The causes and consequences of social loafing. *Journal of Personality and Social Psychology, 37*(6), 822–832.

Lin, W. (2026, January). *Scaling long-running autonomous coding*. Cursor Blog.

Lindsey, J., et al. (2025). On the biology of a large language model. *Transformer Circuits Thread*.

Liu, N. F., Lin, K., Hewitt, J., Paranjape, A., Bevilacqua, M., Petroni, F., & Liang, P. (2024). Lost in the middle: How language models use long contexts. *Transactions of the Association for Computational Linguistics, 12*, 157–173.

Lopopolo, R. (2026, February). *Harness engineering: Leveraging Codex in an agent-first world*. OpenAI. https://openai.com/index/harness-engineering/

McIlroy, M. D., Pinson, E. N., & Tague, B. A. (1978). UNIX time-sharing system: Foreword. *The Bell System Technical Journal, 57*(6), 1899–1904.

Meyer, B. (1992). Applying "design by contract". *Computer, 25*(10), 40–51.

Miller, G. A. (1956). The magical number seven, plus or minus two: Some limits on our capacity for processing information. *Psychological Review, 63*(2), 81–97.

Morrison, J. P. (2010). *Flow-based programming: A new approach to application development* (2nd ed.). CreateSpace. (1st ed. 1994, Van Nostrand Reinhold.)

Nisbett, R. E., & Wilson, T. D. (1977). Telling more than we can know: Verbal reports on mental processes. *Psychological Review, 84*(3), 231–259.

Osmani, A. (2026, April). *Agent harness engineering*. https://addyosmani.com/blog/agent-harness-engineering/

Ousterhout, J. (2018). *A philosophy of software design*. Yaknyam Press.

Page-Jones, M. (1992). Comparing techniques by means of encapsulation and connascence. *Communications of the ACM, 35*(9), 147–151.

Parasuraman, R., & Manzey, D. H. (2010). Complacency and bias in human use of automation: An attentional integration. *Human Factors, 52*(3), 381–410.

Parnas, D. L. (1972). On the criteria to be used in decomposing systems into modules. *Communications of the ACM, 15*(12), 1053–1058.

Parnas, D. L. (1979). Designing software for ease of extension and contraction. *IEEE Transactions on Software Engineering, SE-5*(2), 128–138.

Raymond, E. S. (2003). *The art of Unix programming*. Addison-Wesley.

Reason, J. (1990). *Human error*. Cambridge University Press.

Rice, H. G. (1953). Classes of recursively enumerable sets and their decision problems. *Transactions of the American Mathematical Society, 74*(2), 358–366.

Saltzer, J. H., & Schroeder, M. D. (1975). The protection of information in computer systems. *Proceedings of the IEEE, 63*(9), 1278–1308.

Sangiovanni-Vincentelli, A. (2007). Quo vadis, SLD? Reasoning about the trends and challenges of system level design. *Proceedings of the IEEE, 95*(3), 467–506.

Schluntz, E., & Zhang, B. (2024, December). *Building effective agents*. Anthropic Engineering. https://www.anthropic.com/engineering/building-effective-agents

Sharma, M., et al. (2024). Towards understanding sycophancy in language models. In *Proceedings of the 12th International Conference on Learning Representations (ICLR)*.

Simon, H. A. (1955). A behavioral model of rational choice. *The Quarterly Journal of Economics, 69*(1), 99–118.

Simon, H. A. (1962). The architecture of complexity. *Proceedings of the American Philosophical Society, 106*(6), 467–482.

Skalse, J., Howe, N. H. R., Krasheninnikov, D., & Krueger, D. (2022). Defining and characterizing reward hacking. In *Advances in Neural Information Processing Systems 35 (NeurIPS 2022)*.

Steward, D. V. (1981). The design structure system: A method for managing the design of complex systems. *IEEE Transactions on Engineering Management, EM-28*(3), 71–74.

Suh, N. P. (1990). *The principles of design*. Oxford University Press.

Sumers, T. R., Yao, S., Narasimhan, K., & Griffiths, T. L. (2024). Cognitive architectures for language agents. *Transactions on Machine Learning Research*.

Sweller, J. (1988). Cognitive load during problem solving: Effects on learning. *Cognitive Science, 12*(2), 257–285.

Tversky, A., & Kahneman, D. (1981). The framing of decisions and the psychology of choice. *Science, 211*(4481), 453–458.

Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A. N., Kaiser, Ł., & Polosukhin, I. (2017). Attention is all you need. In *Advances in Neural Information Processing Systems 30 (NIPS 2017)* (pp. 5998–6008).

Wei, J., Wang, X., Schuurmans, D., Bosma, M., Ichter, B., Xia, F., Chi, E., Le, Q. V., & Zhou, D. (2022). Chain-of-thought prompting elicits reasoning in large language models. In *Advances in Neural Information Processing Systems 35 (NeurIPS 2022)*.

Weyuker, E. J. (1982). On testing non-testable programs. *The Computer Journal, 25*(4), 465–470.

Willison, S. (2026, January 19). *Scaling long-running autonomous coding* [Blog post]. https://simonwillison.net/2026/Jan/19/scaling-long-running-autonomous-coding/

Yang, J., Jimenez, C. E., Wettig, A., Lieret, K., Yao, S., Narasimhan, K., & Press, O. (2024). SWE-agent: Agent-computer interfaces enable automated software engineering. In *Advances in Neural Information Processing Systems 37 (NeurIPS 2024)*.

Young, J. (2025, November). *Effective harnesses for long-running agents*. Anthropic Engineering.

Zeller, A., & Hildebrandt, R. (2002). Simplifying and isolating failure-inducing input. *IEEE Transactions on Software Engineering, 28*(2), 183–200.

Zimmermann, T., Weißgerber, P., Diehl, S., & Zeller, A. (2005). Mining version histories to guide software changes. *IEEE Transactions on Software Engineering, 31*(6), 429–445.

## Declaration of generative AI use

This manuscript was drafted with the assistance of a large language model, based on a dialogue in which the author developed the argument. *[Adapt to the policy of the target venue. The author is responsible for the content.]*

---

**Hinweise zum Entwurf**

- **Grundlage:** Der Text folgt deiner Argumentationslinie. Wo du die Gegenseite korrigiert hast, steht deine Position. Das betrifft den Menschen als privilegiertes Orakel, Latenz-Budgets und Ableitungen, den hierarchischen Start, Aspekte als Querschnitt und die These, KI sei dem Menschen nicht nachempfunden. Die verworfenen Varianten erscheinen nur als zurückgewiesene Alternativen, vor allem in §5.4 und §8.2.
- **Ergänzungen von mir:** Folgendes stammt nicht aus dem Gespräch, sondern soll deine Thesen stützen:
  - Conway's Law, Brooks's Law und der Ringelmann-Effekt
  - Distributed und Extended Cognition (Hutchins; Clark & Chalmers)
  - CoALA, Chain-of-Thought, Binz & Schulz, Dasgupta et al., Hagendorff et al., Liu et al.
  - Planning Fallacy, Social Loafing und die Change-Coupling-Literatur
  - §4.6 (Verantwortung als Kante) habe ich aus „jeder Aspekt ist eine Kante" abgeleitet.
  - Die Hypothesen in §10 sind meine Operationalisierung.

  Streiche, was nicht passt.
- **Vor einer Einreichung prüfen:**
  - Die klassischen Referenzen stammen aus meinem Wissen.
  - Die Fallzahlen sind Herstellerangaben aus dem Gespräch. Die Zwischenstufen bei Cursor sind nur über Sekundärquellen belegt.
  - Die Grauliteratur 2025/26 (Young, Lin, Lopopolo, Osmani, Hong et al.) und die URLs bitte gegen die Originale abgleichen.
- **Platzhalter:** Autorenzeile und KI-Erklärung musst du an den Zielort anpassen.

Ich kann den Text als LaTeX setzen (z. B. mit ACM- oder IEEE-Vorlage) oder auf Workshop-Länge von 4–6 Seiten kürzen.
