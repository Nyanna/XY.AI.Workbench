"""Tests for the ``ast_move`` tool."""
from __future__ import annotations
import sys
from pathlib import Path
_SRC = Path(__file__).resolve().parents[1] / 'src'
if _SRC.is_dir() and str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))
'# noqa: E402'
from xy.ai.mcpc.tools.ast import core
'# noqa: E402'
from xy.ai.mcpc.tools.ast.edit import EditTool
'# noqa: E402'
from xy.ai.mcpc.tools.ast.move import MoveItem, MoveTool, ast_move
SRC = 'import os\n\n\ndef foo():\n    return os.sep\n\n\ndef bar():\n    return 1\n'
CLASSES = 'class A:\n\n    def m(self):\n        return 1\n\n    def n(self):\n        return 2\n\n    class Inner:\n\n        def x(self):\n            return 3\n\n\nclass B:\n\n    def k(self):\n        return 4\n'

def _write(tmp_path: Path, name: str, source: str) -> str:
    path = tmp_path / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding='utf-8')
    return str(path)

def _ids(path: str) -> list[str]:
    return [loc.node_id for loc in core.locate_all(core.CACHE.get_tree(Path(path)))]

def _top_ids(path: str) -> list[str]:
    return [i for i in _ids(path) if '.' not in i]

def test_move_to_new_file_creates_it_and_handles_imports(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = str(tmp_path / 'sub' / 'b.py')
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], add_import=['import os'])])
    assert not result.errors
    res, = result.results
    assert res.moved == [('foo', 'foo')]
    text = Path(dst).read_text()
    assert 'def foo' in text and 'import os' in text
    assert 'def foo' not in Path(src).read_text()
    assert res.to_dict()['moved'] == [{'old': 'foo', 'new': 'foo'}]
    assert res.to_dict()['target'] == dst

def test_emptied_source_file_is_deleted(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = _write(tmp_path, 'b.py', 'X = 1\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo', 'bar'], source_remove_import=['import os'])])
    assert not result.errors
    assert result.results[0].source_deleted
    assert not Path(src).exists()
    text = Path(dst).read_text()
    assert text.index('def foo') < text.index('def bar')

def test_result_contains_only_paths_and_moved_ids(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = str(tmp_path / 'b.py')
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo', 'bar'],
                      add_import=['import os'], source_remove_import=['import os'])])
    assert not result.errors
    assert result.results[0].source_deleted
    assert result.results[0].to_dict() == {'path': src, 'target': dst, 'moved': [
        {'old': 'foo', 'new': 'foo'}, {'old': 'bar', 'new': 'bar'}]}

def _code(path: str, node_id: str) -> str:
    tree = core.CACHE.get_tree(Path(path))
    return next((core.edit_node_source(loc) for loc in core.locate_all(tree) if loc.node_id == node_id))

def test_several_nodes_after_target_keep_order_and_exact_ids(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=['bar', 'foo'], target_id='first')])
    assert not result.errors
    assert result.results[0].moved == [('bar', 'bar'), ('foo', 'foo')]
    assert _top_ids(dst) == ['first', 'bar', 'foo', 'last']

def test_name_clash_in_target_is_attributed_to_the_inserted_node(tmp_path: Path):
    src = _write(tmp_path, 'a.py', 'def foo():\n    return 42\n')
    dst = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='foo', position='before')])
    assert not result.errors
    (old, new), = result.results[0].moved
    assert old == 'foo' and isinstance(new, str)
    assert 'return 42' in _code(dst, new)

def test_insert_reports_inserted_id_on_name_clash(tmp_path: Path):
    from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
    path = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
    result = ast_insert([InsertItem(path=path, id='foo', position='before', source='def foo():\n    return 42\n')])
    assert not result.errors
    assert result.results[0].ids == ['foo']
    assert 'return 42' in _code(path, 'foo')

def test_insert_into_class_reports_only_the_new_nodes(tmp_path: Path):
    from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
    path = _write(tmp_path, 'c.py', CLASSES)
    source = 'def p(self):\n    return 5\n\ndef q(self):\n    return 6\n'
    result = ast_insert([InsertItem(path=path, id='B.k', source=source)])
    assert not result.errors
    assert result.results[0].ids == ['B.p', 'B.q']

def test_unknown_source_id_is_error_but_others_move(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = _write(tmp_path, 'b.py', 'X = 1\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=['nope', 'bar'])])
    assert [e.id for e in result.errors] == ['nope']
    assert result.results[0].moved == [('bar', 'bar')]
    assert 'def bar' in Path(dst).read_text()

def test_unknown_target_id_aborts_without_writing(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = _write(tmp_path, 'b.py', 'X = 1\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='missing')])
    assert not result.results
    assert len(result.errors) == 1 and result.errors[0].id is None
    assert Path(src).read_text() == SRC
    assert Path(dst).read_text() == 'X = 1\n'
    assert 'foo' in _ids(src)

def test_move_method_between_classes_in_same_file(tmp_path: Path):
    path = _write(tmp_path, 'c.py', CLASSES)
    result = ast_move([MoveItem(path=path, target=path, ids=['A.m'], target_id='B.k', position='before')])
    assert not result.errors
    assert result.results[0].moved == [('A.m', 'B.m')]
    ids = _ids(path)
    assert 'A.m' not in ids
    assert ids.index('B.m') < ids.index('B.k')

def test_target_inside_moved_node_is_rejected(tmp_path: Path):
    path = _write(tmp_path, 'c.py', CLASSES)
    result = ast_move([MoveItem(path=path, target=path, ids=['A'], target_id='A.n')])
    assert result.errors and (not result.results)
    assert Path(path).read_text() == CLASSES

def test_inner_class_moves_by_its_own_id(tmp_path: Path):
    path = _write(tmp_path, 'c.py', CLASSES)
    dst = str(tmp_path / 'd.py')
    result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner'])])
    assert not result.errors
    assert result.results[0].moved == [('A.Inner', 'Inner')]
    assert 'A.Inner' not in _ids(path)
    assert 'Inner.x' in _ids(dst)

def test_moving_only_children_keeps_container(tmp_path: Path):
    path = _write(tmp_path, 'c.py', CLASSES)
    dst = str(tmp_path / 'd.py')
    result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner.x'])])
    assert not result.errors
    assert 'A.Inner' in _ids(path)
    assert 'x' in _ids(dst)

def test_nested_id_moves_along_with_ancestor(tmp_path: Path):
    path = _write(tmp_path, 'c.py', CLASSES)
    dst = str(tmp_path / 'd.py')
    result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner.x', 'A.Inner'])])
    assert not result.errors
    assert dict(result.results[0].moved) == {'A.Inner': 'Inner', 'A.Inner.x': 'Inner.x'}

def test_whole_file_to_new_target(tmp_path: Path):
    src = _write(tmp_path, 'pkg/a.py', SRC)
    dst = str(tmp_path / 'other' / 'b.py')
    result = ast_move([MoveItem(path=src, target=dst)])
    assert not result.errors
    res = result.results[0]
    assert res.source_deleted
    assert not Path(src).exists() and (not Path(src).parent.exists())
    assert 'def foo' in Path(dst).read_text()
    assert ('foo', 'foo') in res.moved

def test_whole_file_into_existing_target_after_node(tmp_path: Path):
    src = _write(tmp_path, 'a.py', 'def foo():\n    return 1\n')
    dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
    result = ast_move([MoveItem(path=src, target=dst, target_id='first')])
    assert not result.errors
    assert result.results[0].moved == [('foo', 'foo')]
    assert _top_ids(dst) == ['first', 'foo', 'last']
    assert not Path(src).exists()

def test_whole_file_onto_itself_fails(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    result = ast_move([MoveItem(path=src, target=src)])
    assert result.errors and Path(src).read_text() == SRC

def test_javascript_cross_file_move(tmp_path: Path):
    src = _write(tmp_path, 'a.js', 'function foo() {\n  return 1;\n}\n\nfunction bar() {\n  return 2;\n}\n')
    dst = _write(tmp_path, 'b.js', 'function baz() {\n  return 3;\n}\n')
    result = ast_move([MoveItem(path=src, target=dst, ids=[i for i in _ids(src) if 'bar' in i][:1])])
    assert not result.errors
    assert 'function bar' in Path(dst).read_text()
    assert 'function bar' not in Path(src).read_text()
    assert result.results[0].moved[0][1] is not None

def test_batch_and_edit_integration(tmp_path: Path):
    src = _write(tmp_path, 'a.py', SRC)
    dst = str(tmp_path / 'b.py')
    assert 'move' in EditTool.input_schema['properties']
    assert EditTool.input_schema['properties']['move'] == MoveTool.input_schema['properties']['items']
    result = ast_move([MoveItem(path=src, target=dst, ids=['foo']), MoveItem(
        path=src, target=dst, ids=['bar'], target_id='foo', position='before')])
    assert not result.errors
    assert _top_ids(dst) == ['bar', 'foo']