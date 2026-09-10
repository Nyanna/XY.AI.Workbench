"""Canonical structural fingerprint for a single IR node.

A fingerprint depends only on `kind` plus the already-known fingerprints of
direct children -- never on edge metadata (description/example/default)
and never on a RefNode's target expansion (the target's id-token IS the
fingerprint). This lets the caller hash the node graph bottom-up in one pass.
"""

import hashlib

from cgen.model.nodes import (
    AnyDictionaryNode,
    CompositionNode,
    DictionaryNode,
    EnumNode,
    ListNode,
    Node,
    ObjectNode,
    PrimitiveNode,
    RefNode,
    UnsupportedNode,
)


def fingerprint_of(node: Node, child_fingerprints: dict) -> str:
    """Hash the canonical form of `node`.

    `child_fingerprints` maps id(child_node) -> fingerprint for every direct
    child target this node may reference; children must already be present
    (bottom-up/post-order traversal).
    """
    canonical = _canonical_form(node, child_fingerprints)
    return hashlib.sha256(repr(canonical).encode("utf-8")).hexdigest()


def _canonical_form(node: Node, child_fp: dict):
    if isinstance(node, RefNode):
        return ("ref", node.name)
    if isinstance(node, PrimitiveNode):
        return ("primitive", node.primitive_type)
    if isinstance(node, EnumNode):
        return ("enum", node.primitive_type, _canonical_values(node.values))
    if isinstance(node, ObjectNode):
        properties = tuple(
            sorted(
                ((edge.label, child_fp[id(edge.target)]) for edge in node.properties),
                key=lambda pair: pair[0],
            )
        )
        return ("object", properties, tuple(sorted(node.required)))
    if isinstance(node, ListNode):
        # element order is positional for mixed lists -- never sorted
        elements = tuple(child_fp[id(edge.target)] for edge in node.elements)
        return ("list", node.mixed, elements)
    if isinstance(node, DictionaryNode):
        return ("dictionary", child_fp[id(node.value.target)])
    if isinstance(node, AnyDictionaryNode):
        return ("any_dictionary",)
    if isinstance(node, CompositionNode):
        # branch order is positional (discriminator/branch-index relevant) -- never sorted
        branches = tuple(child_fp[id(edge.target)] for edge in node.branches)
        return ("composition", node.keyword, branches, _canonical_discriminator(node.discriminator))
    if isinstance(node, UnsupportedNode):
        return ("unsupported", node.reason)
    raise TypeError(f"cannot fingerprint node kind: {node.kind!r}")


def _canonical_values(values: tuple) -> tuple:
    """Enum values are a set, not a sequence -- sort by (type, repr) for a stable order."""
    return tuple(sorted(((type(v).__name__, v) for v in values), key=lambda pair: (pair[0], repr(pair[1]))))


def _canonical_discriminator(discriminator) -> tuple | None:
    if discriminator is None:
        return None
    mapping = tuple(sorted(discriminator.mapping.items())) if discriminator.mapping else None
    return (discriminator.property_name, mapping)
