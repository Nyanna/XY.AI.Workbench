"""Common result model: items, batches and the single place that owns warnings.

An *error* means the item produced no result (``error: str``). A *warning* means
it did, with a restriction; item warnings live in the item's ``warnings`` entry,
call-wide ones in the root ``warnings`` of ``structuredContent``.
"""
from __future__ import annotations
from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field, replace
from inspect import get_annotations
from typing import Any, Callable, Generic, Iterable, Self, TypeVar, final
__all__ = [
    'WARNINGS_KEY',
    'RESERVED_NAMES',
    'WARNINGS_SCHEMA',
    'compact',
    'compact_asdict',
    'unknown_ids_warnings',
    'ToolPayload',
    'SinglePayload',
    'RawPayload',
    'ItemPayload',
    'ItemOutcome',
    'ItemResult',
    'ItemError',
    'BatchResult']
WARNINGS_KEY = 'warnings'
RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})
WARNINGS_SCHEMA: dict[str, Any] = {'type': 'array', 'items': {'type': 'string'},
                                   'description': 'Restrictions of the result that the caller should take into account.'}

def compact(value: Any) -> Any:
    """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
    if isinstance(value, dict):
        return {k: compact(v) for k, v in value.items() if v is not None and v != []}
    if isinstance(value, list):
        return [compact(v) for v in value]
    return value

def compact_asdict(obj: Any, exclude: Iterable[str]=()) -> dict[str, Any]:
    skipped = set(exclude)
    return compact({k: v for k, v in asdict(obj).items() if k not in skipped})

def _guard_fields(cls: type) -> None:
    clash = set(get_annotations(cls)) & RESERVED_NAMES
    if clash:
        raise TypeError(f'{cls.__name__}: reserved field name(s) {sorted(clash)}; use the base class API')
    if 'to_dict' in cls.__dict__:
        raise TypeError(f'{cls.__name__}: to_dict is final; override payload()')

def _checked_payload(owner: object, payload: dict[str, Any]) -> dict[str, Any]:
    clash = RESERVED_NAMES & payload.keys()
    if clash:
        raise ValueError(f'{type(owner).__name__}: payload uses reserved key(s) {sorted(clash)}')
    return payload

def unknown_ids_warnings(missing: Iterable[str]) -> list[str]:
    ids = list(missing)
    return [f'unknown or expired id(s): {ids}'] if ids else []

@dataclass(frozen=True, kw_only=True)
class ToolPayload(ABC):
    """Typed root payload of a tool result; the only way to fill ``structured_content``."""

    @abstractmethod
    def to_dict(self) -> dict[str, Any]:
        """Wire form of the payload (without root warnings)."""

    def all_warnings(self) -> list[str]:
        """Call-wide warnings that travel at the root of the tool result."""
        return []

    @property
    def failed(self) -> bool:
        """Whether part of the call failed, which withholds automatic approval."""
        return False

@dataclass(frozen=True, kw_only=True)
class SinglePayload(ToolPayload):
    """Payload of a tool that returns one result instead of a batch; fields are serialized as-is."""
    warnings: tuple[str, ...] = ()

    def __init_subclass__(cls, **kwargs: Any) -> None:
        super().__init_subclass__(**kwargs)
        _guard_fields(cls)

    def payload(self) -> dict[str, Any]:
        return compact_asdict(self, exclude={WARNINGS_KEY})

    @final
    def to_dict(self) -> dict[str, Any]:
        return _checked_payload(self, self.payload())

    @final
    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys(self.warnings))

@dataclass(frozen=True)
class RawPayload(ToolPayload):
    """Untyped payload whose shape is fixed by someone else; *reason* documents why."""
    data: dict[str, Any]
    reason: str
    warnings: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        if not self.reason:
            raise ValueError('RawPayload requires a reason for the deviation')

    def to_dict(self) -> dict[str, Any]:
        return dict(self.data)

    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys(self.warnings))

@dataclass(frozen=True)
class ItemPayload(ToolPayload):
    """A single :class:`ItemResult` used as the whole payload; its warnings move to the root."""
    item: ItemResult

    def to_dict(self) -> dict[str, Any]:
        return {k: v for k, v in self.item.to_dict().items() if k != WARNINGS_KEY}

    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys(self.item.warnings))

@dataclass(frozen=True, kw_only=True)
class Done(SinglePayload):
    """Success of a call that returns no data."""
    result: str = 'success'

@dataclass(frozen=True, kw_only=True)
class ItemOutcome(ABC):

    @abstractmethod
    def key(self) -> dict[str, Any]:
        """Input fields mirrored for result association."""

    @abstractmethod
    def payload(self) -> dict[str, Any]:
        """Outcome-specific fields; must not use reserved names."""

    @abstractmethod
    def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
        """Wire form; *payload* replaces :meth:`payload` for context-dependent rendering."""

@dataclass(frozen=True, kw_only=True)
class ItemResult(ItemOutcome):
    """An item that produced a result, possibly with restrictions (``warnings``)."""
    warnings: tuple[str, ...] = ()

    def __init_subclass__(cls, **kwargs: Any) -> None:
        super().__init_subclass__(**kwargs)
        _guard_fields(cls)

    def payload(self) -> dict[str, Any]:
        return compact_asdict(self, exclude={WARNINGS_KEY})

    @final
    def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
        body = self.payload() if payload is None else payload
        out = {**self.key(), **_checked_payload(self, body)}
        if self.warnings:
            out[WARNINGS_KEY] = list(self.warnings)
        return out

    def warn(self, *messages: str) -> Self:
        return replace(self, warnings=(*self.warnings, *messages))

@dataclass(frozen=True, kw_only=True)
class ItemError(ItemOutcome):
    """An item that produced no result."""
    error: str

    def __init_subclass__(cls, **kwargs: Any) -> None:
        super().__init_subclass__(**kwargs)
        _guard_fields(cls)

    def payload(self) -> dict[str, Any]:
        return compact_asdict(self, exclude={'error'})

    @final
    def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
        body = self.payload() if payload is None else payload
        return {**self.key(), 'error': self.error, **_checked_payload(self, body)}
R = TypeVar('R', bound=ItemResult)
E = TypeVar('E', bound=ItemError)

@dataclass(frozen=True, kw_only=True)
class BatchResult(ToolPayload, Generic[R, E]):
    results: list[R] = field(default_factory=list)
    errors: list[E] = field(default_factory=list)
    warnings: tuple[str, ...] = ()

    def __init_subclass__(cls, **kwargs: Any) -> None:
        super().__init_subclass__(**kwargs)
        _guard_fields(cls)

    def __post_init__(self) -> None:
        if not all((isinstance(r, ItemResult) for r in self.results)):
            raise TypeError('results must be ItemResult instances')
        if not all((isinstance(e, ItemError) for e in self.errors)):
            raise TypeError('errors must be ItemError instances')

    @final
    def to_dict(self, result_payload: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
        content: dict[str, Any] = {}
        if self.results:
            content['results'] = [r.to_dict() if result_payload is None else r.to_dict(payload=result_payload(r))
                                  for r in self.results]
        if self.errors:
            content['errors'] = [e.to_dict() for e in self.errors]
        return content

    @property
    def failed(self) -> bool:
        return bool(self.errors)

    def derived_warnings(self) -> tuple[str, ...]:
        """Call-wide warnings that follow from the items and so cannot be forgotten."""
        return ()

    @final
    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))