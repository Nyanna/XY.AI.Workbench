"""Tests for the hierarchical ``.mcpcignore`` configuration, list merging/reduction and ast_read fallback."""
from __future__ import annotations
from pathlib import Path
import pytest
from xy.ai.mcpc.tools import _mcpcignore as ignore
from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read
from xy.ai.mcpc.tools.list import ListItem, list as list_directories

@pytest.fixture(autouse=True)
def _fresh_registry(monkeypatch):
    monkeypatch.setattr(ignore, '_registry', {})
    monkeypatch.setattr(ignore, '_absent', {})

def _expire(monkeypatch, seconds: float=6.0) -> None:
    real = ignore.time.monotonic
    monkeypatch.setattr(ignore.time, 'monotonic', lambda: real() + seconds)

def test_without_any_file_ordinary_paths_are_allowed(tmp_path: Path):
    assert ignore.is_allowed(tmp_path / 'a.txt')
    assert ignore.is_allowed(tmp_path)
    assert not ignore.is_excluded(tmp_path)

def test_builtin_default_excludes_vcs_and_cache_dirs(tmp_path: Path):
    assert ignore.is_excluded(tmp_path / '.git')
    assert not ignore.is_allowed(tmp_path / 'node_modules' / 'x' / 'a.js')
    assert not ignore.is_allowed(tmp_path / 'pkg' / '__pycache__' / 'a.pyc')
    assert ignore.is_allowed(tmp_path / 'src' / 'build.rs')
    '# the default judges the full absolute path, so an excluded name anywhere above hides it too'
    assert not ignore.is_allowed(tmp_path / 'build' / 'a.txt')

def test_a_path_must_be_allowed_by_every_file_in_the_chain(tmp_path: Path):
    (tmp_path / '.mcpcignore').write_text('*.py\n*.txt\n*.md\n')
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / '.mcpcignore').write_text('!secret.py\n*.py\n*.txt\n*.rs\n')
    assert ignore.is_allowed(sub / 'a.py')
    assert ignore.is_allowed(sub / 'a.txt')
    assert not ignore.is_allowed(sub / 'secret.py')
    assert not ignore.is_allowed(sub / 'a.md')
    assert not ignore.is_allowed(sub / 'a.rs')
    assert not ignore.is_allowed(sub / 'a.log')

@pytest.mark.parametrize('content', ['*.py\n!secret.py\n', '!secret.py\n*.py\n'])
def test_deny_wins_regardless_of_rule_order(tmp_path: Path, content: str):
    (tmp_path / '.mcpcignore').write_text(content)
    assert ignore.is_allowed(tmp_path / 'a.py')
    assert not ignore.is_allowed(tmp_path / 'secret.py')

def test_a_deeper_file_cannot_allow_what_a_higher_file_denies(tmp_path: Path):
    (tmp_path / '.mcpcignore').write_text('*.py\n!x.py\n')
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / '.mcpcignore').write_text('*.py\n')
    assert ignore.is_allowed(sub / 'a.py')
    assert not ignore.is_allowed(sub / 'x.py')

@pytest.mark.parametrize('content', ['*.py\n!hidden/\n', '!hidden/\n*.py\n'])
def test_excluded_directory_hides_files_below(tmp_path: Path, content: str):
    (tmp_path / '.mcpcignore').write_text(content)
    assert ignore.is_excluded(tmp_path / 'hidden')
    assert not ignore.is_allowed(tmp_path / 'hidden' / 'deep' / 'a.py')
    assert ignore.is_allowed(tmp_path / 'shown' / 'a.py')
    assert not ignore.is_excluded(tmp_path / 'shown')

def test_allowed_directory_covers_everything_below_but_not_its_siblings(tmp_path: Path):
    (tmp_path / '.mcpcignore').write_text('/src/resident\n!*.tmp\n')
    resident = tmp_path / 'src' / 'resident'
    '# a directory is judged like a file'
    assert ignore.is_allowed(resident)
    assert ignore.is_allowed(resident / 'a' / 'b' / 'c.rs')
    '# a negation still wins'
    assert not ignore.is_allowed(resident / 'a' / 'c.tmp')
    '# sibling'
    assert not ignore.is_allowed(tmp_path / 'src' / 'other' / 'c.rs')
    '# not allowed itself ...'
    assert not ignore.is_allowed(tmp_path / 'src')
    '# ... but it must still be entered'
    assert not ignore.is_excluded(tmp_path / 'src')

def test_the_first_file_found_upwards_from_the_entry_applies(tmp_path: Path):
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / '.mcpcignore').write_text('*.py\n')
    '# outside of sub: default'
    assert ignore.is_allowed(tmp_path / 'c.txt')
    assert ignore.is_allowed(sub / 'a.py')
    assert not ignore.is_allowed(sub / 'b.txt')
    assert not ignore.is_allowed(sub / '.mcpcignore')

def test_file_change_is_picked_up_only_after_interval(tmp_path: Path, monkeypatch):
    ignore_file = tmp_path / '.mcpcignore'
    ignore_file.write_text('*.py\n')
    assert ignore.is_allowed(tmp_path / 'a.py')
    ignore_file.write_text('*.txt\n!x\n')
    '# throttled'
    assert ignore.is_allowed(tmp_path / 'a.py')
    _expire(monkeypatch)
    assert not ignore.is_allowed(tmp_path / 'a.py')

def test_missing_file_is_looked_for_again_only_after_interval(tmp_path: Path, monkeypatch):
    assert ignore.is_allowed(tmp_path / 'a.txt')
    (tmp_path / '.mcpcignore').write_text('*.py\n')
    '# absence is cached'
    assert ignore.is_allowed(tmp_path / 'a.txt')
    _expire(monkeypatch)
    assert not ignore.is_allowed(tmp_path / 'a.txt')

def test_new_parent_file_is_discovered_and_removed_files_fall_back_to_default(tmp_path: Path, monkeypatch):
    sub = tmp_path / 'sub'
    sub.mkdir()
    child_file = sub / '.mcpcignore'
    child_file.write_text('b.py\n')
    assert ignore.is_allowed(sub / 'b.py')
    assert not ignore.is_allowed(sub / 'a.py')
    parent_file = tmp_path / '.mcpcignore'
    parent_file.write_text('*.py\n!b.py\n')
    _expire(monkeypatch)
    assert not ignore.is_allowed(sub / 'b.py')
    parent_file.unlink()
    child_file.unlink()
    _expire(monkeypatch, 12.0)
    assert ignore.is_allowed(sub / 'a.txt')
    assert not ignore.is_allowed(sub / '.git' / 'config')

@pytest.mark.parametrize(('content', 'name'), [(None, 'node_modules'), ('*\n!hidden/\n', 'hidden'), ('*\n!hidden\n', 'hidden')])
def test_an_excluded_directory_is_itself_not_allowed(tmp_path: Path, content: str | None, name: str):
    if content is not None:
        (tmp_path / '.mcpcignore').write_text(content)
    directory = tmp_path / name
    directory.mkdir()
    assert ignore.is_excluded(directory)
    assert not ignore.is_allowed(directory)
    assert not ignore.is_allowed(directory / 'a.txt')

def test_a_corrupt_file_at_first_load_does_not_open_everything(tmp_path: Path):
    (tmp_path / '.mcpcignore').write_bytes(b'\xff\xfe\xfa*.py\n')
    assert not ignore.is_allowed(tmp_path / 'a.txt')
    assert not ignore.is_allowed(tmp_path / 'a.py')

def test_a_file_that_becomes_corrupt_keeps_hiding_what_it_did_not_allow(tmp_path: Path, monkeypatch):
    file = tmp_path / '.mcpcignore'
    file.write_text('*.py\n')
    assert ignore.is_allowed(tmp_path / 'a.py')
    assert not ignore.is_allowed(tmp_path / 'a.txt')
    file.write_bytes(b'\xff\xfe\xfa*.py\n')
    _expire(monkeypatch)
    assert not ignore.is_allowed(tmp_path / 'a.txt')

def test_list_applies_ignore_files_found_below_the_start_directory(tmp_path: Path):
    sub = tmp_path / 'sub'
    sub.mkdir()
    (sub / '.mcpcignore').write_text('*.py\n')
    (sub / 'a.py').write_text('x')
    (sub / 'b.txt').write_text('x')
    (tmp_path / 'c.txt').write_text('x')
    out = list_directories([ListItem(path=str(tmp_path))])
    assert out.results[0].entries == ['.:', 'c.txt', './sub:', 'a.py']

def test_list_enters_not_allowed_directories_to_reach_allowed_ones(tmp_path: Path):
    (tmp_path / '.mcpcignore').write_text('/src/resident\n')
    (tmp_path / 'src' / 'resident').mkdir(parents=True)
    (tmp_path / 'src' / 'other').mkdir()
    (tmp_path / 'src' / 'resident' / 'a.rs').write_text('x')
    (tmp_path / 'src' / 'other' / 'b.rs').write_text('x')
    (tmp_path / 'c.txt').write_text('x')
    out = list_directories([ListItem(path=str(tmp_path))])
    assert out.results[0].entries == ['./src/resident:', 'a.rs']

def test_list_merges_nested_items_onto_the_topmost_directory(tmp_path: Path):
    deep = tmp_path / 'sub' / 'deep'
    deep.mkdir(parents=True)
    (tmp_path / 'x.md').write_text('x')
    (tmp_path / 'c.txt').write_text('x')
    (tmp_path / 'sub' / 'a.py').write_text('x')
    (tmp_path / 'sub' / 'b.txt').write_text('x')
    (deep / 'd.py').write_text('x')
    out = list_directories([ListItem(path=str(tmp_path), pattern='\\.md$'),
                           ListItem(path=str(tmp_path / 'sub'), pattern='\\.py$')])
    assert [r.path for r in out.results] == [str(tmp_path)]
    assert out.results[0].entries == ['.:', 'x.md', './sub:', 'a.py', './sub/deep:', 'd.py']
    assert len(out.results[0].warnings) == 1 and 'merged' in out.results[0].warnings[0]
    assert out.errors == []

def test_list_does_not_merge_below_a_non_recursive_item_or_invalid_items(tmp_path: Path):
    sub = tmp_path / 'sub'
    sub.mkdir()
    (tmp_path / 'c.txt').write_text('x')
    (sub / 'a.py').write_text('x')
    flat = list_directories([ListItem(path=str(tmp_path), recursive=False), ListItem(path=str(sub))])
    assert [r.path for r in flat.results] == [str(tmp_path), str(sub)]
    bad = list_directories([ListItem(path=str(tmp_path)), ListItem(path=str(sub), pattern='(')])
    assert [r.path for r in bad.results] == [str(tmp_path)]
    assert [e.path for e in bad.errors] == [str(sub)]

def test_list_reduction_by_depth_still_lists_up_to_50_entries(tmp_path: Path):
    deep = tmp_path
    for index in range(60):
        deep = deep / f'd{index}'
        deep.mkdir()
        (deep / 'f.rs').write_text('x')
    result = list_directories([ListItem(path=str(tmp_path), pattern='\\.rs$')]).results[0]
    assert 1 < len(result.entries) <= 50
    assert len(result.warnings) == 1

def test_ast_read_without_ids_warns_about_outline_fallback(tmp_path: Path):
    source = tmp_path / 'm.py'
    source.write_text('def f():\n    return 1\n')
    result = ast_read([ReadItem(path=str(source), ids=[])]).results[0]
    assert result.nodes
    assert any(('fell back to ast_outline' in w for w in result.warnings))