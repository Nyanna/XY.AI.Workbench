from pathlib import Path
import pytest
from xy.ai.mcpc.tools.search import SearchItem, search

@pytest.fixture
def tree(tmp_path: Path) -> Path:
    for name in ('a.py', 'keep/b.py', 'skip/c.py', 'skip/d.txt'):
        file = tmp_path / name
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text('needle\n')
    return tmp_path

def _found(root: Path, **masks: list[str]) -> set[str]:
    batch = search([SearchItem(directory=[str(root)], pattern='needle', limit=50, **masks)])
    assert not batch.errors
    return {f'{m.directory}/{m.filename}' if m.directory else m.filename for r in batch.results for m in r.matches}
ALL = {'a.py', 'keep/b.py', 'skip/c.py', 'skip/d.txt'}

def test_without_masks_finds_everything(tree: Path) -> None:
    assert _found(tree) == ALL

def test_file_exclude_mask_on_extension(tree: Path) -> None:
    assert _found(tree, file_exclude_mask=['*.txt']) == ALL - {'skip/d.txt'}

def test_file_exclude_mask_on_base_name(tree: Path) -> None:
    assert _found(tree, file_exclude_mask=['c.py']) == ALL - {'skip/c.py'}

def test_file_include_mask(tree: Path) -> None:
    assert _found(tree, file_include_mask=['*.py']) == ALL - {'skip/d.txt'}

def test_dir_exclude_mask(tree: Path) -> None:
    assert _found(tree, dir_exclude_mask=['skip']) == {'a.py', 'keep/b.py'}

def test_dir_include_mask(tree: Path) -> None:
    assert _found(tree, dir_include_mask=['**/keep']) == {'keep/b.py'}

def test_exclude_wins_over_include(tree: Path) -> None:
    assert _found(tree, file_include_mask=['*.py'], dir_exclude_mask=['skip']) == {'a.py', 'keep/b.py'}

def test_everything_excluded_yields_no_matches(tree: Path) -> None:
    assert _found(tree, file_exclude_mask=['*']) == set()

def test_exclusion_applies_before_limit(tree: Path) -> None:
    batch = search([SearchItem(directory=[str(tree)], pattern='needle', limit=1, dir_exclude_mask=['skip', 'keep'])])
    assert [m.filename for r in batch.results for m in r.matches] == ['a.py']
    assert not batch.results[0].warnings