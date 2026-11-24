import pytest
from xy.ai.mcpc.tools.move import MoveError, MoveItem, move

@pytest.fixture
def trash_home(tmp_path, monkeypatch):
    data = tmp_path / 'data'
    monkeypatch.setenv('XDG_DATA_HOME', str(data))
    return data / 'Trash'

def test_move_file_and_directory(tmp_path):
    (tmp_path / 'a.txt').write_text('x')
    (tmp_path / 'dir').mkdir()
    (tmp_path / 'dir' / 'f').write_text('y')
    result = move([MoveItem(str(tmp_path / 'a.txt'), str(tmp_path / 'sub' / 'b.txt')),
                  MoveItem(str(tmp_path / 'dir'), str(tmp_path / 'dir2'))])
    assert not result.errors
    assert (tmp_path / 'sub' / 'b.txt').read_text() == 'x'
    assert (tmp_path / 'dir2' / 'f').read_text() == 'y'
    assert not (tmp_path / 'a.txt').exists()

@pytest.mark.parametrize('target', ['', 'trash'])
def test_move_to_trash(tmp_path, trash_home, target):
    f = tmp_path / 'a.txt'
    f.write_text('x')
    result = move([MoveItem(str(f), target)])
    assert not result.errors
    assert not f.exists()
    assert (trash_home / 'files' / 'a.txt').read_text() == 'x'
    info = (trash_home / 'info' / 'a.txt.trashinfo').read_text()
    assert info.startswith('[Trash Info]') and f'Path={f}' in info

def test_trash_name_collision(tmp_path, trash_home):
    for sub in ('p', 'q'):
        (tmp_path / sub).mkdir()
        (tmp_path / sub / 'a.txt').write_text(sub)
    result = move([MoveItem(str(tmp_path / 'p' / 'a.txt')), MoveItem(str(tmp_path / 'q' / 'a.txt'))])
    assert not result.errors
    assert len(list((trash_home / 'files').iterdir())) == 2

def test_errors_are_per_item(tmp_path):
    (tmp_path / 'a').write_text('a')
    (tmp_path / 'b').write_text('b')
    result = move([MoveItem('relative', str(tmp_path /
                                            'x')), MoveItem(str(tmp_path /
                                                                'missing'), str(tmp_path /
                                                                                'x')), MoveItem(str(tmp_path /
                                                                                                    'a'), str(tmp_path /
                                                                                                              'b')), MoveItem(str(tmp_path /
                                                                                                                                  'a'), 'relative'), MoveItem(str(tmp_path /
                                                                                                                                                                  'a'), str(tmp_path /
                                                                                                                                                                            'ok'))])
    assert len(result.errors) == 4
    assert [e.source for e in result.errors] == ['relative', str(
        tmp_path / 'missing'), str(tmp_path / 'a'), str(tmp_path / 'a')]
    assert (tmp_path / 'ok').read_text() == 'a'
    assert (tmp_path / 'b').read_text() == 'b'

def test_directory_into_itself(tmp_path):
    (tmp_path / 'd').mkdir()
    result = move([MoveItem(str(tmp_path / 'd'), str(tmp_path / 'd' / 'e'))])
    assert len(result.errors) == 1

def test_empty_items():
    with pytest.raises(MoveError):
        move([])

def _call(arguments):
    from xy.ai.mcpc.server.session import Session
    from xy.ai.mcpc.tools.move import MoveTool
    from xy.ai.mcpc.tools.tool_context import ToolContext
    return MoveTool().handle(ToolContext(session=Session(id='move'), arguments=arguments))

def test_tool_reports_only_success_or_errors(tmp_path):
    (tmp_path / 'a').write_text('a')
    ok = _call({'items': [{'source': str(tmp_path / 'a'), 'target': str(tmp_path / 'b')}]})
    assert ok.structured_content == {'result': 'success'}
    failed = _call({'items': [{'source': str(tmp_path / 'a'), 'target': str(tmp_path / 'c')}]})
    assert failed.structured_content == {'errors': [{'source': str(tmp_path / 'a'), 'error': 'Source does not exist.'}]}