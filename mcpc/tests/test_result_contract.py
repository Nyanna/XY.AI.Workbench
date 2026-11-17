"""Contract tests for the shared result model: one owner for warnings and errors."""
from __future__ import annotations
import ast
import importlib
import pkgutil
from dataclasses import dataclass
from inspect import get_annotations
from pathlib import Path
from typing import Any
import pytest
import xy.ai.mcpc.tools as tools_package
from xy.ai.mcpc.tools._tool_helpers import batch_tool_result, serialize_batch_result
from xy.ai.mcpc.tools.list import ListItem
from xy.ai.mcpc.tools.list import list as list_directories
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult, RESERVED_NAMES
from xy.ai.mcpc.tools.tool_registry import ToolResult, normalize_result
from xy.ai.mcpc.tools.results import Done, ItemPayload, RawPayload, SinglePayload
from xy.ai.mcpc.tools.process import ProcessBatchResult, ProcessItemError, ProcessResult, pack_process_result
from xy.ai.mcpc.tools.ast.validate import InvalidFile, ValidFile, ValidateBatchResult
import re
from dataclasses import fields, is_dataclass
from xy.ai.mcpc.tools.tool_registry import ToolDefinition
TOOLS_DIR = Path(tools_package.__path__[0])
RESULTS_MODULE = tools_package.__name__ + '.results'
FORBIDDEN_NAMES = {'warning', 'notice', 'hint'}

def _import_all_tool_modules() -> None:
    for info in pkgutil.walk_packages(tools_package.__path__, prefix=tools_package.__name__ + '.'):
        importlib.import_module(info.name)

def _subclasses(base: type) -> list[type]:
    found: list[type] = []
    for sub in base.__subclasses__():
        found.append(sub)
        found.extend(_subclasses(sub))
    return found

def _project_subclasses() -> list[type]:
    _import_all_tool_modules()
    classes = [c for base in (ItemResult, ItemError, BatchResult) for c in _subclasses(base)]
    return [c for c in classes if c.__module__.startswith('xy.ai.mcpc.') and c.__module__ != RESULTS_MODULE]

@dataclass(frozen=True)
class _Item(ItemResult):
    path: str
    value: int = 0

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

@dataclass(frozen=True)
class _Failure(ItemError):
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

class _Batch(BatchResult[_Item, _Failure]):

    def derived_warnings(self) -> tuple[str, ...]:
        return ('derived',) if self.results else ()

def test_project_result_classes_are_discovered():
    assert len(_project_subclasses()) > 20

def test_project_result_classes_use_no_reserved_fields_and_keep_to_dict_final():
    for cls in _project_subclasses():
        assert not set(get_annotations(cls)) & RESERVED_NAMES, cls
        assert 'to_dict' not in cls.__dict__, cls

@pytest.mark.parametrize('base, field', [(ItemResult, 'warning'), (ItemResult, 'notice'), (ItemError, 'hint'), (ItemError, 'errors'), (BatchResult, 'notice')])
def test_reserved_field_names_are_rejected_at_class_creation(base, field):
    with pytest.raises(TypeError, match='reserved'):
        type('Bad', (base,), {'__annotations__': {field: 'str'}})

def test_overriding_to_dict_is_rejected():
    with pytest.raises(TypeError, match='to_dict'):
        type('Bad', (ItemResult,), {'to_dict': lambda self: {}})

def test_item_serialization_adds_key_payload_and_warnings():
    item = _Item(path='a', value=1).warn('w1', 'w2')
    assert item.to_dict() == {'path': 'a', 'value': 1, 'warnings': ['w1', 'w2']}
    assert _Item(path='a').to_dict() == {'path': 'a', 'value': 0}

def test_error_serialization_has_error_string_and_no_warnings():
    assert _Failure(path='a', error='boom').to_dict() == {'path': 'a', 'error': 'boom'}

def test_payload_with_reserved_key_is_rejected():

    @dataclass(frozen=True)
    class Leaky(ItemResult):
        path: str

        def key(self) -> dict[str, Any]:
            return {'path': self.path}

        def payload(self) -> dict[str, Any]:
            return {'notice': 'x'}
    with pytest.raises(ValueError, match='reserved'):
        Leaky(path='a').to_dict()

def test_batch_result_rejects_foreign_items():
    with pytest.raises(TypeError):
        _Batch(results=[{'path': 'a'}])
    with pytest.raises(TypeError):
        _Batch(errors=['boom'])

def test_serialize_batch_result_requires_a_batch_result():

    class Duck:
        results = [_Item(path='a')]
        errors: list[Any] = []
        notice = 'x'
    with pytest.raises(TypeError):
        serialize_batch_result(Duck())

def test_batch_warnings_are_deduplicated_and_derived():
    batch = _Batch(results=[_Item(path='a')], warnings=('own', 'derived'))
    assert batch.all_warnings() == ['own', 'derived']
    assert _Batch(errors=[_Failure(path='a', error='x')]).all_warnings() == []

def test_batch_tool_result_puts_call_warnings_at_the_root_and_item_warnings_in_the_item():
    batch = _Batch(results=[_Item(path='a').warn('item-level')], errors=[_Failure(path='b', error='x')])
    out = batch_tool_result(batch).to_dict()['structuredContent']
    assert out['warnings'] == ['derived']
    assert out['results'] == [{'path': 'a', 'value': 0, 'warnings': ['item-level']}]
    assert out['errors'] == [{'path': 'b', 'error': 'x'}]

def test_list_reduction_is_an_item_warning_list(tmp_path: Path):
    for index in range(60):
        (tmp_path / f'f{index}.txt').write_text('x')
    out = batch_tool_result(list_directories([ListItem(path=str(tmp_path))])).to_dict()['structuredContent']
    assert 'warnings' not in out
    assert isinstance(out['results'][0]['warnings'], list) and len(out['results'][0]['warnings']) == 1
    assert 'warning' not in out['results'][0]

def test_tool_result_failure_carries_no_structure_or_warnings():
    failure = ToolResult.failure('nope')
    assert failure.to_dict() == {'content': [{'type': 'text', 'text': 'nope'}], 'isError': True}
    with pytest.raises(ValueError):
        ToolResult(content=[], is_error=True, warnings=['w'])
    with pytest.raises(ValueError):
        ToolResult(is_error=True, structured_content={'a': 1})

def test_tool_result_is_the_only_writer_of_root_warnings():
    assert ToolResult(structured_content={'a': 1}, warnings=['w']).to_dict()[
        'structuredContent'] == {'a': 1, 'warnings': ['w']}
    assert ToolResult(warnings=['w']).to_dict()['structuredContent'] == {'warnings': ['w']}
    for reserved in ('warnings', 'messageFromUser'):
        with pytest.raises(ValueError, match='reserved'):
            ToolResult(structured_content={reserved: 'x'})

@pytest.mark.parametrize('value', ['text', {'a': 1}, None])
def test_handlers_must_return_tool_result(value):
    with pytest.raises(TypeError):
        normalize_result(value)
    assert normalize_result(ToolResult()) is not None

def test_unknown_exa_ids_are_reported_not_only_logged():
    from xy.ai.mcpc.tools.mcp.exa.search_results import _resolve
    results, missing = _resolve(['definitely-unknown-id'])
    assert results == [] and missing == ['definitely-unknown-id']

def _forbidden_names(path: Path) -> list[str]:
    tree = ast.parse(path.read_text(encoding='utf-8'))
    hits: list[str] = []
    for node in ast.walk(tree):
        if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and (node.target.id in FORBIDDEN_NAMES):
            hits.append(f'{path.name}:{node.lineno}: field {node.target.id}')
        if isinstance(node, ast.Dict):
            hits.extend((f'{path.name}:{key.lineno}: key {key.value!r}' for key in node.keys if isinstance(
                key, ast.Constant) and key.value in FORBIDDEN_NAMES))
        if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Constant) and (node.slice.value in FORBIDDEN_NAMES):
            hits.append(f'{path.name}:{node.lineno}: key {node.slice.value!r}')
    return hits

def test_no_ad_hoc_warning_notice_or_hint_fields_outside_results_module():
    hits: list[str] = []
    for path in sorted(TOOLS_DIR.rglob('*.py')):
        if path.name == 'results.py' and path.parent == TOOLS_DIR:
            continue
        hits.extend(_forbidden_names(path))
    assert not hits, 'use ItemResult.warnings / BatchResult.warnings / ToolResult.warnings:\n' + '\n'.join(hits)

@dataclass(frozen=True)
class _Failing(ItemError):
    path: str
    reason: str | None = None
    candidates: list[str] | None = None

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

@dataclass(frozen=True)
class _Ok(ItemResult):
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

class _PlainBatch(BatchResult[_Ok, _Failing]):
    pass

def test_item_error_default_payload_keeps_subclass_fields():
    err = _Failing(path='a', error='boom', reason='why', candidates=['x'])
    assert err.to_dict() == {'path': 'a', 'error': 'boom', 'reason': 'why', 'candidates': ['x']}
    assert _Failing(path='a', error='boom').to_dict() == {'path': 'a', 'error': 'boom'}

def test_tool_result_ok_carries_root_warnings_and_withholds_approval_on_failure():
    clean = ToolResult.ok(_PlainBatch(results=[_Ok(path='a')], warnings=('w',)), auto_approve=True)
    assert clean.auto_approve and clean.warnings == ['w']
    assert clean.to_dict()['structuredContent'] == {'results': [{'path': 'a'}], 'warnings': ['w']}
    failed = ToolResult.ok(_PlainBatch(errors=[_Failing(path='a', error='x')]), auto_approve=True)
    assert not failed.auto_approve

def test_single_payload_rejects_reserved_fields_and_moves_warnings_to_the_root():
    with pytest.raises(TypeError):

        @dataclass(frozen=True, kw_only=True)
        class _Bad(SinglePayload):
            hint: str = ''

    @dataclass(frozen=True, kw_only=True)
    class _Good(SinglePayload):
        answer: str
    result = ToolResult.ok(_Good(answer='x', warnings=('careful',)))
    assert result.to_dict()['structuredContent'] == {'answer': 'x', 'warnings': ['careful']}

def test_done_is_the_only_dataless_success_shape():
    assert ToolResult.ok(Done(), auto_approve=True).to_dict()['structuredContent'] == {'result': 'success'}

def test_item_payload_moves_item_warnings_to_the_root():
    result = ToolResult.ok(ItemPayload(_Ok(path='a').warn('w')))
    assert result.to_dict()['structuredContent'] == {'path': 'a', 'warnings': ['w']}

def test_raw_payload_requires_a_reason_for_the_deviation():
    with pytest.raises(ValueError):
        RawPayload({'a': 1}, reason='')
    result = ToolResult.ok(RawPayload({'a': 1}, reason='remote shape', warnings=('w',)))
    assert result.to_dict()['structuredContent'] == {'a': 1, 'warnings': ['w']}
STRUCTURED_CONTENT_WRITERS = {'tools/tool_registry.py', 'embedded/tools.py'}

def test_structured_content_is_only_written_by_the_registry_and_the_embedded_adapter():
    root = TOOLS_DIR.parent
    hits: list[str] = []
    for path in sorted(root.rglob('*.py')):
        rel = path.relative_to(root).as_posix()
        if rel in STRUCTURED_CONTENT_WRITERS:
            continue
        tree = ast.parse(path.read_text(encoding='utf-8'))
        for node in ast.walk(tree):
            if isinstance(node, ast.keyword) and node.arg == 'structured_content':
                hits.append(f'{rel}:{node.value.lineno}')
    assert not hits, 'use ToolResult.ok(<ToolPayload>):\n' + '\n'.join(hits)

def test_no_ad_hoc_error_dicts_in_tools():
    hits: list[str] = []
    for path in sorted(TOOLS_DIR.rglob('*.py')):
        if path.parent == TOOLS_DIR and path.name == 'results.py' or 'mcp' in path.relative_to(TOOLS_DIR).parts[:1]:
            continue
        tree = ast.parse(path.read_text(encoding='utf-8'))
        for node in ast.walk(tree):
            if isinstance(node, ast.Dict):
                hits.extend((f'{path.name}:{key.lineno}: key {key.value!r}' for key in node.keys if isinstance(
                    key, ast.Constant) and key.value in {'error', 'errors'}))
    assert not hits, 'use ItemError / ToolResult.failure:\n' + '\n'.join(hits)

def test_process_batch_is_associated_by_index_and_omits_a_zero_exit_code():
    """# type: ignore[arg-type]"""
    ok = pack_process_result(ProcessResult(0, 'hi\n', ''), None, omit_zero_exit_code=True, index=1)
    '# type: ignore[arg-type]'
    failed = pack_process_result(ProcessResult(2, '', 'bad'), None, omit_zero_exit_code=True, index=2)
    batch = ProcessBatchResult(results=[ok, failed], errors=[ProcessItemError(index=0, error='no cwd')])
    assert batch.to_dict() == {'results': [{'index': 1, 'stdout': 'hi\n'}, {
        'index': 2, 'exit_code': 2, 'stdout': '', 'stderr': 'bad'}], 'errors': [{'index': 0, 'error': 'no cwd'}]}

def test_validate_batch_reports_failed_files_as_errors_and_approves_only_clean_batches():
    clean = ToolResult.ok(ValidateBatchResult(results=[ValidFile(path='a.py')]), auto_approve=True)
    assert clean.auto_approve and clean.to_dict()['structuredContent'] == {'results': [{'path': 'a.py'}]}
    broken = ToolResult.ok(ValidateBatchResult(errors=[InvalidFile(path='b.py', error='line 1')]), auto_approve=True)
    assert not broken.auto_approve
    assert broken.to_dict()['structuredContent'] == {'errors': [{'path': 'b.py', 'error': 'line 1'}]}

def _description_texts(value: Any) -> list[str]:
    if isinstance(value, dict):
        texts: list[str] = []
        for key, item in value.items():
            if key == 'description' and isinstance(item, str):
                texts.append(item)
            else:
                texts.extend(_description_texts(item))
        return texts
    if isinstance(value, list):
        return [t for item in value for t in _description_texts(item)]
    return []

def _property_names(value: Any) -> set[str]:
    if isinstance(value, dict):
        names = set(value['properties']) if isinstance(value.get('properties'), dict) else set()
        return names.union(*(_property_names(v) for v in value.values())) if value else names
    if isinstance(value, list):
        return set().union(*(_property_names(v) for v in value)) if value else set()
    return set()

def test_descriptions_and_input_schemas_do_not_reference_wire_field_names():
    _import_all_tool_modules()
    payloads = [*_project_subclasses(), *(c for c in _subclasses(SinglePayload) if c.__module__ != RESULTS_MODULE)]
    wire_names = {f.name for c in payloads if is_dataclass(c) for f in fields(c) if '_' in f.name}
    tools = [t for t in _subclasses(ToolDefinition) if t.__module__.startswith('xy.ai.mcpc.')]
    input_names = set().union(*(_property_names(getattr(t, 'input_schema', None)) for t in tools))
    forbidden = wire_names - input_names
    assert forbidden, 'no wire field names collected'
    hits: list[str] = []
    for tool in tools:
        description = getattr(tool, 'description', None)
        texts = [description] if isinstance(description, str) else []
        texts.extend(_description_texts(getattr(tool, 'input_schema', None)))
        for text in texts:
            hits.extend((f'{tool.__name__}: {name}' for name in sorted(forbidden) if re.search(f'\\b{name}\\b', text)))
    assert not hits, 'describe output in prose, do not name wire fields:\n' + '\n'.join(hits)