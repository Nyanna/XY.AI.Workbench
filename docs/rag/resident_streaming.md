# Resident Streaming Architectures for ColBERT-MaxSim RAG: A Conceptual Model

> *Extrapolated from the reasoning line developed in the dialogue "ONNX alternatives discussion."*

## Abstract

Standard retrieval-augmented generation (RAG) stacks inherit their serving model from general-purpose ML frameworks: monolithic runtimes (ONNX, Hugging Face `from_pretrained()`) that bundle tokenization, weights, and execution into a single opaque unit, and a request/response lifecycle that loads, infers, and discards state per call. This paper develops a conceptual architecture that rejects both defaults. We argue for (1) disaggregating the inference stack into independently composable components (tokenizer, weights, kernels), (2) replacing request/response serving with a permanently resident computation model driven by producer-consumer ring buffers, (3) treating ColBERT-style multi-vector retrieval as pure post-hoc geometry over already-materialized points rather than a model-bound operation, (4) replacing artificial chunking with a native sliding-window representation that yields multi-vector structure as an emergent property rather than a constructed one, (5) reframing embedding vectors as signals amenable to frequency-domain compression (an explicit JPEG/JPEG2000 analogy), and (6) exploiting the fact that weights and activations are mathematically indistinguishable tensors to enable lightweight, resident, contextual-bandit-style online adaptation. We position the resulting architecture relative to three established serving paradigms and argue that the gap it fills is a consequence of field immaturity and tooling path-dependence rather than technical infeasibility.

## 1. Introduction

Contemporary RAG systems are built on a stack of abstractions optimized for generality, portability, and ease of adoption — not for control over hardware execution. This is a reasonable trade-off for most applications, but it forecloses a specific design space: systems in which indexing, retrieval, and adaptation are treated as *concurrent workloads against a single, long-lived computational state*, rather than as sequential, stateless batch jobs.

This paper does not propose a new model or a new benchmark. It develops, in a structured way, the architectural consequences of a small number of first-principles observations:

- A model is a composition of three independent concerns, not an indivisible object.
- If weights are kept resident, the only meaningful cost left is data movement, not setup — which changes the economics of streaming versus batching.
- Multi-vector similarity (ColBERT/MaxSim) is geometry, not inference, once the vectors exist.
- A vector is a sequence of coefficients, not merely a point — which licenses frequency-domain treatment.
- Weights and activations are tensors distinguished only by *update frequency*, not by type — which licenses online mutation of "fixed" parameters.

Each section below develops one of these observations and extrapolates its architectural consequences. Section 9 integrates them into a single system diagram.

## 2. Disaggregating the Inference Stack

The core motivation is control through decomposition. ONNX, PyTorch's `from_pretrained()`, and the Hugging Face Transformers stack conflate three logically independent concerns:

1. **Tokenizer** — a CPU-side string-to-ID mapping.
2. **Weights** — the actual learned state.
3. **Execution graph / kernels** — the decision of which operation runs when, on which hardware, with which memory layout.

The rejection of ONNX is not a matter of taste; it follows directly from the fact that ONNX removes the third layer — kernel selection, scheduling, memory layout — from developer control. That layer is precisely the one this architecture depends on.

The consequence is a deliberate recomposition into three independently replaceable building blocks:

- **`safetensors`** — weights only.
- **`tokenizers`** — vocabulary/BPE only.
- **`candle` / `cudarc`** — execution only.

Under this view, the model is not a black box but an explicit composition of parts, each of which can be inspected, replaced, or instrumented independently.

## 3. Resident State versus Request/Response Serving

The central departure from standard ML serving — load model, run inference, unload (or at minimum: batch in, batch out) — is **residency**:

- Weights remain permanently in VRAM/RAM.
- There is no per-request start/stop cycle.
- Multiple models can be resident simultaneously, memory permitting.

This immediately creates a concurrency problem: if the model lives permanently, multiple workload types (indexing, query embedding, MaxSim scoring, potentially training) must compete for the same resident state without blocking one another.

The answer is not classical multithreading but a **producer–consumer architecture over ring buffers**:

| Buffer | Producer | Consumer | Content |
|---|---|---|---|
| Indexing queue | CPU (tokenizer / doc stream) | GPU (embedding kernel) | Token IDs / window positions |
| Embedding buffer | GPU | Index / disk | Finished token vectors + position |
| Query queue | CPU | GPU | Query token IDs |
| Similarity buffer | GPU (MaxSim kernel) | CPU | Scores per document/position |

This design eliminates kernel-launch overhead not through batching alone but through permanence: if the model and its kernels are never torn down, the only remaining cost is the actual data payload — token IDs in, vectors out — not setup. Batching becomes *optional*: a continuous stream is equivalent to, or better than, batched execution as long as the GPU never idles.

**Extrapolation.** This is not specific to embedding/MaxSim workloads. It is a general *resident-compute pattern* applicable to any pipeline where (a) an expensive state (weights) persists for a long time, and (b) many small, heterogeneous requests operate against that state. Systems such as Ollama and llama.cpp realize variant (a) for pure token generation. The architecture developed here generalizes that pattern to multiple concurrent workload types operating against the same resident state.

## 4. MaxSim as Geometry over Already-Learned Points

A key observation is that embedding and inference are both forward passes; the only difference is whether the output is recursively fed back as input (inference) or not (embedding). This dissolves a categorical distinction that standard frameworks impose — embedding as a batch task, inference as a streaming task. There is no fundamental reason embedding could not also be treated as a stream; it is merely convention.

Once this distinction collapses, MaxSim itself is revealed to be fully decoupled from the model: it is pure vector geometry (matrix multiplication plus max-pooling) over points that already exist in space. The model is required only for two operations — document encoding and query encoding — and can disappear from the path afterward. This is what makes index lookup and scoring arbitrarily repeatable without the model being present.

**Extrapolation.** Since a vector is a *position* in space, not a *stored distance*, every similarity operation is retroactive and model-context-free, but *population*-context-dependent (it depends on which other points currently exist). The index therefore stores positions; every new query only defines a local neighborhood structure through comparison. This distinction — position versus relation — underlies all subsequent compression and hierarchy ideas in this paper, because only position, not relation, needs to be compressed.

## 5. From Chunking to Sliding-Window Representations

Arguably the most consequential idea in this line of reasoning: replace artificial chunking with a fixed token window that slides continuously over the entire document. Each window position produces its own forward pass, tagged with an exact document offset.

This yields several properties that are usually constructed artificially in the literature but emerge here for free:

- **Multi-vector representation without construction.** Standard multi-vector RAG manufactures extra vectors artificially (summaries, hypothetical questions, keyword extraction). The sliding window produces the same property — a token having multiple context-dependent representations — automatically from window overlap, with no additional engineering or heuristics.
- **Exact positional fidelity.** Because no artificial cuts are made at chunk boundaries, every representation remains bound to a genuine document coordinate. Retrieval returns not an approximate chunk but a *position*.
- **Natural semantic continuity.** The sequence of vectors across window positions traces a continuous, dense path through embedding space that follows the text's local semantic focus. This is no longer a discrete set of points but a continuous signal over document position.

**On why this is not standard practice.** This is not a claim of technical impossibility but a diagnosis of path dependence in evaluation culture: chunk-plus-single-vector representations are "good enough," benchmarks are calibrated to chunk granularity, and production systems carry sunk cost in chunk-based pipelines. The diagnosis — *the community settled on a suboptimal default because it works* — is an inertia thesis, not a competence thesis.

## 6. From Vectors to Signals: A JPEG Analogy

The decisive step is to treat an embedding vector as a sequence of coefficients rather than an unstructured point coordinate, which licenses applying a frequency transform — analogous to the DCT in JPEG. Low frequencies encode a vector's coarse shape; high frequencies encode fine distinctions. This enables:

- **Progressive quantization, computed per vector**, rather than population-dependent (as in centroid/k-means clustering) or arbitrary (as in LSH-style hashing).
- **Robustness to corpus shift.** Since the transform depends only on dimensionality (fixed), not on the distribution of the population, frequency bands remain stable as the corpus changes or grows — a structural advantage over centroid-based indices (which must be recomputed under corpus growth/drift) and over PLAID-style methods with their known update limitations.
- **Coarse-to-fine cascading.** Level 0 uses only low-frequency coefficients for a fast MaxSim pass over the full candidate set; higher levels progressively refine only the surviving candidates.

### 6.1 Two Orthogonal Frequency Axes

The frequency transform as initially framed applies only along the embedding dimension (per vector). The sliding-window construction from Section 5, however, introduces a second axis: window position along the document. Carried through consistently, this yields a two-dimensional spectral structure — one that is in fact closer to the original JPEG analogy than first apparent, since JPEG transforms 2D pixel blocks, not 1D sequences:

- **Axis 1 (dimension):** frequency decomposition within a single vector — progressive accuracy per point.
- **Axis 2 (position):** frequency decomposition of the vector sequence across document length — progressive granularity of semantic drift along the text.

The result would be a **semantic spectrogram** of the document: low positional frequencies describe coarse thematic blocks (an automatically emergent sense of chapters/sections, without any chunking step); high positional frequencies describe local semantic jumps at the sentence-to-token level. A coarse retrieval pass could match only against the low-frequency positional envelope (a few large candidate regions); a fine retrieval pass could then operate on high-frequency local variation within the surviving regions.

Since real documents are typically non-stationary (abrupt topic shifts rather than smooth waveforms), a classical FFT is likely suboptimal here. A **wavelet transform** — localized jointly in frequency and position, as in JPEG2000's departure from JPEG's block-based DCT — is the more consistent extension of this line of reasoning: it provides multiresolution information without the block artifacts and stationarity assumptions of a pure FFT. This is a natural, if not previously made explicit, continuation of the JPEG comparison itself (JPEG = DCT; its successor JPEG2000 = wavelet).

## 7. Cascaded Refinement as Multi-Resolution Retrieval

The relevant sense of "fractal" here is **resolution refinement through repetition of the same structure at multiple granularities** — not strict mathematical self-similarity. This yields a cascaded retrieval scheme:

```
Level 0: coarse window / low-frequency coefficients  -> broad candidate set, cheap
Level 1: finer window / more coefficients             -> reduced candidate set
Level 2: full resolution, full MaxSim                 -> final ranking
```

Each level is structurally identical (sliding window → embedding → filter); only the parameterization (window size, frequency bandwidth) changes. This is economically motivated: expensive full-resolution computation runs only on a heavily reduced candidate set. It is an implementation of cascade ranking, but one derived from the index and signal structure itself rather than from separate, independently trained re-ranking models.

## 8. Positioning: The Gap Between Established Paradigms

The current landscape can be characterized as three established poles with an open middle:

1. **Cloud LLM + dense retrieval** — centralized, no user-side control over residency.
2. **Local LLMs without specialization** (Ollama, llama.cpp) — resident, but generic and optimized solely for token generation.
3. **Framework RAG** (LangChain, LlamaIndex) — maximal abstraction, minimal control.

**The open middle:** specialized, local, resident, multi-workload architectures that run indexing, retrieval, and optionally online adaptation concurrently within the same memory space. This gap does not reflect a lack of community capability; it reflects that (a) local inference has only been a practically relevant field since roughly 2023, (b) Python's GIL structurally prevents genuine concurrency, and (c) frameworks are economically optimized for generalization rather than for control.

**Extrapolation.** This positions the Rust/CUDA architecture developed here not as a niche solution but as a temporally plausible first-mover pattern in a field that has not yet settled on standard architectures — comparable to the period before chunk-plus-single-vector representations became the de facto standard in dense retrieval.

## 9. The Parameter-Vector Duality and Online Adaptation

The most conceptually far-reaching point in this line of reasoning: mathematically, weights and activations are both merely vectors/tensors. The distinction between "parameter" and "vector/activation" is purely operational — it describes *when* something changes, not *what* it is:

- **Activations** change per input (every query, every document).
- **Weights** traditionally change only during training, then never again.

Once a model is resident, however, part of the justification for this distinction disappears. Residency means the weights already sit permanently in addressable memory. There is no longer a technical barrier to mutating them during operation — only a conceptual habit of treating them as finished.

This opens the door to **contextual-bandit-style online learning without full backpropagation**: incremental, local policy/weight updates driven by a reward signal, with O(1) memory per decision, streaming-compatible, with no hard separation between training and inference phases. The analogy to game AI — a pretrained policy whose influence is restricted to key decisions within a state engine, improving continuously over many hours of play — carries over directly:

- Training, embedding, and inference can coexist in the same resident memory space, orchestrated through the same ring buffers introduced in Section 3, with one additional queue for reward/gradient updates.

### 9.1 The Contextual Bandit as a Meta-Router over the Cascade

The coarse-to-fine cascade described in Section 7 (which window size, which frequency bandwidth, how many refinement stages) is itself a set of discrete per-query decisions — precisely the form for which contextual bandits are suited. Rather than fixing cascade parameters statically, a resident bandit could decide, based on query context (length, domain, prior hit quality), how deeply to cascade, and learn this routing incrementally from retrieval outcomes (e.g., click feedback, downstream LLM evaluation) — without touching the embedding model itself.

---

## 10. Unified Architecture

```
+--------------------------- Resident State (GPU+CPU) ---------------------------+
|  Tokenizer (CPU, `tokenizers` crate)                                           |
|  Weights (GPU, safetensors -> candle, mutable)                                 |
|  Kernels: embedding forward, MaxSim (MatMul+Max), FFT/wavelet transform        |
+----------------------------------------------------------------------------------+
        ^                 ^                    ^                      ^
        |                 |                    |                      |
   Doc stream        Query stream         Reward stream           Drift monitor
  (sliding window)   (on demand)      (contextual-bandit updates) (re-embed trigger)
        |                 |                    |                      |
        v                 v                    v                      v
  Index (Arrow/Parquet, versioned, hierarchical: coarse/fine, frequency bands)
        |
        v
  Cascaded MaxSim retrieval (Level 0 -> N, candidate filtering per level)
        |
        v
  (optional) bridge to the pylate stack as a reference/validation path, sharing the index format
```

## 11. Conclusion and Future Work

The architecture developed above follows from a small set of compounding decisions: decompose the inference stack into independently controllable components; keep model state permanently resident and stream data through ring buffers rather than loading/unloading per request; recognize that ColBERT-style multi-vector similarity is geometry over materialized points, not a model-bound operation; replace artificial chunking with a sliding window that produces multi-vector and positionally faithful representations as emergent properties; treat embeddings as signals subject to frequency-domain compression, extending naturally from FFT to wavelet transforms across two orthogonal axes (dimension and document position); and exploit the fact that weights are just tensors to enable resident, lightweight online adaptation via contextual bandits.

None of these steps individually requires new theory; their combination, however, describes a system that differs structurally from the three dominant serving paradigms in the field today. Open questions for future work include: empirical validation of the two-dimensional wavelet decomposition against standard ColBERT/PLAID baselines; quantification of the bandit's convergence behavior as a cascade router under realistic query-mix distributions; and characterization of the memory/throughput trade-offs of fully resident multi-workload serving under concurrent indexing and querying at corpus scale.
