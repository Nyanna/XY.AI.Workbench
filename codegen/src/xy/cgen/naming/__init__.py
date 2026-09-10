"""Assigns Java class names and package paths to identified IR nodes.

Three name sources feed one collision-resolved result:
- named schemas (key -> class name),
- anonymous shared-type nodes (kind-based packages),
- per-operation transport roots (RequestNode/ResponseNode/CodeNode/ContentTypeView).

Collisions (same package + same class name from different structures) are
resolved by canonical-fingerprint order, never discovery order.
"""
from dataclasses import dataclass, field
from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, RefNode
from xy.cgen.naming.identifiers import class_identifier, content_type_short_name
from xy.cgen.naming.names import derive_class_names
from xy.cgen.naming.packages import Site, anonymous_package, collect_named_references, named_package, response_root_package, site_package

@dataclass(frozen=True)
class NodeName:
    """A resolved class name plus its package."""
    package: str
    class_name: str

    @property
    def fqn(self) -> str:
        return f'{self.package}.{self.class_name}'

@dataclass(frozen=True)
class NamedModel:
    """The identified IR plus a name/package for every node that becomes a class."""
    named_nodes: dict
    operations: tuple
    fingerprints: dict
    base_package: str
    '# id(node) -> NodeName'
    names: dict

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
    final_name: str = field(default='')

def assign_names(identified_model, base_package: str) -> NamedModel:
    """Derive class/package names for every named, anonymous, and transport node."""
    class_names = derive_class_names(identified_model)
    paths_by_name, first_site, methods_by_path = collect_named_references(identified_model)
    entries: list[_Entry] = []
    entries.extend(
        _named_entries(
            identified_model,
            class_names,
            base_package,
            paths_by_name,
            first_site,
            methods_by_path))
    edge_index = getattr(identified_model, 'edge_index', None)
    entries.extend(_anonymous_entries(identified_model, class_names, base_package))
    entries.extend(_transport_entries(identified_model, class_names, base_package, methods_by_path))
    named_ids = {id(node) for node in identified_model.named_nodes.values()}
    default_package = {id(entry.node): entry.package for entry in entries}
    _apply_private_packages(entries, edge_index, default_package, named_ids)
    _apply_enum_edge_names(entries, edge_index)
    _resolve_collisions(entries)
    names = {id(entry.node): NodeName(package=entry.package, class_name=entry.final_name) for entry in entries}
    _mirror_content_type_view_names(identified_model, names)
    _mirror_ref_request_names(identified_model, names)
    return NamedModel(named_nodes=identified_model.named_nodes, operations=identified_model.operations,
                      fingerprints=identified_model.fingerprints, base_package=base_package, names=names)
_KIND_BUCKETED_NAMED_TYPES = (ListNode, EnumNode, DictionaryNode, AnyDictionaryNode)

def _named_entries(identified_model, class_names, base_package, paths_by_name, first_site, methods_by_path):
    for key, node in identified_model.named_nodes.items():
        '# List/Enum/Dictionary schemas are containers whose identity is purely'
        '# structural (like their anonymous siblings): kind bucket, regardless of'
        '# how many paths reference them. Object/Composition schemas keep the'
        '# request/response-vs-.components rule.'
        if isinstance(node, _KIND_BUCKETED_NAMED_TYPES):
            package = anonymous_package(node, base_package)
        else:
            package = named_package(key, base_package, paths_by_name, first_site, methods_by_path)
        sort_key = identified_model.fingerprint_of(node) or key
        yield _Entry(node=node, package=package, class_name=class_names.named[key], sort_key=sort_key)

def _anonymous_entries(identified_model, class_names, base_package):
    for node_id, node in class_names.anonymous_nodes.items():
        package = anonymous_package(node, base_package)
        '# e.g. UnsupportedNode: never gets a class'
        if package is None:
            continue
        sort_key = identified_model.fingerprint_of(node) or str(node_id)
        yield _Entry(node=node, package=package, class_name=class_names.anonymous[node_id], sort_key=sort_key)

def _transport_entries(identified_model, class_names, base_package, methods_by_path):
    for operation_model in identified_model.operations:
        operation = operation_model.operation
        request_node = operation_model.request
        "# a $ref body IS the request root -- the named schema's own entry already"
        '# covers it (see _mirror_ref_request_names); only inline/anonymous bodies get'
        '# their own synthetic-name entry here.'
        if request_node is not None and request_node.body is not None and (not isinstance(request_node.body.target, RefNode)):
            site = Site(path=operation.path, side='request', method=operation.method, code=None, content_type='json')
            yield _Entry(node=request_node, package=site_package(site, base_package, methods_by_path), class_name=class_names.transport[id(request_node)], sort_key=f'request:{operation.path}:{operation.method}')
        response_node = operation_model.response
        yield _Entry(node=response_node, package=response_root_package(operation.path, operation.method, base_package, methods_by_path), class_name=class_names.transport[id(response_node)], sort_key=f'response:{operation.path}:{operation.method}')
        for code_node in response_node.codes:
            for content_type_view in code_node.content_types:
                site = Site(
                    path=operation.path,
                    side='response',
                    method=operation.method,
                    code=code_node.status_code,
                    content_type=content_type_short_name(
                        content_type_view.content_type))
                package = site_package(site, base_package, methods_by_path)
                sort_suffix = f'{
                    operation.path}:{
                        operation.method}:{
                            code_node.status_code}:{
                                content_type_view.content_type}'
                '# CodeNode/ContentTypeView are one synthetic class (doc bullet 18): only the'
                '# CodeNode competes for collision numbering; ContentTypeView mirrors its name below.'
                yield _Entry(node=code_node, package=package, class_name=class_names.transport[id(code_node)], sort_key=f'code:{sort_suffix}')

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
_PRIVATIZABLE_KINDS = (EnumNode, ListNode, ObjectNode, CompositionNode, DictionaryNode, AnyDictionaryNode)

def _resolve_private_package(node, edge_index, default_package: dict, named_ids: set):
    """Walk up the exclusive-ownership chain: a node reached by exactly one
    edge from exactly one owner is "private" and lives with that owner
    instead of its kind-bucketed shared package (.enums/.objects/...).

    The chain keeps climbing through further private, kind-bucketed anonymous
    owners, and stops at the first boundary:
    - a named schema root -> adopt its already-resolved package.
    - a *shared* kind-bucketed owner (referenced from more than one place) ->
      the chain breaks here; the original node falls back to its own
      kind-bucketed default (it is effectively shared too, just not directly).
    - anything else (e.g. a transport wrapper, or ambiguous ownership) ->
      same fallback.
    Returns None when no override applies (caller keeps the default package).
    """
    if edge_index is None or edge_index.site_count(node) != 1:
        return None
    current, seen = (node, set())
    while True:
        seen.add(id(current))
        owners = edge_index.owners(current)
        if len(owners) != 1:
            return None
        owner = owners[0]
        if id(owner) in seen:
            return None
        if id(owner) in named_ids:
            return default_package.get(id(owner))
        if not isinstance(owner, _PRIVATIZABLE_KINDS) or edge_index.site_count(owner) != 1:
            return None
        current = owner

def _apply_private_packages(entries: list, edge_index, default_package: dict, named_ids: set) -> None:
    """Relocate private (exclusively-owned) Enum/List/Dict/Object/Composition
    entries from their shared, kind-bucketed package to their owner's package."""
    for entry in entries:
        if isinstance(entry.node, _PRIVATIZABLE_KINDS):
            package = _resolve_private_package(entry.node, edge_index, default_package, named_ids)
            if package is not None:
                entry.package = package

def _resolve_collisions(entries: list) -> None:
    """Same (package, class_name) from different structures -> Name, Name2, Name3."""
    buckets: dict = {}
    for entry in entries:
        buckets.setdefault((entry.package, entry.class_name), []).append(entry)
    for group in buckets.values():
        group.sort(key=lambda entry: entry.sort_key)
        for index, entry in enumerate(group, 1):
            entry.final_name = entry.class_name if index == 1 else f'{entry.class_name}{index}'

def _meaningful_owner_label(node, edge_index):
    """The label of `node`'s single incoming edge, skipping meaningless
    numeric hops (composition-branch / tuple-list-position indices) by
    walking up to their owner instead -- a oneOf branch has no name of its
    own, but the property that holds the oneOf usually does.

    Returns (label, node_at_that_level) so a caller can climb one more level
    from there, or (None, None) if no single unambiguous non-numeric label
    is found along the way (multiple/zero incoming edges, or a shared
    ancestor with more than one owner).
    """
    current, seen = (node, set())
    while True:
        if id(current) in seen:
            return (None, None)
        seen.add(id(current))
        labels = edge_index.labels(current)
        if len(labels) != 1:
            return (None, None)
        label = next(iter(labels))
        if not label.isdigit():
            return (label, current)
        owners = edge_index.owners(current)
        if len(owners) != 1:
            return (None, None)
        current = owners[0]

def _enum_edge_candidates(node, edge_index) -> list:
    """Names derived from how an Enum is *used*, most to least specific.

    Only proposed when a single unambiguous, non-numeric label reaches the
    node (e.g. all incoming edges named "role"): 'RoleEnum'. If that also
    needs to disambiguate against the owning object's own incoming edge
    (also unanimous), a one-level-up variant is offered too: 'OutputRoleEnum'.
    Otherwise: no candidates, the caller keeps the structural (value-based) name.
    """
    if edge_index is None:
        return []
    label, at_node = _meaningful_owner_label(node, edge_index)
    if label is None:
        return []
    candidates = [f'{class_identifier(label)}Enum']
    owners = edge_index.owners(at_node)
    if len(owners) == 1:
        parent_label, _ = _meaningful_owner_label(owners[0], edge_index)
        if parent_label is not None:
            candidates.append(f'{class_identifier(parent_label)}{class_identifier(label)}Enum')
    return candidates

def _apply_enum_edge_names(entries: list, edge_index) -> None:
    """Prefer an edge-derived Enum name ('RoleEnum') over the value-derived
    one, when it does not collide with any other entry's current name in the
    same package. Runs before `_resolve_collisions` so a genuine remaining
    collision still falls back to numeric disambiguation as usual.
    """
    if edge_index is None:
        return
    by_package_name: dict = {}
    for entry in entries:
        by_package_name.setdefault((entry.package, entry.class_name), []).append(entry)
    for entry in sorted(entries, key=lambda entry: (entry.package, entry.sort_key)):
        if not isinstance(entry.node, EnumNode):
            continue
        for candidate in _enum_edge_candidates(entry.node, edge_index):
            key = (entry.package, candidate)
            holders = by_package_name.get(key, [])
            if holders and holders != [entry]:
                continue
            old_key = (entry.package, entry.class_name)
            by_package_name[old_key].remove(entry)
            entry.class_name = candidate
            by_package_name.setdefault(key, []).append(entry)
            break