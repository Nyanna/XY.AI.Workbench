"""Builds the Jinja2 template context for one model-emission node.

Every child edge is classified into one of accessor strategies:
- 'primitive': read/write a scalar value directly on the bound JsonNode.
- 'enum': read/write the raw scalar, converted through the generated enum's
  rawValue()/fromValue().
- 'any_dictionary': additionalProperties:true|{} -- a raw JsonNode passthrough,
  no wrapper class.
- 'composition': like 'complex', but the getter must distinguish an absent
  field from an explicitly-null one -- only 'child == null' collapses to
  Java null, 'child.isNull()' still yields a bound view (whose own isNull()
  branch check reports true).
- 'complex': lazily instantiate a child proxy class bound to the sub-node.
- 'unsupported': no accessor is generated at all.

Classification follows a RefNode chain to see the referenced node's actual
kind. This never expands/merges structure into the IR
(the IR itself is untouched); it only informs which Java code shape to emit.
"""
import re
from dataclasses import dataclass
from xy.cgen.model.nodes import MISSING, AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode, PrimitiveNode, RefNode, UnsupportedNode
from xy.cgen.naming.identifiers import property_accessor_name, to_pascal_case
from xy.cgen.naming.names import PRIMITIVE_BRANCH_NAME
from xy.cgen.lang import get_language
from xy.cgen.typemap import map_type
'# per-language scalar read/cast and factory methods, see xy.cgen.lang.'

@dataclass(frozen=True)
class Accessor:
    """One child edge rendered as a getter/setter (or get/add/remove, get/put/remove)."""
    label: str
    "# PascalCase accessor fragment, e.g. getTopLogprobs -> 'TopLogprobs'"
    name: str
    java_type: str
    "# 'primitive' | 'enum' | 'any_dictionary' | 'complex'"
    category: str
    read_method: str | None
    factory_method: str | None
    description: str | None
    example_repr: str | None

def resolve_structural(node, named_nodes: dict):
    """Follow a RefNode chain to the underlying structural node.

    Used only to pick a codegen strategy (which accessor shape to emit);
    the IR itself keeps $ref atomic.
    """
    seen = set()
    while isinstance(node, RefNode):
        if node.name in seen:
            raise ValueError(f'cyclic reference chain at {node.name!r}')
        seen.add(node.name)
        node = named_nodes[node.name]
    return node

def classify(node, named_nodes: dict) -> tuple[str, str | None]:
    """-> (category, primitive_type). primitive_type is set for primitive/enum only.

    'any_dictionary' mirrors map_type's own precedence: it
    only collapses to raw JsonNode for a *direct* AnyDictionaryNode edge --
    map_type never resolves through a RefNode, so a named
    additionalProperties:true schema reached via $ref keeps its own
    generated proxy class and must be classified as 'complex' here too,
    or the getter's declared return type and its body would disagree.
    """
    if isinstance(node, AnyDictionaryNode):
        return ('any_dictionary', None)
    resolved = resolve_structural(node, named_nodes)
    if isinstance(resolved, PrimitiveNode):
        return ('primitive', resolved.primitive_type)
    if isinstance(resolved, EnumNode):
        return ('enum', resolved.primitive_type)
    if isinstance(resolved, UnsupportedNode):
        return ('unsupported', None)
    if isinstance(resolved, CompositionNode):
        return ('composition', None)
    return ('complex', None)

def build_accessor(label: str, edge, named_model) -> Accessor | None:
    """Build the accessor context for one edge, or None if it has no view."""
    category, primitive_type = classify(edge.target, named_model.named_nodes)
    if category == 'unsupported':
        return None
    lang = get_language(named_model.language)
    has_read = category in ('primitive', 'enum')
    return Accessor(
        label=label,
        name=to_pascal_case(
            property_accessor_name(label)),
        java_type=map_type(
            edge.target,
            named_model),
        category=category,
        read_method=lang.read_method.get(primitive_type) if has_read else None,
        factory_method=lang.factory_method.get(primitive_type) if has_read else None,
        description=edge.description,
        example_repr=None if edge.example is MISSING else repr(
            edge.example))
'# --- Enum-specific context -------------------------------------------------'
_WORD_BOUNDARY = re.compile('[^A-Za-z0-9]+')

@dataclass(frozen=True)
class EnumConstant:
    constant_name: str
    literal: str

def enum_constants(node, language: str='java') -> list[EnumConstant]:
    """One enum constant per declared value, in declaration order (deterministic input)."""
    lang = get_language(language)
    seen_names: dict = {}
    constants = []
    for value in node.values:
        base = _constant_base(value)
        seen_names[base] = seen_names.get(base, 0) + 1
        name = base if seen_names[base] == 1 else f'{base}_{seen_names[base]}'
        constants.append(EnumConstant(constant_name=name, literal=lang.literal(value, node.primitive_type)))
    return constants

def _constant_base(value) -> str:
    words = [w for w in _WORD_BOUNDARY.split(str(value)) if w]
    if not words:
        return 'VALUE'
    base = '_'.join((w.upper() for w in words))
    return f'_{base}' if base[0].isdigit() else base

def enum_raw_type(node, language: str='java') -> str:
    return get_language(language).primitive_type[node.primitive_type]
'# --- Composition context (allOf/anyOf/oneOf proxy views) -----------'
'#'
'# One CompositionNode -> one class holding a single JsonNode. Every branch is'
'# exposed as a view over that *same* node: a getter for allOf, a getter'
'# plus an applies?() structural/discriminator check for anyOf/oneOf. There is'
'# no merge and no setter at this level -- writing goes either through a'
"# branch's own view (for object-shaped branches, since it shares the node) or"
"# through the enclosing property's setter (which replaces the whole bound"
'# node), never through the composition class itself.'

@dataclass(frozen=True)
class Branch:
    """One composition branch, rendered as get<Name>()/is<Name>()."""
    accessor_name: str
    "# None for the 'null' branch (no getter)"
    java_type: str | None
    "# 'primitive' | 'enum' | 'any_dictionary' | 'composition' | 'complex' | 'null'"
    category: str
    read_method: str | None
    description: str | None
    example_repr: str | None
    has_getter: bool
    '# None for allOf branches (always applies, no check)'
    applies_expr: str | None

def _branch_accessor_name(target, named_model) -> str:
    """The branch's own generated class-name fragment."""
    if isinstance(target, RefNode):
        return named_model.name_of_ref(target.name).class_name
    if isinstance(target, PrimitiveNode):
        return PRIMITIVE_BRANCH_NAME[target.primitive_type]
    return named_model.name_of(target).class_name

def build_branches(node, named_model) -> list[Branch]:
    """Build the render context for every branch of one CompositionNode."""
    named_nodes = named_model.named_nodes
    lang = get_language(named_model.language)
    needs_applies = node.keyword in ('anyOf', 'oneOf')
    discriminator_values = _resolve_discriminator_values(
        node, named_nodes) if needs_applies and node.discriminator else {}
    branches = []
    for index, edge in enumerate(node.branches):
        target = edge.target
        resolved = resolve_structural(target, named_nodes)
        if isinstance(resolved, UnsupportedNode):
            '# no view for an unsupported branch'
            continue
        example_repr = None if edge.example is MISSING else repr(edge.example)
        accessor_name = _branch_accessor_name(target, named_model)
        applies_expr = None
        if needs_applies:
            values = discriminator_values.get(index)
            if values:
                applies_expr = ' || '.join(
                    (lang.discriminator_literal_expr(
                        node.discriminator.property_name,
                        value,
                        primitive_type) for value,
                     primitive_type in values))
            else:
                applies_expr = _structural_applies_expr(resolved, named_model.language)
        if isinstance(resolved, PrimitiveNode) and resolved.primitive_type == 'null':
            branches.append(
                Branch(
                    accessor_name=accessor_name,
                    java_type=None,
                    category='null',
                    read_method=None,
                    description=edge.description,
                    example_repr=example_repr,
                    has_getter=False,
                    applies_expr=applies_expr))
            continue
        category, primitive_type = classify(target, named_nodes)
        branches.append(
            Branch(
                accessor_name=accessor_name,
                java_type=map_type(
                    target,
                    named_model),
                category=category,
                read_method=lang.read_method.get(primitive_type) if category in (
                    'primitive',
                    'enum') else None,
                description=edge.description,
                example_repr=example_repr,
                has_getter=True,
                applies_expr=applies_expr))
    return branches

def _resolve_discriminator_values(node, named_nodes) -> dict:
    """Branch index -> [(value, primitive_type), ...] for the discriminator property.

    `mapping` (if present) wins per branch; every branch left unresolved falls
    back to reading the const/single-value-enum of its own discriminator
    property. A branch with neither yields no entry (caller falls back to a
    structural applies? check for it).
    """
    values: dict = {}
    mapping = node.discriminator.mapping
    if mapping:
        ref_to_values: dict = {}
        for value, ref in mapping.items():
            key = ref.rsplit('/', maxsplit=1)[-1] if '/' in ref else ref
            ref_to_values.setdefault(key, []).append(value)
        for index, edge in enumerate(node.branches):
            target = edge.target
            if isinstance(target, RefNode) and target.name in ref_to_values:
                values[index] = [(value, _discriminator_value_type(node, target.name, named_nodes))
                                 for value in ref_to_values[target.name]]
    for index, edge in enumerate(node.branches):
        if index in values:
            continue
        resolved_value = _const_value_for_branch(edge.target, node.discriminator.property_name, named_nodes)
        if resolved_value is not None:
            values[index] = [resolved_value]
    return values

def _discriminator_value_type(node, branch_ref_name: str, named_nodes) -> str:
    resolved_value = _const_value_for_branch(
        RefNode(
            name=branch_ref_name),
        node.discriminator.property_name,
        named_nodes)
    return resolved_value[1] if resolved_value is not None else 'string'

def _const_value_for_branch(target, property_name: str, named_nodes):
    """-> (value, primitive_type) if the branch declares a single fixed value
    for the discriminator property (enum with exactly one value), else None."""
    resolved = resolve_structural(target, named_nodes)
    if not isinstance(resolved, ObjectNode):
        return None
    for prop_edge in resolved.properties:
        if prop_edge.label != property_name:
            continue
        prop_resolved = resolve_structural(prop_edge.target, named_nodes)
        if isinstance(prop_resolved, EnumNode) and len(prop_resolved.values) == 1:
            return (prop_resolved.values[0], prop_resolved.primitive_type)
        return None
    return None

def _structural_applies_expr(resolved, language: str) -> str:
    """presence of required fields / JSON type, no discriminator const available."""
    lang = get_language(language)
    if isinstance(resolved, ObjectNode):
        if not resolved.required:
            return lang.applies_always()
        return lang.applies_required(sorted(resolved.required))
    if isinstance(resolved, ListNode):
        return lang.applies_array()
    if isinstance(resolved, (DictionaryNode, AnyDictionaryNode)):
        return lang.applies_object()
    if isinstance(resolved, EnumNode):
        return lang.applies_type_check(resolved.primitive_type)
    if isinstance(resolved, PrimitiveNode):
        if resolved.primitive_type == 'null':
            return lang.applies_null()
        return lang.applies_type_check(resolved.primitive_type)
    '# nested composition or other structural node: best-effort, never validated further'
    return lang.applies_always()