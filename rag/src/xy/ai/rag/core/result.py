"""Gemeinsames, schwach typisiertes Result-Objekt der RAG Engine.

Ein ResultSet besteht aus ResultEntry Objekten. Felder sind frei - ein
Layer kann einen Dateinamen, eine AST-Knoten-ID, Zeilennummern oder nur
eine Zusammenfassung ablegen. Aggregation passiert implizit: Layer lesen
und schreiben auf gemeinsamen Feldnamen (z.B. "path", "line", "span").
"""
from __future__ import annotations
import itertools
import threading
from typing import Any, Callable, Iterable, Iterator
_id_counter = itertools.count(1)

class ResultEntry:
    """Ein einzelner, schwach typisierter Eintrag im ResultSet."""
    __slots__ = ('id', 'fields', 'signals', '_lock')

    def __init__(self, entry_id: str | None=None, fields: dict[str, Any] | None=None, signals: Iterable[str] | None=None) -> None:
        self.id = entry_id or f'e{next(_id_counter)}'
        self.fields: dict[str, Any] = dict(fields or {})
        self.signals: list[str] = list(signals or [])
        self._lock = threading.Lock()

    def merge(self, fields: dict[str, Any] | None=None, signals: Iterable[str] | None=None) -> None:
        """Erweitert/überschreibt Felder und ergänzt Signale (thread-sicher).

        Ein Enrichment-Layer ruft dies auf einem bereits bestehenden Eintrag
        auf, z.B. um "line" durch einen Textausschnitt zu ersetzen oder eine
        AST-Outline/FQN hinzuzufügen.
        """
        with self._lock:
            if fields:
                self.fields.update(fields)
            for s in signals or ():
                if s not in self.signals:
                    self.signals.append(s)

    def has(self, field: str) -> bool:
        return field in self.fields and self.fields[field] is not None

    def get(self, field: str, default: Any=None) -> Any:
        return self.fields.get(field, default)

    def to_dict(self) -> dict[str, Any]:
        return {'id': self.id, 'signals': list(self.signals), **self.fields}

    def __repr__(self) -> str:
        return f'ResultEntry({self.to_dict()!r})'

class ResultSet:
    """Thread-sichere, gemeinsam genutzte Menge von ResultEntry Objekten.

    Der erste Layer in der Topologie - auch wenn er ein reiner Enrichment-
    Layer ist - legt das ResultSet implizit an. Nachfolgende Layer erzeugen
    neue Einträge (generate) oder reichern bestehende an (enrich), indem sie
    auf gemeinsamen Feldern operieren.
    """

    def __init__(self) -> None:
        self._entries: dict[str, ResultEntry] = {}
        self._order: list[str] = []
        self._lock = threading.RLock()

    def add(self, entry: ResultEntry) -> ResultEntry:
        with self._lock:
            if entry.id not in self._entries:
                self._order.append(entry.id)
            self._entries[entry.id] = entry
        return entry

    def get(self, entry_id: str) -> ResultEntry | None:
        with self._lock:
            return self._entries.get(entry_id)

    def entries(self) -> list[ResultEntry]:
        with self._lock:
            return [self._entries[i] for i in self._order]

    def find(self, **criteria: Any) -> list[ResultEntry]:
        """Findet Einträge, deren Felder mit allen Kriterien übereinstimmen.

        Die implizite Aggregation der Layer basiert auf diesem Mechanismus:
        ein Layer sucht z.B. alle Einträge mit einem bestimmten "path" und
        reichert sie um weitere Felder/Signale an.
        """
        with self._lock:
            return [e for e in self._ordered() if all((e.get(k) == v for k, v in criteria.items()))]

    def _ordered(self) -> Iterator[ResultEntry]:
        return (self._entries[i] for i in self._order)

    def sort(self, key: Callable[[ResultEntry], Any]) -> None:
        """Stabile Sortierung (kein numerisches Ranking) für Postprocessing-Layer."""
        with self._lock:
            self._order.sort(key=lambda eid: key(self._entries[eid]))

    def __len__(self) -> int:
        with self._lock:
            return len(self._order)

    def __iter__(self) -> Iterator[ResultEntry]:
        return iter(self.entries())