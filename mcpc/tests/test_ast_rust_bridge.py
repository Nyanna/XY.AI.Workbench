"""Eigenständiger Test für die Rust-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._rust``).

Startet den echten ``xy_ai_ast_rust``-Prozess lazy und prüft Parse/Serialize/
Locate/Mutate/Validate end-to-end.
"""
from __future__ import annotations
import pytest
from xy.ai.mcpc.tools.ast.rpc._rust import RustEngine
SOURCE = 'struct Point {\n    x: i32,\n    y: i32,\n}\n\nimpl Point {\n    fn new(x: i32, y: i32) -> Self {\n        Point { x, y }\n    }\n\n    fn sum(&self) -> i32 {\n        self.x + self.y\n    }\n}\n\nfn main() {\n    let p = Point::new(1, 2);\n    println!("{}", p.sum());\n}\n'

@pytest.fixture(scope='module')
def engine() -> RustEngine:
    return RustEngine()

def test_parse_and_serialize_roundtrip(engine: RustEngine) -> None:
    tree = engine.parse(SOURCE)
    assert engine.serialize(tree) == SOURCE

def test_locate_all_finds_struct_impl_and_fn(engine: RustEngine) -> None:
    tree = engine.parse(SOURCE)
    located = engine.locate_all(tree)
    names = {loc.name for loc in located}
    assert 'Point' in names
    assert 'main' in names
    assert 'sum' in names

def test_replace_mutates_and_stays_valid(engine: RustEngine) -> None:
    tree = engine.parse(SOURCE)
    located = engine.locate_all(tree)
    sum_fn = next((loc for loc in located if loc.name == 'sum'))
    engine.replace(sum_fn, 'fn sum(&self) -> i32 {\n        self.x * self.y\n    }')
    mutated = engine.serialize(tree)
    assert 'self.x * self.y' in mutated
    assert engine.validate(mutated) is None

def test_validate_reports_syntax_error(engine: RustEngine) -> None:
    error = engine.validate('fn main( {')
    assert error is not None