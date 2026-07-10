"""Central JSON / stream codec — the single source of truth for escaping.

Every boundary where a JSON value is turned into bytes/text (or back) must go
through :class:`JsonCodec`.  This guarantees that *escaping* and *unescaping*
are applied exactly once and always with the same options, which is what keeps
values intact as they are re-wrapped in and out of containers (an HTTP body, a
WebSocket frame, a control payload, a subprocess' STDIN/STDOUT/STDERR line, or
a log record).

Why a single class matters
--------------------------
JSON escaping is only correct when three rules hold everywhere:

1. **Never escape by hand.**  Backslash/quote/unicode escaping is delegated to
   the ``json`` module.  Manual escaping is what produces the ``[\\\\s\\\\S]``
   backslash explosions seen in the wild.
2. **One canonical option set.**  ``ensure_ascii=False`` everywhere, so a
   non-ASCII or backslash-bearing string is written verbatim (as UTF-8) instead
   of as ``\\uXXXX`` by one call site and literally by another.  Mixed options
   are what make a round-trip look "over-escaped".
3. **Encode exactly once per container.**  A value crossing *n* container
   boundaries is JSON-encoded *n* times and decoded *n* times — never more.
   :meth:`unwrap` / :meth:`maybe_parse` guard against a JSON document that is
   accidentally carried as an opaque string and then encoded a second time.
"""

from __future__ import annotations

import json
from typing import Any, IO, Iterator

__all__ = ["JsonCodec"]


class JsonCodec:
    """Stateless JSON encode/decode + stream helpers with uniform escaping.

    All methods are ``@staticmethod``; the class is a namespace, not a value.
    Two encodings are offered and they escape identically — they differ only in
    whitespace:

    * *pretty* (default) keeps the ``json`` default separators, for logs and for
      text blocks a human reads;
    * *compact* (``compact=True``) drops insignificant whitespace, for the wire.
    """

    #: Compact separators for on-the-wire payloads (no incidental whitespace).
    _COMPACT = (",", ":")

    # -- encoding -----------------------------------------------------------
    @staticmethod
    def encode(obj: Any, *, compact: bool = False, indent: int | None = None) -> str:
        """Serialise *obj* to a JSON string with canonical escaping.

        ``ensure_ascii=False`` keeps text (backslashes, quotes, non-ASCII)
        readable and lets a single downstream ``json`` call own the escaping.
        ``default=str`` makes the call total: any exotic object degrades to its
        ``str()`` rather than raising mid-serialisation.  ``indent`` pretty-prints
        (for human-facing text blocks) and is mutually exclusive with ``compact``.
        """
        separators = JsonCodec._COMPACT if compact else None
        return json.dumps(
            obj, ensure_ascii=False, separators=separators, indent=indent, default=str
        )

    @staticmethod
    def encode_bytes(obj: Any, *, compact: bool = True) -> bytes:
        """Serialise *obj* to UTF-8 bytes (compact by default, for the wire)."""
        return JsonCodec.encode(obj, compact=compact).encode("utf-8")

    # -- decoding -----------------------------------------------------------
    @staticmethod
    def decode(text: str) -> Any:
        """Parse a JSON string.  Raises :class:`json.JSONDecodeError`."""
        return json.loads(text)

    @staticmethod
    def decode_bytes(data: bytes, *, lenient: bool = False) -> Any:
        """Parse JSON from UTF-8 bytes.

        With ``lenient=True`` undecodable bytes are replaced (``errors="replace"``)
        instead of raising :class:`UnicodeDecodeError` — use it only where a
        best-effort read is acceptable (e.g. remote responses, diagnostics).
        """
        text = data.decode("utf-8", "replace") if lenient else data.decode("utf-8")
        return json.loads(text)

    @staticmethod
    def try_decode(value: Any) -> Any | None:
        """Return the parsed JSON of *value*, or ``None`` if it is not JSON.

        Accepts ``str`` or ``bytes``.  Never raises; used for tolerant paths
        such as logging a body that may or may not be well-formed JSON.
        """
        if isinstance(value, (bytes, bytearray)):
            try:
                value = bytes(value).decode("utf-8", "replace")
            except Exception:  # noqa: BLE001 - defensive, decode with replace can't raise
                return None
        if not isinstance(value, str):
            return None
        try:
            return json.loads(value)
        except (json.JSONDecodeError, ValueError):
            return None

    @staticmethod
    def for_log(raw: Any) -> Any:
        """Normalise *raw* for a log record: parsed JSON when possible.

        Replaces the scattered ``raw.decode("utf-8", "replace")`` idiom.  Bytes
        that are valid JSON become the object (so the log stays structured);
        otherwise the replacement-decoded text is returned so nothing is lost.
        """
        parsed = JsonCodec.try_decode(raw)
        if parsed is not None:
            return parsed
        if isinstance(raw, (bytes, bytearray)):
            return bytes(raw).decode("utf-8", "replace")
        return raw

    # -- container (un)wrapping --------------------------------------------
    @staticmethod
    def maybe_parse(value: Any) -> Any:
        """Unwrap a JSON *document* that is being carried as a string.

        When a container hands us a ``str`` whose whole content is a JSON object
        or array, it is parsed so the value is not encoded a *second* time when
        the surrounding structure is serialised (which is exactly what doubles
        the escaping).  Plain strings, numbers and already-parsed values pass
        through unchanged.
        """
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
        """Alias of :meth:`maybe_parse`, read at the *consuming* end."""
        return JsonCodec.maybe_parse(value)

    # -- line-delimited streams (STDIN / STDOUT / STDERR) -------------------
    @staticmethod
    def write_line(stream: IO[str], obj: Any) -> None:
        """Write one compact JSON object as a line and flush it.

        This is the canonical way to push a message into a subprocess' STDIN:
        the object is escaped once, terminated with ``\\n`` and flushed so the
        peer's line reader sees a complete record immediately.
        """
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
        """Yield JSON objects from a text stream, skipping blank/garbage lines.

        Stops at EOF (empty read).  Malformed lines are skipped rather than
        aborting the stream, matching stream-json's forgiving framing.
        """
        for line in iter(stream.readline, ""):
            obj = JsonCodec.decode_line(line)
            if obj is not None:
                yield obj
