Ich will eine Engine "resident" auf Basis der These `/home/user/xyan/xy.ai.workbench/docs/rag/resident_streaming.md`. Implementiere folgende Komponenten im Rust Projekt `/home/user/xyan/xy.ai.workbench/rag` auf Basis des Stacks.

- Entsprechende Fehlerbehandlung und Logging ist vorzusehen

## Einstieg und Fassade

Die "Resident"-Engine ist Fassade und Einstiegspunkt.

- Methode zur Konfiguration vor der Initialisierung
- Methode zur Initialisierung
- Methoden zum Queueing der drei Tasks
	- Embedding: Eingabe von Textdaten, Rückgabe von Vektordaten, Parameterobjekt mit Enum für Query/Embedding Unterscheidung
		- Parameterobjekt soll zukünftig weitere Einstellungen erlauben
		- Dies ist auch für Cross-Encoder wenn Query und Chunk gemeinsam gewertet werden sollen
		- Beispiel: Ein Embedding Task ist Konfiguriert das Embedding direkt mit seinen Metadaten in den Index zu schreiben
		- Beispiel: Ein externer Indexlauf speichert viele Embeddings direkt im Index, ein externer Timer persistiert einen Index periodisch in einer Datei oder der Index persistiert sich periodisch selbst oder der Index persistiert sich bei jeder Änderung selbst
		- Beispiel: Ein Indexlauf nutzt gleichzeitig Embedding über GPU und CPU, die Kernel Informaieren und aktualisieren den Index, der Index informiert die Subengines, der Index wird so effektiv zwischen RAM und VRAM synchronisiert. 
	- Similarity/Ähnlichkeitsvergleich: Eingabe Vektordaten, Rückgabe von Index-Eintragsreferenzen oder ID's (Chunk/Zeile/Bereich/Datei/Index) mit Scoring
		- Parameterobjekt soll zukünftig weitere Einstellungen erlauben
	- Inferenz: eher für dinge wie Late-Interaction
	- Ein Task Kapselt Daten und Parameter 
- Alle interne Initialisierung ist Lazy, Modelle und Exekutoren werden erst Lazy bei der ersten Verwendung initialisiert (Semaphore)
- Es gibt einen Satz gemeinsam verwendbarer Normalisierung (Mittels Parameter konfiguriert, immer CPU)
- Es gibt modellspezifische oder geteilte Tokenizer die auf CPU oder GPU laufen
	- Läuft ein GPU/CUDA-Engine-Tokenizer via CPU, wandelt er Token via Mapping Tabelle in die Speicheradressen eines geladenen Modells um die vom Kernel verarbeitet werden (effizienter)
		- Entsprechende Invalidierung der Mappingtabelle beim Unload
		- Kernel: candles index_select erwartet Token-IDs auf dem Device. Ein Gather über CPU-aufgelöste Adressen ist ein eigener cudarc-Kernel, der den Embedding-Layer ersetzt oder vorschaltet.
		- Effizienzabsicht: Der CPU-Tokenizer löst ID → Speicheradresse im residenten Modell auf. Die GPU braucht dadurch keine Mapping-Tabelle im VRAM und keinen Lookup im Kernel, sie liest direkt, und die Indexstruktur bleibt streambar.
- Indices werden der Engine übergeben, "Resident" verwaltet, wann welche Teile eines Index wie in welchen Speicher geladen werden (VRAM/RAM)
	- Ein Index kann eine Struktur Kapseln oder ein mmaped Pointer auf eine Datei sein oder intern bereits Bytebuffer oder Spaltenformate verwenden

## Modelle

- Modelle und Tokenizer werden `safetensors` in kompatiblen Formaten geladen bzw. zum Laden bereitgehalten
- Modelle sind ein fixer Satz und werden vor der Initialisierung Konfiguriert

## **Stack**

**Model-Loading:**
- `safetensors` für ColBERT-Weights laden
- `candle` für Compute (CUDA-Abstraktion, minimal Overhead)
- `tokenizers` crate für BPE/Tokenization

**CUDA/GPU:**
- `candle-core` mit Feature `cuda` für Forward-Passes
- `cudarc` optional für Custom-Kernel (MaxSim)
- GTX 1660 reicht (6GB VRAM für residentes Modell)

**Concurrency/Streaming:**
- `tokio` für Async
- `crossbeam::queue` oder `parking_lot` für Lock-free Ring-Buffer
- Threads für CPU-Tokenizer parallel zu GPU-Compute

**Index:**
- `parquet` für Persistierung
- Columnar Layout (Embeddings pro Column)
- Mmap für Read-Heavy Zugriff

## Zwei Subengines

Es soll zwei Subengines geben:

1. CPU-Engine lädt Modelle und Indices in den RAM bzw. streamt Daten in den RAM, implementiert Kernel selbst
2. GPU/CUDA-Engine, verwendet CPU Resourcen für Normalisierung/Tokenizer, lädt Modelle und Indices in den VRAM bzw. streamt diese, verwaltet und lädt Kernel in die GPU

- Jede Engine kennt verfügbare Resourcen (VRAM/RAM) swapped/streamed Modelle, Kernel und Resourcen entsprechend
- Normalisierung und Tokenizer werden gemeinsam verwendet
- Holen sich per Pull Tasks aus der "Resident"-Engine Queue
	- Ziel ist die selbstorganisation, hat eine Subengine Kapazität frei bedient sie sich an der Queue
	- Die Zuteilung geschieht selbst gesteuert auf Basis der dynamischen Performance der Subengines selbst. Wer schneller ist, holt sich mehr Tasks.
	- Grundsätzlich muss vorgesehen sein das Tasks priorisiert und gefiltert werden können. Beispielweise wird die GPU/CUDA-Engine für Priorisierung von Indizierung konfiguriert, holt sie sich zuerst diese Tasks, vor allen anderen. Ist ein bestimmter Index frei oder schneller als die andere soll sich die Subengine gezielt Tasks holen können.
- Die Subengines verwalten Ihre internen Ressourcen auf optimale Auslastung hin. Bei der CPU-Engine ist dies simpler (Mehr Speicher, alles verwendet ohnehin CPU)
	- Bei der GPU/CUDA Engine komplizierter (Weniger Speicher, mehr Komponenten, CPU Bottleneck). Beispielweise kann Batching notwendig sein um Indices und Kernel effizient zu Swappen und zu Streamen. Preprocessing für Normalisierung und Tokenizer ist CPU-Bound.
	
## Typischer Kontrollfluss

- ein Rust Programm wird gestartet
- "Resident"-Engine wird konfiguriert
- "Resident"-Engine wird initialisiert
	- Der interne Executor startet Anzahl min Idle-Threads
- "Resident"-Engine ist bereit aber verbraucht noch keine Ressourcen
- Indices können hinzugefügt werden, werden aber noch nicht verwendet oder geladen
- Tasks für verschiedene Aufgaben werden eingereiht (Executor)
- Tasks initialisieren die CPU und GPU/CUDA Subengine Lazy
- Ein Task benötigt bestimmte Resourcen und initialisiert diese Lazy
	- Tokenizer und/oder Normalisierung werden initialisiert Lazy
	- Kernel werden kompiliert und geladen Lazy
	- Indices werden geladen, Modelle werden geladen
	- Falls sinnvoll, werden weitere Tasks aggregiert die mit demselben Index oder Modell laufen können oder müssen
- Ergebnisse bereitgehalten und können abgeholt werden (Event/Notification)
	- Die "Resident"-Engine kann in einem Server/Deamon oder via CLI verwendet werden.
- Nach einer bestimmten Zeit werden unbenutzte Ressourcen entladen
	- Entladezeiten für Modelle und Indices auf GPU/VRAM sind wesentlich länger als RAM/CPU

## Klarstellung

- Dateihandling und Indexpersistierung(option) geschehen außerhalb der "Resident"-Engine
	- Ein Index Objekt das mmap für Persistenz und Streaming verwendet besitzt den Mutex selbst  
- Ein Index Objekt kann Änderungen gleichzeitig in RAM/VRAM/DISK abbilden
	- für GPU/CUDA wird die Subengine informiert, das Daten im VRAM aktualisiert werden müssen. Umgekehrt kann ein Kernel den VRAM Index aktualisieren und spiegelt die Änderungen in das Index-Objekt  (bidirektional)
- Normalisierung meint dem zum Tokenizer zugehörigen. Es kann eine weitere externe vorgeschaltete Textnormalisierung geben für die Chunks.
- Worker Threads für die GPU Engine (Auch CPU bound Normalisierung/Tokenizer) haben eine höhere Priorität and CPU-Engine Worker
	
## Ausblick

Weitere Aspekte des Papers werden noch nicht konkret umgesetzt.
Die Umsetzung von gleitender Indizierung ist vorgesehen. In diesem Fall ist der Embedding Input ein entsprechend großer Chunk und die Parameter entsprechend zur Verwendung eines gleitenden Embedding Kernels konfiguriert.
Es sind für (GPU/CUDA) Modi geplant über den ein Kernel mittels Ringbuffer, permanent Embedding und Ähnlichkeitssuche betreibt, die Ergebnisse wiederum in Ringbuffer schreibt, wo sie ausgelesen werden können.

### Ausblick: GPU-Ringbuffer-Modus

- Residenter Kernel mit begrenzter Lebensdauer (`idle_spin`, `max_run_time`), Neustart bei Bedarf. Modul, Gewichte, Index und Ring bleiben im VRAM.
- SPSC-Ring mit zwei monotonen Zeigern `w`/`r`, je ein Schreiber. CAS nur für den Exit: `CAS(w, r, r | EXIT)`.

## Queue Logik für Eingined und Tasks

### Queue

* **Eine** einfach verkettete Liste, nur Append (CAS auf `tail.next`) und Remove (Unlink per CAS auf den Link).
* Reihenfolge = Alter. Keine Version, kein Snapshot, kein Lock.
* Priorität, kompatible Ressource, Modell und Modus sind normale Task-Parameter (Metadata). Die Queue kennt sie nicht.

### Task

```rust
struct Task { params: Params, data: Data, state: Arc<TaskState> }
struct TaskState {
    state: AtomicU8,              // Queued | Taken | Done | Cancelled
    result: OnceCell<Result<Output, Error>>,
    ready: Notify,
}
struct Node {
    params: Params,                       // unveränderlich, zum Filtern vor dem Claim
    state: Arc<TaskState>,                // Handle des Aufrufers, CAS Queued → Taken
    payload: Mutex<Option<Data>>,         // wird beim Claim herausgenommen
    next: ArcSwapOption<Node>,            // Link, Append/Unlink per CAS
}
```

* `claim` = CAS `Queued → Taken`. Das ist die einzige Stelle, an der Cancel relevant ist (scheitert bei `Cancelled`).
* Fertigstellung = CAS `Taken → Done`, `result.set`, `ready.notify_waiters()`. Bei `Cancelled` wird das Ergebnis verworfen.
* `cancel()` = CAS `Queued → Cancelled`, danach `ready.notify_waiters()`. War der Task schon `Taken`: kein Effekt.

* **Besitz:** Queue-Head → Kette → Durchläufer (hält ein `Arc<Node>` pro Schritt). Nach erfolgreichem Claim übernimmt der Claimer `payload`, die Hülle bleibt leer in der Kette, bis sie unlinkt und der letzte Durchläufer sie verlässt. Dann wird sie automatisch freigegeben.
* **Unlink:** reine Aufräumarbeit, einfacher CAS auf `prev.next`. Der entlinkte Node behält sein `next`, ein Durchläufer auf ihm läuft ohne Restart weiter.
* **Drop:** iterativ implementieren (`while let Some(n) = cur.take_next()`), sonst Stack-Überlauf bei langen Ketten.
* **Kein `unsafe`** außerhalb von `arc-swap`.

* Punkt “Speicherfreigabe über `crossbeam-epoch`” entfällt, ersetzt durch `Arc<Node>`.
* Slab, Freiliste und Generationszähler entfallen.
* Alles andere bleibt: eine Liste, Prio und Kompatibilität als Metadata, `claim` als CAS `Queued → Taken`, Notify-Topologie, Lazy Init im Iterate, Eviction über TTL.

### Aufrufer

* Wartet auf `ready` plus periodisches Aufwachen. Reihenfolge: `notified().enable()`, dann `state.load()`, dann Warten.
* Das Timeout ist seine eigene Deadline. `Cancelled` ist lokal abgeleitet.

### Engine (pro Subengine eine Schleife)



rust

```rust
loop {
    notify.notified().await;          // einzige Wartebedingung
    iterate();
}

fn iterate() {
    for node in queue.iter() {
        if all_busy() { break; }
        if !filter.accepts(&node.params) { continue; }   // Prio-Präferenz = Filter/Reihenfolge, Metadata
        let res = resources.entry(node.params.requires());
        match res.state() {
            Uninit => res.init_lazy(),                   // CAS Uninit → Loading, Task bleibt in Queue
            Ready if res.has_free_capacity() && queue.claim(node) => res.run(node, &queue),
            _ => {}                                      // Loading, Busy, Backoff: überspringen
        }
    }
    evict_idle();
}
```

### Ressource

* `run(seed)`: Batch beginnt mit dem Seed, geht ab dem Seed vorwärts durch die Liste, nimmt per `claim` alles, was `accepts` erfüllt und in den Batch passt. Dann `launch`. Packing, Bucketing und Streams sind reine Ressourcenlogik.
* Result → `notify` der Engine.
* Zustände: `Uninit | Loading | Ready | Backoff`.

### Notify-Topologie

| Quelle                           | Wirkung                                       |
| -------------------------------- | --------------------------------------------- |
| Queue-Append                     | Relay auf das Notify jeder Subengine          |
| Result, Init fertig, Evict-Timer | eigenes Notify                                |
| Iteration mit Fortschritt        | eigenes Notify (Loop ist sein eigenes Notify) |

### Regeln

* Keine Reservierung, kein Ahead-Claim, kein Head-of-Line-Blocking.
* Lazy Init nur durch die Iteration, nie durch den Insert.
* Eviction: `in_flight == 0` und TTL abgelaufen (VRAM deutlich länger als RAM).
* Fehler: Init- oder Launch-Fehler schreiben `Err` in die Task-Results des Batchs, die Ressource geht in Backoff, Logging mit `tracing` und Task-ID.
