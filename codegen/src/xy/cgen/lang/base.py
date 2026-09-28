"""Common interface every target-language module (`java.py`, `php.py`, ...)
implements. The generic pipeline (naming/emit) only ever talks to a
`LanguageSupport` instance obtained via `xy.cgen.lang.get_language`, never
to a hardcoded language name or an `if language == ...` branch. Adding a
language means adding one module here plus a `templates/<language>` dir.
"""
from abc import ABC, abstractmethod

class LanguageSupport(ABC):
    name: str
    extension: str
    primitive_type: dict
    any_dictionary_type: str
    parameter_type: dict
    parameter_default_type: str
    read_method: dict
    factory_method: dict
    jinja_filters: dict

    @abstractmethod
    def string_literal(self, text: str) -> str:
        """A quoted, escaped string literal."""

    @abstractmethod
    def literal(self, value, primitive_type: str) -> str:
        """An enum-constant literal for `value` of the given primitive type."""

    @abstractmethod
    def type_hint(self, type_name: str) -> str:
        """A dotted fqn or scalar type name as it appears in a type position."""

    @abstractmethod
    def parameter_declaration(self, type_hint: str, var_name: str) -> str:
        """One '<type> <name>'-style method-signature parameter."""

    @abstractmethod
    def reference_expr(self, name: str) -> str:
        """A bare local-variable reference, e.g. for a call-argument list."""

    @abstractmethod
    def path_url_expression(self, path: str, path_params) -> str:
        """An expression rebuilding a URL path, every '{param}' token replaced
        by its URL-encoded argument value. `path_params` items have
        `.raw_name` (the '{token}') and `.name` (the local holding the value)."""

    @abstractmethod
    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        """A boolean expression testing a discriminator property against one value."""

    @abstractmethod
    def applies_always(self) -> str:
        """Structural-applicability expression for a branch with no check."""

    @abstractmethod
    def applies_required(self, fields: list) -> str:
        """Structural-applicability expression: all `fields` are present."""

    @abstractmethod
    def applies_array(self) -> str:
        """Structural-applicability expression: value is a JSON array."""

    @abstractmethod
    def applies_object(self) -> str:
        """Structural-applicability expression: value is a JSON object."""

    @abstractmethod
    def applies_type_check(self, primitive_type: str) -> str:
        """Structural-applicability expression: value has the given primitive type."""

    @abstractmethod
    def applies_null(self) -> str:
        """Structural-applicability expression: value is JSON null."""

    @abstractmethod
    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        """Statements decoding matcher groups / query params / body into locals
        for the server dispatcher; one `ServerMethod.binding_lines` tuple."""