"""Item outcome building blocks shared by the node-addressed ``ast_*`` mutation tools."""
from __future__ import annotations
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry
from xy.ai.mcpc.tools.results import ItemError
__all__ = ['node_key', 'edit_payload', 'NodeError']

def node_key(path: str, id: str | None) -> dict[str, Any]:
    return {'path': path} if id is None else {'path': path, 'id': id}

def edit_payload(*, new_id: str | None=None, imports: ImportOutcome | None=None, **extra: Any) -> dict[str, Any]:
    out: dict[str, Any] = {k: v for k, v in extra.items() if v is not None}
    if new_id is not None:
        out['new_id'] = new_id
    if (entry := import_entry(imports)) is not None:
        out['imports'] = entry
    return out

@dataclass(frozen=True)
class NodeError(ItemError):
    """Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as given."""
    path: str
    id: str | None

    def key(self) -> dict[str, Any]:
        return node_key(self.path, self.id)