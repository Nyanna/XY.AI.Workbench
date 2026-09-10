"""Package assignment building blocks.

- '.components': a named top-level schema actually referenced from more than
  one place in the deduped graph (`edge_index.site_count(node) > 1`).
- kind buckets ('.enums'/'.lists'/'.objects'/'.operators'/'.dictionaries'):
  an anonymous node referenced from more than one place.
- everything else (site_count <= 1, i.e. privately/uniquely owned) lives
  under its single owner's own package instead of a shared bucket -- see
  `naming/__init__.py::_resolve_package`, which climbs the ownership chain
  up to either a named schema's own (already resolved) package or a
  request/response transport root.
- '.request.<path>.<method>.<ct>': the request root of one operation.
- '.response.<path>[.<method>].code<code>.<ct>': one status-code/content-type
  view of one operation; the method segment is dropped when the path has
  exactly one method, kept otherwise to stay unambiguous.
"""
import re
from collections import defaultdict
from dataclasses import dataclass
from xy.cgen.model.nodes import AnyDictionaryNode, CompositionNode, DictionaryNode, EnumNode, ListNode, ObjectNode
from xy.cgen.naming.paths import path_to_package_segments
ANONYMOUS_KIND_PACKAGE = {
    EnumNode: 'enums',
    ListNode: 'lists',
    ObjectNode: 'objects',
    CompositionNode: 'operators',
    DictionaryNode: 'dictionaries',
    AnyDictionaryNode: 'dictionaries'}

@dataclass(frozen=True)
class Site:
    """Where a transport-rooted node sits: the request/response location a
    package can be derived from."""
    path: str
    "# 'request' | 'response'"
    side: str
    method: str
    code: str | None
    content_type: str

def _leaf_segment_matches(path_segment: str, pattern: str) -> bool:
    """True if the last dotted package segment fully matches `pattern` (e.g. a
    path ending in '/responses' already says 'response' -- no need for the
    fixed '.response.' prefix on top, avoiding a '...response.responses...' stutter).
    """
    leaf = path_segment.rsplit('.', 1)[-1] if path_segment else ''
    return bool(re.fullmatch(pattern, leaf, re.IGNORECASE))

def anonymous_package(node, base_package: str) -> str | None:
    """Kind-based package for a shared anonymous node, or None if it has no class."""
    bucket = ANONYMOUS_KIND_PACKAGE.get(type(node))
    return f'{base_package}.{bucket}' if bucket else None

def site_package(site: Site, base_package: str, methods_by_path: dict) -> str:
    """The request/response package a Site maps to."""
    path_segment = '.'.join(path_to_package_segments(site.path))
    if site.side == 'request':
        prefix = '' if _leaf_segment_matches(path_segment, 'requests?') else 'request.'
        return f'{base_package}.{prefix}{path_segment}.{site.method.lower()}.{site.content_type}'
    method_segment = f'.{site.method.lower()}' if len(methods_by_path.get(site.path, ())) > 1 else ''
    prefix = '' if _leaf_segment_matches(path_segment, 'responses?') else 'response.'
    return f'{base_package}.{prefix}{path_segment}{method_segment}.code{site.code}.{site.content_type}'

def response_root_package(path: str, method: str, base_package: str, methods_by_path: dict) -> str:
    """Package of a ResponseNode root (no code/content-type segment)."""
    path_segment = '.'.join(path_to_package_segments(path))
    method_segment = f'.{method.lower()}' if len(methods_by_path.get(path, ())) > 1 else ''
    prefix = '' if _leaf_segment_matches(path_segment, 'responses?') else 'response.'
    return f'{base_package}.{prefix}{path_segment}{method_segment}'

def collect_methods_by_path(identified_model) -> dict:
    """path -> set of HTTP methods declared for it (used to decide whether the
    method segment is needed to keep a response package unambiguous)."""
    methods_by_path: dict = defaultdict(set)
    for operation_model in identified_model.operations:
        methods_by_path[operation_model.operation.path].add(operation_model.operation.method)
    return methods_by_path