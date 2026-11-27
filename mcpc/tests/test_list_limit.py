from __future__ import annotations
from pathlib import Path
import pytest
from xy.ai.mcpc.tools.list import ListItem, list as list_directories

def _touch(base: Path, *names: str) -> None:
    for name in names:
        path = base / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('x')

def _list(path: Path, **kwargs):
    return list_directories([ListItem(path=str(path), **kwargs)]).results[0]

def test_files_are_hidden_evenly_per_depth_and_deepest_first(tmp_path: Path):
    _touch(tmp_path, 'top.txt')
    _touch(tmp_path, *(f'a/f{i:02d}.txt' for i in range(30)))
    _touch(tmp_path, *(f'b/f{i:02d}.txt' for i in range(30)))
    result = _list(tmp_path)
    assert len(result.entries) == 50
    assert result.entries[:2] == ['.:', 'top.txt']
    assert result.entries.count('8 files hidden') == 2
    assert 'f21.txt' in result.entries and 'f22.txt' not in result.entries
    assert len(result.warnings) == 1

def test_directories_are_removed_deepest_first_after_all_files(tmp_path: Path):
    _touch(tmp_path, *(f'p/d{i:02d}/f.txt' for i in range(30)))
    result = _list(tmp_path)
    assert result.entries[:3] == ['./p:', '7 files hidden', '7 subdirectories hidden']
    assert result.entries[3:5] == ['./p/d07:', '1 file hidden']
    assert len(result.entries) <= 50

def test_branch_totals_roll_up_into_the_parent(tmp_path: Path):
    _touch(tmp_path, *(f'a/b/c{i:02d}/d/f{j}.txt' for i in range(40) for j in range(3)))
    result = _list(tmp_path)
    assert len(result.entries) <= 50
    assert any((e.endswith('subdirectories hidden') for e in result.entries))
    hidden = sum((int(e.split()[0]) for e in result.entries if e.endswith(('files hidden', 'file hidden'))))
    shown = sum((1 for e in result.entries if e.startswith('f') and e.endswith('.txt')))
    assert hidden + shown == 120

def test_non_recursive_listing_is_limited_too(tmp_path: Path):
    _touch(tmp_path, *(f'f{i:03d}.txt' for i in range(80)), 'sub/x.txt')
    result = _list(tmp_path, recursive=False)
    assert len(result.entries) == 50
    assert result.entries[-1] == '32 files hidden'

def test_empty_result_is_reported_as_no_entries(tmp_path: Path):
    result = _list(tmp_path)
    assert result.entries == []
    assert result.to_dict()['entries'] == 'No entries'

def test_hierarchical_nests_shared_prefixes(tmp_path: Path):
    _touch(tmp_path, 'README.md', 'third_party/candle-kernels/Cargo.toml', 'third_party/candle-transformers/Cargo.toml')
    assert _list(tmp_path, style='hierarchical').entries == ['README.md', {
        './third_party/': [{'candle-kernels/': ['Cargo.toml']}, {'candle-transformers/': ['Cargo.toml']}]}]

def test_hierarchical_consolidates_single_chains(tmp_path: Path):
    _touch(tmp_path, 'third_party/x/y/z/kernels/a.rs', 'third_party/x/y/z/transformers/b.rs')
    assert _list(tmp_path, style='hierarchical').entries == [
        {'./third_party/x/y/z/': [{'kernels/': ['a.rs']}, {'transformers/': ['b.rs']}]}]
    single = tmp_path / 'one'
    _touch(single, 'a/b/c/f.rs')
    assert _list(single, style='hierarchical').entries == [{'./a/b/c/': ['f.rs']}]

def test_hierarchical_does_not_nest_when_repeating_the_prefix_is_shorter(tmp_path: Path):
    _touch(tmp_path, 'a/b/f.rs', 'a/c/g.rs')
    assert _list(tmp_path, style='hierarchical').entries == [{'./a/b/': ['f.rs']}, {'./a/c/': ['g.rs']}]

def test_hierarchical_keeps_own_files_next_to_subdirectories(tmp_path: Path):
    _touch(tmp_path, 'pkg/mod.py', 'pkg/sub/deep.py')
    assert _list(tmp_path, style='hierarchical').entries == [{'./pkg/': ['mod.py', {'sub/': ['deep.py']}]}]

def test_hierarchical_shows_pseudo_entries(tmp_path: Path):
    _touch(tmp_path, *(f'p/d{i:02d}/f.txt' for i in range(30)))
    entries = _list(tmp_path, style='hierarchical').entries
    assert entries[0]['./p/'][:2] == ['7 files hidden', '7 subdirectories hidden']

def test_unknown_style_is_an_item_error(tmp_path: Path):
    batch = list_directories([ListItem(path=str(tmp_path), style='tree')])
    assert not batch.results and 'style' in batch.errors[0].error