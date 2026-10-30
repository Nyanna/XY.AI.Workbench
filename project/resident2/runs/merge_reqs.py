import sys
import yaml

TREE_FILE = "/home/user/xyan/xy.ai.workbench/project/resident2/runs/modules_optimized.yml"
REQUIREMENTS_FILE = "/home/user/xyan/xy.ai.workbench/project/resident2/runs/all_reqs.yml"
OUTPUT_FILE = "/home/user/xyan/xy.ai.workbench/project/resident2/runs/all_reqs_merged.yml"


def load_yaml(path):
    with open(path, encoding="utf-8") as f:
        return yaml.safe_load(f) or []


def collect_requirements(entries):
    """Name -> Liste der Requirements (mehrfache Einträge gemerged, Duplikate entfernt)."""
    merged = {}
    for entry in entries:
        bucket = merged.setdefault(entry["Name"], [])
        for req in entry.get("Requirements") or []:
            if req not in bucket:
                bucket.append(req)
    return merged


def merge_node(node, reqs, used):
    result = {}
    for key, value in node.items():
        if key == "Children":
            continue
        result[key] = value

    name = node.get("Name")
    if name in reqs:
        used.add(name)
        result["Requirements"] = list(reqs[name])

    if "Children" in node:
        result["Children"] = [merge_node(c, reqs, used) for c in node["Children"] or []]
    return result


def main():
    tree = load_yaml(TREE_FILE)
    reqs = collect_requirements(load_yaml(REQUIREMENTS_FILE))
    used = set()

    merged = [merge_node(n, reqs, used) for n in tree]

    for name in sorted(set(reqs) - used):
        print(f"Warnung: Knoten '{name}' nicht im Baum gefunden", file=sys.stderr)

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        yaml.dump(merged, f, allow_unicode=True, sort_keys=False, default_flow_style=False, width=1000)


if __name__ == "__main__":
    main()