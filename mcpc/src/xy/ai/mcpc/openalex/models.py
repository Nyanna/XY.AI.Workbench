"""Typed entity records for the OpenAlex API, mirroring ``openapi_openalex.json``.

Each dataclass covers the top-level fields of one OpenAlex entity schema.
Nested/complex sub-structures (locations, authorships, ``ids``, ...) are kept
as plain ``dict``/``list`` since their shape does not depend on the ``select``
presets and typing them would add depth without changing how callers use them.

Every field defaults to ``None`` because OpenAlex's ``select`` parameter (and
the tools-layer field presets) may return only a subset of an entity's
fields.
"""

from __future__ import annotations

import dataclasses
from dataclasses import dataclass
from typing import Any

__all__ = [
    "GroupByItem",
    "Work",
    "Author",
    "Source",
    "Institution",
    "Topic",
    "Keyword",
    "Concept",
    "Publisher",
    "Funder",
    "OpenAlexRecord",
    "ENTITY_MODELS",
    "parse_entity",
    "parse_group_by",
]


@dataclass(frozen=True, slots=True)
class GroupByItem:
    """One bucket of a ``group_by`` aggregation."""

    key: str | None = None
    key_display_name: str | None = None
    count: int | None = None


@dataclass(frozen=True, slots=True)
class Work:
    """A scholarly document (article, book, dataset, thesis, ...)."""

    id: str | None = None
    doi: str | None = None
    title: str | None = None
    display_name: str | None = None
    publication_year: int | None = None
    publication_date: str | None = None
    type: str | None = None
    language: str | None = None
    cited_by_count: int | None = None
    is_retracted: bool | None = None
    is_paratext: bool | None = None
    primary_location: dict[str, Any] | None = None
    locations: list[dict[str, Any]] | None = None
    best_oa_location: dict[str, Any] | None = None
    open_access: dict[str, Any] | None = None
    authorships: list[dict[str, Any]] | None = None
    ids: dict[str, Any] | None = None
    biblio: dict[str, Any] | None = None
    # Reconstructed by xy.ai.mcpc.openalex.presets.project_results from
    # OpenAlex's abstract_inverted_index.
    abstract: str | None = None
    referenced_works: list[str] | None = None
    referenced_works_count: int | None = None
    related_works: list[str] | None = None
    topics: list[dict[str, Any]] | None = None
    primary_topic: dict[str, Any] | None = None
    keywords: list[dict[str, Any]] | None = None
    funders: list[dict[str, Any]] | None = None
    awards: list[dict[str, Any]] | None = None
    fwci: float | None = None
    citation_normalized_percentile: dict[str, Any] | None = None
    cited_by_percentile_year: dict[str, Any] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    sustainable_development_goals: list[dict[str, Any]] | None = None
    mesh: list[dict[str, Any]] | None = None
    indexed_in: list[str] | None = None
    has_content: dict[str, Any] | None = None
    content_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    # Only present on search/list responses, not on single-record fetches.
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Author:
    id: str | None = None
    orcid: str | None = None
    display_name: str | None = None
    display_name_alternatives: list[str] | None = None
    longest_name: str | None = None
    parsed_longest_name: dict[str, Any] | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    summary_stats: dict[str, Any] | None = None
    affiliations: list[dict[str, Any]] | None = None
    last_known_institutions: list[dict[str, Any]] | None = None
    topics: list[dict[str, Any]] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    ids: dict[str, Any] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Source:
    id: str | None = None
    issn_l: str | None = None
    issn: list[str] | None = None
    display_name: str | None = None
    type: str | None = None
    host_organization: str | None = None
    host_organization_name: str | None = None
    host_organization_lineage: list[str] | None = None
    is_oa: bool | None = None
    is_in_doaj: bool | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    summary_stats: dict[str, Any] | None = None
    apc_usd: int | None = None
    homepage_url: str | None = None
    ids: dict[str, Any] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Institution:
    id: str | None = None
    ror: str | None = None
    display_name: str | None = None
    country_code: str | None = None
    type: str | None = None
    homepage_url: str | None = None
    image_url: str | None = None
    image_thumbnail_url: str | None = None
    display_name_acronyms: list[str] | None = None
    display_name_alternatives: list[str] | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    summary_stats: dict[str, Any] | None = None
    geo: dict[str, Any] | None = None
    lineage: list[str] | None = None
    ids: dict[str, Any] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Topic:
    id: str | None = None
    display_name: str | None = None
    description: str | None = None
    keywords: list[str] | None = None
    subfield: dict[str, Any] | None = None
    field: dict[str, Any] | None = None
    domain: dict[str, Any] | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    ids: dict[str, Any] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Keyword:
    id: str | None = None
    display_name: str | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Concept:
    id: str | None = None
    display_name: str | None = None
    description: str | None = None
    level: int | None = None
    wikidata: str | None = None
    ids: dict[str, Any] | None = None
    image_url: str | None = None
    image_thumbnail_url: str | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    ancestors: list[dict[str, Any]] | None = None
    related_concepts: list[dict[str, Any]] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Publisher:
    id: str | None = None
    display_name: str | None = None
    alternate_titles: list[str] | None = None
    country_codes: list[str] | None = None
    hierarchy_level: int | None = None
    parent_publisher: dict[str, Any] | None = None
    lineage: list[str] | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    sources_api_url: str | None = None
    ids: dict[str, Any] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


@dataclass(frozen=True, slots=True)
class Funder:
    id: str | None = None
    display_name: str | None = None
    alternate_titles: list[str] | None = None
    country_code: str | None = None
    description: str | None = None
    homepage_url: str | None = None
    image_url: str | None = None
    image_thumbnail_url: str | None = None
    grants_count: int | None = None
    works_count: int | None = None
    cited_by_count: int | None = None
    ids: dict[str, Any] | None = None
    counts_by_year: list[dict[str, Any]] | None = None
    works_api_url: str | None = None
    created_date: str | None = None
    updated_date: str | None = None
    relevance_score: float | None = None


#: Union of every entity record type a search/lookup can return.
OpenAlexRecord = (
    Work | Author | Source | Institution | Topic | Keyword | Concept | Publisher | Funder
)

#: Maps an OpenAlex entity name (as used by the client/ENTITIES) to its dataclass.
ENTITY_MODELS: dict[str, type[OpenAlexRecord]] = {
    "works": Work,
    "authors": Author,
    "sources": Source,
    "institutions": Institution,
    "topics": Topic,
    "keywords": Keyword,
    "concepts": Concept,
    "publishers": Publisher,
    "funders": Funder,
}


def parse_entity(entity: str, data: dict[str, Any]) -> OpenAlexRecord:
    """Build the dataclass matching *entity* from a raw OpenAlex record.

    Keys not defined on the target dataclass (e.g. from a newer OpenAlex
    field, or a foreign shape) are silently dropped rather than raising.
    """
    model = ENTITY_MODELS.get(entity, Work)
    known = {f.name for f in dataclasses.fields(model)}
    return model(**{key: value for key, value in data.items() if key in known})


def parse_group_by(items: list[dict[str, Any]]) -> list[GroupByItem]:
    """Build :class:`GroupByItem` records from a raw ``group_by`` list."""
    known = {f.name for f in dataclasses.fields(GroupByItem)}
    return [
        GroupByItem(**{key: value for key, value in item.items() if key in known})
        for item in items
    ]
