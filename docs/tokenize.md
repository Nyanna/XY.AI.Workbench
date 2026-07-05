# Tokenizing — Statements and Arguments

## 1. Tokenization and Input Processing

**Tokenizers convert text into sequences of numbers.** Input text is broken down by a tokenizer into a chain of tokens — each token representing a word, sub-word, or character drawn from a fixed vocabulary. This numerical representation is what the model actually operates on.

**The entire input token sequence is processed in a single parallel pass.** Unlike output generation, the full input context is evaluated at once, not token by token. This is a fundamental architectural distinction between input and output handling.

## 2. Autoregressive Output Generation

**LLMs generate output one token at a time.** At each step, the model computes a probability distribution over all possible next tokens and selects one candidate.

**Each new token is appended to the existing sequence and fed back as input.** This iterative loop — predict, append, re-process — continues until a designated stop token is reached. The process is strictly sequential on the output side, regardless of how the input was processed.

## 3. The Disproportionate Weight of Early Output Tokens

**Early tokens in the output carry outsized influence.** Because every generated token shifts the context for all subsequent predictions, an early word choice can strongly constrain or steer everything that follows.

**Output generation resembles a cascade of committed decisions.** Unlike the flexible, parallel attention applied to the input, each output token is a point of no return that narrows the probability space for the rest of the sequence.

## 4. The Attention Mechanism

**Attention decouples token significance from position.** Rather than assigning importance based on where a token appears in the input sequence, attention weights are computed dynamically according to contextual relevance.

**Input tokens form a flexible network, not a hierarchy.** No single token dominates by virtue of position alone. The model can attend to any part of the input with equal facility, making the input context a flat, fully connected reference space rather than a tree with diminishing significance.

## 5. Reasoning in Large Language Models

**Reasoning is not a separate module — it emerges from the autoregressive process.** The model reasons by generating tokens, meaning that the act of producing intermediate steps is itself the mechanism of reasoning.

**Chain-of-Thought (CoT) prompting externalises intermediate reasoning steps.** By prompting the model to write out its reasoning, those intermediate tokens become part of the context window, improving the quality of the final answer.

**Many reasoning models are standard LLMs with specialised prompting.** The distinction between a "reasoning model" and a base LLM is often primarily a matter of prompting strategy and fine-tuning, not a fundamentally different architecture.

## 6. Training Data and Application Schemas

**LLMs are not trained exclusively on dialogue.** Training corpora encompass a broad range of text types — books, articles, source code, web content — and the core task throughout is next-token prediction on this diverse material.

**API call structures translate tasks into internal prompts but do not change the training base.** System prompts and instruction formats are a layer on top of the model's underlying capabilities, which remain rooted in general-purpose next-token prediction.

## 7. Semantic Dimensions and Distributed Representation

**Knowledge is stored in a distributed form, not as a symbolic relational database.** Meaning is encoded across the model's weights and embedding vectors rather than in explicit, look-up-style knowledge structures.

**Embeddings map meaning into high-dimensional vector spaces.** Each token is represented as a vector, and semantic relationships are expressed as geometric proximity within that space.

**Self-attention produces weighted retrievals of relevant context tokens.** During processing, the model dynamically pulls in the most contextually relevant parts of the input, functioning as a form of content-addressable working memory.

## 8. KV-Cache and Specialised Attention Heads

**The KV-Cache enables efficient inference by storing pre-computed key and value vectors.** Once a token has been processed, its key and value representations are cached and reused for all subsequent generation steps, avoiding redundant computation.

**Specialised attention heads handle distinct sub-tasks within the model.** Research has identified heads that perform specific functions:

- **Copying heads** — replicate tokens directly from input to output.
- **Coreference heads** — resolve pronouns and referential expressions to their antecedents.
- **Pattern-matching heads** — handle rudimentary counting and simple comparison tasks.

## 9. Logic in LLMs

**LLM "logic" is statistical approximation, not formal deduction.** The model has learned to produce outputs that resemble logical inference from training data, but there is no underlying symbolic reasoning engine.

**Logic-like behaviour can be improved through targeted methods.** Chain-of-Thought training, fine-tuning on structured reasoning tasks, and integration with external symbolic modules can all enhance the reliability of inferential outputs.

**Hybrid systems address the need for verifiable factual accuracy.** Retrieval-Augmented Generation (RAG) and knowledge-base integrations are used where reliable, auditable facts are required — areas where pure statistical generation is insufficient.