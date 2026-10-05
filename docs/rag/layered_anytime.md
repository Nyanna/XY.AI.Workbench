# Layered Anytime Retrieval

> A retrieval engine that delivers signals, not answers. Many independent layers, one shared contract, one agent that decides.

---

## 1. Purpose

This thesis defines a **multi-layer, incremental retrieval engine** for agent-driven knowledge work. It serves heterogeneous corpora (source code, scientific papers, prose, tool outputs, external APIs) through a single, agnostic interface.

The engine does one thing: **it returns well-tagged evidence about where something is.** Interpretation, aggregation, ranking policy, and the final conclusion belong to the calling agent, which holds the most context at the latest possible moment.

## 2. Thesis

1. Retrieval answers one question: *where does something about X stand?* Everything else (answer form, relations, aggregation, negation, conditions) is a property of the query and the reader, not of the index.
2. The meaningful design axis is **where understanding happens**: at indexing time, at query time, or in the agent. The engine should not anticipate what the agent can do better with full query context.
3. Retrieval methods are not competing categories. They are **layers**: independent signal producers with different cost, granularity, and characteristics, composed additively under a time budget set by the caller.
4. A retriever is a weaker model than the agent that uses it. Its role is to widen and order the field of view, never to replace judgment.
5. Cheap, deterministic, always-current layers come first. Expensive, learned layers refine, correct, and extend, and they are built lazily from actual use.

## 3. Guiding Principles

| Principle | Meaning |
|---|---|
| **Signals, not conclusions** | The engine tags evidence. The agent weighs it. |
| **Additive layers** | A layer can only add information. A missing, partial, or stale layer contributes less, never something wrong. |
| **Understanding as late as possible** | Interpretation is shifted toward query time and toward the agent (late interaction, probing, iterative search). |
| **Agnostic composition** | Layers share only storage, orchestration, and change detection. Chunking, granularity, model, and index format are private to each layer. |
| **Usage-driven materialization** | Layers are built lazily, prioritized by what the agent actually touches. |
| **Caller-defined quality** | The caller sets timebox, target set size, and recall/precision emphasis. |
| **Incremental updates over retrieval speed** | Cheap, granular, abortable updates outrank index compression and lookup latency. |
| **Lossless by reference** | Full outputs stay cached. Reductions are views, never deletions. |

## 4. The Argument Chain

### 4.1 A continuum, not categories

Static embeddings, dense bi-encoders, multi-vector late interaction, and cross-encoders are points on shared knobs:

- **Granularity**: document, section, sentence, token.
- **Vectors per unit**: one, k, or one per token (adjustable through chunk size, pooling, pruning).
- **Capacity per vector**: dimension, quantization, Matryoshka truncation.
- **Encoder context**: isolated lookup, per-chunk encoding, whole-document encoding followed by pooling (late chunking).
- **Interaction**: none, late (MaxSim), early (cross-encoder).

These knobs trade pre-computation, static share, index size, and query-time interaction against each other. A generic pipeline exposes the knobs instead of committing to one method. Switching along the continuum by fit and threshold (margin, disagreement between signals) is a natural extension.

### 4.2 Where understanding happens

One axis organizes the whole space:

```
raw text + terms  →  embeddings, doc2query, type annotations  →  extracted facts, graphs, summaries  →  agent as index
(no understanding        (some interpretation                      (relations precomputed,                (all understanding
 in the index)            precomputed)                              queries become lookups)                at query time)
```

Pre-computed conclusions amortize across many queries but freeze assumptions. Query-time understanding adapts to each question and sees the full context. Raw text is the only lossless end; the agent is the only component that can answer questions nobody foresaw at indexing time.

### 4.3 Two levels that must not be mixed

- **Level 1, the single call**: precision, compactness, and speed of the returned material.
- **Level 2, the whole prompt**: navigability. The tool must be steerable over several calls.

Level 2 is why the project works with multi-vector signals. MaxSim decomposes a score into contributions per query token, each with a concrete counterpart in the document. This yields, without extra machinery:

- **Span localization**: which sentences carry the match, so chunks can be trimmed.
- **State information**: which part of the query is covered and which is not.
- **Steerable breadth**: query-token weights can be changed or zeroed at search time to widen or narrow the result, with no re-indexing.

### 4.4 The retriever is the weaker model

Dense embeddings fail on private jargon unless trained for it. The agent does not: it learns the vocabulary *during* the search (a first hit shows the internal term in context, the next query uses it), which is pseudo-relevance feedback with a model as evaluator.

Plain text search with fuzzy matching already imitates multi-vector behavior. Per aspect, the agent runs one search, receives fragments, and combines the findings. That is late interaction with a learned, context-sensitive aggregator instead of a fixed sum. The agent filters with more intent and context than any retrieval model could anticipate, which is the same reason agentic search with grep has displaced vector RAG in coding agents.

### 4.5 The two-stage pattern

Tool outputs (list, grep, bash, context7, OpenAlex, Exa, and others) are cached. A deterministic filter reduces the signal and returns an ID-associated view. Only a second call retrieves details.

- **Smaller turn-over-turn context** reduces KV-cache reads in prefill and in every decode step, which directly lowers latency and cost.
- **Lossless by reference**: the full output stays in the cache. Whether something was filtered or never requested makes no difference to the agent.
- **Requesting more is the natural retrieval flow.** The extra turn is cheap compared with carrying raw output through every following turn.
- **Generic**: the same filter works for any tool, and it has not hit a worst case in practice.

## 5. Architecture

### 5.1 Overview

```
                        ┌──────────────────────────────┐
                        │            Agent             │
                        │ sets timebox, targets, scope │
                        └──────────────┬───────────────┘
                                       │ status / search / fetch
                        ┌──────────────▼───────────────┐
                        │         Orchestrator         │
                        │ scope · timebox · tagging ·  │
                        │ layer protocol               │
                        └─┬────┬────┬────┬────┬────┬───┘
   fast / always current  │    │    │    │    │    │   slow / lazily built
                          ▼    ▼    ▼    ▼    ▼    ▼
                       glob  text  tri-  BM25  AST  embeddings … late interaction
                             search gram
                        ┌──────────────────────────────┐
                        │ Shared: file index · cache · │
                        │ change detection             │
                        └──────────────────────────────┘
```

### 5.2 What is shared, what is private

**Shared**

- Storage (the file system, plus a central file index).
- The orchestrator.
- Change detection and the extraction cache.

**Private to each layer**

- Chunking and granularity.
- Model, encoder, and index format.
- Tokenization, normalization, and query language.
- Derived artifacts (summaries, generated questions, descriptions).

### 5.3 Coordinate system

The only common coordinate is the file: **`(content hash, start, end)`**. Every layer reports hits in these spans.

- Renames and moves cost nothing, and identical files share entries.
- Overlap and containment between layers are detected by span intersection (a section hit contains a sentence hit), performed by the orchestrator only where useful.
- Spans are valid for one hash. When a file changes, each layer notices for itself.
- When extraction runs (PDF to text, LaTeX parsing), spans refer to the extracted text in the cache, with a mapping back to the source (page, line).

### 5.4 Layer contract

```python
class Layer(Protocol):
    name: str                       # stable identity; version is separate
    roles: set[Literal["generate", "refine"]]
    def applies(self, scope) -> bool: ...
    def status(self, scope) -> LayerStatus: ...   # origin, granularity, coverage, query format, cost
    async def search(self, query, scope, budget, candidates=None) -> list[Hit]: ...
    async def build(self, units, cancel: asyncio.Event) -> None: ...
```

- **generate** produces candidates from the whole scope; **refine** scores only given candidates.
- Hits carry `(hash, span, excerpt, signals[])`. Signals are names.
- Each layer declares applicability (glob, MIME type, language) and the **query form** it accepts: free text, regex or substring, structural pattern, SMILES. The agent chooses or translates.

### 5.5 Signals as tags, meaning in the status

```
hit:     {"id": "a1b2…", "span": [120, 480], "excerpt": "…",
          "signals": ["bm25", "summary"]}

status:  summary → derived, granularity: file,  coverage: 12 %
         bm25    → in text, granularity: chunk, coverage: 100 %
```

- Hits stay small. Origin, granularity, and coverage are described once per layer in the status, one line each.
- A new layer is a new name. The engine knows no semantics; the agent interprets signals through the status.
- Names are stable and unique. They are part of layer identity, and logs and agent habits depend on them.

**Meaning arises from combinations:**

| Signals on a span | Reading for the agent |
|---|---|
| text layers only | The passage states it literally. |
| derived layer only (summary, generated question) | Abstraction, inference, or paraphrase not directly stated in the text. A pointer for where reading pays off. |
| both | Strongest evidence, directly quotable. |

A derived signal carries optional pointers to the source spans it was produced from, so the agent can verify in the second stage.

### 5.6 Metadata and derived views

- **Metadata is enriched at match time**, not stored in layers. Path, size, modification time, language, and enclosing symbol come from the current file state. No schema lives in the layers, and a new field costs no re-index.
- **Summaries, generated questions, and metadata that should be searchable are separate layers.** A metadata layer can act as a scope generator: it returns hashes that become `candidates` for content layers.
- Expensive derived texts (LLM-generated) are produced once in the central cache, keyed by hash, prompt version, and model, and can feed several layers.
- Pre-search attributes (language, test, directory, age) come from the glob layer as scope.

## 6. Layer Catalogue

Ordered by overall cost, build plus query. **G** = generates candidates, **R** = refines given candidates.

| # | Layer | Tools | Build | Role |
|---|---|---|---|---|
| 1 | File names | `fd`, glob | none | G |
| 2 | Exact / regex text | grep → `ripgrep` → `ugrep` (fuzzy, boolean) | none, always current | G |
| 3 | Text across formats | `ripgrep-all`, `pdfgrep` | cached extraction | G |
| 4 | Fuzzy rescoring | `rapidfuzz` | none | R |
| 5 | Structural search | `ast-grep` | none (parses at run time) | G |
| 6 | Trigram index | `csearch`/`cindex`, Zoekt, `fcs`, FTS5 trigram, `ugrep-indexer` | cheap | G |
| 7 | Word-level BM25 | FTS5, `bm25s`, Tantivy, Recoll, `qmd search`, with custom tokenization (camelCase, LaTeX) | cheap | G |
| 8 | Symbol / AST index | tree-sitter (definitions, callers, spans) | medium, per language | G |
| 9 | Formula structure | Tangent-CFT, SSEmb, Approach0 | medium, specialized | G |
| 10 | Learned sparse | SPLADE, BGE-M3 sparse | encoder run | G |
| 11 | Static embeddings | Model2Vec + matrix | lookup, no transformer | G |
| 12 | Dense bi-encoder | BGE-M3, Jina, Qwen3-Embedding, SPECTER2 (abstracts) | encoder run per chunk | G |
| 13 | Derived views | summaries, generated questions, formula descriptions, context lines | LLM call per unit; cheap to query | G |
| 14 | Late interaction (exact MaxSim) | PyLate / fastembed encoders, sidecar token vectors | encoder run | G + R |
| 15 | Multiple late-interaction models in parallel | e.g. BGE-M3 multi + domain-specific ColBERT | doubled cost, decorrelated by choosing different specializations | G |

**Beyond the ranking**: cross-encoder and LLM rerankers evaluate query and text jointly. The agent itself is the final, richest layer.

**Reading the table**

- Layers 1–5 need no index. They are always current, so they double as the **change detector** for the layers behind them and as the fallback.
- Layer 13 is expensive at build time and as cheap as layer 12 at query time.
- Refine roles fit short timeboxes because they only load candidates; the generate role is what closes vocabulary gaps and takes time.
- Spans from a structural layer give natural boundaries (function, class, call site); text layers give lines. Their intersection assigns a text hit to its enclosing symbol.

## 7. Late Interaction Without a Heavy Index

Because update cost outranks lookup speed, the late-interaction layer is **exact MaxSim over stored token vectors**, with no centroid index.

- One sidecar per content hash: token vectors of all chunks in sequence (fp16, memory-mapped) plus chunk boundaries and spans.
- An update is: write a new sidecar, drop the old one. A changed chunker or model is a new sidecar. No distribution drift, no ID shifts on delete, no cold-start thresholds.
- Scores are exact, free of candidate-selection approximation.

```python
sim   = q @ block.T                              # (nq, N_block), block as float32
best  = np.maximum.reduceat(sim, starts, axis=1) # max per chunk
score = best.sum(axis=0)                         # sum over query tokens
```

- Process in blocks. Convert fp16 to float32 per block, or use Torch (GPU trivial).
- Loop order: candidates from earlier layers first, then the rest of the scope by usage. The loop is naturally abortable; on timebox end the layer returns what it has scanned and reports coverage.
- Levers for size: token pooling (factor 2–3 at small quality cost), token pruning, int8, small-dimension models.
- Encoders: PyLate's `models.ColBERT`, fastembed (ONNX, CPU), FlagEmbedding (BGE-M3 gives dense, sparse, and multi-vector in one pass). Starting points: LateOn-Code-edge / LateOn-Code for code, a general ColBERT such as GTE-ModernColBERT for prose and papers.
- If a single corpus ever needs an index, one can be added for that corpus alone behind the same contract.

## 8. Scheduling, Timebox, and Build Behavior

- **The agent sets the policy.** It knows the corpus size and the goal: a small precise set, a large candidate set, or the fastest possible run. The engine executes without judging.
- **Status call** (cheap, cached): corpus size in scope, available layers with role, coverage, accepted query format, estimated latency per scope size.
- **Search parameters**: timebox, target size (k or token budget), emphasis (precision vs. recall), allowed or excluded layers.
- **Layer protocol per call**: ran, aborted, skipped, number of contributions. A null result is therefore distinguishable from "did not run".
- **Anytime behavior**: the fastest layer (e.g. grep) starts; each further layer adds, corrects, and refines until the timebox ends.
- **Lazy background building**: free CPU/GPU is used until the timebox or the next request. Each unit is written atomically (temp file, rename, manifest entry), so an abort loses at most the current unit and a partially built layer is immediately usable. Small batches keep abort latency in the millisecond range.
- **Build priority**: files from the last request's candidates and fetches first, then the remaining scope by usage frequency, then the rest of the corpus.
- **Synergy between calls**: a second call finds layers filled by the first (for example, newly indexed BM25). Per-layer, parameter-keyed caching applies: query embeddings, candidate lists, refinement results per candidate set, and empty results. Keys use layer, layer version, normalized query, parameters, and per-file hash.
- **The layer list is stable across calls**; only the fill level changes.

## 9. Change Detection and Upserts

A shared concern with a single mechanism:

1. **Central file index**: path, hash, size, mtime, sequence number, deleted flag, plus the extraction cache. A monotonic sequence number is the cursor (not the timestamp). Deletions are tombstone rows that collapse repeated changes per path.
2. **Lazy detection**: the always-current text layer is the detector. When it resolves a match, a `stat` comparison against the index detects the change; a hash over the read bytes confirms it. The changed file is queued for all following layers with high priority. A cheap `stat` pass over the directory listing the text layer produces anyway also catches files without a lexical hit.
3. **Per-layer cursor**: each layer asks the central index for changes since its last update and refreshes its own chunk view. The cursor is stored with the layer version; a new version restarts lazily. Application is idempotent over `(layer, hash)`.
4. **Hierarchical option**: an index file balanced by size, with a parent index mapping time and hash of the child index files (Merkle-style, content-defined boundaries). Snapshot diffs yield added, changed, and removed entries without tombstones; the structure doubles as a versionable and synchronizable file format.
5. **Per-layer upsert contract**: `apply(add, remove)`, idempotent and abortable. FTS5 deletes and inserts per hash; vector sidecars are written atomically; AST results are sidecars per hash.
6. **Hash from the bytes actually read**, so spans and hash always agree. Debounce editor save patterns.

## 10. Storage Layout

- Content-addressed storage, distributed by hash prefix over subdirectories.
- A mirror tree of tiny manifests (path → hash, mtime, size) provides subtree scoping; a rename only changes a manifest.
- Immutable segments with tombstones and occasional compaction; no in-place rewrites.
- Rule-based balancing: target segment size, split or merge by size, cut first along directory and corpus boundaries, then by size.
- Optional per-segment centroid or summary vector for skipping segments before loading.
- Columnar formats (Parquet in a Hive layout, or Lance) are suitable for chunk tables, derived views, and vector columns; DuckDB/Arrow read them directly.
- Model ID and chunker version are part of each segment header.

## 11. Multi-Index, Multi-Space Generalization

The same philosophy covers multimodal and heterogeneous data.

- **Multi-vector shifts learning into query processing.** The pipeline does not need to understand the use case; it needs to represent a semantic space. A query term finds its best counterpart wherever it lives.
- **Any number of representation spaces** can coexist (text embeddings, fingerprints, structural trees, image patches, graph encodings). The only criterion is that the query can be posed in a compatible space; an approximation suffices.
- **Space selection by the query** replaces fusion and avoids the incommensurability of scores across spaces.
- **Probing** allows unlearned correlation: if the signal in the query and in each index can be quantified, similarity across adjacent spaces can be estimated. Quantification tools are margin and gap, comparison against a null model per space (query performance prediction), local hubness correction, and an extreme-value p-value per hit in the BLAST manner. Spaces without signal drop out automatically.
- **Aggregation by object identity**: the object that stands out in several spaces receives the combined evidence.
- Example: a molecule by name and its derivatives by structure. The name resolves in a text space, the structure in a fingerprint or graph space; the identity of the object joins them, and no use-case-specific mapping has to be hard-coded.
- Established components (inverted index, trigram, BKD trees, fingerprints, spatial indexes) are all layers of the same kind.

## 12. Domain Hints

- **Code**: tree-sitter chunks (function, class, method) with signature and docstring; identifiers split into camelCase/snake_case for the lexical layer; code-specialized late-interaction model as an additional layer.
- **Scientific papers**: prefer LaTeX sources (e.g. arXiv) over PDF extraction. Never split formula or theorem environments. For formulas, use normalized LaTeX tokens for the lexical layer (macros resolved, variable names replaced by placeholders), structural formula layers where formulas dominate, and LLM-written descriptions as a derived layer. SPECTER2/SciNCL are useful on abstract level.
- **Prose**: paragraph or chapter chunks; summary and question layers for abstract queries.
- **Domain vocabulary**: glossary and abbreviation expansion, corpus-derived thesauri (co-occurrence, fastText on the corpus), and agent-side query expansion bridge author language and user language.
- **Rare terms**: exact-match layers (grep, BM25, trigram) cover them by design; vector quality for rare terms matters only for the layers that need it.

## 13. Cost and Latency Reasoning

- Raw output enters the context once and is read again on every following turn. A filtered view stays small, and only requested details are added. The saving grows with the number of follow-up turns.
- Smaller context reduces KV-cache traffic in every decode step, which is the dominant cost at long contexts. Filtering outside the model also removes the prefill of the raw output.
- The extra retrieval turn has a fixed floor (round trip, time to first token). It is repaid in longer sessions and with larger outputs.
- Logging per call: raw tokens, view tokens, share of requested IDs, number of follow-up turns. Share of derived-only hits that are requested and confirmed or discarded measures the reliability of derived layers.

## 14. Implementation Hints

- **Language and stack**: Python. `ripgrep --json` via subprocess, `rapidfuzz`, SQLite (WAL) with FTS5 for BM25 and trigram tables, `bm25s`, `tree-sitter`, `ast-grep` bindings, `pyarrow`/`duckdb`, `model2vec`, NumPy/Torch, PyLate and fastembed as encoders, `watchfiles` optional, `asyncio` with deadline and cancel event per layer.
- **Single writer** to the central file index (the scanner); layers read.
- **Atomic writes** everywhere; units small; cancel checked between units.
- **Query formats per layer** are advertised in the status; the agent translates.
- **Dependency pinning** for engine and encoder versions; model ID plus chunker version identify a layer version.
- **Reference tooling** for each layer is listed in section 6; replacements stay behind the contract.
- **Paraphrase and rare-term test sets** can be generated from the corpus itself (LLM questions with known source spans) and extended by real agent traces.

## 15. Why This Is Generic

- Corpus-agnostic: any file type that some layer can read participates; others still appear through text layers.
- Model-agnostic: a thematic model becomes one more layer; a layer may handle only code or only a JavaScript AST.
- Method-agnostic: lexical, structural, sparse, dense, multi-vector, and derived views live side by side.
- Tool-agnostic: the two-stage filter applies to any tool output, including external APIs.
- Hardware-agnostic: from CPU-only text layers to GPU late interaction, under one timebox concept.

## 16. Lineage and Related Work

The building blocks have established precedents that this project combines:

- **Data fusion and metasearch** (CombSUM/CombMNZ, RRF); **inference networks** (INQUERY/Indri); **federated search** (resource selection, result merging); **anytime ranking** and cascades under budget.
- **Tool-output compression with ID-based retrieval** (context-compress, mcp-context, replicant-mcp, claude-mem progressive disclosure, MCP resources pattern).
- **Lexical-first, semantic-later responses** (ephemeral-buffer-mcp with semantic coverage pending, sifs, Cursor's trigram index plus Merkle-synced semantic index).
- **Agentic search** with grep and glob in coding agents.
- **Late interaction** (ColBERT, ColBERTv2, PLAID, ColPali), **token pooling and pruning**, **MUVERA**, **late chunking**, **BGE-M3**, **Model2Vec**.
- **Query performance prediction**, **BLAST E-values**, **hubness reduction**.

What this manifest adds is the generalization: an open set of independent layers with their own granularity, one minimal contract of spans and signal tags, caller-defined timebox and quality targets, lazy change detection through the always-current text layer, and the agent as the only aggregator.

## 17. Glossary

- **Layer**: independent signal producer with its own chunking, model, and index.
- **Signal**: a named contribution to a hit, interpreted by the agent via the layer status.
- **Span**: `(hash, start, end)` interval, the shared coordinate.
- **Generate / refine**: candidate production vs. rescoring of given candidates.
- **Coverage**: share of the applicable scope a layer has processed.
- **Timebox**: the caller's time budget for a call.
- **Derived view**: model-produced text (summary, question, description) indexed as its own layer.
- **Two-stage retrieval**: filtered view with IDs first, details on request second.
- **Probing**: querying several spaces and estimating signal strength per space.
