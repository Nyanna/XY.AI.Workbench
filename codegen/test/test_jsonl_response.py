import importlib.util
import textwrap
from pathlib import Path
import pytest
from xy.cgen.cli import main
from xy.cgen.config import Config
from xy.cgen.ingest import ingest_schema
from xy.cgen.model.build import build_model
from xy.cgen.model.nodes import ListNode, RefNode
JSONL = 'application/x-jsonl'
JSON = 'application/json'
LANGUAGES = [('java', 'java', 'parseBody', 'writeBody'), ('python', 'py', 'parse_body', 'write_body'),
             ('php', 'php', 'parseBody', 'writeBody'), ('rust', 'rs', 'parse_body', 'write_body')]

def make_spec(response_content: str) -> str:
    return textwrap.dedent("        openapi: 3.1.0\n        info:\n          title: test\n          version: '1'\n        paths:\n          /batches/results:\n            get:\n              operationId: getResults\n              responses:\n                '200':\n                  description: Successful Response\n                  content:\n        ") + \
        textwrap.indent(textwrap.dedent(response_content), ' ' * 12) + textwrap.dedent('\n        components:\n          schemas:\n            MessageBatchIndividualResponse:\n              type: object\n              properties:\n                custom_id:\n                  type: string\n                result:\n                  type: string\n        ')
JSONL_ONLY = f'{JSONL}:\n  schema:\n    $ref: "#/components/schemas/MessageBatchIndividualResponse"\n'
JSON_ONLY = f'{JSON}:\n  schema:\n    $ref: "#/components/schemas/MessageBatchIndividualResponse"\n'
BOTH = JSON_ONLY + JSONL_ONLY
TEXT_ONLY = 'text/plain:\n  schema:\n    type: string\n'

@pytest.fixture
def write_spec(tmp_path):

    def write(response_content: str) -> Path:
        path = tmp_path / 'openapi.yaml'
        path.write_text(make_spec(response_content), encoding='utf-8')
        return path
    return write

def generate(schema: Path, out: Path, language: str) -> None:
    main(['--schema', str(schema), '--out', str(out), '--language', language])

def all_files(out: Path) -> list[Path]:
    return [p for p in out.rglob('*') if p.is_file()]

def code_files(out: Path) -> list[Path]:
    return [p for p in all_files(out) if 'Code200' in p.stem]

def model_of(schema: Path):
    config = Config(input_schema=schema, output_dir=schema.parent / 'unused')
    return build_model(ingest_schema(config))

def test_ingest_keeps_jsonl_response(write_spec):
    operation = model_of(write_spec(JSONL_ONLY)).operations[0].operation
    assert operation.responses == {'200': {JSONL: {'$ref': '#/components/schemas/MessageBatchIndividualResponse'}}}

def test_ingest_drops_unsupported_content_type(write_spec):
    assert model_of(write_spec(TEXT_ONLY)).operations[0].operation.responses == {}

def test_ingest_keeps_declaration_order(write_spec):
    operation = model_of(write_spec(JSONL_ONLY + JSON_ONLY)).operations[0].operation
    assert list(operation.responses['200']) == [JSONL, JSON]

def test_model_wraps_jsonl_schema_in_list(write_spec):
    code, = model_of(write_spec(JSONL_ONLY)).operations[0].response.codes
    view, = code.content_types
    assert view.content_type == JSONL
    body = view.body.target
    assert isinstance(body, ListNode)
    element, = body.elements
    assert isinstance(element.target, RefNode)
    assert element.target.name == 'MessageBatchIndividualResponse'

def test_model_keeps_json_schema_unwrapped(write_spec):
    code, = model_of(write_spec(JSON_ONLY)).operations[0].response.codes
    view, = code.content_types
    assert view.content_type == JSON
    assert isinstance(view.body.target, RefNode)

def test_model_has_one_view_per_content_type(write_spec):
    code, = model_of(write_spec(BOTH)).operations[0].response.codes
    assert [view.content_type for view in code.content_types] == [JSON, JSONL]
    assert isinstance(code.content_types[0].body.target, RefNode)
    assert isinstance(code.content_types[1].body.target, ListNode)

@pytest.mark.parametrize('language, extension, parse_name, write_name', LANGUAGES)
def test_jsonl_generates_code_class_and_list(write_spec, tmp_path, language, extension, parse_name, write_name):
    out = tmp_path / 'out'
    generate(write_spec(JSONL_ONLY), out, language)
    codes = code_files(out)
    assert codes, 'no response code class generated'
    assert any((JSONL in p.read_text(encoding='utf-8') for p in codes))
    assert any((p.stem.endswith('List') for p in all_files(out))), 'no list class generated'

@pytest.mark.parametrize('language, extension, parse_name, write_name', LANGUAGES)
def test_json_still_generates_code_class(write_spec, tmp_path, language, extension, parse_name, write_name):
    out = tmp_path / 'out'
    generate(write_spec(JSON_ONLY), out, language)
    codes = code_files(out)
    assert codes
    assert all((JSONL not in p.read_text(encoding='utf-8') for p in codes))

@pytest.mark.parametrize('language, extension, parse_name, write_name', LANGUAGES)
def test_both_content_types_in_one_code_class(write_spec, tmp_path, language, extension, parse_name, write_name):
    out = tmp_path / 'out'
    generate(write_spec(BOTH), out, language)
    text = '\n'.join((p.read_text(encoding='utf-8') for p in code_files(out)))
    assert JSON + '"' in text or JSON + "'" in text
    assert JSONL in text

@pytest.mark.parametrize('language, extension, parse_name, write_name', LANGUAGES)
def test_unsupported_content_type_generates_no_code_class(write_spec, tmp_path, language, extension, parse_name, write_name):
    out = tmp_path / 'out'
    generate(write_spec(TEXT_ONLY), out, language)
    assert code_files(out) == []

@pytest.mark.parametrize('language, extension, parse_name, write_name', LANGUAGES)
def test_response_and_json_support_use_content_type_aware_body(write_spec, tmp_path, language, extension, parse_name, write_name):
    out = tmp_path / 'out'
    generate(write_spec(JSONL_ONLY), out, language)
    support, = out.rglob(f'JsonSupport.{extension}')
    support_text = support.read_text(encoding='utf-8')
    assert parse_name in support_text
    assert write_name in support_text
    assert 'application/x-jsonl' in support_text
    others = [p.read_text(encoding='utf-8') for p in all_files(out) if p != support]
    assert any((parse_name in t and write_name in t for t in others))

def load_python_json_support(out: Path):
    path, = out.rglob('JsonSupport.py')
    spec = importlib.util.spec_from_file_location('generated_json_support', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

@pytest.fixture
def json_support(write_spec, tmp_path):
    out = tmp_path / 'out'
    generate(write_spec(JSONL_ONLY), out, 'python')
    return load_python_json_support(out)

def test_python_parse_jsonl_body(json_support):
    body = '{"custom_id": "a"}\n\n{"custom_id": "b"}\n'
    assert json_support.parse_body(body, 'application/x-jsonl') == [{'custom_id': 'a'}, {'custom_id': 'b'}]

def test_python_parse_jsonl_ignores_charset_and_case(json_support):
    assert json_support.parse_body('{"a": 1}', 'Application/X-JSONL; charset=utf-8') == [{'a': 1}]

def test_python_parse_empty_jsonl_body_is_empty_list(json_support):
    assert json_support.parse_body('', 'application/x-jsonl') == []

def test_python_parse_json_body_unchanged(json_support):
    assert json_support.parse_body('{"a": 1}', 'application/json') == {'a': 1}
    assert json_support.parse_body('[1, 2]', None) == [1, 2]

def test_python_write_jsonl_body_one_line_per_element(json_support):
    assert json_support.write_body([{'a': 1}, {'b': 2}], 'application/x-jsonl') == '{"a": 1}\n{"b": 2}\n'

def test_python_write_json_body_unchanged(json_support):
    assert json_support.write_body([{'a': 1}], 'application/json') == '[{"a": 1}]'

def test_python_jsonl_roundtrip(json_support):
    lines = [{'custom_id': 'a', 'n': 1}, {'custom_id': 'b', 'n': 2}]
    body = json_support.write_body(lines, 'application/x-jsonl')
    assert json_support.parse_body(body, 'application/x-jsonl') == lines