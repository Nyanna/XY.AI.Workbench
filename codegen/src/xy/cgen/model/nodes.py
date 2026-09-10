"""IR node and edge types (the state graph).

A node describes structure only: no name, no Java type, no schema metadata.
All schema-site metadata (description/example/default) lives on the Edge
that points at a node, never on the node itself -- this is what allows two
structurally identical nodes to become one shared type later.
"""

from dataclasses import dataclass, field
from typing import ClassVar

MISSING = object()  # sentinel: distinguishes "no default/example given" from an explicit None


class Node:
    """Base marker for all IR node kinds."""

    kind: ClassVar[str]


@dataclass(frozen=True)
class Edge:
    """A labeled transition from a node to its target node.

    Carries the schema-site metadata for this particular use of the target
    (description/example/default), including $ref-sibling overrides (OpenAPI 3.1).
    """

    label: str
    target: Node
    description: str | None = None
    example: object = MISSING
    default: object = MISSING


@dataclass(frozen=True)
class Discriminator:
    """Raw discriminator declaration on a CompositionNode.

    Only what the schema states verbatim; resolving const/enum values per
    branch into a value->branch map is a later concern.
    """

    property_name: str
    mapping: dict | None = None


@dataclass(frozen=True)
class RefNode(Node):
    """Atomic reference to a named components.schemas node. Never expanded."""

    kind: ClassVar[str] = "ref"
    name: str


@dataclass(frozen=True)
class PrimitiveNode(Node):
    """One of string | integer | number | boolean | null."""

    kind: ClassVar[str] = "primitive"
    primitive_type: str


@dataclass(frozen=True)
class EnumNode(Node):
    """A closed value set over a primitive base type."""

    kind: ClassVar[str] = "enum"
    primitive_type: str
    values: tuple = ()


@dataclass(frozen=True)
class ObjectNode(Node):
    """Named properties (edge label = property name) plus a required-set."""

    kind: ClassVar[str] = "object"
    properties: tuple = ()  # tuple[Edge, ...], declaration order preserved
    required: frozenset = field(default_factory=frozenset)


@dataclass(frozen=True)
class ListNode(Node):
    """A JSON array. Normally one 'element' edge; MixedList has one edge per position."""

    kind: ClassVar[str] = "list"
    elements: tuple = ()  # tuple[Edge, ...]
    mixed: bool = False


@dataclass(frozen=True)
class DictionaryNode(Node):
    """A map with a typed value schema (additionalProperties: {schema})."""

    kind: ClassVar[str] = "dictionary"
    value: Edge = None


@dataclass(frozen=True)
class AnyDictionaryNode(Node):
    """additionalProperties: true|{} -- a raw JsonNode passthrough map."""

    kind: ClassVar[str] = "any_dictionary"


@dataclass(frozen=True)
class CompositionNode(Node):
    """allOf/anyOf/oneOf. Branch order is preserved, never sorted (positional)."""

    kind: ClassVar[str] = "composition"
    keyword: str  # 'allOf' | 'anyOf' | 'oneOf'
    branches: tuple = ()  # tuple[Edge, ...]
    discriminator: Discriminator | None = None


@dataclass(frozen=True)
class UnsupportedNode(Node):
    """A schema construct that is explicitly out of scope."""

    kind: ClassVar[str] = "unsupported"
    reason: str = ""


# --- Root / transport nodes (not part of the shared-type graph, never deduped) ---


@dataclass(frozen=True)
class ContentTypeView(Node):
    """One content-type of a CodeNode, wrapping the body schema node."""

    kind: ClassVar[str] = "content_type_view"
    content_type: str
    body: Edge


@dataclass(frozen=True)
class CodeNode(Node):
    """One status code of a ResponseNode."""

    kind: ClassVar[str] = "code"
    status_code: str
    content_types: tuple = ()  # tuple[ContentTypeView, ...]


@dataclass(frozen=True)
class ResponseNode(Node):
    """Root node of an operation's response side; children are CodeNodes."""

    kind: ClassVar[str] = "response"
    codes: tuple = ()  # tuple[CodeNode, ...]


@dataclass(frozen=True)
class RequestNode(Node):
    """Root node of an operation's request side; wraps the body structure."""

    kind: ClassVar[str] = "request"
    body: Edge | None = None
