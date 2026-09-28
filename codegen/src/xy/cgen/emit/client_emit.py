"""Renders the HTTP client interface and implementation. Depends on model and io emission.

One interface + one HttpClient-based implementation for the whole API (doc:
"eine Client Facade"), never per-path/per-operation classes.
"""
from pathlib import Path
from xy.cgen.emit.templates import file_extension, get_env
from xy.cgen.emit.client_context import build_client_methods, client_interface_name

def emit_client(model, writer) -> None:
    """Render the client interface and its HttpClient-based implementation."""
    if not model.operations:
        return
    package = model.base_package
    interface_name = client_interface_name(model)
    impl_name = f'{interface_name}Impl'
    interface_fqn = f'{package}.{interface_name}'
    methods = build_client_methods(model)
    env = get_env(model.language)
    ext = file_extension(model.language)
    interface_content = env.get_template('client/interface.jinja').render(package=package,
                                                                          class_name=interface_name, methods=methods)
    writer.write(Path(*package.split('.')) / f'{interface_name}.{ext}', interface_content)
    impl_content = env.get_template('client/impl.jinja').render(package=package,
                                                                class_name=impl_name, interface_fqn=interface_fqn, methods=methods)
    writer.write(Path(*package.split('.')) / f'{impl_name}.{ext}', impl_content)