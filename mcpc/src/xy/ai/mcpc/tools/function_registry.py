"""Registry for plain Python functions/bound methods exposed to the
``tool_search`` / ``tool_usage`` / ``tool_call`` family.

Complements the classic MCP :class:`~xy.ai.mcpc.tools.tool_registry.ToolRegistry`:
entries here are never advertised via ``tools/list``. They are ordinary
Python callables (module-level functions or bound methods on a live
instance) that ``tool_search`` can find by keyword, ``tool_usage`` can
introspect (signature, docstring, referenced project-local types), and
``tool_call`` can inject by id into its sandboxed script namespace.
"""
import inspect
import logging
from dataclasses import dataclass
from typing import Any, Callable
__all__ = ['FunctionEntry', 'FunctionRegistry']

logger = logging.getLogger("xy.ai.mcpc.control")

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

    def register(self, func: Callable[..., Any]) -> str:
        """Register *func* (a function or a bound method) under *id*.
        """
        entry_id = getattr(func, '__qualname__', None) or getattr(func, '__name__')
        existing = self._entries.get(entry_id)
        if existing is not None and existing.func is not func:
            raise ValueError(f'Function already registered under id: {entry_id}')
        self._entries[entry_id] = FunctionEntry(id=entry_id, func=func)
        logger.debug('Registered function: %s', entry_id)
        return entry_id

    def get(self, name: str) -> FunctionEntry | None:
        return self._entries.get(name)

    def all(self) -> list[FunctionEntry]:
        return list(self._entries.values())

    def ids(self) -> list[str]:
        return list(self._entries)