"""Tests for the multi-engine ``ast_*`` tool family.

Covers engine selection by file extension, the Python (``ast``) engine, the
generic tree-sitter engine, engine-independent node addressing (by id and by
type/name), and reuse/invalidation of the shared parse cache.
"""

import sys
from pathlib import Path

import pytest

# Make the ``src`` layout importable without requiring an editable install.
_SRC = Path(__file__).resolve().parents[1] / "src"
if _SRC.is_dir() and str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))

from xy.ai.mcpc.tools.ast import core  # noqa: E402
from xy.ai.mcpc.tools.ast.create_file import ast_create_file  # noqa: E402
from xy.ai.mcpc.tools.ast.delete import ast_delete  # noqa: E402
from xy.ai.mcpc.tools.ast.edit import ast_edit  # noqa: E402
from xy.ai.mcpc.tools.ast.find import ast_find  # noqa: E402
from xy.ai.mcpc.tools.ast.insert import ast_insert  # noqa: E402
from xy.ai.mcpc.tools.ast.list import ast_list  # noqa: E402
from xy.ai.mcpc.tools.ast.outline import ast_outline  # noqa: E402
from xy.ai.mcpc.tools.ast.read import ast_read  # noqa: E402
from xy.ai.mcpc.tools.ast.replace import ast_replace  # noqa: E402
from xy.ai.mcpc.tools.ast.validate import ast_validate  # noqa: E402


PY_SOURCE = (
    "import os\n"
    "\n"
    "class A:\n"
    "    def foo(self):\n"
    "        x = 1\n"
    "        return x\n"
    "\n"
    "def bar():\n"
    "    return 2\n"
)

JSON_SOURCE = '{\n  "name": "demo",\n  "deps": {\n    "a": 1,\n    "b": 2\n  }\n}\n'


@pytest.fixture
def py_file(tmp_path: Path) -> str:
    path = tmp_path / "m.py"
    path.write_text(PY_SOURCE, encoding="utf-8")
    return str(path)


@pytest.fixture
def json_file(tmp_path: Path) -> str:
    path = tmp_path / "c.json"
    path.write_text(JSON_SOURCE, encoding="utf-8")
    return str(path)


# --------------------------------------------------------------------------- #
# Engine selection
# --------------------------------------------------------------------------- #

def test_engine_selection_by_extension(tmp_path: Path):
    assert core.engine_for_path(tmp_path / "x.py").name == "python"
    assert core.engine_for_path(tmp_path / "x.pyi").name == "python"
    assert core.engine_for_path(tmp_path / "x.json").name == "tree-sitter:json"
    assert core.engine_for_path(tmp_path / "x.yaml").name == "tree-sitter:yaml"


def test_engine_selection_unsupported_extension(tmp_path: Path):
    with pytest.raises(core.AstError):
        core.engine_for_path(tmp_path / "x.unknownext")


def test_snippet_defaults_to_python():
    tree = core.tree_from_input(None, "def f():\n    return 1\n")
    assert tree.engine is core.python.ENGINE


# --------------------------------------------------------------------------- #
# Python engine
# --------------------------------------------------------------------------- #

def test_python_outline(py_file):
    nodes = ast_outline([py_file]).files[0].nodes
    kinds = [(n.type, n.qualified_name) for n in nodes]
    assert ("ClassDef", "A") in kinds
    assert ("FunctionDef", "bar") in kinds
    cls = next(n for n in nodes if n.qualified_name == "A")
    assert any(c.qualified_name == "A.foo" for c in cls.children)


def test_python_find_by_name_and_type(py_file):
    hits = ast_find(path=py_file, name="bar", node_type="FunctionDef").nodes
    assert [h.qualified_name for h in hits] == ["bar"]


def test_python_read_returns_source(py_file):
    node = ast_read(path=py_file, qualified_name="A.foo").node
    assert node.code is not None
    assert "return x" in node.code


def test_python_full_crud_roundtrip(py_file):
    ast_replace(py_file, "def bar():\n    return 42", qualified_name="bar")
    ast_edit(py_file, "x = 1", "return x", "return 99", qualified_name="A.foo")
    ast_insert(py_file, "z = 5", qualified_name="bar", position="after")
    ast_delete(py_file, node_type="Import")
    assert ast_validate([py_file]).all_ok

    text = Path(py_file).read_text()
    assert "return 42" in text
    assert "return 99" in text
    assert "z = 5" in text
    assert "import os" not in text


# --------------------------------------------------------------------------- #
# Generic tree-sitter engine
# --------------------------------------------------------------------------- #

def test_generic_uses_treesitter_engine(json_file):
    _, tree = core.load(json_file)
    assert tree.engine.name == "tree-sitter:json"


def test_generic_qualified_names(json_file):
    _, tree = core.load(json_file)
    qnames = {loc.qualified_name for loc in core.locate_all(tree)}
    assert {"name", "deps", "deps.a", "deps.b"} <= qnames


def test_generic_replace_by_qualified_name(json_file):
    ast_replace(json_file, '"a": 111', qualified_name="deps.a")
    assert '"a": 111' in Path(json_file).read_text()
    assert ast_validate([json_file]).all_ok


def test_generic_edit_between_markers(json_file):
    ast_edit(json_file, '"a": 1', '"b": 2', '"a": 10,\n    "b": 20', qualified_name="deps")
    text = Path(json_file).read_text()
    assert '"a": 10' in text and '"b": 20' in text
    assert ast_validate([json_file]).all_ok


def test_generic_validate_reports_error(tmp_path: Path):
    bad = tmp_path / "bad.json"
    bad.write_text('{"a": }\n', encoding="utf-8")
    check = ast_validate([str(bad)]).files[0]
    assert not check.ok
    assert check.error


# --------------------------------------------------------------------------- #
# Engine-independent node addressing
# --------------------------------------------------------------------------- #

def test_addressing_by_id_matches_qualified_name(json_file):
    _, tree = core.load(json_file)
    target = next(loc for loc in core.locate_all(tree) if loc.qualified_name == "deps.a")
    by_id = ast_find(path=json_file, id=target.node_id).nodes
    assert len(by_id) == 1
    assert by_id[0].qualified_name == "deps.a"


def test_addressing_by_id_is_parser_agnostic(py_file):
    _, tree = core.load(py_file)
    target = next(loc for loc in core.locate_all(tree) if loc.qualified_name == "bar")
    hits = ast_find(path=py_file, id=target.node_id).nodes
    assert hits and hits[0].qualified_name == "bar"


def test_list_filters_by_type(py_file):
    result = ast_list(path=py_file, node_type="FunctionDef")
    assert result.count >= 1
    assert all(n.type == "FunctionDef" for n in result.nodes)


# --------------------------------------------------------------------------- #
# Cache reuse
# --------------------------------------------------------------------------- #

def test_cache_returns_same_tree_until_changed(py_file):
    path = Path(py_file)
    first = core.CACHE.get_tree(path)
    assert core.CACHE.get_tree(path) is first

    core.CACHE.invalidate(path)
    assert core.CACHE.get_tree(path) is not first


def test_cache_is_shared_across_engines(py_file, json_file):
    _, py_tree = core.load(py_file)
    _, json_tree = core.load(json_file)
    assert py_tree.engine.name == "python"
    assert json_tree.engine.name == "tree-sitter:json"
    # Same cache instance serves both.
    assert core.CACHE.get_tree(Path(py_file)) is py_tree
    assert core.CACHE.get_tree(Path(json_file)) is json_tree


# --------------------------------------------------------------------------- #
# create_file across engines
# --------------------------------------------------------------------------- #

def test_create_file_typescript(tmp_path: Path):
    ts = tmp_path / "app.ts"
    ast_create_file(str(ts), "function greet(name: string): string {\n  return name;\n}\n")
    hits = ast_find(path=str(ts), node_type="function_declaration").nodes
    assert hits and hits[0].qualified_name == "greet"
    assert ast_validate([str(ts)]).all_ok
