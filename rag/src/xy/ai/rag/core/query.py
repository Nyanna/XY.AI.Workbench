"""Schwach typisiertes, dynamisches Query-Objekt der RAG Engine."""
from __future__ import annotations
from typing import Any, Iterator, Mapping

class Query(Mapping[str, Any]):
    """Dynamisches, schwach typisiertes Query-Objekt.

    Layer entscheiden selbst, auf welche Felder sie reagieren: durch
    Vorhandensein eines Feldes (``query.has("include")``) oder durch
    Inspektion des gesamten Objekts (``query.inspect()``).
    """

    def __init__(self, fields: Mapping[str, Any] | None=None, **kwargs: Any) -> None:
        self._fields: dict[str, Any] = dict(fields or {})
        self._fields.update(kwargs)

    def has(self, field: str) -> bool:
        return field in self._fields and self._fields[field] is not None

    def get(self, field: str, default: Any=None) -> Any:
        return self._fields.get(field, default)

    def inspect(self) -> dict[str, Any]:
        """Vollständige Kopie der Felder zur freien Analyse durch Layer."""
        return dict(self._fields)

    def with_fields(self, **overrides: Any) -> 'Query':
        merged = dict(self._fields)
        merged.update(overrides)
        return Query(merged)

    def __getattr__(self, name: str) -> Any:
        try:
            return self._fields[name]
        except KeyError as exc:
            raise AttributeError(name) from exc

    def __getitem__(self, key: str) -> Any:
        return self._fields[key]

    def __iter__(self) -> Iterator[str]:
        return iter(self._fields)

    def __len__(self) -> int:
        return len(self._fields)

    def __repr__(self) -> str:
        return f'Query({self._fields!r})'