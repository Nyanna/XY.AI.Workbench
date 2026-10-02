"""Registry für Layer-Implementierungen."""
from __future__ import annotations
from typing import Iterable, Iterator
from xy.ai.rag.core.layer import Layer

class LayerRegistry:
    """Verwaltet alle verfügbaren Layer-Instanzen, adressiert über ihre ID."""

    def __init__(self, layers: Iterable[Layer] | None=None) -> None:
        self._layers: dict[str, Layer] = {}
        for layer in layers or ():
            self.register(layer)

    def register(self, layer: Layer) -> None:
        if not getattr(layer, 'id', None):
            raise ValueError('Layer benötigt eine eindeutige, nicht-leere id')
        if layer.id in self._layers:
            raise ValueError(f'Layer-ID bereits registriert: {layer.id}')
        self._layers[layer.id] = layer

    def unregister(self, layer_id: str) -> None:
        self._layers.pop(layer_id, None)

    def get(self, layer_id: str) -> Layer | None:
        return self._layers.get(layer_id)

    def all(self) -> list[Layer]:
        return list(self._layers.values())

    def __iter__(self) -> Iterator[Layer]:
        return iter(self._layers.values())

    def __len__(self) -> int:
        return len(self._layers)

    def __contains__(self, layer_id: str) -> bool:
        return layer_id in self._layers