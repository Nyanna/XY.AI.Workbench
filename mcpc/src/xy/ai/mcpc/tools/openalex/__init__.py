"""OpenAlex tools.

Three tools sit on top of the :mod:`xy.ai.mcpc.openalex` interface package and
apply standard assumptions so an AI agent can use OpenAlex without knowing the
raw API:

* ``openalex_search``          – keyword / boolean full-text search.
* ``openalex_semantic_search`` – AI (embedding) search by meaning.
* ``openalex_work``            – fetch a single work by id / DOI.

Each tool lives in its own module (:mod:`.search`, :mod:`.semantic_search`,
:mod:`.work`); shared client state and helpers live in :mod:`._common`.

Shared conventions
------------------
* **First page only.** Results are always page 1; ``limit`` controls how many
  hits come back (paging deeper is intentionally not exposed).
* **Field presets.** Instead of raw ``select`` fields, callers pick a semantic
  preset (see :mod:`xy.ai.mcpc.openalex.presets`).
* **Readable abstracts.** OpenAlex's ``abstract_inverted_index`` is rebuilt into
  a plain-text ``abstract`` field.
* **Authentication.** The API key (``MCPC_OPENALEX_KEY``) and optional
  ``mailto`` come from the server config and are handled by the client.
"""
from xy.ai.mcpc.tools.tool_context import AppEnvironment
from xy.ai.mcpc.tools.tool_registry import ToolRegistry
from ._common import SearchResult, WorkResult, build_client, set_client
from .search import OpenalexSearchTool, openalex_search
from .semantic_search import OpenalexSemanticSearchTool, openalex_semantic_search
from .work import OpenalexWorkTool, openalex_work
__all__ = [
    'SearchResult',
    'WorkResult',
    'openalex_search',
    'openalex_semantic_search',
    'openalex_work',
    'OpenalexSearchTool',
    'OpenalexSemanticSearchTool',
    'OpenalexWorkTool',
    'register_openalex_tools']

def register_openalex_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register the three OpenAlex tools onto *registry*."""
    set_client(build_client(environment.config))
    registry.register(OpenalexSearchTool())
    registry.register(OpenalexSemanticSearchTool())
    registry.register(OpenalexWorkTool())
    functions = environment.functions
    if functions is not None:
        functions.register(openalex_search)
        functions.register(openalex_semantic_search)
        functions.register(openalex_work)