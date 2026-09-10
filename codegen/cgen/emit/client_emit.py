"""Renders the HTTP client interface and implementation. Depends on model and io emission.

One interface + one HttpClient-based implementation for the whole API (doc:
"eine Client Facade"), never per-path/per-operation classes.
"""

from pathlib import Path

from jinja2 import Environment, FileSystemLoader, StrictUndefined

from cgen.emit.client_context import build_client_methods, client_interface_name

TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "templates"

_ENV = Environment(
    loader=FileSystemLoader(str(TEMPLATES_DIR)),
    trim_blocks=True,
    lstrip_blocks=True,
    keep_trailing_newline=True,
    undefined=StrictUndefined,
)


def emit_client(model, writer) -> None:
    """Render the client interface and its HttpClient-based implementation."""
    if not model.operations:
        return

    package = f"{model.base_package}.client"
    interface_name = client_interface_name(model)
    impl_name = f"{interface_name}Impl"
    interface_fqn = f"{package}.{interface_name}"
    methods = build_client_methods(model)

    interface_content = _ENV.get_template("client/interface.java.jinja").render(
        package=package, class_name=interface_name, methods=methods
    )
    writer.write(Path(*package.split(".")) / f"{interface_name}.java", interface_content)

    impl_content = _ENV.get_template("client/impl.java.jinja").render(
        package=package, class_name=impl_name, interface_fqn=interface_fqn, methods=methods
    )
    writer.write(Path(*package.split(".")) / f"{impl_name}.java", impl_content)
