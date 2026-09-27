"""Renders the abstract, transport-agnostic server base class. Depends on model, io and
client emission (mirrors the client interface's operation set 1:1)."""
from pathlib import Path
from jinja2 import Environment, FileSystemLoader, StrictUndefined
from xy.cgen.emit.server_context import build_server_methods, server_class_name
TEMPLATES_DIR = Path(__file__).resolve().parent.parent / 'templates'
_ENV = Environment(
    loader=FileSystemLoader(
        str(TEMPLATES_DIR)),
    trim_blocks=True,
    lstrip_blocks=True,
    keep_trailing_newline=True,
    undefined=StrictUndefined)

def emit_server(model, writer) -> None:
    """Render the abstract server base class, one method per operation."""
    if not model.operations:
        return
    package = model.base_package
    class_name = server_class_name(model)
    methods = build_server_methods(model)
    content = _ENV.get_template('server/abstract_server.java.jinja').render(package=package,
                                                                            class_name=class_name, methods=methods)
    writer.write(Path(*package.split('.')) / f'{class_name}.java', content)