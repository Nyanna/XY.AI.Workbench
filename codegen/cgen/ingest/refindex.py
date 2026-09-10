"""Reference index over components.schemas and components.responses.

Keys are the full JSON-pointer style refs (e.g. '#/components/schemas/Foo').
The original YAML key is kept as the canonical name; nodes are stored raw,
never expanded, so callers can treat $ref as an atomic id token.
"""

SCHEMA_REF_PREFIX = "#/components/schemas/"
RESPONSE_REF_PREFIX = "#/components/responses/"


class RefIndex:
    """Maps a component ref string to its raw (unexpanded) node."""

    def __init__(self):
        self._nodes: dict[str, dict] = {}

    def add(self, ref: str, node: dict) -> None:
        self._nodes[ref] = node

    def get(self, ref: str) -> dict:
        try:
            return self._nodes[ref]
        except KeyError:
            raise KeyError(f"unresolved $ref: {ref}") from None

    def __contains__(self, ref: str) -> bool:
        return ref in self._nodes

    def schema_refs(self) -> list[str]:
        return [ref for ref in self._nodes if ref.startswith(SCHEMA_REF_PREFIX)]


def build_ref_index(document: dict) -> RefIndex:
    """Index components.schemas and components.responses under their original keys."""
    index = RefIndex()
    components = document.get("components") or {}
    for name, node in (components.get("schemas") or {}).items():
        index.add(SCHEMA_REF_PREFIX + name, node)
    for name, node in (components.get("responses") or {}).items():
        index.add(RESPONSE_REF_PREFIX + name, node)
    return index
