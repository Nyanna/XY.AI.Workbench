# From Static Lookup Tables to Agent-Orchestrated Retrieval: A Unified Model of Representation, Interaction, and Aggregation in Information Retrieval

**Thesis.** Retrieval systems — from a Model2Vec lookup table to a ColBERT late-interaction index to a multimodal CLIP space — are not categorically different machines but configurations of a small set of shared dials: *where* context is injected (training time vs. query time), *how many* vectors represent a unit of text, *how* query and document interact (single-vector cosine, multi-vector MaxSim, joint cross-attention), and *where in the pipeline* heterogeneous evidence is reconciled (filter, late fusion, joint candidate generation, identity bridge, or query-side selection). The two parts below trace this claim from opposite ends — bottom-up from static embeddings toward an agent-driven multi-layer retrieval engine, and top-down from the question of where aggregation across heterogeneous representation spaces happens — and converge on the same conclusion: no architecture eliminates the need for a mapping between representation and meaning; every design merely relocates *where* and *when* that mapping is paid for.

# Part I: Model2Vec, Static Models, and the Continuum to Multi-Layer Retrieval

> Extrapolation of the conceptual conversation flow: from "What is Model2Vec" to a generic, agent-driven multi-layer retrieval architecture.

## 1. Starting Point: Static vs. Contextual Embeddings

**Model2Vec** distills a sentence-encoder model (teacher) into a pure lookup table: each vocabulary token is passed through the teacher in isolation (without sentence context), the result is reduced via PCA and Zipf-weighted. At runtime there is no transformer forward pass anymore — only lookup + averaging.

**Core difference from contextual models:** A transformer computes the vector of a token depending on the entire sentence (self-attention). A static model has *one* vector per token, independent of context. The sentence vector is a (weighted) average — permutation-invariant, without word order, without negation, without disambiguation.

**Advantages:** 100–500x faster, tiny (MB instead of GB), CPU-only, cheap distillation, often sufficient for coarse topical similarity (~85–90% of teacher quality on MTEB).

**Disadvantages:** no polysemy resolution, no syntax/scope ("not good" ≈ "good"), weaker at paraphrase/NLI/reranking, averaging blurs long texts.

**Rule of thumb:** Static for pre-filtering/clustering/dedup/large indices; contextual for nuance or as a second stage (rerank) on statically pre-filtered candidates.

## 2. Partial Context Compensation in Static Models

Local context can be shifted into the lookup table, without achieving true context-dependence:

- **N-grams/phrases as entries** (fastText bigrams, hashing trick) — captures negation/fixed expressions locally, costs table size.
- **Baking in context:** averaging contextual embeddings over many occurrences in the corpus (Bommasani et al.) — results in an average over all meanings.
- **End-to-end trained tables:** contrastive loss directly on the embedding table (`StaticEmbedding` in Sentence-Transformers).
- **Learned token weights** (SIF, Model2Vec weighting) — steer contribution, do not replace context.
- **Sense embeddings** (sense2vec) — multiple vectors per token selected by heuristic.

**Limit:** As long as the sentence vector is a sum/average of lookups, it remains permutation-invariant. Middle ground: static embeddings + a shallow layer (CNN, gating, mini-attention) — no longer purely static, but orders of magnitude cheaper than a full encoder.

## 3. The Continuum: Several Independent Axes

Instead of a single continuum, there are several axes:

- **Timing:** When does context flow in? (training vs. runtime)
- **Range:** Token → bigram → window (CNN) → whole sentence (attention)
- **Mixing mechanism:** fixed operation (average) vs. input-dependent (attention)

| Stage | Context | Cost per text |
|---|---|---|
| Lookup + average | none | linear, minimal |
| + N-gram entries | local, precomputed | linear, larger table |
| + CNN/gating | fixed window | linear |
| RNN | order, sequential | linear, not parallel |
| Transformer | arbitrary pairs, input-dependent | quadratic |

The qualitative leap lies in the **input-dependence of mixing**: only attention decides at runtime which token is relevant to which. Rule of thumb: the more context is shifted into parameters in advance, the cheaper the inference, the less the model reacts to unforeseen constellations.

## 4. Second Axis: Interaction Between Query and Document

Independent of "how are token vectors created" there is the axis "how is matching done":

| | Token vectors | Document representation | Matching |
|---|---|---|---|
| Model2Vec | static | 1 vector (average) | cosine |
| Dense bi-encoder | contextual | 1 vector (pooling) | cosine |
| ColBERT (late interaction) | contextual | n vectors (per token) | MaxSim |
| Cross-encoder | contextual, joint | none, per pair | transformer score |

**ColBERT / Late Interaction:** Documents are encoded offline (query-independent, therefore precomputable), only the query is encoded at runtime. MaxSim: for each query token the most similar document token, sum of the maxima. This is a **soft inverted index** — instead of exact term matching, a vector-proximity match.

**Cross-encoder** is NOT precomputable: query and document run jointly through attention, the document representation depends on the query.

## 5. BM25 and the Inverted Index as Foundation

**Inverted index:** word → list of documents (inverts document→words). **BM25** is the scoring formula on top of it: term frequency (with saturation) × inverse document frequency (IDF) × length normalization.

**word2vec vs. BM25:** word2vec is a learned, dense representation (semantic similarity, but no document score). BM25 is an unlearned, sparse scoring formula (only exact terms, but IDF weighting and length normalization built in). Complementary — hence hybrid search.

## 6. Embeddings Are Not "Exact Exports"

An embedding is a **projection** of a particular tap (usually the last layer, pooled), not a complete export of internal representations. Raw hidden states of generative decoders are poorly suited for similarity search (anisotropic space, optimized for next-token prediction). Good embedding models require additional **contrastive training**.

Model2Vec exports only the **context-free** slice of a teacher model: vector(token | no context), not vector(token | context). The attention mechanism that processes context cannot be copied into a table.

## 7. Expanding the Vector Space: Multiple Vectors per Chunk

- **By text structure:** propositions/sentences, late chunking (whole text through the encoder, then pooling per section), hierarchical (RAPTOR).
- **By expected queries:** Doc2Query (indexing generated questions), HyDE (query → hypothetical answer document).
- **By token (ColBERT):** extreme case, one vector per token.
- **Fixed intermediate forms:** Poly-Encoder, ME-BERT, MUVERA (multi-vector → fixed vector).

**Limits:** memory/latency grow linearly with vector count, aggregation must be defined, redundancy creates dedup need, coverage is always an assumption about future queries.

## 8. Central Thesis: Retrieval Is Localization ("Where Does It Say Something About X")

The core thesis: a search over an index fundamentally answers only one question type — localization. More complex question types (answer form, relation, aggregation) can often be solved **query-side** (decomposition into sub-questions, intersection formation), not necessarily index-side (e.g. Doc2Query).

This leads to the **axis "when and where does understanding happen?"**

| | Index performs the conclusion | Agent performs the conclusion |
|---|---|---|
| Cost | once, amortized over all queries | repeated per query |
| Latency | low | high |
| Fit | optimal only for anticipated questions | adapts to the query |
| Freshness | stale when the source changes | always on raw data |
| Errors | frozen, invisible | visible, correctable |
| Auditability | provenance is lost | source location in view |

The distribution of queries decides where one stands on this axis. Raw text is the only lossless end; every precomputed conclusion is a compression assumption.

## 9. Dials Instead of Categories: A Shared Parameter Space

The seemingly different methods (static, bi-encoder, ColBERT) are **points on the same dials**, not different categories:

| Dial | Affects | Trade-off |
|---|---|---|
| Chunk size | addressing granularity | precision vs. more vectors |
| Pooling factor / pruning | vectors per chunk | smaller index vs. less late interaction |
| Dimension, quantization | capacity per vector | memory vs. resolution |
| Encoder context (isolated → late chunking) | context baked in in advance | indexing cost vs. disambiguation |
| Encoder size | quality of the pre-computed representation | indexing cost vs. quality |

Extreme points: Model2Vec (all dials coarse) ↔ dense bi-encoder (medium) ↔ ColBERT (all dials fine).

**Caveats:** dials are not independent (interactions exist), and not every model is equally well-trained for every dial position (Matryoshka training, ColBERT pooling tolerance as solutions).

## 10. Routing/Cascade as a Practical Compromise

Instead of a fixed dial position: **switching within the continuum based on fit and threshold.**

- **Index-side:** fit signal (length, density, domain) selects resolution per chunk.
- **Query-side (cascade):** cheap stage delivers candidates + confidence; on low separability (margin, score entropy, disagreement between BM25 and embedding) escalation to finer matching.

**Pitfalls:** calibration between stages, false confidence of the cheap stage, recall ceiling (stage 1 must be tuned for recall), consistency (stages sequential, not in mixed vector spaces).

## 11. Existing Building Blocks (State of Research)

- **Vector-count dial:** token pooling/pruning in ColBERT.
- **Capacity dial:** Matryoshka training, quantization, ColBERTv2 residual compression.
- **Context dial:** late chunking (Jina).
- **Connecting models:** BGE-M3 (dense + sparse + multi-vector from one model), SPLADE (learned sparse), MUVERA (multi-vector → fixed).
- **Infrastructure:** Vespa (full freedom, steep learning curve, ranking expressions in-cluster), Qdrant (lean, prefetch chains, Rust), Weaviate (comfort, modules, multi-tenancy, Go).

No framework found treats all dials as a consciously designed, shared trade-off surface.

## 12. Turning Point: Agent as Universal Retriever

The decisive counterpoint: an already established **2-stage concept** — tools (grep, bash, context7, OpenAlex, Exa …) are cached, stage 1 delivers only filtered, ID-associated excerpts, stage 2 delivers details on request — has in practice **never fallen into the worst case** and is superior to existing retrieval methods.

**Why this is robust:**
- **Lossless by reference:** the full output stays in the cache; stage 1 is only a deterministic view of it. Nothing is lost, it is only deferred.
- **Deterministic instead of learned:** no unpredictable model failure.
- **The agent is the ranker:** it sees the full query context and intermediate state — structurally superior to any pretrained bi-encoder (100M–few billion parameters, one-time decision without follow-up).

**Cost logic (corrected in dialogue):** an additional turn costs a cache read, but every decode step reads the entire KV cache anyway — that is the dominant cost for long context. A filtered, small view shortens **every** subsequent turn (not just the current one), while a large raw output permanently slows it down. The net clearly favors filtering for long sessions and large outputs.

**Vocabulary gaps (private jargon):** here the agent has a structural advantage that no retriever has — it learns the vocabulary *during* the search (pseudo-relevance feedback with the model as evaluator), without training and without an index. For exact terms BM25/grep wins literally anyway; dense embeddings are the actual weak point here.

**Limits of the pattern (remaining, not refuted):**
- Recall ceiling: what the underlying tools don't deliver, the agent cannot find.
- The reduction rule (top-N, truncation) must make visible what was cut off (no silent loss source).
- Aggregation questions ("all passages that contradict X") increase turn count and cost.

## 13. Final Architecture: Multi-Layer Anytime Retrieval Engine

A concrete, generic architecture emerges that unites all previous axes — as an **additive, agent-driven composition of independent layers** over a shared store (the file system).

### 13.1 Basic Principles

1. **Shared storage:** the file system itself is the "table". No database needed, as long as layers remain independent.
2. **Layers are fully independent:** each layer chooses its own chunk granularity, its own model, its own index — or none at all (grep).
3. **Only shared coordinate system:** `(content hash, start, end)` — spans within a file. No shared chunk schema needed.
4. **Additive, never subtractive:** a missing, outdated, or incomplete layer can only deliver *less* signal, never a wrong one. Partial coverage is therefore uncritical (only make it visible in status, so that "nothing found" is not confused with "not run").
5. **Signals are tagged, not fused.** Every hit carries a list of signal names (`["bm25", "summary"]`). The meaning of a signal (origin: text vs. derived; granularity; coverage) is stated once in the status, not per hit. Fusion/weighting remains with the agent, which interprets the signals with full query context.
6. **Timebox and quality goal come from the caller (the agent).** It knows corpus size, prompt goal (precise small set vs. large candidate set vs. fast answer) and chooses parameters accordingly. The engine delivers only signals; it does not anticipate.
7. **Lazy, usage-driven materialization:** layers are not built preemptively for the whole corpus, but on-demand, prioritized by usage (files from the last request first), with spare resources used up to the timebox or the next request.
8. **Change detection is lazy, no separate service.** The always-current text-search layer (grep/ripgrep) runs first, detects changed files in scope via `stat`/hash deviation and queues them for the more expensive subsequent layers. No watcher, no periodic scan needed.

### 13.2 Layer Roles

- **Generate (G):** delivers its own candidates from the entire scope.
- **Refine (R):** only evaluates existing candidates (cheap, but limited by the recall of the previous stage).

### 13.3 Sorted Layer List (Roughly by Total Cost)

| # | Layer | Tools/Approach | Role |
|---|---|---|---|
| 1 | File names | `fd`, glob | G |
| 2 | Exact/regex text search | grep → ripgrep → ugrep (fuzzy, boolean) | G |
| 3 | Text search over formats | ripgrep-all, pdfgrep | G |
| 4 | Fuzzy re-scoring | rapidfuzz | R |
| 5 | Structural search | ast-grep | G |
| 6 | Trigram index | csearch/cindex, Zoekt, FTS5 trigram | G |
| 7 | BM25 word level | FTS5, bm25s, Tantivy, Recoll | G |
| 8 | Symbol/AST index | tree-sitter | G |
| 9 | Formula structure | Tangent-CFT, SSEmb | G |
| 10 | Learned sparse | SPLADE, BGE-M3 sparse | G |
| 11 | Static embeddings | Model2Vec + NumPy matrix | G |
| 12 | Dense bi-encoder | BGE-M3 dense, Jina, Qwen3-Embedding | G |
| 13 | Dense vectors on LLM-derived views | context lines, Doc2Query, summaries | G |
| 14–15 | Late interaction (MaxSim) | ColBERT, PyLate encoder + brute-force MaxSim (no PLAID needed, since incrementality matters more than speed) | G/R |
| — | Cross-encoder / LLM rerank | qmd, reranker models | R, most expensive stage before the agent |

### 13.4 Separate Layers for Metadata and Derived Views

- **Metadata is not index load, but enrichment on match.** Path, mtime, language, symbol context are derived only on hit from the current file state — no reindex on rename, no schema constraint in the layer.
- **Summaries, generated questions, and metadata are separate layers**, not baked into the chunk text. This keeps the contract uniform (`hash, span, signals`).
- **Insight:** the constellation of signals itself is information for the agent:

| Hit constellation | Meaning |
|---|---|
| chunk layer only (`source`) | passage states it literally; document not recognizably on-topic |
| summary/question layer only (`derived`) | abstraction/conclusion, not literally evidenced — hypothesis to verify |
| both | strongest evidence, directly citable |

This makes derived layers **signposts**, not **evidence** — the agent can distinguish, via the origin marker (`source` vs. `derived`), what is actually in the corpus and what was only interpreted.

### 13.5 Change Detection & Upserts as a Shared Building Block

- **No Merkle tree, no central watcher needed** — instead: the always-current text-search layer *is* the change detector. During the pass over the scope, it compares `stat` (size, mtime) of every hit/scope file against the central index; on deviation it is hashed and the file is queued with high priority into the build queue of the subsequent layers.
- Hits from subsequent layers on meanwhile-changed files are discarded or marked as outdated (span no longer matches).
- Content addressing (hash as key instead of path) makes renaming/moving free and automatically deduplicates identical files.
- Idempotent `apply(add, remove)` per layer, keyed by `(layer, hash)`, makes aborts and repetitions uncritical — fits abortable, lazy background work.

### 13.6 Layer Contract (Protocol)

```python
class Layer(Protocol):
    name: str
    roles: set[Literal["generate", "refine"]]
    def applies(self, scope) -> bool: ...
    def status(self, scope) -> LayerStatus:  # origin, granularity, coverage, query format, cost
        ...
    async def search(self, query, scope, budget, candidates=None) -> list[Hit]: ...
    async def build(self, units, cancel: asyncio.Event) -> None: ...
```

`Hit = (hash, start, end, excerpt, signals: list[str])`. Status is communicated once per layer, not per hit — keeps the contract minimal and arbitrarily extensible (a new layer = a new name, no schema change).

### 13.7 Positioning Against Existing Approaches

The individual building blocks have precursors (Cursor: two fixed layers — local trigram index + semantic index, Merkle-diff updates; mcp-context/context-compress: tool-output filtering with ID follow-up request; anytime ranking in IR research: budget-controlled cascade abort). The combination of (a) an open, not hard-wired set of layers, (b) caller-controlled timebox/quality goal across heterogeneous layers, (c) the text-search layer as implicit change detector for all more expensive subsequent layers, and (d) a generic tool-filter protocol over arbitrary external sources (OpenAlex, Exa) was not found in this form in the research.

## 14. Practical Consequences for Implementation

- **Start lean, local, embedded:** SQLite+FTS5 (word and trigram tables), NumPy/FAISS for dense vectors, PyLate for ColBERT encoding, brute-force MaxSim over sidecar files (no PLAID index needed, since incrementality matters more than query speed).
- **Qdrant/Vespa only when needed** (multiple users/services on the same index, strong filters, very large corpora) — not as a starting point, when fusion/control lies with the agent anyway.
- **Add domain-specific layers as needed:** code (tree-sitter, ast-grep, LateOn-Code), scientific texts (LaTeX-aware tokenization, formula-to-prose informalization via LLM, SPECTER2 at abstract level), prose (standard hybrid).
- **Evaluation per knowledge base instead of global:** a small test set per KB (synthetic via LLM or from real agent follow-up requests as implicit relevance labels), sweep over a few dials, store configuration instead of changing architecture.
- **Logging of agent decisions is the actual control instrument:** which layers/parameters were chosen, how often follow-up or rephrasing occurred, empirically shows where more expensive layers pay off — not an a priori architecture decision.

---

# Part II: Multi-Layer Retrieval: A Conceptual Model of the Aggregation of Heterogeneous Representation Spaces

> Extrapolation of the conceptual thinking developed in dialogue. As of: 2026-10-05.

## 15. Starting Question and Movement of Thought

The discussion began with a technical observation (Solr: one index, low algorithmic variance) and moved through six refinement stages to a general architectural principle for retrieval across heterogeneous, not necessarily learned representation spaces. The movement is remarkable because it repeatedly resolves seemingly settled questions (fusion vs. pipeline, learned vs. unlearned, one index vs. many indices) anew by adding a condition, rather than discarding them.

## 16. Central Distinction: Where Does Aggregation Take Place?

The consistent analytical grid is the question of *at which point in the pipeline* heterogeneous evidence is combined. Four levels were identified, ascending by degree of coupling:

| Level | Mechanism | Example |
|---|---|---|
| **Filter / set operation** | Boolean intersection of hard hits, only one branch ranks | SQL query planner, Solr filter queries, faceted search |
| **Late fusion (score/rank)** | Each retriever delivers top-k, then normalization + combination | RRF, CombSUM/CombMNZ, Elasticsearch retriever framework, Qdrant prefetch |
| **Joint candidate generation** | Shared traversal over multiple index structures before scoring | Vespa (WAND + HNSW in the same query tree), inference-network model (INQUERY/Indri) |
| **Identity bridge (pipeline)** | A canonical key chains incommensurable modalities serially | Name → InChIKey → fingerprint search |

The historical thesis initially stated: late fusion is a distortion (top-k truncation before combination), joint candidate generation is superior. This thesis was relativized in the course of the dialogue: for multi-vector methods (PLAID, MUVERA) a two-stage architecture (candidate index + rerank) is not a compromise but cost optimization without quality loss, as long as the recall of the first stage is sufficient.

## 17. Correction of an Initial Assumption: Multimodality Is Not New

The assumption that combined/multimodal queries are a genuinely AI-induced phenomenon was rejected. Evidence:

- Data fusion (Fox & Shaw 1994), inference-network retrieval (INQUERY, 1990s)
- CBIR systems (QBIC, 1993): late fusion over color, texture, shape, text
- Enterprise search (Autonomy IDOL, FAST ESP, Endeca): probabilistic, faceted, concept-based
- Federated IR (CORI, GlOSS, result merging)

**What has actually changed** is not the idea of combination, but **commensurability**: a learned shared vector space (CLIP et al.) makes scores directly comparable across modalities for the first time, without normalization heuristics. Additionally, HNSW structures make a second/third index operationally cheap enough to make joint candidate generation practical.

## 18. The BM25 Objection: Data-Type Specificity Was Always Already Solved

The objection that BM25/text search is not suited for arbitrary syntax forms led to a clarification: the "one-index reduction" applied only to *ranked text search*. For other data types (geographic, chemical, biological, time-series-based, structured) dedicated index structures have existed for decades (R-trees, suffix arrays, fingerprints, BLAST, SAX/DTW). Combination took place there, but predominantly **conjunctively/filtering**, not score-unified — because only text retrieval had a mature, generalizable relevance model.

**Consequence for the conceptual model:** the pipeline/bridge solution (molecule example: name → identifier → substructure search) is the historical normal case for incommensurable modalities with hard semantics (containment, identity).

## 19. The Multi-Vector Objection: Learning Is Not Eliminated, Only Shifted

Core thesis: multi-vector/late-interaction approaches (ColBERT, ColPali) shift understanding out of indexing/pipeline construction and into query processing (MaxSim at query time). The pipeline does not need to understand the domain, only be able to map a semantic space.

**Conceded:** the interaction function (which aspects count) is indeed decided only at query time, not at indexing time. This is a genuine structural difference from fusion/pipeline.

**Limit that remains:** representation learning itself (the encoder that maps tokens/patches of both modalities into compatible vectors) does not disappear — it is presupposed. The agnosticism of the pipeline is bound to the *existence* of such an encoder. For text-image (CLIP, ColPali) it exists; for molecule name↔structure graph at token level not at comparable quality. Additionally, MaxSim remains a soft similarity without a containment guarantee — structurally unsuitable for substructure search.

## 20. The Decisive Turn: Selection Instead of Fusion

The central conceptual contribution in the later course of the dialogue: if the **query itself selects the appropriate representation space** (instead of fusing scores across spaces), the incommensurability problem disappears entirely — there is no need to normalize/compare at all, because there is no fusion.

This yields a fourth architectural principle, complementary to the three named in Section 16:

| Principle | Coupling between spaces |
|---|---|
| Fusion (late/joint) | Scores/candidates from multiple spaces are combined |
| **Selection** | Query selects exactly the appropriate space, no combination needed |

**Condition that does not disappear:** "posing the query in a compatible space" means that per space there exists a query→space encoder. The mapping is not eliminated, but concentrated on the query side and possibly generated dynamically (e.g. by an LLM as a runtime translator instead of pretrained pair alignment). For the molecule query, the chain name→structure→fingerprint would remain, only dynamic instead of hard-coded.

**Index question as a follow-up question, not a precondition:** whether this requires one index or many dynamic indices depends on whether a shared space exists (→ one index suffices, the CLIP case) or each modality has its own space/metric (→ many indices, the named-vector pattern: Qdrant, Milvus, Vespa tensor fields, Weaviate).

## 21. Finalization: Unlearned Space Selection via Signal Estimation (Probing)

The last and most far-reaching refinement: if signal in query and index is **quantifiable** (not: must be learned), similarity between representation spaces can be *estimated* without a trained mapping. Probing thus becomes, from "trying out a space," a **statistical signal estimation per space**.

### Unlearned Quantification Methods

- **Margin/gap** between rank 1 and rank k as a signal-sharpness indicator
- **Query performance prediction** (NQC, Clarity Score, WIG) — Z-score of the top hit against the null model of the index distribution; classic IR, older than dense retrieval
- **Hubness correction** (CSLS) for high-dimensional neighborhood density
- **Extreme-value normalization** (E-value principle from BLAST): yields a p-value comparable across spaces, without training

### Resulting Architecture

1. Multiple representation spaces (learned or classic, e.g. fingerprint space), each with its own null model/index statistic.
2. The query is projected in parallel into all (or candidate-worthy) spaces.
3. Signal strength per space is normalized via the null model (instead of being directly compared).
4. Aggregation happens via **document identity**: an object that stands out in multiple spaces receives combined signal — this is itself fusion again, but on a normalized evidence level instead of on raw, incommensurable scores.
5. Spaces without signal fall out automatically through the null model — no manual pruning needed.

### Limit of the Principle

Signal ≠ relevance: a sharp peak shows that the query hits *something specific*, not that it is the *intended* thing. With incorrectly translated queries (e.g. a name mismatch), a space can point very confidently to the wrong object. Query-side translation thus remains a non-eliminable error channel, even if the space selection itself is unlearned.

## 22. Overall Model: Four Coupling Degrees of Heterogeneous Retrieval Spaces

```
Coupling weak ─────────────────────────────────────► Coupling strong

Identity bridge      Selection (probing)    Fusion (late)       Joint candidate gen.
(serial mapping,     (query selects space,  (scores/rankings    (shared traversal,
 hard semantics)      signal instead of      of several top-k    one query tree over
                      training)              combined)            multiple index structures)

Example: name→        Example: E-value       Example: RRF,       Example: Vespa
InChIKey→fingerprint  normalization over      Elasticsearch       (WAND + HNSW),
                      fingerprint and         retriever framework  Indri inference network
                      embedding space
```

**Core statement of the conceptual model:** the mapping between modalities does not disappear completely in any of the four variants. It migrates each time to a different place:

- in the bridge, into an explicit identifier step,
- in fusion, into a normalization/weighting function,
- in joint candidate generation, into the construction of a shared but multi-part query tree,
- in selection/probing, into a query-side encoder or a null model per target space.

The decisive progress of this line of thought lies not in proving that learning becomes superfluous, but in showing that the **load distribution of the mapping** is itself a design parameter — and that unlearned, statistically grounded methods (query performance prediction, extreme-value normalization) represent a historically older, but so far little-used alternative to learned shared spaces for this purpose.

## 23. Open Implementation Gap

Standard systems (Qdrant, Vespa, Elasticsearch, OpenSearch) offer parallel multi-index queries and named-vector fields, but provide **no** built-in null model and no extreme-value normalization across heterogeneous spaces. This layer — signal estimation per space plus identity-based aggregation of normalized signals — would currently have to be built from scratch; it exists conceptually in classic IR (QPP, BLAST E-value), but is not available as a generic pattern in modern multi-vector/hybrid-search stacks.
