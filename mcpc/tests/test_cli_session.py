import json
import os
import stat
import sys
import threading
import time
import pytest
from xy.ai.mcpc.cli import CacheMode, CliParameters, CliSessionError, CliSessionManager, Effort
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.agent import _cache_mode
from xy.ai.mcpc.tools.agent.profiles import AgentProfile
FAKE_CLI = '#!{python}\nimport json, os, sys, time\n\nfor raw in sys.stdin:\n    msg = json.loads(raw)\n    text = msg["message"]["content"][0]["text"]\n    if text == "EXIT":\n        sys.stderr.write("boom\\n")\n        sys.stderr.flush()\n        sys.exit(3)\n    if text == "NOISE":\n        for i in range(5000):\n            sys.stderr.write("x" * 100 + "\\n")\n        sys.stderr.flush()\n    if text == "HANG":\n        time.sleep(1.5)\n    if text == "SLOW":\n        time.sleep(1.0)\n    print(json.dumps({{"type": "assistant", "message": {{"content": [{{"type": "text", "text": "partial"}}]}}}}), flush=True)\n    if text == "ERRORS":\n        out = {{"type": "result", "subtype": "error_during_execution", "is_error": True,\n               "errors": ["first", {{"message": "second"}}]}}\n    elif text == "DUMP":\n        env = {{k: v for k, v in os.environ.items()\n               if k.startswith(("CLAUDE", "MCP", "DISABLE_", "FORCE_", "ENABLE_", "API_", "MAX_"))}}\n        out = {{"type": "result", "subtype": "success", "result": json.dumps({{"argv": sys.argv[1:], "env": env}})}}\n    elif text == "NORESULTTEXT":\n        out = {{"type": "result", "subtype": "success"}}\n    else:\n        out = {{"type": "result", "subtype": "success", "result": "echo:" + text}}\n    print(json.dumps(out), flush=True)\n'

@pytest.fixture
def fake_cli(tmp_path):
    path = tmp_path / 'fake-claude'
    path.write_text(FAKE_CLI.format(python=sys.executable))
    path.chmod(path.stat().st_mode | stat.S_IEXEC)
    return path

@pytest.fixture
def make_manager(tmp_path):
    managers = []

    def factory(**kwargs):
        manager = CliSessionManager(log_dir=tmp_path / 'logs', **kwargs)
        managers.append(manager)
        return manager
    yield factory
    for manager in managers:
        manager.shutdown()

def make_params(fake_cli, **kwargs):
    return CliParameters(
        config=ServerConfig(),
        model='sonnet',
        system_prompt='sys',
        mcpc_session_id='sid',
        executable=str(fake_cli),
        **kwargs)

def dump(session):
    return json.loads(session.prompt('DUMP').text)

def test_prompt_round_trip_and_resume_flag(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli), session_id='abc')
    assert session.prompt('hello').text == 'echo:hello'
    first = dump(session)['argv']
    assert first[first.index('--session-id') + 1] == 'abc'
    session.terminate()
    assert session.prompt('again').text == 'echo:again'
    second = dump(session)['argv']
    assert second[second.index('--resume') + 1] == 'abc'

def test_command_line_and_settings(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli, cc_profile='work'), session_id='abc')
    argv = dump(session)['argv']
    for removed in ('--include-hook-events', '--replay-user-messages', '--include-partial-messages'):
        assert removed not in argv
    settings = json.loads(argv[argv.index('--settings') + 1])
    hook = settings['hooks']['PreToolUse'][0]['hooks'][0]
    assert hook['type'] == 'http'
    assert hook['headers'] == {ServerConfig().session_header: 'abc'}
    permission = settings['hooks']['PermissionRequest'][0]['hooks'][0]
    assert permission['url'].endswith(ServerConfig().permission_hook_path)
    assert permission['headers'] == hook['headers']
    mcp = json.loads(argv[argv.index('--mcp-config') + 1])['mcpServers']['mcpc']
    assert mcp['type'] == 'ws'
    assert mcp['alwaysLoad'] is True
    assert mcp['headers'][ServerConfig().ccprofile_header] == 'work'

def test_environment_synced_with_workbench(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli, cc_profile='work'), session_id='abc')
    env = dump(session)['env']
    expected = {
        'CLAUDE_CODE_DISABLE_WEB_FETCH': '1',
        'CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS': '0',
        'CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT': '0',
        'MAX_MCP_OUTPUT_TOKENS': '250000',
        'MCP_TOOL_TIMEOUT': '86400000',
        'MCP_TIMEOUT': '86400000',
        'CLAUDE_ENABLE_STREAM_WATCHDOG': '0',
        'CLAUDE_ENABLE_BYTE_WATCHDOG': '0',
        'CLAUDE_STREAM_IDLE_TIMEOUT_MS': '86400000',
        'API_FORCE_IDLE_TIMEOUT': '0',
        'ENABLE_TOOL_SEARCH': 'false',
        'MCPC_SESSION_ID': 'sid'}
    for key, value in expected.items():
        assert env[key] == value, key
    assert env['CLAUDE_CONFIG_DIR'].endswith('/.claude-work')
    assert 'MAX_THINKING_TOKENS' not in env

def test_effort_values_and_thinking(fake_cli, make_manager):
    assert [e.value for e in Effort] == ['disabled', 'low', 'medium', 'high', 'xhigh', 'max']
    session = make_manager().request(make_params(fake_cli, effort='disabled'), session_id='abc')
    result = dump(session)
    assert '--effort' not in result['argv']
    assert result['env']['MAX_THINKING_TOKENS'] == '0'
    session = make_manager().request(make_params(fake_cli, effort='max'), session_id='def')
    argv = dump(session)['argv']
    assert argv[argv.index('--effort') + 1] == 'max'

@pytest.mark.parametrize(('mode', 'variable'), [('disabled', 'DISABLE_PROMPT_CACHING'), ('5m', 'FORCE_PROMPT_CACHING_5M'), ('1h', 'ENABLE_PROMPT_CACHING_1H')])
def test_cache_mode_environment(fake_cli, make_manager, mode, variable):
    session = make_manager().request(make_params(fake_cli, cache_mode=mode), session_id='abc')
    assert dump(session)['env'][variable] == '1'

def test_cache_mode_default_sets_nothing(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli), session_id='abc')
    env = dump(session)['env']
    assert not {'DISABLE_PROMPT_CACHING', 'FORCE_PROMPT_CACHING_5M', 'ENABLE_PROMPT_CACHING_1H'} & env.keys()

def test_profile_cache_mode_derivation():
    profile = AgentProfile(name='p', tools=('a',), description='d', system_prompt='s')
    assert _cache_mode(profile, ['a']) == CacheMode.MINUTES_5.value
    assert _cache_mode(None, []) == CacheMode.DISABLED.value
    assert _cache_mode(profile, []) == CacheMode.DISABLED.value
    explicit = AgentProfile(name='p', tools=(), description='d', system_prompt='s', cache_mode='1h')
    assert _cache_mode(explicit, []) == '1h'

def test_default_response_timeout_is_server_default(fake_cli, make_manager):
    manager = make_manager()
    assert manager.response_timeout == ServerConfig().agent_response_timeout_seconds
    assert manager.request(make_params(fake_cli)).response_timeout == manager.response_timeout

def test_result_errors_are_reported(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli))
    result = session.prompt('ERRORS')
    assert result.is_error
    assert 'first' in result.text and 'second' in result.text

def test_missing_result_text_falls_back_to_assistant_text(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli))
    assert session.prompt('NORESULTTEXT').text == 'partial'

def test_stderr_is_drained(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli), session_id='abc')
    assert session.prompt('NOISE').text == 'echo:NOISE'

def test_process_exit_reports_stderr(fake_cli, make_manager):
    session = make_manager().request(make_params(fake_cli))
    with pytest.raises(CliSessionError, match='boom'):
        session.prompt('EXIT')

def test_timeout_terminates_and_drops_stale_result(fake_cli, make_manager):
    session = make_manager(response_timeout=0.5).request(make_params(fake_cli))
    with pytest.raises(CliSessionError, match='timed out'):
        session.prompt('HANG')
    assert not session.running
    assert session.prompt('next').text == 'echo:next'

def test_running_prompt_survives_ttl_and_sweep(fake_cli, make_manager):
    manager = make_manager(ttl_seconds=0.3)
    session = manager.request(make_params(fake_cli), session_id='abc')
    sweeper = threading.Timer(0.6, manager.sweep)
    sweeper.start()
    assert session.prompt('SLOW').text == 'echo:SLOW'
    sweeper.join()
    assert manager.resolve('abc') is session
    assert session.is_valid()

def test_idle_session_expires_after_ttl(fake_cli, make_manager):
    manager = make_manager(ttl_seconds=0.2)
    session = manager.request(make_params(fake_cli), session_id='abc')
    session.prompt('hi')
    assert session.is_valid()
    time.sleep(0.4)
    assert not session.is_valid()
    with pytest.raises(CliSessionError):
        manager.request(resume='abc')
    assert manager.resolve('abc') is None
    assert not session.running