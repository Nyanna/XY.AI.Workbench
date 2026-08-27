"""Registry for plain Python functions/bound methods exposed to the
``tool_search`` / ``tool_usage`` / ``tool_call`` family.

Complements the classic MCP :class:`~xy.ai.mcpc.tools.registry.ToolRegistry`:
entries here are never advertised via ``tools/list``. They are ordinary
Python callables (module-level functions or bound methods on a live
instance) that ``tool_search`` can find by keyword, ``tool_usage`` can
introspect (signature, docstring, referenced project-local types), and
``tool_call`` can inject by id into its sandboxed script namespace.
"""
from __future__ import annotations
import inspect
from dataclasses import dataclass
from typing import Any, Callable
__all__ = ['FunctionEntry', 'FunctionRegistry']

@dataclass(slots=True)
class FunctionEntry:
    """A single registered callable and the id it was published under."""
    id: str
    func: Callable[..., Any]

    @property
    def name(self) -> str:
        return getattr(self.func, '__name__', self.id)

    @property
    def doc(self) -> str:
        return inspect.getdoc(self.func) or ''

class FunctionRegistry:
    """Process-wide registry of functions/bound methods usable as tools."""

    def __init__(self) -> None:
        self._entries: dict[str, FunctionEntry] = {}

    def register(self, func: Callable[..., Any], *, id: str | None=None) -> str:
        """Register *func* (a function or a bound method) under *id*.

        *id* defaults to ``func.__qualname__`` (falling back to
        ``func.__name__``), so a bound method is published as
        ``"ClassName.method_name"`` while a module-level function keeps its
        plain name. Re-registering the same callable under the same id is a
        no-op; registering a *different* callable under an id already in use
        raises.
        """
        entry_id = id or getattr(func, '__qualname__', None) or getattr(func, '__name__')
        existing = self._entries.get(entry_id)
        if existing is not None and existing.func is not func:
            raise ValueError(f'Function already registered under id: {entry_id}')
        self._entries[entry_id] = FunctionEntry(id=entry_id, func=func)
        return entry_id

    def get(self, id: str) -> 'FunctionEntry | None':
        return self._entries.get(id)

    def all(self) -> list[FunctionEntry]:
        return list(self._entries.values())

    def ids(self) -> list[str]:
        return list(self._entries)