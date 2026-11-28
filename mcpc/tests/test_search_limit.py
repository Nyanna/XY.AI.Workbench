from pathlib import Path
from xy.ai.mcpc.tools.search import SearchItem, search

def _run(root: Path, limit: int):
    batch = search([SearchItem(directory=[str(root)], pattern='needle', limit=limit)])
    assert not batch.errors
    return batch.results[0]

def _write(root: Path, name: str, count: int) -> None:
    (root / name).write_text(''.join((f'needle {i}\n' for i in range(count))))

def _by_name(result) -> dict[str, tuple[int, int]]:
    return {m.filename: (len(m.matches), m.hidden) for m in result.matches}

def test_within_limit_nothing_hidden(tmp_path: Path) -> None:
    _write(tmp_path, 'a.txt', 2)
    _write(tmp_path, 'b.txt', 2)
    result = _run(tmp_path, 4)
    assert _by_name(result) == {'a.txt': (2, 0), 'b.txt': (2, 0)}
    assert not result.warnings

def test_one_match_per_path_is_removed_stepwise(tmp_path: Path) -> None:
    _write(tmp_path, 'a.txt', 5)
    _write(tmp_path, 'b.txt', 5)
    result = _run(tmp_path, 6)
    counts = _by_name(result)
    assert sum((kept for kept, _ in counts.values())) == 6
    assert {kept for kept, _ in counts.values()} == {3}
    assert {hidden for _, hidden in counts.values()} == {2}
    assert len(result.warnings) == 1 and 'Limit of 6' in result.warnings[0]

def test_every_file_keeps_at_least_one_match(tmp_path: Path) -> None:
    _write(tmp_path, 'a.txt', 10)
    _write(tmp_path, 'b.txt', 1)
    result = _run(tmp_path, 2)
    assert _by_name(result) == {'a.txt': (1, 9), 'b.txt': (1, 0)}
    assert not any(('files are hidden' in w for w in result.warnings))

def test_files_are_hidden_when_limit_is_below_file_count(tmp_path: Path) -> None:
    for name in ('a.txt', 'b.txt', 'c.txt', 'd.txt'):
        _write(tmp_path, name, 3)
    result = _run(tmp_path, 3)
    counts = _by_name(result)
    assert len(counts) == 3
    assert all((kept == 1 and hidden == 2 for kept, hidden in counts.values()))
    assert any((w == '1 files are hidden.' for w in result.warnings))
    assert any(('Limit of 3' in w for w in result.warnings))