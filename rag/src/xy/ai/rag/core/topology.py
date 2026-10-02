"""Dynamische Topologie (DAG) über die aktiven Layer einer Query.

Unterstützt sowohl rein sequentielle Ausführung als auch ein Fork/Join-
Modell für parallele Ausführung unabhängiger Layer. Die Topologie ergibt
sich pro Query neu aus den aktiven Layern (``Layer.applies``) und ihren
expliziten Abhängigkeiten (``Layer.depends_on``), ergänzt um die grobe
Stage-Ordnung (GENERATE vor ENRICH vor POSTPROCESS) als Default-Kante,
wenn keine expliziten Abhängigkeiten bestehen.
"""
from __future__ import annotations
from xy.ai.rag.core.layer import Layer, LayerStage
_STAGE_ORDER = {LayerStage.GENERATE: 0, LayerStage.ENRICH: 1, LayerStage.POSTPROCESS: 2}

class CyclicDependencyError(RuntimeError):
    pass

def build_levels(layers: list[Layer]) -> list[list[Layer]]:
    """Topologische Sortierung in Ebenen ("Fork/Join"-Gruppen).

    Jede Ebene enthält Layer, die parallel ausgeführt werden können, weil
    alle ihre Abhängigkeiten in vorherigen Ebenen bereits erfüllt sind.
    Für sequentielle Ausführung werden die Ebenen einfach nacheinander und
    innerhalb jeder Ebene einzeln verarbeitet.
    """
    by_id = {layer.id: layer for layer in layers}
    deps: dict[str, set[str]] = {}
    for layer in layers:
        d = {dep for dep in layer.depends_on if dep in by_id}
        d |= {other.id for other in layers if other.id !=
              layer.id and _STAGE_ORDER[other.stage] < _STAGE_ORDER[layer.stage]}
        deps[layer.id] = d
    remaining = dict(deps)
    done: set[str] = set()
    levels: list[list[Layer]] = []
    while remaining:
        ready = [lid for lid, d in remaining.items() if d <= done]
        if not ready:
            raise CyclicDependencyError(f'Zyklische Abhängigkeiten unter Layern: {sorted(remaining)}')
        levels.append([by_id[lid] for lid in ready])
        done.update(ready)
        for lid in ready:
            remaining.pop(lid)
    return levels