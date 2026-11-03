## Encapsulation, Generations and Exclusive Clusters: Deriving a Directory Structure from a Module Dependency DAG

### Abstract

Mapping a module dependency DAG onto a hierarchical package structure is usually done by hand or by clustering heuristics. We argue that a tree is the wrong primary structure for such graphs, because shared modules have users in unrelated subtrees and no tree position can express that. We propose a three-stage derivation. First, a strict encapsulation order places a module under another only if it is hidden from all other users. Second, a generation order assigns every unit its composition depth. Third, within each generation, units are grouped into exclusive clusters when they share dependencies on earlier generations. The result is projected onto a directory chain in which each directory holds the units of one generation and contains the directory of the previous generation. The structure is a pure function of the graph, makes primitives and composites directly visible, and provides invariants that can be enforced as architecture rules. We report one qualitative case study and discuss limitations. The individual building blocks are known (levelization, DSM partitioning, dominator-based clustering).

### 1. Introduction

Software architecture is commonly presented as a tree: packages contain packages, and a module has one parent. Dependency structures are DAGs. Deriving a tree from a DAG therefore loses information or invents it. Where a module is used from two unrelated parts of the system, any parent is either arbitrary or a visibility leak.

This paper starts from a specific setting. The modules are not hand-written packages. They are the result of aggregating requirements by their intersections, so that each module already minimizes its interface and maximizes structural safety. A second optimization with the same objective finds nothing more to merge. Consequently the graph carries the full information, and any tree can only be a projection of it.

Contributions:

1. A hard encapsulation order based on exclusive closure with bounded interface width `K`.
2. A generation order (longest path to the leaves) as a complete, parameter-free layering of all units.
3. An exclusive clustering within generations, derived from shared lower-generation dependencies.
4. A directory projection in which depth equals compositional depth, with invariants suitable for architecture rules.
5. A record of approaches that failed, and why.

### 2. Background and Related Work

**Information hiding.** Parnas \[1] proposed decomposing systems by the design decisions likely to change, hiding each in one module. The module guide \[2] adds a hierarchical document structure. Our first order operationalizes hiding on the dependency graph: what is hidden is what has no users outside its package.

**Levelization.** Lakos \[3, 4] defines level numbers for components: a component’s level is one plus the maximum level of the components it depends on, cyclic dependencies are forbidden, and the levelization supports incremental testing and bounds the cumulative component dependency. He also contrasts “heavy” layered architectures with lateral designs. Our second order is this level number, applied to units of the first order. Lakos uses levels as a design and testing discipline, with the level encoded in package prefixes and documentation, not as directory depth.

**Design structure matrices.** DSMs \[5, 6] represent dependencies as a square matrix. Partitioning reorders the matrix toward triangular form, which corresponds to a layering, and clustering finds blocks of coupled elements. Our generations correspond to partitioning, our clusters to a dependency-based block structure.

**Software clustering.** Bunch \[7, 8] searches partitions of a module dependency graph with a fitness function that rewards cohesion and penalizes coupling, using hill climbing and genetic algorithms. ACDC \[9] applies structural patterns, including a subgraph-dominator pattern that groups a module with the modules it dominates. Our first order for `K=1` coincides with dominator grouping.

**Layer recovery.** A body of work recovers layers from code, using rank, fan-in and fan-out, clustering, ego networks \[10] and responsibility and abstraction principles \[11]. Typical rules are the back-call ban, the skip-call ban and acyclicity \[12]. Those approaches usually tolerate violations and heuristically ignore offending dependencies. Our layering is exact on the DAG and has no violations by construction.

**Industrial practice.** Layering is enforced in practice by acyclic import rules (Go, Cargo workspaces), hand-defined layers checked by tools (ArchUnit, import-linter, visibility rules in monorepos), and levelized libraries in the Lakos tradition. In these cases the layers are authored and checked, not derived.

### 3. Problem Setting

Let `G = (V, E)` be a DAG, with an edge `a → b` meaning that `a` needs `b`. The users of `b` are its predecessors. Modules with no users are roots. Modules with no dependencies are primitives. Cycles are rejected before any processing.

The goal is a structure that (a) is uniquely determined by `G` and independent of input order, (b) gives guarantees that can be checked per edge, and (c) is legible to humans without claiming ownership relations that do not exist.

### 4. Failed Approaches

The final design is the result of eliminating alternatives. Each failure was informative.

1. **Inverted topological list with deepest placement.** Hoisting modules to the common ancestor of their users converges to a flat list, because in real graphs almost everything is shared by something.
2. **Dominator tree.** For a DAG, the immediate dominator is the lowest common ancestor of the predecessors, so the “deepest placement” rule is already the dominator tree. It is unique but flat, since depth arises only from exclusive dependency. Thresholds on hub size change membership of a `shared` package, not depth.
3. **Root signatures.** Grouping modules by the set of roots whose subtree contains them gives a lattice by set inclusion. Depth was good but cohesion was poor, and it required synthetic package nodes.
4. **Greedy contraction with bounded interface width.** Repeatedly merging a module into a user while the package keeps at most `K` externally used modules gives depth, but the result depends on the merge order, which carries no semantics. A single-pass variant in canonical order was flatter.
5. **Similarity clustering.** Agglomerative clustering with Jaccard distance over users and dependencies, and an annealing search over parent assignments, did not merge anything or gave unsatisfying results. The remaining modules have users in unrelated subtrees, so their neighbor sets barely overlap. Distance and leak metrics also collapse the identity of a connection: all members of a package count as one neighbor, all neighbors weigh the same, and edges carry no type.

The common cause: for modules with users in separate subtrees, the graph contains no information from which a tree could be derived. Any position is either a leak or arbitrary. This is not a defect of the metric.

### 5. Method

#### 5.1 First order: encapsulation

For a head set `H` with `|H| ≤ K`, the closure `S(H)` is the largest subset of `H` and the descendants of `H` in which every member outside `H` has users only inside `S`. It is computed by removing members that have an outside user and propagating the removal to their dependencies.

Candidates are head sets that share a dependency (otherwise the heads are unrelated) and are minimal (no head is itself hidden). The value of a candidate is the number of hidden modules, `|S| − |H|`.

Closures of different head sets either nest or are disjoint in the cases where selection succeeds. The selection keeps a laminar family: candidates are sorted by value, then by `|H|`, then by names, and a candidate is accepted if it is disjoint from or nested with every accepted one. The result is independent of input order. For `K=1` it equals the dominator tree. The greedy selection is deterministic but not optimal. An exact tree dynamic program over the laminar candidates is possible and left open.

Each accepted `S` becomes a package. A single head is the package, with the module itself as its entry. Heads with `|H| > 1` receive a neutral label (“Package n”). Modules in no package remain individual units. Outermost packages and unpackaged modules together form the **units** of the next stages.

**Guarantee.** At most `K` modules of a package are used from outside. Everything else can be changed without affecting any outside user.

#### 5.2 Second order: generations

Contract the graph onto units. Strongly connected components between units, which can arise from the contraction, share one generation. Then

`gen(u) = 1 + max{ gen(d) : u needs d }`, with `gen(u) = 1` for units without dependencies.

Every edge leaves a unit toward a lower generation or stays inside the unit. The layering is complete (every unit has a generation, including shared ones), unique, parameter-free, and implies acyclicity. Generation is a measure of composition: generation 1 holds primitives, every higher generation composes from lower ones.

#### 5.3 Third order: exclusive clusters

Within a generation, two units are put into one cluster when they have a common dependency in a lower generation. The relation is closed transitively (union-find). Clusters of the same generation therefore have disjoint sets of lower dependencies, and every unit lies in exactly one cluster. Units with no overlap remain individual. Clusters express role: siblings with the same composition depth and separate foundations, typically alternative implementations of one role.

A hub, a lower module used by very many units, joins everything into one cluster. A threshold on user count excludes such dependencies from the overlap test.

#### 5.4 Directory projection

The highest generation is the root directory. Each directory contains, in this order, its individual units, then its clusters, and a subdirectory for the previous generation, which all siblings share. Directory depth equals the number of generations. Because every dependency points into a lower generation or into the same unit, a module only depends on its own chain downward, and that chain is its reading closure.

The rule applies recursively. The content of a package is a subgraph with its own generations and clusters, and the heads form the top layer. Cycles inside a package share a generation, as on the top level.

### 6. Invariants and Change Rules

The structure is an invariant against which changes are checked.

| Rule            | Statement                                                                   |
| --------------- | --------------------------------------------------------------------------- |
| Encapsulation   | An edge crossing a package boundary targets a public module of that package |
| Interface width | At most `K` public modules per package                                      |
| Generation      | An edge between units targets a strictly lower generation                   |
| Acyclicity      | Follows from the generation rule                                            |
| Depth           | `max gen` stays below a threshold (here 15)                                 |

Rules are local: each needs one edge and the structure, no recomputation.

A new edge that violates a rule triggers a re-placement of the unit involved, not a rejection. Structural change, such as adding a directory, is rare. It happens when the depth threshold is exceeded, and it needs a component that is non-exclusive (several users, otherwise it would be a first-order inclusion) and that is itself a nesting (a bundled subgraph). A dependent object beyond the threshold is treated as a symptom of a missing abstraction or a wrong requirement cut.

Absolute generation numbers are not stable, since a new dependency at a leaf shifts everything above. Rules should therefore be expressed on edges (`gen(target) < gen(source)`), not on absolute numbers.

### 7. Case Study

The method was applied to a component graph of roughly 100 modules from an embedding and index system. We report qualitative observations only.

* The first order hid a part of the modules in packages. Roughly 60 modules with two to five users remained unhidden. These were shared services whose requirement cut was already the result of intersection aggregation, so no further isolation was available.
* In the generation order, the CPU and the GPU sub-engines appeared side by side in one cluster, each with its own foundations and shared services below them. No names or descriptions entered the algorithm.
* Reviewers found the result intuitive. It was clear which modules are primitives and which are composites.

There is no quantitative evaluation, no comparison with baselines, and one system only. These observations motivate the method and do not validate it.

### 8. Discussion

**Why a tree fails and layers do not.** A tree states ownership. A shared module has several owners. A generation states only depth, so it never claims a relation that does not exist. The tree remains valid exactly where ownership is exclusive, which is the first order.

**Depth as measurement.** Directory depth is equal in cost to nested packages in a classical design, which reach similar path lengths. The difference is meaning: here depth is the composition depth. A dependent module far beyond the expected depth indicates a structural problem. This turns the structure into a diagnostic.

**Role of the graph.** The graph carries all information. The directory structure is a projection, and other projections (by root signature, by package) are possible. Only the generation is guaranteed complete. For AI-assisted development this suggests querying the graph (blast radius as the closure of users, reading context as the closure of dependencies) and treating the tree as a rendering for humans.

**Cost of restructuring.** In languages whose module path is independent of the directory (for example a Cargo workspace with explicit paths), inserting an intermediate directory is cheap. In languages where the path is the name, it requires tool support. Since the structure is generated, such tooling is an acceptable cost.

### 9. Limitations

* Selection in the first order is greedy and not optimal. The candidate set grows as `O(n^K)`.
* Longest-path rank overstates the complexity of a module with a single deep dependency.
* Clusters are defined per generation. Strands across generations are not identified.
* Hubs can merge a generation into one cluster and need a threshold.
* Semantics beyond the graph (names, descriptions, change history) is not used.
* The evaluation is a single qualitative case.

### 10. Conclusion

A DAG of aggregated modules contains the information for a complete layering and a partial, exclusive nesting, and not more. The proposed pipeline derives both deterministically, adds exclusive clusters inside generations, and projects the result onto a directory chain whose depth is a meaningful measure. The building blocks come from levelization, DSM partitioning and dominator clustering. Whether the combination holds up on further systems, and how well it supports automated change rules, remains to be tested.

### References

\[1] D. L. Parnas. On the criteria to be used in decomposing systems into modules. *Communications of the ACM*, 15(12), 1972.\
\[2] D. L. Parnas, P. C. Clements, D. M. Weiss. The modular structure of complex systems. *IEEE Transactions on Software Engineering*, SE-11(3), 259–266, 1985.\
\[3] J. Lakos. *Large-Scale C++ Software Design*. Addison-Wesley, 1996.\
\[4] J. Lakos. *Large-Scale C++ Volume I: Process and Architecture*. Addison-Wesley, 2020.\
\[5] D. V. Steward. The design structure system: a method for managing the design of complex systems. *IEEE Transactions on Engineering Management*, 28(3), 71–74, 1981.\
\[6] S. D. Eppinger, T. R. Browning. *Design Structure Matrix Methods and Applications*. MIT Press, 2012.\
\[7] S. Mancoridis et al. Bunch: a clustering tool for the recovery and maintenance of software system structures. 1999.\
\[8] B. S. Mitchell, S. Mancoridis. On the evaluation of the Bunch search-based software modularization algorithm. *Soft Computing*, 12(1), 77–93, 2008.\
\[9] V. Tzerpos, R. C. Holt. ACDC: an algorithm for comprehension-driven clustering. 2000.\
\[10] S. Thakare, A. W. Kiwelekar. Discovery of layered software architecture from source code using ego networks. arXiv:2106.03040, 2021.\
\[11] A. Boaye Belle et al. Recovering software layers from object oriented systems. 2014.\
\[12] Systematic literature review on layering principles in software architecture reconstruction. arXiv:2112.01644, 2021.\
\[13] T. Lengauer, R. E. Tarjan. A fast algorithm for finding dominators in a flowgraph. *ACM TOPLAS*, 1(1), 1979.
