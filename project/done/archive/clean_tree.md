Bereinige das folgende Skript.
- Alle Kommentare in Englisch
- Auskommentierten Code entfernen
- Prüfen, ob die Kommentare noch zum Code passen
- Direkt hier iweder ausgeben

```python
[include](/home/user/xyan/xy.ai.workbench/project/resident_02/00_approach/26_tree.py)
```
Result Stats: id=c69f6142-8ff3-4e31-b339-47f30a5b0588, total: 13425, in: 4, out: 8508, reason: 3705, read: 811, write: 4913, start: 1793742582557, end: 1793742635354, time: 52581, total: 58724, io: 0, control: 216, sent: 1, recv: 688
SystemInit:  id=c69f6142-8ff3-4e31-b339-47f30a5b0588, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 3705
Hier das bereinigte Skript:

```python
#!/usr/bin/env python3
"""Usage: python 26_tree.py [-o tree.yml] file1.yml [file2.yml ...]
Reads component lists (same input as 19_graph.py), merges entries with the same name
and builds a module tree (Name/Children) from the Dependencies as YAML.

Step 1 - packages: a package consists of head modules H (at most K, the public
modules) and their closure, i.e. all modules reachable from H that are used only from
inside the package (hidden modules). Candidates are ranked by the number of hidden
modules and accepted greedily as long as the accepted packages stay laminar (pairwise
disjoint or nested). Packages are nested accordingly; every module is placed in the
smallest package that contains it, otherwise on the top level. For a single head the
module itself is the package node, for several heads a node "Package N" is created.
Step 2 - generations: every top-level unit gets a generation number, 1 + the highest
generation of the units it depends on (units without dependencies are generation 1;
units in a dependency cycle share a generation).
Step 3 - clusters: units of the same generation that share a dependency in a lower
generation are grouped into a node "Cluster <generation>.<n>".
The generations are nested: "Generation n" contains "Generation n-1" as its last
child, the output starts with the highest generation.

Unknown dependencies are ignored (warning on stderr).
On cycles: print all elementary cycles to stderr, then abort.
Output goes to stdout, or to a file with -o/--out."""
import argparse
import sys
import networkx as nx
import yaml
import itertools
import math
import random

SHARED = "shared"
SHARED_MAX_LEVELS = 2  # unused (leftover from the former "shared" rule)


class Dumper(yaml.SafeDumper):
    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)


ap = argparse.ArgumentParser()
ap.add_argument("-o", "--out", help="output file (default: stdout)")
ap.add_argument("files", nargs="+")
args = ap.parse_args()

comps = {}  # name -> merged component (all fields)
for path in args.files:
    with open(path, encoding="utf-8") as f:
        for comp in yaml.safe_load(f) or []:
            cur = comps.setdefault(comp["Name"], {})
            for k, v in comp.items():
                if isinstance(v, list):
                    cur.setdefault(k, [])
                    cur[k] += [x for x in v if x not in cur[k]]
                else:
                    cur[k] = v

if SHARED in comps:
    sys.exit(f"Error: a module named '{SHARED}' already exists")

deps = {n: set(c.get("Dependencies") or []) for n, c in comps.items()}

unknown = sorted({d for ds in deps.values() for d in ds} - deps.keys())
for name in unknown:
    print(f"Warning: unknown dependency ignored: {name}", file=sys.stderr)
deps = {n: ds - set(unknown) for n, ds in deps.items()}

G = nx.DiGraph()  # edge A -> B: A needs B
G.add_nodes_from(deps)
G.add_edges_from((n, d) for n, ds in deps.items() for d in ds)

cycles = list(nx.simple_cycles(G))
if cycles:
    cycles = [c[c.index(min(c)):] + c[:c.index(min(c))] for c in cycles]
    cycles.sort()
    for c in cycles:
        print("Error: cycle: " + " needs ".join(c + [c[0]]), file=sys.stderr)
    sys.exit(f"{len(cycles)} cycles found, no tree possible")

K = 1  # max. number of heads (public modules) per package

desc = {n: nx.descendants(G, n) for n in G}


def closure(H):
    """Largest set S ⊇ H in which all modules except those in H have only dependents in S."""
    S = set(H).union(*(desc[h] for h in H))
    todo = list(S - set(H))
    while todo:
        m = todo.pop()
        if m in S and m not in H and any(p not in S for p in G.predecessors(m)):
            S.discard(m)
            todo += [s for s in G.successors(m) if s in S]
    return S


# Candidates: head sets H (|H| <= K) with their closure S(H)
nonleaf = sorted(n for n in G if desc[n])
cands = []
for r in range(1, K + 1):
    for H in itertools.combinations(nonleaf, r):
        if r > 1 and not all(any(desc[h] & desc[o] for o in H if o != h) for h in H):
            continue  # heads without a common dependency
        S = closure(H)
        if any(G.in_degree(h) and all(p in S for p in G.predecessors(h)) for h in H):
            continue  # a head would be hidden itself, H is not minimal
        cands.append((H, frozenset(S)))

# Canonical order: value (number of hidden modules) descending, then |H|, then names
cands.sort(key=lambda c: (-(len(c[1]) - len(c[0])), len(c[0]), c[0]))
accepted = []
for H, S in cands:
    if all(not S & T or S < T or T < S for _, T in accepted):  # laminar
        accepted.append((H, S))

# Build the tree
mod = {n: {"Name": n, "Children": []} for n in G}
single = {H[0] for H, _ in accepted if len(H) == 1}
pn, k = {}, 0
for i, (H, S) in enumerate(accepted):
    if len(H) == 1:
        pn[i] = mod[H[0]]
    else:
        k += 1
        while f"Package {k}" in G:
            k += 1
        pn[i] = {"Name": f"Package {k}", "Children": []}

top = []
for i, (H, S) in enumerate(accepted):
    sup = [j for j, (_, T) in enumerate(accepted) if S < T]
    tgt = pn[min(sup, key=lambda j: len(accepted[j][1]))]["Children"] if sup else top
    tgt.append(pn[i])
for m in sorted(G):
    if m in single:
        continue
    own = [i for i, (_, S) in enumerate(accepted) if m in S]
    tgt = pn[min(own, key=lambda i: len(accepted[i][1]))]["Children"] if own else top
    tgt.append(mod[m])

# ---------- Second order: generations; third order: exclusive clusters ----------
HUB_USERS = None  # e.g. 8: units with more dependents than this are ignored when merging clusters


def walk(t):
    if t["Name"] in G:
        yield t["Name"]
    for ch in t["Children"]:
        yield from walk(ch)


unit = {m: t["Name"] for t in top for m in walk(t)}
tnode = {t["Name"]: t for t in top}

U = nx.DiGraph()  # unit graph: edge a -> b = a needs b
U.add_nodes_from(tnode)
U.add_edges_from((unit[a], unit[b]) for a, b in G.edges if unit[a] != unit[b])

C = nx.condensation(U)  # units in a cycle share a generation
sg = {}
for s in reversed(list(nx.topological_sort(C))):  # dependencies first
    sg[s] = 1 + max((sg[t] for t in C.successors(s)), default=0)
gen = {u: sg[C.graph["mapping"][u]] for u in U}

# Union-find within a generation: units that share a dependency in a lower generation
uf = {u: u for u in U}


def find(x):
    while uf[x] != x:
        uf[x] = uf[uf[x]]
        x = uf[x]
    return x


for v in U:
    if HUB_USERS is not None and U.in_degree(v) > HUB_USERS:
        continue
    by_gen = {}
    for u in U.predecessors(v):
        if gen[u] > gen[v]:
            by_gen.setdefault(gen[u], []).append(u)
    for us in by_gen.values():
        for u in us[1:]:
            uf[find(u)] = find(us[0])

# (generation, cluster root) -> units (reuses the name "comps", the module dict is no longer needed)
comps = {}
for u in sorted(U):
    comps.setdefault((gen[u], find(u)), []).append(u)


def finish(t):
    ch = t.get("Children", [])
    ch.sort(key=lambda c: (
        c["Name"].startswith("Generation "),  # generation nodes last
        c["Name"]))
    for c in ch:
        finish(c)
    if not ch:
        t.pop("Children", None)
    return t


levels = []
for g in sorted(set(gen.values())):
    kids, k = [], 0
    for (gg, _), us in sorted(comps.items(), key=lambda kv: (kv[0][0], min(kv[1]))):
        if gg != g:
            continue
        if len(us) == 1:
            kids.append(finish(tnode[us[0]]))
        else:
            k += 1
            kids.append({"Name": f"Cluster {g}.{k}", "Children": [finish(tnode[u]) for u in sorted(us)]})
    levels.append({"Name": f"Generation {g}", "Children": kids})

for upper, lower in zip(levels[1:], levels):
    upper["Children"].append(lower)
tree = [finish(levels[-1])]

dump = dict(Dumper=Dumper, allow_unicode=True, sort_keys=False,
            default_flow_style=False, width=float("inf"))
if args.out:
    with open(args.out, "w", encoding="utf-8") as f:
        yaml.dump(tree, f, **dump)
else:
    yaml.dump(tree, sys.stdout, **dump)
```

**Änderungen an Kommentaren und Docstring**

- **Docstring:** Er beschrieb noch die alte Logik (LCA-Platzierung und das Modul „shared"). Der Code arbeitet aber mit Paketen (Köpfe plus Abschluss), Generationen und Clustern. Ich habe ihn entsprechend neu geschrieben. Außerdem hieß das Skript dort noch `20_baum.py`, jetzt `26_tree.py`.
- **`SHARED_MAX_LEVELS`:** Der Kommentar passte nicht mehr, da die Konstante nirgends verwendet wird. Er markiert sie jetzt als ungenutzt.
- **`HUB_USERS`:** Der alte Kommentar sprach von „Vorgängern mit mehr Nutzern". Tatsächlich werden Einheiten `v` übersprungen, die mehr als `HUB_USERS` Nutzer haben. Das ist jetzt korrigiert.
- **Union-Find-Kommentar:** Er sagt jetzt genauer, dass Einheiten derselben Generation zusammengelegt werden, wenn sie eine gemeinsame Abhängigkeit in einer niedrigeren Generation haben.
- **Übrige Kommentare:** Alle übrigen deutschen Kommentare und der Docstring von `closure` sind ins Englische übersetzt. Sie passten inhaltlich.
- **Entfernt:** Die auskommentierten Zeilen `#mod = {...}`, `#c.get("Placement") == "cluster"` und der auskommentierte `kids.append({... "Placement": "cluster" ...})`-Block.
- **Neue Kommentare:** Ich habe zwei ergänzt. Der eine erklärt `# generation nodes last` im Sortierschlüssel. Der andere weist darauf hin, dass `comps` später für etwas anderes wiederverwendet wird.

**Nicht angefasst, aber auffällig**

Ich habe nur Kommentare geändert, den Code nicht. Folgendes könntest du noch entfernen:

- Die Imports `math` und `random` werden nicht verwendet.
- `SHARED_MAX_LEVELS` wird nicht verwendet.
- Der `SHARED`-Check bricht bei einem Modul namens „shared" ab, obwohl kein solcher Knoten mehr erzeugt wird.
- `comps` wird weiter unten mit anderer Bedeutung neu belegt. Eine Umbenennung, etwa in `clusters`, wäre klarer.
