"""Exception hierarchy for the OpenAlex interface package."""

from __future__ import annotations

from typing import Any


class OpenAlexError(RuntimeError):
    """Base class for all errors raised by the OpenAlex interface.

    Covers client-side problems (invalid arguments, unreachable host,
    malformed responses) as well as, via :class:`OpenAlexAPIError`, error
    responses returned by the OpenAlex REST API itself.
    """


class OpenAlexAPIError(OpenAlexError):
    """Raised when the OpenAlex API returns an error status.

    Attributes
    ----------
    status:
        The HTTP status code returned by the API, when available.
    url:
        The request URL that produced the error (with the ``api_key`` value
        redacted so secrets never leak into logs or agent output).
    payload:
        The raw (truncated) response body, useful for debugging 4xx errors.
    """

    def __init__(
        self,
        message: str,
        *,
        status: int | None = None,
        url: str | None = None,
        payload: Any = None,
    ) -> None:
        super().__init__(message)
        self.status = status
        self.url = url
        self.payload = payload
