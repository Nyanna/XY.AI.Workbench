"""Bottom-up rebuild of the IR graph: fingerprints every node, shares anonymous
nodes with equal fingerprint, never merges named components.schemas.

No bisimulation/partition-refinement is needed: cycles only ever run through a
named $ref, which is an atomic id-token, and named schemas are never
merge candidates in the first place -- the only case bisimulation would be
needed for is one that must not happen here.
"""
from xy.cgen.model.build import Model, OperationModel
from xy.cgen.model.nodes import CodeNode, ContentTypeView, Edge, ListNode, Node, ObjectNode, RequestNode, ResponseNode, CompositionNode, DictionaryNode, EnumNode, AnyDictionaryNode
from xy.cgen.identity.fingerprint import fingerprint_of
'# Named schemas of these kinds have purely structural identity (like their'
'# anonymous siblings) -- dedup them'
'# fully instead of exempting them via named=True.'
_KIND_BUCKETED_TYPES = (ListNode, EnumNode, DictionaryNode, AnyDictionaryNode)

class DedupContext:
    """Traversal state: memoized fingerprints plus the anonymous-node canonical registry."""

    def __init__(self):
        """# id(canonical node) -> fingerprint"""
        self.fingerprints: dict[int, str] = {}
        '# anonymous nodes only'
        self._canonical_by_fingerprint: dict[str, Node] = {}

    def canonicalize(self, node: Node, *, named: bool=False) -> Node:
        """Rebuild `node`'s children, fingerprint it, then dedup if anonymous."""
        rebuilt = _rebuild_children(node, self)
        fp = fingerprint_of(rebuilt, self.fingerprints)
        if named:
            self.fingerprints[id(rebuilt)] = fp
            return rebuilt
        existing = self._canonical_by_fingerprint.get(fp)
        if existing is not None:
            return existing
        self.fingerprints[id(rebuilt)] = fp
        self._canonical_by_fingerprint[fp] = rebuilt
        return rebuilt

    def canonicalize_edge(self, edge: Edge) -> Edge:
        new_target = self.canonicalize(edge.target)
        if new_target is edge.target:
            return edge
        return Edge(
            label=edge.label,
            target=new_target,
            description=edge.description,
            example=edge.example,
            default=edge.default)

def _rebuild_children(node: Node, ctx: DedupContext) -> Node:
    """Return a copy of `node` with every child edge's target canonicalized.

    Leaf kinds (RefNode/PrimitiveNode/EnumNode/AnyDictionaryNode/UnsupportedNode)
    carry no child edges and pass through unchanged.
    """
    if isinstance(node, ObjectNode):
        return ObjectNode(properties=tuple((ctx.canonicalize_edge(edge)
                          for edge in node.properties)), required=node.required)
    if isinstance(node, ListNode):
        return ListNode(elements=tuple((ctx.canonicalize_edge(edge) for edge in node.elements)), mixed=node.mixed)
    if isinstance(node, DictionaryNode):
        return DictionaryNode(value=ctx.canonicalize_edge(node.value))
    if isinstance(node, CompositionNode):
        return CompositionNode(keyword=node.keyword, branches=tuple((ctx.canonicalize_edge(edge)
                               for edge in node.branches)), discriminator=node.discriminator)
    return node

def canonicalize_model(model: Model) -> tuple[dict, tuple, DedupContext]:
    """Rebuild every named schema and every operation's transport tree, deduped."""
    ctx = DedupContext()
    named_nodes = {
        name: ctx.canonicalize(
            node,
            named=not isinstance(
                node,
                _KIND_BUCKETED_TYPES)) for name,
        node in model.named_nodes.items()}
    operations = tuple((_canonicalize_operation(operation, ctx) for operation in model.operations))
    return (named_nodes, operations, ctx)

def _canonicalize_operation(operation: OperationModel, ctx: DedupContext) -> OperationModel:
    request = None
    if operation.request is not None:
        body = ctx.canonicalize_edge(operation.request.body) if operation.request.body else None
        request = RequestNode(body=body)
    codes = tuple((_canonicalize_code(code, ctx) for code in operation.response.codes))
    return OperationModel(operation=operation.operation, request=request, response=ResponseNode(codes=codes))

def _canonicalize_code(code: CodeNode, ctx: DedupContext) -> CodeNode:
    content_types = tuple((_canonicalize_content_type(view, ctx) for view in code.content_types))
    return CodeNode(status_code=code.status_code, content_types=content_types)

def _canonicalize_content_type(view: ContentTypeView, ctx: DedupContext) -> ContentTypeView:
    return ContentTypeView(content_type=view.content_type, body=ctx.canonicalize_edge(view.body))