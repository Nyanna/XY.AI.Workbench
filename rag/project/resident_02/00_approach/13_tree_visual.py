#!/usr/bin/env python3
"""Visualisiert die hierarchische Struktur einer YAML-Datei als Baum."""
import sys
import yaml

try:
    from rich.tree import Tree
    from rich.console import Console
    HAS_RICH = True
except ImportError:
    HAS_RICH = False


def label(node: dict) -> str:
    return str(node.get("Name", "<ohne Name>"))


def counts(node: dict) -> str:
    r = len(node.get("Responsibility", []) or [])
    q = len(node.get("Requirements", []) or [])
    return f"[R:{r} | Req:{q}]"


def children(node: dict) -> list:
    return node.get("Children", []) or []


def print_plain(nodes, prefix="", details=False):
    for i, node in enumerate(nodes):
        last = i == len(nodes) - 1
        branch = "└── " if last else "├── "
        print(f"{prefix}{branch}{label(node)}")
        ext = "    " if last else "│   "
        if details:
            for key in ("Responsibility", "Requirements"):
                for item in node.get(key, []) or []:
                    short = item if len(item) <= 70 else item[:67] + "..."
                    print(f"{prefix}{ext}  · {key[:3]}: {short}")
        print_plain(children(node), prefix + ext, details)


def build_rich(nodes, parent, details=False):
    for node in nodes:
        branch = parent.add(f"[bold]{label(node)}[/bold]")
        if details:
            for key in ("Responsibility", "Requirements"):
                for item in node.get(key, []) or []:
                    short = item if len(item) <= 70 else item[:67] + "..."
                    branch.add(f"[dim]{key[:3]}:[/dim] {short}")
        build_rich(children(node), branch, details)


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("-")]
    details = "-d" in sys.argv or "--details" in sys.argv
    if not args:
        sys.exit("Nutzung: python yaml_tree.py <datei.yaml> [-d|--details]")

    with open(args[0], encoding="utf-8") as f:
        data = yaml.safe_load(f)

    if isinstance(data, dict):
        data = [data]

    if HAS_RICH:
        root = Tree("📄 " + args[0])
        build_rich(data, root, details)
        Console().print(root)
    else:
        print(args[0])
        print_plain(data, details=details)


if __name__ == "__main__":
    main()