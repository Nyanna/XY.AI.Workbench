import json
import uuid
import urllib.request
import pytest
from xy.ai.mcpc.embedded import EmbeddedMcpc, FunctionTool, ToolProvider
PROTOCOL = '2025-06-18'

class EchoProvider(ToolProvider):

    def tools(self, environment):
        return [FunctionTool('echo', 'Echo', {'type': 'object', 'properties': {}}, lambda ctx: 'pong')]

class Client:

    def __init__(self, url, session_id, tools_header=None):
        self.url = url
        self.session_id = session_id
        self.tools_header = tools_header
        self._id = 0

    def rpc(self, method, params=None, notify=False):
        body = {'jsonrpc': '2.0', 'method': method}
        if params is not None:
            body['params'] = params
        if not notify:
            self._id += 1
            body['id'] = self._id
        headers = {'Content-Type': 'application/json',
                   'Accept': 'application/json, text/event-stream',
                   'X-MCPC-SESSION-ID': self.session_id}
        if self.tools_header is not None:
            headers['X-MCPC-TOOLS'] = self.tools_header
        request = urllib.request.Request(self.url, json.dumps(body).encode(), headers)
        with urllib.request.urlopen(request, timeout=20) as response:
            text = response.read().decode()
        return json.loads(text) if text.strip() else None

    def connect(self):
        self.rpc(
            'initialize', {
                'protocolVersion': PROTOCOL, 'capabilities': {}, 'clientInfo': {
                    'name': 'pytest', 'version': '1'}})
        self.rpc('notifications/initialized', notify=True)
        return self

    def tool_names(self):
        return sorted((t['name'] for t in self.rpc('tools/list', {})['result']['tools']))

    def call(self, name, **arguments):
        arguments.setdefault('reason', 'test')
        return self.rpc('tools/call', {'name': name, 'arguments': arguments})

def client_for(mcpc, session_id=None, tools_header=None):
    return Client(mcpc.url, session_id or str(uuid.uuid4()), tools_header).connect()

def test_builtin_filter_and_custom_tool_registration():
    with EmbeddedMcpc(builtin_tools=['list'], tools=[EchoProvider()]) as mcpc:
        assert sorted(mcpc.tool_names) == ['echo', 'list']

def test_builtin_tools_disabled():
    with EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider()]) as mcpc:
        assert mcpc.tool_names == ['echo']

def test_builtin_filter_supports_globs():
    with EmbeddedMcpc(builtin_tools=['ast_*']) as mcpc:
        assert mcpc.tool_names
        assert all((name.startswith('ast_') for name in mcpc.tool_names))

def test_decorator_registration():
    mcpc = EmbeddedMcpc(builtin_tools=False, auto_approve=True)

    @mcpc.tool('hello', 'Say hello')
    def hello(ctx):
        return 'hi'
    with mcpc:
        assert mcpc.tool_names == ['hello']
        assert client_for(mcpc, mcpc.open_session()).call('hello')['result']['content'][0]['text'] == 'hi'

def test_duplicate_tool_name_is_rejected():
    mcpc = EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider(), EchoProvider()])
    with pytest.raises(ValueError):
        mcpc.start()

def test_header_selects_tools_by_default():
    with EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider()]) as mcpc:
        assert client_for(mcpc, tools_header='echo').tool_names() == ['echo']
        assert client_for(mcpc).tool_names() == []

def test_ignore_tools_header_enables_all_registered_tools():
    with EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider()], ignore_tools_header=True) as mcpc:
        assert client_for(mcpc, tools_header='nonexistent').tool_names() == ['echo']

def test_open_session_toolset_is_not_overridden_by_header():
    with EmbeddedMcpc(builtin_tools=['list'], tools=[EchoProvider()]) as mcpc:
        sid = mcpc.open_session(tools={'echo'})
        assert client_for(mcpc, sid, tools_header='list').tool_names() == ['echo']

def test_shared_server_gives_each_session_its_own_toolset():
    with EmbeddedMcpc(builtin_tools=['list'], tools=[EchoProvider()]) as mcpc:
        a = client_for(mcpc, mcpc.open_session(tools={'echo'}))
        b = client_for(mcpc, mcpc.open_session(tools={'list'}))
        assert a.tool_names() == ['echo']
        assert b.tool_names() == ['list']

def test_close_session():
    with EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider()]) as mcpc:
        sid = mcpc.open_session()
        assert mcpc.close_session(sid) is True
        assert mcpc.close_session(sid) is False

def test_auto_approve_sets_control_disabled_and_call_succeeds():
    with EmbeddedMcpc(builtin_tools=False, tools=[EchoProvider()], auto_approve=True) as mcpc:
        sid = mcpc.open_session()
        result = client_for(mcpc, sid).call('echo')
        assert result['result']['content'][0]['text'] == 'pong'
        assert mcpc.server.sessions.get(sid).control_disabled is True

def test_websocket_enabled_by_default():
    with EmbeddedMcpc(builtin_tools=False) as mcpc:
        assert mcpc.ws_url is not None
        assert mcpc.ws_url.startswith('ws://')

def test_stop_is_idempotent():
    mcpc = EmbeddedMcpc(builtin_tools=False).start()
    mcpc.stop()
    mcpc.stop()
    with pytest.raises(RuntimeError):
        mcpc.url