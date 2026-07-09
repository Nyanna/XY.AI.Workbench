"""OpenAlex interface package — a complete, unopinionated OpenAlex API client.

This package is the low-level *Schnittstelle* (interface) layer:

* :class:`~xy.ai.mcpc.openalex.client.OpenAlexClient` — full coverage of the
  OpenAlex entity endpoints (list + single) with every query parameter.
* :mod:`~xy.ai.mcpc.openalex.presets` — semantic field presets and result
  post-processing (abstract reconstruction) shared by the tools layer.
* :mod:`~xy.ai.mcpc.openalex.errors` — the exception hierarchy.

The agent-facing tools that build on this client live in
:mod:`xy.ai.mcpc.tools.openalex`.
"""

from __future__ import annotations

from .client import ENTITIES, OpenAlexClient
from .errors import OpenAlexAPIError, OpenAlexError
from .presets import (
    DEFAULT_SEARCH_PRESET,
    DEFAULT_WORK_PRESET,
    GENERIC_PRESETS,
    WORK_PRESET_NAMES,
    WORK_PRESETS,
    project_results,
    reconstruct_abstract,
    resolve_select,
)

__all__ = [
    "ENTITIES",
    "OpenAlexClient",
    "OpenAlexError",
    "OpenAlexAPIError",
    "WORK_PRESETS",
    "WORK_PRESET_NAMES",
    "GENERIC_PRESETS",
    "DEFAULT_SEARCH_PRESET",
    "DEFAULT_WORK_PRESET",
    "resolve_select",
    "reconstruct_abstract",
    "project_results",
]
