"""Field presets and result post-processing for the OpenAlex tools layer.

The OpenAlex ``select`` parameter can request any subset of an entity's
top-level fields.  Rather than expose that raw surface to an AI agent, the tools
offer a handful of **semantic presets** — named bundles of fields chosen for a
particular intent (bibliographic metadata, open-access status, citation
metrics, ...).  ``full`` maps to *no* ``select`` (every field is returned).

Presets are intentionally kept to top-level fields only, which is all the
OpenAlex ``select`` parameter accepts for works.
"""

from __future__ import annotations

from typing import Any

#: Fields that carry the essential identity of any entity.  Used as the base
#: preset for the non-``works`` entity types, whose object shapes differ.
_GENERIC_MINIMAL = ["id", "display_name", "relevance_score"]

#: Presets for the ``works`` entity.  ``None`` means "no select" (all fields).
WORK_PRESETS: dict[str, list[str] | None] = {
    # Just enough to identify and rank a hit.
    "minimal": [
        "id",
        "doi",
        "title",
        "publication_year",
        "type",
        "cited_by_count",
        "relevance_score",
    ],
    # Sensible default: identity, venue, access and impact at a glance.
    "core": [
        "id",
        "doi",
        "title",
        "display_name",
        "publication_year",
        "publication_date",
        "type",
        "language",
        "primary_location",
        "open_access",
        "cited_by_count",
        "primary_topic",
        "relevance_score",
    ],
    # Everything needed to build a citation / reference entry.
    "bibliographic": [
        "id",
        "doi",
        "title",
        "display_name",
        "publication_year",
        "publication_date",
        "type",
        "biblio",
        "primary_location",
        "authorships",
        "language",
        "relevance_score",
    ],
    # Who wrote it and where they are affiliated.
    "authorship": [
        "id",
        "title",
        "publication_year",
        "authorships",
        "corresponding_author_ids",
        "corresponding_institution_ids",
        "countries_distinct_count",
        "institutions_distinct_count",
        "relevance_score",
    ],
    # Open-access status, locations and article-processing charges.
    "access": [
        "id",
        "doi",
        "title",
        "publication_year",
        "open_access",
        "best_oa_location",
        "primary_location",
        "locations",
        "has_fulltext",
        "apc_list",
        "apc_paid",
        "relevance_score",
    ],
    # Citation impact and reference counts over time.
    "metrics": [
        "id",
        "title",
        "publication_year",
        "cited_by_count",
        "cited_by_percentile_year",
        "counts_by_year",
        "referenced_works_count",
        "relevance_score",
    ],
    # Subject classification: topics, keywords, concepts, SDGs, MeSH.
    "topics": [
        "id",
        "title",
        "publication_year",
        "primary_topic",
        "topics",
        "keywords",
        "concepts",
        "sustainable_development_goals",
        "mesh",
        "relevance_score",
    ],
    # Title plus the (reconstructed) abstract text.
    "abstract": [
        "id",
        "doi",
        "title",
        "publication_year",
        "abstract_inverted_index",
        "relevance_score",
    ],
    # Citation graph: referenced and related works.
    "references": [
        "id",
        "title",
        "publication_year",
        "referenced_works",
        "referenced_works_count",
        "related_works",
        "relevance_score",
    ],
    # Everything OpenAlex returns for the work.
    "full": None,
}

#: Presets available for non-``works`` entities (authors, sources, ...).
GENERIC_PRESETS: dict[str, list[str] | None] = {
    "minimal": _GENERIC_MINIMAL,
    "full": None,
}

#: Preset names an agent may pick for a works search / lookup.
WORK_PRESET_NAMES: tuple[str, ...] = tuple(WORK_PRESETS)

#: Defaults chosen for each tool.
DEFAULT_SEARCH_PRESET = "core"
DEFAULT_WORK_PRESET = "full"


def resolve_select(preset: str | None, entity: str) -> list[str] | None:
    """Translate a preset name into a ``select`` field list (or ``None``).

    ``None`` (returned for the ``full`` preset or an unknown name on an entity
    with no rich presets) means: send no ``select`` and let OpenAlex return
    every field.  Unknown preset names fall back to a sensible default rather
    than raising, so an agent typo never hard-fails a lookup.
    """
    table = WORK_PRESETS if entity == "works" else GENERIC_PRESETS
    if preset in table:
        return table[preset]
    # Unknown preset -> fall back to the entity's default.
    if entity == "works":
        return WORK_PRESETS[DEFAULT_SEARCH_PRESET]
    return GENERIC_PRESETS["minimal"]


def reconstruct_abstract(inverted_index: dict[str, list[int]] | None) -> str | None:
    """Rebuild readable abstract text from OpenAlex's inverted index.

    OpenAlex stores abstracts as ``{word: [positions]}``.  This restores the
    original word order.  Returns ``None`` for an empty or missing index.
    """
    if not inverted_index:
        return None
    positioned: list[tuple[int, str]] = []
    for word, positions in inverted_index.items():
        for position in positions:
            positioned.append((position, word))
    if not positioned:
        return None
    positioned.sort(key=lambda item: item[0])
    return " ".join(word for _, word in positioned)


def project_results(results: list[Any]) -> list[Any]:
    """Post-process raw records for agent consumption.

    The only transformation applied is replacing the machine-oriented
    ``abstract_inverted_index`` with a plain-text ``abstract`` field, which is
    both smaller and directly readable.  All other fields pass through
    untouched.
    """
    processed: list[Any] = []
    for item in results:
        processed.append(_project_one(item))
    return processed


def _project_one(item: Any) -> Any:
    if not isinstance(item, dict) or "abstract_inverted_index" not in item:
        return item
    clone = dict(item)
    abstract = reconstruct_abstract(clone.pop("abstract_inverted_index"))
    if abstract is not None:
        clone["abstract"] = abstract
    return clone
