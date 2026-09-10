"""Java identifier sanitizing: keywords, leading underscores, case conversion."""

import re

JAVA_KEYWORDS = frozenset(
    """
    abstract continue for new switch assert default goto package synchronized
    boolean do if private this break double implements protected throw byte
    else import public throws case enum instanceof return transient catch
    extends int short try char final interface static void class finally
    long strictfp volatile const float native super while var record yield
    sealed permits non-sealed true false null
    """.split()
)

_NON_IDENTIFIER_CHARS = re.compile(r"[^A-Za-z0-9_]+")
_WORD_SPLIT = re.compile(r"[^A-Za-z0-9]+")


def sanitize_identifier(raw: str) -> str:
    """Turn an arbitrary string into a valid, non-keyword Java identifier.

    Strips illegal characters, removes/replaces a leading underscore (so
    '_MisalignmentErrorType' stays readable rather than starting with '_'),
    guards against a leading digit, and escapes Java keywords with a suffix.
    """
    cleaned = _NON_IDENTIFIER_CHARS.sub("_", raw or "")
    cleaned = cleaned.strip("_") or "Value"
    if cleaned[0].isdigit():
        cleaned = f"_{cleaned}"
    if cleaned in JAVA_KEYWORDS:
        cleaned = f"{cleaned}_"
    return cleaned


def to_pascal_case(raw: str) -> str:
    """Split on non-alphanumeric boundaries and title-case each word."""
    words = [w for w in _WORD_SPLIT.split(raw or "") if w]
    if not words:
        return "Value"
    return "".join(w[:1].upper() + w[1:] for w in words)


def to_camel_case(raw: str) -> str:
    """Same as to_pascal_case but the first letter is lower-case."""
    pascal = to_pascal_case(raw)
    return pascal[:1].lower() + pascal[1:]


def property_accessor_name(label: str) -> str:
    """JSON field label -> Java accessor-name fragment (snake_case -> camelCase)."""
    return sanitize_identifier(to_camel_case(label))


def class_identifier(raw: str) -> str:
    """Class-name-safe identifier: PascalCase, then sanitized/keyword-escaped."""
    return sanitize_identifier(to_pascal_case(raw))


def content_type_short_name(content_type: str) -> str:
    """'application/json' -> 'json': the subtype is enough within our JSON-only scope."""
    subtype = (content_type or "").rsplit("/", maxsplit=1)[-1]
    return sanitize_identifier(subtype).lower() or "json"
