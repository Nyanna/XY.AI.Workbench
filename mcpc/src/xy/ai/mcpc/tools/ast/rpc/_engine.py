"""Generic :class:`Engine` back-end for any parser exposed via the RPC/PIC
HTTP protocol described by ``ast-engines/openapi.yaml``.

A single :class:`RpcEngine` instance, given an :class:`~xy.ai.mcpc.tools.ast
.rpc._process.RpcProcess`, implements every :class:`Engine` operation by
calling the openapi-generated :class:`AppendInfoNodesValidateClientImpl`
against that process's (lazily started) base URL. Per-engine subclasses only
need to supply their :class:`RpcProcess` and static metadata (``name``,
``validates_syntax``).
"""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.ast.openapi.AppendInfoNodesValidateClientImpl import AppendInfoNodesValidateClientImpl
from xy.ai.mcpc.ast.openapi.components.CodeRequest import CodeRequest
from xy.ai.mcpc.ast.openapi.components.Error import Error
from xy.ai.mcpc.ast.openapi.components.SourceRequest import SourceRequest
from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest import InsertRequest
from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.PositionEnum import PositionEnum
from xy.ai.mcpc.ast.openapi.request.nodes.post.json.LocateRequest import LocateRequest
from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, Engine, Located, Tree, check_no_control_chars
from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
__all__ = ['RpcEngine']

@dataclass
class _RpcNode:
    """One node as reported by ``POST /nodes``.

    Carries everything :meth:`RpcEngine.signature`/``docstring``/``node_code``
    need without a further round-trip: signature/docstring come straight from
    the server, and code -- unless eagerly included -- is sliced from the
    already-known ``tree.source`` by line range.
    """
    tree: Tree
    lineno: int
    end_lineno: int
    signature_text: str | None
    docstring_text: str | None
    code: str | None

class RpcEngine(Engine):
    """One RPC/PIC engine process exposed through the common :class:`Engine` API.

    Structure, signatures and docstrings are computed remotely; mutations
    send the current full source and node id and replace ``tree.source``
    with the server's answer, mirroring how :class:`TreeSitterEngine` edits
    ``Tree.source`` in place so :meth:`serialize` just returns it.
    """

    def __init__(self, symbol: str, process: RpcProcess) -> None:
        self.symbol = symbol
        self.name = symbol
        self._process = process
        self._client: AppendInfoNodesValidateClientImpl | None = None
        '#: node_type -> is_definition, learned from the last ``locate_all`` calls'
        '#: (Engine.is_definition only takes a type name, not a specific node).'
        self._is_definition: dict[str, bool] = {}

    def _rpc(self) -> AppendInfoNodesValidateClientImpl:
        if self._client is None:
            self._client = AppendInfoNodesValidateClientImpl(self._process.base_url())
        return self._client

    @staticmethod
    def _unwrap(response: Any) -> Any:
        """Return a 200 response's typed body, or raise on any other status."""
        code = response.status_code
        getter = getattr(response, f'get_code_{code}', None)
        view = getter() if getter else None
        payload = view.get_Json() if view is not None else None
        if code == '200':
            return payload
        error = payload if payload is not None else Error({})
        message = error.get_Message() or f'AST engine request failed (HTTP {code}).'
        if code == '409':
            candidates = error.get_Candidates()
            items = [candidates.get(i) for i in range(len(candidates))] if candidates is not None else []
            raise AstAmbiguous(message, items)
        raise AstError(message)
    '# -- Engine ------------------------------------------------------------'

    def parse(self, source: str, path: Path | None=None) -> Tree:
        error = self.validate(source)
        if error:
            raise AstError(error)
        return Tree(self, None, source, path)

    def empty_tree(self, path: Path | None=None) -> Tree:
        return Tree(self, None, '', path)

    def serialize(self, tree: Tree) -> str:
        return tree.source

    def validate(self, source: str) -> str | None:
        request = SourceRequest()
        request.set_Source(source)
        payload = self._unwrap(self._rpc().validateSource(request))
        return payload.get_Error() if payload is not None else None

    def is_definition(self, node_type: str) -> bool:
        return self._is_definition.get(node_type, True)

    def locate_all(self, tree: Tree) -> list[Located]:
        request = LocateRequest()
        request.set_Source(tree.source)
        if tree.path is not None:
            request.set_Path(str(tree.path))
        request.set_IncludeCode(False)
        payload = self._unwrap(self._rpc().listNodes(request))
        nodes = payload.get_Nodes() if payload is not None else None
        count = len(nodes) if nodes is not None else 0
        results: list[Located] = []
        sibling_index: dict[str, int] = {}
        for i in range(count):
            node = nodes.get(i)
            node_id = node.get_Id()
            node_type = node.get_Type()
            self._is_definition[node_type] = bool(node.get_IsDefinition())
            parent_id = node_id.rsplit('.', 1)[0] if '.' in node_id else ''
            index = sibling_index.get(parent_id, 0)
            sibling_index[parent_id] = index + 1
            lineno = node.get_Lineno()
            end_lineno = node.get_EndLineno()
            results.append(
                Located(
                    tree=tree,
                    node=_RpcNode(
                        tree=tree,
                        lineno=lineno,
                        end_lineno=end_lineno,
                        signature_text=node.get_Signature(),
                        docstring_text=node.get_Docstring(),
                        code=node.get_Code()),
                    parent=None,
                    index=index,
                    node_id=node_id,
                    node_type=node_type,
                    name=node.get_Name(),
                    lineno=lineno,
                    end_lineno=end_lineno,
                    parent_type=node.get_ParentType(),
                    expandable=bool(
                        node.get_Expandable())))
        return results

    def signature(self, node: _RpcNode) -> str:
        return node.signature_text if node.signature_text else self.default_signature(node)

    def docstring(self, node: _RpcNode) -> str | None:
        return node.docstring_text

    def node_code(self, node: _RpcNode) -> str:
        if node.code is not None:
            return node.code
        lines = node.tree.source.splitlines(keepends=True)
        return ''.join(lines[node.lineno - 1:node.end_lineno])

    def replace(self, loc: Located, code: str) -> None:
        check_no_control_chars(code)
        request = CodeRequest()
        request.set_Source(loc.tree.source)
        if loc.tree.path is not None:
            request.set_Path(str(loc.tree.path))
        request.set_Code(code)
        payload = self._unwrap(self._rpc().replaceNode(loc.node_id, request))
        loc.tree.source = payload.get_Source()

    def insert(self, loc: Located, code: str, position: str) -> int:
        check_no_control_chars(code)
        request = InsertRequest()
        request.set_Source(loc.tree.source)
        if loc.tree.path is not None:
            request.set_Path(str(loc.tree.path))
        request.set_Code(code)
        request.set_Position(PositionEnum.BEFORE if position == 'before' else PositionEnum.AFTER)
        payload = self._unwrap(self._rpc().insertRelativeToNode(loc.node_id, request))
        loc.tree.source = payload.get_Source()
        return payload.get_UnitsInserted()

    def delete(self, loc: Located) -> None:
        request = SourceRequest()
        request.set_Source(loc.tree.source)
        if loc.tree.path is not None:
            request.set_Path(str(loc.tree.path))
        payload = self._unwrap(self._rpc().deleteNode(loc.node_id, request))
        loc.tree.source = payload.get_Source()

    def append(self, tree: Tree, code: str) -> int:
        check_no_control_chars(code)
        request = CodeRequest()
        request.set_Source(tree.source)
        if tree.path is not None:
            request.set_Path(str(tree.path))
        request.set_Code(code)
        payload = self._unwrap(self._rpc().appendTopLevel(request))
        tree.source = payload.get_Source()
        return payload.get_UnitsAppended()