#!/usr/bin/env python3
import re
import sys
import yaml

TREE_FILE = "/home/user/xyan/xy.ai.workbench/project/resident_02/13_modules_optimized.yml"
MODULES_FILE = "/home/user/xyan/xy.ai.workbench/project/resident_02/18_modules_deps.yml"
ORDER_FILE = "/home/user/xyan/xy.ai.workbench/project/resident_02/20_plan_order.md"
OUTPUT_FILE = "/home/user/xyan/xy.ai.workbench/project/resident_02/21_modules_ordered.yml"


def norm(s):
    return re.sub(r"\s+", " ", str(s)).strip().casefold()


def warn(msg):
    print(f"WARN: {msg}", file=sys.stderr)


def load_yaml(path):
    with open(path, encoding="utf-8") as f:
        return yaml.safe_load(f) or []


def flatten_tree(nodes, out=None):
    out = {} if out is None else out
    for n in nodes or []:
        out[norm(n["Name"])] = n.get("Responsibility") or []
        flatten_tree(n.get("Children"), out)
    return out


def load_order(path):
    order = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            m = re.match(r"^\s*\d+(?:\.\d+)*\.?\s+(.+?)\s*$", line)
            if m:
                order.setdefault(norm(m.group(1)), len(order) + 1)
    return order


def main():
    resp = flatten_tree(load_yaml(TREE_FILE))
    modules = load_yaml(MODULES_FILE)
    order = load_order(ORDER_FILE)

    names = {norm(m["Name"]) for m in modules}
    for k in order:
        if k not in names:
            warn(f"Order-Eintrag ohne Modul: {k}")

    result = []
    for m in modules:
        k = norm(m["Name"])
        if k not in order:
            warn(f"Modul ohne Order: {m['Name']}")
        if k not in resp:
            warn(f"Modul ohne Baumknoten: {m['Name']}")
        for d in m.get("Dependencies") or []:
            if norm(d) not in names:
                warn(f"{m['Name']}: unbekannte Dependency {d}")
            elif order.get(norm(d), 0) >= order.get(k, 0):
                warn(f"{m['Name']}: Dependency {d} nicht früher in Order")
        result.append({
            "Name": m["Name"],
            "Order": order.get(k),
            "Responsibilities": resp.get(k, []),
            "Requirements": m.get("Requirements") or [],
            "Dependencies": m.get("Dependencies") or [],
        })

    result.sort(key=lambda r: (r["Order"] is None, r["Order"] or 0))

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        yaml.safe_dump(result, f, default_flow_style=False, sort_keys=False,
                       allow_unicode=True, width=10**6)


if __name__ == "__main__":
    main()