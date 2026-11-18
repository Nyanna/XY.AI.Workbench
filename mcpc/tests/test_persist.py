from __future__ import annotations
import pytest
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools.mcp.exa.core import search_cache as exa_search_cache
from xy.ai.mcpc.tools.openalex._common import search_cache as openalex_search_cache
from xy.ai.mcpc.tools.persist import Document, persist_result
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolRegistry

@pytest.fixture
def ctx() -> ToolContext:
    return ToolContext(session=Session(id='persist'), arguments={})

def _docs() -> list[Document]:
    return [
        Document(
            name='First Result!',
            body='# First\n\nbody one',
            metadata={
                'id': 'a',
                'year': 2024,
                'empty': None}),
        Document(
            name='Second',
            body='# Second\n\nbody two',
            metadata={
                'id': 'b',
                'tags': [
                    'x',
                    'y']})]

def test_file_target_creates_and_appends(tmp_path, ctx):
    target = tmp_path / 'sub' / 'notes.md'
    first = persist_result(_docs()[:1], str(target), ctx, reason='test')
    second = persist_result(_docs()[1:], str(target), ctx, reason='test')
    assert not first.is_error and (not second.is_error)
    text = target.read_text(encoding='utf-8')
    assert text.startswith('---\nid: "a"\nyear: 2024\nsaved_at: ')
    assert 'empty' not in text
    assert text.count('\n---\n\n#') == 2
    assert 'body one' in text and 'body two' in text
    assert text.index('body one') < text.index('body two')
    assert 'tags: ["x", "y"]' in text
    files = first.structured_content['files']
    assert [f['path'] for f in files] == [str(target.resolve())]
    assert files[0]['stats']

def test_directory_target_writes_one_file_per_document(tmp_path, ctx):
    result = persist_result(_docs(), str(tmp_path / 'out') + '/', ctx, reason='test')
    files = result.structured_content['files']
    assert len(files) == 2
    names = sorted((p.name for p in (tmp_path / 'out').iterdir()))
    assert names == ['First-Result.md', 'Second.md']
    assert 'body two' not in (tmp_path / 'out' / 'First-Result.md').read_text(encoding='utf-8')

def test_existing_directory_without_trailing_slash(tmp_path, ctx):
    result = persist_result(_docs()[:1], str(tmp_path), ctx, reason='test')
    assert len(result.structured_content['files']) == 1
    assert (tmp_path / 'First-Result.md').is_file()

def test_directory_name_collision_gets_suffix(tmp_path, ctx):
    persist_result(_docs()[:1], str(tmp_path), ctx, reason='test')
    persist_result(_docs()[:1], str(tmp_path), ctx, reason='test')
    assert sorted((p.name for p in tmp_path.iterdir())) == ['First-Result-2.md', 'First-Result.md']

def test_outline_included_when_ast_outline_enabled(tmp_path):
    session = Session(id='persist-outline')
    session.enabled_tools.add('ast_outline')
    ctx = ToolContext(session=session, arguments={})
    result = persist_result(_docs()[:1], str(tmp_path / 'o.md'), ctx, reason='test')
    assert result.structured_content['files'][0]['outline']

def test_no_documents_is_an_error(tmp_path, ctx):
    assert persist_result([], str(tmp_path / 'x.md'), ctx, reason='test').is_error

def test_unwritable_target_is_an_error(tmp_path, ctx):
    blocker = tmp_path / 'file'
    blocker.write_text('x')
    result = persist_result(_docs()[:1], str(blocker / 'child.md'), ctx, reason='test')
    assert result.is_error

def test_openalex_search_results_persists_record(registry: ToolRegistry, tmp_path, ctx):
    openalex_search_cache.put({'id': 'W-persist-1', 'title': 'A Paper',
                              'abstract': 'Some abstract.', 'publication_year': 2023})
    tool = registry.get('openalex_search_results')
    result = tool.handler(
        ToolContext(
            session=ctx.session,
            arguments={
                'ids': [
                    'W-persist-1',
                    'W-missing'],
                'path': str(tmp_path)}))
    assert not result.is_error
    assert 'results' not in result.structured_content
    text = (tmp_path / 'A-Paper.md').read_text(encoding='utf-8')
    assert 'publication_year: 2023' in text
    assert '## Abstract\n\nSome abstract.' in text
    assert 'abstract:' not in text.split('---')[1]
    assert any(('W-missing' in w for w in result.warnings))

def test_exa_search_results_persists_text(registry: ToolRegistry, tmp_path, ctx):
    exa_search_cache.put({'id': 'exa-persist-1', 'title': 'Page', 'url': 'https://example.org', 'text': 'full text'})
    tool = registry.get('web_search_exa_results')
    result = tool.handler(
        ToolContext(
            session=ctx.session,
            arguments={
                'ids': ['exa-persist-1'],
                'path': str(
                    tmp_path /
                    'r.md')}))
    assert not result.is_error
    text = (tmp_path / 'r.md').read_text(encoding='utf-8')
    assert 'url: "https://example.org"' in text
    assert text.rstrip().endswith('full text')

def test_path_is_part_of_tool_schemas(registry: ToolRegistry):
    for name in ('openalex_work', 'openalex_search_results', 'web_search_exa_results', 'web_fetch_exa_results', 'context7_documentation'):
        tool = registry.get(name)
        if tool is not None:
            assert 'path' in tool.input_schema['properties'], name