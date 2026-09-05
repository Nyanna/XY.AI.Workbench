"""Tests for the multi-engine ``ast_*`` tool family.

Covers engine selection by file extension, the Python (``ast``) engine, the
generic tree-sitter engine, engine-independent node addressing (by id and by
type/name), and reuse/invalidation of the shared parse cache.
"""
from __future__ import annotations
import sys
from pathlib import Path
import pytest
'# Make the ``src`` layout importable without requiring an editable install.'
_SRC = Path(__file__).resolve().parents[1] / 'src'
if _SRC.is_dir() and str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))
'# noqa: E402'
from xy.ai.mcpc.tools.ast import core
'# noqa: E402'
from xy.ai.mcpc.tools.ast.create import ast_create
'# noqa: E402'
from xy.ai.mcpc.tools.ast.delete import ast_delete
'# noqa: E402'
from xy.ai.mcpc.tools.ast.edit_marks import ast_edit_marks
'# noqa: E402'
from xy.ai.mcpc.tools.ast.find import ast_find
'# noqa: E402'
from xy.ai.mcpc.tools.ast.insert import ast_insert
'# noqa: E402'
from xy.ai.mcpc.tools.ast.list import ast_list
'# noqa: E402'
from xy.ai.mcpc.tools.ast.read import ast_read
'# noqa: E402'
from xy.ai.mcpc.tools.ast.replace import ast_replace
'# noqa: E402'
from xy.ai.mcpc.tools.ast.validate import ast_validate
PY_SOURCE = 'import os\n\nclass A:\n    def foo(self):\n        x = 1\n        return x\n\ndef bar():\n    return 2\n'
JSON_SOURCE = '{\n  "name": "demo",\n  "deps": {\n    "a": 1,\n    "b": 2\n  }\n}\n'

@pytest.fixture
def py_file(tmp_path: Path) -> str:
    path = tmp_path / 'm.py'
    path.write_text(PY_SOURCE, encoding='utf-8')
    return str(path)

@pytest.fixture
def json_file(tmp_path: Path) -> str:
    path = tmp_path / 'c.json'
    path.write_text(JSON_SOURCE, encoding='utf-8')
    return str(path)
'# --------------------------------------------------------------------------- #'
'# Engine selection'
'# --------------------------------------------------------------------------- #'

def test_engine_selection_by_extension(tmp_path: Path):
    assert core.engine_for_path(tmp_path / 'x.py').name == 'python'
    assert core.engine_for_path(tmp_path / 'x.pyi').name == 'python'
    assert core.engine_for_path(tmp_path / 'x.json').name == 'tree-sitter:json'
    assert core.engine_for_path(tmp_path / 'x.yaml').name == 'tree-sitter:yaml'

def test_engine_selection_unsupported_extension(tmp_path: Path):
    with pytest.raises(core.AstError):
        core.engine_for_path(tmp_path / 'x.unknownext')

def test_snippet_defaults_to_python():
    tree = core.parse_source('def f():\n    return 1\n')
    assert tree.engine is core.python.ENGINE
'# --------------------------------------------------------------------------- #'
'# Python engine'
'# --------------------------------------------------------------------------- #'

def test_python_outline(py_file):
    nodes = ast_list(py_file).nodes
    kinds = [(n.type, n.id) for n in nodes]
    assert ('ClassDef', 'A') in kinds
    assert ('FunctionDef', 'bar') in kinds
    cls = next((n for n in nodes if n.id == 'A'))
    assert any((c.id == 'A.foo' for c in cls.children))

def test_python_find_by_name_and_type(py_file):
    hits = ast_find(paths=[py_file], name='bar', node_type='FunctionDef').files[0].nodes
    assert [h.id for h in hits] == ['bar']

def test_python_read_returns_source(py_file):
    result = ast_read(ids=['A.foo'], path=py_file)
    assert not result.errors
    node = result.nodes[0]
    assert node.code is not None
    assert 'return x' in node.code

def test_python_full_crud_roundtrip(py_file):
    ast_replace(py_file, 'def bar():\n    return 42', id='bar')
    ast_edit_marks(py_file, 'x = 1', 'return x', 'return 99', id='A.foo')
    ast_insert(py_file, 'z = 5', id='bar', position='after')
    import_id = ast_find(paths=[py_file], node_type='imports').files[0].nodes[0].id
    ast_delete(py_file, id=import_id)
    assert ast_validate([py_file]).all_ok
    text = Path(py_file).read_text()
    assert 'return 42' in text
    assert 'return 99' in text
    assert 'z = 5' in text
    assert 'import os' not in text
'# --------------------------------------------------------------------------- #'
'# Generic tree-sitter engine'
'# --------------------------------------------------------------------------- #'

def test_generic_uses_treesitter_engine(json_file):
    _, tree = core.load(json_file)
    assert tree.engine.name == 'tree-sitter:json'

def test_generic_qualified_names(json_file):
    """# Nested pairs smaller than SEGMENT_MAX_CHARS are reached through their"""
    '# parent, not addressable in their own right; only the top-level value is.'
    _, tree = core.load(json_file)
    locs = core.locate_all(tree)
    assert len(locs) == 1
    assert locs[0].node_type == 'object'

def test_generic_replace_by_qualified_name(json_file):
    _, tree = core.load(json_file)
    obj_id = core.locate_all(tree)[0].node_id
    ast_replace(json_file, '{"a": 111}', id=obj_id)
    assert '"a": 111' in Path(json_file).read_text()
    assert ast_validate([json_file]).all_ok

def test_generic_edit_between_markers(json_file):
    _, tree = core.load(json_file)
    obj_id = core.locate_all(tree)[0].node_id
    ast_edit_marks(json_file, '"a": 1', '"b": 2', '"a": 10,\n    "b": 20', id=obj_id)
    text = Path(json_file).read_text()
    assert '"a": 10' in text and '"b": 20' in text
    assert ast_validate([json_file]).all_ok

def test_generic_validate_reports_error(tmp_path: Path):
    bad = tmp_path / 'bad.json'
    bad.write_text('{"a": }\n', encoding='utf-8')
    check = ast_validate([str(bad)]).files[0]
    assert not check.ok
    assert check.error
'# --------------------------------------------------------------------------- #'
'# Engine-independent node addressing'
'# --------------------------------------------------------------------------- #'

def test_addressing_by_id_matches_located_node(json_file):
    _, tree = core.load(json_file)
    target = core.locate_all(tree)[0]
    by_id = ast_find(paths=[json_file], id=target.node_id).files[0].nodes
    assert len(by_id) == 1
    assert by_id[0].id == target.node_id

def test_addressing_by_id_is_parser_agnostic(py_file):
    _, tree = core.load(py_file)
    target = next((loc for loc in core.locate_all(tree) if loc.node_id == 'bar'))
    hits = ast_find(paths=[py_file], id=target.node_id).files[0].nodes
    assert hits and hits[0].id == 'bar'

def test_list_filters_by_type(py_file):
    result = ast_find(paths=[py_file], node_type='FunctionDef').files[0].nodes
    assert len(result) >= 1
    assert all((n.type == 'FunctionDef' for n in result))
'# --------------------------------------------------------------------------- #'
'# Cache reuse'
'# --------------------------------------------------------------------------- #'

def test_cache_returns_same_tree_until_changed(py_file):
    path = Path(py_file)
    first = core.CACHE.get_tree(path)
    assert core.CACHE.get_tree(path) is first
    core.CACHE.invalidate(path)
    assert core.CACHE.get_tree(path) is not first

def test_cache_is_shared_across_engines(py_file, json_file):
    _, py_tree = core.load(py_file)
    _, json_tree = core.load(json_file)
    assert py_tree.engine.name == 'python'
    assert json_tree.engine.name == 'tree-sitter:json'
    '# Same cache instance serves both.'
    assert core.CACHE.get_tree(Path(py_file)) is py_tree
    assert core.CACHE.get_tree(Path(json_file)) is json_tree
'# --------------------------------------------------------------------------- #'
'# create across engines'
'# --------------------------------------------------------------------------- #'

def test_create_file_typescript(tmp_path: Path):
    ts = tmp_path / 'app.ts'
    ast_create(str(ts), 'function greet(name: string): string {\n  return name;\n}\n')
    hits = ast_find(paths=[str(ts)], node_type='function_declaration').files[0].nodes
    assert hits and hits[0].id == 'greet'
    assert ast_validate([str(ts)]).all_ok