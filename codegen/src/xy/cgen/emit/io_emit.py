"""Renders request/response (de)serialization code. Depends on model, not on client.

Request-side serialization (toString()/fromString()) is added directly onto
the already-generated model class that is a request root (see
model_emit.py's is_request_root context) -- there is no separate wrapper
class. This module renders what has no other emission path: the shared
JsonSupport helper, and the per-operation response-side transport roots
(ResponseNode/CodeNode), which model_emit.py never touches (they are not
part of the shared type graph).
"""

from pathlib import Path

from jinja2 import Environment, FileSystemLoader, StrictUndefined

from xy.cgen.emit.io_context import build_code_branches, build_content_type_branches, json_support_fqn

TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "templates"

_ENV = Environment(
    loader=FileSystemLoader(str(TEMPLATES_DIR)),
    trim_blocks=True,
    lstrip_blocks=True,
    keep_trailing_newline=True,
    undefined=StrictUndefined,
)


def emit_io(model, writer) -> None:
    """Render JsonSupport plus one class per operation's ResponseNode and CodeNode."""
    _emit_json_support(model, writer)
    for operation_model in model.operations:
        _emit_response(operation_model.response, model, writer)
        for code_node in operation_model.response.codes:
            _emit_code(code_node, model, writer)


def _write(name, content: str, writer) -> None:
    relative_path = Path(*name.package.split(".")) / f"{name.class_name}.java"
    writer.write(relative_path, content)


def _emit_json_support(model, writer) -> None:
    package = f"{model.base_package}.io"
    template = _ENV.get_template("io/json_support.java.jinja")
    content = template.render(package=package)
    writer.write(Path(*package.split(".")) / "JsonSupport.java", content)


def _emit_response(response_node, model, writer) -> None:
    name = model.name_of(response_node)
    codes = build_code_branches(response_node, model)
    template = _ENV.get_template("io/response.java.jinja")
    content = template.render(
        package=name.package,
        class_name=name.class_name,
        codes=codes,
        json_support_fqn=json_support_fqn(model.base_package),
    )
    _write(name, content, writer)


def _emit_code(code_node, model, writer) -> None:
    name = model.name_of(code_node)
    content_types = build_content_type_branches(code_node, model)
    template = _ENV.get_template("io/code.java.jinja")
    content = template.render(package=name.package, class_name=name.class_name, content_types=content_types)
    _write(name, content, writer)
