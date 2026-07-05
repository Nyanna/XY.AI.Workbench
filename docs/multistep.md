
# 1. How LLMs Process Your Prompt

Understanding what happens inside a language model when it reads and responds to your prompt is not just an academic exercise — it directly shapes which prompting strategies work, which fail, and why.

## 1.1 The Key–Value Cache: A Passive Accumulator

Every token your model generates is stored in the **Key–Value (KV) cache** as a pair of matrices. These represent that token's contribution to the model's internal attention mechanism. The cache grows with every step, and at each new token the model recomputes attention over the entire accumulated history — at cost $O(n)$ per step rather than the full $O(n^2)$ of a naive implementation.

What this means for your prompts: the model has **no working memory** and **no global plan**. It does not hold a mental model of your task and then execute it. It produces the next token based on what came before — and that is all.

**There is no revision.** Once a token is generated, it is final. The model cannot go back, reconsider, or correct an earlier step without an external mechanism forcing it to do so. Every intermediate result exists only as generated text sitting in the cache, not as a structured internal state.

## 1.2 Three Structural Limits Every Prompt Designer Must Know

| Limit | Cause | Effect on Your Prompt |
|---|---|---|
| **Autoregressive irreversibility** | Each token is final — no native backtracking | Early errors compound uncorrected |
| **No working memory** | Intermediate states exist only as generated text | Complex state is easily lost or corrupted |
| **Depth limit** | An $L$-layer transformer can execute at most ~$L$ serial reasoning steps | Very deep chains of logic fail structurally |

# 2. Failure Modes to Design Around

These are not theoretical weaknesses — each has been quantified in peer-reviewed research. Knowing them lets you design prompts and workflows that avoid them.

## 2.1 Position Bias: Where You Put Things Matters

Liu et al. (2023) demonstrated that attention over a long context follows a **U-shaped curve** by position. In a 20-document retrieval task, accuracy was approximately 60 % for documents placed first, but dropped to roughly 30 % for documents in the middle.

**Design implication:** Put the most critical context, constraints, and instructions at the **beginning** or **end** of your prompt. Do not bury essential information in the middle of a long input, and do not rely on the model to faithfully retrieve early intermediate results in a long chain.

## 2.2 Error Accumulation Is Multiplicative

In a chain of $k$ steps, each with an individual error rate $\varepsilon$, the probability of a fully correct output is:

$$P(\text{success}) = (1 - \varepsilon)^k$$

At $\varepsilon = 0.1$ and $k = 10$ steps, success probability falls to approximately **35 %** — with no built-in recovery mechanism.

**Design implication:** Do not ask a model to execute long, serial reasoning chains in a single prompt if the individual steps are non-trivial. Break the task up and validate each step programmatically before proceeding.

## 2.3 Reasoning Chains Can Be Decorative

Turpin et al. (2023) showed that Chain-of-Thought (CoT) outputs can be syntactically coherent yet **causally disconnected** from the final answer. The model does not necessarily use its generated intermediate reasoning — it can produce a plausible-looking chain that post-hoc rationalises an answer derived by other means.

**Design implication:** Do not treat a CoT explanation as ground truth verification. If correctness matters, validate intermediate results externally rather than trusting the model's own commentary on its reasoning.

# 3. Prompt Design Patterns

## 3.1 The Naive Pattern and Why It Breaks

```
[Prompt] ──► [LLM] ──► [Answer]
```

The naive single-prompt approach places the entire burden on one forward pass. There is no error recovery, no parallelism, no constraint enforcement, and no ability to inspect what went wrong. For simple, single-step tasks this is fine. For anything more complex, it is structurally fragile.

## 3.2 The Structured Orchestration Pattern

```
[Prompt] ──► [LLM: intent extraction] ──► [Data structure / plan]
          ──► [Validator]
          ──► [Loop / DAG]
          ──► [LLM: subtask × n, parallel if independent]
          ──► [Aggregation] ──► [Answer]
```

Rather than asking the model to solve everything at once, this pattern uses the LLM only for what it is good at — language understanding, generation, and structured extraction — while delegating control flow, validation, and state management to deterministic code.

| Aspect | Single-prompt LLM | Structured orchestration |
|---|---|---|
| Error correction | None | Retry logic after each step |
| Parallelisation | Not possible | Independent tasks run in parallel |
| Constraint enforcement | Implicit, unreliable | Deterministic in code |
| Debugging | Black box | Every step is inspectable |
| Intermediate caching | Not possible | Partial results are cacheable |

## 3.3 Research-Proven Techniques

The following approaches are ordered roughly from minimal overhead to maximal capability.

**Chain-of-Thought (CoT) prompting** asks the model to generate explicit intermediate steps before producing a final answer. Wei et al. (2022) showed improvements of 50–87 % on arithmetic benchmarks (GSM8K) over direct-answer prompting. Best used as a first upgrade from naive prompting when tasks require sequential reasoning.

**ReAct (Reasoning + Acting)** interleaves model reasoning with external tool calls in a repeating `Thought → Action → Observation` loop. Yao et al. (2022) reported a +34 % gain on HotpotQA over pure CoT. The key mechanism: external observations provide ground-truth signals that correct internal drift.

**Tree of Thoughts (ToT)** extends CoT into a branching search structure, enabling systematic backtracking across multiple reasoning paths. Yao et al. (2023) achieved a +16 % gain on Game of 24. Use when the problem has a large solution space and early commitments are costly.

**Skeleton-of-Thought** generates a structured plan first, then expands each section in parallel. Ning et al. (2024) measured a 1.5–2× generation speedup at comparable quality — useful when latency matters and the output has a natural sectional structure.

**Process supervision** uses a reward model that evaluates each reasoning step rather than only the final answer. Lightman et al. (2023) demonstrated a ~15 percentage-point accuracy gain on the MATH benchmark. This is the strongest empirical argument for external step-level validation over trusting the model's own output.

**LLM Compiler** treats the full task as a directed acyclic graph (DAG), compiling independent tool calls for parallel execution. Kim et al. (2024) report a **5.5× latency reduction** over sequential ReAct. The right architecture for production systems with complex, multi-tool workflows.

**DSPy** abstracts prompts away entirely, replacing hand-written instructions with declarative modules (`dspy.ChainOfThought`) compiled and optimised against validation data. Khattab et al. (2023) demonstrated consistent gains over hand-tuned prompts. Use when you want to decouple program logic from prompt wording and optimise systematically.

# 4. Choosing the Right Pattern

Not every task benefits from orchestration. Applying heavy structure to the wrong problem adds engineering overhead without improving results.

**Use structured orchestration when:**

- The task requires four or more serial steps with validatable intermediate results
- External tool calls are involved — deterministic execution is always preferable to LLM hallucination for retrievable facts
- Hard constraints must be enforced (SQL schemas, API contracts, code correctness)
- The system operates in production with audit, logging, or compliance requirements

**Stick to simpler prompting when:**

- The task is creative or open-ended without a clear step decomposition
- Latency is critical and DAG parallelisation is not feasible — multiple sequential LLM calls add a 2–5× overhead compared to a single call
- The step structure of the problem is itself unknown — in that case, complexity shifts into the orchestration layer and may not be justified

**The real trade-off** is not quality versus simplicity. Structured orchestration produces demonstrably better outcomes on complex tasks. The cost is engineering complexity and, without parallelisation, increased latency. DAG-based execution (LLM Compiler pattern) eliminates most of the latency cost when subtasks are independent.

# 5. Key References

| Paper | Authors | Link | Key result |
|---|---|---|---|
| Chain-of-Thought Prompting | Wei et al., 2022 | [arxiv:2201.11903](https://arxiv.org/abs/2201.11903) | +50–87 % on GSM8K over direct answering |
| Lost in the Middle | Liu et al., 2023 | [arxiv:2307.03172](https://arxiv.org/abs/2307.03172) | Accuracy halved for middle-context documents |
| ReAct | Yao et al., 2022 | [arxiv:2210.03629](https://arxiv.org/abs/2210.03629) | +34 % on HotpotQA vs. pure CoT |
| Tree of Thoughts | Yao et al., 2023 | [arxiv:2305.10601](https://arxiv.org/abs/2305.10601) | +16 % on Game of 24 with backtracking |
| Let's Verify Step by Step | Lightman et al., 2023 | OpenAI technical report | +15 pp. accuracy on MATH with process supervision |
| LLM Compiler | Kim et al., 2024 | [arxiv:2407.06030](https://arxiv.org/abs/2407.06030) | 5.5× latency reduction over sequential ReAct |
| DSPy | Khattab et al., 2023 | [arxiv:2310.03025](https://arxiv.org/abs/2310.03025) | Compiled prompt optimisation outperforms hand-tuning |
| Skeleton-of-Thought | Ning et al., 2024 | — | 1.5–2× generation speedup at comparable quality |
