"""Central JSON codec — guarantees uniform escaping across all encode/decode boundaries.

Stateless encode/decode + stream helpers. All methods are ``@staticmethod``.
"""

from __future__ import annotations

import json
from typing import Any, IO, Iterator

__all__ = ["JsonCodec"]


class JsonCodec:
    """Stateless JSON encode/decode + stream helpers."""

    _COMPACT = (",", ":")

    @staticmethod
    def encode(obj: Any, *, compact: bool = False, indent: int | None = None) -> str:
        """Serialise *obj* to a JSON string."""
        separators = JsonCodec._COMPACT if compact else None
        return json.dumps(
            obj, ensure_ascii=False, separators=separators, indent=indent, default=str
        )

    @staticmethod
    def encode_bytes(obj: Any, *, compact: bool = True) -> bytes:
        """Serialise *obj* to UTF-8 bytes."""
        return JsonCodec.encode(obj, compact=compact).encode("utf-8")

    @staticmethod
    def decode(text: str) -> Any:
        """Parse a JSON string.  Raises :class:`json.JSONDecodeError`."""
        return json.loads(text)

    @staticmethod
    def decode_bytes(data: bytes, *, lenient: bool = False) -> Any:
        """Parse JSON from UTF-8 bytes.

        With ``lenient=True``, undecodable bytes are replaced instead of raising.
        """
        text = data.decode("utf-8", "replace") if lenient else data.decode("utf-8")
        return json.loads(text)

    @staticmethod
    def try_decode(value: Any) -> Any | None:
        """Return parsed JSON of *value*, or ``None`` if not JSON.

        Accepts ``str`` or ``bytes``. Never raises.
        """
        if isinstance(value, (bytes, bytearray)):
            try:
                value = bytes(value).decode("utf-8", "replace")
            except Exception:  # noqa: BLE001
                return None
        if not isinstance(value, str):
            return None
        try:
            return json.loads(value)
        except (json.JSONDecodeError, ValueError):
            return None

    @staticmethod
    def for_log(raw: Any) -> Any:
        """Return structured JSON if possible, else decoded text."""
        parsed = JsonCodec.try_decode(raw)
        if parsed is not None:
            return parsed
        if isinstance(raw, (bytes, bytearray)):
            return bytes(raw).decode("utf-8", "replace")
        return raw

    # -- container (un)wrapping
    @staticmethod
    def maybe_parse(value: Any) -> Any:
        """Parse if *value* is a JSON document string."""
        if not isinstance(value, str):
            return value
        stripped = value.strip()
        if not stripped or stripped[0] not in "{[":
            return value
        try:
            return json.loads(stripped)
        except (json.JSONDecodeError, ValueError):
            return value

    @staticmethod
    def unwrap(value: Any) -> Any:
        """Alias of :meth:`maybe_parse`."""
        return JsonCodec.maybe_parse(value)

    # -- line-delimited streams
    @staticmethod
    def write_line(stream: IO[str], obj: Any) -> None:
        """Write one JSON object as a line and flush it."""
        stream.write(JsonCodec.encode(obj, compact=True))
        stream.write("\n")
        stream.flush()

    @staticmethod
    def decode_line(line: str) -> Any | None:
        """Parse a single NDJSON line, or ``None`` if blank/unparseable."""
        line = line.strip()
        if not line:
            return None
        try:
            return json.loads(line)
        except (json.JSONDecodeError, ValueError):
            return None

    @staticmethod
    def read_lines(stream: IO[str]) -> Iterator[Any]:
        """Yield JSON objects from a text stream, skipping blank/garbage lines."""
        for line in iter(stream.readline, ""):
            obj = JsonCodec.decode_line(line)
            if obj is not None:
                yield obj