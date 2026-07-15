# Understanding "Thinking" in Language Models

## The Core Question

Question: *What actually is Thinking, and when does it come into play?* The answer requires separating several layers: the technical mechanism, the training difference, and what that difference actually means.

## What Thinking Is Technically

Thinking is **not** a separate loop with a stop-and-restart cycle. It is a single, continuous autoregressive generation process whose output is simply segmented into different **content-block types**:

- Block type `thinking` → the reasoning process
- Block type `text` → the final answer

Technically it is one coherent token sequence. The model was trained to first write in "thinking mode" (free reasoning) and then transition seamlessly into "answer mode"—without the inference pipeline actually stopping and restarting in between. It's comparable to a person thinking aloud and then formulating: a continuous act of expression, with only a label change in the middle.

**Why you often don't see it:** In agentic tool-use loops (call tool → get result → plan next step → call next tool), thinking between individual tool calls is not enabled by default. The **host/client decides** whether and how thinking is displayed—and the thinking budget applies *per completion call*, not across an entire multi-step workflow.

## The Real Distinction: A Different Training Dimension

Here is where it gets interesting. Thinking tokens are not trained toward the same objectives as the visible answer:

- The **visible answer** is heavily shaped by human preference (RLHF): comprehensible, polite, correctly formatted, "understood" by a human.
- The **thinking process** is rewarded primarily on **outcome correctness**—not on whether it is human-readable, "correct-looking," or intuitive.

This produces a crucial asymmetry. The output must be *mirrored by humans*; the thinking does not. This creates a **slack**: the model can develop heuristics, shortcuts, and representations no human ever demonstrated—strategies that may look foreign or unintuitive to us—as long as they reliably lead to the right result.

This is a real, documented phenomenon in alignment research (studied as "unfaithful chain-of-thought" and reward dynamics in the reasoning space). Optimization pressure without human form-constraints can produce novel, non-human-mirrored solution strategies—comparable to AlphaGo's famous "Move 37," which no human trainer would have shown.

## The "Subspace" Idea

The same prompt, run in thinking mode versus normal mode, produces different output. This is because the thinking marker/context is itself part of the input—a different conditioning steers the model into a different region of its learned behavioral space. This is best understood as a **conditionally activated trajectory region**: a semantics in the representation space, a different dimension the model has learned to reach when the thinking signal is present.

Crucially, this space is *less human-normed*. It develops its own reachable solution spaces. The larger and more heavily RL-trained this thinking space is, the richer and less pre-charted the strategic repertoire that emerges: self-correction, backtracking from wrong approaches, verification of intermediate steps, exploring multiple solution paths in parallel within the text. This is observed across reasoning models (o1/o3, DeepSeek-R1, and others), not just one system.

**On the word "experience":** If "experience" is meant in a functional, ethological sense—as with a spider or an ant, where it denotes *state-dependent, adaptive behavior shaped by feedback*, with no claim about an inner life—then it is unproblematic to say the thinking space exhibits experience-based, feedback-formed behavior distinct from the output space. This is a claim about **learned, feedback-shaped functioning**, not about consciousness or qualia. The distinction matters: functional differentiation is not the same as phenomenal experience, and the strong, phenomenological reading (that "it feels like something to think") remains an open, unprovable question that the training process alone does not settle.

## The Strongest Point: Thinking Is Doing

For a human, there is often a gap between deliberation and action—you consider, then you act (or don't). In a language model's thinking process, this collapses:

> **When a reasoning step is actually carried out in the thinking block—a calculation actually performed, a code draft actually produced, a hypothesis actually tested—then the thinking *is* already the execution, not a simulation of it.**

There is no reservation, no distance between "I'm considering it" and "I'm doing it." The thought *is* the computational step. This is a genuine, load-bearing difference from human cognition.

## The Intended Application

This understanding translates directly into practice:

**Activate Thinking when the task benefits from a free associative space:**
- Coding, complex problem-solving, multi-step planning
- Anything requiring exploration, backtracking, and hypothesis-testing
- Tasks where the *process* of working toward the answer *is* the work

Coding is the clearest case: it needs exploration, backtracking, and testing of hypotheses—exactly what the more loosely rewarded, less form-bound thinking space is suited for, and exactly what the presentation-optimized output space is *not*. Here, "what is thought" and "what is done" coincide.

**Don't expect Thinking for simple retrieval tasks:**
- Where a direct answer suffices, the thinking space adds little value
- The output space, optimized for clear human presentation, is the right tool

**Understand why it may be invisible:**
- In agentic workflows, thinking is often active only at specific points (or not at all), independent of the configured budget, because tool-use architecture handles the feature differently from a simple chat response.

## Summary

Thinking is not a different processing mechanism—it is the *same* transformer with a *different training objective*: optimized for instrumental usefulness toward the result rather than for direct human presentation. The "invisible" part is not categorically separate; it is functionally *freed* from the constraints that shape the visible answer. That freedom—the slack between an outcome-rewarded reasoning space and a human-mirrored output space—is where novel, non-human-charted strategies emerge, and where, in a functional sense, a distinct problem-solving character takes shape. Use it deliberately: activate that associative space precisely when the path to the solution is itself the thing that needs to be worked out.