"""Tests for the differentiated text-match diagnostics.

Covers the shared matcher (``tools._text_match``) and the four edit tools that
build on it: ``edit_block``, ``edit_marks`` (plain files) and
``ast.edit_block``/``ast.edit_marks`` (AST nodes). A failed match now reports a
machine-readable ``reason`` plus, depending on the case, a verified
``corrected_text``, an unverified ``guess``, a marker ``position``, or a
``next_step`` fallback instruction.
"""
from __future__ import annotations
import pytest
from xy.ai.mcpc.tools._text_match import TextAmbiguous, TextMatchError, TextNotFound, TextOrderError, line_preserving, replace_between, replace_in_block
from xy.ai.mcpc.tools.edit_block import EditBlockItem, edit_block
from xy.ai.mcpc.tools.edit_marks import EditMarksItem, edit_marks
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.edit_block import EditBlockItem as AstEditBlockItem, ast_edit_block
'# ---------------------------------------------------------------------------'
'# replace_in_block'
'# ---------------------------------------------------------------------------'

def test_replace_in_block_exact_match_succeeds():
    assert replace_in_block('a = 1\n', 'a = 1', 'a = 2', exact=False) == 'a = 2\n'

def test_replace_in_block_tolerates_whitespace_kind():
    block = 'x = 1;\n\tfoo\t=\tbar();\n'
    result = replace_in_block(block, 'foo = bar();', 'foo = baz();', exact=False, max_level=2)
    assert 'foo = baz();' in result

def test_replace_in_block_exact_rejects_whitespace_difference():
    with pytest.raises(TextNotFound):
        replace_in_block('x  =  1;\n', 'x = 1;', 'x = 2;', exact=True)

def test_replace_in_block_whitespace_mismatch_reports_corrected_text():
    block = 'foo();\n    x = 1;\n    bar();\n'
    with pytest.raises(TextNotFound) as ei:
        replace_in_block(block, 'x=1;', 'x=2;', exact=False, max_level=2, where='node')
    err = ei.value
    assert err.reason == 'whitespace_mismatch'
    assert err.corrected_text == 'x = 1;'
    assert err.guess is None
    assert err.next_step is None

def test_replace_in_block_content_changed_reports_guess():
    old = 'claudeCode = new CCConnector(cfg, sessionManager);'
    block = 'void m(){\n    claudeCode = new CCConnector(cfg, this.sessionManager);\n}\n'
    with pytest.raises(TextNotFound) as ei:
        replace_in_block(block, old, 'X', exact=False, max_level=2, where='node')
    err = ei.value
    assert err.reason == 'content_changed'
    assert err.guess == '    claudeCode = new CCConnector(cfg, this.sessionManager);'
    assert err.corrected_text is None

def test_replace_in_block_guard_rejected_when_accept_declines():
    block = 'a = 1;\n'
    with pytest.raises(TextNotFound) as ei:
        replace_in_block(block, 'a = 1;', 'a = 2;', exact=False, accept=lambda span, result: False, where='node')
    assert ei.value.reason == 'guard_rejected'

def test_replace_in_block_line_preserving_guard_rejects_merge():
    old = 'foo bar;'
    block = 'x\nfoo\nbar;\ny\n'
    with pytest.raises(TextNotFound) as ei:
        replace_in_block(block, old, 'X', exact=False, max_level=2, where='node', accept=line_preserving(old))
    assert ei.value.reason == 'guard_rejected'

def test_replace_in_block_unrelated_content_gives_reread_hint():
    old = 'claudeCode = new CCConnector(cfg, sessionManager);'
    with pytest.raises(TextNotFound) as ei:
        replace_in_block('completely unrelated content\n', old, 'X', exact=False, where='node')
    err = ei.value
    assert err.reason == 'not_found'
    assert err.next_step == 'reread_node'
    assert err.guess is None
    assert err.corrected_text is None

def test_replace_in_block_ambiguous_reports_count():
    block = 'x = 1;\nx = 1;\n'
    with pytest.raises(TextAmbiguous) as ei:
        replace_in_block(block, 'x = 1;', 'x = 2;', exact=False)
    assert ei.value.reason == 'ambiguous'
    assert ei.value.count == 2
'# ---------------------------------------------------------------------------'
'# replace_between'
'# ---------------------------------------------------------------------------'

def test_replace_between_basic():
    assert replace_between('pre BEGIN middle END post', 'BEGIN', 'END', 'NEW', exact=False) == 'pre NEW post'

def test_replace_between_marker_interior_tab_and_newline_interchangeable():
    marker_as_given = 'start of\tblock'
    actual_source = 'xxx start of\nblock yyy END here'
    result = replace_between(actual_source, marker_as_given, 'END', 'MID', exact=False)
    assert result == 'xxx MID here'

def test_replace_between_start_not_found_reports_position():
    with pytest.raises(TextNotFound) as ei:
        replace_between('nothing relevant here', 'START', 'END', 'X', exact=False)
    err = ei.value
    assert err.reason == 'not_found'
    assert err.position == 'start'

def test_replace_between_end_not_found_reports_position():
    with pytest.raises(TextNotFound) as ei:
        replace_between('has START but no closer', 'START', 'CLOSER_TAG', 'X', exact=False)
    err = ei.value
    assert err.reason == 'not_found'
    assert err.position == 'end'

def test_replace_between_end_whitespace_mismatch_reports_corrected_text():
    block = 'START stuff\n    x = 1;\n    END'
    with pytest.raises(TextNotFound) as ei:
        replace_between(block, 'START', 'x=1;', 'X', exact=False)
    err = ei.value
    assert err.reason == 'whitespace_mismatch'
    assert err.position == 'end'
    assert err.corrected_text == 'x = 1;'

def test_replace_between_marker_order_violation():
    with pytest.raises(TextOrderError) as ei:
        replace_between('AAA END middle START BBB', 'START', 'END', 'X', exact=False)
    assert ei.value.reason == 'marker_order'

def test_replace_between_start_ambiguous():
    with pytest.raises(TextAmbiguous):
        replace_between('START a END START b END', 'START', 'END', 'X', exact=False)
'# ---------------------------------------------------------------------------'
'# tools.edit_block / tools.edit_marks (plain files)'
'# ---------------------------------------------------------------------------'

def test_edit_block_tool_success_unaffected(tmp_path):
    f = tmp_path / 'sample.txt'
    f.write_text('x = 1;\n')
    result = edit_block([EditBlockItem(path=str(f), old_text='x = 1;', new_text='x = 2;')])
    assert not result.errors
    assert f.read_text() == 'x = 2;\n'

def test_edit_block_tool_reports_whitespace_mismatch(tmp_path):
    f = tmp_path / 'sample.txt'
    original = 'foo();\n    x = 1;\n    bar();\n'
    f.write_text(original)
    result = edit_block([EditBlockItem(path=str(f), old_text='x=1;', new_text='x=2;')])
    assert not result.results
    err = result.errors[0]
    assert err.reason == 'whitespace_mismatch'
    assert err.corrected_text == 'x = 1;'
    assert f.read_text() == original

def test_edit_block_tool_reports_content_changed_guess(tmp_path):
    f = tmp_path / 'sample.txt'
    old = 'claudeCode = new CCConnector(cfg, sessionManager);'
    f.write_text('void m(){\n    claudeCode = new CCConnector(cfg, this.sessionManager);\n}\n')
    result = edit_block([EditBlockItem(path=str(f), old_text=old, new_text='X;')])
    err = result.errors[0]
    assert err.reason == 'content_changed'
    assert 'this.sessionManager' in err.guess

def test_edit_marks_tool_reports_end_marker_position(tmp_path):
    f = tmp_path / 'sample.txt'
    f.write_text('BEGIN middle here, no closing tag')
    result = edit_marks([EditMarksItem(path=str(f), begin_marker='BEGIN middl', end_marker='CLOSINGTAG', content='X')])
    err = result.errors[0]
    assert err.reason == 'not_found'
    assert err.position == 'end'
    assert err.next_step == 'reread_node'
'# ---------------------------------------------------------------------------'
'# ast.edit_block (node-scoped edits)'
'# ---------------------------------------------------------------------------'

def _function_node_id(path) -> str:
    tree = core.CACHE.get_tree(path)
    return next((loc.node_id for loc in core.locate_all(tree) if loc.node_type == 'FunctionDef'))

def test_ast_edit_block_success_unaffected(tmp_path):
    f = tmp_path / 'mod.py'
    f.write_text('def target():\n    x = 1\n    return x\n')
    node_id = _function_node_id(f)
    result = ast_edit_block([AstEditBlockItem(path=str(f), old_text='x = 1', new_text='x = 2', id=node_id)])
    assert not result.errors
    assert 'x = 2' in f.read_text()

def test_ast_edit_block_reports_whitespace_mismatch(tmp_path):
    f = tmp_path / 'mod.py'
    f.write_text('def target():\n    foo()\n    x = 1\n    bar()\n')
    node_id = _function_node_id(f)
    result = ast_edit_block([AstEditBlockItem(path=str(f), old_text='x=1', new_text='x=2', id=node_id)])
    assert not result.results
    err = result.errors[0]
    assert err.reason == 'whitespace_mismatch'
    assert err.corrected_text == 'x = 1'

def test_ast_edit_block_reports_content_changed_guess(tmp_path):
    f = tmp_path / 'mod.py'
    old = 'result = compute(cfg, session)'
    f.write_text('def target():\n    result = compute(cfg, other_session)\n    return result\n')
    node_id = _function_node_id(f)
    result = ast_edit_block([AstEditBlockItem(path=str(f), old_text=old, new_text='X', id=node_id)])
    err = result.errors[0]
    assert err.reason == 'content_changed'
    assert 'other_session' in err.guess