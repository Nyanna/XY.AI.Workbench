"""A complete, thin client for the OpenAlex REST API.

This module is the *interface* layer described in the project brief: it exposes
the full surface of the OpenAlex entity endpoints (list + single) with every
supported query parameter, and applies no opinionated defaults of its own.  The
higher-level *tools* layer (:mod:`xy.ai.mcpc.tools.openalex`) builds on top of
this client, wiring in presets and agent-friendly defaults.

Design notes
------------
* Only the standard library is used (``urllib``), matching the rest of MCPC.
* Every method returns the parsed JSON response as a plain ``dict`` — OpenAlex
  responses are already well structured, so no ORM-style modelling is imposed.
* Authentication follows the brief: the API key is appended to the request URL
  as ``api_key=<KEY>`` (never sent as a header).
* The ``api_key`` value is redacted from any URL surfaced through an exception.

References: https://docs.openalex.org/
"""

from __future__ import annotations

import json
import urllib.error
import urllib.parse
import urllib.request
from typing import Any, Iterable, Mapping

from .errors import OpenAlexAPIError, OpenAlexError

#: The entity endpoints exposed by OpenAlex.  Each is reachable both as a list
#: (``/works``) and as a single record (``/works/{id}``).
ENTITIES: frozenset[str] = frozenset(
    {
        "works",
        "authors",
        "sources",
        "institutions",
        "topics",
        "keywords",
        "concepts",
        "publishers",
        "funders",
    }
)

#: The three mutually exclusive full-text search parameters.  OpenAlex permits
#: at most one per request.
_SEARCH_PARAMS = ("search", "search.exact", "search.semantic")

_DEFAULT_USER_AGENT = "xy.ai.mcpc-openalex/0.1"


class OpenAlexClient:
    """Talks to the OpenAlex REST API over HTTPS (GET only).

    Parameters
    ----------
    api_key:
        OpenAlex API key.  When set it is appended to every request URL as
        ``api_key=<KEY>``.
    base_url:
        Root of the API.  Defaults to the public endpoint.
    mailto:
        Optional contact e-mail added as ``mailto=<addr>`` to opt into the
        faster "polite pool".
    timeout:
        Per-request socket timeout, in seconds.
    user_agent:
        ``User-Agent`` header sent with every request.
    """

    def __init__(
        self,
        *,
        api_key: str | None = None,
        base_url: str = "https://api.openalex.org",
        mailto: str | None = None,
        timeout: float = 30.0,
        user_agent: str = _DEFAULT_USER_AGENT,
    ) -> None:
        self.api_key = api_key
        self.base_url = base_url.rstrip("/")
        self.mailto = mailto
        self.timeout = timeout
        self.user_agent = user_agent

    # ------------------------------------------------------------------ API
    def list_entities(
        self,
        entity: str,
        *,
        search: str | None = None,
        search_exact: str | None = None,
        search_semantic: str | None = None,
        filters: str | Mapping[str, Any] | Iterable[tuple[str, Any]] | None = None,
        sort: str | Iterable[str] | None = None,
        select: str | Iterable[str] | None = None,
        page: int | None = None,
        per_page: int | None = None,
        sample: int | None = None,
        seed: int | None = None,
        group_by: str | Iterable[str] | None = None,
        cursor: str | None = None,
        extra: Mapping[str, Any] | None = None,
    ) -> dict[str, Any]:
        """List records for *entity*, returning the raw OpenAlex response.

        The response has the standard OpenAlex shape: ``meta`` (count, page,
        per_page, ...), ``results`` (the records), and optionally ``group_by``.

        Every documented list parameter is supported.  ``filters`` accepts a
        pre-formatted filter string, a mapping (``{"is_oa": True}``), or an
        iterable of ``(key, value)`` pairs; list values become OR groups
        (joined with ``|``).  At most one of ``search`` / ``search_exact`` /
        ``search_semantic`` may be provided.
        """
        entity = self._check_entity(entity)
        params: dict[str, Any] = {}

        provided = [
            (name, value)
            for name, value in (
                ("search", search),
                ("search.exact", search_exact),
                ("search.semantic", search_semantic),
            )
            if value not in (None, "")
        ]
        if len(provided) > 1:
            raise OpenAlexError(
                "Only one of search, search.exact or search.semantic may be "
                "used per request."
            )
        for name, value in provided:
            params[name] = value

        filter_str = self._format_filters(filters)
        if filter_str:
            params["filter"] = filter_str
        sort_str = self._format_csv(sort)
        if sort_str:
            params["sort"] = sort_str
        select_str = self._format_csv(select)
        if select_str:
            params["select"] = select_str
        group_str = self._format_csv(group_by)
        if group_str:
            params["group_by"] = group_str
        if page is not None:
            params["page"] = page
        if per_page is not None:
            params["per-page"] = per_page
        if sample is not None:
            params["sample"] = sample
        if seed is not None:
            params["seed"] = seed
        if cursor is not None:
            params["cursor"] = cursor
        if extra:
            params.update(extra)

        return self._request(f"/{entity}", params)

    def get_entity(
        self,
        entity: str,
        entity_id: str,
        *,
        select: str | Iterable[str] | None = None,
    ) -> dict[str, Any]:
        """Fetch a single record by id, DOI, or other supported external id.

        *entity_id* may be an OpenAlex id (``W2741809807``), an OpenAlex URL,
        a bare DOI (``10.7717/peerj.4375``), a DOI URL, or a namespaced id
        such as ``pmid:14907713`` — all are normalised for the API.
        """
        entity = self._check_entity(entity)
        ident = self._normalize_id(entity_id)
        params: dict[str, Any] = {}
        select_str = self._format_csv(select)
        if select_str:
            params["select"] = select_str
        return self._request(f"/{entity}/{ident}", params)

    # ----------------------------------------------------- convenience (works)
    def search_works(
        self,
        query: str,
        *,
        exact: bool = False,
        **kwargs: Any,
    ) -> dict[str, Any]:
        """Full-text search over works (``search`` or ``search.exact``)."""
        if exact:
            return self.list_entities("works", search_exact=query, **kwargs)
        return self.list_entities("works", search=query, **kwargs)

    def semantic_search_works(self, query: str, **kwargs: Any) -> dict[str, Any]:
        """Meaning-based (embedding) search over works (``search.semantic``)."""
        return self.list_entities("works", search_semantic=query, **kwargs)

    def get_work(
        self, work_id: str, *, select: str | Iterable[str] | None = None
    ) -> dict[str, Any]:
        """Fetch a single work by id/DOI/external id."""
        return self.get_entity("works", work_id, select=select)

    # --------------------------------------------------------------- internals
    @staticmethod
    def _check_entity(entity: str) -> str:
        normalised = entity.strip().lower()
        if normalised not in ENTITIES:
            raise OpenAlexError(
                f"Unknown entity type: {entity!r}. "
                f"Valid types: {', '.join(sorted(ENTITIES))}."
            )
        return normalised

    def _format_filters(
        self,
        filters: str | Mapping[str, Any] | Iterable[tuple[str, Any]] | None,
    ) -> str | None:
        if filters is None:
            return None
        if isinstance(filters, str):
            return filters.strip() or None
        if isinstance(filters, Mapping):
            pairs: Iterable[tuple[str, Any]] = filters.items()
        else:
            pairs = filters  # assume iterable of (key, value)
        parts = [f"{key}:{self._filter_value(value)}" for key, value in pairs]
        return ",".join(parts) or None

    @staticmethod
    def _filter_value(value: Any) -> str:
        if isinstance(value, bool):
            return "true" if value else "false"
        if isinstance(value, (list, tuple)):
            return "|".join(str(item) for item in value)
        return str(value)

    @staticmethod
    def _format_csv(value: str | Iterable[str] | None) -> str | None:
        if value is None:
            return None
        if isinstance(value, str):
            return value.strip() or None
        joined = ",".join(str(item) for item in value)
        return joined or None

    @staticmethod
    def _normalize_id(value: str) -> str:
        ident = value.strip()
        low = ident.lower()
        if low.startswith(("http://", "https://")):
            if "doi.org/" in low:
                return "doi:" + ident.split("doi.org/", 1)[1]
            # Any other OpenAlex/ROR/ORCID URL: keep the trailing segment.
            return ident.rstrip("/").rsplit("/", 1)[-1]
        # A bare DOI ("10.x/....") -> namespaced form the API understands.
        if low.startswith("10.") and "/" in ident:
            return "doi:" + ident
        return ident

    def _build_url(self, path: str, params: Mapping[str, Any]) -> str:
        query = dict(params)
        if self.mailto and "mailto" not in query:
            query["mailto"] = self.mailto
        if self.api_key:
            query["api_key"] = self.api_key
        url = f"{self.base_url}{path}"
        if query:
            url = f"{url}?{urllib.parse.urlencode(query)}"
        return url

    def _request(self, path: str, params: Mapping[str, Any]) -> dict[str, Any]:
        url = self._build_url(path, params)
        safe_url = _redact_api_key(url)
        request = urllib.request.Request(
            url,
            method="GET",
            headers={"User-Agent": self.user_agent, "Accept": "application/json"},
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout) as response:
                body = response.read()
        except urllib.error.HTTPError as exc:
            detail = exc.read().decode("utf-8", "replace")
            message = _extract_api_error(detail) or f"HTTP {exc.code} error"
            raise OpenAlexAPIError(
                f"OpenAlex request failed: {message}",
                status=exc.code,
                url=safe_url,
                payload=detail[:1000],
            ) from exc
        except (urllib.error.URLError, OSError) as exc:
            raise OpenAlexAPIError(
                f"Cannot reach OpenAlex ({safe_url}): {exc}", url=safe_url
            ) from exc

        try:
            parsed = json.loads(body.decode("utf-8", "replace"))
        except json.JSONDecodeError as exc:
            raise OpenAlexAPIError(
                f"Malformed JSON in OpenAlex response: {exc}", url=safe_url
            ) from exc
        if not isinstance(parsed, dict):
            raise OpenAlexAPIError(
                "Unexpected OpenAlex response (not a JSON object).", url=safe_url
            )
        return parsed


def _redact_api_key(url: str) -> str:
    """Return *url* with the value of any ``api_key`` parameter masked."""
    split = urllib.parse.urlsplit(url)
    if not split.query:
        return url
    pairs = urllib.parse.parse_qsl(split.query, keep_blank_values=True)
    redacted = [
        (key, "***" if key == "api_key" else value) for key, value in pairs
    ]
    new_query = urllib.parse.urlencode(redacted)
    return urllib.parse.urlunsplit(split._replace(query=new_query))


def _extract_api_error(body: str) -> str | None:
    """Pull a human-readable message out of an OpenAlex error body."""
    try:
        data = json.loads(body)
    except (json.JSONDecodeError, TypeError):
        return None
    if isinstance(data, dict):
        message = data.get("message") or data.get("error")
        if isinstance(message, str) and message.strip():
            return message.strip()
    return None
