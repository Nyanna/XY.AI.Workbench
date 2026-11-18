"""Persist research results as Markdown files and answer with outlines.

Shared by the web research tools. Each result becomes a :class:`Document`
(front matter metadata + Markdown body). With a *file* path all documents are
appended to that file; with a *directory* path every document gets its own
generated file. Instead of the content, the tool result carries file stats and
an outline per written file, so the agent can keep its context small.
"""
import json
import os
import re
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.process import _spill_extra_info
from xy.ai.mcpc.tools.results import RawPayload
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolResult
__all__ = ['PATH_PROPERTY', 'Document', 'persist_result']
PATH_PROPERTY: dict[str, Any] = {
    'type': 'string', 'description': 'Optional. Persist the full results as Markdown. A file path appends to the file; a directory path (existing, or ending with "/") gets one generated file per result.'}
_SLUG_RE = re.compile('[^A-Za-z0-9]+')
_MAX_SLUG = 60

@dataclass(frozen=True)
class Document:
    """One result to persist; ``name`` seeds the generated file name."""
    name: str
    body: str
    metadata: dict[str, Any] = field(default_factory=dict)

def _render(doc: Document, saved_at: str) -> str:
    meta = {k: v for k, v in doc.metadata.items() if v not in (None, '', [], {})}
    meta['saved_at'] = saved_at
    lines = ['---']
    '# JSON scalars and flow collections are valid YAML.'
    lines.extend((f'{key}: {json.dumps(value, ensure_ascii=False, default=str)}' for key, value in meta.items()))
    lines.append('---')
    return '\n'.join(lines) + '\n\n' + doc.body.strip() + '\n'

def _unique_path(directory: Path, name: str) -> Path:
    slug = _SLUG_RE.sub('-', name).strip('-')[:_MAX_SLUG].strip('-') or 'result'
    candidate = directory / f'{slug}.md'
    counter = 2
    while candidate.exists():
        candidate = directory / f'{slug}-{counter}.md'
        counter += 1
    return candidate

def _write(documents: list[Document], target: str) -> list[Path]:
    saved_at = datetime.now(timezone.utc).isoformat(timespec='seconds')
    path = Path(target).expanduser()
    if path.is_dir() or target.endswith(('/', os.sep)):
        path.mkdir(parents=True, exist_ok=True)
        written = []
        for doc in documents:
            file = _unique_path(path, doc.name)
            file.write_text(_render(doc, saved_at), encoding='utf-8')
            written.append(file)
        return written
    path.parent.mkdir(parents=True, exist_ok=True)
    separator = '\n' if path.exists() and path.stat().st_size > 0 else ''
    with path.open('a', encoding='utf-8') as fh:
        fh.write(separator + '\n'.join((_render(doc, saved_at) for doc in documents)))
    return [path]

def persist_result(documents: list[Document], target: str, ctx: ToolContext, *, reason: str, warnings: list[str] | None=None) -> ToolResult:
    """Write *documents* to *target* and return the file outlines as tool result."""
    if not documents:
        return ToolResult.failure('Nothing to persist: no results found.')
    try:
        written = _write(documents, target)
    except OSError as exc:
        return ToolResult.failure(f'Cannot write `{target}`: {exc}')
    notes = list(warnings or [])
    files = []
    for file in written:
        stats, outline, extra = _spill_extra_info(str(file), ctx)
        notes.extend(extra)
        files.append({'path': str(file.resolve()), 'stats': stats, 'outline': outline})
    return ToolResult.ok(RawPayload({'files': files}, reason=reason, warnings=tuple(notes)))