"""``openalex_work`` tool: fetch a single work by id / DOI."""
from typing import Any
from xy.ai.mcpc.openalex import DEFAULT_WORK_PRESET, OpenAlexError, parse_entity, project_results, resolve_select
from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
from ._common import WorkResult, error_result, get_client, ok_result
_WORK_PRESETS = list(WORK_PRESET_NAMES)
_WORK_DESCRIPTION = 'Fetch a single OpenAlex work by identifier. Accepts an OpenAlex id (W2741809807), an OpenAlex URL, a DOI (10.7717/peerj.4375 or a doi.org URL) or a namespaced id such as pmid:14907713. Returns the full record by default, with the abstract reconstructed to plain text.'
_WORK_INPUT_SCHEMA: dict[str,
                         Any] = {'type': 'object',
                                 'properties': {'id': {'type': 'string',
                                                       'description': 'Work identifier: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).'},
                                                'fields': {'type': 'string',
                                                           'enum': _WORK_PRESETS,
                                                           'description': 'Field preset (default: full). Use a narrower preset such as bibliographic or abstract to reduce size.'}},
                                 'required': ['id']}

def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:
    client = get_client()
    preset = fields or DEFAULT_WORK_PRESET
    select = resolve_select(preset, 'works')
    data = client.get_work(id, select=select)
    work = project_results([data])[0]
    return {'work': work}

def openalex_work(id: str, fields: str | None=None) -> WorkResult:
    """Fetch a single OpenAlex work by identifier.

    Args:
        id: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).
        fields: Field preset (default: full).

    Returns:
        The parsed work record.

    Raises:
        OpenAlexError: If the OpenAlex API request fails.
    """
    structured = _openalex_work_raw(id, fields=fields)
    return WorkResult(work=parse_entity('works', structured['work']))

class OpenalexWorkTool(ToolDefinition):
    name = 'openalex_work'
    title = 'OpenAlex work'
    description = _WORK_DESCRIPTION
    input_schema = _WORK_INPUT_SCHEMA

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            structured = _openalex_work_raw(id=args['id'], fields=args.get('fields'))
        except OpenAlexError as exc:
            return error_result(exc)
        return ok_result(structured)