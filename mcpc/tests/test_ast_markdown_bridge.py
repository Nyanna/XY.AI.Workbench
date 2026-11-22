"""Eigenständige Tests für die Markdown-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._markdown``).

Startet den echten ``ast-engines/markdown``-Prozess (Node/remark) lazy und prüft
Konfiguration, Prozess-Lifecycle, Struktur, Mutationen, Fehlerfälle,
Nebenläufigkeit und die komplette ``ast_*``-Tool-Kette end-to-end.

Die Engine normalisiert beim Mutieren den gesamten Text (remark-stringify);
die Tests prüfen daher bewusst den normalisierten Output.
"""
from __future__ import annotations
import dataclasses
import os
import shutil
import signal
import socket
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import pytest
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.base import AstError, Located, OutlineNode, Tree
from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create
from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete
from xy.ai.mcpc.tools.ast.edit_block import EditBlockItem, ast_edit_block
from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks
from xy.ai.mcpc.tools.ast.find import ast_find
from xy.ai.mcpc.tools.ast.generic import get_engine, language_for_extension
from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
from xy.ai.mcpc.tools.ast.outline import ast_outline
from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read
from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
from xy.ai.mcpc.tools.ast.rpc import MarkdownEngine as ExportedMarkdownEngine
from xy.ai.mcpc.tools.ast.rpc._engine import RpcEngine
from xy.ai.mcpc.tools.ast.rpc._markdown import _ENGINE_DIR, _MAIN, _PROCESS, MarkdownEngine, MarkdownProcess
from xy.ai.mcpc.tools.ast.validate import ast_validate
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolRegistry
requires_engine = pytest.mark.skipif(
    shutil.which('node') is None or not (
        _ENGINE_DIR / 'node_modules').is_dir(),
    reason='node or the engine dependencies (npm install in ast-engines/markdown) are missing')
DOC = '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Section B\n\nSecond paragraph.\n'
FILE_DOC = '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 2\n\nbeta\n'

@pytest.fixture(scope='module')
def engine() -> MarkdownEngine:
    return MarkdownEngine()

def _section_id(tree: Tree, *names: str) -> str:
    names_by_id = {loc.node_id: loc.name for loc in tree.engine.locate_all(tree) if loc.node_type == 'Section'}
    for node_id in names_by_id:
        parts = node_id.split('.')
        if tuple(names_by_id.get('.'.join(parts[:n])) for n in range(1, len(parts) + 1)) == names:
            return node_id
    raise LookupError(names)

def _file_id(path: str, *names: str) -> str:
    return _section_id(core.load(path)[1], *names)

def _find(engine: MarkdownEngine, tree: Tree, *names: str) -> Located:
    node_id = _section_id(tree, *names)
    return next((loc for loc in engine.locate_all(tree) if loc.node_id == node_id))

def _ids(engine: MarkdownEngine, source: str) -> list[str]:
    return [loc.node_id for loc in engine.locate_all(engine.parse(source))]

def _free_port() -> int:
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]

def _flatten(nodes: list[OutlineNode]) -> list[OutlineNode]:
    flat: list[OutlineNode] = []
    for node in nodes:
        flat.append(node)
        flat.extend(_flatten(list(node.children)))
    return flat
'# ---------------------------------------------------------------------------'
'# Konfiguration & Registrierung (ohne Prozess)'
'# ---------------------------------------------------------------------------'

def test_engine_dir_and_entry_point_exist() -> None:
    assert _ENGINE_DIR.is_dir()
    assert (_ENGINE_DIR / _MAIN).is_file()

def test_process_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv('AST_MARKDOWN_PORT', raising=False)
    monkeypatch.delenv('NODE', raising=False)
    process = MarkdownProcess()
    assert process.default_port() == 8789
    assert process.cwd() == str(_ENGINE_DIR)
    assert process.command(1234) == ['node', str(_MAIN), '1234']

def test_process_honours_environment(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv('AST_MARKDOWN_PORT', '9911')
    monkeypatch.setenv('NODE', '/opt/node/bin/node')
    process = MarkdownProcess()
    assert process.default_port() == 9911
    assert process.command(9911)[0] == '/opt/node/bin/node'

def test_default_port_does_not_collide_with_other_engines(monkeypatch: pytest.MonkeyPatch) -> None:
    from xy.ai.mcpc.tools.ast.rpc._java import JavaProcess
    from xy.ai.mcpc.tools.ast.rpc._rust import RustProcess
    for var in ('AST_JAVA_PORT', 'AST_RUST_PORT', 'AST_MARKDOWN_PORT'):
        monkeypatch.delenv(var, raising=False)
    ports = {JavaProcess().default_port(), RustProcess().default_port(), MarkdownProcess().default_port()}
    assert len(ports) == 3

def test_engine_is_registered_as_rpc_engine() -> None:
    resolved = get_engine('markdown')
    assert isinstance(resolved, RpcEngine)
    assert isinstance(resolved, MarkdownEngine)
    assert type(resolved).__module__ == 'xy.ai.mcpc.tools.ast.rpc._markdown'
    assert resolved is get_engine('markdown')
    assert ExportedMarkdownEngine is MarkdownEngine

@pytest.mark.parametrize('name', ['x.md', 'x.markdown'])
def test_extensions_resolve_to_markdown_engine(tmp_path: Path, name: str) -> None:
    assert language_for_extension(Path(name).suffix) == 'markdown'
    assert core.engine_for_path(tmp_path / name).name == 'markdown'
    assert isinstance(core.engine_for_path(tmp_path / name), MarkdownEngine)

def test_engine_metadata() -> None:
    resolved = MarkdownEngine()
    assert resolved.symbol == 'markdown'
    assert resolved.name == 'markdown'
    assert resolved.validates_syntax is False

def test_constructing_engine_does_not_start_process() -> None:
    process = MarkdownProcess()
    RpcEngine('markdown', process)
    assert process._process is None
'# ---------------------------------------------------------------------------'
'# Prozess-Lifecycle'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_process_starts_lazily_and_terminates(monkeypatch: pytest.MonkeyPatch) -> None:
    port = _free_port()
    monkeypatch.setenv('AST_MARKDOWN_PORT', str(port))
    process = MarkdownProcess()
    isolated = RpcEngine('markdown', process)
    assert process._process is None
    assert isolated.validate('# a\n') is None
    assert process._process is not None and process._process.poll() is None
    assert process.base_url() == f'http://127.0.0.1:{port}'
    first_pid = process._process.pid
    assert isolated.validate('# b\n') is None
    assert process._process.pid == first_pid
    process._terminate()
    with pytest.raises(OSError):
        socket.create_connection(('127.0.0.1', port), timeout=1).close()

def test_missing_node_binary_fails_fast(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv('NODE', '/nonexistent/node')
    monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
    with pytest.raises((OSError, RuntimeError)):
        MarkdownProcess().base_url()

def test_invalid_port_configuration_is_rejected(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv('AST_MARKDOWN_PORT', 'not-a-port')
    with pytest.raises(ValueError):
        MarkdownProcess().base_url()
'# ---------------------------------------------------------------------------'
'# Parse / Serialize / Validate'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_parse_and_serialize_roundtrip(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    assert engine.serialize(tree) == DOC

@requires_engine
@pytest.mark.parametrize('source', ['', '\n', 'plain\n', '---\ntitle: X\n---\n\n# H\n', '# A\r\n\r\ntext\r\n', '* a\n* b\n', 'no trailing newline', '# Ünï ✓ 日本\n\nText äöü 😀\n'])
def test_parse_does_not_modify_source(engine: MarkdownEngine, source: str) -> None:
    tree = engine.parse(source)
    assert engine.serialize(tree) == source

@requires_engine
@pytest.mark.parametrize('source', ['', '# ok\n', 'text', '\x00', '```\nunclosed fence', '<div>', '[broken](link'])
def test_any_text_is_valid_markdown(engine: MarkdownEngine, source: str) -> None:
    assert engine.validate(source) is None
    engine.parse(source)

@requires_engine
def test_empty_tree_and_empty_document(engine: MarkdownEngine, tmp_path: Path) -> None:
    assert engine.serialize(engine.empty_tree()) == ''
    assert engine.empty_tree(tmp_path / 'x.md').path == tmp_path / 'x.md'
    assert engine.locate_all(engine.empty_tree()) == []
    assert engine.locate_all(engine.parse('')) == []
'# ---------------------------------------------------------------------------'
'# Struktur'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_locate_all_builds_heading_tree(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse(DOC))
    assert [
        loc.node_type for loc in located] == [
            'Section',
            'BlockGroup',
            'Section',
            'BlockGroup',
            'Section',
        'BlockGroup']
    sections = [loc for loc in located if loc.node_type == 'Section']
    section_ids = [loc.node_id for loc in sections]
    assert len(set(section_ids)) == 3
    assert all(i.startswith(section_ids[0] + '.') for i in section_ids[1:])
    assert [loc.name for loc in sections] == ['Title', 'Section A', 'Section B']
    assert located[0].parent_type == 'Root'
    assert all((loc.parent_type == 'Section' for loc in located[1:]))

@requires_engine
def test_line_ranges_are_one_based_and_inclusive(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    ranges = {loc.name: (loc.lineno, loc.end_lineno)
              for loc in engine.locate_all(tree) if loc.node_type == 'Section'}
    assert ranges == {'Title': (1, 11), 'Section A': (5, 7), 'Section B': (9, 11)}

@requires_engine
def test_child_ids_extend_parent_id_and_are_unique(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse(DOC))
    ids = [loc.node_id for loc in located]
    assert len(ids) == len(set(ids))
    for loc in located[1:]:
        assert loc.node_id.startswith(located[0].node_id + '.')

@requires_engine
def test_anonymous_block_ids_are_content_stable(engine: MarkdownEngine) -> None:

    def block_ids(source: str) -> set[str]:
        return {loc.node_id for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'BlockGroup'}
    first = block_ids(DOC)
    assert first == block_ids(DOC)
    third = block_ids(DOC.replace('Second paragraph.', 'Other words.'))
    assert len(first & third) == 2 and len(third - first) == 1

@requires_engine
def test_signature_docstring_and_code(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    title = _find(engine, tree, 'Title')
    section_a = _find(engine, tree, 'Title', 'Section A')
    assert engine.signature(title.node) == '# Title'
    assert engine.signature(section_a.node) == '## Section A'
    assert engine.docstring(section_a.node) == 'First paragraph.'
    assert engine.node_code(section_a.node) == '## Section A\n\nFirst paragraph.\n'
    assert engine.node_code(title.node) == DOC
    block = next((loc for loc in engine.locate_all(tree) if loc.node_type == 'BlockGroup'))
    assert engine.signature(block.node) == 'Intro text.'
    assert engine.node_code(block.node) == 'Intro text.\n'

@requires_engine
def test_document_without_headings_has_root_blocks(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse('just text\n\nmore text\n'))
    assert [loc.node_type for loc in located] == ['BlockGroup']
    assert located[0].parent_type == 'Root'
    assert (located[0].lineno, located[0].end_lineno) == (1, 3)

@requires_engine
def test_frontmatter_is_its_own_node(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse('---\ntitle: X\n---\n\n# H1\n\ntext\n'))
    assert [loc.node_type for loc in located] == ['Frontmatter', 'Section', 'BlockGroup']
    assert (located[0].lineno, located[0].end_lineno) == (1, 3)
    assert located[0].parent_type == 'Root'
    assert located[1].name == 'H1' and '.' not in located[1].node_id

@requires_engine
def test_setext_headings_are_sections(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse('Setext\n======\n\npara\n\nSub\n---\n\nmore\n'))
    sections = [loc for loc in located if loc.node_type == 'Section']
    assert [loc.name for loc in sections] == ['Setext', 'Sub']
    assert sections[1].node_id.startswith(sections[0].node_id + '.')
    assert [engine.signature(loc.node) for loc in sections] == ['# Setext', '## Sub']

@requires_engine
def test_skipped_heading_levels_nest_under_previous(engine: MarkdownEngine) -> None:
    sections = [loc for loc in engine.locate_all(engine.parse('# A\n\n#### Deep\n\ntext\n\n## B\n')) if loc.node_type == 'Section']
    assert [loc.name for loc in sections] == ['A', 'Deep', 'B']
    assert all(loc.node_id.startswith(sections[0].node_id + '.') and loc.node_id.count('.') == 1 for loc in sections[1:])

@requires_engine
def test_duplicate_headings_get_numbered_ids(engine: MarkdownEngine) -> None:
    ids = [i for i in _ids(engine, '# T\n\n## A\n\nx\n\n## A\n\ny\n') if '|' not in i]
    assert len(ids) == 3 and ids[2] == ids[1] + '_2'
    ids = [i for i in _ids(engine, '# A\n\n# A\n') if '|' not in i]
    assert len(ids) == 2 and ids[1] == ids[0] + '_2'

@requires_engine
def test_headings_inside_code_quote_and_list_are_not_sections(engine: MarkdownEngine) -> None:
    source = '# Real\n\n```\n# not heading\n```\n\n> # quoted\n\n- # in list\n\n## Next\n'
    sections = [loc for loc in engine.locate_all(engine.parse(source)) if loc.node_type == 'Section']
    assert [loc.name for loc in sections] == ['Real', 'Next']
    assert sections[1].node_id.startswith(sections[0].node_id + '.')

@requires_engine
def test_mixed_blocks_form_one_block_group(engine: MarkdownEngine) -> None:
    source = '# A\n\n- a\n- b\n\n1. one\n2. two\n\n```py\nprint(1)\n```\n\n> quote\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n<div>x</div>\n'
    located = engine.locate_all(engine.parse(source))
    assert [loc.node_type for loc in located] == ['Section', 'BlockGroup']
    assert (located[1].lineno, located[1].end_lineno) == (3, 19)

@requires_engine
def test_unicode_headings_and_content(engine: MarkdownEngine) -> None:
    tree = engine.parse('# Ünï ✓ 日本\n\nText äöü 😀\n')
    section = engine.locate_all(tree)[0]
    assert section.name == 'Ünï ✓ 日本'
    assert engine.node_code(section.node) == '# Ünï ✓ 日本\n\nText äöü 😀\n'
    engine.replace(section, '# Ünï ✓ 日本\n\nNeu ß 🚀')
    assert engine.serialize(tree) == '# Ünï ✓ 日本\n\nNeu ß 🚀\n'

@requires_engine
def test_thematic_breaks_do_not_split_sections(engine: MarkdownEngine) -> None:
    located = engine.locate_all(engine.parse('# A\n\nx\n\n---\n\n# B\n\ny\n'))
    assert [loc.name for loc in located if loc.node_type == 'Section'] == ['A', 'B']
    assert all((loc.parent_type != 'Page' for loc in located))
'# ---------------------------------------------------------------------------'
'# Mutationen'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_replace_paragraph(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    block = next((loc for loc in engine.locate_all(tree) if loc.node_type ==
                 'BlockGroup' and 'Second' in engine.node_code(loc.node)))
    engine.replace(block, 'Changed paragraph.')
    assert engine.serialize(tree) == DOC.replace('Second paragraph.', 'Changed paragraph.')

@requires_engine
def test_replace_section_with_new_heading_and_body(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    engine.replace(_find(engine, tree, 'Title', 'Section A'), '## Section A2\n\nnew body')
    assert engine.serialize(
        tree) == '# Title\n\nIntro text.\n\n## Section A2\n\nnew body\n\n## Section B\n\nSecond paragraph.\n'
    reparsed = engine.parse(tree.source)
    assert _section_id(reparsed, 'Title', 'Section A2')
    with pytest.raises(LookupError):
        _section_id(reparsed, 'Title', 'Section A')

@requires_engine
def test_replace_changes_heading_level(engine: MarkdownEngine) -> None:
    tree = engine.parse('# A\n\n## B\n\nb\n\n## C\n\nc\n')
    engine.replace(_find(engine, tree, 'A', 'B'), '### B3\n\nlevel3')
    assert engine.serialize(tree) == '# A\n\n### B3\n\nlevel3\n\n## C\n\nc\n'

@requires_engine
def test_replace_root_section_replaces_whole_document(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    engine.replace(_find(engine, tree, 'Title'), '# Only\n\nleft')
    assert engine.serialize(tree) == '# Only\n\nleft\n'

@requires_engine
def test_replace_with_multiple_blocks(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith(_section_id(tree, 'Title', 'Section B') + '.')))
    engine.replace(block, 'one\n\n- a\n- b\n\n```js\nx\n```')
    assert engine.serialize(tree).endswith('## Section B\n\none\n\n- a\n- b\n\n```js\nx\n```\n')

@requires_engine
def test_insert_section_before_and_after(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    assert engine.insert(_find(engine, tree, 'Title', 'Section B'), '## Mid\n\nmid', 'before') == 2
    assert engine.serialize(
        tree) == '# Title\n\nIntro text.\n\n## Section A\n\nFirst paragraph.\n\n## Mid\n\nmid\n\n## Section B\n\nSecond paragraph.\n'
    tree = engine.parse(DOC)
    assert engine.insert(_find(engine, tree, 'Title', 'Section B'), '## After\n\nx', 'after') == 2
    assert engine.serialize(tree).endswith('Second paragraph.\n\n## After\n\nx\n')

@requires_engine
def test_insert_returns_number_of_blocks(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    assert engine.insert(_find(engine, tree, 'Title', 'Section A'), 'single', 'after') == 1
    tree = engine.parse(DOC)
    assert engine.insert(_find(engine, tree, 'Title', 'Section A'), 'p1\n\np2\n\np3', 'after') == 3

@requires_engine
def test_insert_before_first_node(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    engine.insert(_find(engine, tree, 'Title'), 'prefix', 'before')
    assert engine.serialize(tree) == 'prefix\n\n' + DOC

@requires_engine
def test_delete_section(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    engine.delete(_find(engine, tree, 'Title', 'Section A'))
    assert engine.serialize(tree) == '# Title\n\nIntro text.\n\n## Section B\n\nSecond paragraph.\n'

@requires_engine
def test_delete_paragraph_and_whole_document(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith(_section_id(tree, 'Title', 'Section A') + '.')))
    engine.delete(block)
    assert 'First paragraph.' not in engine.serialize(tree)
    assert '## Section A' in engine.serialize(tree)
    tree = engine.parse(DOC)
    engine.delete(_find(engine, tree, 'Title'))
    assert engine.serialize(tree) == ''
    assert engine.locate_all(tree) == []

@requires_engine
def test_append(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    assert engine.append(tree, '## Section C\n\nappended') == 2
    assert engine.serialize(tree) == DOC + '\n## Section C\n\nappended\n'

@requires_engine
def test_append_to_empty_tree(engine: MarkdownEngine) -> None:
    tree = engine.empty_tree()
    assert engine.append(tree, '# New\n\nbody') == 2
    assert engine.serialize(tree) == '# New\n\nbody\n'
    assert engine.append(tree, 'tail') == 1
    assert engine.serialize(tree) == '# New\n\nbody\n\ntail\n'

@requires_engine
def test_mutation_normalizes_document(engine: MarkdownEngine) -> None:
    source = '# Title\n\n* item one\n* item two\n\n\nSome   *emph*   and __strong__ text.\n\n## Sec\n\nlast\n'
    tree = engine.parse(source)
    assert engine.serialize(tree) == source
    block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith(_section_id(tree, 'Title', 'Sec') + '.')))
    engine.replace(block, 'changed')
    assert engine.serialize(
        tree) == '# Title\n\n- item one\n- item two\n\nSome   _emph_   and **strong** text.\n\n## Sec\n\nchanged\n'

@requires_engine
def test_mutation_normalizes_crlf_to_lf(engine: MarkdownEngine) -> None:
    tree = engine.parse('# A\r\n\r\ntext\r\n\r\n## B\r\n\r\nmore\r\n')
    block = next((loc for loc in engine.locate_all(tree) if loc.node_id.startswith(_section_id(tree, 'A', 'B') + '.')))
    engine.replace(block, 'new')
    assert '\r' not in engine.serialize(tree)
    assert engine.serialize(tree) == '# A\n\ntext\n\n## B\n\nnew\n'

@requires_engine
def test_mutation_keeps_frontmatter(engine: MarkdownEngine) -> None:
    tree = engine.parse('---\ntitle: X\n---\n\n# H\n\ntext\n')
    engine.replace(_find(engine, tree, 'H'), '# H\n\nchanged')
    assert engine.serialize(tree) == '---\ntitle: X\n---\n\n# H\n\nchanged\n'

@requires_engine
def test_mutations_chain_on_the_same_tree(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    engine.replace(_find(engine, tree, 'Title', 'Section A'), '## A\n\nfirst')
    engine.insert(_find(engine, tree, 'Title', 'A'), '## A2\n\nsecond', 'after')
    engine.delete(_find(engine, tree, 'Title', 'Section B'))
    engine.append(tree, '## Z\n\nlast')
    assert engine.serialize(tree) == '# Title\n\nIntro text.\n\n## A\n\nfirst\n\n## A2\n\nsecond\n\n## Z\n\nlast\n'
'# ---------------------------------------------------------------------------'
'# Fehlerfälle'
'# ---------------------------------------------------------------------------'

@requires_engine
@pytest.mark.parametrize('operation', ['replace', 'insert', 'delete'])
def test_unknown_node_id_raises_and_keeps_source(engine: MarkdownEngine, operation: str) -> None:
    tree = engine.parse(DOC)
    ghost = dataclasses.replace(_find(engine, tree, 'Title'), node_id='Does.Not.Exist')
    with pytest.raises(AstError, match='not found'):
        if operation == 'replace':
            engine.replace(ghost, 'x')
        elif operation == 'insert':
            engine.insert(ghost, 'x', 'after')
        else:
            engine.delete(ghost)
    assert tree.source == DOC

@requires_engine
def test_empty_replacement_is_rejected(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    with pytest.raises(AstError):
        engine.replace(_find(engine, tree, 'Title', 'Section A'), '')
    assert tree.source == DOC

@requires_engine
def test_control_characters_are_rejected_before_the_call(engine: MarkdownEngine) -> None:
    tree = engine.parse(DOC)
    loc = _find(engine, tree, 'Title', 'Section A')
    with pytest.raises(AstError, match='control character'):
        engine.replace(loc, 'a\x01b')
    with pytest.raises(AstError, match='control character'):
        engine.insert(loc, 'a\x01b', 'after')
    with pytest.raises(AstError, match='control character'):
        engine.append(tree, 'a\x01b')
    assert tree.source == DOC

@requires_engine
def test_imports_are_not_supported_for_markdown(engine: MarkdownEngine) -> None:
    tree = engine.parse('# A\n')
    with pytest.raises(AstError):
        engine.apply_imports(tree, ['import x'], [])
    assert tree.source == '# A\n'
'# ---------------------------------------------------------------------------'
'# Robustheit'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_concurrent_requests_are_isolated(engine: MarkdownEngine) -> None:

    def work(i: int) -> bool:
        tree = engine.parse(f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nbody {i}\n')
        engine.replace(_find(engine, tree, f'Doc{i}', 'Sub'), f'## Sub\n\nnew {i}')
        return tree.source == f'# Doc{i}\n\ntext {i}\n\n## Sub\n\nnew {i}\n'
    with ThreadPoolExecutor(16) as pool:
        assert all(pool.map(work, range(96)))

@requires_engine
def test_concurrent_first_use_spawns_a_single_process(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv('AST_MARKDOWN_PORT', str(_free_port()))
    process = MarkdownProcess()
    isolated = RpcEngine('markdown', process)
    try:
        with ThreadPoolExecutor(8) as pool:
            results = list(pool.map(lambda _: isolated.validate('# a\n'), range(8)))
        assert results == [None] * 8
        assert process._process is not None
    finally:
        process._terminate()

@requires_engine
def test_large_document(engine: MarkdownEngine) -> None:
    sections = 300
    source = '# Big\n\n' + \
        ''.join((f'## Section {i}\n\n' + ''.join((f'Paragraph {i}.{j} lorem ipsum.\n\n' for j in range(5))) for i in range(sections)))
    tree = engine.parse(source)
    assert len(engine.locate_all(tree)) == 1 + 2 * sections
    engine.replace(_find(engine, tree, 'Big', f'Section {sections - 1}'), f'## Section {sections - 1}\n\nX')
    assert tree.source.endswith(f'## Section {sections - 1}\n\nX\n')
    assert 'Section 0' in tree.source

@requires_engine
def test_long_single_line(engine: MarkdownEngine) -> None:
    line = 'word ' * 20000
    tree = engine.parse(f'# A\n\n{line}\n')
    engine.append(tree, 'tail')
    assert tree.source.endswith('tail\n')
    assert line.strip() in tree.source
'# ---------------------------------------------------------------------------'
'# Tool-Ebene (Dateien, Cache, ast_*-Tools)'
'# ---------------------------------------------------------------------------'

@pytest.fixture
def md_file(tmp_path: Path) -> str:
    path = tmp_path / 'doc.md'
    path.write_text(FILE_DOC, encoding='utf-8')
    return str(path)

@requires_engine
def test_load_uses_markdown_rpc_engine(md_file: str) -> None:
    _, tree = core.load(md_file)
    assert tree.engine.name == 'markdown'
    assert isinstance(tree.engine, RpcEngine)

@requires_engine
def test_outline_lists_sections_and_blocks(md_file: str) -> None:
    nodes = ast_outline([md_file]).results[0].nodes
    assert [node.id for node in nodes] == [_file_id(md_file, 'Doc')]
    child_ids = [child.id for child in nodes[0].children]
    assert _file_id(md_file, 'Doc', 'Part 1') in child_ids and _file_id(md_file, 'Doc', 'Part 2') in child_ids
    assert nodes[0].excert == '# Doc'

@requires_engine
def test_find_by_type_and_name(md_file: str) -> None:
    sections = _flatten(ast_find(paths=[md_file], node_type='Section').files[0].nodes)
    assert [node.id for node in sections] == [_file_id(md_file, 'Doc'), _file_id(md_file, 'Doc', 'Part 1'), _file_id(md_file, 'Doc', 'Part 2')]
    named = _flatten(ast_find(paths=[md_file], name='Part 2').files[0].nodes)
    assert [node.id for node in named] == [_file_id(md_file, 'Doc', 'Part 2')]
    assert named[0].code == '## Part 2\n\nbeta\n'

@requires_engine
def test_read_returns_section_source(md_file: str) -> None:
    result = ast_read(items=[ReadItem(path=md_file, ids=[_file_id(md_file, 'Doc', 'Part 1')])])
    assert not result.errors
    assert result.results[0].nodes[0].code == '## Part 1\n\nalpha\n'

@requires_engine
def test_replace_via_tool_writes_file(md_file: str) -> None:
    result = ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nALPHA', id=_file_id(md_file, 'Doc', 'Part 1'))])
    assert not result.errors
    assert result.results[0].new_id == _file_id(md_file, 'Doc', 'Part 1')
    assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nALPHA\n\n## Part 2\n\nbeta\n'

@requires_engine
def test_insert_via_tool_reports_ids(md_file: str) -> None:
    result = ast_insert(items=[InsertItem(path=md_file, source='## Part 1b\n\nmid', id=_file_id(md_file, 'Doc', 'Part 1'), position='after')])
    assert not result.errors
    assert result.results[0].inserted == 2
    assert result.results[0].ids[0] == _file_id(md_file, 'Doc', 'Part 1b')
    assert Path(md_file).read_text(
        encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nalpha\n\n## Part 1b\n\nmid\n\n## Part 2\n\nbeta\n'

@requires_engine
def test_delete_via_tool(md_file: str) -> None:
    result = ast_delete(items=[DeleteItem(path=md_file, id=_file_id(md_file, 'Doc', 'Part 2'))])
    assert not result.errors
    assert Path(md_file).read_text(encoding='utf-8') == '# Doc\n\nintro\n\n## Part 1\n\nalpha\n'

@requires_engine
def test_edit_marks_via_tool(md_file: str) -> None:
    result = ast_edit_marks(
        items=[
            EditMarksItem(
                path=md_file,
                start_marker='alpha',
                end_marker='alpha',
                content='gamma',
                id=_file_id(md_file, 'Doc', 'Part 1'))])
    assert not result.errors
    text = Path(md_file).read_text(encoding='utf-8')
    assert 'gamma' in text and 'alpha' not in text and ('beta' in text)

@requires_engine
def test_edit_block_via_tool(md_file: str) -> None:
    result = ast_edit_block(items=[EditBlockItem(path=md_file, old_text='beta', new_text='delta', id=_file_id(md_file, 'Doc', 'Part 2'))])
    assert not result.errors
    text = Path(md_file).read_text(encoding='utf-8')
    assert 'delta' in text and 'beta' not in text and ('alpha' in text)

@requires_engine
def test_batch_of_edits_in_one_call(md_file: str) -> None:
    result = ast_replace(
        items=[
            ReplaceItem(
                path=md_file,
                source='## Part 1\n\nONE',
                id=_file_id(md_file, 'Doc', 'Part 1')),
            ReplaceItem(
                path=md_file,
                source='## Part 2\n\nTWO',
                id=_file_id(md_file, 'Doc', 'Part 2'))])
    assert not result.errors and len(result.results) == 2
    text = Path(md_file).read_text(encoding='utf-8')
    assert 'ONE' in text and 'TWO' in text and ('alpha' not in text) and ('beta' not in text)

@requires_engine
def test_unknown_id_via_tool_is_reported_and_file_untouched(md_file: str) -> None:
    result = ast_replace(items=[ReplaceItem(path=md_file, source='## X\n\ny', id='Doc.Nope')])
    assert result.errors and (not result.results)
    assert Path(md_file).read_text(encoding='utf-8') == FILE_DOC

@requires_engine
def test_validate_via_tool_accepts_any_markdown(tmp_path: Path) -> None:
    path = tmp_path / 'odd.md'
    path.write_text('```\nunclosed\n\n[x](\n', encoding='utf-8')
    assert ast_validate([str(path)]).all_ok

@requires_engine
def test_create_via_tool_and_overwrite(tmp_path: Path) -> None:
    path = tmp_path / 'new.md'
    first = ast_create(items=[CreateFileItem(path=str(path), source='# One\n\ntext\n')])
    assert not first.errors
    assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
    refused = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n')])
    assert refused.errors
    assert path.read_text(encoding='utf-8') == '# One\n\ntext\n'
    again = ast_create(items=[CreateFileItem(path=str(path), source='# Two\n', overwrite=True)])
    assert not again.errors
    assert path.read_text(encoding='utf-8') == '# Two\n'

@requires_engine
def test_cache_follows_external_changes(md_file: str) -> None:
    path = Path(md_file)
    _, first = core.load(md_file)
    assert core.CACHE.get_tree(path) is first
    path.write_text('# Other\n\nchanged\n', encoding='utf-8')
    _, second = core.load(md_file)
    assert [loc.name for loc in core.locate_all(second) if loc.node_type == 'Section'] == ['Other']

@requires_engine
def test_cache_follows_tool_edits(md_file: str) -> None:
    ast_replace(items=[ReplaceItem(path=md_file, source='## Part 1\n\nNEW', id=_file_id(md_file, 'Doc', 'Part 1'))])
    _, tree = core.load(md_file)
    assert 'NEW' in tree.source
    result = ast_read(items=[ReadItem(path=md_file, ids=[_file_id(md_file, 'Doc', 'Part 1')])])
    assert result.results[0].nodes[0].code == '## Part 1\n\nNEW\n'

@requires_engine
def test_unicode_file_roundtrip_via_tool(tmp_path: Path) -> None:
    path = tmp_path / 'u.md'
    path.write_text('# Grüße\n\nText 日本 😀\n\n## Teil\n\nalt\n', encoding='utf-8')
    ast_replace(items=[ReplaceItem(path=str(path), source='## Teil\n\nneu äöü', id=_file_id(str(path), 'Grüße', 'Teil'))])
    assert path.read_text(encoding='utf-8') == '# Grüße\n\nText 日本 😀\n\n## Teil\n\nneu äöü\n'
'# ---------------------------------------------------------------------------'
'# ast_retrieve über die Tool-Registry'
'# ---------------------------------------------------------------------------'

def _retrieve(registry: ToolRegistry, arguments: dict):
    tool = registry.get('ast_retrieve')
    assert tool is not None
    session = Session(id='markdown-bridge')
    session.enabled_tools.add('ast_retrieve')
    return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))

@requires_engine
def test_retrieve_outline_find_read(registry: ToolRegistry, md_file: str) -> None:
    result = _retrieve(registry, {'outline': [md_file], 'find': [{'paths': [md_file], 'name': 'Part 1'}], 'read': [
                       {'path': md_file, 'ids': [_file_id(md_file, 'Doc', 'Part 2')]}]})
    assert not result.is_error
    content = result.structured_content
    assert set(content) == {'outline', 'find', 'read'}
    assert _file_id(md_file, 'Doc', 'Part 1') in str(content['find'])
    assert 'beta' in str(content['read'])

@requires_engine
def test_retrieve_search_returns_node_ids_usable_for_read(registry: ToolRegistry, md_file: str) -> None:
    directory = str(Path(md_file).parent)
    first = _retrieve(registry, {'search': [{'directory': [directory],
                      'pattern': 'beta', 'include': ['*.md'], 'intent': 'edit'}]})
    assert not first.is_error
    entry = first.structured_content['search']['results'][0]['matches'][0]
    node_id = entry['nodes'][0]['id']
    assert node_id.startswith(_file_id(md_file, 'Doc', 'Part 2'))
    second = _retrieve(registry, {'read': [{'path': md_file, 'ids': [node_id]}]})
    assert not second.is_error
    assert 'beta' in str(second.structured_content['read'])
'# ---------------------------------------------------------------------------'
'# Absturz des Engine-Prozesses (zuletzt: stellt den Prozess danach wieder her)'
'# ---------------------------------------------------------------------------'

@requires_engine
def test_new_engine_instance_restarts_crashed_process() -> None:
    MarkdownEngine().validate('# warm\n')
    process = _PROCESS._process
    assert process is not None
    os.kill(process.pid, signal.SIGKILL)
    process.wait(timeout=5)
    fresh = MarkdownEngine()
    assert fresh.validate('# a\n') is None
    assert _PROCESS._process is not process

@requires_engine
@pytest.mark.xfail(strict=True, reason='RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.')
def test_existing_engine_instance_recovers_after_process_crash() -> None:
    used = MarkdownEngine()
    used.validate('# warm\n')
    process = _PROCESS._process
    assert process is not None
    os.kill(process.pid, signal.SIGKILL)
    process.wait(timeout=5)
    try:
        assert used.validate('# a\n') is None
    finally:
        MarkdownEngine().validate('# restore\n')