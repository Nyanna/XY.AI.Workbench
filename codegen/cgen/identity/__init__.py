"""Computes structural identity (fingerprints) and deduplicates IR nodes.

State-minimization step of the generator: every node gets a canonical
fingerprint; anonymous nodes sharing a fingerprint collapse onto a
single object (dedup), while named components.schemas are rebuilt in place
and never merged, even on fingerprint collision. Naming/packaging
and Java type mapping (06) are out of scope here.
"""

from dataclasses import dataclass

from cgen.identity.dedup import canonicalize_model
from cgen.model.build import Model, OperationModel
from cgen.model.nodes import Node


@dataclass(frozen=True)
class IdentifiedModel:
    """The IR after fingerprinting/dedup: same shape as Model, canonicalized.

    `fingerprints` maps id(node) -> canonical fingerprint for every node in
    the shared-type graph (RefNode..UnsupportedNode); transport root kinds
    (RequestNode/ResponseNode/CodeNode/ContentTypeView) are never deduped and
    carry no fingerprint. Node object identity (`is`) now reflects sharing:
    two edges pointing at the same anonymous structure point at the same
    Python object.
    """

    named_nodes: dict[str, Node]
    operations: tuple[OperationModel, ...]
    fingerprints: dict[int, str]

    def fingerprint_of(self, node: Node) -> str | None:
        """Look up a node's fingerprint, or None for un-fingerprinted (root) kinds."""
        return self.fingerprints.get(id(node))


def compute_identity(model: Model) -> IdentifiedModel:
    """Assign fingerprints to nodes and resolve anonymous-node sharing/dedup."""
    named_nodes, operations, ctx = canonicalize_model(model)
    return IdentifiedModel(named_nodes=named_nodes, operations=operations, fingerprints=ctx.fingerprints)
