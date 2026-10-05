# Isolated Semantic Space as a Navigation Tool for LLM Agents

> A systematic elaboration of the conceptual model. This document extends and formalizes the line of reasoning — it is not merely a summary, but a thesis in its own right.

## 1. Initial Observation: The Granularity Continuum

Retrieval representations do not fall into discrete categories ("dense" vs. "multi-vector"); rather, they lie on a continuum governed by two coupled parameters:

- **Segment size** (how finely text is cut: document → paragraph → sentence → token)
- **Vector size/count** per unit

A dense index using very small chunks structurally converges toward multi-vector search. In this framing, ColBERT is simply the point of maximal granularity (one vector per token) — the product of a fixed convention rather than a deliberate design choice.

**Extension developed in dialogue:** The continuum has at least two further, orthogonal axes:

1. **Contextualization**: Is each unit encoded in isolation, or jointly with the rest of the text? (Standard chunking = isolated; ColBERT / Late Chunking = jointly encoded, then split)
2. **Interaction/Aggregation**: One query vector against many document vectors (max) vs. many query vectors against many document vectors (sum of maxima, MaxSim)

**Central thesis:** *These axes are not fundamentally distinct — they are mutually imitable through training design.* Query-splitting combined with fine-grained chunking and end-to-end training of the aggregation step approximates ColBERT closely. What remains irreducible is the *isolation barrier* at encoding time (solvable through Late-Chunking-style methods) — not the architecture itself.

**Confirming evidence from the field:** MUVERA moves in the opposite direction (multi-vector → compressed into a fixed vector for fast search), demonstrating that this axis is real and can be traversed in either direction.

## 2. Two-Level Model: Precision vs. Navigability

A central distinction that emerged from the dialogue:

| Level | Question | Responsibility | Metric |
|---|---|---|---|
| **1. Single call** | How precise and context-economical is a single retrieval result? | Scoring quality, chunk/span granularity, reranking | Precision@k, tokens per answer, latency |
| **2. Overall task** | How well can an agent navigate purposefully across multiple turns? | State feedback, controllable breadth, determinism, set operations | Steps to target chunk, total context tokens |

**Core thesis:** The RAG system should not try to anticipate what the LLM can already do better within dialogue — e.g., building synonym bridges or rephrasing terms. Instead, it should function as a *precise, iterable tool*. This separation defuses the common objection "BM25 doesn't know synonyms" as a knockout argument against lexical/deterministic indexes — because bridging synonyms is a task for the agent across multiple steps, not a task for the index.

**Consequence:** Every retrieval tool must be evaluated separately on both levels. An improvement at Level 1 (e.g., a better embedding model) can be worthless — or even harmful — at Level 2 if it fails to provide state feedback.

## 3. The Isolated, Corpus-Relative Semantic Space

### 3.1 Basic Idea

Rather than applying a large, pretrained embedding model to a small knowledge base (KB), a **standalone, small vector space is constructed exclusively relative to the KB** (e.g., via word2vec/fastText-style training from scratch, or via distillation). Rationale:

- The KB is orders of magnitude smaller than any general vocabulary → the index stays small, search stays fast.
- Semantics is *differential* — meaning arises from contrasts within the KB, not from global world knowledge.
- Rejection ("this lies outside the space") is explicitly the **goal, not a deficiency** — for a medical KB, philosophical queries should sensibly remain unresolved.

### 3.2 The Radius-Ring Model

A construction developed to encode evidence/importance:

- **Initialization at the boundary** (high-dimensional sphere surface) rather than at the center or randomly distributed throughout: random boundary points in high dimensions are nearly orthogonal to one another → the geometric equivalent of "unconnected" / "lacking evidence."
- **Movement toward the center** proportional to frequency/importance during training.
- Result: a **bandpass in radius** (related to Luhn's 1958 observation on term frequency):
  - **Center**: stop words, overly general terms, scoring-wise irrelevant/down-weighted.
  - **Ring (middle band)**: the actually relevant, information-bearing region.
  - **Boundary**: unsubstantiated/rare terms, lacking reliable semantics, deliberately kept "outside."

**Important clarification from the dialogue:** Radius, ring, and thresholds are **search-navigation parameters, not indexing parameters**. They are progressively widened or narrowed at search time (ranking cutoff), without reindexing. The ability to *react* to too few or too many hits resides with the navigator (the LLM), not within the index itself.

**Separation of semantic roles:** Operators such as "not," "for," "without" are **not** encoded as directions within the semantic space (antonyms share contexts and lie geometrically close together — directional shift is unreliable here). Instead, they are implemented as **explicit navigation operations at the set level** (intersection, subtraction, Rocchio-style feedback). Sentence-level relational semantics (e.g., "drug for X" vs. "drug against X") is not resolved by the space itself, but verified by the LLM when reading candidate results.

### 3.3 The Adapter as an Exchangeable Bridge Layer

Since a strictly isolated space naturally cannot unlock KB-foreign phrasings (user/lay language, unlearned relations such as "greater than"), this task is explicitly **delegated** to an exchangeable adapter layer:

| Adapter Type | Mechanism | Data Requirement |
|---|---|---|
| Standard world knowledge | LLM formulates directly in KB-proximate terms, iteratively refined via neighbor feedback | none |
| Reference navigation | Glossary/overview as anchor | curated |
| Analogy/navigation | Pivoting from a known anchor | none, locally bounded |
| Mapping | Learned mapping from user space → KB space | anchor pairs |
| Trained adapter | Model/MLP, contrastively trained | pairs; loses interpretability |

The space itself remains unchanged; the adapter is swapped experimentally. This renders the semantic space a stable core with a variable outer layer — a deliberate separation of responsibilities.

## 4. Why Multi-Vector (ColBERT-style) Was Chosen as the Carrier Structure

The decisive justification that emerged in dialogue: **MaxSim decomposes additively into per-query-token contributions.** This serves both levels simultaneously:

- **Level 1:** The query-token → document-token alignment reveals *which* span of text carries the match → span extraction instead of whole chunks → smaller context without a second model.
- **Level 2:** Per-token scores reveal which part of the query is "covered" (high contribution) and which is not (unknown term/boundary) → native state feedback, the foundation for the radius/ring mechanism, and the basis for targeted sharpening or broadening of individual query terms at search time.

Learned sparse representations (SPLADE) exhibit the same additive decomposability — with even more direct interpretability, since they operate over actual vocabulary terms rather than latent token vectors. This remains an open and natural competing candidate.

## 5. Practical Constraints

- **No persistent server process**: on-demand execution, index files rather than a database/metadata layer (model: ColGREP).
- **Avoid background load**: if preloading is necessary (model/batch), it must run only as a short-lived, self-terminating daemon process (idle timeout) — no service, no autostart.
- **Parallelization via shards rather than a monolith**: one small index per KB/topic; parallel queries across multiple shards; merge of top-k results. Requires an identical model and identical thresholds across all shards.
- **Frequent corrections as the norm, not the exception**: index design must support upsert without full rebuild/requantization (tombstones + periodic compaction rather than immediate reclustering); compression is deliberately deferred or avoided, since it introduces drift sensitivity under incremental updates. Speed should come primarily from centroid pruning and sharding, not from quantization.

## 6. Condensed Formula of the Overall Concept

> A small, exclusively corpus-relative trained multi-vector space, whose vector radius encodes evidence/importance (boundary = unsubstantiated, center = overly general, ring = relevant), serves an LLM as an inspectable, file-based, on-demand-runnable navigation tool — not as an answer system. The additive decomposability of MaxSim delivers both precise span localization (Level 1) and native state feedback for iterative navigation (Level 2). The bridge to KB-foreign user language lies deliberately outside the space, in an exchangeable adapter layer carried primarily by the LLM itself (world knowledge, iterative feedback).
