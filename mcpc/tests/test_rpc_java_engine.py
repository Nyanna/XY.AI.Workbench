"""pytest tests for the RPC/PIC-based Java engine (xy.ai.mcpc.tools.ast.rpc).

Starts the real ``ast-java`` JavaParser subprocess lazily (module-scoped
fixture, so it is spawned once and reused); requires ``java``/``javac`` (or
``$JAVA_HOME``) and the prebuilt ``ast-engines/java/.bin`` classes.
"""
from __future__ import annotations
import sys
from pathlib import Path
import pytest
_SRC = Path(__file__).resolve().parents[1] / 'src'
if str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))
from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError
from xy.ai.mcpc.tools.ast.generic import get_engine
VALID_SOURCE = 'public class Foo {\n    void bar() {\n        System.out.println(1);\n    }\n}\n'

@pytest.fixture(scope='module')
def engine():
    return get_engine('java')

def test_validate_accepts_well_formed_source(engine):
    assert engine.validate(VALID_SOURCE) is None

def test_validate_reports_malformed_source(engine):
    error = engine.validate('public class Broken {')
    assert error

def test_parse_raises_ast_error_on_malformed_source(engine):
    with pytest.raises(AstError):
        engine.parse('public class Broken {')

def test_locate_all_reports_class_and_method(engine):
    tree = engine.parse(VALID_SOURCE)
    located = {loc.node_id: loc for loc in engine.locate_all(tree)}
    assert 'Foo' in located
    assert 'Foo.bar' in located
    foo = located['Foo']
    bar = located['Foo.bar']
    assert foo.node_type == 'ClassOrInterfaceDeclaration'
    assert foo.expandable is True
    assert bar.parent_type == 'ClassOrInterfaceDeclaration'
    assert engine.is_definition(foo.node_type)
    assert engine.signature(foo.node).startswith('public class Foo')
    assert engine.signature(bar.node) == 'void bar()'

def test_node_code_matches_source_slice(engine):
    tree = engine.parse(VALID_SOURCE)
    bar = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo.bar'))
    assert 'System.out.println(1)' in engine.node_code(bar.node)

def test_replace_updates_tree_source(engine):
    tree = engine.parse(VALID_SOURCE)
    bar = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo.bar'))
    engine.replace(bar, '    void bar() {\n        System.out.println(2);\n    }\n')
    assert 'println(2)' in tree.source
    assert 'println(1)' not in tree.source

def test_insert_before_adds_a_sibling_field(engine):
    tree = engine.parse(VALID_SOURCE)
    bar = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo.bar'))
    units = engine.insert(bar, 'int x;', 'before')
    assert units == 1
    assert 'int x;' in tree.source
    assert any((loc.node_type == 'FieldDeclaration' for loc in engine.locate_all(tree)))

def test_delete_removes_the_node(engine):
    tree = engine.parse(VALID_SOURCE)
    bar = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo.bar'))
    engine.delete(bar)
    assert 'void bar()' not in tree.source
    assert not any((loc.node_id == 'Foo.bar' for loc in engine.locate_all(tree)))

def test_append_adds_a_top_level_type(engine):
    tree = engine.parse(VALID_SOURCE)
    units = engine.append(tree, 'class Extra {}')
    assert units == 1
    assert 'class Extra' in tree.source
    assert any((loc.node_id == 'Extra' for loc in engine.locate_all(tree)))

def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
    """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
    tree = engine.parse(VALID_SOURCE)
    foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
    with pytest.raises(AstError) as excinfo:
        engine.insert(foo, 'int x;', 'before')
    assert not isinstance(excinfo.value, AstAmbiguous)