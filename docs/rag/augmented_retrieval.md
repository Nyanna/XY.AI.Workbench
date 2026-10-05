# Retrieval Augmentation Without an Intermediate Agent Layer

An intermediate search agent between a user-facing (outer) model and a repository cannot raise retrieval quality above what the outer model achieves with full context and the same tools. It can only shift cost. Augmentation should therefore sit at the tool interface, not in a second agent: the outer model keeps intent and control, and a trained augmentation component improves tool parameters and tool returns, anticipating what the outer model needs next without hiding what happened.

## 1. Information-theoretic core

1. **Query bottleneck.** An intermediate agent sees a compressed query, not the outer model's context. User intent and earlier decisions that exist only in the conversation are lost at this interface and cannot be recovered downstream.
2. **Upper bound.** An outer model with full context and identical tools is never worse than the intermediate layer. The layer adds no information about intent.
3. **What the layer can add.** Tool results from the repository are new external evidence, so a search loop can learn about the repository and correct mechanical errors (wrong pattern, wrong directory, too narrow a match). This is not pure autoregression, but the evidence concerns what exists, not what was meant.
4. **Return-channel loss.** The outer model receives only the final result list. Failed searches, discarded candidates, negative findings ("pattern X does not exist") and uncertainty are dropped, although they are often the most useful signals for the next step.
5. **Asymmetry.** The intermediate layer loses information in both directions: intent on the way in, search trajectory on the way out. Because the outer model did not produce the result, it cannot judge it as it would judge its own mistake, and it tends to accept a confident, structured list.

## 2. Error classes

| Class | Effect of an intermediate loop |
|---|---|
| Mechanical errors (pattern, path) | Reduced, because tool feedback contradicts the earlier choice |
| Intent errors (query misunderstood) | Not detectable from repository evidence, likely amplified by further iteration, and hidden from the outer model |
| Context pollution (over-broad match) | Contained in the inner context, not prevented; cheaper there than in the outer working context |

Iteration count is not a multiplier on error. The loop is closed by external observations. Whether the net effect is positive depends on the ratio of mechanical to intent errors, which is unmeasured.

## 3. When an intermediate layer is justified

Only if it brings an advantage the outer model cannot obtain more cheaply:

- **Information advantage:** persistent index, cross-repository data, history, telemetry, organizational knowledge.
- **Capability advantage:** a stronger or specialized model that anticipates the outer model's needs better than the outer model steers its own search.
- **Preserved return channel:** the layer passes on provenance, negative results and uncertainty, so the outer model can audit it.

Otherwise it is cost and context arbitrage (a cheaper model, a cleaner outer context), not a quality gain.

## 4. Design principles

Assumption: "bootstrapping" means deriving training and tuning signal from the outer model's own tool-use trajectories and outcomes.

1. **No second agent layer.** The outer model keeps intent, context and control over the search.
2. **Augmentation at the interface, in two places:**
   - *Pre-call:* rewrite or complete tool parameters (patterns, path scoping, result limits).
   - *Post-call:* filter noise, rank, compress, and attach provenance.
3. **Return channel stays informative.** Failed queries, negative findings and confidence remain visible to the outer model.
4. **Anticipation.** Learn from observed trajectories what the outer model needs after a given step, and precompute or prefetch it. Bootstrapped from real sessions, not from a generic retrieval benchmark.
5. **Deterministic first.** Static analysis (import and call graphs, LSP), lexical and embedding hybrid retrieval, hard output caps. Use learned components only where determinism does not reach (lexical gap, ranking by expected usefulness).
6. **Optimize for signal per context token,** not for retrieval recall alone.