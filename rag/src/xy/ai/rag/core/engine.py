"""Orchestrator (Engine) der RAG Engine.

Baut pro Query die Liste der aktiven Layer, ermittelt deren Topologie und
führt sie sequentiell oder im Fork/Join-Modell aus. Verwaltet zusätzlich
den globalen Kontrollfluss für Hintergrundaktivitäten der Layer.
"""
from __future__ import annotations
import asyncio
import enum
from pathlib import Path
from xy.ai.rag.core.layer import BackgroundContext, Layer, LayerContext, LayerStatus
from xy.ai.rag.core.persistence import PersistenceManager
from xy.ai.rag.core.query import Query
from xy.ai.rag.core.registry import LayerRegistry
from xy.ai.rag.core.result import ResultSet
from xy.ai.rag.core.topology import build_levels

class ExecutionMode(str, enum.Enum):
    SEQUENTIAL = 'sequential'
    PARALLEL = 'parallel'

class Engine:
    """Verbindet Registry, Persistenz und Topologie-Ausführung.

    Wird sowohl als Lib eingebunden (``Engine(...).run_query(...)``) als
    auch von der CLI on-demand instanziiert.
    """

    def __init__(self, registry: LayerRegistry, root: Path | str | None=None, mode: ExecutionMode=ExecutionMode.PARALLEL) -> None:
        self.registry = registry
        self.persistence = PersistenceManager(root)
        self.mode = mode
        self._cancel_events: dict[str, asyncio.Event] = {}
        self._background_tasks: dict[str, asyncio.Task] = {}
    '# ---- Query-Channel Kontrollfluss -----------------------------------'

    def active_layers(self, query: Query) -> list[Layer]:
        return [layer for layer in self.registry if layer.applies(query)]

    async def run_query(self, query: Query) -> tuple[ResultSet, list[LayerStatus]]:
        """Führt eine Query gegen alle aktivierten Layer aus.

        Der erste ausgeführte Layer legt das ResultSet implizit an, indem
        er auf die hier erzeugte, leere Instanz schreibt - auch wenn er
        selbst nur anreichert.
        """
        result_set = ResultSet()
        layers = self.active_layers(query)
        levels = build_levels(layers)
        statuses: list[LayerStatus] = []
        for level in levels:
            if self.mode is ExecutionMode.PARALLEL and len(level) > 1:
                results = await asyncio.gather(*(self._run_layer(layer, query, result_set) for layer in level))
                statuses.extend(results)
            else:
                for layer in level:
                    statuses.append(await self._run_layer(layer, query, result_set))
        return (result_set, statuses)

    async def _run_layer(self, layer: Layer, query: Query, result_set: ResultSet) -> LayerStatus:
        ctx = LayerContext(
            query=query,
            storage=self.persistence.layer_storage(
                layer.id),
            shared_index=self.persistence.shared_index)
        return await layer.run(query, result_set, ctx)
    '# ---- Globaler Hintergrund-Kontrollfluss ----------------------------'

    def start_background(self) -> None:
        """Startet die Hintergrundaktivität jedes Layers (einmalig, global).

        Jeder Layer entscheidet selbst anhand seiner Implementierung, ob er
        tatsächlich aktiv wird; Default ist keine Aktivität.
        """
        for layer in self.registry:
            if layer.id in self._background_tasks:
                continue
            cancel = asyncio.Event()
            self._cancel_events[layer.id] = cancel
            ctx = BackgroundContext(storage=self.persistence.layer_storage(
                layer.id), shared_index=self.persistence.shared_index)
            self._background_tasks[layer.id] = asyncio.create_task(layer.background(ctx, cancel), name=f'bg:{layer.id}')

    async def wait_background(self) -> None:
        """Wartet, bis alle Hintergrundaktivitäten beendet sind.

        Ein CLI-Prozess ruft dies nach Rückkehr aller Queries auf, bevor er
        sich beendet.
        """
        if self._background_tasks:
            await asyncio.gather(*self._background_tasks.values())

    def cancel_background(self) -> None:
        for event in self._cancel_events.values():
            event.set()

    async def shutdown(self, cancel: bool=False) -> None:
        if cancel:
            self.cancel_background()
        await self.wait_background()