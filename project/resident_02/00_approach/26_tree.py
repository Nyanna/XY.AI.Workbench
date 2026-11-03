#!/usr/bin/env python3
"""Usage: python 20_baum.py [-o tree.yml] file1.yml [file2.yml ...]
Reads component lists (same input as 19_graph.py), merges entries with the same name
and builds a module tree (Name/Children) from the Dependencies as YAML.

Rule: every module appears exactly once in the tree. A module is placed as a child of
the deepest node that is visible to all of its dependents, i.e. the lowest common
ancestor (dependents themselves included) of all modules that need it. With a single
dependent it sits directly below it; with several it is pulled up only as far as
necessary. If its dependents share no ancestor (separate subtrees), the module goes to
the top level. Modules nobody needs are roots.
Shared: root modules whose own tree has at most 2 levels (the module plus children,
no grandchildren) are moved into a top-level module "shared" (basic types, base
libraries, project-wide tools). Their subtrees move along unchanged.
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
SHARED_MAX_LEVELS = 2  # max tree height of a root module to count as shared


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

K = 1  # max. Köpfe (öffentliche Module) pro Paket

desc = {n: nx.descendants(G, n) for n in G}


def closure(H):
    """Größte Menge S ⊇ H, in der alle Module außer H nur Nutzer in S haben."""
    S = set(H).union(*(desc[h] for h in H))
    todo = list(S - set(H))
    while todo:
        m = todo.pop()
        if m in S and m not in H and any(p not in S for p in G.predecessors(m)):
            S.discard(m)
            todo += [s for s in G.successors(m) if s in S]
    return S


# Kandidaten: Kopfmengen H (|H| ≤ K) mit ihrem Abschluss S(H)
nonleaf = sorted(n for n in G if desc[n])
cands = []
for r in range(1, K + 1):
    for H in itertools.combinations(nonleaf, r):
        if r > 1 and not all(any(desc[h] & desc[o] for o in H if o != h) for h in H):
            continue  # Köpfe ohne gemeinsame Abhängigkeit
        S = closure(H)
        if any(G.in_degree(h) and all(p in S for p in G.predecessors(h)) for h in H):
            continue  # Kopf wäre selbst versteckt, H nicht minimal
        cands.append((H, frozenset(S)))

# Kanonische Auswahl: nur Wert (versteckte Module), dann |H|, dann Namen.
cands.sort(key=lambda c: (-(len(c[1]) - len(c[0])), len(c[0]), c[0]))
accepted = []
for H, S in cands:
    if all(not S & T or S < T or T < S for _, T in accepted):  # laminar
        accepted.append((H, S))

# Baum aufbauen
#mod = {n: {"Name": n, "Dependencies": sorted(deps[n]), "Children": []} for n in G}
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

# ---------- Zweite Ordnung: Generationen, dritte: exklusive Cluster ----------
HUB_USERS = None  # z. B. 8: Vorgänger mit mehr Nutzern zählen nicht für die Überschneidung


def walk(t):
    if t["Name"] in G:
        yield t["Name"]
    for ch in t["Children"]:
        yield from walk(ch)


unit = {m: t["Name"] for t in top for m in walk(t)}
tnode = {t["Name"]: t for t in top}

U = nx.DiGraph()  # Einheitengraph: Kante a -> b = a braucht b
U.add_nodes_from(tnode)
U.add_edges_from((unit[a], unit[b]) for a, b in G.edges if unit[a] != unit[b])

C = nx.condensation(U)  # Zyklen zwischen Einheiten teilen sich eine Generation
sg = {}
for s in reversed(list(nx.topological_sort(C))):  # Abhängigkeiten zuerst
    sg[s] = 1 + max((sg[t] for t in C.successors(s)), default=0)
gen = {u: sg[C.graph["mapping"][u]] for u in U}

# Union-Find innerhalb einer Generation: gemeinsame Abhängigkeit zu früheren Generationen
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

comps = {}
for u in sorted(U):
    comps.setdefault((gen[u], find(u)), []).append(u)


def finish(t):
    ch = t.get("Children", [])
    ch.sort(key=lambda c: (
        c["Name"].startswith("Generation "),
        #c.get("Placement") == "cluster",
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
            #kids.append({"Name": f"Cluster {g}.{k}", "Placement": "cluster",
            #             "Children": [finish(tnode[u]) for u in sorted(us)]})
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
