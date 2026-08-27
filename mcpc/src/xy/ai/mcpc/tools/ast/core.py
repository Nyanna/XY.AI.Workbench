"""Shared AST machinery for the ``python-ast-*`` tool family.

Central pieces:

* :class:`AstCache` – a process-wide cache of parsed modules, validated on every
  access by ``st_mtime_ns`` and, on change, by a content hash before re-parsing.
* comment handling – existing ``#`` comments are converted into standalone
  string-literal annotations (:func:`comments_to_annotations`) before parsing so
  they survive the round-trip through :func:`ast.parse` / :func:`ast.unparse`.
* node location – :func:`locate_all` / :func:`node_summary` expose the subset of
  nodes (imports, classes, functions and top-level statements) the tools act on,
  each with its Python-style qualified name.

``import ast`` inside this package resolves to the standard library module
(absolute import), not the package itself.
"""

from __future__ import annotations

import ast
import hashlib
import io
import re
import threading
import tokenize
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


class AstError(Exception):
    """A user-facing, path-free error raised by the AST tools."""

def _annotation_literal(comment: str) -> str:
    """Return a Python source literal representing *comment* (incl. its ``#``)."""
    return repr(comment.rstrip())


#: Matches the header line of a clause that must directly follow its sibling
#: clause (``elif``/``else``/``except``/``finally``) or, heuristically, a
#: ``match`` statement's ``case`` (a soft keyword, so it additionally requires
#: the line to end in a colon to avoid matching a plain ``case = ...``
#: assignment). No statement, including an injected annotation literal, may be
#: placed *between* such a header and the suite it continues.
_CONTINUATION_HEADER_RE = re.compile(r"^\s*(elif|else|except|finally)\b")
_CASE_HEADER_RE = re.compile(r"^\s*case\b.*:\s*(#.*)?$")


def _is_continuation_header(line: str) -> bool:
    """Whether *line* opens a clause that must immediately follow its sibling clause."""
    return bool(_CONTINUATION_HEADER_RE.match(line) or _CASE_HEADER_RE.match(line))


def _next_code_line_index(lines: list[str], start: int) -> int | None:
    """Return the 0-based index of the first non-blank, non-comment-only line at/after *start*."""
    for i in range(start, len(lines)):
        stripped = lines[i].strip()
        if stripped == "" or stripped.startswith("#"):
            continue
        return i
    return None


def _suite_indent(lines: list[str], header_lineno: int) -> str:
    """Return the indentation of the suite opened by the 1-based *header_lineno* line.

    Falls back to the header's own indentation plus four spaces if the suite's
    first line cannot be found (e.g. the header is the last line of the file).
    """
    header_line = lines[header_lineno - 1]
    header_indent = header_line[: len(header_line) - len(header_line.lstrip())]
    for line in lines[header_lineno:]:
        if line.strip() == "":
            continue
        return line[: len(line) - len(line.lstrip())]
    return header_indent + "    "


def comments_to_annotations(source: str) -> str:
    """Rewrite ``#`` comments into standalone string-literal statements.

    A comment on its own line becomes an equally-indented string literal; a
    trailing comment is lifted onto its own literal line in front of the
    statement it belonged to. Comments inside brackets/continuations cannot be
    represented as standalone literals without breaking syntax and are dropped.
    Style and exact placement are explicitly *not* preserved – only semantics
    plus the recovered annotation text.

    A comment immediately preceding, or trailing on, an ``elif``/``else``/
    ``except``/``finally``/``case`` header is special-cased: such a header must
    directly follow its sibling clause, so no literal may precede it. The
    literal is instead placed as the first statement inside the suite the
    header opens (see :func:`_is_continuation_header`).
    """
    if "#" not in source:
        return source

    lines = source.splitlines(keepends=True)
    replaces: dict[int, str] = {}
    strips: dict[int, int] = {}
    inserts: dict[int, list[str]] = {}

    depth = 0
    logical_start: int | None = None
    try:
        for tok in tokenize.generate_tokens(io.StringIO(source).readline):
            ttype = tok.type
            if ttype == tokenize.NEWLINE:
                logical_start = None
                continue
            if ttype in (
                tokenize.NL,
                tokenize.INDENT,
                tokenize.DEDENT,
                tokenize.ENCODING,
                tokenize.ENDMARKER,
            ):
                continue
            if ttype == tokenize.COMMENT:
                lineno, col = tok.start
                prefix = lines[lineno - 1][:col]
                standalone = prefix.strip() == ""
                literal = _annotation_literal(tok.string)
                if depth == 0 and standalone and logical_start is None:
                    next_idx = _next_code_line_index(lines, lineno)
                    if next_idx is not None and _is_continuation_header(lines[next_idx]):
                        header_lineno = next_idx + 1
                        target_indent = _suite_indent(lines, header_lineno)
                        inserts.setdefault(header_lineno + 1, []).append(f"{target_indent}{literal}\n")
                        replaces[lineno] = "\n"
                    else:
                        replaces[lineno] = f"{prefix}{literal}\n"
                elif depth == 0 and not standalone and logical_start is not None:
                    stmt_line = lines[logical_start - 1]
                    if _is_continuation_header(stmt_line):
                        target_indent = _suite_indent(lines, lineno)
                        inserts.setdefault(lineno + 1, []).append(f"{target_indent}{literal}\n")
                    else:
                        indent = stmt_line[: len(stmt_line) - len(stmt_line.lstrip())]
                        inserts.setdefault(logical_start, []).append(f"{indent}{literal}\n")
                    strips[lineno] = col
                elif standalone:
                    replaces[lineno] = "\n"
                else:
                    strips[lineno] = col
                continue

            if logical_start is None:
                logical_start = tok.start[0]
            if ttype == tokenize.OP:
                if tok.string in "([{":
                    depth += 1
                elif tok.string in ")]}":
                    depth = max(0, depth - 1)
    except (tokenize.TokenError, IndentationError):
        # Malformed source: let the real parser produce the error later.
        return source

    out: list[str] = []
    for i, line in enumerate(lines, start=1):
        if i in inserts:
            out.extend(inserts[i])
        if i in replaces:
            out.append(replaces[i])
        elif i in strips:
            out.append(line[: strips[i]].rstrip() + "\n")
        else:
            out.append(line)
    return "".join(out)


def parse_source(source: str) -> ast.Module:
    """Parse *source* into a module, converting comments to annotations first."""
    try:
        return ast.parse(comments_to_annotations(source))
    except SyntaxError as exc:
        raise AstError(f"Syntax error: {exc.msg} (line {exc.lineno})") from exc


def parse_snippet(code: str) -> list[ast.stmt]:
    """Parse *code* into a list of top-level statement nodes."""
    return parse_source(code).body


def unparse(tree: ast.AST) -> str:
    """Serialise *tree* back to source, filling in any missing locations."""
    return ast.unparse(ast.fix_missing_locations(tree))

@dataclass
class _CacheEntry:
    mtime_ns: int
    content_hash: str
    tree: ast.Module


class AstCache:
    """Content-hash validated cache of parsed modules keyed by absolute path."""

    def __init__(self) -> None:
        self._entries: dict[str, _CacheEntry] = {}
        self._lock = threading.RLock()

    def get_tree(self, path: Path) -> ast.Module:
        key = str(path)
        with self._lock:
            entry = self._entries.get(key)
            mtime_ns = path.stat().st_mtime_ns
            if entry is not None and entry.mtime_ns == mtime_ns:
                return entry.tree
            source = path.read_text(encoding="utf-8")
            digest = hashlib.sha256(source.encode("utf-8")).hexdigest()
            if entry is not None and entry.content_hash == digest:
                entry.mtime_ns = mtime_ns
                return entry.tree
            tree = parse_source(source)
            self._entries[key] = _CacheEntry(mtime_ns, digest, tree)
            return tree

    def save(self, path: Path, tree: ast.Module) -> str:
        """Unparse *tree*, write it to *path* and refresh the cache entry."""
        source = unparse(tree)
        path.write_text(source, encoding="utf-8")
        # Re-parse so cached line numbers match the file exactly.
        normalized = ast.parse(source)
        digest = hashlib.sha256(source.encode("utf-8")).hexdigest()
        with self._lock:
            self._entries[str(path)] = _CacheEntry(
                path.stat().st_mtime_ns, digest, normalized
            )
        return source

    def invalidate(self, path: Path) -> None:
        with self._lock:
            self._entries.pop(str(path), None)


#: Process-wide shared cache instance.
CACHE = AstCache()

#: AST node types exposed by the structural tools (outline / list / find).
_DEF_TYPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
_IMPORT_TYPES = (ast.Import, ast.ImportFrom)


@dataclass
class Located:
    """A statement node together with its container and qualified name."""

    node: ast.stmt
    name: str | None
    qualified_name: str | None
    parent: ast.AST  # Module / ClassDef / FunctionDef whose ``body`` holds node
    index: int


def import_names(node: ast.Import | ast.ImportFrom) -> str:
    """Return a compact, canonical description of an import statement."""
    if isinstance(node, ast.Import):
        return ", ".join(
            a.name + (f" as {a.asname}" if a.asname else "") for a in node.names
        )
    module = ("." * node.level) + (node.module or "")
    imported = ", ".join(
        a.name + (f" as {a.asname}" if a.asname else "") for a in node.names
    )
    return f"{module}:{imported}"


def locate_all(tree: ast.Module) -> list[Located]:
    """Flatten *tree* into located statements (recursing into class/def bodies)."""
    results: list[Located] = []

    def walk(container: ast.AST, prefix: str) -> None:
        for index, node in enumerate(getattr(container, "body", [])):
            if isinstance(node, _IMPORT_TYPES):
                name = import_names(node)
                results.append(Located(node, name, name, container, index))
            elif isinstance(node, _DEF_TYPES):
                qual = f"{prefix}.{node.name}" if prefix else node.name
                results.append(Located(node, node.name, qual, container, index))
                walk(node, qual)
            else:
                results.append(Located(node, None, None, container, index))

    walk(tree, "")
    return results


def short_docstring(node: ast.AST, limit: int = 80) -> str | None:
    """Return the node's docstring truncated to *limit* characters, if any."""
    if not isinstance(node, (ast.Module, ast.ClassDef, *_DEF_TYPES)):
        return None
    doc = ast.get_docstring(node, clean=True)
    if doc is None:
        return None
    doc = " ".join(doc.split())
    return doc if len(doc) <= limit else doc[: limit - 1] + "…"


def node_summary(loc: Located) -> dict[str, object]:
    node = loc.node
    return {
        "type": type(node).__name__,
        "name": loc.name,
        "qualified_name": loc.qualified_name,
        "lineno": node.lineno,
        "end_lineno": getattr(node, "end_lineno", node.lineno),
        "parent_type": type(loc.parent).__name__,
        "docstring": short_docstring(node),
    }


def matches(
    loc: Located,
    *,
    node_type: str | None = None,
    name: str | None = None,
    qualified_name: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> bool:
    node = loc.node
    if node_type is not None and type(node).__name__.lower() != node_type.lower():
        return False
    if name is not None and loc.name != name:
        return False
    if qualified_name is not None and loc.qualified_name != qualified_name:
        return False
    if lineno is not None and node.lineno != lineno:
        return False
    if end_lineno is not None and getattr(node, "end_lineno", None) != end_lineno:
        return False
    if parent_type is not None and type(loc.parent).__name__.lower() != parent_type.lower():
        return False
    return True


def find(tree: ast.Module, **filters: object) -> list[Located]:
    active = {k: v for k, v in filters.items() if v is not None}
    return [loc for loc in locate_all(tree) if matches(loc, **active)]  # type: ignore[arg-type]


def require_path(path_str: str, *, must_exist: bool = True) -> Path:
    """Validate a mandatory absolute path, raising :class:`AstError` on failure."""
    path = Path(path_str)
    if not path.is_absolute():
        raise AstError("Path must be absolute.")
    if must_exist:
        if not path.exists():
            raise AstError("File not found.")
        if not path.is_file():
            raise AstError("Not a regular file.")
    return path


def load(path_str: str) -> tuple[Path, ast.Module]:
    """Resolve *path_str* and return it together with its cached AST."""
    path = require_path(path_str)
    return path, CACHE.get_tree(path)


def tree_from_input(path: str | None, code: str | None) -> ast.Module:
    """Return an AST from an existing file (*path*) or raw *code* text."""
    if code is not None:
        return parse_source(code)
    if path is not None:
        return load(path)[1]
    raise AstError("Either 'path' or 'code' is required.")


def replace_in_body(loc: Located, new_nodes: Iterable[ast.stmt]) -> None:
    body = loc.parent.body  # type: ignore[attr-defined]
    body[loc.index : loc.index + 1] = list(new_nodes)


def delete_from_body(loc: Located) -> None:
    del loc.parent.body[loc.index]  # type: ignore[attr-defined]
