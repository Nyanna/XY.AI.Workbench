"""Strip non-printable control characters from external tool output.

Third-party MCP servers and APIs occasionally leak raw control bytes into
otherwise textual fields (e.g. a stray ``\\x02`` STX byte embedded in an Exa
search result). Left untouched, such a byte breaks anything that later
re-serialises the value as YAML block scalars (``|``/``>``), since those
require the content to be free of ASCII control characters other than
whitespace.

:func:`sanitize_value` recursively walks a JSON-like structure (as produced by
:class:`~xy.ai.mcpc.codec.JsonCodec` or a remote ``CallToolResult``) and
removes such characters from every string it finds, leaving dict keys,
numbers, booleans and ``None`` untouched.
"""


import re
from typing import Any

__all__ = ["sanitize_text", "sanitize_value"]

#: C0 controls (except tab/newline/carriage-return), DEL, and C1 controls.
#: These are the characters that are never legitimately part of readable text
#: and that break YAML block-scalar / plain-scalar representation.
_CONTROL_CHARS_RE = re.compile(
    "[\x00-\x08\x0b\x0c\x0e-\x1f\x7f-\x9f]"
)


def sanitize_text(text: str) -> str:
    """Remove non-printable ASCII/C1 control characters from *text*.

    Printable whitespace (``\\t``, ``\\n``, ``\\r``) is preserved; every other
    control character (``\\x00``-``\\x08``, ``\\x0b``, ``\\x0c``,
    ``\\x0e``-``\\x1f``, ``\\x7f``-``\\x9f``) is dropped.
    """
    if not text:
        return text
    return _CONTROL_CHARS_RE.sub("", text)


def sanitize_value(value: Any) -> Any:
    """Recursively sanitise *value*, descending into dicts/lists/tuples.

    Strings are cleaned via :func:`sanitize_text`; all other types are
    returned unchanged (dict keys are sanitised too, when they are strings).
    """
    if isinstance(value, str):
        return sanitize_text(value)
    if isinstance(value, dict):
        return {
            (sanitize_text(key) if isinstance(key, str) else key): sanitize_value(val)
            for key, val in value.items()
        }
    if isinstance(value, list):
        return [sanitize_value(item) for item in value]
    if isinstance(value, tuple):
        return tuple(sanitize_value(item) for item in value)
    return value
