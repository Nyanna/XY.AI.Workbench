"""Minimal tests for the recursive argument validation of tools/call."""
from __future__ import annotations
import pytest
from xy.ai.mcpc.server import errors
from xy.ai.mcpc.server.mcp_protocol import _validate_arguments
from xy.ai.mcpc.tools._tool_helpers import batch_schema
_SCHEMA = batch_schema({'directory': {'type': 'array', 'items': {'type': 'string'}, 'minItems': 1}, 'pattern': {
                       'type': 'string'}, 'limit': {'type': 'integer', 'minimum': 1, 'maximum': 10}}, ['directory', 'pattern'], 'items')

def _item(**overrides):
    return {**{'directory': ['/tmp'], 'pattern': 'x'}, **overrides}

def test_valid_arguments_pass():
    _validate_arguments(_SCHEMA, {'items': [_item(limit=5)]})

@pytest.mark.parametrize(('arguments', 'fragment'), [({}, 'items'), ({'items': [{'directory': ['/tmp']}]}, "in 'items[0]': pattern"), ({'items': [_item(reason='x')]}, "Unexpected argument(s) in 'items[0]': reason"), ({'items': [_item(), _item(pattern=1)]}, "'items[1].pattern' must be of type string"), ({'items': [_item(directory=[1])]}, "'items[0].directory[0]' must be of type string"), ({'items': [_item(directory=[])]}, 'at least 1 item'), ({'items': [_item(limit=11)]}, '<= 10'), ({'items': []}, 'at least 1 item'), ({'items': [_item()], 'extra': 1}, 'Unexpected argument(s): extra')])
def test_invalid_arguments_are_rejected(arguments, fragment):
    with pytest.raises(errors.JsonRpcError) as excinfo:
        _validate_arguments(_SCHEMA, arguments)
    assert fragment in excinfo.value.message