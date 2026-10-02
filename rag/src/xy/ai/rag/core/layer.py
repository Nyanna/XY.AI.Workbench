"""Layer-Contract der RAG Engine.

Ein Layer ist durch eine eindeutige ID identifiziert und wird über die
LayerRegistry verwaltet. Er operiert auf einem gemeinsamen ResultSet und
entscheidet selbst, ob und wie er auf eine Query reagiert.
"""
from __future__ import annotations
import abc
import asyncio
import enum
from dataclasses import dataclass, field
from typing import Any
from xy.ai.rag.core.persistence import LayerStorage, SharedIndex
from xy.ai.rag.core.query import Query
from xy.ai.rag.core.result import ResultSet

class LayerStage(str, enum.Enum):
    """Grobe Ordnungs-Hinweise für die Topologie; ersetzen keine expliziten Abhängigkeiten.

    GENERATE: legt neue Einträge an, autonom oder anhand der Query.
    ENRICH: beobachtet/erweitert bestehende Einträge (z.B. semantisch, AST).
    POSTPROCESS: wartet auf ein vollständiges ResultSet (z.B. Sortierung, Cache-Reduktion).
    """
    GENERATE = 'generate'
    ENRICH = 'enrich'
    POSTPROCESS = 'postprocess'

@dataclass(frozen=True)
class LayerStatus:
    """Protokoll eines Layer-Laufs: ran/aborted/skipped unterscheidbar von "kein Ergebnis"."""
    layer_id: str
    stage: LayerStage
    ran: bool
    skipped: bool = False
    aborted: bool = False
    contributions: int = 0
    detail: dict[str, Any] = field(default_factory=dict)

@dataclass
class LayerContext:
    """Von der Engine pro Query bereitgestellter Kontext für einen Layer."""
    query: Query
    storage: LayerStorage
    shared_index: SharedIndex

@dataclass
class BackgroundContext:
    """Von der Engine global bereitgestellter Kontext für Hintergrundaktivitäten."""
    storage: LayerStorage
    shared_index: SharedIndex

class Layer(abc.ABC):
    """Basisklasse/Protokoll für einen RAG-Layer.

    Zwei unabhängige Kontrollflüsse:
      1. ``run``        - gekoppelt an den Query-Channel (pro Suchanfrage).
      2. ``background``  - globaler Kontrollfluss, vom Layer selbst gesteuert
                           (z.B. Indexaufbau), unabhängig von einzelnen Queries.
    """
    id: str
    stage: LayerStage = LayerStage.ENRICH
    '# IDs anderer Layer, auf deren Abschluss (im aktuellen Query-Lauf) gewartet wird.'
    depends_on: frozenset[str] = frozenset()

    def applies(self, query: Query) -> bool:
        """Standard: Aktivierung durch Vorhandensein der eigenen ID als Feld.

        Layer können dies überschreiben, um die gesamte Query zu inspizieren.
        """
        return query.has(self.id)

    @abc.abstractmethod
    async def run(self, query: Query, result_set: ResultSet, ctx: LayerContext) -> LayerStatus:
        """Verarbeitet eine Query gegen das gemeinsame ResultSet."""
        raise NotImplementedError

    async def background(self, ctx: BackgroundContext, cancel: asyncio.Event) -> None:
        """Optionale Hintergrundaktivität (z.B. Index-/Cache-Aufbau).

        Default: keine Hintergrundaktivität. Layer entscheiden selbst, ob
        und wann sie basierend auf vorherigen Queries aktiv werden.
        """
        return None