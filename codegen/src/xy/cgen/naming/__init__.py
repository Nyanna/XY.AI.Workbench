"""Assigns Java class names and package paths to identified IR nodes.

Naming is strictly the last pipeline stage and works exclusively on the
already-deduped DAG plus its edge-reverse index (see identity/optimize.py);
it never invents or reserves names during dedup/identity itself.

Two name sources feed one collision-resolved result:
- structural class names (named schemas by key, anonymous nodes by shape --
  see naming/names.py), refined by edge-derived names where an unambiguous
  edge label exists (RoleEnum, TagsList, AnyOfStatus, ...);
- per-operation transport roots (RequestNode/ResponseNode/CodeNode/ContentTypeView).

Package placement: every node is private under its single owner by default
(climbing the ownership chain up to a request/response transport root or a
named schema's own package); only a node actually referenced from more than
one place (`edge_index.site_count(node) > 1`) is shared -- a named top-level
schema goes to '.components' (even when it is also a List/Enum/Dict/
Composition), an anonymous one to its kind bucket ('.enums'/'.lists'/...).

Collisions (same package + same class name from different structures) are
resolved by: (1) an edge-derived name if unambiguous, (2) climbing one
ancestor level for semantic disambiguation, (3) a numeric suffix in
canonical-fingerprint order -- never discovery order.
"""
from dataclasses import dataclass, field
from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, RefNode
from xy.cgen.naming.identifiers import class_identifier, content_type_short_name
from xy.cgen.naming.names import COMPOSITION_KEYWORD_NAME, KIND_SUFFIX, derive_class_names
from xy.cgen.naming.packages import Site, anonymous_package, collect_methods_by_path, response_root_package, site_package

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
    edge_index = identified_model.edge_index
    methods_by_path = collect_methods_by_path(identified_model)
    transport_owner_packages = _transport_owner_packages(identified_model, base_package, methods_by_path)
    entries: list[_Entry] = []
    entries.extend(_schema_entries(identified_model, class_names, base_package, edge_index, transport_owner_packages))
    entries.extend(_transport_entries(identified_model, class_names, base_package, methods_by_path))
    _apply_edge_names(entries, edge_index)
    _resolve_collisions(entries, edge_index)
    names = {id(entry.node): NodeName(package=entry.package, class_name=entry.final_name) for entry in entries}
    _mirror_content_type_view_names(identified_model, names)
    _mirror_ref_request_names(identified_model, names)
    return NamedModel(named_nodes=identified_model.named_nodes, operations=identified_model.operations,
                      fingerprints=identified_model.fingerprints, base_package=base_package, names=names)
_SHARED = object()
"# ownership-chain cycle guard sentinel: 'currently being resolved'"

def _shared_package(node, base_package: str, named_ids: set) -> str:
    """The package a node falls back to once it counts as genuinely shared
    (site_count > 1), or when its ownership chain is ambiguous/unreachable."""
    if id(node) in named_ids:
        return f'{base_package}.components'
    return anonymous_package(node, base_package) or f'{base_package}.objects'

def _resolve_package(node, edge_index, base_package: str, named_ids: set, transport_owner_packages: dict, memo: dict) -> str:
    """Default: private under the single owner that reaches this node --
    climbing the ownership chain up to a request/response transport root or
    a genuinely shared ancestor. Exception: a node referenced from more than
    one place (site_count > 1) is shared itself."""
    key = id(node)
    cached = memo.get(key)
    if cached is _SHARED:
        return _shared_package(node, base_package, named_ids)
    if cached is not None:
        return cached
    if edge_index.site_count(node) > 1:
        package = _shared_package(node, base_package, named_ids)
        memo[key] = package
        return package
    owners = edge_index.owners(node)
    if len(owners) != 1:
        package = _shared_package(node, base_package, named_ids)
        memo[key] = package
        return package
    owner = owners[0]
    owner_id = id(owner)
    if owner_id in transport_owner_packages:
        package = transport_owner_packages[owner_id]
    else:
        memo[key] = _SHARED
        package = _resolve_package(owner, edge_index, base_package, named_ids, transport_owner_packages, memo)
    memo[key] = package
    return package

def _transport_owner_packages(identified_model, base_package: str, methods_by_path: dict) -> dict:
    """id(node) -> package for every transport node that can own schema
    children directly (RequestNode via its body, ContentTypeView via its
    body) -- the roots the private-ownership chain climbs up to."""
    packages: dict = {}
    for operation_model in identified_model.operations:
        operation = operation_model.operation
        request_node = operation_model.request
        if request_node is not None and request_node.body is not None:
            site = Site(path=operation.path, side='request', method=operation.method, code=None, content_type='json')
            packages[id(request_node)] = site_package(site, base_package, methods_by_path)
        for code_node in operation_model.response.codes:
            for content_type_view in code_node.content_types:
                site = Site(
                    path=operation.path,
                    side='response',
                    method=operation.method,
                    code=code_node.status_code,
                    content_type=content_type_short_name(
                        content_type_view.content_type))
                packages[id(content_type_view)] = site_package(site, base_package, methods_by_path)
    return packages

def _schema_entries(identified_model, class_names, base_package, edge_index, transport_owner_packages):
    named_ids = {id(node) for node in identified_model.named_nodes.values()}
    memo: dict = {}
    for key, node in identified_model.named_nodes.items():
        package = _resolve_package(node, edge_index, base_package, named_ids, transport_owner_packages, memo)
        sort_key = identified_model.fingerprint_of(node) or key
        yield _Entry(node=node, package=package, class_name=class_names.named[key], sort_key=sort_key)
    for node_id, node in class_names.anonymous_nodes.items():
        package = _resolve_package(node, edge_index, base_package, named_ids, transport_owner_packages, memo)
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
_MAX_CLIMB_DEPTH = 6

def _label_chain(owner, label, edge_index) -> list:
    """Nearest-first list of meaningful (non-numeric) labels reachable by
    climbing from a single incoming edge (owner, label): numeric/positional
    hops (composition-branch / tuple-list-position indices) are skipped
    silently by climbing through them, via that owner's own first incoming
    edge -- deterministic, not requiring a uniquely-owned chain.

    Each further entry in the list is one more ancestor level out, used to
    extend a name when a shorter one collides. Stops at a cycle, a node with
    no further incoming edge, or the depth cap.
    """
    chain = []
    current_owner, current_label, seen = (owner, label, set())
    for _ in range(_MAX_CLIMB_DEPTH):
        if current_label is not None and (not current_label.isdigit()):
            chain.append(current_label)
        if id(current_owner) in seen:
            break
        seen.add(id(current_owner))
        sites = edge_index.sites(current_owner)
        if not sites:
            break
        current_owner, current_label = sites[0]
    return chain

def _label_chains(node, edge_index) -> list:
    """One nearest-first label chain per site (incoming edge) reaching `node`."""
    return [_label_chain(owner, label, edge_index) for owner, label in edge_index.sites(node)]
_EDGE_NAMEABLE_KINDS = (EnumNode, ListNode, DictionaryNode, AnyDictionaryNode, CompositionNode)

def _build_edge_name(node, labels: list) -> str:
    """Build a name from a chain of labels (nearest-first): farther ancestors
    become an outer prefix, the immediate label sits right next to the
    kind marker (AllOf/AnyOf/OneOf prefix, List/Enum/Dict/AnyDict suffix)."""
    parts = ''.join((class_identifier(part) for part in reversed(labels)))
    if isinstance(node, CompositionNode):
        return COMPOSITION_KEYWORD_NAME[node.keyword] + parts
    return parts + KIND_SUFFIX[type(node)]

def _edge_name_rounds(node, edge_index):
    """Yield successive rounds of edge-derived name candidates, one round
    per ancestor depth: round 1 is each site's own (digit-hops-skipped)
    label, round 2 prepends each site's next ancestor label, etc. -- tried
    in order by the caller, so a round is only reached once every candidate
    from the previous, shorter round collided.
    """
    chains = _label_chains(node, edge_index)
    max_depth = max((len(chain) for chain in chains), default=0)
    for depth in range(1, max_depth + 1):
        round_names = [_build_edge_name(node, chain[:depth]) for chain in chains if len(chain) >= depth]
        if round_names:
            yield round_names

def _apply_edge_names(entries: list, edge_index) -> None:
    """Prefer an edge-derived name ('RoleEnum', 'TagsList', 'AnyOfStatus', ...)
    over the structural (value-/content-based) one: try each site's name in
    turn; if all sites collide at one ancestor depth, climb to the next
    depth (one more ancestor label prepended) and retry every site again.
    Runs before `_resolve_collisions` so a genuine remaining collision still
    falls back to ancestor-climbing/numeric disambiguation as usual.
    """
    by_package_name: dict = {}
    for entry in entries:
        by_package_name.setdefault((entry.package, entry.class_name), []).append(entry)
    for entry in sorted(entries, key=lambda entry: (entry.package, entry.sort_key)):
        if not isinstance(entry.node, _EDGE_NAMEABLE_KINDS):
            continue
        chosen = None
        for round_names in _edge_name_rounds(entry.node, edge_index):
            for candidate in round_names:
                key = (entry.package, candidate)
                holders = by_package_name.get(key, [])
                if holders and holders != [entry]:
                    continue
                chosen = candidate
                break
            if chosen is not None:
                break
        if chosen is None:
            continue
        old_key = (entry.package, entry.class_name)
        by_package_name[old_key].remove(entry)
        entry.class_name = chosen
        by_package_name.setdefault((entry.package, chosen), []).append(entry)

def _climb_name_rounds(entry, edge_index):
    """Yield successive rounds of ancestor-qualified name candidates for a
    colliding entry: round 1 prepends each site's own label to the current
    class name, round 2 prepends two ancestor levels, etc. -- same
    iterate-then-climb algorithm as `_edge_name_rounds`, generalized to
    every kind (not just the edge-nameable ones), since any structural name
    can still collide.
    """
    chains = _label_chains(entry.node, edge_index)
    max_depth = max((len(chain) for chain in chains), default=0)
    for depth in range(1, max_depth + 1):
        round_names = [
            f'{''.join((class_identifier(part) for part in reversed(chain[:depth])))}{entry.class_name}' for chain in chains if len(chain) >= depth]
        if round_names:
            yield round_names

def _resolve_collisions(entries: list, edge_index) -> None:
    """Same (package, class_name) from different structures:
    1. try climbing ancestor levels per colliding entry, one site at a time,
       one more level for all sites once every site collided at the
       previous, shorter level (e.g. RoleEnum -> UserRoleEnum);
    2. anything still colliding afterward gets a numeric suffix
       (Name, Name2, Name3...), deterministically ordered by fingerprint.
    """
    buckets: dict = {}
    for entry in entries:
        buckets.setdefault((entry.package, entry.class_name), []).append(entry)
    for group in list(buckets.values()):
        if len(group) <= 1:
            continue
        for entry in list(group):
            chosen = None
            for round_names in _climb_name_rounds(entry, edge_index):
                for candidate in round_names:
                    key = (entry.package, candidate)
                    holders = buckets.get(key, [])
                    if holders and holders != [entry]:
                        continue
                    chosen = candidate
                    break
                if chosen is not None:
                    break
            if chosen is None:
                continue
            old_key = (entry.package, entry.class_name)
            buckets[old_key].remove(entry)
            entry.class_name = chosen
            buckets.setdefault((entry.package, chosen), []).append(entry)
    final_buckets: dict = {}
    for entry in entries:
        final_buckets.setdefault((entry.package, entry.class_name), []).append(entry)
    for group in final_buckets.values():
        group.sort(key=lambda entry: entry.sort_key)
        for index, entry in enumerate(group, 1):
            entry.final_name = entry.class_name if index == 1 else f'{entry.class_name}{index}'