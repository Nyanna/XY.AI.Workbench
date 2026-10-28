Gleiche die High-Level Konzeption `/home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md` mit der Implementierung in `/home/user/xyan/xy.ai.workbench/rag/src/resident/planner` ab und ergänze `/home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md` damit.
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b503-1
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
  reason: Read concept doc and existing README
```
/answer b503-1 allow
Control Request:
```yaml
id: b503-2
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
  reason: List implementation files
```
/answer b503-2 allow
Control Request:
```yaml
id: b503-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      entries:
      - ".:"
      - README.md
      - adapter.rs
      - config.rs
      - expand.rs
      - mod.rs
      - offers.rs
      - rules.rs
      - "./test:"
      - adapter.rs
      - mod.rs
```
/answer b503-3 allow
Control Request:
```yaml
id: b503-4
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/offers.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/adapter.rs
  reason: Read implementation
```
/answer b503-4 allow
ReasoningToken: 938
Control Request:
```yaml
id: b503-5
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    start: "Tests:"
  reason: Get exact tail for append anchor
```
/answer b503-5 allow
Control Request:
```yaml
id: b503-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
      checksum: 46b2bacb9a331535fed8d0a743ad707a4ad5e8266f92150b21237c34643426a8
      content: |
        Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).
```
/answer b503-6 allow
Control Request:
```yaml
id: b503-7
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: ""
    old_text: "Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge)."
    new_text: |-
      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).

      ## Relation to the design (`project/resident/resident_planner.md`)

      ### Implemented as designed

      - Pure expansion of a task into an ordered chain; the graph is implicit in the order. No queues, workers or notification in the planner.
      - Each operation's `requires` names resource kind, model, engine and (for index/scorer steps) the index.
      - Embedding chain `Tokenize → Embed → Pool → L2Norm → WriteIndex`, with pool, normalise and write optional.
      - Inference chain `Tokenize → Infer(CrossEncoder)`.
      - Structured errors (`PlannerError`: invalid config, missing capability, unknown model, scorer unavailable) and a `tracing` event per expansion.
      - Follow-up operations are produced by a successor of the finished operation. Queueing, the open-operation counter and cancellation belong to the processor, not to this module.

      ### Deviations and decisions beyond the design

      - **Engine choice is made here.** The design leaves the device to the claim. The planner writes `Cpu`, `Gpu` or `Either` into `requires`: GPU preferred, CPU if only it offers the kind, `Either` when the kind is starved and both sides offer it. This settles the design's open item on the starved hint: it is a preference between equivalent variants and never changes the operation set.
      - **Materialisation is stricter.** A result stays on the device only when the operation and its successor both run on the GPU. With `Either`, a CPU producer or the last operation, the result is returned to the host. The design's "GPU successor ⇒ stay" holds only when the producer is also on the GPU.
      - **Rule format** (design: open) is settled as TOML with `default_model` and a `models` table, validated on load (`deny_unknown_fields`, `max_seq_len > 0`, `dim > 0`). Expanders are registered per (model, task kind); `RuleSet::register` overrides them.
      - **Lazy chaining.** The adapter enqueues only the first operation; each finished operation yields the next with the real predecessor output as input.
      - **Query similarity is not one chain.** The design lists `Tokenize → Embed → Similarity → TopK`. Here, similarity takes ready vectors and plans `Similarity → TopK` only; the query embedding is a separate embedding task. The scorer's first resource is `Scorer`.
      - **Model choice:** only inference tasks may name a model; embedding and similarity use the default model.
      - **Validation:** `top_k == 0` is rejected. An embedding task needs an `Encoder` offer even before the tokenizer is chosen, and a target index needs an `Index` offer.

      ### Gaps (design not yet reflected in code)

      - **Pipeline fields are carried but not consumed.** Only `pooling`, `normalize` and `dim` influence the plan. `prefixes`, `max_seq_len`, `dtype` and `scorer` are validated or stored but not forwarded to operation parameters, so they currently have no effect.
      - **Cross-encoder similarity** (query/payload pairs, `Infer(CrossEncoder)`, scope from an earlier result) is rejected with `ScorerUnavailable`.
      - **Sliding-window indexing** (design: outlook) is not implemented.
      - **Priority and task ID** are not part of `PlannedOp`. The task context is attached by the adapter; priority is not set in this module.
      - **Tracing:** a `debug` event with task kind and operation count, not a span per task ID.
      - **Affinity field** in `requires()` (design: extension point) does not exist.
      - **Offers are fixed** when `PlannerExpander` is created; only the rules can be reloaded.
  reason: Append alignment section to README
```
/answer b503-7 deny Das ist eine Beurteilung der Umsetzung und nicht gewünscht
ReasoningToken: 89
Control Request:
```yaml
id: b503-8
toolName: ask_user
arguments:
  question: "Mein Eintrag in die README wurde abgelehnt mit dem Hinweis, er sei eine Beurteilung der Umsetzung und nicht gewünscht. Was soll die README stattdessen enthalten? Zum Beispiel: nur neutrale Fakten zur Implementierung (z. B. Kettenaufbau, Engine-Wahl, TOML-Format, Materialisierungsregel) ohne Abweichungen und Lücken gegenüber dem Konzept, oder etwas anderes?"
  reason: "Die Änderung an der README wurde abgelehnt; ich brauche die gewünschte Ausrichtung, bevor ich einen neuen Versuch mache."
```
/answer b503-8 allow Dein Implemented as designed ist lediglich ein übertrag. Was gibt es mehr dazu zu sagen? Was denkst du gehört in eine README, doch nicht etwa eine Beruteilung der Umsetzung und KOnzeption?!
Text:
 Verstanden. Die README beschreibt, was der Code tut, ohne Bewertung und ohne Abgleich mit dem Konzept. Ich ergänze nur Fakten, die bisher fehlen.
ReasoningToken: 143
Control Request:
```yaml
id: b503-9
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: ""
    old_text: "Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge)."
    new_text: |-
      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).

      ## Details

      ### Operations

      Each planned operation carries a description (kind plus parameters) and a `materialize` flag. The parameters hold the requirements used to filter before a claim: resource kind, model, engine selector (`Cpu`, `Gpu` or `Either`) and, for index and scorer steps, the index. Embed and similarity steps also carry the configured dimension; top-k carries `k`.

      ### Chains per task kind

      | Task | Operations (resource kind) |
      |---|---|
      | Embedding | `Tokenize` (tokenizer), `Embed` (encoder), `Pool` (encoder), `L2Norm` (encoder), `WriteIndex` (index) |
      | Similarity | `Similarity` (scorer), `TopK` (scorer); both require the target index |
      | Inference | `Tokenize` (tokenizer), `Infer` (cross-encoder) |

      `Pool` is left out with pooling `none`, `L2Norm` with `normalize = false`, `WriteIndex` without a target index. The first resource of a task is the tokenizer (embedding, inference) or the scorer (similarity).

      ### Model selection

      Inference tasks use the model they name, otherwise the default model. Embedding and similarity tasks use the default model. A model without a pipeline entry yields `UnknownModel`.

      ### Rules in TOML

      ```toml
      default_model = "m"

      [models.m]
      pooling = "mean"      # mean | cls | none
      normalize = true
      max_seq_len = 512
      dim = 384             # optional
      dtype = "f32"         # f16 | f32
      scorer = "cosine"     # cosine | maxsim

      [models.m.prefixes]
      query = "query: "
      passage = "passage: "
      ```

      All fields are optional and default as shown. Unknown keys are rejected, as are `max_seq_len = 0`, `dim = 0` and a default model without an entry. `RuleSet::register` replaces the expander of a configured model and task kind.

      ### Side choice and materialisation

      - Per kind: GPU only offered → `Gpu`; CPU only → `Cpu`; both → `Gpu`, or `Either` when the starved hint contains the kind; neither → `MissingCapability`.
      - `materialize` is `false` only when an operation and its successor both require `Gpu`. For every other operation, including the last, the result returns to the host.

      ### Adapter

      The adapter plans once per task and enqueues only the first operation. Each finished operation produces its successor with the actual predecessor output as input. The host result of the last operation is converted into the facade's output: embeddings (split per text), hits, a similarity report or inference scores. A result of the wrong type or a missing result is reported as an invalid-config error.

      ### Errors and logging

      `PlannerError` covers invalid config (including `top_k = 0` and an expander receiving the wrong task kind), missing capability, unknown model and unavailable scorer. A similarity task with a cross-encoder scorer fails with the last one. Each expansion logs task kind and operation count at debug level.
  reason: Append neutral implementation details to README as clarified by the user
```
/answer b503-9 deny das ist genausowenig sinnvoll
ReasoningToken: 43
Control Request:
```yaml
id: b503-a
toolName: ask_user
arguments:
  question: "Auch die zweite Fassung (neutrale Detailbeschreibung: Operationen, Ketten, TOML-Format, Seitenwahl, Adapter, Fehler) wurde abgelehnt. Bevor ich noch einmal rate: Was genau soll aus dem Abgleich in die README? Mögliche Richtungen: (a) nur kurze Ergänzungen in den bestehenden Abschnitten (z. B. ein, zwei Sätze), (b) nur Dinge aus dem Konzept, die die README noch nicht nennt (z. B. Operation-Felder, materialize-Regel, Fehlerarten), (c) etwas anderes. Und soll ich vorab einen Abgleichsbericht im Chat geben, statt in die Datei zu schreiben?"
  reason: "Zwei Entwürfe wurden abgelehnt; ich brauche eine konkrete Vorgabe, statt weiter zu raten."
```
/answer b503-a allow Die README ist die aktuelle Zustandsbeschreibung. Das Konzept ist das dahinterliegende Konzept. Das Konzept erklärt und Begründet die Umsetzung in sofern es sich dort überhaupt wiederfinden lässt
ReasoningToken: 166
Control Request:
```yaml
id: b503-b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: ""
    old_text: "Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge)."
    new_text: |-
      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).

      ## Why it is built this way

      The design notes behind the points above, limited to what the code reflects.

      - **Pure function.** The planner only translates task parameters into operation parameters (index, model, resource kind, engine) from rules and engine offers. Scheduling, queues and completion belong to the processor, which keeps the planner testable on its own: same input, same operations.
      - **Parameters, not data.** The plan depends only on task parameters. The data enters the chain as the first operation's input.
      - **Filterable requirements.** Each operation states its needs (resource kind, model, index, engine) up front, so a worker can filter before claiming it. The planner names the resource kind; the engine side decides which device serves it. The selector is `Either` only for a starved kind.
      - **Materialisation is the planner's decision.** It knows the successor, so it decides whether a result returns to the host or stays on the device as a handle. A result stays only between two GPU operations; before a CPU step, at the end of a task or before an index write it must be on the host. The operation just carries out the flag.
      - **Starved hint is a preference.** It only widens the side choice to `Either` for equivalent variants and never adds or drops operations, so it cannot change a task's result.
      - **One engine per kind and side.** Each resource kind exists at most once per side, so requirements identify the engine unambiguously and no handle affinity is needed.
      - **Successors one at a time.** The graph is implied by the order of operations. The next operation is created from the real output of its predecessor, and a failed step never leaves queued operations with missing inputs.
      - **Data and code apart.** Per-model values (prefixes, pooling, normalisation, sequence length, dimension, dtype, scorer) are plain data; the chain structure sits in the expanders. A new model needs only data, a new chain shape needs a registered expander.
      - **Reload by swap.** Rules sit behind an atomic pointer; an expansion works on one snapshot, so a reload never affects a running one.
      - **Structured errors.** Invalid config, missing capability, unknown model and unavailable scorer are separate `PlannerError` variants; the cross-encoder similarity case is reported explicitly instead of being planned wrongly.
  reason: "Append rationale section: README stays current-state description, enriched with concept reasoning that the code reflects"
```
/answer b503-b deny
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 35115, in: 18, out: 7997, reason: 88, read: 154761, write: 27100, start: 1793181556736, end: 1793181566119, time: 9305, total: 66951, io: 0, control: 78, sent: 0, recv: 245
ReasoningToken: 88
Ich habe die README nicht geändert. Meine drei Entwürfe wurden alle abgelehnt, deshalb höre ich auf zu raten.

**Abgleich Konzept ↔ Code**

Folgendes stimmt überein:
- Der Planner ist eine reine Funktion. Er hängt nur von den Task-Parametern ab, nicht von den Daten.
- Die Ketten für Embedding und Inferenz entsprechen dem Konzept.
- Die Fehlerarten sind wie beschrieben, und der Planner nutzt `tracing`.
- Die Folgeoperationen entstehen über einen Nachfolger der fertigen Operation.

Folgendes weicht ab oder ergänzt das Konzept:
- **Engine-Wahl:** Der Planner trägt `Cpu`, `Gpu` oder `Either` in `requires` ein. Das Konzept lässt das Gerät dem Claim. Der Starved-Hint entscheidet nur zwischen gleichwertigen Varianten und ändert die Operationen nicht.
- **`materialize`:** Es ist nur dann `false`, wenn die Operation und ihr Nachfolger beide auf der GPU laufen. Das Konzept sagt „GPU-Nachfolger ⇒ bleibt".
- **Query-Similarity:** Der Code plant nur `Similarity → TopK` auf fertigen Vektoren. Die Query-Einbettung ist ein eigener Task.
- **Regelformat:** TOML mit Registry pro Modell und Tasktyp. Das war im Konzept offen.
- **Modellwahl:** Nur Inferenz-Tasks können ein Modell nennen. Embedding und Similarity nutzen das Default-Modell.
- **Verkettung:** Der Adapter reiht nur die erste Operation ein. Jede fertige Operation erzeugt die nächste.

Folgendes aus dem Konzept fehlt im Code:
- `prefixes`, `max_seq_len`, `dtype` und `scorer` werden gespeichert, aber nicht an die Operationen weitergegeben. Nur `pooling`, `normalize` und `dim` wirken.
- Similarity mit CrossEncoder wird mit `ScorerUnavailable` abgelehnt.
- Gleitende Indizierung fehlt.
- `PlannedOp` hat weder Priorität noch Task-ID.
- Es gibt keinen Span pro Task-ID, nur ein Debug-Event.
- Das Affinitätsfeld in `requires()` fehlt.

Ich habe deine Hinweise so verstanden: Die README beschreibt den Ist-Zustand, und das Konzept liefert dazu Erklärung und Begründung, soweit der Code sie widerspiegelt. Mein dritter Entwurf war ein Abschnitt „Why it is built this way". Er hatte Begründungen zu Pure Function, Parametern statt Daten, Filterbarkeit, `materialize`, Starved-Hint, Nachfolger-Verkettung, Daten und Code getrennt, Reload per Swap und Fehlerarten. Auch den hat der Controller abgelehnt, ohne Begründung.

Wie soll ich weitermachen? Ich kann den Entwurf kürzen, in die bestehenden Abschnitte einarbeiten oder ihn dir zuerst im Chat zeigen. Falls dir eine andere Form vorschwebt, sag sie mir bitte.
In der README Fehlt das Konzept und du sollst das nicht anhängen sondern vollständig Integrieren in dide Darstellung des IST Zustandes. Glaubst du ein Fehlender SPan interessiert irgenjemanden? Der Steht so nichtmal direkt im KOnzept.
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 273
Control Request:
```yaml
id: b503-c
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: ""
    start_marker: "# planner"
    end_marker: for the bridge).
    content: |-
      # planner

      Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification. Scheduling, the counter of open operations and cancellation belong to the processor; the planner only decides what has to happen and with which parameters.

      ## Concept

      - **Inputs:** the task's parameters (never its data, which enters the chain only as the first operation's input), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint. Nothing else influences the plan, which keeps expansion a pure function that can be tested on its own.
      - **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) is plain data, loadable from TOML and validated. The shape of a chain lives in one expander per model × task kind; the registry can be overridden. A new model therefore needs only data, a new chain shape a registered expander. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
      - **Operations.** Each planned operation has a description (kind and parameters) and a `materialize` flag. The parameters hold the requirements a worker filters on before claiming: resource kind, model, engine selector (`Cpu`, `Gpu`, `Either`) and, for index and scorer steps, the index. Embed and similarity steps also carry the dimension, top-k carries `k`. An operation belongs to exactly one task; the graph is implicit in the order.
      - **Chains.**

        | Task | Operations (resource kind) |
        |---|---|
        | Embedding | tokenize (tokenizer) → embed (encoder) → pool (encoder, unless pooling is `none`) → L2-normalise (encoder, if configured) → write to index (index, if a target index is set) |
        | Similarity | score (scorer) → top-k (scorer); both bound to the target index; `top_k` must be positive |
        | Inference | tokenise (tokenizer) → infer (cross-encoder) |

        Similarity works on ready vectors; the query embedding is a separate embedding task. A similarity task with a cross-encoder scorer is currently rejected. A task starts with the tokenizer (embedding, inference) or the scorer (similarity).
      - **Model.** Inference tasks use the model they name, otherwise the default; embedding and similarity use the default. A model without pipeline data is an error.
      - **Side choice.** The planner enters only the resource kind and an engine selector; which device serves it is settled at the claim. GPU is preferred when both sides offer a kind, CPU when only it does, and neither is a missing capability. A starved kind widens the choice to `Either`. The hint is a preference between equivalent variants: it never changes the operation set, so it cannot change a task's result. Each kind exists at most once per side, so the requirements identify the engine unambiguously and no affinity to handles is needed.
      - **Materialisation.** The planner knows the successor, so it decides whether a result returns to the host or stays on the device as a handle; the operation only carries the flag out. A result stays only between two GPU operations. Before a CPU step, at the end of a task or before an index write it is on the host.
      - **Adapter.** The processor-facing adapter plans a task once and enqueues only the first operation. Each finished operation yields the next through its successor, with the real predecessor output as input, so a failed step never leaves queued operations without input. The final host data is converted into the facade's result type (embeddings split per text, hits, similarity report, inference scores).
      - **Errors and logging.** Structured `PlannerError`: invalid config (including a wrongly routed task kind), missing capability, unknown model, unavailable scorer. Each expansion logs task kind and operation count at debug level.

      ## Rules in TOML

      ```toml
      default_model = "m"

      [models.m]
      pooling = "mean"      # mean | cls | none
      normalize = true
      max_seq_len = 512
      dim = 384             # optional
      dtype = "f32"         # f16 | f32
      scorer = "cosine"     # cosine | maxsim

      [models.m.prefixes]
      query = "query: "
      passage = "passage: "
      ```

      All fields are optional and default as shown. Unknown keys, `max_seq_len = 0`, `dim = 0` and a default model without an entry are rejected.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | The planner: rule snapshot, reload, expansion entry |
      | `rules.rs` | Rule set: pipelines per model, expander registry, TOML loading, model selection |
      | `config.rs` | Per-model pipeline data (planner-side types, see root README on name clashes) |
      | `expand.rs` | Task view, planned operation, expander contract and the three standard expanders |
      | `offers.rs` | Which resource kinds each side offers; side choice |
      | `adapter.rs` | Bridge to the processor's expander interface; result conversion |

      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).
  reason: Rewrite README integrating concept rationale into the current-state description
```
/answer b503-c deny warum sollten die TOML rules in diesem Subpackage erwähnt werden, das macht keinen SInn
ReasoningToken: 37
Control Request:
```yaml
id: b503-d
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: ""
    start_marker: "# planner"
    end_marker: for the bridge).
    content: |-
      # planner

      Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification. Scheduling, the counter of open operations and cancellation belong to the processor; the planner only decides what has to happen and with which parameters.

      ## Concept

      - **Inputs:** the task's parameters (never its data, which enters the chain only as the first operation's input), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint. Nothing else influences the plan, which keeps expansion a pure function that can be tested on its own.
      - **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) is plain data, loadable from TOML and validated. The shape of a chain lives in one expander per model × task kind; the registry can be overridden. A new model therefore needs only data, a new chain shape a registered expander. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
      - **Operations.** Each planned operation has a description (kind and parameters) and a `materialize` flag. The parameters hold the requirements a worker filters on before claiming: resource kind, model, engine selector (`Cpu`, `Gpu`, `Either`) and, for index and scorer steps, the index. Embed and similarity steps also carry the dimension, top-k carries `k`. An operation belongs to exactly one task; the graph is implicit in the order.
      - **Chains.**

        | Task | Operations (resource kind) |
        |---|---|
        | Embedding | tokenize (tokenizer) → embed (encoder) → pool (encoder, unless pooling is `none`) → L2-normalise (encoder, if configured) → write to index (index, if a target index is set) |
        | Similarity | score (scorer) → top-k (scorer); both bound to the target index; `top_k` must be positive |
        | Inference | tokenise (tokenizer) → infer (cross-encoder) |

        Similarity works on ready vectors; the query embedding is a separate embedding task. A similarity task with a cross-encoder scorer is currently rejected. A task starts with the tokenizer (embedding, inference) or the scorer (similarity).
      - **Model.** Inference tasks use the model they name, otherwise the default; embedding and similarity use the default. A model without pipeline data is an error.
      - **Side choice.** The planner enters only the resource kind and an engine selector; which device serves it is settled at the claim. GPU is preferred when both sides offer a kind, CPU when only it does, and neither is a missing capability. A starved kind widens the choice to `Either`. The hint is a preference between equivalent variants: it never changes the operation set, so it cannot change a task's result. Each kind exists at most once per side, so the requirements identify the engine unambiguously and no affinity to handles is needed.
      - **Materialisation.** The planner knows the successor, so it decides whether a result returns to the host or stays on the device as a handle; the operation only carries the flag out. A result stays only between two GPU operations. Before a CPU step, at the end of a task or before an index write it is on the host.
      - **Adapter.** The processor-facing adapter plans a task once and enqueues only the first operation. Each finished operation yields the next through its successor, with the real predecessor output as input, so a failed step never leaves queued operations without input. The final host data is converted into the facade's result type (embeddings split per text, hits, similarity report, inference scores).
      - **Errors and logging.** Structured `PlannerError`: invalid config (including a wrongly routed task kind), missing capability, unknown model, unavailable scorer. Each expansion logs task kind and operation count at debug level.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | The planner: rule snapshot, reload, expansion entry |
      | `rules.rs` | Rule set: pipelines per model, expander registry, TOML loading, model selection |
      | `config.rs` | Per-model pipeline data (planner-side types, see root README on name clashes) |
      | `expand.rs` | Task view, planned operation, expander contract and the three standard expanders |
      | `offers.rs` | Which resource kinds each side offers; side choice |
      | `adapter.rs` | Bridge to the processor's expander interface; result conversion |

      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).
  reason: "Rewrite README integrating concept into current-state description, without TOML example section"
```
/answer b503-d allow
Control Request:
```yaml
id: b503-e
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
      error: No node matched the selector.
      id: ""
```
/answer b503-e allow
ReasoningToken: 14
Control Request:
```yaml
id: b503-f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
  reason: Get node IDs of README sections
```
/answer b503-f allow
Control Request:
```yaml
id: b503-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
      nodes:
      - id: 229eb4
        signature: planner
        children:
        - id: 229eb4.aPeTak|SEmthf
          signature: Stateless expansion of a task…
        - id: 229eb4.c05baf
          signature: Concept
        - id: 229eb4.6ce6c5
          signature: Files
          children:
          - id: 229eb4.6ce6c5.bPzwGg|LTSHgI
            signature: "Tests: `test/` (`mod.rs` for …"
```
/answer b503-10 allow
ReasoningToken: 54
Control Request:
```yaml
id: b503-11
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: 229eb4.aPeTak|SEmthf
    source: "Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification. Scheduling, the counter of open operations and cancellation belong to the processor; the planner only decides what has to happen and with which parameters."
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    id: 229eb4.c05baf
    source: |-
      ## Concept

      - **Inputs:** the task's parameters (never its data, which enters the chain only as the first operation's input), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint. Nothing else influences the plan, which keeps expansion a pure function that can be tested on its own.
      - **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) is plain data, loadable from TOML and validated. The shape of a chain lives in one expander per model × task kind; the registry can be overridden. A new model therefore needs only data, a new chain shape a registered expander. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
      - **Operations.** Each planned operation has a description (kind and parameters) and a `materialize` flag. The parameters hold the requirements a worker filters on before claiming: resource kind, model, engine selector (`Cpu`, `Gpu`, `Either`) and, for index and scorer steps, the index. Embed and similarity steps also carry the dimension, top-k carries `k`. An operation belongs to exactly one task; the graph is implicit in the order.
      - **Chains.**

        | Task | Operations (resource kind) |
        |---|---|
        | Embedding | tokenize (tokenizer) → embed (encoder) → pool (encoder, unless pooling is `none`) → L2-normalise (encoder, if configured) → write to index (index, if a target index is set) |
        | Similarity | score (scorer) → top-k (scorer); both bound to the target index; `top_k` must be positive |
        | Inference | tokenise (tokenizer) → infer (cross-encoder) |

        Similarity works on ready vectors; the query embedding is a separate embedding task. A similarity task with a cross-encoder scorer is currently rejected. A task starts with the tokenizer (embedding, inference) or the scorer (similarity).
      - **Model.** Inference tasks use the model they name, otherwise the default; embedding and similarity use the default. A model without pipeline data is an error.
      - **Side choice.** The planner enters only the resource kind and an engine selector; which device serves it is settled at the claim. GPU is preferred when both sides offer a kind, CPU when only it does, and neither is a missing capability. A starved kind widens the choice to `Either`. The hint is a preference between equivalent variants: it never changes the operation set, so it cannot change a task's result. Each kind exists at most once per side, so the requirements identify the engine unambiguously and no affinity to handles is needed.
      - **Materialisation.** The planner knows the successor, so it decides whether a result returns to the host or stays on the device as a handle; the operation only carries the flag out. A result stays only between two GPU operations. Before a CPU step, at the end of a task or before an index write it is on the host.
      - **Adapter.** The processor-facing adapter plans a task once and enqueues only the first operation. Each finished operation yields the next through its successor, with the real predecessor output as input, so a failed step never leaves queued operations without input. The final host data is converted into the facade's result type (embeddings split per text, hits, similarity report, inference scores).
      - **Errors and logging.** Structured `PlannerError`: invalid config (including a wrongly routed task kind), missing capability, unknown model, unavailable scorer. Each expansion logs task kind and operation count at debug level.
  reason: Replace intro and Concept section with integrated concept + current state
```
/answer b503-11 allow
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 49111, in: 28, out: 14184, reason: 0, read: 307105, write: 34899, start: 1793181920194, end: 1793181927711, time: 7437, total: 109067, io: 4, control: 80, sent: 0, recv: 120
Ich habe die README überarbeitet. Das Konzept steht jetzt im Abschnitt „Concept“, und es ist kein Anhang mehr. Der Abschnitt beschreibt den Ist-Zustand und nennt dazu die Begründungen aus dem Konzept.

- **Eingearbeitet:** reine Funktion, Eingaben ohne Daten, Daten und Code getrennt, filterbare `requires`, Seitenwahl durch den Claim, `materialize` als Entscheidung des Planners, Starved-Hint als Präferenz und Nachfolger-Verkettung.
- **Ist-Zustand:** Ketten, Modellwahl, Fehlerarten und Adapter beschreiben das, was der Code tut.
- **Weggelassen:** Lücken- und Abweichungslisten, also auch der fehlende Span. Das TOML-Beispiel habe ich ebenfalls nicht übernommen. TOML erwähne ich nur kurz als Ladeweg. Die Dateitabelle ist unverändert.

Ich habe die Datei nach dem Schreiben nicht noch einmal gelesen.

Eine Stelle ist bewusst nüchtern formuliert: Der Satz „Similarity mit CrossEncoder wird derzeit abgelehnt“ steht weiter drin, weil er sonst fehlt, wenn jemand diesen Scorer ausprobiert.
Jetzt schreibe in eine separate Datei deine festgestellten Abweichungen und ofennen Punkte, aber nur jene die konkret umgesetzt werden sollten und nicht perspektivische
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 344
Control Request:
```yaml
id: b503-12
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_abweichungen.md
    source: |
      # Resident – Planner: Abweichungen und offene Punkte

      Abgleich von `resident_planner.md` mit `rag/src/resident/planner`. Nur Punkte, die konkret umzusetzen sind.

      ## 1. Pipeline-Config wirkt nicht auf die Operationen

      `PipelineConfig` enthält `prefixes`, `max_seq_len`, `dtype` und `scorer`. Sie werden geladen und validiert, aber nicht in die Operation-Parameter (`OpParams`) übernommen. Im Plan wirken nur `pooling`, `normalize` und `dim`.

      Umzusetzen:
      - `OpParams` um die Felder erweitern, die die Operationen brauchen: Präfix je Textrolle, `max_seq_len` (Tokenize), `dtype` (Embed), `scorer` (Similarity).
      - Die Expander tragen sie aus `ctx.pipeline` ein.
      - Tests: Plan für ein Modell mit abweichender Config enthält die Werte.

      ## 2. Similarity mit CrossEncoder

      Das Konzept sieht eine Expansionsregel vor: Paare (Query, Text aus Payload) → `Infer(CrossEncoder)` → Scores, Scope aus früherem Ergebnis. `SimilarityExpander` lehnt `ScorerKind::CrossEncoder` derzeit mit `ScorerUnavailable` ab.

      Umzusetzen:
      - Kette für den CrossEncoder-Scorer im `SimilarityExpander`.
      - Klären, wie Paare und Scope in den Input der ersten Operation kommen (`initial_input` im Adapter liefert bisher nur Vektoren).

      ## 3. Priorität und Task-ID an der geplanten Operation

      Das Konzept nennt beides als Metadaten der Operation. `PlannedOp` enthält nur `desc` und `materialize`. Die Task-Zuordnung entsteht erst im Adapter über den `TaskCtx`.

      Umzusetzen:
      - Klären, wo die Priorität gesetzt wird (Planner oder Processor) und ob `PlannedOp` sie tragen muss.
      - Entscheidung dokumentieren; falls der Processor sie setzt, das Konzept anpassen.

      ## 4. Konzept an den Ist-Zustand anpassen

      Diese Punkte sind im Code bewusst anders gelöst. Das Konzept sollte nachgezogen werden:
      - **Engine-Wahl:** Der Planner schreibt `Cpu`/`Gpu`/`Either` in `requires`; das Konzept lässt nur die Ressourcenart eintragen. Der Starved-Hint ist damit umgesetzt (Konzept: offen).
      - **`materialize`:** `false` nur, wenn Operation und Nachfolger beide auf der GPU laufen. Das Konzept sagt „GPU-Nachfolger ⇒ false“.
      - **Konfigurationsformat:** TOML mit Registry pro (Modell, Tasktyp) ist umgesetzt (Konzept: offen).
      - **Query-Similarity:** Die Kette ist `Similarity → TopK` auf fertigen Vektoren; die Query-Einbettung ist ein eigener Task. Das Konzept zeigt `Tokenize → Embed → Similarity → TopK`.
      - **Modellwahl:** Nur Inferenz-Tasks können ein Modell nennen; Embedding und Similarity nutzen das Default-Modell.
  reason: Create separate file with concrete deviations and open points
```
/answer b503-12 deny in englisch und zwar für agentische umsetzung und verarbeitung
ReasoningToken: 51
Control Request:
```yaml
id: b503-13
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
    source: |
      # Resident Planner: deviations and open work items

      Source of truth for intent: `project/resident/resident_planner.md`.
      Implementation: `rag/src/resident/planner/` (`expand.rs`, `rules.rs`, `config.rs`, `offers.rs`, `adapter.rs`, tests in `test/`).
      Scope: only items to be implemented. Each item has a goal, the code locations, the required change and acceptance criteria.

      ## T1. Wire pipeline config into operation parameters

      - **Problem:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`. They are loaded and validated, but no expander copies them into `OpParams`. Only `pooling`, `normalize` and `dim` influence the plan (`expand.rs`).
      - **Change:**
        - Extend `OpParams` (module `resident::op`) with the fields the operations need: text prefixes per `TextRole`, `max_seq_len` for `Tokenize`, `dtype` for `Embed`, `scorer` for `Similarity`.
        - In `EmbeddingExpander`, `SimilarityExpander` and `InferenceExpander` fill them from `ctx.pipeline`.
        - Check the consumers (tokenizer, encoder, scorer engines) read the new fields, or note which consumer still has to.
      - **Acceptance:**
        - A planner test with a non-default pipeline asserts the values appear on the right operations.
        - Defaults produce the same plan as before apart from the new fields.
        - `cargo test` for the crate passes.

      ## T2. Similarity with cross-encoder scorer

      - **Problem:** the design specifies: pairs (query, payload text) → `Infer(CrossEncoder)` → scores, scope taken from an earlier result. `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`.
      - **Change:**
        - Add the cross-encoder branch in `SimilarityExpander`: `Tokenize` → `Infer` with `ResourceKind::CrossEncoder`, requirements bound to the model of the scorer.
        - Decide and implement how pairs and scope reach the first operation. `initial_input` in `adapter.rs` currently turns a similarity payload into one flat `F32` vector list only.
        - Extend `into_output` for the result type of this branch.
      - **Acceptance:**
        - Planner test: cross-encoder similarity yields the expected chain and engine selectors.
        - Adapter test: end-to-end conversion of the result into the facade output.
        - The `ScorerUnavailable` test is replaced or limited to scorers that really are unavailable.

      ## T3. Priority and task ID on planned operations

      - **Problem:** the design lists priority and task ID as operation metadata. `PlannedOp` (`expand.rs`) carries only `desc` and `materialize`; the task link is attached in `adapter.rs` via `TaskCtx`; priority is not set anywhere in the planner module.
      - **Change:**
        - Find where the processor reads priority for an `OperationNode`.
        - Either set it from the task metadata in `Chain::node` (adapter), or add it to `PlannedOp` if the planner is meant to decide it.
        - Record the decision in `resident_planner.md`.
      - **Acceptance:** every `OperationNode` created by `PlannerExpander` has the task's priority and ID; covered by an adapter test.

      ## T4. Update the design document to the implemented behaviour

      Edit `resident_planner.md` so it matches the code; do not change the code for these.

      - **Engine selector:** the planner writes `Cpu`/`Gpu`/`Either` into `requires` (GPU preferred; `Either` when the kind is starved and both sides offer it). The design says the planner enters only the resource kind. The "Offen" entry on the starved hint can be closed: it is a preference between equivalent variants and never changes the operation set.
      - **`materialize`:** `false` only when the operation and its successor both require `Gpu`; otherwise `true`. Replace the rule "GPU successor ⇒ false".
      - **Rule format:** TOML (`default_model`, `models.<name>`) plus an expander registry keyed by (model, task kind). Close the "Offen" entry on the configuration format.
      - **Query similarity:** the chain is `Similarity` → `TopK` on ready vectors; the query embedding is a separate embedding task. Replace `Tokenize → Embed → Similarity → TopK` or state that the query embedding is a preceding task.
      - **Model choice:** only inference tasks may name a model; embedding and similarity use the default model.
      - **Acceptance:** each statement above is consistent with the code; open items that are now settled are removed from the "Offen" section.
  reason: "Create separate English, agent-oriented work item file with concrete deviations only"
```
/answer b503-13 deny Ich verstehe deine Absicht nicht, das `resident_planner.md` wird nciht geändert und das steht auch nciht zur debatte
ReasoningToken: 255
Control Request:
```yaml
id: b503-14
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
    source: |
      # Resident Planner: deviations and open work items

      Source of truth for intent: `project/resident/resident_planner.md` (not to be changed).
      Implementation: `rag/src/resident/planner/` (`expand.rs`, `rules.rs`, `config.rs`, `offers.rs`, `adapter.rs`, tests in `test/`).
      Scope: only items to be implemented in the code. Each item has the problem, the change and acceptance criteria.

      ## T1. Wire pipeline config into operation parameters

      - **Problem:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`. They are loaded and validated, but no expander copies them into `OpParams`. Only `pooling`, `normalize` and `dim` influence the plan (`expand.rs`). The design requires the planner to translate task parameters into concrete operation parameters (tokenizer, model, scorer, pooling, prefix).
      - **Change:**
        - Extend `OpParams` (module `resident::op`) with the fields the operations need: text prefix per `TextRole`, `max_seq_len` for `Tokenize`, `dtype` for `Embed`, `scorer` for `Similarity`.
        - In `EmbeddingExpander`, `SimilarityExpander` and `InferenceExpander`, fill them from `ctx.pipeline`.
        - Check that the consuming engines (tokenizer, encoder, scorer) read the new fields; list any consumer that still has to be changed.
      - **Acceptance:**
        - A planner test with a non-default pipeline asserts the values on the right operations.
        - Default pipelines produce the same plan as before, apart from the new fields.
        - The crate's tests pass.

      ## T2. Similarity with cross-encoder scorer

      - **Problem:** the design specifies: pairs (query, text from payload) → `Infer(CrossEncoder)` → scores, scope from an earlier result. `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`.
      - **Change:**
        - Add the cross-encoder branch in `SimilarityExpander`: `Tokenize` → `Infer` with `ResourceKind::CrossEncoder`, requirements bound to the scorer's model.
        - Decide and implement how pairs and scope reach the first operation. `initial_input` in `adapter.rs` currently turns a similarity payload into one flat `F32` vector list only.
        - Extend `into_output` for the result type of this branch.
      - **Acceptance:**
        - Planner test: cross-encoder similarity yields the expected chain and engine selectors.
        - Adapter test: the result converts end to end into the facade output.
        - The existing `ScorerUnavailable` test is replaced or limited to scorers that really are unavailable.

      ## T3. Priority and task ID on planned operations

      - **Problem:** the design lists priority and task ID as operation metadata. `PlannedOp` (`expand.rs`) carries only `desc` and `materialize`. The task link is attached in `adapter.rs` via `TaskCtx`; priority is not set anywhere in the planner module.
      - **Change:**
        - Find where the processor reads priority for an `OperationNode`.
        - Set it from the task metadata in `Chain::node` (adapter), or add it to `PlannedOp` if the planner is to decide it.
      - **Acceptance:** every `OperationNode` created by `PlannerExpander` carries the task's priority and ID; covered by an adapter test.
  reason: "Create the work item file in English with only code changes (T1-T3), design doc untouched"
```
/answer b503-14 deny Dein "Source of truth for intent" ist kein offener Punkt, schmeiß den ganzen Absatz raus, er ist nutzlos
Control Request:
```yaml
id: b503-15
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
    source: |
      # Resident Planner: open work items

      ## T1. Wire pipeline config into operation parameters

      - **Problem:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`. They are loaded and validated, but no expander copies them into `OpParams`. Only `pooling`, `normalize` and `dim` influence the plan (`expand.rs`). The design requires the planner to translate task parameters into concrete operation parameters (tokenizer, model, scorer, pooling, prefix).
      - **Change:**
        - Extend `OpParams` (module `resident::op`) with the fields the operations need: text prefix per `TextRole`, `max_seq_len` for `Tokenize`, `dtype` for `Embed`, `scorer` for `Similarity`.
        - In `EmbeddingExpander`, `SimilarityExpander` and `InferenceExpander`, fill them from `ctx.pipeline`.
        - Check that the consuming engines (tokenizer, encoder, scorer) read the new fields; list any consumer that still has to be changed.
      - **Acceptance:**
        - A planner test with a non-default pipeline asserts the values on the right operations.
        - Default pipelines produce the same plan as before, apart from the new fields.
        - The crate's tests pass.

      ## T2. Similarity with cross-encoder scorer

      - **Problem:** the design specifies: pairs (query, text from payload) → `Infer(CrossEncoder)` → scores, scope from an earlier result. `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`.
      - **Change:**
        - Add the cross-encoder branch in `SimilarityExpander`: `Tokenize` → `Infer` with `ResourceKind::CrossEncoder`, requirements bound to the scorer's model.
        - Decide and implement how pairs and scope reach the first operation. `initial_input` in `adapter.rs` currently turns a similarity payload into one flat `F32` vector list only.
        - Extend `into_output` for the result type of this branch.
      - **Acceptance:**
        - Planner test: cross-encoder similarity yields the expected chain and engine selectors.
        - Adapter test: the result converts end to end into the facade output.
        - The existing `ScorerUnavailable` test is replaced or limited to scorers that really are unavailable.

      ## T3. Priority and task ID on planned operations

      - **Problem:** the design lists priority and task ID as operation metadata. `PlannedOp` (`expand.rs`) carries only `desc` and `materialize`. The task link is attached in `adapter.rs` via `TaskCtx`; priority is not set anywhere in the planner module.
      - **Change:**
        - Find where the processor reads priority for an `OperationNode`.
        - Set it from the task metadata in `Chain::node` (adapter), or add it to `PlannedOp` if the planner is to decide it.
      - **Acceptance:** every `OperationNode` created by `PlannerExpander` carries the task's priority and ID; covered by an adapter test.
  reason: Create the work item file without the header paragraph
```
/answer b503-15 deny das sind keine open work items
ReasoningToken: 22
Control Request:
```yaml
id: b503-16
toolName: ask_user
arguments:
  question: "Du hast T1–T3 (Pipeline-Config nicht in OpParams, CrossEncoder-Similarity, Priorität/Task-ID an PlannedOp) als „keine open work items\" abgelehnt. Welche Punkte aus dem Abgleich sollen dann in die Datei? Die einzigen weiteren Abweichungen, die ich gefunden habe, sind: Engine-Selector (Cpu/Gpu/Either) im Planner statt nur Ressourcenart, materialize nur bei GPU→GPU, Query-Similarity ohne Tokenize/Embed in der Kette und Modellwahl nur bei Inferenz. Welche davon (oder welche anderen) willst du als Umsetzungspunkte, und sollen sie als Codeänderung formuliert sein?"
  reason: "Die Datei wurde dreimal abgelehnt; ich muss klären, welche Punkte gemeint sind."
```
/answer b503-16 allow Abweichungen oder nicht abgebildete Aspekte aus dem Design
ReasoningToken: 240
Control Request:
```yaml
id: b503-17
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
    source: |
      # Resident Planner: deviations from the design

      Deviations of `rag/src/resident/planner/` from `project/resident/resident_planner.md`, and design aspects not yet mapped in the code. Outlook items are excluded.

      ## D1. Pipeline config is not mapped to operation parameters

      - **Design:** the planner translates task parameters into concrete operation parameters: tokenizer, model, scorer, pooling, prefix. Pipeline config per model: prefixes, pooling, normalisation, `max_seq_len`, dtype.
      - **Code:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`; they are loaded and validated, but no expander copies them into `OpParams` (`expand.rs`). Only `pooling`, `normalize` and `dim` affect the plan.
      - **Required:** extend `OpParams` (`resident::op`) with prefix per `TextRole`, `max_seq_len` (Tokenize), `dtype` (Embed), `scorer` (Similarity); fill them from `ctx.pipeline` in all three expanders; check that the consuming engines read them.
      - **Check:** planner test with a non-default pipeline asserts the values on the right operations; default pipelines plan as before.

      ## D2. Similarity with CrossEncoder is not expanded

      - **Design:** pairs (query, text from payload) → `Infer(CrossEncoder)` → scores; scope from an earlier result.
      - **Code:** `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`. `initial_input` (`adapter.rs`) builds only a flat `F32` vector list for similarity payloads; `into_output` has no result type for this branch.
      - **Required:** cross-encoder chain in `SimilarityExpander`; input of pairs and scope in the adapter; result conversion in `into_output`.
      - **Check:** planner test for the chain and engine selectors; adapter test for the end-to-end result; the `ScorerUnavailable` test is limited to scorers that really are unavailable.

      ## D3. Priority and task ID are not part of the planned operation

      - **Design:** the operation carries priority and task ID as metadata.
      - **Code:** `PlannedOp` has only `desc` and `materialize`. The task link comes from `TaskCtx` in `Chain::node` (`adapter.rs`); priority is set nowhere in the planner module.
      - **Required:** determine how the processor takes priority for an `OperationNode`; set it from the task metadata in `Chain::node`, or add it to `PlannedOp`.
      - **Check:** every `OperationNode` created by `PlannerExpander` carries the task's priority and ID; adapter test.

      ## D4. Query similarity chain

      - **Design:** `Tokenize → Embed → Similarity(Scope) → TopK`.
      - **Code:** `SimilarityExpander` plans `Similarity → TopK` on ready vectors (`initial_input` flattens `input.vectors()`); the query embedding is a separate embedding task.
      - **Required:** decide whether the planner expands the query embedding into the similarity chain; if yes, add `Tokenize` and `Embed` in front and take the query text as input.

      ## D5. `materialize` rule

      - **Design:** `true` if the successor runs on the CPU, the task ends or the result is mirrored into the index; `false` if a GPU successor follows.
      - **Code:** `Chain::finish` (`expand.rs`) sets `false` only if the operation and its successor both require `Gpu`. With a CPU or `Either` producer and a GPU successor the result is materialised.
      - **Required:** decide whether a result of an `Either` or CPU operation before a GPU successor must stay a device handle; adapt `Chain::finish` and its tests accordingly.

      ## D6. Engine selection in `requires()`

      - **Design:** the planner enters only the resource kind; the claim of the subengine decides the device. The starved hint is a preference.
      - **Code:** `EngineOffers::choose` (`offers.rs`) writes `Cpu`, `Gpu` or `Either` into `requires()` (GPU preferred, `Either` for a starved kind offered on both sides).
      - **Required:** decide whether a fixed `Cpu` or `Gpu` entry by the planner is intended; if not, emit the kind only and let the claim decide.

      ## D7. Model choice per task kind

      - **Design:** the pipeline config is selected per model; the registry entry is per model and task type.
      - **Code:** `RuleSet::expand` (`rules.rs`) takes a model name only from inference parameters; embedding and similarity always use the default model.
      - **Required:** decide whether embedding and similarity parameters may name a model, and read it in `RuleSet::expand`.
  reason: "Create deviations file, framed as design deviations/unmapped aspects, no outlook items"
```
/answer b503-17 deny entferne deine Lösungsvorschläge, du hast keine Ahnung vom Kontext und kannst die Lösungen nicht bewerten
ReasoningToken: 116
Control Request:
```yaml
id: b503-18
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
    source: |
      # Resident Planner: deviations from the design

      Deviations of `rag/src/resident/planner/` from `project/resident/resident_planner.md`, and design aspects not mapped in the code. Outlook items are excluded.

      ## D1. Pipeline config is not mapped to operation parameters

      - **Design:** the planner translates task parameters into concrete operation parameters: tokenizer, model, scorer, pooling, prefix. Pipeline config per model: prefixes, pooling, normalisation, `max_seq_len`, dtype.
      - **Code:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`; they are loaded and validated, but no expander copies them into `OpParams` (`expand.rs`). Only `pooling`, `normalize` and `dim` affect the plan.

      ## D2. Similarity with CrossEncoder is not expanded

      - **Design:** pairs (query, text from payload) → `Infer(CrossEncoder)` → scores; scope from an earlier result.
      - **Code:** `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`. `initial_input` (`adapter.rs`) builds only a flat `F32` vector list for similarity payloads; `into_output` has no result type for this case.

      ## D3. Priority and task ID are not part of the planned operation

      - **Design:** the operation carries priority and task ID as metadata.
      - **Code:** `PlannedOp` has only `desc` and `materialize`. The task link comes from `TaskCtx` in `Chain::node` (`adapter.rs`); priority is set nowhere in the planner module.

      ## D4. Query similarity chain

      - **Design:** `Tokenize → Embed → Similarity(Scope) → TopK`.
      - **Code:** `SimilarityExpander` plans `Similarity → TopK` on ready vectors (`initial_input` flattens `input.vectors()`); the query embedding is a separate embedding task.

      ## D5. `materialize` rule

      - **Design:** `true` if the successor runs on the CPU, the task ends or the result is mirrored into the index; `false` if a GPU successor follows.
      - **Code:** `Chain::finish` (`expand.rs`) sets `false` only if the operation and its successor both require `Gpu`. With a CPU or `Either` producer and a GPU successor the result is materialised.

      ## D6. Engine selection in `requires()`

      - **Design:** the planner enters only the resource kind; the claim of the subengine decides the device. The starved hint is a preference.
      - **Code:** `EngineOffers::choose` (`offers.rs`) writes `Cpu`, `Gpu` or `Either` into `requires()` (GPU preferred, `Either` for a starved kind offered on both sides).
  reason: "Create deviations file with design vs code only, no solution proposals"
```
/answer b503-18 allow
Control Request:
```yaml
id: b503-19
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner_todo.md
      error: File not found.
```
/answer b503-19 allow
ReasoningToken: 67
Control Request:
```yaml
id: b503-1a
toolName: ask_user
arguments:
  question: "Der Inhalt ist jetzt freigegeben (Design vs. Code, ohne Lösungsvorschläge), aber meine Werkzeuge können keine neue Datei anlegen: `resident_planner_todo.md` existiert nicht. Bitte lege eine leere Datei an, zum Beispiel `/home/user/xyan/xy.ai.workbench/project/resident/resident_planner_deviations.md`, und nenne mir den Pfad. Dann schreibe ich den Inhalt hinein."
  reason: ast_insert scheitert mit "File not found"; ohne bestehende Datei kann ich nichts schreiben.
```
/answer b503-1a allow gib sie hier aus
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 73115, in: 46, out: 26042, reason: 0, read: 673012, write: 47027, start: 1793182471793, end: 1793182478367, time: 6492, total: 201603, io: 0, control: 82, sent: 0, recv: 149
# Resident Planner: deviations from the design

Deviations of `rag/src/resident/planner/` from `project/resident/resident_planner.md`, and design aspects not mapped in the code. Outlook items are excluded.

## D1. Pipeline config is not mapped to operation parameters

- **Design:** the planner translates task parameters into concrete operation parameters: tokenizer, model, scorer, pooling, prefix. Pipeline config per model: prefixes, pooling, normalisation, `max_seq_len`, dtype.
- **Code:** `PipelineConfig` (`config.rs`) holds `prefixes`, `max_seq_len`, `dtype`, `scorer`. They are loaded and validated, but no expander copies them into `OpParams` (`expand.rs`). Only `pooling`, `normalize` and `dim` affect the plan.

## D2. Similarity with CrossEncoder is not expanded

- **Design:** pairs (query, text from payload) → `Infer(CrossEncoder)` → scores; scope from an earlier result.
- **Code:** `SimilarityExpander` returns `PlannerError::ScorerUnavailable` for `ScorerKind::CrossEncoder(_)`. `initial_input` (`adapter.rs`) builds only a flat `F32` vector list for similarity payloads. `into_output` has no result type for this case.

## D3. Priority and task ID are not part of the planned operation

- **Design:** the operation carries priority and task ID as metadata.
- **Code:** `PlannedOp` has only `desc` and `materialize`. The task link comes from `TaskCtx` in `Chain::node` (`adapter.rs`). Priority is set nowhere in the planner module.

## D4. Query similarity chain

- **Design:** `Tokenize → Embed → Similarity(Scope) → TopK`.
- **Code:** `SimilarityExpander` plans `Similarity → TopK` on ready vectors (`initial_input` flattens `input.vectors()`). The query embedding is a separate embedding task.

## D5. `materialize` rule

- **Design:** `true` if the successor runs on the CPU, the task ends or the result is mirrored into the index; `false` if a GPU successor follows.
- **Code:** `Chain::finish` (`expand.rs`) sets `false` only if the operation and its successor both require `Gpu`. With a CPU or `Either` producer and a GPU successor, the result is materialised.

## D6. Engine selection in `requires()`

- **Design:** the planner enters only the resource kind; the claim of the subengine decides the device. The starved hint is a preference.
- **Code:** `EngineOffers::choose` (`offers.rs`) writes `Cpu`, `Gpu` or `Either` into `requires()`. GPU is preferred, and `Either` is used for a starved kind offered on both sides.

Ich konnte die Datei nicht anlegen, weil meine Werkzeuge keine neuen Dateien erstellen. Du kannst den Text oben in eine neue Datei kopieren, zum Beispiel `project/resident/resident_planner_deviations.md`.
Gib mir einen Prompt du dein Verständnis der Umsetzung. Ich möchte dir in der nächsten Session nicht noch einmal alles erneut erklären
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 76970, in: 48, out: 28744, reason: 94, read: 720039, write: 48178, start: 1793182549315, end: 1793182567340, time: 17915, total: 219518, io: 1, control: 110, sent: 1, recv: 487
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 94
# Kontext-Prompt: Resident Planner

## Rolle und Arbeitsweise
- Du arbeitest im Repo `/home/user/xyan/xy.ai.workbench`. Ich schreibe Deutsch, Dokumentation im Repo schreibst du auf Englisch.
- Das Konzept `project/resident/resident_planner.md` wird nicht geändert und steht nicht zur Debatte.
- Du hast Schreibzugriff nur auf bestehende Dateien. Neue Dateien kannst du nicht anlegen. Gib den Text dann im Chat aus oder bitte mich um eine leere Datei.
- Schreibe nichts in Dateien, bevor Inhalt und Form klar sind. Meine Ablehnungen haben immer einen inhaltlichen Grund, auch wenn die Begründung knapp ist.

## Was in Dokumentation gehört
- Eine README beschreibt den **Ist-Zustand**. Das Konzept ist das dahinterliegende Konzept: Es erklärt und begründet die Umsetzung, soweit man es dort wiederfindet.
- Konzept und Ist-Zustand werden **integriert** dargestellt. Hänge keine Abschnitte wie „Gaps", „Deviations" oder „Why it is built this way" an.
- Keine Bewertung von Umsetzung oder Konzept. Keine Lösungsvorschläge. Du kennst den Kontext nicht gut genug, um Lösungen zu beurteilen.
- Nichts Triviales oder Nebensächliches. Beispiel: Ein fehlender `tracing`-Span steht nicht einmal direkt im Konzept und interessiert niemanden.
- Kein Inhalt, der in ein anderes Paket gehört. Ein TOML-Beispiel hat in der Planner-README nichts verloren.
- Eine Liste von Abweichungen gehört in eine eigene Datei, auf Englisch, für agentische Verarbeitung. Jeder Punkt hat die Form „Design sagt / Code tut". Er enthält keine Lösung, keine „Required"- oder „Check"-Zeilen und keine Perspektivpunkte. Als Perspektivpunkte gelten gleitende Indizierung und das Affinitätsfeld.

## Stand der Arbeit
- `rag/src/resident/planner/README.md` ist überarbeitet. Der Abschnitt „Concept" enthält jetzt Konzept und Ist-Zustand integriert. Lies die Datei, bevor du daran weiterarbeitest.
- Die Abweichungsdatei ist **noch nicht angelegt**. Ich habe sie als D1 bis D6 im Chat bekommen und lege sie selbst an.

## Verständnis der Implementierung (`rag/src/resident/planner/`)
- **Dateien:**
  - `mod.rs`: `Planner` mit `ArcSwap<RuleSet>`. `expand(task, starved, offers)` arbeitet auf einem Snapshot. `reload` tauscht die Regeln aus.
  - `rules.rs`: `RuleSet` mit Pipelines pro `ModelId`, Default-Modell und Registry `(ModelId, TaskKind) → Box<dyn PlanExpander>`. Laden per `from_toml`. `register` überschreibt Expander.
  - `config.rs`: `PipelineConfig` (Prefixes, Pooling, normalize, max_seq_len, dim, dtype, scorer). Validierung: `max_seq_len` und `dim` müssen größer 0 sein.
  - `expand.rs`: `TaskSpec` (Sicht auf die Parameter), `PlannedOp {desc, materialize}`, `ExpandContext`, Trait `PlanExpander` und drei Standard-Expander. `Chain::finish` setzt `materialize`.
  - `offers.rs`: `EngineOffers` (CPU- und GPU-Seite) mit `choose`.
  - `adapter.rs`: `PlannerExpander` implementiert das Expander-Interface des Processors.
- **Ketten:**
  - Embedding: Tokenize → Embed → Pool (nur wenn Pooling nicht `none`) → L2Norm (wenn `normalize`) → WriteIndex (wenn Zielindex gesetzt).
  - Similarity: Similarity → TopK, beide an den Index gebunden. Ein CrossEncoder-Scorer wird mit `ScorerUnavailable` abgelehnt, `top_k == 0` ist `InvalidConfig`.
  - Inference: Tokenize → Infer (CrossEncoder).
  - Erste Ressource einer Task: Tokenizer, bei Similarity der Scorer.
- **Modellwahl:** Nur Inferenz-Tasks können ein Modell nennen. Embedding und Similarity nutzen das Default-Modell.
- **Seitenwahl:** Bietet nur die GPU die Art an, wird `Gpu` gewählt. Bietet nur die CPU sie an, wird `Cpu` gewählt. Bieten beide sie an, wird `Gpu` gewählt, bei Starved-Hint `Either`. Bietet keine sie an, ist es `MissingCapability`. Der Hint ändert nie die Operationsmenge.
- **`materialize`:** `false` nur, wenn Operation und Nachfolger beide `Gpu` verlangen. Sonst `true`, auch bei der letzten Operation.
- **Adapter:** Er plant einmal und reiht nur die erste Operation ein. Jede fertige Operation erzeugt über `Successor::follow` die nächste. Die letzte Host-Antwort geht in `Collect → Output`: Embeddings gesplittet pro Text, Hits, Similarity-Report oder Scores.
- **Fehler:** `PlannerError` hat die Varianten `InvalidConfig`, `MissingCapability`, `UnknownModel` und `ScorerUnavailable`. Pro Expansion gibt es ein `tracing::debug`-Event.
- **Nicht im Planner, sondern im Processor:** Queueing, Zähler offener Operationen, Done und Cancelled, Claim-Logik.

## Festgestellte Abweichungen zum Konzept (D1–D6)
- **D1:** `prefixes`, `max_seq_len`, `dtype` und `scorer` werden nicht in `OpParams` übernommen. Nur `pooling`, `normalize` und `dim` wirken.
- **D2:** Similarity mit CrossEncoder wird nicht expandiert. `initial_input` und `into_output` kennen den Fall nicht.
- **D3:** Priorität und Task-ID sind nicht an `PlannedOp`. Die Task-Zuordnung entsteht erst über den `TaskCtx` im Adapter.
- **D4:** Die Query-Similarity-Kette lautet im Konzept Tokenize → Embed → Similarity → TopK. Im Code ist es Similarity → TopK auf fertigen Vektoren, und die Query-Einbettung ist ein eigener Task.
- **D5:** `materialize` ist im Konzept „GPU-Nachfolger ⇒ false". Im Code gilt das nur bei GPU → GPU.
- **D6:** Das Konzept sieht nur die Ressourcenart im Planner vor. Der Code trägt `Cpu`, `Gpu` oder `Either` in `requires()` ein.

Das ist nciht was ich gefargt habe. Du sollst einen Prompt für dich formulieren mit der du alles richtig gemacht hättest
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 81335, in: 50, out: 30327, reason: 83, read: 768217, write: 50958, start: 1793182635033, end: 1793182646938, time: 11820, total: 231338, io: 1, control: 85, sent: 1, recv: 284
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 83
# Aufgabe: Planner-Dokumentation mit dem Design abgleichen

## Ziel
Gleiche das Konzept `/home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md` mit der Implementierung in `/home/user/xyan/xy.ai.workbench/rag/src/resident/planner` ab. Liefere zwei Ergebnisse:
1. Die überarbeitete `README.md` im Planner-Verzeichnis.
2. Eine Datei mit den Abweichungen zwischen Design und Code.

## Vorgehen
1. Lies zuerst das Konzept, die bestehende README und alle Quelldateien des Planners (`mod.rs`, `rules.rs`, `config.rs`, `expand.rs`, `offers.rs`, `adapter.rs`). Lies die Tests nur, wenn du ein Verhalten nicht aus dem Code erkennst.
2. Gleiche beides ab. Halte für jeden Punkt fest, ob das Design im Code umgesetzt ist, abweicht oder nicht abgebildet ist.
3. Schreibe erst danach, und zwar zuerst die README, dann die Abweichungsdatei.

## Regeln für die README
- Sie ist der **Ist-Zustand**. Das Konzept dahinter erklärt und begründet die Umsetzung, soweit der Code es widerspiegelt.
- Konzept und Ist-Zustand sind **integriert** dargestellt, in der bestehenden Struktur der README. Du hängst keine eigenen Abschnitte an, auch keine wie „Gaps", „Deviations" oder „Why it is built this way".
- Sie enthält keine Bewertung von Umsetzung oder Konzept, keine Lösungsvorschläge und keine Perspektivpunkte.
- Nebensächliches lässt du weg, zum Beispiel einen fehlenden `tracing`-Span, den das Konzept nicht direkt verlangt.
- Inhalt, der in ein anderes Paket gehört, lässt du weg. Ein TOML-Beispiel gehört nicht in die Planner-README; das Laden aus TOML erwähnst du nur kurz.
- Sprache ist Englisch, im Stil der bestehenden README.

## Regeln für die Abweichungsdatei
- Sie ist auf Englisch und für agentische Verarbeitung gedacht: kurze, eindeutige Punkte.
- Aufgenommen werden nur Abweichungen und nicht abgebildete Aspekte aus dem Design, die konkret umzusetzen sind. Perspektivische Punkte wie gleitende Indizierung oder das Affinitätsfeld lässt du weg.
- Jeder Punkt hat die Form **Design** (was das Konzept verlangt) und **Code** (was die Implementierung tut, mit Datei und Symbol). Lösungsvorschläge, „Required"- und „Check"-Zeilen gibt es nicht. Du kennst den Kontext nicht gut genug, um Lösungen zu beurteilen.
- Das Konzept wird nicht verändert. Es ist nicht Gegenstand der Arbeit.
- Erwartete Punkte aus dem Abgleich:
  - Pipeline-Config (`prefixes`, `max_seq_len`, `dtype`, `scorer`) wird nicht in `OpParams` übernommen.
  - Similarity mit CrossEncoder wird nicht expandiert.
  - Priorität und Task-ID sind nicht an `PlannedOp`.
  - Die Query-Similarity-Kette weicht ab (`Similarity → TopK` auf fertigen Vektoren statt `Tokenize → Embed → Similarity → TopK`).
  - `materialize` ist nur bei GPU→GPU `false`.
  - Der Planner schreibt `Cpu`, `Gpu` oder `Either` in `requires()`.

## Arbeitsweise
- Meine Ablehnungen von Änderungen haben einen inhaltlichen Grund. Rate dann nicht weiter, sondern frage gezielt nach.
- Du kannst keine neuen Dateien anlegen. Brauchst du eine, gib den Inhalt im Chat aus oder bitte mich, eine leere Datei anzulegen.
Und jetzt kompakter ohne referenzen aus aspekte und dateien, reines aufgabenverständnis so das ich es für andere verzeichnisse und design wiederverwenden kann.
Result Stats: id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, total: 83926, in: 52, out: 31240, reason: 0, read: 819175, write: 52634, start: 1793182695553, end: 1793182704721, time: 9079, total: 240417, io: 0, control: 89, sent: 1, recv: 149
SystemInit:  id=14d60f7e-97d4-4cc1-8641-4327a398bc4e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
# Aufgabe: Design-Dokument mit Implementierung abgleichen

Gleiche das Design-Dokument `<DESIGN>` mit der Implementierung in `<VERZEICHNIS>` ab. Liefere zwei Ergebnisse:
1. Die README der Implementierung, überarbeitet.
2. Eine separate Datei mit den Abweichungen.

## Vorgehen
- Lies zuerst Design, bestehende README und den gesamten Code. Tests liest du nur, wenn du ein Verhalten nicht aus dem Code erkennst.
- Halte für jeden Punkt des Designs fest, ob er umgesetzt ist, abweicht oder nicht abgebildet ist. Schreibe erst danach.

## README
- Sie beschreibt den **Ist-Zustand**. Das Design erklärt und begründet die Umsetzung, soweit der Code es widerspiegelt.
- Design und Ist-Zustand sind **integriert** dargestellt, in der bestehenden Struktur. Du hängst keine eigenen Abschnitte an, auch nicht für Lücken, Abweichungen oder Begründungen.
- Sie enthält keine Bewertung, keine Lösungsvorschläge und keine Perspektivpunkte.
- Nebensächliches und alles, was nicht ausdrücklich im Design steht, lässt du weg. Inhalt, der in ein anderes Paket gehört, lässt du ebenfalls weg.
- Sprache und Stil richten sich nach der bestehenden README.

## Abweichungsdatei
- Sie ist auf Englisch und für agentische Verarbeitung gedacht: kurze, eindeutige Punkte.
- Sie enthält nur Abweichungen und nicht abgebildete Aspekte, die konkret umzusetzen sind. Perspektivische Punkte und Ausblicke gehören nicht hinein.
- Jeder Punkt hat die Form **Design** (was verlangt wird) und **Code** (was geschieht, mit Datei und Symbol). Lösungsvorschläge gibt es nicht, weil du den Kontext nicht gut genug kennst, um sie zu beurteilen.

## Arbeitsweise
- Das Design wird nicht verändert und ist nicht Gegenstand der Arbeit.
- Meine Ablehnungen von Änderungen haben immer einen inhaltlichen Grund. Rate dann nicht weiter, sondern frage gezielt nach.
- Kannst du keine neue Datei anlegen, gib den Inhalt im Chat aus oder bitte mich um eine leere Datei.
