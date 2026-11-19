"""Tests for passing a script file path instead of script content.

Covers the shared helper ``resolve_script_source`` and its use by the
``python``, ``bash``, ``tool_call`` and ``ast_script`` tools.
"""
from __future__ import annotations
from pathlib import Path
import pytest
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools._tool_helpers import resolve_script_source
from xy.ai.mcpc.tools.ast.script import ScriptError, ast_script
from xy.ai.mcpc.tools.bash import BashError, bash
from xy.ai.mcpc.tools.python import PythonError, python
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolRegistry

def _call(registry: ToolRegistry, tool_name: str, **arguments):
    tool = registry.get(tool_name)
    assert tool is not None, f'tool not registered: {tool_name}'
    return tool.handler(ToolContext(session=Session(id=f'script-file-{tool_name}'), arguments=arguments))
'# --- resolve_script_source ----------------------------------------------------'

def test_resolve_absolute_path_returns_file_content(tmp_path: Path):
    """# no extension on purpose: extensions are not checked"""
    script = tmp_path / 'job'
    script.write_text('print(1)\n', encoding='utf-8')
    assert resolve_script_source(str(script)) == 'print(1)\n'

def test_resolve_tolerates_surrounding_whitespace(tmp_path: Path):
    script = tmp_path / 'a.txt'
    script.write_text('X', encoding='utf-8')
    assert resolve_script_source(f'  {script}\n') == 'X'

def test_resolve_relative_path_uses_base_dir(tmp_path: Path):
    (tmp_path / 'rel.sh').write_text('echo rel', encoding='utf-8')
    assert resolve_script_source('rel.sh', tmp_path) == 'echo rel'

def test_resolve_relative_path_uses_process_cwd_by_default(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
    (tmp_path / 'rel.py').write_text('print(2)', encoding='utf-8')
    monkeypatch.chdir(tmp_path)
    assert resolve_script_source('rel.py') == 'print(2)'

def test_resolve_absolute_path_ignores_base_dir(tmp_path: Path):
    script = tmp_path / 'abs.txt'
    script.write_text('abs', encoding='utf-8')
    other = tmp_path / 'other'
    other.mkdir()
    assert resolve_script_source(str(script), other) == 'abs'

@pytest.mark.parametrize('source', ['', '   ', 'print(1)', '/definitely/not/existing/script.py', 'x' * 5000, 'bad\x00name'])
def test_resolve_non_files_are_returned_unchanged(source: str):
    assert resolve_script_source(source) == source

def test_resolve_directory_is_not_a_file(tmp_path: Path):
    assert resolve_script_source(str(tmp_path)) == str(tmp_path)

def test_resolve_path_plus_more_content_is_script_content(tmp_path: Path):
    script = tmp_path / 'a.py'
    script.write_text('print(1)', encoding='utf-8')
    multi_line = f'{script}\nprint(2)'
    assert resolve_script_source(multi_line) == multi_line
    with_args = f'{script} --flag'
    assert resolve_script_source(with_args) == with_args

def test_resolve_unreadable_encoding_raises(tmp_path: Path):
    script = tmp_path / 'bin'
    script.write_bytes(b'\xff\xfe\x00\x80')
    with pytest.raises(UnicodeDecodeError):
        resolve_script_source(str(script))
'# --- python -------------------------------------------------------------------'

def test_python_runs_script_file(tmp_path: Path):
    script = tmp_path / 'hello'
    script.write_text("print('from file')", encoding='utf-8')
    result = python(str(script))
    assert result.exit_code == 0
    assert result.stdout.strip() == 'from file'

def test_python_still_runs_inline_content():
    assert python("print('inline')").stdout.strip() == 'inline'

def test_python_non_utf8_file_raises_python_error(tmp_path: Path):
    script = tmp_path / 'bin'
    script.write_bytes(b'\xff\xfe\x00\x80')
    with pytest.raises(PythonError):
        python(str(script))

def test_python_tool_handler_runs_script_file(registry: ToolRegistry, tmp_path: Path):
    script = tmp_path / 'tool.py'
    script.write_text("print('via tool')", encoding='utf-8')
    result = _call(registry, 'python', script=str(script))
    assert not result.is_error
    assert 'via tool' in str(result.structured_content)
'# --- bash ---------------------------------------------------------------------'

def test_bash_runs_single_entry_script_file_with_absolute_path(tmp_path: Path):
    script = tmp_path / 'run'
    script.write_text('echo one\necho two', encoding='utf-8')
    [result] = bash([{'cwd': str(tmp_path), 'script': [str(script)]}])
    assert result.exit_code == 0
    assert result.stdout.split() == ['one', 'two']

def test_bash_resolves_relative_path_against_item_cwd(tmp_path: Path):
    (tmp_path / 'rel.sh').write_text('pwd', encoding='utf-8')
    [result] = bash([{'cwd': str(tmp_path), 'script': ['rel.sh']}])
    assert result.exit_code == 0
    assert Path(result.stdout.strip()).resolve() == tmp_path.resolve()

def test_bash_multi_entry_list_is_never_treated_as_path(tmp_path: Path):
    script = tmp_path / 'run.sh'
    script.write_text('echo from-file', encoding='utf-8')
    [result] = bash([{'cwd': str(tmp_path), 'script': ['echo inline', f'# {script}']}])
    assert result.stdout.strip() == 'inline'

def test_bash_single_inline_command_still_works(tmp_path: Path):
    [result] = bash([{'cwd': str(tmp_path), 'script': ['echo hi']}])
    assert result.stdout.strip() == 'hi'

def test_bash_unreadable_script_file_reports_item_error(tmp_path: Path):
    script = tmp_path / 'bin'
    script.write_bytes(b'\xff\xfe\x00\x80')
    [outcome] = bash([{'cwd': str(tmp_path), 'script': [str(script)]}])
    assert not hasattr(outcome, 'exit_code')
    assert 'script file' in outcome.error

def test_bash_error_type_is_exported():
    assert issubclass(BashError, Exception)
'# --- tool_call ----------------------------------------------------------------'

def test_tool_call_runs_code_file(registry: ToolRegistry, tmp_path: Path):
    script = tmp_path / 'code'
    script.write_text('answer = 6 * 7\nprint(answer)', encoding='utf-8')
    result = _call(registry, 'tool_call', tool_ids=[], code=str(script))
    assert not result.is_error
    assert result.structured_content['stdout'] == '42\n'

def test_tool_call_inline_code_still_works(registry: ToolRegistry):
    result = _call(registry, 'tool_call', tool_ids=[], code='print(1 + 1)')
    assert result.structured_content['stdout'] == '2\n'

def test_tool_call_unreadable_file_is_failure(registry: ToolRegistry, tmp_path: Path):
    script = tmp_path / 'bin'
    script.write_bytes(b'\xff\xfe\x00\x80')
    result = _call(registry, 'tool_call', tool_ids=[], code=str(script))
    assert result.is_error
'# --- ast_script ---------------------------------------------------------------'

def test_ast_script_runs_code_file(tmp_path: Path):
    target = tmp_path / 'target.py'
    target.write_text('x = 1\n', encoding='utf-8')
    script = tmp_path / 'edit_script'
    script.write_text('result = len(tree.source)', encoding='utf-8')
    outcome = ast_script(str(target), str(script))
    assert outcome.result == 'success'
    assert outcome.value == repr(len('x = 1\n'))

def test_ast_script_inline_code_still_works(tmp_path: Path):
    target = tmp_path / 'target.py'
    target.write_text('x = 1\n', encoding='utf-8')
    outcome = ast_script(str(target), 'result = 1 + 1')
    assert outcome.value == '2'

def test_ast_script_unreadable_file_raises_script_error(tmp_path: Path):
    target = tmp_path / 'target.py'
    target.write_text('x = 1\n', encoding='utf-8')
    script = tmp_path / 'bin'
    script.write_bytes(b'\xff\xfe\x00\x80')
    with pytest.raises(ScriptError):
        ast_script(str(target), str(script))