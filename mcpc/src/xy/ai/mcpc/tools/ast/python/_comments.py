"""Comment-preserving pre-processing for the Python ``ast_*`` engine.

Existing ``#`` comments are converted into standalone string-literal
annotations (:func:`comments_to_annotations`) before parsing so they survive
the round-trip through :func:`ast.parse` / :func:`ast.unparse`.
"""
from __future__ import annotations
import io
import re
import tokenize
_CONTINUATION_HEADER_RE = re.compile('^\\s*(elif|else|except|finally)\\b')
_CASE_HEADER_RE = re.compile('^\\s*case\\b.*:\\s*(#.*)?$')

def _annotation_literal(comment: str) -> str:
    return repr(comment.rstrip())

def _is_continuation_header(line: str) -> bool:
    return bool(_CONTINUATION_HEADER_RE.match(line) or _CASE_HEADER_RE.match(line))

def _next_code_line_index(lines: list[str], start: int) -> int | None:
    for i in range(start, len(lines)):
        stripped = lines[i].strip()
        if stripped == '' or stripped.startswith('#'):
            continue
        return i
    return None

def _suite_indent(lines: list[str], header_lineno: int) -> str:
    header_line = lines[header_lineno - 1]
    header_indent = header_line[:len(header_line) - len(header_line.lstrip())]
    for line in lines[header_lineno:]:
        if line.strip() == '':
            continue
        return line[:len(line) - len(line.lstrip())]
    return header_indent + '    '

def comments_to_annotations(source: str) -> str:
    """Rewrite ``#`` comments into standalone string-literal statements.

    A comment on its own line becomes an equally-indented string literal; a
    trailing comment is lifted onto its own literal line in front of the
    statement it belonged to. Comments inside brackets/continuations cannot be
    represented as standalone literals without breaking syntax and are dropped.
    Style and exact placement are explicitly *not* preserved – only semantics
    plus the recovered annotation text. Comments preceding or trailing a
    continuation header (``elif``/``else``/``except``/``finally``/``case``) are
    moved into the suite that header opens.
    """
    if '#' not in source:
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
            if ttype in (tokenize.NL, tokenize.INDENT, tokenize.DEDENT, tokenize.ENCODING, tokenize.ENDMARKER):
                continue
            if ttype == tokenize.COMMENT:
                lineno, col = tok.start
                prefix = lines[lineno - 1][:col]
                standalone = prefix.strip() == ''
                literal = _annotation_literal(tok.string)
                if depth == 0 and standalone and (logical_start is None):
                    next_idx = _next_code_line_index(lines, lineno)
                    if next_idx is not None and _is_continuation_header(lines[next_idx]):
                        header_lineno = next_idx + 1
                        target_indent = _suite_indent(lines, header_lineno)
                        inserts.setdefault(header_lineno + 1, []).append(f'{target_indent}{literal}\n')
                        replaces[lineno] = '\n'
                    else:
                        replaces[lineno] = f'{prefix}{literal}\n'
                elif depth == 0 and (not standalone) and (logical_start is not None):
                    stmt_line = lines[logical_start - 1]
                    if _is_continuation_header(stmt_line):
                        target_indent = _suite_indent(lines, lineno)
                        inserts.setdefault(lineno + 1, []).append(f'{target_indent}{literal}\n')
                    else:
                        indent = stmt_line[:len(stmt_line) - len(stmt_line.lstrip())]
                        inserts.setdefault(logical_start, []).append(f'{indent}{literal}\n')
                    strips[lineno] = col
                elif standalone:
                    replaces[lineno] = '\n'
                else:
                    strips[lineno] = col
                continue
            if logical_start is None:
                logical_start = tok.start[0]
            if ttype == tokenize.OP:
                if tok.string in '([{':
                    depth += 1
                elif tok.string in ')]}':
                    depth = max(0, depth - 1)
    except (tokenize.TokenError, IndentationError):
        return source
    out: list[str] = []
    for i, line in enumerate(lines, start=1):
        if i in inserts:
            out.extend(inserts[i])
        if i in replaces:
            out.append(replaces[i])
        elif i in strips:
            out.append(line[:strips[i]].rstrip() + '\n')
        else:
            out.append(line)
    return ''.join(out)