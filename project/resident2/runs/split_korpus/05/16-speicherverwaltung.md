## Speicherverwaltung

### Arenen

- Der Speicher wird in sechs Arenen (Speicherklassen) nach Lebensdauer eingeteilt:
  - Weights: langlebig, nur beim Unload frei.
  - Index-Segmente: gestreamt, evictable. Die Kapazität umfasst alle gleichzeitig residenten Generationen.
  - Workspace je Lane: kurzlebig.
  - Staging: Pinned Memory im Host für Kopien in beide Richtungen, bei der CPU entfallend.
  - Parameter-Block je Lane: kleine feste Region für dynamische Kernel-Parameter.
  - Mapped Host-Memory: Pinned Memory, den das Device über den Systembus (PCIe) direkt liest.
- Jede Arena hat eine stabile, eindeutige numerische ID, aus der sie rückgewinnbar ist. Unbekannte IDs ergeben „keine Arena“. Die Liste ist vollständig in fester Reihenfolge abrufbar und erweiterbar.
- Je Arena ist eine Kapazität in Byte konfigurierbar.
- Der tatsächliche Device-Bedarf ist die Summe aus Weights, Index-Segmenten, Workspace und Parameter-Block, saturierend statt wrapping. Staging und Mapped Host-Memory zählen nicht, sie liegen im Host-Memory.
- Die Reserve für andere Generationen ist die Index-Kapazität abzüglich des Anteils der aktiven Generation bei gleichmäßiger Aufteilung. Eine Generationszahl unter eins gilt als eins.
- Die Arena wird lazy beim ersten Laden nach Budget angelegt. Alle Adressen sind 64-Bit-Offsets innerhalb der Arena.
- Allokation und Freigabe von VRAM im laufenden Betrieb ist zu vermeiden, weil es das Device implizit synchronisiert.
- Stream-ordered Reuse: Eine Region wird erst frei oder überschrieben, wenn alle Streams, die sie nutzen, ihr Event erreicht haben.
- Kopien laufen über einen eigenen Copy-Stream mit Pinned Staging-Memory, getrennt vom Compute-Stream.

### Budget

- Das Budget besteht aus freiem Speicher, Gesamtspeicher und Generationsreserve. Der nutzbare Anteil ist der freie Speicher abzüglich Reserve, nie negativ.
- Freier und gesamter Device-Speicher werden zur Laufzeit über eine austauschbare Abfrage ermittelt. Scheitern ergibt „Speicherabfrage fehlgeschlagen“ mit Begründung.
- Default-Headroom: 512 MiB für Treiberkontext und andere Prozesse, etwa den Display-Compositor.
- Der Fit-Check stellt fest, ob die geplanten Device-Arenen in den freien Speicher abzüglich des saturierend abgezogenen Headrooms passen, sonst Fehler mit angeforderter und nutzbarer Menge.
- Das Budget plant den kurzen Doppelbedarf durch Copy-on-Write ein.

### Region, Layout und Elementtypen

- Eine Region ist ein pointerfreies Handle aus Arena-ID, Offset, Länge und Generation, kopierbar, vergleichbar und als Key verwendbar. Die Generation entwertet Handles nach Eviction und Neuladen.
- Das Auflösen einer Region auf eine Device-Adresse ist ausschließlich Sache des Memory-Backends. Device-Adressen verlassen das Backend nicht.
- Eigene Elementtypen unabhängig von Fremd-Libraries: FP32 (4 Byte), FP16 (2 Byte), BF16 (2 Byte), u8 (1 Byte), u32 (4 Byte), i64 (8 Byte).
- Elementtypen sind aus dem safetensors-Format übernehmbar, nicht unterstützte ergeben „nicht unterstützter Datentyp“ mit Quelle. Mit der Tensor-Library wird in beide Richtungen konvertiert, ebenso zwischen Elementzahl (Library) und Bytezahl (Engine).
- Für additive Attention-Masks gilt der feste endliche Mask-Wert −10 000. Negativ unendlich ergäbe bei FP16 ungültige Zeilen (NaN).
- Datentypen gelten je Device: Default auf der GPU FP16, auf der CPU FP32. Der Datentyp eines Index ist je Index fest.
- Ein Layout besteht aus Shape, Strides in Byte und Elementtyp. Views ändern nur das Layout, nie die Daten:
  - Contiguous Layout: letzte Dimension mit Elementgröße als Stride, jede vordere als Produkt der hinteren Dimensionen. Dimensionen der Größe null zählen als eins.
  - Abfragbar: Elementzahl, Bytelänge, Contiguity.
  - Transponieren zweier Dimensionen tauscht Shape und Strides. Narrowing ändert nur die Shape der Dimension.

### Free-Space-Verwaltung

- Je Arena vergibt ein Offset-Allocator Bereiche aus fester Kapazität, ohne selbst Speicher zu halten, nach dem First-Fit-Prinzip in aufsteigender Adressreihenfolge.
- Beim Freigeben verschmelzen Lücken mit angrenzenden freien Nachbarn auf beiden Seiten (Coalescing).
- Alignment rundet den Offset auf, null gilt als eins. Overflows führen zu „passt nicht“, nie zum Absturz. Reste vorn und hinten bleiben freie Lücken.
- Anforderungen der Länge null werden nicht erfüllt. Kapazität und freie Menge sind abfragbar. Bei Kapazität null gibt es keine Lücke.

### Bookkeeping und Gültigkeit

- Je Arena verfolgt ein thread-safe Bookkeeping-Allocator Offsets und Generationen, ohne Device-Speicher zu besitzen.
- Erstmals vergebene Kombinationen aus Offset und Länge erhalten Generation null. Erneute Vergabe derselben Kombination erhöht die Generation mit Wrap-around.
- Eine Region ist live, wenn Arena, Offset, Länge und Generation einer aktuellen, nicht freigegebenen Vergabe entsprechen.
- Freigabe scheitert bei fremder Arena, unbekannter, bereits freigegebener oder abweichender Region. Das allgemeine Verwaltungs-Interface dereferenziert stale Handles nie, sondern ignoriert sie mit Warnung.
- Platzmangel nennt angeforderte und freie Menge. Für Tests lässt sich ein Allocation-Fehler je Arena künstlich auslösen (Fault Injection), gemeldet wie Erschöpfung.
- Allokationen und Freigaben werden auf Debug-Level mit Arena, Offset und Größe geloggt.

### Allocator je Arena mit Deferred Free

- Jeder Allocator kennt Kapazität, Arena und Device-Art. Vor jeder Allokation sammelt er abgeschlossene Deferred Frees ein.
- Erschöpfung ergibt einen Fehler mit Arena, angeforderter und freier Menge, andere Fehler „Allokation fehlgeschlagen“.
- Sofortige Freigabe wird bei vorgemerkten oder ungültigen Regionen mit „stale Region“ abgelehnt.
- Deferred Free nimmt Region und Event. Die Region kehrt erst nach Abschluss des Events zurück, sodass laufende Arbeit nie wiederverwendeten Speicher sieht. Abgelehnt bei nicht lebenden oder bereits vorgemerkten Regionen. Nach dem Vormerken wird sofort auf Abschluss geprüft.
- Das Einsammeln (Reaping) gibt abgeschlossene Regionen frei und liefert die Anzahl. Ungültige Regionen erzeugen nur eine Warnung.
- Die sofort verfügbare Menge schließt wartende Regionen aus. Anzahl wartender Regionen und Liveness sind abfragbar. Alles ist thread-safe.
- Ein Allocator-Set stellt je Arena genau einen bereit, liefert die Größenvorgabe und findet den zuständigen Allocator zu einer Region. Fremde IDs ergeben „stale Region“.
- Jedes Memory-Backend erfüllt dieselbe nebenläufige Schnittstelle: Allokation in einer Arena mit Größe und Alignment, Freigabe, Freigabe nach Event, Budget-Abfrage.

### CPU-Backend

- Je Arena ein mit Nullen gefüllter Bereich im RAM, ohne Streams und Staging.
- Schreiben, Lesen und Kopieren auf Regionsbasis mit Prüfung auf Liveness und Länge, sonst „stale Region“. Beim Kopieren gilt die Quelllänge, innerhalb einer Arena überlappungssicher (memmove), zwischen Arenen über einen Zwischenpuffer.
- Budget: Summe der verfügbaren Mengen als frei, Summe der Kapazitäten als gesamt, plus Generationsreserve.

### GPU-Backend

- Nur mit einem Nvidia-Device anlegbar, sonst „Device-Speicher nicht verfügbar“.
- Beim Anlegen zuerst Speicherabfrage und Fit-Check mit Headroom. Bei Misserfolg wird nichts reserviert.
- Freigegebener, stream-ordered allokierter Speicher bleibt im Default-Pool des Treibers, damit im Dauerbetrieb keine Treiber-Allokation mehr stattfindet.
- Je Device-Arena ein zusammenhängender, genullter Block in voller Kapazität. Arenen der Größe null erhalten keinen Block. Danach wird auf Abschluss aller Vorgänge gewartet.
- Für Mapped Host-Memory größer null ein Pinned, genullter Block mit eigener Device-Adresse. Scheitert das Mapping, wird er freigegeben. Beim Shutdown wird er freigegeben, sofern der Device-Context noch bindbar ist.
- Das Allocator-Set setzt die Staging-Kapazität auf null, da diese über den Ring verwaltet wird.
- Nach dem Anlegen werden freier und gesamter Speicher sowie die Arena-Größen in MiB geloggt.
- Kopieren und Transfer:
  - D2D zwischen gültigen Regionen verschiedener Arenen auf einem Stream. Gleiche Arena wird mit Transfer-Error abgelehnt, fehlende Blöcke mit „stale Region“. Locks in fester Reihenfolge verhindern Deadlocks.
  - Asynchroner Upload (H2D) aus Pinned Buffer, der bis zum danach aufgezeichneten Event unverändert bleibt. Asynchroner Download (D2H), lesbar erst nach dem Event. Region live, Datenmenge passend.
  - Treiberfehler: „Device-Speicher nicht verfügbar“ oder Transfer-Error mit Richtung und Begründung.
- Mapped Host-Memory ist ohne Transfer beschreib- und lesbar. Voraussetzung sind existierender Block, passende Arena und lebende Region. Offset und Länge werden overflow-sicher gegen Region und Block geprüft. Die Device-Adresse ist für Kernel-Launches bereitstellbar.
- Ein Ring aus Pinned, genullten Host-Buffern mit Anzahl und Größe ist auf Anforderung erzeugbar.
- Das Budget wird bei jedem Aufruf frisch vom Treiber ermittelt und auf Debug-Level geloggt.

### Staging-Ring

- Feste Anzahl von Slots mit Payload im Zustand frei, belegt oder in Übertragung mit Event. Wiederverwendung erst nach Abschluss des Events.
- Acquire reihum hinter dem zuletzt vergebenen Slot, sonst nichts. Blockierendes Acquire wartet auf die älteste laufende Übertragung. Sind alle Slots belegt ohne Übertragung: „Staging erschöpft“. Scheitert das Warten: Transfer-Error Richtung H2D.
- Payload-Zugriff und die Übergänge „in Übertragung“ und „freigeben“ nur aus „belegt“, sonst „ungültiger Slot“. Anzahl und Slots in Übertragung sind abfragbar. Thread-safe.

### Einbindung von Regionen in Tensoren

- Ein Teilbereich eines Device-Blocks ist in einen Tensor der Tensor-Library einhüllbar (Wrapping), sodass sie den korrekten Bereich liest.
- Der gewrappte Tensor besitzt den Speicher nicht und wird bewusst nicht freigegeben, damit Context und Block intakt bleiben.
- Kopien erhalten eigenen Speicher außerhalb des Blocks. Schreiben schlägt auf den Teilbereich durch, Nachbarelemente bleiben unverändert.
- Diese Prüfungen laufen nur auf einem echten Nvidia-Device.

### Übergreifende Eigenschaften

- Alle gemeinsam genutzten Strukturen sind nebenläufig nutzbar.
- Allocators, Event-Dependencies und Staging sind ohne GPU testbar, etwa mit einem künstlich steuerbaren Event (Mock-Event).

