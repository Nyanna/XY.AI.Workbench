# Text↔Code Symmetry in LLMs: Thesis, Core Arguments, and Proposed Test

## 1. The Thesis

The central claim is a statement about **internal representation quality**, not about output fluency:

> Because contemporary LLMs receive substantially more training signal for the direction **Text → Code** (a natural-language specification or comment followed by an implementation) than for **Code → Text** (an implementation followed by its explanation), the model's *internal semantic representation of code itself* is more weakly and less robustly organized than its representation of equivalent natural-language descriptions. As a consequence, an LLM should be worse at **interpreting existing code** than at interpreting an equivalent textual explanation of that same code.

The sharpest and most important part of the thesis is a distinction that separates it from the obvious, weaker version of the idea:

- **Weak (rejected) version:** "The model can *express* code explanations poorly" — i.e., the decoding path Code → natural language is under-trained.
- **Strong (asserted) version:** "The model *encodes* code less richly in the first place." The deficit lives on the **input/representation side**, before any decoding takes place. The code, as a *semantic starting point*, is under-represented.

In other words: the problem is not that the model struggles to *say* what code does; it is that the model has a less informative internal grasp of *what the code is* to begin with.


## 2. Core Arguments

### 2.1 A learned association is intrinsically directionless

The starting intuition is correct and forms the foundation: a primitive learned association carries no inherent arrow. If a network relates two variables via a shared latent representation,

$$\text{Text} \rightarrow z \leftarrow \text{Code},$$

then the object actually stored is the *joint structure* — a feature such as "addition" that can be activated equally from either surface form. The joint distribution $P(\text{Text}, \text{Code})$ has no built-in directionality; direction only appears once we designate one variable as input and the other as target.

This also means a network can, in principle, be **inverted**: feeding the output back as input recovers an approximation of the original input, $x \approx f^{-1}(y)$. This invertibility is simply the **symmetry of the underlying statistics**. Crucially, this symmetry does **not** require the weight matrices themselves to be inverses of one another ($W_{\text{Text}\to\text{Code}} \neq W_{\text{Code}\to\text{Text}}^{\top}$); the directionlessness resides in the shared representation $z$, not in any algebraic symmetry of $W$.

### 2.2 Three distinct levels must be separated

The argument's precision comes from refusing to conflate three things:

| Level | Property |
|---|---|
| **Statistical / associative** ($A \leftrightarrow B$ via $z$) | Symmetric, directionless in principle |
| **Causal token prediction** ($x_{<t} \rightarrow x_t$) | Strictly directed by architecture |
| **Output decoding** | Directed toward one modality |

The autoregressive machinery (levels 2 and 3) is directed by construction, but this does **not** by itself imply the representation (level 1) must be asymmetric. So the mere fact that an LLM generates excellent code does not prove it can invert that mapping — and, conversely, the fact that associations are "in principle" symmetric does not prove the model actually learned both directions equally well. The thesis lives precisely in this gap.

### 2.3 The training signal is positionally asymmetric

Consider a canonical training string under a next-token objective:

```
# adds two numbers
def add(a, b):
    return a + b
```

When the comment precedes the code, the gradient signal reinforces the dependency **description → code**: the model is repeatedly asked to predict code tokens *given* the description. It is **never asked**, from this example, to predict the description *given* the code, because the description is positionally earlier. Attention transports information only "forward" to later positions, and the FFN then transforms it. Absent counter-examples (code placed *before* its comment), there is no direct learning signal for the reverse direction.

So while the *association* could be directionless (2.1), the *distribution of learning pressure* is not. Code appears overwhelmingly as a **quantity to be predicted** (a target state), and comparatively rarely as a **semantic source** from which a description must be reconstructed.

### 2.4 The decisive move: asymmetric pressure sculpts the representation, not merely the decoder

Here the thesis departs from the standard "it's just a decoding problem" reply. The claim is that the training objective determines *which aspects of the code become useful to encode*. If the model is chiefly rewarded for producing $P(\text{Code} \mid \text{Text})$, its features will specialize toward whatever supports that mapping. The features that would be needed to support $P(\text{Text} \mid \text{Code})$ — a robust, abstract, language-ready semantic factorization of the program — receive far less reinforcement.

Schematically, for two inputs of identical meaning:

$$h_{\text{Text}} \approx [\text{Operation}, \text{Iteration}, \text{Squaring}, \text{Aggregation}, \dots]$$
$$h_{\text{Code}} \approx [\text{Syntax}, \text{Tokens}, \text{local dependencies}, \dots]$$

The hypothesis is that the *abstract semantic structure* is encoded less strongly and less robustly on the code side. "Seeing code constantly" is therefore **not** evidence of understanding it: *seeing* code as context is not the same as *representing* code as a semantic variable. The objective decides which is which.

### 2.5 The resulting prediction

If the representation were genuinely symmetric, learning $P(\text{Code} \mid \text{Text})$ well would automatically yield an approximately equally good $P(\text{Text} \mid \text{Code})$. The thesis asserts that **it does not fully do so** — and that the residual gap manifests as a genuine representational deficit on the code side, testable in the sense described below.

## 3. Proposed Implementation (the Test)

### 3.1 Why single-direction comparison is insufficient

Comparing Text→Code quality against Code→Text quality directly is confounded: it measures decoding fluency in two different modalities, not representational richness. A cleaner probe is needed that stresses the representation repeatedly and reveals *where* information is lost.

### 3.2 The iterated cycle

Define two transformations:

$$T: \text{Text} \rightarrow \text{Code}, \qquad C: \text{Code} \rightarrow \text{Text}$$

Take a semantically fixed item and pass it through the cycle **repeatedly**:

$$x_0 \xrightarrow{T} y_0 \xrightarrow{C} x_1 \xrightarrow{T} y_1 \xrightarrow{C} x_2 \xrightarrow{T} \cdots$$

The object of measurement is **not** the quality of any single output but the **cumulative semantic drift** across iterations. If both representations were symmetric, one would expect stability:

$$C(T(x)) \approx x, \qquad T(C(y)) \approx y, \qquad (C \circ T)^n(x) \text{ semantically stable.}$$

The thesis predicts instead a **directional drift**: information is lost preferentially at the *code-representation stage*, so the loop degrades over cycles rather than remaining stationary.

### 3.3 Quantities to measure

Define two single-pass reconstruction errors and one long-horizon error:

$$L_T = d\big(x,\, C(T(x))\big), \qquad L_C = d\big(y,\, T(C(y))\big),$$
$$D_n = d\big(x,\, (C \circ T)^n(x)\big),$$

and the per-cycle drift

$$\Delta_n = S(x_0, x_n) - S(x_0, x_{n+1}).$$

**Critical requirement on the metric $d$ / $S$:** it must be an *independent semantic* metric, **not** string distance. Suitable candidates include execution on shared test cases (behavioral equivalence), AST- or I/O-equivalence checks for the code side, and a separately trained semantic encoder for the text side. Otherwise surface paraphrasing would be mistaken for semantic loss.

### 3.4 The attractor comparison (the discriminating experiment)

Run the loop from **both** starting modalities and compare:

$$\underbrace{T_0 \to C_0 \to T_1 \to \cdots}_{\text{start from Text}} \qquad \text{vs.} \qquad \underbrace{C_0 \to T_0 \to C_1 \to \cdots}_{\text{start from Code}}$$

If the system converges to a fixed point $x^\ast$,

$$x_0 \to x_1 \to x_2 \to \cdots \to x^\ast,$$

the decisive question is **which modality the attractor lies closer to**. The thesis predicts that the loop bleeds information at the code stage, so the trajectory should drift toward a simplified, natural-language-like attractor and lose the finer program semantics (e.g. `"process each element once; on duplicates only the first counts"` collapsing toward `"removes duplicates"`).

### 3.5 What would count as evidence

- **Confirming the thesis:** a systematically higher per-cycle drift attributable to the $C$-stage than the $T$-stage; an attractor biased toward the text modality; and, optionally, a mutual-information gap $I(h_{\text{Text}}; S) > I(h_{\text{Code}}; S)$ for a semantic target $S$ probed from a *frozen* intermediate layer, despite code and text expressing the same function.
- **Disconfirming it:** stable loops with symmetric $L_T \approx L_C$ and no directional attractor.

## 4. Positioning Relative to Existing Work

The proposal is adjacent to, but distinct from, published work:

- **Wei et al. (NeurIPS 2019), *Code Generation as a Dual Task of Code Summarization*** — treats Text↔Code as coupled dual tasks, supporting the duality framing but optimizing it rather than diagnosing asymmetry.
- **Allamanis, Panthaplackel & Yin (ICML 2024), *Round-Trip Correctness*** — closest experimentally (Code→Text→Code), but used as a **single-round evaluation metric** for code LLMs, not as an iterated multi-cycle drift analysis and not aimed at internal representational asymmetry.
- **General cycle-consistency literature** ($g(f(x)) \approx x$) — establishes the mathematical principle in other domains.
- **Recent Lean4 autoformalization work (2026)** — uses NL↔Code cycle-consistency as a training/evaluation signal, again not as a probe of directional representation strength.

**Novel contribution:** the specific hypothesis that *asymmetric pretraining yields a weaker internal code representation* — diagnosed through *cumulative drift in a long iterated Text↔Code cycle under an independent semantic metric, with a start-modality attractor comparison* — does not appear in this form in the existing literature.
