"""Package assignment (see 05_naming_and_packaging.md, 'Packaging' section).

- '.components': named schemas referenced by more than one path (or by none --
  no single path to host them in, so they default to the shared location too).
- '.request.<path>.<method>.<ct>': the request root of one operation.
- '.response.<path>[.<method>].code<code>.<ct>': one status-code/content-type
  view of one operation; the method segment is dropped when the path has
  exactly one method, kept otherwise to stay unambiguous.
- '.enums' / '.lists' / '.objects' / '.operators' / '.dictionaries': anonymous
  nodes, bucketed by kind -- always, regardless of how many sites reference
  them (their class name already carries the disambiguating context prefix).
- A named schema used by exactly one path lives in that path's request/
  response package instead of '.components'.
"""

from collections import defaultdict
from dataclasses import dataclass

from cgen.model.nodes import (
    AnyDictionaryNode,
    CompositionNode,
    DictionaryNode,
    EnumNode,
    ListNode,
    ObjectNode,
    RefNode,
)
from cgen.naming.identifiers import content_type_short_name
from cgen.naming.paths import path_to_package_segments
from cgen.naming.traverse import iter_child_edges

ANONYMOUS_KIND_PACKAGE = {
    EnumNode: "enums",
    ListNode: "lists",
    ObjectNode: "objects",
    CompositionNode: "operators",
    DictionaryNode: "dictionaries",
    AnyDictionaryNode: "dictionaries",
}


@dataclass(frozen=True)
class Site:
    """Where a named schema was first encountered while walking an operation."""

    path: str
    side: str  # 'request' | 'response'
    method: str
    code: str | None
    content_type: str


def anonymous_package(node, base_package: str) -> str | None:
    """Kind-based package for an anonymous node, or None if it has no class."""
    bucket = ANONYMOUS_KIND_PACKAGE.get(type(node))
    return f"{base_package}.{bucket}" if bucket else None


def site_package(site: Site, base_package: str, methods_by_path: dict) -> str:
    """The request/response package a Site maps to."""
    path_segment = ".".join(path_to_package_segments(site.path))
    if site.side == "request":
        return f"{base_package}.request.{path_segment}.{site.method.lower()}.{site.content_type}"
    method_segment = f".{site.method.lower()}" if len(methods_by_path.get(site.path, ())) > 1 else ""
    return f"{base_package}.response.{path_segment}{method_segment}.code{site.code}.{site.content_type}"


def response_root_package(path: str, method: str, base_package: str, methods_by_path: dict) -> str:
    """Package of a ResponseNode root (no code/content-type segment)."""
    path_segment = ".".join(path_to_package_segments(path))
    method_segment = f".{method.lower()}" if len(methods_by_path.get(path, ())) > 1 else ""
    return f"{base_package}.response.{path_segment}{method_segment}"


def collect_named_references(identified_model):
    """Walk every operation's request/response tree to find, per named schema,
    which paths reference it (directly or transitively via other named
    schemas) and where it was first encountered (site).

    Returns (paths_by_name: dict[str, set[str]], first_site_by_name: dict[str, Site],
    methods_by_path: dict[str, set[str]]).
    """
    paths_by_name: dict = defaultdict(set)
    first_site: dict = {}
    methods_by_path: dict = defaultdict(set)
    for operation_model in identified_model.operations:
        methods_by_path[operation_model.operation.path].add(operation_model.operation.method)

    for operation_model in identified_model.operations:
        operation = operation_model.operation
        if operation_model.request is not None and operation_model.request.body is not None:
            site = Site(path=operation.path, side="request", method=operation.method, code=None, content_type="json")
            _walk(
                operation_model.request.body.target,
                operation.path,
                site,
                paths_by_name,
                first_site,
                identified_model.named_nodes,
                frozenset(),
            )
        for code_node in operation_model.response.codes:
            for content_type_view in code_node.content_types:
                site = Site(
                    path=operation.path,
                    side="response",
                    method=operation.method,
                    code=code_node.status_code,
                    content_type=content_type_short_name(content_type_view.content_type),
                )
                _walk(
                    content_type_view.body.target,
                    operation.path,
                    site,
                    paths_by_name,
                    first_site,
                    identified_model.named_nodes,
                    frozenset(),
                )
    return paths_by_name, first_site, methods_by_path


def _walk(node, path, site, paths_by_name, first_site, named_nodes, visited):
    if isinstance(node, RefNode):
        name = node.name
        paths_by_name[name].add(path)
        if name not in first_site:
            first_site[name] = site
        if name in visited:
            return  # cycle guard: a named schema's own subtree is walked once per chain
        _walk(named_nodes[name], path, site, paths_by_name, first_site, named_nodes, visited | {name})
        return
    for _, edge in iter_child_edges(node):
        _walk(edge.target, path, site, paths_by_name, first_site, named_nodes, visited)


def named_package(name: str, base_package: str, paths_by_name: dict, first_site: dict, methods_by_path: dict) -> str:
    """Package for a named schema: its single referencing path's site, or '.components'."""
    paths = paths_by_name.get(name) or set()
    if len(paths) == 1:
        return site_package(first_site[name], base_package, methods_by_path)
    return f"{base_package}.components"
