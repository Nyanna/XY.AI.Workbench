"""URL path -> package segments / class-name fragments (deterministic, structural)."""

from cgen.naming.identifiers import class_identifier, sanitize_identifier, to_pascal_case


def _raw_segments(path: str) -> list[str]:
    """Split an OpenAPI path into non-empty segments, stripping '{...}' braces."""
    segments = []
    for part in (path or "").split("/"):
        part = part.strip()
        if not part:
            continue
        segments.append(part.strip("{}"))
    return segments


def path_to_package_segments(path: str) -> list[str]:
    """Path -> lower-case, sanitized package segments (e.g. '/responses' -> ['responses'])."""
    return [sanitize_identifier(segment).lower() for segment in _raw_segments(path)]


def path_to_class_fragment(path: str) -> str:
    """Path -> PascalCase class-name fragment (e.g. '/responses' -> 'Responses')."""
    segments = _raw_segments(path)
    if not segments:
        return "Root"
    return "".join(class_identifier(segment) for segment in segments)


def method_to_class_fragment(method: str) -> str:
    return to_pascal_case(method)
