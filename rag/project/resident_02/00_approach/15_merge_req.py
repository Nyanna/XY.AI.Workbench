import yaml

INPUT_FILE = "/home/user/xyan/xy.ai.workbench/project/resident2/runs/all_reqs.yml"
OUTPUT_FILE = "/home/user/xyan/xy.ai.workbench/project/resident2/runs/all_reqs_merged.yml"


def merge_nodes(nodes):
    merged = {}
    for node in nodes:
        name = node["Name"]
        reqs = merged.setdefault(name, [])
        for req in node.get("Requirements") or []:
            if req not in reqs:  # Duplikate vermeiden, Reihenfolge beibehalten
                reqs.append(req)
    return [{"Name": name, "Requirements": reqs} for name, reqs in merged.items()]


def main():
    with open(INPUT_FILE, encoding="utf-8") as f:
        nodes = yaml.safe_load(f) or []

    result = merge_nodes(nodes)

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        yaml.safe_dump(result, f, allow_unicode=True, sort_keys=False, width=1000)


if __name__ == "__main__":
    main()