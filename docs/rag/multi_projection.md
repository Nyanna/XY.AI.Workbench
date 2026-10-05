# Beyond Ranking: A Multi-Projection Methodology for Search Based on Transparent Representation and Confirmed Convergence

## Abstract

This thesis develops an argument that begins with a narrow technical complaint about trigram-based file signatures and ends in a general epistemological and architectural position on search systems. The argument proceeds in three movements. First, a classical objection to local frequency-based trigram filtering — that it produces false negatives — is shown to rest on a mischaracterization of the method: once the procedure is correctly specified as an overlap-threshold filter rather than a containment test, the "false negatives" turn out to be a deliberate and defensible scope boundary, not a defect. Second, this reframing is generalized: no representation of data — not trigrams, not embeddings, not even "raw bytes" — occupies a neutral zero point, because every representation is already a projection that discards information. If no representation is privileged, the only defensible strategy is to implement a *diverse* set of representations and to declare their projection parameters transparently, rather than to search for the "correct" one. Third, this architectural commitment is extended into the interaction model: because relevance is a private function of the user, and because a system cannot rank by a criterion the user has not made explicit, ranking is replaced by a two-phase, cache-backed exploration process whose success criterion is *monotone, user-confirmed convergence* rather than ranking efficiency under the Probabilistic Ranking Principle.

The thesis is thus: **a search system should be built as a federation of independent, parameter-transparent projections whose results are only ever unioned and contrasted — never ranked or filtered by one another — because convergence toward an unspecified information need can only be achieved through user-confirmed narrowing, not through system-side anticipation of relevance.**

## 1. Introduction

### 1.1 Motivation: The Trigram Method and Its Discontents

The starting point is a well-known technique for approximate file matching. For every file, a signature is built by counting all contained trigrams (3-character sequences). The most frequent 10% of trigrams are removed; the remaining 90% form a bitmap signature.

The classical objection to this procedure is straightforward: removing the most frequent trigrams *locally* (i.e., per file) produces false negatives, because a trigram removed in file A as "too frequent" might be preserved in file B, where it is not locally frequent. A query built from that trigram cannot distinguish between "trigram absent" and "trigram removed" — the 0-bit is semantically overloaded.

Three classical remedies suggest themselves:

- **Global document frequency (df)** instead of local counting — rejected, because a global corpus-wide frequency table presupposes a linguistic/structural homogeneity across the corpus that frequently does not hold.
- **Explicit removal set R** carried as side information — correct, but costly in storage.
- **Winnowing / minimizers** — a deterministic, content-independent selection scheme with formal guarantees.

Each of these remedies treats the local removal as a *flaw to be patched*. This thesis argues that this framing is itself mistaken.

### 1.2 Thesis Statement

> The trigram method is not a faulty containment test; it is a correctly scoped overlap-threshold filter. Generalizing this insight — that every retrieval representation has a legitimate, declarable scope rather than a universal correctness claim — leads to an architecture in which multiple independent projections are unioned without ranking, and in which progress toward the user's (possibly inexplicit) information need is measured by monotone, user-confirmed convergence rather than by the predictive accuracy of a system-estimated relevance ranking.

### 1.3 Structure of the Argument

Chapter 2 reformulates the semantics of the trigram procedure and shows that its "false negatives" are a scope boundary. Chapter 3 generalizes this into a multi-channel retrieval architecture with no filtering between channels and no ranking of results. Chapter 4 grounds the refusal to rank in a broader epistemological claim: there is no neutral representation, hence no neutral ranking. Chapter 5 formalizes the resulting space of projections as a measurable object with axes, distances, and gaps. Chapter 6 operationalizes this into a two-phase interaction protocol between client and system. Chapter 7 extends the architecture with a cache that is itself a searchable object. Chapter 8 proposes a convergence criterion as the explicit alternative to the Probabilistic Ranking Principle, together with a scaling model and testable hypotheses. Chapter 9 positions the proposal against established work. Chapter 10 concludes.

## 2. From Containment to Overlap-Threshold Filtering

### 2.1 The Decisive Reframing

The pivotal move of this thesis is to reject the implicit specification under which the trigram method was being judged. The procedure is **not a containment test** ("is X contained in file D?"). It is an **overlap-threshold filter**:

> d is a candidate if |T(q) ∩ S(d)| ≥ θ — with no ranking and no sorting implied.

Under this specification, "false negatives" are not an error but a **deliberate scoping** of the method's domain of applicability. Queries that consist predominantly of trigrams that are "background" for the target file — i.e., locally frequent, hence removed — are, in principle, not meaningfully separable by this method. This is not a bug to be fixed by a smarter removal rule; it is a statement about what question the method can answer at all.

**The starry-sky analogy.** One cannot search for a star in a patch of sky that consists of nothing but stars; the background must be subtracted before a threshold can separate anything at all. The trigram method's local removal step *is* that subtraction — performed, necessarily, relative to the file's own background, because no universal background exists (see Chapter 4).

### 2.2 The Effect of Removal, Reframed as Benefit

Reframed in this way, the removal step is not information loss to be minimized but a mechanism with a positive, quantifiable function:

- It lowers the *random* overlap between files that share a common corpus "baseline tone," making true hits stand out more clearly against noise.
- It is especially effective for files drawn from a similar corpus, because in that setting local and class-wide frequency are strongly correlated — the similarity of the corpus implicitly supplies the consistency that a global stop-list would otherwise need to supply explicitly.
- The removal rate (10%) has an interior optimum: too little removal yields no gain in discriminability; too much removal begins to strike selective, Zipf-tail trigrams and damages true hits more than it damages noise.

This reframing dissolves the original objection without requiring any of the three classical remedies: global df is unnecessary because the method never claimed corpus-wide uniformity; an explicit removal set R is unnecessary because the scope boundary is accepted rather than patched; winnowing remains available as an alternative point in the design space, not as a correction.

## 3. Architecture: Multi-Channel Search Without a Filter Chain

### 3.1 Five Independent Retrieval Channels

The reframing of Chapter 2 generalizes into an architectural principle: rather than seeking a single corrected procedure, the system runs several **independent, parallel projections** (retrieval channels), each scoped to what it can legitimately answer:

1. **Exact text search** — containment, no tolerance.
2. **Trigram/10% signature** — set overlap with threshold θ, frequency suppression.
3. **Multi-vector late interaction** (ColBERT family) — learned token similarity (MaxSim), knowledge external to the corpus (encoder training).
4. **AST structural search** — syntactic tree structure, invariant under identifier renaming and layout.
5. **Corpus-trained vectors** — structurally like (3), but the knowledge source is the corpus itself rather than an externally trained model.

### 3.2 Union, Not Filtering

No channel filters another; all run independently over the full corpus. The output is the **union** of all candidate sets, and every element carries a **provenance declaration**: which channel produced it, under which parameters, and which raw features support it (overlap value, MaxSim score, structural match).

### 3.3 Why No Ranking

The system does not rank the unioned results. Three arguments support this:

- **Relevance is a private function** of the person issuing the query. Any system-side ranking is only an estimator of that function, and cannot outperform the user's own application of it when the user is capable of applying it directly.
- **Ranking is impossible when the relevance function is not explicit even to the user** (the Anomalous State of Knowledge / berrypicking condition). Any anticipation of an unknown dimension is logically impossible, because the very measure of approximation to that dimension would already presuppose the dimension itself.
- **The sole exception** is a query that supplies its own evaluation function (an explicit scoring mechanism). In that case, "ranking" is merely delegated evaluation of a function the client already possesses — not anticipation of one it does not.

## 4. No Neutral Ground: An Epistemology of Representation

### 4.1 There Is No Zero Point

The central epistemological claim of this thesis is that the refusal to rank is a special case of a more general refusal: the refusal to privilege any single representation. Not only ranking, but **every representation** — the trigram, the bitmap, even "raw data" conceived as bytes or file boundaries — is already a projection, an anticipation of what matters. There is no neutral intermediate stage and no zero point; raw data is itself an encoding.

### 4.2 Consistency Instead of Absolute Soundness

A direct consequence is that the correctness of an index cannot be defined absolutely, only relative to a comparison *between* projection levels. One can speak of **consistency** between representations, but not of **soundness** in an absolute sense, because soundness would require a ground truth outside all representation — which does not exist.

### 4.3 Transparency as the Achievable Property

If neutrality is unattainable, the only property that remains achievable is **transparency of the projection**: an explicit declaration of operator, parameters, and the order of steps applied. Transparency replaces neutrality as the epistemic standard the architecture holds itself to.

### 4.4 Diversity as Strategy

Since every single representation necessarily discards something, the only consistent strategy is to **implement as diverse a set of representations as possible**, covering the theoretically available space of projections, rather than searching for "the right one." This is the epistemological justification for the five-channel architecture of Chapter 3, and it motivates the formal treatment of that space in Chapter 5.

## 5. The Space of Projections

### 5.1 A First Axis: Tolerance

The implemented procedures can be placed on axes. A first, one-dimensional axis is "tolerance / degree of abstraction from the literal character sequence":

| Method | Tolerance | Invariant |
|---|---|---|
| Exact | none | order and identity of characters |
| Trigram/10% | local, set-like | character adjacency, not position/order |
| Multi-vector | learned, continuous | token meaning, not spelling |

### 5.2 Four Axes

Adding AST search and corpus-trained vectors yields at least a four-dimensional overview:

| Axis | Exact | Trigram/10% | Multi-vector | AST | Corpus-vectors |
|---|---|---|---|---|---|
| Tolerance | none | set-like | learned | structural | learned |
| Structural binding | character | character | token | tree | segment |
| Knowledge source | none | none | foreign | grammar | corpus |
| Background suppression | no | explicit | implicit | no | implicit |

### 5.3 Measuring Diversity

"Even coverage" presupposes a metric. Since no privileged metric over projections exists, one is constructed from the **behavior of the methods themselves**: the distance between two methods is defined as the Jaccard distance of their result sets over a set of samples and the corpus. A new method is valuable to the extent that it is maximally distant from all existing ones (maximin / farthest-point selection). The rank of the resulting distance matrix estimates the effective dimensionality of coverage; redundancy shows up as low rank.

### 5.4 Interpolation

Within a single parameter family (removal rate, θ, n-gram length), interpolation is well-defined, since the parameter forms a continuum. **Between** method families, no canonical intermediate step exists; "interpolation" there means only that the union or contrast of result sets *approximates* an intermediate projection — valid only if the result space is sufficiently smooth, which is itself measurable: does set-distance grow with parameter distance?

### 5.5 Gaps in the Space

Gaps in the space become visible as pairs of files that are indistinguishable under every implemented method but differ in raw content. Identified open axes include:

- **Edit distance / fuzzy matching** (between Exact and Trigram),
- **Token normalization / BM25-style overlap** (between Trigram and Multi-vector),
- **Layout / compressibility** (Normalized Compression Distance),
- **Graph relations** (imports, references),
- **Time / provenance** (metadata, history).

## 6. The Two-Phase Interaction Model

The architecture is operationalized through a two-phase protocol between client and system.

### 6.1 Phase 1 — Exploration (unspecific)

- The client supplies a sample or a vague query, together with a target volume N and a distribution w over methods.
- The system returns, per method i, at most n_i = w_i · N candidates, at sampling rate n_i / |C_i|.
- If |C_i| ≤ n_i, the output is **complete** (rate = 1); this is explicitly reported, because completeness is qualitatively different from a sample — statements about *absence* become possible.
- Feedback per method includes |C_i|, n_i, rate, and seed, so that even incomplete sets remain interpretable.
- **Residual budget rule** (when a method does not exhaust its allotment): the default is **lapse** (the distribution is preserved); the alternative is **redistribution** (the volume is preserved, the distribution is skewed). The choice belongs to the client.
- **Sampling mode** is selectable: uniform (reveals dominant clusters) vs. diversity-oriented / farthest-point over signature distance (reveals the margins).
- **Deterministic sampling** (hash with seed) is used instead of randomness — reproducible and traceable ("more" yields a continuation of the same order).

### 6.2 Phase 2 — Specification (after sufficient explicability has been reached)

The client chooses among four paths:

1. **Refine the query** (new samples as contrast, positive or negative).
2. **Formulate scoring** (a client-authored evaluation function over delivered raw features; the system either evaluates it or returns raw data for client-side evaluation).
3. **Restrict** to specific methods, axes, or parameter ranges.
4. **Continue exploring** (other samples, resolution levels, signature maps).

### 6.3 Presentation Rule

Results are presented in a **fixed, meaningless order** (e.g., alphabetical by method identifier), with equal rank given to every entry. Any form of highlighting — including sorting or emphasis on multiply-confirmed hits — would already constitute a weighting introduced through the back door, in violation of the principle established in Chapter 3.3 and Chapter 4.

## 7. The Cache as an Inspection Space

### 7.1 Separating Search from Sampling

The cache stores complete search runs — candidate sets, raw features, provenance — not merely the selected sample. This separates:

- the **search run** (expensive, performed once), from
- **sampling and scoring** (cheap, repeatable arbitrarily often against the cache).

### 7.2 Consequences

- Search runs themselves become a searchable object — the same projection principles (contrast, overlap, mapping) can be applied to earlier runs.
- The **cache key** is (corpus state, method version, parameters); otherwise projections would silently intermix. Staleness is **flagged**, never silently replaced.
- The **retention policy** (what is discarded under storage pressure) is itself an anticipation and must therefore be either declared explicitly or delegated to the client (pinning).
- **User judgments** (confirmation / rejection) are recorded with timestamp and round, and remain **revisable** without requiring recomputation.

## 8. Convergence as a Criterion for Abandoning Ranking

### 8.1 Against the Probabilistic Ranking Principle

The Probabilistic Ranking Principle justifies ranking as optimal under uncertainty and bounded user attention. This thesis proposes a competing criterion that permits the exhaustive user evaluation that the architecture of Chapters 3–7 implies:

> Exhaustive evaluation by the user is acceptable **as long as every iteration refines the search space by a step, rather than anticipating something the user has not confirmed.**

### 8.2 Requirements for Legitimate Convergence

- **Monotone refinement:** what has been confirmed stays confirmed; what has been rejected stays rejected (a guarantee enforced by the cache of Chapter 7).
- **Confirmation is the only legitimate source of narrowing.** Contrasts derived from user judgments ("these features separate your confirmed from your rejected samples") are *suggestions*, never automatic applications.
- **Stagnation detection:** the system can detect and report when confirmed and rejected samples remain indistinguishable across all views — a finding about missing coverage, not a value judgment.

### 8.3 A Scaling Model

Two quantities govern convergence:

1. **Corpus size N:** a lower bound on the number of rounds is ≈ log₂(N/k) bits, *if* each round divides the candidate set by a roughly constant factor (clean separation by the projections). If this fails, the number of rounds grows linearly with N.
2. **Representation distance d:** the distance between the dimension the user actually means and the nearest implemented projections, measured via result-set difference.
 - small d → convergence within few rounds,
 - medium d → the target set is only approximable as an intersection/combination of several projections (the interpolation case of §5.4),
 - large d → stagnation regardless of N (no combination of implemented projections separates the dimension).

**Interaction:** large N amplifies small deficits in d — at large corpus size, the hit fraction within any sample shrinks, so the precision of individual methods becomes more important. At small corpus size, a user can compensate for even weak projections through exhaustive inspection.

### 8.4 Core Hypothesis to Be Tested

> Round count scales as ~log N when d is small or medium, with a clear stagnation signal when d is large.

- If the first part turns out to be linear in N, the method is viable only for small-to-medium corpora.
- If the stagnation signal is absent, the user cannot distinguish between "keep exploring" and "demand a new method" — undermining the entire convergence criterion of §8.1–8.2.

## 9. Related Work and Positioning

| Aspect of the Concept | Established Approach | Key Difference |
|---|---|---|
| Multiple representations, contrast instead of weighting | Principle of Polyrepresentation (Ingwersen) | there: overlap as a relevance indicator; here: overlap only as contrast |
| Unsorted sets with provenance | Metasearch / Federated Search | there: usually fused into a ranking (CombSUM, RRF) |
| Raw features instead of scores | Vespa, Solr/Elasticsearch feature logging, PyTerrier | there: scoring is part of the system, exchangeable rather than mandatorily client-side |
| Exploration instead of query–response | Exploratory Search (Marchionini), Berrypicking (Bates), ASK (Belkin), Information Foraging (Pirolli/Card), Scatter/Gather | typically delivers clusters/facets, not method diversity |
| Sample as query | Query by Example, Relevance Feedback (Rocchio), AIDE | feedback there is usually converted back into a ranking |
| Volume/distribution, sampling | Approximate Query Processing (BlinkDB), Online Aggregation (Hellerstein), Technology-Assisted Review | covers budget/feedback, not multi-projection coverage |
| Diversity selection over methods | Result Diversification (MMR, DPP), Quality-Diversity / MAP-Elites, Novelty Search | typically operates at the result level, here at the method level |
| Cached runs as corpus | TREC Session Track, Scroll/PIT (Elasticsearch), notebook provenance | as a searchable object, rarely implemented |

**Individual methods** this thesis builds on:

- **Trigram signatures:** Signature Files (Faloutsos), BitFunnel (Bing, frequency-aware bit allocation), Google Code Search / Zoekt, Winnowing/MOSS, MinHash/SimHash.
- **AST search:** Deckard, SourcererCC, NiCad, Semgrep/ast-grep, Comby.
- **Multi-vector:** ColBERT, ColBERTv2/PLAID.
- **Structure-free-yet-structural:** Normalized Compression Distance (Cilibrasi/Vitányi).

**Where this thesis goes beyond established work:**

- Treating the set of methods itself as a **measured, coverable space** (result-set distance as a metric).
- Grounding the refusal of any system-side ordering in an **epistemological argument** (the impossibility of anticipating unknown dimensions), not merely a pragmatic one.
- Taking the **contrast between projections**, not the quality of any single projection, as the unit of epistemic gain.
- Proposing a **convergence criterion** (monotone, confirmed refinement) as an explicit replacement for the Probabilistic Ranking Principle's efficiency criterion, rather than merely declining to implement ranking for lack of resources.

## 10. Conclusion and Open Questions

This thesis has traced a single argumentative thread from a narrow technical dispute to a general architectural and epistemological position. The trigram method's "false negatives" dissolve once the method is correctly specified as an overlap-threshold filter with a legitimate scope boundary rather than a universal containment test. Generalizing the lesson — that every representation has a legitimate scope and no representation occupies neutral ground — motivates an architecture of independent, transparently parameterized projection channels whose outputs are unioned, never filtered or ranked against each other. Because relevance is a private and often inexplicit function of the user, ranking is replaced by a two-phase exploration protocol backed by a cache that preserves full search runs, and the system's success is measured not by ranking efficiency but by **monotone, user-confirmed convergence**.

The thesis leaves open, and explicitly proposes as testable, the central scaling hypothesis of Chapter 8.4: that round count grows logarithmically with corpus size when representation distance is small or medium, accompanied by a detectable stagnation signal when representation distance is large. Confirming or refuting this hypothesis — rather than any further argument about ranking in the abstract — is the decisive empirical test of the methodology proposed here.
