# From One Document to Isolated Work Packages

**A deterministic harness for the planned trial: bounded passes, self-contained atoms, contracts first, provenance for the operator only**

*Concept paper · Draft · October 2026 · Companion to the position paper "No New Discipline"*

---

## Abstract

The position paper *No New Discipline* proposes three things for a large agentic project. It should start from independent problems. It should use its top-level requirements once, to derive node properties and communication domains. It should then compose real parts bottom-up along fixed exchange formats. This paper specifies how the planned trial does this when the top-level requirements are too large for any single pass. Even in abstract form they run to about 10,000 lines.

The goal differs from the reported cases. The trial seeks safe and efficient development, not maximal autonomy. It therefore uses an elaborate, algorithmic harness with deterministic components, gates and boundaries. The pipeline works as follows:

- It segments the document deterministically.
- It derives a closed tag vocabulary.
- It tags units one-to-many in parallel, bounded passes.
- It rewrites units into self-contained atoms, and an independent component checks each rewrite for fidelity.
- Deterministic code computes the cut, checks the bounds and lists the cut edges. Agents change the result only through a closed set of operations.
- Every cut edge becomes a frozen exchange format before any package is built.

The output is a set of isolated work packages and separate integration packages. Provenance stays in an operator ledger that implementing agents never see. An agent that can trace its requirements will climb the requirements tree and lose the isolation the cut was meant to provide. No step needs a pass over the whole document, and every gate is itself measured.

**Keywords:** requirements partitioning · separation of aspects · deterministic harness · traceability · work packages · flow-based composition

---

## 1 Purpose and scope

Section 10 of the position paper announces a trial. A large project will be developed according to Sections 5 and 6 of that paper and instrumented as an experiment. This paper specifies the first and riskiest part of that trial: the step from a large requirements document to work that agents can execute in isolation. It also specifies how the results are composed and how they flow back.

The target differs from the projects in Section 2 of the position paper. Those projects explored how far agents get with few rules: a compiler built without an orchestrator, and a browser built by hundreds of agents in a week. Here the aim is safe and efficient development, so the harness may be elaborate and algorithmic.

- Wherever a decision can be computed, deterministic code computes it.
- Agents are used where a judgment is needed. Schemas, closed vocabularies and closed operation sets constrain their outputs.
- Gates separate the phases.
- Boundaries limit what each component can read and write.

The procedure descends once from the document to atoms and then builds upward: atoms into packages, and packages into integrated parts. The descent is the single use of the top level that Step 1 of the position paper allows. It leaves no persistent hierarchy behind. Hierarchy appears only as an optional view, formed by contracting dense subgraphs into composite nodes. Everything the descent produces (atoms, cuts, contracts) is treated as requirements and checked as such.

**Table 1.** Design rules.

| | Rule | § |
|---|---|---|
| D1 | **Bounded passes.** No agent step reads more than one chunk plus fixed reference material. No agent step writes more than the metadata of one chunk, one rewritten unit or one contract. Global steps are deterministic or operate on labels only. | 2.1, 5 |
| D2 | **Deterministic where computable.** Segmentation, graph construction, partitioning, bound checks, validation, export and the ledger are code. | 5 |
| D3 | **Closed sets.** Tags come from a closed vocabulary, and changes come from a closed set of operations. Extensions are proposals that take effect only after a merge gate. | 4.3 |
| D4 | **Lossless by invariant.** Coverage and definition invariants are checked after every phase. Nothing is lossless by trust. | 2.3, 8 |
| D5 | **Rewrite, then hide.** Units are rewritten into self-contained atoms. The applier writes provenance, and only validators and the operator read it. | 2.4, 4 |
| D6 | **Contracts first.** Every cut edge becomes a frozen, content-addressed exchange format before any package is built. Only the operator changes it. | 5, 7 |
| D7 | **Aspects become edges.** A cross-cutting tag is a queue to be emptied, not a layer. Quality requirements become checks at named edges, or they trigger a re-cut. | 5, 6 |
| D8 | **Bounds are gates.** Package size, places per aspect (*k*) and edges per package (*d*) are checked on the package graph. Violations trigger a re-cut. | 8 |
| D9 | **Closed packages, narrow return.** A package contains everything needed to complete it. An agent that cannot complete it files a structured blocker report instead of searching. | 7.3 |
| D10 | **Every gate is measured.** Gates that rely on agents are fallible components. Fault injection measures their detection rates. | 11 |

## 2 The problem

### 2.1 One pass does not suffice

The requirements document of the trial is large. Even its abstract description runs to about 10,000 lines. Three limits rule out deriving packages from it in one pass, whatever the model.

- **Input.** Context windows can hold the document, but accuracy declines with context length, and material in the middle of a long context is used less reliably (Liu et al., 2024; Hong et al., 2025). Finding one statement in a long context does not mean reasoning reliably over all of them.
- **Output.** A restructured document is about as long as its source. Output limits are separate from context limits and much smaller. Any step that restates the document, or a large part of it, in one response fails by construction.
- **Checkability.** A single pass produces one large judgment. Its errors cannot be localized, and a rerun produces a different large judgment.

The design response is D1. Every agent step has a bounded input and a bounded output. Steps that need a global view run on labels, which are small, or in deterministic code, which no context window bounds. The cost of the pipeline then grows linearly with the size of the document: one call per chunk per pass, plus merges on labels and local checks. Model-specific limits enter only as parameters: chunk size, package size and budget per call.

### 2.2 Partition first

The first task is not specification but separation of aspects: deciding which statements belong together and where the cuts lie. This is a categorization problem, solved in four moves.

1. One pass over the chunks proposes tags.
2. A merge on the label level turns the proposals into a closed vocabulary.
3. Parallel passes tag the document section by section, one unit to *n* tags.
4. Scopes are computed as the inverse index of the tags.

Every unit receives at least one tag, so scoping is lossless: the union of all scopes is the set of all units. Further passes can cut a scope further with a sub-vocabulary of its own, or derive edges between aspects.

Tagging yields scopes, not a cut. Two sources supply edges:

- **Co-tagging.** A unit carrying tags *a* and *b* links the two aspects.
- **Dependencies.** A unit uses a term that another unit defines. These edges are directed.

Deterministic code turns the annotated units into a graph and computes the cut (Section 5).

### 2.3 Congruence

The parts must fit together. We call a partition *congruent* if four conditions hold. Each can be checked mechanically.

- **C1 Coverage.** Every source unit has at least one active successor atom, or the operator has recorded its rejection. Every atom lies in exactly one package.
- **C2 Unique definition.** Exactly one atom defines each term.
- **C3 Resolution.** Every term an atom uses is either defined in its own package or crosses an edge as part of an exchange format.
- **C4 Agreement.** Both ends of every cut edge receive the same frozen contract, identified by its hash.

C1 replaces trust that a rewrite has lost nothing with a ledger check. C2 and C3 turn gaps and conflicts into reports. C4 makes the parts fit at composition time.

### 2.4 The inverse problem

Requirements traceability is a classic goal of requirements engineering (Gotel & Finkelstein, 1994). Here it is kept, but withheld from the implementing agents. An agent that can follow a requirement back to its source, its siblings and its parent section will do so whenever the package leaves a question open. It climbs the requirements tree up and down, loads material from outside its package, and loses the isolation the cut was meant to give it. Context grows and performance degrades. A first report suggests that inline back-references also reduce output determinism (Section 12; to be verified).

In the terms of the position paper, provenance is knowledge of neighbors. A node knows its edges, not the nodes at their other ends (Section 5.1 there). Two kinds of neighbors are involved: the requirements a package came from, and the packages that share their origin. Provenance therefore stays in the operator zone. A question that a package leaves open travels along one narrow edge, the blocker report, to the operator. The operator answers by changing the package, never by widening the agent's view.

Withholding provenance makes the rewrite the critical step. Once a meaning has drifted, the implementing agent cannot notice it. Two counterweights remain, and both are measured (Section 11):

- the fidelity gate (Phase 4 in Section 5);
- the executable acceptance checks inside the package.

## 3 Architecture: two zones

**Figure 1.** Zones and edges.

```
OPERATOR ZONE                                      AGENT ZONE
source document (read-only)
units ─ atoms ─ vocabulary ─ glossary
ops log ─► applier ─► ledger           compile     work packages (one container each)
validators, reports, overviews     ──────────────► integration packages
                                                   frozen contracts (read-only)
                                   ◄────────────── results, blocker reports
map local ids through the ledger                   (local ids only)
```

The system has two zones, a one-way export between them, and one narrow return edge.

The **operator zone** holds:

- the source document;
- the units and atoms;
- the vocabulary and glossary;
- the operations log and the ledger;
- the validators and all generated views.

Pipeline agents work in the operator zone: taggers, rewriters, checkers and restructurers. Each sees only the bounded input of its step, and none of them reads or writes provenance. The applier is a deterministic component. It executes their operations and records provenance in the ledger.

The **agent zone** holds the compiled packages and the frozen contracts. Each work package runs in its own container, with only that package mounted. Contracts are mounted read-only. Packages carry local identifiers only.

The **operator** is a role, not a component type. Most operator tasks are checks, and any component with a suitable error profile can perform them. Three tasks are accountability edges in the sense of Section 4.6 of the position paper:

- accepting inferred atoms;
- freezing contracts;
- releasing integrated parts.

The organization sets the contract of these edges.

**Table 2.** Terms of the method. They are kept in the repository as a vocabulary file (position paper, Section 7).

| Term | Meaning |
|---|---|
| Unit | A deterministic segment of the source (heading, paragraph, list item) with stable ID, line range and heading path. Never edited. |
| Atom | A self-contained statement, derived from one or more units or inferred. It holds one checkable statement and an acceptance criterion where possible. |
| Tag | A label from the closed vocabulary. A unit or atom carries one or more. |
| Scope | The set of units or atoms carrying a tag. Computed, never edited. |
| Package | A node: a set of atoms implemented in isolation. It owns the terms its atoms define. |
| Cut edge | A dependency between atoms in different packages, typed by the entity that crosses it. |
| Communication domain | The set of packages that exchange the same entity. |
| Contract | The frozen exchange format of a communication domain: schema, field semantics, example packets and contract tests. Content-addressed. |
| Integration package | A composition step: network definition, contracts, artifact slots, integration checks and the end-to-end check. |
| Ledger | Operator-zone record of provenance and outcomes: unit → atoms → package → results. |
| Blocker report | A structured message from an agent to the operator: reason code, local IDs and a short note. |

## 4 Data model

### 4.1 Atom files

Each atom is a Markdown file with YAML front matter, and all atom files sit in one flat directory. Grouping lives in metadata, not in folders, because moving files breaks references and diffs. Folder trees are generated views. The front matter holds only what pipeline agents may see:

```markdown
---
id: a-0412
kind: requirement          # requirement | constraint | definition | quality
tags: [session]
defines: [Session expiry]
uses: [Session, Idle timeout]
package: null              # set by the cut
status: active             # active | superseded | rejected
---
A session expires when no request referencing it has been received
for the idle timeout.

Acceptance: with an idle timeout of 30 minutes, a request that arrives
30 minutes after the previous request on the same session is rejected
as expired.
```

### 4.2 Ledger

Provenance lives in the ledger, keyed by atom ID. Only the applier writes it. Validators and the operator read it.

```yaml
a-0412:
  origin: source                 # source | inferred
  derived_from: [u-0731]
  src_range: "2210-2214"
  src_hash: 9f3c51e0
  basis: []                      # inferred atoms: the atoms they were inferred from
  superseded_by: null
  op: split#1187                 # entry in the operations log
  fidelity: {checker: cfg-B, verdict: none, run: 1}
  package: P-017
  outcomes: []
```

Because provenance is stored apart from the atoms, it is hidden by structure, not by filtering.

### 4.3 Vocabulary, glossary, operations

The **vocabulary** is closed and versioned. Each tag has four parts:

- a name;
- a definition;
- a boundary that states what the tag excludes;
- one or two example units.

Boundaries matter more than definitions, because they keep parallel taggers consistent. A tag may name a parent tag, but parents are used only for contraction views.

The **glossary** maps each domain term to its defining atom. Each package receives the subset of terms it uses, with the definitions inlined.

Agents change the atom set only through a closed set of operations. They write the operations to the operations log, and the applier executes them:

| Operation | Effect |
|---|---|
| `rewrite(u) → a` | replace a unit by a self-contained atom |
| `split(x) → a₁…aₙ` | divide a unit or atom into atoms with one statement each |
| `merge(a₁…aₙ) → a` | combine duplicates or fragments |
| `infer(basis) → a` | add an atom the source implies but does not state; it enters the operator queue |
| `retag(a, tags)` | change tags within the vocabulary |
| `move(a, π)` | assign an atom to a package |
| `group(a₁…aₙ) → π` | form a package |
| `contract(π₁…πₙ) → π` | contract dense packages into a composite node (a view) |

The applier rejects any operation that violates an invariant, and its message states the correction, as the linters in the reported projects do. Superseded atoms are marked with tombstones, never deleted. Parallel agents work on disjoint regions, which they claim through lock files in the repository.

### 4.4 Generated views

All overviews are generated, never edited by hand. With thousands of atoms, a flat overview exceeds any useful context, so the views are layered:

1. an index of packages and tags, with sizes and edge counts;
2. one summary per package or scope;
3. the atoms themselves.

An agent writes each summary from the atoms of one package, and the summary is regenerated when they change. An agent loads only the layer its step needs, for example one package with its cut edges. The other generated views are the scope index, the edge table, the glossary, a design structure matrix of the packages, and the reports of every gate.

## 5 Pipeline

**Figure 2.** Pseudo-algorithm.

```
INPUT   source document S
PARAMS  chunk size c, package size p, bounds k and d, thresholds θ, r_min, a_min
OUTPUT  operator zone: units, atoms, vocabulary, glossary, ledger, reports
        agent zone:    work packages, integration packages, frozen contracts
LEGEND  [D] deterministic   [A] agent step, bounded in and out   [O] operator

── Phase 0  Segment                                                     [D]
 1  U ← split S at headings, paragraphs and list items
 2  for u in U: u.id, u.range, u.path (heading path), u.hash
 3  C ← group U into chunks of ≤ c lines at unit boundaries
    G0: the ranges of U tile S exactly

── Phase 1  Vocabulary                                            [A, D, O]
 4  for c in C, parallel: cand[c] ← candidate tags and terms with short definitions
 5  V ← merge(cand) on labels: name, definition, boundary, examples, version
 6  T ← candidate glossary terms from cand
    G1: no two tags more similar than θ; |V| within bounds; O approves V

── Phase 2  Tag and extract                                          [A, D]
 7  for c in C, parallel; for u in c:
        u.tags            ← 1..n tags from V, or CROSS-CUTTING, or UNASSIGNED
        u.defines, u.uses ← terms from T;   u.refs ← unit ids
        u.proposed        ← new tags (inactive)
    output: ids and labels only, never text
 8  if proposals: V ← merge(V, proposals) [O]; repeat 7 for affected units only
    G2: every unit has ≥ 1 tag; ⋃ scopes = U; UNASSIGNED = ∅ or accepted by O;
        every used term has exactly one defining unit (0: gap, > 1: conflict)

── Phase 3  Rewrite                                                  [A, D]
 9  for u in U, parallel:
        reads   u, u.path, glossary entries of u.uses, units in u.refs
        writes  ops rewrite | split | infer → self-contained atoms,
                one checkable statement each, acceptance criterion where possible
10  applier executes ops; ledger records derived_from, basis, src_hash      [D]
11  repeat 7 for new atoms only
    G3: every unit has ≥ 1 active successor or a rejection by O;
        every atom has a source unit or origin = inferred; G2 holds for atoms;
        no atom refers to sections, units, "above" or "below"

── Phase 4  Fidelity                                              [A, D, O]
12  for u in U, parallel: verdict ← checker(u, successors(u))
        the checker differs from the rewriter in model or prompt
        classes: none | omission | addition | shift | scope change
13  inferred atoms → operator queue                                         [O]
    G4: all verdicts none or resolved by O;
        detection rate on seeded distortions ≥ r_min

── Phase 5  Cut                                                      [D, A]
14  graph over active atoms; weights uses→defines > refs > shared tags (× idf)
15  P ← partition with fixed seed; split parts > p recursively;
        merge small parts with strong mutual coupling                      [D]
16  agents review one region each (overview + metadata);
        ops move | split | merge | group | contract                        [A]
17  applier executes ops; recompute 14–15                                  [D]
18  contradiction check on atom pairs sharing a term,
        within each package and along each cut edge                        [A]
    G5: every atom in exactly one package; size ≤ p;
        partition stable across seeds (ARI ≥ a_min); no open contradiction

── Phase 6  Aspects and qualities                                    [A, O]
19  for each atom tagged CROSS-CUTTING, resolve into either
        an aspect node with explicit edges at named packages, or
        atoms local to packages ("package π emits E")
20  for each quality atom: a check at a named edge, or a re-cut request
    G6: no CROSS-CUTTING atom left; every aspect touches ≤ k packages

── Phase 7  Communication domains and contracts                   [D, A, O]
21  E ← cut edges (producer, consumer, entity, evidence atoms)              [D]
22  domains ← group E by entity                                            [D]
23  for each domain, parallel: draft the exchange format from its evidence
        atoms only: schema, field semantics, example packets, contract tests [A]
24  check: schema valid; every consuming use covered;
        ≥ 1 producer and ≥ 1 consumer                                      [D]
25  O freezes; contract id ← hash(contract)
    G7: every package has ≤ d edges; every cut edge has a frozen contract

── Phase 8  Compile                                                     [D]
26  for each package π: goal, atoms as R1..Rn, glossary subset,
        contracts (read-only), acceptance checks, report schema;
        strip ids, provenance, section numbers, names of other packages
27  for each composition step: integration package (network definition,
        contracts, artifact slots, integration checks, end-to-end check)
    G8: export lint: no provenance, no references outside the package,
        size within budget

── Phase 9  Execute and integrate                               [agent zone]
28  per package: fresh session; container with only this package mounted
29  build against the acceptance checks: packets in, packets out; no mocks
30  as soon as two connected packages pass: run their integration package
31  every run ends with a result report or a blocker report
    G9, G10: checks pass; reports conform to the schema

── Phase 10 Return                                                   [D, O]
32  map local ids through the ledger; record outcomes                       [D]
33  blocker reports, adapters, change coupling → O: fix atoms, contracts or cut;
        rerun affected phases (cached by input hash); recompile affected packages
```

### 5.1 Notes on the phases

- **Tagging comes before rewriting.** Tags and the glossary bound the rewrite. A rewriter sees one unit, its heading path, the glossary entries of the terms it uses, and the units it references explicitly. It sees nothing else. Splitting along tags also produces atoms with one aspect each. After the rewrite, only the new atoms are tagged and extracted again.
- **Rewrite rules.** An atom is complete when it can be understood without its surroundings. The rewriter resolves references such as "as above" or "see 3.2", replaces pronouns, states implicit context from the enclosing sections, and uses terms in their glossary form. Each atom carries one checkable statement. Two existing standards serve as targets: template syntaxes such as EARS (Mavin et al., 2009), and the characteristics *singular*, *unambiguous* and *verifiable* from ISO/IEC/IEEE 29148.
- **Fidelity checks are independent.** The checker differs from the rewriter in model or prompt. It sees only the source unit and its successors, and it returns a classification, not prose. A checker that shares the rewriter's configuration adds less than its count suggests (Knight & Leveson, 1986). Optionally, a third configuration reconstructs the source statement from the atoms, and the reconstruction is compared with the source.
- **Cuts are deterministic.** Dependency edges weigh most and explicit references less. Shared tags weigh least, and they are weighted by inverse frequency, so that ubiquitous tags do not dominate. The partition uses community detection (Blondel et al., 2008; Traag et al., 2019) with fixed seeds. Such algorithms can be unstable on sparse graphs, so stability across seeds is a gate. Agents review the computed cut through operations. They do not draw it from scratch.
- **Contradiction checks are local.** Atom pairs are compared only within a package and along cut edges, and only if they share a term. A global pairwise comparison never happens.
- **Aspects and qualities.** Units with high fan-in across packages are candidates for aspects, as in aspect mining (Marin et al., 2007). They are not collected into a cross-cutting layer. Each is resolved into an aspect node with explicit edges at specific packages, or into atoms local to packages (Section 6 gives an example). Quality requirements are treated as effects of the cut. Each becomes a check at a named edge, or a signal to re-cut (position paper, Section 5.4).
- **Contracts.** An agent drafts the exchange format of a domain from the evidence atoms of that domain only. Deterministic checks confirm that the schema is valid and that a field covers every consuming use. The operator then freezes the result. Contracts are content-addressed, so agreement (C4) is a hash comparison.

## 6 Worked example

Unit `u-0731` covers lines 2210–2214 under the heading path *3 Sessions › 3.4 Expiry*:

> As described above, they expire after the configured idle time (default 30 minutes). Every expiry must be logged. Only administrators may change the idle time.

The unit has two problems:

- "As described above" and "they" point outside the unit.
- It mixes behavior, an aspect (logging) and access control.

The rewriter resolves "they" through the heading path and splits the unit into five atoms:

| Atom | Tags | Statement |
|---|---|---|
| a-0412 | session | A session expires when no request referencing it has been received for the idle timeout. |
| a-0413 | configuration | The idle timeout is a configuration value. Its default is 30 minutes. |
| a-0414 | session, audit | When a session expires, the session package emits one `SessionExpired` event with session ID, user ID and time. |
| a-0415 | configuration, access | Changing the idle timeout requires the role Administrator. The check takes place when the new value is written. |
| a-0416 | audit | Every `SessionExpired` event is recorded in the audit log. |

The cut assigns the atoms to four packages and yields three communication domains:

| Package | Atoms |
|---|---|
| Sessions | a-0412, a-0414 |
| Configuration | a-0413, a-0415 |
| Audit | a-0416 |
| Identity | (supplies roles) |

| Domain | Producer | Consumer |
|---|---|---|
| `session-expired` | Sessions | Audit |
| `idle-timeout` | Configuration | Sessions |
| `role` | Identity | Configuration |

The logging requirement has become one edge at one place. The audit aspect touches two packages, within *k* = 3. The access check sits at the write of the value, the boundary of the protected resource (position paper, Section 5.3).

The session package, as exported to the agent zone:

```markdown
# Package: Sessions

## Goal
Maintain sessions and end them after inactivity.

## Requirements
R1  A session expires when no request referencing it has been received
    for the idle timeout.
R2  When a session expires, emit one SessionExpired event
    (contract session-expired@3f9a).

## Inputs
idle-timeout@b71c      current idle timeout

## Outputs
session-expired@3f9a

## Glossary
Idle timeout           the period without requests after which a session expires
```

The export contains no unit or atom IDs and no section numbers. It does not mention auditing or configuration as packages. The event exists because of an audit requirement, but the package knows only the contract it must fulfill.

## 7 Work packages and integration packages

### 7.1 Work packages

```
package/
  TASK.md           goal (package summary), scope, non-goals, definition of done
  requirements.md   atoms as R1..Rn, flat, ordered by dependency
  glossary.md       terms used, with definitions
  contracts/        input and output exchange formats (read-only)
  acceptance/       executable checks: packets in, packets out
  report.schema     format of result and blocker reports
```

The export linter (G8) enforces four rules:

- No source IDs, section numbers, provenance fields or history.
- No names of other packages. Outside the package, only contracts exist.
- The package fits within the package budget.
- Every requirement has an acceptance check or is marked as checked by review.

Acceptance checks feed in packets that conform to the input contracts and check the packets that come out. They replace mocks: the package is tested against data at its edges, not against simulated neighbors.

### 7.2 Integration packages

An integration package composes parts that have passed their own checks. It contains:

- the network definition that connects the parts;
- the contracts on the connecting edges;
- slots for the built artifacts;
- integration checks;
- the end-to-end acceptance check of the system.

It knows the parts through their artifacts and contracts, never through their requirements. It may change the network definition but not a part. In the terms of flow-based programming, composing the network changes no component (Morrison, 2010). Artifacts are selected by contract hash, never by implementation (position paper, Section 5.5).

Integration starts early and thin. As soon as two connected packages pass their checks, their integration package runs. Adapters written during integration are recorded, because each one shows an exchange format that did not hold.

### 7.3 The return edge

Every run ends with a report in a fixed schema. A result report lists the checks passed and the artifacts produced. A blocker report carries one reason code:

| Code | Meaning |
|---|---|
| `UNDERSPECIFIED` | a requirement admits more than one reasonable implementation |
| `CONTRADICTION` | two requirements cannot both hold |
| `CONTRACT_INSUFFICIENT` | an edge lacks a field or a guarantee the package needs |
| `CHECK_DEFECT` | an acceptance check is wrong or cannot be run |
| `OUT_OF_BOUNDS` | the package cannot be completed within its budget |

The report names local IDs and contains a short note. The operator then proceeds in three steps:

1. Map the report through the ledger.
2. Fix the atoms, the contracts or the cut.
3. Recompile the affected packages.

The agent never receives the trace, only a new package version.

## 8 Gates and boundaries

**Table 3.** Gates. D = deterministic, A = agent, O = operator.

| Gate | After | Check | By | On failure |
|---|---|---|---|---|
| G0 | Segment | units tile the source exactly | D | fix the segmenter |
| G1 | Vocabulary | no near-duplicate tags; size within bounds; approval | D, O | merge, split or redefine tags |
| G2 | Tag, extract | every unit tagged; scopes complete; each used term defined exactly once | D | re-tag; gap or conflict report |
| G3 | Rewrite | ledger coverage; G2 for atoms; no dangling references | D | rewrite the unit again |
| G4 | Fidelity | no distortion found; inferred atoms decided; detection rate ≥ *r_min* | A, O | rewrite again; operator decision; replace the checker |
| G5 | Cut | one package per atom; size ≤ *p*; stable partition; no open contradiction | D, A | re-cut; revise the vocabulary |
| G6 | Aspects | no cross-cutting atom left; each aspect at ≤ *k* packages | D | resolve the aspect; re-cut |
| G7 | Contracts | ≤ *d* edges per package; every cut edge under a frozen contract | D, O | re-cut; draft the contract |
| G8 | Compile | no provenance, no outside references, size within budget | D | fix the compiler |
| G9 | Execute | acceptance checks pass; report conforms to schema | D | rerun, or blocker report |
| G10 | Integrate | integration and end-to-end checks pass | D | blocker report to the producing package |

**Boundaries.**

- Each work package runs in a container with only its package mounted. Undeclared edges fail: there is no network and no access to the operator zone.
- Contracts are read-only in the agent zone.
- Pipeline agents write only to the operations log. Only the applier writes atoms and the ledger.
- Parallel restructuring agents claim regions through lock files.

Deterministic gates are reproducible but not infallible. Their specification can be wrong and their coverage incomplete (position paper, Section 4.3). They are therefore tested like code. Every defect that slips past them is entered in the deviation log.

## 9 Operator loop: change, re-cut, replan

A fixed decomposition is a hypothesis. Some structure becomes visible only during implementation and makes cuts wrong, and a process that cannot revise its cuts fails systematically (cf. Adaptive Task Graphs, Section 12). Three signals trigger a revision:

- blocker reports;
- adapters written during integration;
- change coupling mined from the version history (Gall et al., 1998; Zimmermann et al., 2005).

Revisions are incremental. Every agent step is cached by the hash of its input, the vocabulary version and the prompt version. A changed source unit is traced through the ledger to its atoms, packages and contracts. Only the phases for those elements rerun, and only the affected packages are recompiled.

A contract change is the expensive case. It reaches every participant of its domain, requires an operator decision, and invalidates the integration packages on that edge. Agents always receive the current package version and never its history.

## 10 Correspondence with the position paper

**Table 4.** Elements of the position paper and their mechanisms here.

| Position paper | Mechanism here |
|---|---|
| P2, §4.4: independence decides complementarity | the fidelity checker differs from the rewriter in model, prompt or context; detection rates are measured (H9) |
| P3: every check is fallible | gates measured by fault injection; deterministic gates tested like code; deviation log |
| P5: nodes know their edges, not their neighbors | packages contain contracts, no neighbor names and no provenance |
| P6: bounded places and edges | gates G5–G7: size ≤ *p*, places per aspect ≤ *k*, edges per package ≤ *d* |
| P7: aspects are edges | CROSS-CUTTING is a queue, resolved into edges at named packages (G6) |
| P8: qualities are effects of the cut | quality atoms become edge checks or trigger re-cuts |
| P9: selection by contract | integration packages select artifacts by contract hash |
| P10, Step 1: top level used once | a single descent to atoms; no persistent hierarchy; contraction only as a view |
| P10, Step 2: independent problems | packages with their own atoms, terms and checks; tested by change coupling |
| P10, Step 3: communication domains | cut edges grouped by entity; one frozen exchange format per domain |
| P10, Step 4: bottom-up without mocks | packets in, packets out; integration early and thin |
| §7: one vocabulary | closed tag vocabulary, glossary and method terms (Table 2) in the repository |

## 11 Evaluation

The measures and hypotheses H1–H7 of the position paper (Section 10 there) apply unchanged. The pipeline adds the following measures:

- agreement between two independent tagging runs on the same units (Krippendorff, 2004);
- stability of the partition across seeds and runs (Hubert & Arabie, 1985);
- gap, conflict and coverage reports per phase;
- the detection rate of the fidelity gate on seeded distortions (DeMillo et al., 1978), in four kinds: omission, addition, shift of meaning and change of scope;
- per package: tokens consumed, blocker reports by reason code, and file accesses outside the package (zero by construction, logged by the container);
- the share of defects caught before execution by deterministic gates, by agent checks and by the operator;
- the effort of contract changes and re-cuts;
- calls and tokens per pass as a function of document size.

**Hypotheses.** The numbering continues from the position paper.

- **H8 Closed vocabulary.** Tagging with a closed vocabulary that states boundaries yields higher agreement between independent runs than tagging with open labels. The resulting partitions are stable across seeds.
- **H9 Independent fidelity checks.** A fidelity checker that differs from the rewriter in model or prompt detects more seeded distortions than one that shares the rewriter's configuration. This is H1 applied to the rewrite.
- **H10 Hidden provenance.** Packages exported without provenance consume fewer tokens per completed package and produce more consistent outputs across repeated runs than the same packages with back-references, without more defects. A random half of the packages is also exported with back-references for the comparison.
- **H11 Determinism dividend.** Deterministic gates, not agent checks, catch most of the defects caught before execution.
- **H12 Linear cost.** The calls and tokens of the pipeline grow linearly with document size.

**Expected weak points.**

- **Shared blind spots.** Rewriter and checker may miss the same drift. Independence is measured, not assumed, and the acceptance checks in the packages remain the last line of defense.
- **Vocabulary granularity.** A vocabulary that is too coarse yields oversized scopes. One that is too fine yields many multi-tagged units and dense coupling. Scope sizes and the share of multi-tagged units are the indicators. The remedy is a revised vocabulary and an incremental rerun.
- **Contract timing.** Contracts frozen too early churn. Contracts frozen too late let integration failures cluster.
- **Inferred atoms.** Extrapolation can introduce requirements nobody asked for. Inferred atoms therefore enter packages only after operator acceptance.
- **Operator bottleneck.** Review queues grow with the project, and reviewers of plausible output drift into approval (Parasuraman & Manzey, 2010). Queues are bounded, and operator decisions are sampled and checked like those of any other component.
- **Stateful and interactive parts.** They do not fit data-flow packages. They are modeled as separate problem models with edge types of their own.

## 12 Related work and verification status

We found no work that describes the pipeline as a whole, but several works address its components. The 2026 preprints listed below were seen only as abstracts or snippets. Their claims must be verified before a final version cites them.

**Table 5.** Related work.

| Topic | Work | Relevance | Status |
|---|---|---|---|
| Deterministic control | *A Deterministic Control Plane for LLM Coding Agents* (arXiv 2606.26924) | phase state machine; requirements hash-addressed at approval; requirement → file → test trace; governance not delegated to LLMs | abstract only |
| | *Verification as an Architectural Layer for LLM Agents* (arXiv 2609.31937) | deterministic gates separated from LLM judges; in a pilot, 8 of 9 corrections came from gates | abstract only |
| | Deterministic envelope / controlled variation space (arXiv 2610.06255) | agent variation bounded by schema gates | abstract only |
| | Reddy et al., *Reason less, verify more* (arXiv 2607.07405) | deterministic gates against silent policy violations | reference list only |
| Requirements clustering | ARLO (arXiv 2504.06143) | clustering of requirements before deriving architecture | to verify |
| | Cluster-aware dual-level test specification (arXiv 2606.17197) | UMAP and HDBSCAN on requirement corpora | abstract only |
| | iReFeed (arXiv 2603.28677) | more "requires" relations found within topic clusters than in the whole set | abstract only |
| | RADIANT (arXiv 2607.16708) | requirement-to-verification trace; multi-agent decomposition improves syntactic validity | abstract only |
| Contracts | Contract-Coding (arXiv 2604.13100) | a language contract as a symbolic layer for repository generation | abstract only |
| | MetaGPT (Hong et al., 2024) | structured intermediate artifacts raise success rates | published |
| Partitioning | GraphRAG (Edge et al., 2024) | hierarchical Leiden partitioning with precomputed summaries | published |
| | k-core variant (arXiv 2603.05207) | Leiden unstable on sparse graphs; k-core yields deterministic hierarchies | abstract only |
| Provenance in context | *Citation Discipline in Spec-Driven Development* (arXiv 2606.30689) | inline back-references lower output determinism (d ≈ −0.7) | abstract only |
| Fixed decomposition | Adaptive Task Graphs (arXiv 2605.06320) | decompositions fixed before execution fail on structure visible only later | abstract only |
| | SAFEdit (arXiv 2604.25737) | role decomposition alone does not fix reliability | secondary source |
| Aspects in requirements | Theme/Doc (Baniassad & Clarke, 2004); aspectual requirements (Rashid et al., 2003) | aspects identified through shared terms in requirement text; there they become crosscutting themes, here edges | published |
| Surveys | He et al. (2024); NLP4RE survey (arXiv 2401.01508) | overviews | published / to verify |

Two gaps remain. We found no work on rewriting requirements into self-contained atoms with an independent fidelity check. We also found no work on separating operator-level from agent-level traceability. The second design decision is supported only indirectly.

## 13 Conclusion

The position paper claims that large-scale agentic development is complexity management. This concept turns the claim into a procedure for the step where it is hardest to uphold: going from a document no component can hold at once to work that components can complete in isolation.

- Every agent step is bounded.
- Every global step is deterministic or operates on labels.
- Every invariant is checked rather than trusted.
- Atoms are rewritten until they stand alone.
- Cut edges become frozen exchange formats before anything is built.
- Provenance stays with the operator.

Agents receive closed packages and a narrow edge for their questions. The trial measures whether this keeps the cuts independent, the rewrites faithful and the integration cheap. The deviation log will show where it does not.
