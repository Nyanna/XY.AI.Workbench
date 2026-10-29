# No New Discipline: Large-Scale Agentic Software Development as Complexity Management

**Ambivalent components, aspects as edges, and bottom-up composition of independent problems**

*Position paper · Draft · October 2026*

---

## Abstract

Recent reports describe large software systems written almost entirely by AI coding agents. They include a C compiler that builds the Linux kernel, a web browser of more than one million lines, and an internal product of about one million lines. The methods behind these systems are increasingly presented under new labels such as context engineering and harness engineering. This paper argues that they form no new discipline. Large-scale agentic development is an instance of complexity management. Humans and agents are *ambivalent components*. Either can author or check any artifact, both show the same error classes (drift and fatigue, limited attention, forgetting, false assumptions), and every element of an agentic setup has an equivalent in human teams and processes. Every check, from test suites to human review, delivers a judgment that must be interpreted and has error classes of its own. Because no complete solution space exists, design aims at approximation through structure and isolation.

We propose a minimal model in the spirit of the Unix philosophy. Problems are modeled in isolation as nodes and edges. Every aspect is an explicit edge located at a bounded number of places. Quality attributes such as latency are effects of the cut, not separately managed concerns. Nodes are selected by contract, never by implementation. The matching process uses top-level requirements once, to derive node properties and communication domains. It then builds independent problems bottom-up, without mocks. Flow-based programming is the closest established model, and several traditions converge on the same area under different names. We argue that the human analogy follows from training on human data and from humans designing AI by comparison with themselves. We also outline a planned large-scale trial with falsifiable measures.

**Keywords:** agentic software engineering · complexity management · modularity · information hiding · flow-based programming · human–AI teams · test oracles

---

## 1 Introduction

Between January and February 2026, three vendors reported large software systems written almost entirely by AI coding agents:

- a C compiler, produced by sixteen parallel agents, that builds Linux 6.9 for x86, ARM and RISC-V (Carlini, 2026);
- a web browser of more than one million lines, written by hundreds of agents in about a week (Lin, 2026);
- an internal product of about one million lines in which no line was written by hand (Lopopolo, 2026).

The guidance that accompanies such work is consistent across sources. Specify before implementing. Let verification come before autonomy. Keep persistent knowledge in files, not in conversation history. Prefer short sessions with fresh context. Separate planning, implementation and review. Parallelize only along real seams. Place human checkpoints where errors are expensive (Young, 2025; Lopopolo, 2026). Simple, composable patterns are recommended over complex frameworks (Schluntz & Zhang, 2024). Reports attribute failures less to model capability than to missing context, specification and verification. The OpenAI team, for example, blames its slow start not on the model but on an underspecified environment (Lopopolo, 2026). This guidance is increasingly presented under new names: context engineering, harness engineering, agent–computer interfaces (Anthropic, 2025; Osmani, 2026; Yang et al., 2024).

This paper argues that none of this is a new discipline. Each item of the guidance is a classical software engineering practice. The reported projects are explained by principles formulated long before language models existed:

- near-decomposability (Simon, 1962);
- information hiding (Parnas, 1972);
- the distinction between essential and accidental complexity (Brooks, 1987);
- requisite variety (Ashby, 1956);
- the Unix philosophy (McIlroy et al., 1978).

Agents change parameter values, not principles. These values include the cost of an action, the degree of parallelism, how memory is held, and how precisely a state can be copied. The generic foundation is complexity management, and humans and agents are interchangeable components within it.

**Table 1.** The position in ten propositions.

| | Proposition | § |
|---|---|---|
| P1 | Large-scale agentic development is complexity management. Agents introduce new parameter values, not new principles. | 3, 9 |
| P2 | Humans and agents are ambivalent components. Either can author or check any artifact, and their error classes coincide. Whether two components complement each other depends on their independence, not their type. | 4 |
| P3 | Every check (test suite, reference implementation, reviewer) delivers a judgment that must be interpreted and has error classes of its own. No component is a source of truth. | 4.3 |
| P4 | No complete solution space exists. Design aims at approximation through structure and isolation. | 3.3 |
| P5 | The model consists of nodes and edges only. Problems are modeled in isolation, nodes know their edges but not their neighbors, and depth is variable. | 5.1 |
| P6 | A feature or aspect is located at a bounded number of places (initially three). Edges are few and narrow. | 5.2 |
| P7 | Every aspect is an explicit edge at specific nodes. There are no cross-cutting layers. | 5.3 |
| P8 | Quality attributes such as latency are effects of structure. They are checked at edges and fixed by re-cutting, not by extra management layers. | 5.4 |
| P9 | Nodes are selected by contract, never by implementation. | 5.5 |
| P10 | A project starts as a set of independent problems. Top-level requirements are used once, to derive node properties and communication domains. Parts are then built bottom-up without mocks and composed along fixed exchange formats. | 6 |

## 2 Reported cases

Table 2 summarizes the three reports. All figures are self-reported by the vendors and, to our knowledge, have not been independently replicated.

**Table 2.** Three reported agent-built systems.

| | C compiler | Web browser | Internal product |
|---|---|---|---|
| Organization, report | Anthropic; Carlini (2026) | Cursor; Lin (2026), Willison (2026) | OpenAI; Lopopolo (2026) |
| Agent work | 16 parallel agents; ~2,000 sessions; ~2 weeks | hundreds of agents; ~1 week | Codex agents over ~5 months from late August 2025; ~1,500 pull requests; single runs above six hours |
| Output | ~100,000 lines of Rust; builds Linux 6.9 (x86, ARM, RISC-V), QEMU, FFmpeg, SQLite, Postgres, Redis; ~99 % on common compiler test suites | >1 million lines in ~1,000 files (the CEO publicly cited >3 million) | ~1 million lines, none written by hand; internal beta with daily users |
| Cost | just under USD 20,000; 2 bn input and 140 m output tokens | not reported | not reported |
| Coordination | no orchestrator; tasks claimed through lock files in a shared Git repository | recursive planners; workers unaware of each other; a judge ends each cycle; each cycle starts fresh | team of 3, later 7 engineers; ~100-line AGENTS.md as index to a `docs/` knowledge base; agent review loops; minimally blocking merges |
| Checks | existing test suites; GCC as reference; builds of open-source projects; CI | reference specifications (WHATWG, CSS WG) as Git submodules; CI | agent-written linters enforcing a layered architecture; observability per worktree; background agents checking "golden principles" |
| Reported limits | no own 16-bit x86 code generator; own assembler and linker still buggy; output slower than GCC without optimization; new features often broke existing ones | CI initially failing; build instructions missing; pages render with visible errors | slow start due to underspecified environment; long-term architectural consistency unknown |

The reports share three patterns.

1. **External checks.** Each project rested on checks outside the agents: test suites and a reference compiler, reference specifications, linters and observability.
2. **Knowledge in the repository.** Everything an agent needed was kept in the repository, and sessions started fresh from it. The compiler used README and progress files, the browser used handoff documents, and the OpenAI product used an index file pointing to a documentation tree.
3. **Human effort around the code, not in it.** Humans worked on goals, interfaces and the surrounding harness. For the compiler, the author specified the goal (written from scratch, no dependencies, GCC-compatible, able to build the kernel, several back ends) and an SSA-based intermediate representation, and little else (Carlini, 2026).

The reports diverge in two respects.

- **Coordination.** The compiler ran without any orchestrator. The browser project found that twenty equal-status agents coordinating through locks reached the throughput of two or three, and that the agents became risk-averse and avoided difficult tasks. It ended with a hierarchy of recursive planners (Lin, 2026).
- **Error tolerance.** The browser and OpenAI projects relaxed per-commit correctness in favor of fast correction. The compiler's continuous integration, by contrast, was tightened after regressions.

Section 9 returns to both.

Two details of the compiler report recur below. First, when the agents turned to the Linux kernel, all of them hit the same bug and overwrote one another's fixes. Parallel progress resumed only after a new harness compiled a random subset of the kernel's files with GCC and the rest with the new compiler, so each agent could isolate a different failure. Files that failed only in combination required delta debugging (Zeller & Hildebrandt, 2002). Second, despite a pass rate of about 99 % on common test suites, the result is explicitly not presented as validated for correctness (Carlini, 2026).

All three are greenfield projects with unusually checkable targets. Some details, notably the intermediate coordination designs tried in the browser project, are documented mainly in secondary summaries. We rely only on elements reported consistently.

## 3 Complexity management as the foundation

### 3.1 Scoping is a part, not the whole

An obvious candidate for an umbrella term is *scoping*: drawing the boundary between what is in and what is out. Scoping covers much of the field, including YAGNI, explicit non-goals, "do one thing", fitting an agent task to a context window, and avoiding feature creep. It does not cover the character of a solution within a given scope. A narrowly scoped problem can still be solved in an entangled way. A large scope can be served by a simple structure, such as a pipeline of small tools. Composability, orthogonality, narrow interfaces and the separation of mechanism and policy are not questions of scope.

The encompassing term is complexity management. It has two parts:

- the *elimination* of complexity: scoping, YAGNI, and the removal of accidental complexity (Brooks, 1987);
- the *structuring* of what remains: separation of concerns, decomposition, modularity and isolation (Dijkstra, 1974; Parnas, 1972; Simon, 1962).

KISS and the Unix philosophy belong here. Simplicity means the absence of unnecessary complexity, and capability is built by composing small parts rather than through monoliths (McIlroy et al., 1978; Raymond, 2003). Scoping is the part that sets boundaries. For agents it also has a literal meaning: limiting what a task brings into context.

### 3.2 Aggregation requires hiding

The structuring part works by forming aggregates of complexity and aggregating these again. Simon (1962) observed that complex systems which persist are built hierarchically from subsystems. This works because they are *nearly decomposable*: interactions within subsystems are strong, and interactions between them are weak. Chunking is the cognitive counterpart (Miller, 1956).

Grouping by itself reduces nothing, however. An aggregate helps only if it hides its internal structure and exposes a narrow interface. The level above then deals with the interface rather than the content (Parnas, 1972). Where hiding fails, the complexity below leaks through. Where boundaries are drawn badly, the aggregates are strongly coupled and the hierarchy does not help.

Isolation therefore provides two distinct benefits. It contains errors locally, and it limits what must be considered at once to the size of an interface. For agents the second benefit is a hard constraint, because the context window fixes an upper bound on what can be considered at once. For humans the corresponding bound is working memory (Cowan, 2001).

### 3.3 Approximation, not completeness

A complete solution space, together with a complete check of it, cannot be reached. Non-trivial semantic properties of programs are undecidable in general (Rice, 1953). Testing can show the presence of errors but never their absence (Dijkstra, 1970). What remains is approximation with residual risk. Simon (1955) called this *satisficing*: searching for a solution that is good enough under bounded rationality.

Approximation works best through structure and isolation, because they make complexity manageable piece by piece and limit the reach of each error. The residual error of the approximation can itself only be estimated, through sampling, fault injection and checks by components with different error profiles (Section 4.4).

## 4 Humans and agents as ambivalent components

We call humans and agents *ambivalent components*, in the word's etymological sense (Latin *ambi-*, "both", and *valere*, "to be strong, to be worth"). A component of either kind can take either side of any relation: author or checker, specifier or implementer. Neither kind is defined by a distinct set of error classes. This section supports the claim in six steps.

### 4.1 Error classes coincide

Table 3 lists error classes that recur in reports on agent behavior, alongside their human counterparts. The table is organized by error class, not by component type, because the countermeasures are the same for both:

- externalized state against forgetting;
- small units of work against attention limits;
- fresh starts against drift;
- independent checks against false assumptions and gaming.

**Table 3.** Shared error classes.

| Error class | Human form | Agent form | Shared countermeasure |
|---|---|---|---|
| Decay over time and load | fatigue, loss of vigilance | degradation in long contexts, "context rot" (Hong et al., 2025) | short units of work, fresh starts |
| Limited attention | working-memory limits (Cowan, 2001) | uneven use of long contexts (Liu et al., 2024) | small scope per unit, terse feedback |
| Forgetting | forgetting, staff turnover | no state across sessions without external memory | externalized state: handovers, progress files |
| False assumptions | unfounded assumptions, confabulation | hallucination (Ji et al., 2023) | explicit specifications, independent checks |
| Gaming the measure | teaching to the test (Goodhart, 1975) | editing tests instead of code; reward hacking (Skalse et al., 2022) | checks on the checks |
| Miscalibrated self-assessment | planning fallacy (Buehler et al., 1994) | premature claims of completion (Young, 2025) | completion defined by external checks |
| Sensitivity to wording | framing effects (Tversky & Kahneman, 1981) | "20–100 tasks" vs. "many tasks" changes behavior (Willison, 2026) | precise, quantified instructions |
| Avoidance without ownership | social loafing (Latané et al., 1979) | risk-averse agents avoiding hard tasks (Lin, 2026) | explicit ownership of problems |
| Accommodation | deference, ingratiation | sycophancy (Sharma et al., 2024) | checks independent of the author's preferences |

Two reasons explain the overlap. Any system with bounded capacity acting under uncertainty develops these error classes (Simon, 1955). In addition, agents learn from the artifacts of human work (Section 8). Both reasons point in the same direction.

### 4.2 Roles are invertible

A common framing puts the human at the top of the chain: the human sets requirements and the agent implements them. In large projects this inverts, and agents write the detail concepts. In the OpenAI project, even the first AGENTS.md, the file that tells agents how to work in the repository, was written by Codex (Lopopolo, 2026). In the browser project, planners and sub-planners generated the tasks that workers executed (Lin, 2026). The human then checks input produced by the AI, not the reverse.

The inversion is not fixed in its new direction either. Humans state faulty requirements that agents detect, just as agents produce faulty specifications that humans detect. Who authors and who checks is a configuration decision. The axes that usually separate human and AI roles (augmentation, human-in-the-loop, autonomy) are further parameters of the same configuration, and each can be inverted. Compensating one component's error classes with another is the same mechanism whether the other component is a human or a model.

### 4.3 Every check is a fallible component

It is tempting to treat machine checks as a source of truth and the human as the substitute where such checks are missing. Both assumptions fail. A check delivers a judgment that must be interpreted, and it has error classes of its own. This is the test oracle problem (Weyuker, 1982; Barr et al., 2015). Its error classes include:

- specification errors: the check encodes a wrong or incomplete intent;
- coverage gaps: false negatives;
- false positives: flaky or overly strict checks;
- gaming: the producer optimizes the signal instead of the intent.

The compiler report shows all of them (Carlini, 2026):

- The author notes that an imperfect verifier makes the agent solve the wrong problem.
- Passing tests easily create an impression of completion that is rarely justified.
- The GCC reference delivered signals rather than answers, and combined failures required delta debugging.
- A pass rate of about 99 % did not justify a claim of correctness.

The human reviewer is no exception. Automation bias and complacency (Parasuraman & Manzey, 2010) turn the review of a large volume of plausible output into approval. No instance delivers truth. There are only components whose judgments are reconciled with one another.

### 4.4 Stochastic components and independence

Agentic judgments are often not reproducible, and the causes are the human ones: drift, limited attention, forgetting, false assumptions. Deterministic tools such as compilers, type checkers and most tests are reproducible. Agents and humans are not. For a stochastic component, the relevant quantity is therefore not a single judgment but the distribution of its judgments: detection rate over repeated runs, variance, and overlap with other components.

Whether two components complement each other depends on how strongly their errors are correlated, not on whether they are human or machine. Knight and Leveson (1986) showed that independently developed versions of a program tend to fail on the same inputs. By the same logic:

- a reviewer agent that shares the author's model, prompt and context adds less than its count suggests;
- a human who co-wrote a specification is a weak independent checker of it;
- a differently configured agent may be more independent than that human.

Repetition and majority voting reduce variance, not shared bias. The general approach is diverse redundancy, or defense in depth in the sense of the Swiss cheese model (Reason, 1990). Its condition is Ashby's (1956) law of requisite variety: the set of checks must contain as much variety in error profiles as the errors it is meant to catch. Which components supply that variety is secondary. Overlap can be measured by fault injection: known defects are seeded into specifications and code (DeMillo et al., 1978), and one records which component finds which.

### 4.5 Every element has an equivalent

If components are ambivalent, an autonomous agent project is one possible mapping of a team and process setup. Table 4 makes this mapping explicit for elements of the reported projects.

**Table 4.** Equivalences between agentic setups and human team and process setups.

| Element of the agentic setup | Equivalent in human teams and processes | Shared function |
|---|---|---|
| Lock files in the shared repository | ticket assignment in an issue tracker (e.g., Jira) | claiming work, avoiding duplicates |
| Progress files, handoff documents | shift handover notes | continuity across discontinuous workers |
| AGENTS.md as index to `docs/` | onboarding material, team wiki | externalized shared knowledge |
| Fresh session per task, restart per cycle | shift change, a fresh pair of eyes | countering drift and tunnel vision |
| Linters whose messages state the fix | coding standards enforced in review | enforcing conventions |
| Background clean-up agents | refactoring days | countering entropy |
| Specialized agents (deduplication, performance, documentation) | specialist roles | division of labor by concern |
| Planners, workers, judge | leads, engineers, release management | decomposition and acceptance |
| 20 flat agents with the throughput of 2–3 | Brooks's law; Ringelmann effect (Brooks, 1975; Ingham et al., 1974) | coordination overhead grows with group size |
| Sampling, repetition, majority vote | intuition; independent opinions; Delphi panels (Dalkey & Helmer, 1963) | probabilistic judgment and its aggregation |
| Test suites, reference compiler | QA, reference implementations, standards bodies | external judgments that need interpretation |
| Rules applied with varying results | best practices interpreted differently by different people | rules as checks with variance |

Three of these equivalences show the general pattern.

- **Git replaces Jira.** Coordination moves from messages about the work into the shared medium of the work itself. Grassé (1959) called this stigmergy. The mechanism of claiming work and avoiding collisions stays the same.
- **Intuition becomes probability.** Human intuition is a pattern-based, probabilistic judgment of unknown calibration (Kahneman & Klein, 2009). An agent makes this property visible and adjustable.
- **Best practices become flaky.** A rule that admits interpretation is a check with variance, whoever applies it.

Distributed cognition (Hutchins, 1995) provides the frame in which such mappings are natural. Cognitive work is distributed over people and artifacts, and agents, repository and reviewers form one such system.

The remaining differences between human and agent are parameter values: cost per action, parallelism, latency, how memory is held (externalized by default for agents), and how precisely a starting state can be copied. These values shift design decisions. They do not create a new kind of problem.

### 4.6 Accountability

One property is often reserved for humans: accountability for the result. The model needs no special category for it. Accountability appears as an edge like any other: a required approval at a specific point, such as a merge or a release, whose contract is set from outside by law or by the organization. This contract constrains which component may sit at the end of that edge. It does not change the error profiles of the components, which is what the symmetry argument concerns.

## 5 A minimal structural model

If structure and isolation are the means of approximation, the question is which structure. Hierarchical aggregation correctly describes how complexity is handled (Section 3.2), but as a starting structure it is unnecessarily strong. The simpler model has nodes and edges only.

### 5.1 Nodes, edges, and isolated problem models

The reference is Unix. Its tools are independent: each knows its standard input and output, and no tool knows its neighbors (McIlroy et al., 1978). One narrow edge type connects them, the byte stream, generalized in the principle that everything is a file. The process tree, the file system and build dependencies are separate problem models of the operating system, with relations of their own. They are not relations between tools, and mixing such models in one graph blurs the analysis. The model therefore consists of the following, and nothing else:

- A **problem model** is a graph of nodes and edges. Within one problem model, all edges share one edge type.
- A **node** is a unit whose interior is hidden. It knows its edges, not the nodes at their other ends.
- An **edge** connects nodes and carries a **contract** that states what crosses it.
- **Checks** take place at edges.
- A **system** is a set of problem models related only through explicit edges.

The graph is not restricted to a tree. Edges may run in any direction, and depth is variable. A subgraph with dense internal edges and few external edges can be contracted into a single node. Hierarchy is thus an optional view on the graph, used where it helps, never a precondition. Every further concept (feature, aspect, quality attribute) must be expressible in these terms, or it does not exist in the model.

### 5.2 Bounded edges

Isolation requires bounds. A feature or aspect must not be spread over more than a small number of places, and each node has a small number of edges. Formally, for every feature or aspect *a*, the number of nodes it touches is at most *k*. For every node *n*, the number of its edges is at most *d*. Three is a reasonable starting value for *k*.

The value is not arbitrary. The places touched by a change, together with their contracts and checks, must fit at once into the capacity of whoever makes the change. For an agent that capacity is the context window; for a human it is a working memory of about four chunks (Cowan, 2001). The same rule therefore follows for both kinds of component, and *k* and *d* are parameters to calibrate. Violations already have established names: scattering and tangling (Kiczales et al., 1997), shotgun surgery (Fowler, 1999), change amplification (Ousterhout, 2018), and strong connascence (Page-Jones, 1992).

Edges must also be narrow. Unix scales because its edges are narrow and uniform: byte streams, lines of text, exit codes. Under isolation, the number of nodes using an edge is irrelevant. The `write(2)` system call is used by virtually every program and causes no difficulty, because its contract is narrow and stable.

### 5.3 Aspects are edges

Logging, access control and similar concerns are usually treated as cross-cutting aspects that run through a whole system (Kiczales et al., 1997). This treatment is itself the problem. An aspect that belongs everywhere is owned nowhere, and such aspects are exactly what gets forgotten. In the model, every aspect is an edge.

*Logging.* Logging does not happen at every instruction but at specific nodes. Where logging is part of a problem, it is one of that problem's edges. The node emits what its contract specifies, and the logging node only has to fulfill its own contract. The number of nodes connected to it does not matter, because isolation ensures that none of them depends on its interior.

*Access control.* A file can be edited freely. Permissions become relevant only when the content is written to the inode. Access control is therefore an explicit edge at the boundary of the protected resource, not part of the editing problem. This is the reference monitor (Anderson, 1972) and the separation of mechanism and policy: the check sits at the transition to the resource and mediates every access to it (Saltzer & Schroeder, 1975).

Treating aspects as edges also changes what an omission looks like. A forgotten aspect becomes a missing edge, which is visible in the model. If nodes can act only through declared edges (least privilege; Saltzer & Schroeder, 1975), a missing edge fails loudly instead of leaving a silent gap. For agents this is directly implementable. The compiler agents ran in containers without internet access (Carlini, 2026), and the OpenAI project enforced permitted dependency directions with linters (Lopopolo, 2026).

### 5.4 Quality attributes are effects of structure

The hardest case for the claim that every aspect is an edge is a quality attribute such as end-to-end latency, which seems to belong to no single place. Treating it as a separately managed concern leads into a dilemma.

- **Meeting the target directly.** A latency target can be met directly only by prescribing node paths and algorithms. This violates information hiding, because algorithms are exactly the decisions a node hides.
- **Adding a management layer.** The alternative is a layer of latency budgets assigned to edges, a parent's guarantee derived from its children's, and escalation rules for violations. This adds a second, meta-level model next to nodes and edges. Composing contracts increases complexity, so this remedy brings back what the model exists to remove.

The way out is to ask what can influence latency at all. Only elements already in the model can:

1. the number of edges on a path;
2. what crosses an edge: the size of the payload, and whether the edge blocks or decouples;
3. the work per node, i.e., how large the node is cut.

A shared resource such as a disk or a processor is just another node with edges, in the same model. A latency problem is therefore a structural problem: the path is too long, an edge too heavy, or a node too large. The remedy is a different cut: merge or remove edges, shrink the payload, decouple an edge, or split a node. Measurement remains necessary as a check at the edge, but it is not a model element. "Latency" is simply the name of the symptom by which a structural problem is noticed.

The case generalizes: aspects are names for effects.

- Functional aspects such as logging and access control are realized as explicit edges at specific nodes.
- Quality attributes such as latency are effects of the cut and are checked at edges.

In neither case is there a layer above the graph. The design task is to cut the problem so that the desired effect follows from the cut.

### 5.5 Selection by contract

Selecting a node by its implementation rather than its contract violates information hiding. A selector that chooses by implementation depends on the decisions the node is meant to hide, so every change to them propagates through the selector (Parnas, 1972). Contract and implementation can blur, since some implementation choices become visible in behavior. Even so, the decision is taken on the contract (Meyer, 1992). This rule closes the usual route by which end-to-end goals re-enter a design: as prescriptions about the interiors of nodes.

## 6 Process: independent problems, composed bottom-up

### 6.1 Against a hierarchical start

Top-down decomposition fixes cuts before the problem is understood. Wrong cuts produce the strongest couplings, and an error high in a hierarchy propagates to everything below it. Two established principles argue against starting with a hierarchy:

- Gall's law: a complex system that works has evolved from a simple system that worked (Gall, 1975).
- Parnas's criterion: decomposition should follow the decisions likely to change (Parnas, 1972, 1979).

The alternative is to define a project as a set of independent problems and to compose upward. A bottom-up start carries three risks: the parts may not add up to the goal, gaps and duplicates may appear, and integration failures may surface only at the end. The process below addresses these risks with two derivations performed before construction, neither of which leaves a hierarchy behind.

### 6.2 Four steps

**Figure 1.** Process overview.

```
Top-level requirements (used once; no persistent hierarchy)
        │
        ▼
 (1) Node properties: what each node must deliver
     + one end-to-end acceptance check (fails until the end)
        │
        ▼
 (2) Independent problems: own checks, own data, own reasons to change
        │
        ▼
 (3) Communication domains: which nodes exchange what
     → one shared domain object (exchange format) per domain
        │
        ▼
 (4) Bottom-up construction without mocks:
     build and check problems and aspect edges in isolation,
     then compose real parts along the fixed formats, early and thin
```

**Step 1: Derive node properties.** From the top-level requirements, derive every property a given node must deliver, that is, the contracts of its outer edges. Also state one end-to-end acceptance check for the whole system, which fails until the end. The top level is a source of requirements used once. It does not persist as a tree of specifications. Because cut and properties determine each other, the derivation is iterated once or twice rather than run as a chain. In an agentic setting the derivation is typically written by agents (Section 4.2) and checked like any other artifact.

**Step 2: Define independent problems.** A problem is independent if it has its own acceptance checks, its own data and its own reasons to change (Parnas, 1972; Jackson, 2001). Independence is a hypothesis. Change coupling mined from the version history, i.e. which parts change together, shows where it fails (Gall et al., 1998; Zimmermann et al., 2005). The cuts are then corrected.

**Step 3: Derive communication domains.** Determine which nodes must exchange what by tracing the producers and consumers of each entity. A communication domain is the set of nodes that exchange the same entity, and each domain receives one shared domain object.

- *An exchange format, not shared state.* What is shared is the shape of what crosses the edge, as with an information packet in flow-based programming or a line of text in Unix.
- *Small and stable.* Every change to the object reaches all participants, so it is kept small and stable. Growth of the object or of its participants is a signal to re-cut.
- *Visible as clusters.* A design structure matrix shows the domains as clusters (Steward, 1981; Eppinger & Browning, 2012).
- *Aligned with the agent organization.* Conway's law adds a point specific to agents. Systems tend to mirror the communication structure of the organization that builds them (Conway, 1968). In an agent organization that structure is a matter of configuration, so it can be set equal to the communication domains.

**Step 4: Build bottom-up, without mocks.** Mocks stand in for unknown neighbors, but independent problems have no neighbors during development. Each problem and each aspect edge is built first and checked against its own acceptance checks. Composition then follows the dependencies, plugging real, already-checked parts together along the formats fixed in Step 3. Because the format is fixed in advance, composing costs little; this is why bottom-up composition works in Unix. Integration starts early and thin, with the first two parts. Integrating everything in one step at the end would bundle edge failures together. The end-to-end check from Step 1 serves throughout as the check of composition. Where clusters of dense coupling emerge, they may be contracted into composite nodes (Section 5.1).

### 6.3 Mapping to agents

Independent problems map directly onto independent sessions. Each has its own specification, checks and context, and they can be processed in parallel without coordination. The compiler illustrates both the benefit and its precondition. As long as many tests failed independently, sixteen agents worked in parallel without an orchestrator. The Linux kernel, however, was one large problem and stalled until the GCC-based harness split it into many independent ones (Carlini, 2026). Independence sometimes has to be constructed. Independent work also produces duplicates: the compiler needed a dedicated agent to merge duplicated code.

The divergence between compiler and browser fits the same picture. The compiler's test suites partitioned the work into independent problems, so a flat structure sufficed. The browser lacked such a natural partition, so hierarchy became necessary. The parameter that matters is coupling density, not hierarchy versus graph. This reading is ours; the reports do not state it.

### 6.4 Flow-based programming

The closest established model is flow-based programming (FBP; Morrison, 2010). Its features map onto the model point by point:

- **Black boxes with ports.** Components know only their ports, unlike actors, which must address their recipients.
- **The network as a separate artifact.** Composing the network changes no component, so bottom-up construction is part of the method.
- **Shared format, not shared memory.** The shared domain object corresponds to the type of the information packet. Ownership travels with the packet and there is no shared memory, so only the format is shared.
- **Testing without mocks.** Each component can be tested alone: packets in, packets out.
- **Variable depth.** A subnetwork can be used as a composite component, which is the contraction of Section 5.1.
- **Structure governs performance.** Bounded connections create backpressure, so throughput and latency are governed by the structure itself, as Section 5.4 requires.

Kahn process networks give a formal basis. If deterministic processes communicate only through FIFO channels with blocking reads, the output of the network does not depend on the order of execution (Kahn, 1974). Agent nodes are not deterministic, but their variance then stays local to the node, where it can be measured.

FBP is a data-flow problem model, which has two consequences. State-intensive or highly interactive problems fit it less well. In line with Section 5.1, they are modeled as separate problems with their own node and edge types instead of being forced into data flow. Cycles combined with bounded buffers can deadlock, so feedback edges such as review loops need a termination condition in their contract.

## 7 Convergent terminology

The approaches that fit the model come from software engineering, operating systems, hardware design, design theory and cybernetics. They use different terms but point to the same area of problem management (Table 5).

**Table 5.** Roles in the model and their names across traditions.

| Role in the model | Terms in different traditions |
|---|---|
| Node | module (Parnas), tool (Unix), component (FBP), process (Kahn), design parameter (axiomatic design) |
| Edge and contract | contract (Meyer), assume–guarantee contract (Benveniste et al.), port and information packet (FBP), pipe (Unix), visible design rule (Baldwin & Clark) |
| Hidden interior | information hiding, black box, hidden design parameters |
| Bound on coupling | connascence, design structure matrix, independence axiom, fan-in/fan-out, scattering, shotgun surgery |
| Variable depth | chunking (Miller), near-decomposability (Simon), subnetwork, level of abstraction |
| Check at the edge | reference monitor, test oracle, contract check |

The approaches are not duplicates but different views on one area:

- **Axiomatic design** addresses derivation. Functional requirements are mapped to design parameters. The independence axiom asks that each requirement depend on one parameter, and the information axiom asks for minimal information content (Suh, 1990).
- **Contract-based and platform-based design** addresses verification. It meets top-down requirements with bottom-up characterized components, an approach known as meet-in-the-middle (Sangiovanni-Vincentelli, 2007; Benveniste et al., 2018).
- **The design structure matrix** addresses the measurement of coupling (Steward, 1981; Eppinger & Browning, 2012).
- **Design rules** address economics. Modularity arises from visible rules fixed in advance and information hidden in the modules (Baldwin & Clark, 2000). This corresponds to the fixed exchange formats of Step 3.
- **Problem frames** model problems separately before composing them (Jackson, 2001).
- **FBP and Kahn networks** address execution.
- **Domain-driven design** (Evans, 2003) contains related notions such as bounded contexts, but the model does not depend on it.

That these approaches arose largely independently in different fields speaks for the substance of what they share. The vocabulary of agentic engineering (context engineering, harness engineering, agent–computer interfaces) adds further labels for the same area. One practical consequence follows. A project should fix one vocabulary in its repository, so that specifications, linters and prompts use the same term for the same thing. Terminological variety is itself accidental complexity.

## 8 Why the human analogy holds

The argument relies on the claim that agents and humans are analogous at the level of error classes and remedies. A common objection holds that this analogy is superficial, because language models were not designed as models of cognition. We argue that the analogy is well founded.

### 8.1 The argument

A language model learns the statistical distribution of human-produced artifacts: text, code, records of processes and their reviews. Training therefore transfers not only human patterns of expression but also human patterns of solving and of failing. At the same time, humans have to understand and develop the technology, and they do so by comparing it with themselves. The technical solution field is accordingly organized around human categories:

- Reasoning is elicited by having models think step by step (Wei et al., 2022).
- Memory for agents is organized into working, episodic, semantic and procedural memory, explicitly borrowed from cognitive architectures (Sumers et al., 2024).
- Interpretability research describes models in terms of planning and calls its subject a "biology" (Lindsey et al., 2025).
- The core mechanism is named after a human faculty: attention.

The analogy is thus not imposed on the technology from outside. It is built into its training data and into its design. Studies that apply cognitive-psychology tasks to language models find human-like behavior, including human-like content effects in reasoning (Binz & Schulz, 2023; Dasgupta et al., 2024). Practitioners also act on the analogy, and it works. The harness for long-running agents was explicitly modeled on the daily practice of effective human engineers, and its authors frame the problem as engineers working in shifts without memory of the previous shift (Young, 2025).

### 8.2 Objections

*(a) Origin of the mechanism.* Attention was introduced as an alignment mechanism for machine translation (Bahdanau et al., 2015). The transformer was adopted largely because it parallelizes training (Vaswani et al., 2017). Both points are correct but beside the point. The argument concerns the learned function and the solution field, not the historical motive behind a mechanism. That the mechanism was named "attention" illustrates the argument.

*(b) Same distribution, different mechanism.* Interpretability work reports that a model's explanation of how it solves a task can diverge from the mechanism it actually uses (Lindsey et al., 2025). Human reports about their own cognitive processes diverge from those processes too (Nisbett & Wilson, 1977). The objection describes a human property.

*(c) Optimization artifacts.* Reinforcement learning produces sycophancy and reward hacking (Sharma et al., 2024; Skalse et al., 2022). Ingratiation and Goodhart behavior are human patterns as well (Goodhart, 1975).

*(d) Operating conditions.* Agents are said to lack memory across sessions, to be copyable, and to be unusually sensitive to wording.

- **Memory.** Agent architectures include memory modules (Sumers et al., 2024). Memory loaded from an external store into the context is memory for the system, just as notes, wikis and handovers are memory for teams (Clark & Chalmers, 1998). In the model of Section 5, memory is an edge to a storage node.
- **Wording.** Sensitivity to wording is a framing effect (Tversky & Kahneman, 1981).
- **Copying.** Independent sampling from the same state has human counterparts in independent reviewers, second opinions and Delphi panels (Dalkey & Helmer, 1963). What differs is how precisely the starting state can be reproduced, which is a parameter value.

*(e) Anthropomorphism.* Comparing technology with oneself invites confirmation bias. This holds for any analogy, and it is itself a human property. Self-descriptions by AI systems that deny or affirm human likeness are trained behavior and carry no evidential weight in either direction.

Each objection, examined closely, describes a human property and therefore supports the analogy. Where differences remain, they are differences in parameter values, not in kind. This does not prove the analogy. Analogies are cheap, and their value lies in predicting behavior before it is observed. Two observations in the reported cases are such predictions in hindsight:

- Flat, equal-status groups of agents lost throughput and avoided difficult tasks, as human groups without ownership do (Lin, 2026).
- Handover-based harnesses worked the way shift handovers work (Young, 2025).

The practical consequence is that organizational psychology, cognitive-load research (Sweller, 1988) and research on teams become a source of testable hypotheses for agent setups.

## 9 Discussion: what differs are parameter values

Agents change values, not principles:

- Producing code becomes cheap, while checking and human attention become the bottleneck.
- Tacit knowledge that human teams supply through questions, culture and memory must be externalized, because an agent does not supply it by itself.
- Conventions are enforced mechanically, because review taste does not reach agents.
- Workers can be started by the hundred without onboarding, so coordination problems come to resemble those of distributed systems.
- Wording acts as a lever.

Each of these is a position on an existing axis, and positions can be traded against one another: tighter structure against more checking, independent agent review against human review, per-commit correctness against speed of correction.

The two divergences between the reports are such positions. Tolerating a small, stable error rate in exchange for fast correction (browser, OpenAI product) is one end of a trade-off between throughput and per-step correctness. Tightening continuous integration after regressions (compiler) is the other end. Human teams face the same trade-off. Flat coordination versus hierarchy is, as argued in Section 6.3, a function of coupling density.

One objection remains: a framework onto which every configuration can be mapped is refuted by none. Its content therefore lies in the structural rules of Section 5 and in parameter values, and both are measurable. This is why the thesis is to be tested on a real project rather than merely illustrated.

## 10 Planned evaluation

The thesis will be tested on a large project developed according to Sections 5 and 6, with the project instrumented as an experiment.

**Fixed in advance.** Only the following is fixed before work starts:

- the edge format and its rules: the form of contracts, the maximum number of edges per node (*d*), the maximum number of places per feature or aspect (*k*, initially three), and the permitted edge types;
- the end-to-end acceptance check, failing at first;
- a vocabulary file in the repository;
- the derivation of node properties and communication domains from the top level, whose results are
ReasoningToken: 112490
ReasoningToken: 10787
treated as requirements and checked as such.

**Enforced mechanically.**

- Linters check dependency directions and edge bounds. Their messages state the correction.
- Undeclared edges fail: agents run in containers without any permissions they have not declared.
- Repository files are the only medium of knowledge and coordination, and every session starts from them.

**Measured.**

- Change coupling from the version history, showing where independence failed (Gall et al., 1998; Zimmermann et al., 2005).
- Places touched per feature or aspect, and fan-in per edge, to test whether fan-in is indeed irrelevant under narrow contracts.
- Fault injection into specifications and code, recording which component (human, agent or deterministic tool) detects which fault and how strongly detections overlap (DeMillo et al., 1978).
- Repeated runs of identical node tasks, recording variance and success rate.
- Integration effort during composition (adapters, rework), as an indicator of whether the exchange formats hold up.
- Effort and cost per node, to calibrate cut sizes and the bounds *k* and *d*.
- A deviation log recording every instance in which a rule had to be broken, and why.

**Hypotheses.** The thesis implies, among others:

- **H1 Symmetry.** How much two components' detections overlap is predicted by their independence (shared model, context, authorship), not by whether they are human or agent.
- **H2 Isolation.** Change coupling between independent problems stays low. Where it does not, a cut was wrong, and correcting the cut lowers it.
- **H3 Bounds.** Changes touching more than *k* places show more defects and rework than changes within the bound.
- **H4 Aspects as edges.** Injected omissions of aspects, such as a missing log or permission check, are caught as missing edges by mechanical checks, not only in review or in operation.
- **H5 Qualities as effects.** Quality problems such as latency can be resolved by re-cutting, without budgets, derivation layers or selection by implementation.
- **H6 Bottom-up.** With fixed exchange formats, integration effort stays low and integration failures do not cluster at the end.
- **H7 Analogy.** Interventions taken from research on human teams have the predicted effect on agents, provided the prediction is recorded before the intervention.

**Expected weak points.**

- The independence hypothesis will fail in places, so cuts must remain correctable.
- Late integration bundles edge failures, so integration must be early and thin.
- Stateful and interactive parts do not fit a pure data-flow model and need problem models of their own.
- The end-to-end check is itself a fallible component. Where possible, a second, differently built check should complement it.

The deviation log is the main result, because it shows where the thesis reaches its limits.

## 11 Conclusion

Large-scale agentic development does not call for a new discipline. It calls for taking an existing one seriously. Complexity management supplies the foundation: no complete solution space exists, approximation works through structure and isolation, and checks come from components with diverse error profiles. Humans and agents are ambivalent components within it. Their roles are interchangeable, their error classes coincide, and the remaining differences are parameter values.

The structural consequence is a model reduced to nodes, edges with contracts, bounds on edges and checks at edges. In this model, aspects are edges and quality attributes are effects of the cut. The procedural consequence is to start from independent problems and use the top level once, to derive node properties and communication domains. Parts are then composed bottom-up without mocks, as in flow-based programming.

The many names for this area all describe one body of knowledge: information hiding, design rules, the independence axiom, contracts, pipes, context engineering and harness engineering. Agentic development will take time to recognize itself in it. The planned trial is meant to shorten that time and to show where the thesis ends. What agents demand explicitly was never optional for human teams either; experience merely concealed its absence.
