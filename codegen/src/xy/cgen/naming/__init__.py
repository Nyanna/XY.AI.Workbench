"""Assigns Java class names and package paths to identified IR nodes.

Three name sources feed one collision-resolved result:
- named schemas (key -> class name),
- anonymous shared-type nodes (kind-based packages),
- per-operation transport roots (RequestNode/ResponseNode/CodeNode/ContentTypeView).

Collisions (same package + same class name from different structures) are
resolved by canonical-fingerprint order, never discovery order.
"""

from dataclasses import dataclass, field

from xy.cgen.model.nodes import AnyDictionaryNode, DictionaryNode, EnumNode, ListNode, RefNode
from xy.cgen.naming.identifiers import content_type_short_name
from xy.cgen.naming.names import derive_class_names
from xy.cgen.naming.packages import (
    Site,
    anonymous_package,
    collect_named_references,
    named_package,
    response_root_package,
    site_package,
)


@dataclass(frozen=True)
class NodeName:
    """A resolved class name plus its package."""

    package: str
    class_name: str

    @property
    def fqn(self) -> str:
        return f"{self.package}.{self.class_name}"


@dataclass(frozen=True)
class NamedModel:
    """The identified IR plus a name/package for every node that becomes a class."""

    named_nodes: dict
    operations: tuple
    fingerprints: dict
    base_package: str
    names: dict  # id(node) -> NodeName

    def name_of(self, node) -> NodeName | None:
        return self.names.get(id(node))

    def name_of_ref(self, ref_name: str) -> NodeName | None:
        node = self.named_nodes.get(ref_name)
        return self.names.get(id(node)) if node is not None else None


@dataclass
class _Entry:
    """One candidate (node, package, class_name) before collision resolution."""

    node: object
    package: str
    class_name: str
    sort_key: str
    final_name: str = field(default="")


def assign_names(identified_model, base_package: str) -> NamedModel:
    """Derive class/package names for every named, anonymous, and transport node."""
    class_names = derive_class_names(identified_model)
    paths_by_name, first_site, methods_by_path = collect_named_references(identified_model)

    entries: list[_Entry] = []
    entries.extend(_named_entries(identified_model, class_names, base_package, paths_by_name, first_site, methods_by_path))
    entries.extend(_anonymous_entries(identified_model, class_names, base_package))
    entries.extend(_transport_entries(identified_model, class_names, base_package, methods_by_path))

    _resolve_collisions(entries)

    names = {id(entry.node): NodeName(package=entry.package, class_name=entry.final_name) for entry in entries}
    _mirror_content_type_view_names(identified_model, names)
    _mirror_ref_request_names(identified_model, names)
    return NamedModel(
        named_nodes=identified_model.named_nodes,
        operations=identified_model.operations,
        fingerprints=identified_model.fingerprints,
        base_package=base_package,
        names=names,
    )


_KIND_BUCKETED_NAMED_TYPES = (ListNode, EnumNode, DictionaryNode, AnyDictionaryNode)


def _named_entries(identified_model, class_names, base_package, paths_by_name, first_site, methods_by_path):
    for key, node in identified_model.named_nodes.items():
        # List/Enum/Dictionary schemas are containers whose identity is purely
        # structural (like their anonymous siblings): kind bucket, regardless of
        # how many paths reference them. Object/Composition schemas keep the
        # request/response-vs-.components rule.
        if isinstance(node, _KIND_BUCKETED_NAMED_TYPES):
            package = anonymous_package(node, base_package)
        else:
            package = named_package(key, base_package, paths_by_name, first_site, methods_by_path)
        sort_key = identified_model.fingerprint_of(node) or key
        yield _Entry(node=node, package=package, class_name=class_names.named[key], sort_key=sort_key)


def _anonymous_entries(identified_model, class_names, base_package):
    for node_id, node in class_names.anonymous_nodes.items():
        package = anonymous_package(node, base_package)
        if package is None:  # e.g. UnsupportedNode: never gets a class
            continue
        sort_key = identified_model.fingerprint_of(node) or str(node_id)
        yield _Entry(node=node, package=package, class_name=class_names.anonymous[node_id], sort_key=sort_key)


def _transport_entries(identified_model, class_names, base_package, methods_by_path):
    for operation_model in identified_model.operations:
        operation = operation_model.operation
        request_node = operation_model.request
        # a $ref body IS the request root -- the named schema's own entry already
        # covers it (see _mirror_ref_request_names); only inline/anonymous bodies get
        # their own synthetic-name entry here.
        if (
            request_node is not None
            and request_node.body is not None
            and not isinstance(request_node.body.target, RefNode)
        ):
            site = Site(path=operation.path, side="request", method=operation.method, code=None, content_type="json")
            yield _Entry(
                node=request_node,
                package=site_package(site, base_package, methods_by_path),
                class_name=class_names.transport[id(request_node)],
                sort_key=f"request:{operation.path}:{operation.method}",
            )

        response_node = operation_model.response
        yield _Entry(
            node=response_node,
            package=response_root_package(operation.path, operation.method, base_package, methods_by_path),
            class_name=class_names.transport[id(response_node)],
            sort_key=f"response:{operation.path}:{operation.method}",
        )
        for code_node in response_node.codes:
            for content_type_view in code_node.content_types:
                site = Site(
                    path=operation.path,
                    side="response",
                    method=operation.method,
                    code=code_node.status_code,
                    content_type=content_type_short_name(content_type_view.content_type),
                )
                package = site_package(site, base_package, methods_by_path)
                sort_suffix = f"{operation.path}:{operation.method}:{code_node.status_code}:{content_type_view.content_type}"
                # CodeNode/ContentTypeView are one synthetic class (doc bullet 18): only the
                # CodeNode competes for collision numbering; ContentTypeView mirrors its name below.
                yield _Entry(
                    node=code_node,
                    package=package,
                    class_name=class_names.transport[id(code_node)],
                    sort_key=f"code:{sort_suffix}",
                )


def _mirror_content_type_view_names(identified_model, names: dict) -> None:
    """ContentTypeView shares its CodeNode sibling's resolved name (one synthetic class)."""
    for operation_model in identified_model.operations:
        for code_node in operation_model.response.codes:
            code_name = names.get(id(code_node))
            if code_name is None:
                continue
            for content_type_view in code_node.content_types:
                names[id(content_type_view)] = code_name


def _mirror_ref_request_names(identified_model, names: dict) -> None:
    """A $ref request body IS the request root: mirror the named schema's entry."""
    for operation_model in identified_model.operations:
        request_node = operation_model.request
        if request_node is None or request_node.body is None:
            continue
        target = request_node.body.target
        if isinstance(target, RefNode):
            resolved = names.get(id(identified_model.named_nodes[target.name]))
            if resolved is not None:
                names[id(request_node)] = resolved


def _resolve_collisions(entries: list) -> None:
    """Same (package, class_name) from different structures -> Name, Name2, Name3."""
    buckets: dict = {}
    for entry in entries:
        buckets.setdefault((entry.package, entry.class_name), []).append(entry)
    for group in buckets.values():
        group.sort(key=lambda entry: entry.sort_key)
        for index, entry in enumerate(group, 1):
            entry.final_name = entry.class_name if index == 1 else f"{entry.class_name}{index}"
