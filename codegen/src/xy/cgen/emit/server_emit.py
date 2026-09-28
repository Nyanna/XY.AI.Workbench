"""Renders the abstract, transport-agnostic server base class. Depends on model, io and
client emission (mirrors the client interface's operation set 1:1)."""
from pathlib import Path
from xy.cgen.emit.templates import file_extension, get_env
from xy.cgen.emit.server_context import build_server_methods, server_class_name

def emit_server(model, writer) -> None:
    """Render the abstract server base class, one method per operation."""
    if not model.operations:
        return
    package = model.base_package
    class_name = server_class_name(model)
    methods = build_server_methods(model)
    content = get_env(model.language).get_template(
        'server/abstract_server.jinja').render(package=package, class_name=class_name, methods=methods)
    writer.write(Path(*package.split('.')) / f'{class_name}.{file_extension(model.language)}', content)