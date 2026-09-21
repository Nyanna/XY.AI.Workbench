# AI Code Editing via a Tight, Symmetric Normalization Layer

*An elaboration of the thesis, core arguments, and proposed implementation*

## Thesis

An AI code-editing architecture should **not** try to solve the error-proneness of an LLM by making the model work natively on an AST, nor by validating raw text output after the fact. Instead, it should insert a **transparent, tight, symmetric normalization layer** between the model and the code.

In this design:

- the AI keeps its **natural output format — plain source text**,
- that text is **parsed and normalized into a single canonical structure (AST)**,
- **deterministic operations** are executed on that structure,
- and **unambiguous code** is regenerated from it —

all **without requiring full program compilability** at the point of the edit.

The AST is not a representation *for the model to reason about*. It is an **infrastructure-side shield** that absorbs the model's statistical errors before they take effect.

## Core Arguments

### 1. An LLM is a statistical prediction model, not a semantic reasoner

The decisive correction we make to the conventional framing: AI does **not** understand semantics better than a human does. Treating the model as if it could "work semantically" is a category error. It produces plausible token sequences; it does not comprehend structure. Any architecture must be built around this fact, not against it.

### 2. Both established paradigms are wrong for this reason

- **AI operates directly on the AST** → unnatural to how the model is trained and to its actual capacity. Forcing the model onto an artificial IR fights its strengths.
- **AI operates directly on text** → inefficient, error-prone, and requires separate validation runs (compile/test cycles) to catch mistakes after they occur.

Both approaches misplace responsibility: they either overestimate the model or leave error handling too late.

### 3. Separation of concerns is the actual solution

The key insight is to **split the probabilistic part from the deterministic part**:

```
        LLM  (probabilistic)
          │  plain code text
          ▼
   Parse / Normalize  (deterministic)
          │
          ▼
   Canonical AST  ──► semantic operation
          │
          ▼
   Generator  (deterministic)
          │
          ▼
        Source code
```

- The LLM may be "dumb" and faulty — that is expected.
- The normalization layer holds the rules that decide what is structurally admissible.
- **The model never sees the AST.** It only ever reads and writes code text — the exact format it was trained on.
- Errors (missing `;`, wrong bracketing, alternative spellings) are **not fatal**, because the proposed fragment is parsed and normalized *behind the model's back* before being applied.

This shifts reliability out of the model and into deterministic code, rather than relying on a downstream `code → compiler/tests → LLM fixes it` loop.

### 4. Tight normalization is the essential property (Python `ast` as the model)

The value is not in a rich representation, but in a **narrow, deterministic** one. Python's `ast` is the ideal reference: it breaks code down to instruction level and re-emits it uniformly, with **no variation in representation**.

"Tight" means:

- every syntactic variation of the same construct collapses to **exactly one** canonical structure,
- no formatting or token information,
- no redundant intermediate syntax nodes,
- no implicit semantic relationships beyond what the syntax states,
- **one** deterministic renderer.

The required property is **symmetry / bidirectionality**:

```
normalize(print(IR)) == IR
normalize(code1)     == normalize(code2)   // for semantically equal, syntactically different inputs
```

This yields a **normal form for code**. We explicitly rates this as **more important than a rich, generic cross-language IR** — "IRs only help conditionally if they are not tight and cannot be read and written symmetrically." A general IR that is merely a good internal representation is not enough; it must be a tight, invertible normal form.

### 5. AST-completeness must be decoupled from compilability

Every operation must produce a **structurally complete, internally consistent AST** — but that AST need **not** compile or be semantically correct in the full project context. For example:

```
foo(unknownVariable);
```

is a perfectly valid AST node even though `foo` and `unknownVariable` may not exist. Whether they resolve is a **later** concern.

This gives three distinct validation levels:

1. **Syntax** — "Did this become a complete AST?"
2. **Structure** — "Is this AST admissible and canonical?"
3. **Compilation** — "Is the whole program semantically correct?"

Only **levels 1 and 2** belong in the normalization layer. Level 3 stays with the compiler. Keeping compilation out means each AI edit remains **local, fast, and context-light** — no need to pull in imports, type resolution, or the build system for a single block change.

## Implementation

### Target use case

The AI wants to change a **semantic block inside a function**. It generates the character sequence (error-prone). The infrastructure parses that sequence into an AST, applies the operation, and regenerates code:

```
Existing:
if (x > 10) {
    foo(x);
    bar(x);
}

LLM proposes (possibly malformed):
foo(x);
baz(x)          // missing ';'

Flow:
LLM text → AST(fragment) → normalized operation → transform target AST → generator → source
```

The AST thus acts as a **transaction and validation boundary** around each edit.

### Parsing / normalization backend (Java)

- **JavaParser (`javaparser-core`)** as the parser — it is **dependency-free** and produces a pure AST without formatting.
- **Not** `LexicalPreservingPrinter` and **not** JDT `ASTRewrite` — those preserve formatting, which is the opposite of the goal.
- **No** Symbol Solver, **no** Spoon — type/symbol resolution and reference relations are explicitly out of scope.
- The critical evaluation question for the parser is therefore not "does it have an AST?" but: **how deterministically can a `parse → canonicalize → print → parse` loop be built?**

### Process architecture

- **No JNI / JPype / embedded JVM.** Instead, a **long-running Java worker process**, so the JVM start-up cost is paid once.
- Clean channel separation: protocol on the data channel, logs/errors on `stderr`.

### RPC / serialization

- **Protobuf** as the contract, chosen over JSONL (simple but inefficient) and MessagePack (fast but less type-safe).
- Rationale: a **typed, versioned schema** that generates code for **Python, Java, Rust, and JS** from one `.proto`, enabling later **language-agnostic** parser servers.
- **Send a custom canonical IR — not the raw internal JavaParser AST** (which carries unnecessary ballast such as ranges and token information).

### Contract shape

An LSP-like — but custom — AST-RPC contract, e.g.:

```
service AstService {
  rpc Parse(ParseRequest)       returns (ParseResponse);
  rpc Generate(GenerateRequest) returns (GenerateResponse);
}
```

- A **common operation contract** (`replace block`, `insert statement`, `delete expression`, `replace method body`, `rename declaration`, `add parameter`, …).
- **Language-specific** `parse` / `generate` pairs behind it (`Java text ↔ Java AST`, later `Rust text ↔ Rust AST`, `JS text ↔ JS AST`).
- The AI always stays at the **plain-code** boundary; normalization happens on the server side.

### Overall picture

```
                   LLM
                    │  plain code
             ┌──────▼──────┐
             │ Normalizer  │  parse · validate · recover
             └──────┬──────┘
                    │  canonical AST
             semantic operation
                    │
             ┌──────▼──────┐
             │ Generator   │  deterministic, variation-free
             └──────┬──────┘
                    │
                 source
```

## The Genuinely Novel Point

The interesting engineering problem is **not** the AST schema itself. It is the normalization layer as a **constraint- and recovery mechanism between a probabilistic model and a deterministic program representation** — reconstructing as much as possible from faulty AI code text **without inventing semantic decisions of its own**.

In one line:

> **The model is allowed to be dumb and wrong; the representation is designed to absorb its errors** — via a tight, symmetric, bidirectional normalization layer, decoupled from full compilation.