"""Generischer Filter fuer OpenAPI-YAML-Specs.

Wendet eine Liste providerunabhaengiger Operationen auf eine Basis-Spec an,
um daraus eine reduzierte Spec fuer die Client-Generierung zu erzeugen.

Aufruf:
    filter_spec.py <filter-config.yaml> <output-spec.yaml>

Filter-Config-Format (YAML):
    base: openai.yaml            # relativ zu diesem Script-Verzeichnis
    ops:
      - op: keep-paths
        values: ["/responses", "/responses/{response_id}"]
      - op: delete-path
        values: ["/v1/messages/batches/{message_batch_id}?beta=true"]n      - op: delete
        pointer: "/components/schemas/*/properties/summary"
      - op: set
        pointer: "/components/schemas/ReasoningEffort/anyOf/0/enum"
        value: [low, high, max]
      - op: prune-orphan-schemas

Pointer-Segmente sind JSON-Pointer-artig ("/" getrennt), das Segment "*"
matcht dabei jeden Schluessel/Index auf dieser Ebene.
"""
import sys
import re
from pathlib import Path
import yaml
REF_RE = re.compile('^#/components/([a-zA-Z]+)/(.+)$')
COMPONENT_SECTIONS = ('schemas', 'parameters', 'requestBodies', 'responses', 'headers', 'examples')

def _split_pointer(pointer: str) -> list[str]:
    return [seg.replace('~1', '/').replace('~0', '~') for seg in pointer.strip('/').split('/') if seg != '']

def _walk_apply(node, segments: list[str], op: str, value=None) -> None:
    if not segments:
        return
    key, rest = (segments[0], segments[1:])
    if not rest:
        _apply_leaf(node, key, op, value)
        return
    if key == '*':
        if isinstance(node, dict):
            for k in list(node.keys()):
                _walk_apply(node[k], rest, op, value)
        elif isinstance(node, list):
            for item in node:
                _walk_apply(item, rest, op, value)
        return
    if isinstance(node, dict) and key in node:
        _walk_apply(node[key], rest, op, value)
    elif isinstance(node, list):
        try:
            _walk_apply(node[int(key)], rest, op, value)
        except (ValueError, IndexError):
            pass

def _apply_leaf(node, key: str, op: str, value) -> None:
    if key == '*':
        if isinstance(node, dict):
            for k in list(node.keys()):
                _apply_leaf(node, k, op, value)
        elif isinstance(node, list):
            if op == 'delete':
                node.clear()
            else:
                for i in range(len(node)):
                    node[i] = value
        return
    if isinstance(node, list):
        try:
            idx = int(key)
        except ValueError:
            return
        if op == 'delete':
            if 0 <= idx < len(node):
                del node[idx]
        elif op == 'set':
            if 0 <= idx < len(node):
                node[idx] = value
        return
    if not isinstance(node, dict):
        return
    if op == 'delete':
        node.pop(key, None)
    elif op == 'set':
        node[key] = value

def op_delete(spec: dict, pointer: str) -> None:
    _walk_apply(spec, _split_pointer(pointer), 'delete')

def op_set(spec: dict, pointer: str, value) -> None:
    _walk_apply(spec, _split_pointer(pointer), 'set', value)

def _resolve_pointer(node, segments: list[str]):
    for seg in segments:
        if isinstance(node, dict):
            node = node.get(seg)
        elif isinstance(node, list):
            try:
                node = node[int(seg)]
            except (ValueError, IndexError):
                return None
        else:
            return None
    return node

def op_delete_refs(spec: dict, pointer: str, values: list[str]) -> None:
    """Entfernt Eintraege aus einer Liste (z.B. oneOf/anyOf), deren $ref-Ziel
    (letztes Pointer-Segment) in `values` enthalten ist."""
    node = _resolve_pointer(spec, _split_pointer(pointer))
    if not isinstance(node, list):
        return
    node[:] = [item for item in node if not (isinstance(item, dict) and isinstance(
        item.get('$ref'), str) and (item['$ref'].rsplit('/', 1)[-1] in values))]
_ANNOTATION_KEYS = {'title', 'description', 'default', 'example', 'examples', 'deprecated', 'readOnly', 'writeOnly'}

def _prune_refs(node, target: str) -> bool:
    """Entfernt rekursiv jedes Vorkommen von {'$ref': target} aus node.

    Dict-Keys, deren Wert komplett verschwindet (leer wird), werden beim
    Elternknoten mitentfernt; Listen-Eintraege, die nur noch aus 'type: null'
    bestehen (typisches anyOf-Optional-Pattern), gelten ebenfalls als leer.
    Bleibt bei anyOf/oneOf nur 'null' uebrig, wird es zu 'type: null' im
    Elternschema zusammengezogen, damit das Feld typisiert bleibt.
    Gibt True zurueck, wenn der Knoten selbst beim Aufrufer entfernt werden soll.
    """
    if isinstance(node, dict):
        if node.get('$ref') == target:
            return True
        had_keys = bool(node)
        removed_any = False
        for key in list(node.keys()):
            child = node[key]
            if _prune_refs(child, target):
                del node[key]
                removed_any = True
                if key in ('anyOf', 'oneOf') and isinstance(child, list) and child:
                    node.setdefault('type', 'null')
        if removed_any and node and (set(node) <= _ANNOTATION_KEYS):
            return True
        return had_keys and (not node)
    if isinstance(node, list):
        node[:] = [item for item in node if not _prune_refs(item, target)]
        meaningful = [item for item in node if not (isinstance(item, dict) and item.get('type') == 'null')]
        return len(meaningful) == 0
    return False

def op_strip_refs(spec: dict, values: list[str]) -> None:
    """Entfernt alle eingehenden Referenzen auf die genannten Schemas.

    Die Schemas selbst werden nicht geloescht - das erledigt im Anschluss
    'prune-orphan-schemas', sobald sie unerreichbar geworden sind.
    """
    for name in values:
        target = f'#/components/schemas/{name}'
        _prune_refs(spec.get('paths', {}), target)
        _prune_refs(spec.get('components', {}), target)
    removed = {f'#/components/schemas/{name}' for name in values}
    _prune_discriminator_mappings(spec, lambda ref: ref not in removed)

def op_keep_paths(spec: dict, values: list[str]) -> None:
    paths = spec.get('paths', {})
    spec['paths'] = {k: v for k, v in paths.items() if k in values}

def op_delete_paths(spec: dict, values: list[str]) -> None:
    paths = spec.get('paths', {})
    spec['paths'] = {k: v for k, v in paths.items() if k not in values}

def _ref_exists(components: dict, ref: str) -> bool:
    match = REF_RE.match(ref)
    return bool(match) and match.group(2) in components.get(match.group(1), {})

def _prune_discriminator_mappings(node, keep) -> None:
    """Removes discriminator mapping entries whose target ref fails `keep`.

    Mapping values are plain strings, so they are invisible to $ref handling.
    Bare schema names are normalized to '#/components/schemas/<name>'.
    A discriminator whose mapping becomes empty is removed entirely.
    """
    if isinstance(node, dict):
        discriminator = node.get('discriminator')
        if isinstance(discriminator, dict) and isinstance(discriminator.get('mapping'), dict):
            mapping = discriminator['mapping']
            for key, target in list(mapping.items()):
                if not isinstance(target, str):
                    continue
                ref = target if target.startswith('#/') else f'#/components/schemas/{target}'
                if not keep(ref):
                    del mapping[key]
            if not mapping:
                del node['discriminator']
        for value in node.values():
            _prune_discriminator_mappings(value, keep)
    elif isinstance(node, list):
        for item in node:
            _prune_discriminator_mappings(item, keep)

def _collect_refs(node, refs: set[str]) -> None:
    if isinstance(node, dict):
        ref = node.get('$ref')
        if isinstance(ref, str):
            refs.add(ref)
        for v in node.values():
            _collect_refs(v, refs)
    elif isinstance(node, list):
        for item in node:
            _collect_refs(item, refs)

def _prune_empty_properties(node) -> None:
    """Removes optional properties that only carry annotations (e.g. just a title)."""
    if isinstance(node, dict):
        properties = node.get('properties')
        if isinstance(properties, dict):
            required = node.get('required')
            required = required if isinstance(required, list) else []
            for name, prop in list(properties.items()):
                if isinstance(prop, dict) and name not in required and (set(prop) <= _ANNOTATION_KEYS):
                    del properties[name]
        for value in node.values():
            _prune_empty_properties(value)
    elif isinstance(node, list):
        for item in node:
            _prune_empty_properties(item)

def op_prune_orphan_schemas(spec: dict) -> None:
    components = spec.get('components', {})
    _prune_empty_properties(spec)
    used: dict[str, set[str]] = {section: set() for section in COMPONENT_SECTIONS}
    pending: set[str] = set()
    _collect_refs(spec.get('paths', {}), pending)
    _collect_refs(spec.get('security', []), pending)
    seen: set[str] = set()
    while pending:
        ref = pending.pop()
        if ref in seen:
            continue
        seen.add(ref)
        match = REF_RE.match(ref)
        if not match:
            continue
        section, name = (match.group(1), match.group(2))
        if section not in used:
            continue
        used[section].add(name)
        node = components.get(section, {}).get(name)
        if node is not None:
            more: set[str] = set()
            _collect_refs(node, more)
            pending |= more - seen
    for section in COMPONENT_SECTIONS:
        if section in components:
            components[section] = {name: schema for name,
                                   schema in components[section].items() if name in used[section]}
        spec['components'] = components
    _prune_discriminator_mappings(spec, lambda ref: _ref_exists(components, ref))

def _strip_vendor_extensions(node) -> None:
    if isinstance(node, dict):
        for key in [k for k in node if isinstance(k, str) and k.startswith('x-')]:
            del node[key]
        for v in node.values():
            _strip_vendor_extensions(v)
    elif isinstance(node, list):
        for item in node:
            _strip_vendor_extensions(item)

def op_strip_vendor_extensions(spec: dict) -> None:
    _strip_vendor_extensions(spec)

def _strip_operation_tags(node) -> None:
    if isinstance(node, dict):
        node.pop('tags', None)

def op_strip_tags(spec: dict) -> None:
    spec.pop('tags', None)
    for path_item in spec.get('paths', {}).values():
        if isinstance(path_item, dict):
            for op in path_item.values():
                _strip_operation_tags(op)
OPS = {
    'keep-paths': lambda spec,
    o: op_keep_paths(
        spec,
        o['values']),
    'delete-path': lambda spec,
    o: op_delete_paths(
        spec,
        o['values']),
    'delete': lambda spec,
    o: op_delete(
        spec,
        o['pointer']),
    'set': lambda spec,
    o: op_set(
        spec,
        o['pointer'],
        o['value']),
    'prune-orphan-schemas': lambda spec,
    o: op_prune_orphan_schemas(spec),
    'strip-tags': lambda spec,
    o: op_strip_tags(spec),
    'strip-vendor-extensions': lambda spec,
    o: op_strip_vendor_extensions(spec),
    'delete-refs': lambda spec,
    o: op_delete_refs(
        spec,
        o['pointer'],
        o['values']),
    'strip-refs': lambda spec,
    o: op_strip_refs(
        spec,
        o['values'])}

def filter_spec(config_path: Path, output_path: Path) -> None:
    config = yaml.safe_load(config_path.read_text())
    base_path = Path(__file__).parent / config['base']
    spec = yaml.safe_load(base_path.read_text())
    for op_conf in config.get('ops', []):
        op_name = op_conf['op']
        handler = OPS.get(op_name)
        if handler is None:
            raise ValueError(f'Unbekannte Filter-Operation: {op_name}')
        handler(spec, op_conf)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(yaml.safe_dump(spec, sort_keys=False, allow_unicode=True))

def main() -> None:
    if len(sys.argv) != 3:
        print('Usage: filter_spec.py <filter-config.yaml> <output-spec.yaml>', file=sys.stderr)
        sys.exit(1)
    filter_spec(Path(sys.argv[1]), Path(sys.argv[2]))
if __name__ == '__main__':
    main()