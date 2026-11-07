#!/usr/bin/env python3
"""Usage: python waves.py [-b baum.yml] datei1.yml [datei2.yml ...]
Liest Listen von Komponenten, vereinigt Einträge gleichen Namens und gibt jedes Modul
mit seiner Ordnungsnummer aus.
Unbekannte Dependencies gelten als vorhanden (Nummer 0) und erzeugen eine Warnung.
Bei Zyklen: alle elementaren Zyklen auf stderr, alle beteiligten Module mit allen
Feldern nach zyklen.yml, dann Abbruch.
Mit -b/--baum (Baumdatei mit Name/Children) wird innerhalb jeder Ebene nach Baumnähe
sortiert (Postorder: Geschwister und Teilbäume bleiben zusammen, Kinder vor Eltern)
und die Nummer hat die Form "Ebene.Position". Module ohne Baumeintrag kommen am
Ebenenende, alphabetisch."""
import argparse
import sys
from graphlib import TopologicalSorter
import networkx as nx
import yaml

OUT = "zyklen.yml"


class Dumper(yaml.SafeDumper):
    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)


def str_repr(dumper, s):
    style = "|" if "\n" in s else None
    return dumper.represent_scalar("tag:yaml.org,2002:str", s, style=style)


Dumper.add_representer(str, str_repr)


def postorder_index(tree):
    """Name -> Postorder-Position (Kinder vor Eltern, Geschwister in Dateireihenfolge)."""
    idx = {}

    def visit(node):
        for child in node.get("Children") or []:
            visit(child)
        idx.setdefault(node["Name"], len(idx))

    for root in tree or []:
        visit(root)
    return idx


ap = argparse.ArgumentParser()
ap.add_argument("-b", "--baum", help="optionale Baumdatei zur Subsortierung")
ap.add_argument("dateien", nargs="+")
args = ap.parse_args()

order = None
if args.baum:
    with open(args.baum, encoding="utf-8") as f:
        order = postorder_index(yaml.safe_load(f))


def sort_key(name):
    if order is None:
        return (0, name)
    return (0, order[name], name) if name in order else (1, len(order), name)


comps = {}  # Name -> zusammengeführte Komponente (alle Felder)
for path in args.dateien:
    with open(path, encoding="utf-8") as f:
        for comp in yaml.safe_load(f) or []:
            cur = comps.setdefault(comp["Name"], {})
            for k, v in comp.items():
                if isinstance(v, list):
                    cur.setdefault(k, [])
                    cur[k] += [x for x in v if x not in cur[k]]
                else:
                    cur[k] = v

deps = {n: set(c.get("Dependencies") or []) for n, c in comps.items()}

unknown = sorted({d for ds in deps.values() for d in ds} - deps.keys())
for name in unknown:
    print(f"0 {name}")

known = {n: {d for d in ds if d not in unknown} for n, ds in deps.items()}

G = nx.DiGraph()  # Kante A -> B: A braucht B
G.add_nodes_from(known)
G.add_edges_from((n, d) for n, ds in known.items() for d in ds)

cycles = list(nx.simple_cycles(G))
if cycles:
    cycles = [c[c.index(min(c)):] + c[:c.index(min(c))] for c in cycles]
    cycles.sort()
    for c in cycles:
        print("Fehler: Zyklus: " + " braucht ".join(c + [c[0]]), file=sys.stderr)
    involved = sorted({n for c in cycles for n in c})
    with open(OUT, "w", encoding="utf-8") as f:
        yaml.dump([comps[n] for n in involved], f, Dumper=Dumper, allow_unicode=True,
                  sort_keys=False, default_flow_style=False, width=float("inf"))
    sys.exit(f"{len(cycles)} Zyklen gefunden, {len(involved)} Module nach {OUT} geschrieben")

ts = TopologicalSorter(known)
ts.prepare()
level = 1
while ts.is_active():
    ready = sorted(ts.get_ready(), key=sort_key)
    for pos, name in enumerate(ready, 1):
        print(f"{level}.{pos} {name}" if order is not None else f"{level} {name}")
    ts.done(*ready)
    level += 1