"""#!/usr/bin/env python3"""
'Usage: python 26_tree.py [-o tree.yml] file1.yml [file2.yml ...]\nReads component lists (same input as 19_graph.py), merges entries with the same name\nand builds a module tree (Name/Children) from the Dependencies as YAML.\n\nStep 1 - packages: a package consists of head modules H (at most K, the public\nmodules) and their closure, i.e. all modules reachable from H that are used only from\ninside the package (hidden modules). Candidates are ranked by the number of hidden\nmodules and accepted greedily as long as the accepted packages stay laminar (pairwise\ndisjoint or nested). Packages are nested accordingly; every module is placed in the\nsmallest package that contains it, otherwise on the top level. For a single head the\nmodule itself is the package node, for several heads a node "Package N" is created.\nStep 2 - generations: every top-level unit gets a generation number, 1 + the highest\ngeneration of the units it depends on (units without dependencies are generation 1;\nunits in a dependency cycle share a generation).\nStep 3 - clusters: units of the same generation that share a dependency in a lower\ngeneration are grouped into a cluster node.\nThe generations are nested: a generation node contains the next lower generation as its\nlast child, the output starts with the highest generation.\nGeneration and cluster nodes have a "Path" (directory path relative to the root) instead\nof a "Name". Generation directory: three digits, generation number zero-padded to two\ndigits plus "0" (generation 13 -> "130", generation 5 -> "050"). Cluster directory: running\nlowercase letter per generation (cluster 13.1 -> "a", 27th -> "aa"). Paths are nested,\ne.g. "130/120/a".\n\nUnknown dependencies are ignored (warning on stderr).\nOn cycles: print all elementary cycles to stderr, then abort.\nOutput goes to stdout, or to a file with -o/--out.'
import argparse
import sys
import networkx as nx
import yaml
import itertools

class Dumper(yaml.SafeDumper):

    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)
ap = argparse.ArgumentParser()
ap.add_argument('-o', '--out', help='output file (default: stdout)')
ap.add_argument('files', nargs='+')
args = ap.parse_args()
'# name -> merged component (all fields)'
comps = {}
for path in args.files:
    with open(path, encoding='utf-8') as f:
        for comp in yaml.safe_load(f) or []:
            cur = comps.setdefault(comp['Name'], {})
            for k, v in comp.items():
                if isinstance(v, list):
                    cur.setdefault(k, [])
                    cur[k] += [x for x in v if x not in cur[k]]
                else:
                    cur[k] = v
deps = {n: set(c.get('Dependencies') or []) for n, c in comps.items()}
unknown = sorted({d for ds in deps.values() for d in ds} - deps.keys())
for name in unknown:
    print(f'Warning: unknown dependency ignored: {name}', file=sys.stderr)
deps = {n: ds - set(unknown) for n, ds in deps.items()}
'# edge A -> B: A needs B'
G = nx.DiGraph()
G.add_nodes_from(deps)
G.add_edges_from(((n, d) for n, ds in deps.items() for d in ds))
cycles = list(nx.simple_cycles(G))
if cycles:
    cycles = [c[c.index(min(c)):] + c[:c.index(min(c))] for c in cycles]
    cycles.sort()
    for c in cycles:
        print('Error: cycle: ' + ' needs '.join(c + [c[0]]), file=sys.stderr)
    sys.exit(f'{len(cycles)} cycles found, no tree possible')
'# max. number of heads (public modules) per package'
K = 1
desc = {n: nx.descendants(G, n) for n in G}

def closure(H):
    """Largest set S ⊇ H in which all modules except those in H have only dependents in S."""
    S = set(H).union(*(desc[h] for h in H))
    todo = list(S - set(H))
    while todo:
        m = todo.pop()
        if m in S and m not in H and any((p not in S for p in G.predecessors(m))):
            S.discard(m)
            todo += [s for s in G.successors(m) if s in S]
    return S
'# Candidates: head sets H (|H| <= K) with their closure S(H)'
nonleaf = sorted((n for n in G if desc[n]))
cands = []
for r in range(1, K + 1):
    for H in itertools.combinations(nonleaf, r):
        if r > 1 and (not all((any((desc[h] & desc[o] for o in H if o != h)) for h in H))):
            '# heads without a common dependency'
            continue
        S = closure(H)
        if any((G.in_degree(h) and all((p in S for p in G.predecessors(h))) for h in H)):
            '# a head would be hidden itself, H is not minimal'
            continue
        cands.append((H, frozenset(S)))
'# Canonical order: value (number of hidden modules) descending, then |H|, then names'
cands.sort(key=lambda c: (-(len(c[1]) - len(c[0])), len(c[0]), c[0]))
accepted = []
for H, S in cands:
    '# laminar'
    if all((not S & T or S < T or T < S for _, T in accepted)):
        accepted.append((H, S))
'# Build the tree'
mod = {n: {'Name': n, 'Children': []} for n in G}
single = {H[0] for H, _ in accepted if len(H) == 1}
pn, k = ({}, 0)
for i, (H, S) in enumerate(accepted):
    if len(H) == 1:
        pn[i] = mod[H[0]]
    else:
        k += 1
        while f'Package {k}' in G:
            k += 1
        pn[i] = {'Name': f'Package {k}', 'Children': []}
top = []
for i, (H, S) in enumerate(accepted):
    sup = [j for j, (_, T) in enumerate(accepted) if S < T]
    tgt = pn[min(sup, key=lambda j: len(accepted[j][1]))]['Children'] if sup else top
    tgt.append(pn[i])
for m in sorted(G):
    if m in single:
        continue
    own = [i for i, (_, S) in enumerate(accepted) if m in S]
    tgt = pn[min(own, key=lambda i: len(accepted[i][1]))]['Children'] if own else top
    tgt.append(mod[m])
'# ---------- Second order: generations; third order: exclusive clusters ----------'
'# e.g. 8: units with more dependents than this are ignored when merging clusters'
HUB_USERS = None

def walk(t):
    if t['Name'] in G:
        yield t['Name']
    for ch in t['Children']:
        yield from walk(ch)
unit = {m: t['Name'] for t in top for m in walk(t)}
tnode = {t['Name']: t for t in top}
'# unit graph: edge a -> b = a needs b'
U = nx.DiGraph()
U.add_nodes_from(tnode)
U.add_edges_from(((unit[a], unit[b]) for a, b in G.edges if unit[a] != unit[b]))
'# units in a cycle share a generation'
C = nx.condensation(U)
sg = {}
'# dependencies first'
for s in reversed(list(nx.topological_sort(C))):
    sg[s] = 1 + max((sg[t] for t in C.successors(s)), default=0)
gen = {u: sg[C.graph['mapping'][u]] for u in U}
'# Union-find within a generation: units that share a dependency in a lower generation'
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
'# (generation, cluster root) -> units (reuses the name "comps", the module dict is no longer needed)'
comps = {}
for u in sorted(U):
    comps.setdefault((gen[u], find(u)), []).append(u)

def finish(t):
    ch = t.get('Children', [])
    ch.sort(key=lambda c: ('Path' in c and c['Path'].rsplit('/', 1)[-1].isdigit(), c.get('Name', c.get('Path'))))
    for c in ch:
        finish(c)
    if not ch:
        t.pop('Children', None)
    return t

def letters(n):
    s = ''
    while n:
        n, r = divmod(n - 1, 26)
        s = chr(97 + r) + s
    return s
gpath, prefix = ({}, [])
for g in sorted(set(gen.values()), reverse=True):
    prefix.append(f'{g:02d}0')
    gpath[g] = '/'.join(prefix)
levels = []
for g in sorted(set(gen.values())):
    kids, k = ([], 0)
    for (gg, _), us in sorted(comps.items(), key=lambda kv: (kv[0][0], min(kv[1]))):
        if gg != g:
            continue
        if len(us) == 1:
            kids.append(finish(tnode[us[0]]))
        else:
            k += 1
            kids.append({'Path': f'{gpath[g]}/{letters(k)}', 'Children': [finish(tnode[u]) for u in sorted(us)]})
    levels.append({'Path': gpath[g], 'Children': kids})
for upper, lower in zip(levels[1:], levels):
    upper['Children'].append(lower)
tree = [finish(levels[-1])]
dump = dict(Dumper=Dumper, allow_unicode=True, sort_keys=False, default_flow_style=False, width=float('inf'))
if args.out:
    with open(args.out, 'w', encoding='utf-8') as f:
        yaml.dump(tree, f, **dump)
else:
    yaml.dump(tree, sys.stdout, **dump)