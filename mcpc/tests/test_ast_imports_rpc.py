"""Import handling against the real Java (JavaParser) and Rust (syn) RPC processes."""
from __future__ import annotations
import sys
from pathlib import Path
import pytest
_SRC = Path(__file__).resolve().parents[1] / 'src'
if _SRC.is_dir() and str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))
'# noqa: E402'
from xy.ai.mcpc.tools.ast.base import AstError
'# noqa: E402'
from xy.ai.mcpc.tools.ast.generic import get_engine
JAVA = 'package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic class Foo {\n    void bar() {}\n}\n'
RUST = 'use std::fmt;\nuse std::io::{self, Read};\n\nstruct P {\n    x: i32,\n}\n'

@pytest.fixture(scope='module')
def java():
    return get_engine('java')

@pytest.fixture(scope='module')
def rust():
    return get_engine('rust')

def test_java_add_present_and_remove(java):
    tree = java.parse(JAVA)
    outcome = java.apply_imports(tree, ['import java.util.Set;', 'import java.util.List'], ['import java.util.Map;'])
    assert outcome.added == ['import java.util.Set;']
    assert outcome.removed == ['import java.util.Map;']
    assert not outcome.unchecked
    assert 'import java.util.Set;' in tree.source and 'java.util.Map' not in tree.source
    assert tree.source.count('import java.util.List;') == 1
    assert java.validate(tree.source) is None

def test_java_without_imports_goes_behind_package(java):
    tree = java.parse('package p;\n\npublic class Foo {\n}\n')
    outcome = java.apply_imports(tree, ['import java.util.Set;'], [])
    assert outcome.added == ['import java.util.Set;']
    assert tree.source.index('package p;') < tree.source.index('import java.util.Set;') < tree.source.index('class Foo')

def test_java_without_package_and_imports(java):
    tree = java.parse('public class Foo {\n}\n')
    java.apply_imports(tree, ['import static java.lang.Math.max'], [])
    assert tree.source.startswith('import static java.lang.Math.max;')
    assert java.validate(tree.source) is None

def test_java_non_import_raises_and_leaves_source(java):
    tree = java.parse(JAVA)
    with pytest.raises(AstError):
        java.apply_imports(tree, ['int x;'], [])
    assert tree.source == JAVA

def test_rust_add_present_and_remove(rust):
    tree = rust.parse(RUST)
    outcome = rust.apply_imports(tree, ['use std::collections::HashMap;',
                                 'use std::fmt'], ['use std::io::{self, Read};'])
    assert outcome.added == ['use std::collections::HashMap;']
    assert outcome.removed == ['use std::io::{self, Read};']
    assert tree.source.count('use std::fmt;') == 1
    assert 'std::io' not in tree.source
    assert rust.validate(tree.source) is None

def test_rust_without_imports(rust):
    tree = rust.parse('struct P {\n    x: i32,\n}\n')
    outcome = rust.apply_imports(tree, ['use std::fmt;'], [])
    assert outcome.added == ['use std::fmt;']
    assert tree.source.index('use std::fmt;') < tree.source.index('struct P')

def test_rust_non_import_raises_and_leaves_source(rust):
    tree = rust.parse(RUST)
    with pytest.raises(AstError):
        rust.apply_imports(tree, ['fn x() {}'], [])
    assert tree.source == RUST

def test_rust_keeps_surrounding_formatting(rust):
    tree = rust.parse(RUST)
    rust.apply_imports(tree, ['use std::collections::HashMap;'], [])
    assert tree.source == 'use std::fmt;\nuse std::io::{self, Read};\nuse std::collections::HashMap;\n\nstruct P {\n    x: i32,\n}\n'

def test_rust_import_does_not_split_doc_comment_from_item(rust):
    tree = rust.parse('//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n')
    rust.apply_imports(tree, ['use std::fmt;'], [])
    assert tree.source.index('//! crate') < tree.source.index('use std::fmt;') < tree.source.index('/// Point.')

def test_java_keeps_surrounding_formatting(java):
    tree = java.parse(JAVA)
    java.apply_imports(tree, ['import java.util.Set;'], [])
    assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Map;\nimport java.util.Set;\n')

def test_java_add_and_remove_in_one_pass(java):
    tree = java.parse(JAVA)
    outcome = java.apply_imports(tree, ['import java.util.Set;'], ['import java.util.Map;'])
    assert outcome.added and outcome.removed
    assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Set;\n')