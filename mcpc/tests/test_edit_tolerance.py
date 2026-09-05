"""Tests for the shared tolerant text matcher and the four edit tools.

Covers the two hallucination failure modes that motivated the change:

* escaped ``\\n`` in an AST-unparsed single-line string literal vs. a needle /
  replacement carrying *real* newlines (escape tolerance + escape mirroring), and
* wrong string-literal quoting/escaping (quote/delimiter tolerance),

plus the safety rails: line-structure preservation for the parser-less plain-text
tools and the ``validates_syntax`` gate that reserves the aggressive level-3
tolerance for engines whose re-parse actually rejects corruption.
"""
from __future__ import annotations
import ast
from pathlib import Path
import pytest
from xy.ai.mcpc.tools import _text_match as tm
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.edit_block import ast_edit_block
from xy.ai.mcpc.tools.ast.edit_marks import ast_edit_marks
from xy.ai.mcpc.tools.edit_block import EditBlockError, edit_block
from xy.ai.mcpc.tools.edit_marks import EditMarksError, edit_marks, edit_marks_text
'# --------------------------------------------------------------------------- #'
'# _text_match: tolerance levels'
'# --------------------------------------------------------------------------- #'
'# A module docstring as ``ast.unparse`` renders it: one single-quoted line whose'
'# newlines are the two-char escape ``\\n`` rather than real line breaks.'
_ESCAPED_LITERAL = "'``web_fetch_exa`` - stage 1.\\n\\nFetches page content and caches each full result (incl. text and url) by id;\\nreturns only an overview with file_stats-style text metrics, no text/url.\\n'"
_REAL_NL_NEEDLE = 'Fetches page content and caches each full result (incl. text and url) by id;\nreturns only an overview with file_stats-style text metrics, no text/url.'

def test_level1_whitespace_tolerant():
    result = tm.replace_in_block('a   =\t1', 'a = 1', 'a = 2', exact=False, where='block')
    assert result == 'a = 2'

def test_exact_requires_verbatim_whitespace():
    with pytest.raises(tm.TextNotFound):
        tm.replace_in_block('a   = 1', 'a = 1', 'a = 2', exact=True, where='block')

def test_level2_matches_escaped_newlines():
    """# Real newlines in the needle match the literal ``\\n`` of the escaped literal."""
    out = tm.replace_in_block(_ESCAPED_LITERAL, _REAL_NL_NEEDLE, 'REPLACED', exact=False, where='node')
    assert 'REPLACED' in out

def test_level2_blocked_at_max_level_1():
    with pytest.raises(tm.TextNotFound):
        tm.replace_in_block(_ESCAPED_LITERAL, _REAL_NL_NEEDLE, 'REPLACED', exact=False, max_level=1, where='node')

def test_escape_mirroring_keeps_literal_valid():
    """# A replacement with *real* newlines must be re-escaped so the single-line"""
    '# literal stays syntactically valid Python.'
    new = 'Fetches (incl. text) by id; returns\nan overview with url, but no text.'
    out = tm.replace_in_block(_ESCAPED_LITERAL, _REAL_NL_NEEDLE, new, exact=False, where='node')
    '# would raise on an unterminated string literal'
    ast.parse(out)
    assert '\\n' in out
    assert '\n' not in out.strip("'")

def test_mirror_escaping_noop_on_real_newline_region():
    """# Region already has real newlines: leave the replacement untouched."""
    assert tm._mirror_escaping('a\nb', 'x\ny') == 'x\ny'
'# Case 2: hallucinated quoting/escaping around a double-quoted literal.'
_QUOTED = 'x = "#: the page\'s content; consecutive urls are"'
_HALLUCINATED = "'#: the page's content; consecutive urls are\\''"

def test_level3_absorbs_hallucinated_quotes():
    out = tm.replace_in_block(_QUOTED, _HALLUCINATED, 'R', exact=False, where='node')
    assert out == 'x = R'

def test_level3_blocked_at_max_level_2():
    with pytest.raises(tm.TextNotFound):
        tm.replace_in_block(_QUOTED, _HALLUCINATED, 'R', exact=False, max_level=2, where='node')

def test_ambiguous_match_raises():
    with pytest.raises(tm.TextAmbiguous):
        tm.replace_in_block('x x', 'x', 'y', exact=False, where='node')

def test_replace_all_replaces_every_occurrence():
    out = tm.replace_in_block('x x x', 'x', 'y', exact=False, replace_all=True, where='node')
    assert out == 'y y y'
'# --------------------------------------------------------------------------- #'
'# _text_match: line-structure guard (parser-less safety)'
'# --------------------------------------------------------------------------- #'

def test_line_preserving_rejects_merge_across_escaped_newline():
    """# Needle spans two lines but would match a single physical (escaped) line;"""
    '# the guard rejects it rather than risk merging lines into a syntax error.'
    guard = tm.line_preserving('foo();\nbar()')
    with pytest.raises(tm.TextNotFound):
        tm.replace_in_block('foo();\\nbar()', 'foo();\nbar()', 'X', exact=False, accept=guard, where='file')

def test_line_preserving_allows_same_line_count():
    guard = tm.line_preserving('a\nb')
    assert tm.replace_in_block('a\nb', 'a\nb', 'c\nd', exact=False, accept=guard, where='file') == 'c\nd'
'# --------------------------------------------------------------------------- #'
'# _text_match: markers'
'# --------------------------------------------------------------------------- #'

def test_replace_between_includes_markers():
    out = tm.replace_between('<<START>> junk <<END>>', '<<START>>', '<<END>>', 'NEW', exact=False, where='block')
    assert out == 'NEW'

def test_replace_between_requires_order():
    with pytest.raises(tm.TextMatchError):
        tm.replace_between('<<END>> x <<START>>', '<<START>>', '<<END>>', 'NEW', exact=False, where='block')
'# --------------------------------------------------------------------------- #'
'# Engine capability gate'
'# --------------------------------------------------------------------------- #'

def test_python_engine_validates_syntax():
    assert core.engine_for_path(Path('m.py')).validates_syntax is True

def test_markup_engine_does_not_validate_syntax():
    """# Tree-sitter markup grammars accept almost anything; level-3 must stay off."""
    assert core.engine_for_path(Path('c.json')).validates_syntax is False
'# --------------------------------------------------------------------------- #'
'# Plain-text tools'
'# --------------------------------------------------------------------------- #'

def _write(tmp_path: Path, name: str, text: str) -> str:
    path = tmp_path / name
    path.write_text(text, encoding='utf-8')
    return str(path)

def test_edit_block_whitespace_tolerant(tmp_path: Path):
    path = _write(tmp_path, 'm.py', 'value   =    1\n')
    edit_block(path, 'value = 1', 'value = 2')
    assert Path(path).read_text(encoding='utf-8') == 'value = 2\n'

def test_edit_block_ambiguous_raises(tmp_path: Path):
    path = _write(tmp_path, 'm.py', 'a = 1\na = 1\n')
    with pytest.raises(EditBlockError):
        edit_block(path, 'a = 1', 'a = 9')

def test_edit_block_not_found_raises(tmp_path: Path):
    path = _write(tmp_path, 'm.py', 'a = 1\n')
    with pytest.raises(EditBlockError):
        edit_block(path, 'nope', 'x')

def test_edit_block_replace_all(tmp_path: Path):
    path = _write(tmp_path, 'm.py', 'a = 1\na = 1\n')
    edit_block(path, 'a = 1', 'a = 9', replace_all=True)
    assert Path(path).read_text(encoding='utf-8') == 'a = 9\na = 9\n'

def test_edit_marks_text_parameter_order():
    """# Regression: signature is (text, begin_marker, content, end_marker); the"""
    '# content sits between the begin and end markers in the argument list.'
    out = edit_marks_text('<<A>> old <<B>>', '<<A>>', 'NEW', '<<B>>')
    assert out == 'NEW'

def test_edit_marks_end_to_end(tmp_path: Path):
    path = _write(tmp_path, 'm.txt', 'keep <<A>> drop <<B>> keep\n')
    edit_marks(path, '<<A>>', '<<B>>', 'X')
    assert Path(path).read_text(encoding='utf-8') == 'keep X keep\n'

def test_edit_marks_missing_marker_raises(tmp_path: Path):
    path = _write(tmp_path, 'm.txt', 'only <<A>> here\n')
    with pytest.raises(EditMarksError):
        edit_marks(path, '<<A>>', '<<MISSING>>', 'X')
'# --------------------------------------------------------------------------- #'
'# AST tools: the two motivating failure cases, live'
'# --------------------------------------------------------------------------- #'
_PROBE_SOURCE = '"""``web_fetch_exa`` - stage 1 of the two-stage Exa fetch retrieval.\n\nFetches page content and caches each full result (incl. text and url) by id;\nreturns only an overview with file_stats-style text metrics, no text/url.\nCall ``web_fetch_exa_results`` with the returned ids to resolve url and text.\n"""\n\n#: line, then the page\'s extracted markdown content; consecutive urls are\nNEXT = 1\n'

def _node_id(path: str, needle: str) -> str:
    _, tree = core.load(path)
    loc = next((loc for loc in core.locate_all(tree) if needle in core.edit_node_source(loc)))
    return loc.node_id

def test_ast_edit_block_case1_escaped_docstring(tmp_path: Path):
    path = _write(tmp_path, 'probe.py', _PROBE_SOURCE)
    node_id = _node_id(path, 'web_fetch_exa')
    res = ast_edit_block(
        path,
        'Fetches page content and caches each full result (incl. text and url) by id;\nreturns only an overview with file_stats-style text metrics, no text/url.',
        'Fetches page content and caches each full result (incl. text) by id; returns\nan overview with url and file_stats-style text metrics, but no text.',
        id=node_id)
    assert res.result == 'success'
    text = Path(path).read_text(encoding='utf-8')
    ast.parse(text)
    assert 'incl. text) by id; returns' in text

def test_ast_edit_block_case2_hallucinated_quotes(tmp_path: Path):
    path = _write(tmp_path, 'probe.py', _PROBE_SOURCE)
    node_id = _node_id(path, 'consecutive urls are')
    res = ast_edit_block(
        path,
        "'#: line, then the page's extracted markdown content; consecutive urls are\\''",
        '"#: line (optionally preceded by a \'Published:\' line); consecutive urls are"',
        id=node_id)
    assert res.result == 'success'
    text = Path(path).read_text(encoding='utf-8')
    ast.parse(text)
    assert 'Published:' in text

def test_ast_edit_block_rejects_corrupting_edit(tmp_path: Path):
    """# Even when a tolerant match succeeds, an edit that cannot re-parse must fail"""
    '# loudly instead of writing broken source.'
    path = _write(tmp_path, 'm.py', 'value = 1\n')
    node_id = _node_id(path, 'value = 1')
    with pytest.raises(core.AstError):
        ast_edit_block(path, 'value = 1', 'value = (', id=node_id)
    assert Path(path).read_text(encoding='utf-8') == 'value = 1\n'

def test_ast_edit_marks_between_markers(tmp_path: Path):
    path = _write(tmp_path, 'm.py', 'A = 1\nB = 2\nC = 3\n')
    node_id = _node_id(path, 'A = 1')
    res = ast_edit_marks(path, 'A = 1', 'C = 3', 'A = 99', id=node_id)
    assert res.result == 'success'
    text = Path(path).read_text(encoding='utf-8')
    ast.parse(text)
    assert 'A = 99' in text
    assert 'B = 2' not in text