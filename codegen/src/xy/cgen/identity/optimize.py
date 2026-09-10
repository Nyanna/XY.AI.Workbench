"""Edge-reverse index: which edge labels reach each shared node.

Built as its own pipeline stage, right after dedup: since anonymous shared
nodes are already merged at that point, every edge that points at the same
object contributes its label here. The naming stage uses this to derive
semantic names/packages from *how* a node is used (e.g. an Enum whose only
incoming edges are all labeled "role" can be called RoleEnum), instead of
purely from its structural shape.
"""
from dataclasses import dataclass, field
from xy.cgen.identity import IdentifiedModel
from xy.cgen.model.nodes import Node, RefNode
from xy.cgen.naming.traverse import iter_child_edges

@dataclass(frozen=True)
class EdgeIndex:
    """id(target node) -> every (owner node, edge label) that points at it."""
    incoming: dict = field(default_factory=dict)

    def labels(self, node: Node) -> frozenset:
        return frozenset((label for _, label in self.incoming.get(id(node), ())))

    def site_count(self, node: Node) -> int:
        return len(self.incoming.get(id(node), ()))

    def owners(self, node: Node) -> tuple:
        seen: dict = {}
        for owner, _ in self.incoming.get(id(node), ()):
            if owner is not None:
                seen[id(owner)] = owner
        return tuple(seen.values())

@dataclass(frozen=True)
class OptimizedModel:
    """An IdentifiedModel plus its edge-reverse index. Drop-in compatible with
    IdentifiedModel for all downstream naming code (same attributes/methods)."""
    named_nodes: dict
    operations: tuple
    fingerprints: dict
    edge_index: EdgeIndex

    def fingerprint_of(self, node: Node) -> str | None:
        return self.fingerprints.get(id(node))

def optimize_identity(identified_model: IdentifiedModel) -> OptimizedModel:
    """Build the edge-reverse index over an already-deduped model."""
    incoming: dict = {}

    def record(owner, label: str, target: Node) -> None:
        incoming.setdefault(id(target), []).append((owner, label))
        '# a $ref edge also "reaches" the named schema it points at -- record the'
        "# same label there too, so e.g. a single-use $ref'd enum can be named"
        '# from its use site just like an inline one (see naming.enum candidates).'
        if isinstance(target, RefNode):
            resolved = identified_model.named_nodes.get(target.name)
            if resolved is not None:
                incoming.setdefault(id(resolved), []).append((owner, label))

    def visit(node: Node, visited: set) -> None:
        if id(node) in visited:
            return
        visited.add(id(node))
        for label, edge in iter_child_edges(node):
            record(node, label, edge.target)
            visit(edge.target, visited)
    visited: set = set()
    for node in identified_model.named_nodes.values():
        visit(node, visited)
    for operation_model in identified_model.operations:
        request = operation_model.request
        if request is not None and request.body is not None:
            record(request, request.body.label, request.body.target)
            visit(request.body.target, visited)
        for code_node in operation_model.response.codes:
            for content_type_view in code_node.content_types:
                record(content_type_view, content_type_view.body.label, content_type_view.body.target)
                visit(content_type_view.body.target, visited)
    return OptimizedModel(
        named_nodes=identified_model.named_nodes,
        operations=identified_model.operations,
        fingerprints=identified_model.fingerprints,
        edge_index=EdgeIndex(
            incoming={
                k: tuple(v) for k,
                v in incoming.items()}))