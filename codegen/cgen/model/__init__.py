"""Builds the intermediate representation (IR) from ingested schema data."""

from cgen.model.build import Model, OperationModel, build_model
from cgen.model.nodes import (
    AnyDictionaryNode,
    CodeNode,
    CompositionNode,
    ContentTypeView,
    Discriminator,
    DictionaryNode,
    Edge,
    EnumNode,
    ListNode,
    Node,
    ObjectNode,
    PrimitiveNode,
    RefNode,
    RequestNode,
    ResponseNode,
    UnsupportedNode,
)

__all__ = [
    "AnyDictionaryNode",
    "CodeNode",
    "CompositionNode",
    "ContentTypeView",
    "Discriminator",
    "DictionaryNode",
    "Edge",
    "EnumNode",
    "ListNode",
    "Model",
    "Node",
    "ObjectNode",
    "OperationModel",
    "PrimitiveNode",
    "RefNode",
    "RequestNode",
    "ResponseNode",
    "UnsupportedNode",
    "build_model",
]
