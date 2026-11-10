"""Tests for ``addImport``/``removeImport`` handling of the node-editing ``ast_*`` tools."""
from __future__ import annotations
import sys
from pathlib import Path
import pytest
_SRC = Path(__file__).resolve().parents[1] / 'src'
if _SRC.is_dir() and str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))
'# noqa: E402'
from xy.ai.mcpc.tools.ast import core
'# noqa: E402'
from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete
'# noqa: E402'
from xy.ai.mcpc.tools.ast.find import ast_find
'# noqa: E402'
from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, normalize_import, statement_spans
'# noqa: E402'
from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
'# noqa: E402'
from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace
PY_SOURCE = '"""Doc."""\nimport os\n\n\nclass A:\n    pass\n'
JS_SOURCE = "import a from 'a';\nimport {b} from 'b';\n\nfunction f() {\n  return 1;\n}\n"
GO_SOURCE = 'package main\n\nimport "fmt"\n\nfunc main() {\n\tfmt.Println(1)\n}\n'

def _write(tmp_path: Path, name: str, source: str) -> str:
    path = tmp_path / name
    path.write_text(source, encoding='utf-8')
    return str(path)

def _first_id(path: str, node_type: str) -> str:
    return ast_find(paths=[path], node_type=node_type).files[0].nodes[0].id
'# --------------------------------------------------------------------------- helpers'

def test_normalize_import_ignores_whitespace_quotes_and_semicolon():
    assert normalize_import('import  { a ,b }  from "x" ;') == normalize_import("import {a, b} from 'x'")
    assert normalize_import('use a::{b, c,};') == normalize_import('use a::{b,c};')

def test_statement_spans_skip_comments_and_braces():
    data = b'// x; y\nuse a::{b, c};\n/* z; */ use d;\n'
    assert [data[s:e] for s, e in statement_spans(data)] == [b'use a::{b, c};', b'use d;']
'# --------------------------------------------------------------------------- python'

def test_python_adds_missing_imports_after_existing_ones(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    result = ast_insert(
        items=[
            InsertItem(
                path=path,
                source='z = 5',
                id='A',
                add_import=[
                    'import sys',
                    'import os',
                    'from os import path'])])
    assert not result.errors and result.notice is None
    assert result.results[0].imports.added == ['import sys', 'from os import path']
    text = Path(path).read_text()
    assert text.index('import os') < text.index('import sys') < text.index(
        'from os import path') < text.index('class A')
    assert text.startswith('"""Doc."""')

def test_python_add_is_idempotent(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    for _ in range(2):
        ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['import sys'])])
    assert Path(path).read_text().count('import sys') == 1

def test_python_remove_import_and_single_alias(tmp_path: Path):
    path = _write(tmp_path, 'm.py', '"""Doc."""\nimport os, sys\nfrom a import b, c\n\n\nclass A:\n    pass\n')
    result = ast_insert(
        items=[
            InsertItem(
                path=path,
                source='z = 5',
                id='A',
                remove_import=[
                    'import sys',
                    'from a import c',
                    'import missing'])])
    assert result.results[0].imports.removed == ['import sys', 'from a import c']
    text = Path(path).read_text()
    assert 'import os' in text and 'from a import b' in text
    assert 'sys' not in text and 'import c' not in text

def test_python_future_import_goes_first(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['from __future__ import annotations'])])
    text = Path(path).read_text()
    assert text.index('from __future__') < text.index('import os')

def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])
    assert not result.errors
    assert result.notice == IMPORT_NOTICE
    text = Path(path).read_text()
    assert 'z = 5' in text and 'x = 1' not in text

def test_delete_with_imports_removes_unused_import(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    result = ast_delete(items=[DeleteItem(path=path, id='A', remove_import=['import os'])])
    assert not result.errors
    assert 'import os' not in Path(path).read_text()

def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):
    path = _write(tmp_path, 'm.py', PY_SOURCE)
    result = ast_delete(items=[DeleteItem(path=path, add_import=['import sys'])])
    assert not result.errors
    assert result.notice == IMPORT_NOTICE
    assert not Path(path).exists()
'# --------------------------------------------------------------------------- javascript'

def test_javascript_add_and_remove(tmp_path: Path):
    path = _write(tmp_path, 'm.js', JS_SOURCE)
    node = _first_id(path, 'function_declaration')
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source='function f() {\n  return 2;\n}',
                id=node,
                add_import=[
                    'import { c } from "c";',
                    'import a from "a"'],
                remove_import=["import {b} from 'b';"])])
    assert not result.errors and result.notice is None
    assert result.results[0].imports.added == ['import { c } from "c";']
    assert result.results[0].imports.removed == ["import {b} from 'b';"]
    text = Path(path).read_text()
    assert text.startswith('import a from \'a\';\nimport { c } from "c";\n')
    assert 'return 2' in text and "from 'b'" not in text

def test_javascript_without_imports_inserts_after_directive(tmp_path: Path):
    path = _write(tmp_path, 'm.js', "'use strict';\n\nfunction f() {\n  return 1;\n}\n")
    node = _first_id(path, 'function_declaration')
    ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source='function f() {\n  return 2;\n}',
                id=node,
                add_import=["import a from 'a';"])])
    text = Path(path).read_text()
    assert text.index('use strict') < text.index('import a') < text.index('function f')

def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):
    path = _write(tmp_path, 'm.js', JS_SOURCE)
    node = _first_id(path, 'function_declaration')
    result = ast_replace(items=[ReplaceItem(path=path, source='function f() {}', id=node, add_import=['const a = 1;'])])
    assert not result.errors
    assert result.notice == IMPORT_NOTICE
    text = Path(path).read_text()
    assert 'function f() {}' in text and 'const a' not in text

def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):
    path = _write(tmp_path, 'm.js', JS_SOURCE)
    node = _first_id(path, 'function_declaration')
    engine = core.load(path)[1].engine

    def broken(tree, add, remove):
        raise RuntimeError('boom')
    monkeypatch.setattr(engine, 'apply_imports', broken)
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source='function f() {\n  return 2;\n}',
                id=node,
                add_import=["import c from 'c';"])])
    assert not result.errors
    assert result.notice == IMPORT_NOTICE
    assert Path(path).read_text() == JS_SOURCE.replace('return 1', 'return 2')
'# --------------------------------------------------------------------------- generic fallback'

def test_generic_present_import_needs_no_notice(tmp_path: Path):
    path = _write(tmp_path, 'm.go', GO_SOURCE)
    node = core.locate_all(core.load(path)[1])[0]
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source=node.node.text.decode(),
                id=node.node_id,
                add_import=['import "fmt"'])])
    assert not result.errors and result.notice is None

def test_generic_missing_import_is_not_written_but_flagged(tmp_path: Path):
    path = _write(tmp_path, 'm.go', GO_SOURCE)
    node = core.locate_all(core.load(path)[1])[0]
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source=node.node.text.decode(),
                id=node.node_id,
                add_import=['import "os"'])])
    assert not result.errors
    assert result.notice == IMPORT_NOTICE
    assert 'import "os"' not in Path(path).read_text()

def test_generic_removes_whole_line_only(tmp_path: Path):
    path = _write(tmp_path, 'm.go', GO_SOURCE)
    node = core.locate_all(core.load(path)[1])[0]
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source=node.node.text.decode(),
                id=node.node_id,
                remove_import=['import "fmt"'])])
    assert not result.errors and result.notice is None
    assert 'import "fmt"' not in Path(path).read_text()

def test_generic_unsafe_remove_is_flagged(tmp_path: Path):
    path = _write(tmp_path, 'm.go', GO_SOURCE)
    node = core.locate_all(core.load(path)[1])[0]
    result = ast_replace(
        items=[
            ReplaceItem(
                path=path,
                source=node.node.text.decode(),
                id=node.node_id,
                remove_import=['fmt.Println'])])
    assert result.notice == IMPORT_NOTICE
    assert 'fmt.Println(1)' in Path(path).read_text()

@pytest.mark.parametrize('bad', [[''], 'import os', [1]])
def test_schema_level_validation_rejects_malformed_values(bad):
    from xy.ai.mcpc.tools.ast.imports import parse_import_args
    with pytest.raises(ValueError):
        parse_import_args({'addImport': bad})