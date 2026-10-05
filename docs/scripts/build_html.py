"""Compile the markdown topics (basic, howto, rag) into static HTML pages.

Every ``*.md`` file directly inside one of the topic source directories is
rendered as a standalone HTML page using a plain, scientific-paper style
(scholar-theme.css, no animations or JavaScript). The topics are kept in
separate listings on the index page.

The markdown content stays in its original language; this script and all
generated markup/UI strings are English.

Usage:
    python3 build_html.py
"""
from __future__ import annotations

import html
import re
from dataclasses import dataclass, field
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
DOCS_DIR = SCRIPT_DIR.parent

# Topic name -> source directory (all relative to the docs/ directory).
TOPIC_DIRS: dict[str, Path] = {
    'basic': DOCS_DIR / 'basic',
    'howto': DOCS_DIR / 'howto',
    'rag': DOCS_DIR / 'rag',
}
TOPIC_LABELS: dict[str, str] = {
    'basic': 'Basics',
    'howto': 'How-To Guides',
    'rag': 'RAG',
}

OUTPUT_DIR = DOCS_DIR / 'pages'
CSS_FILENAME = 'scholar-theme.css'
SITE_TITLE = 'Documentation'
SITE_TAGLINE = 'Reference material, grouped by topic.'

# --------------------------------------------------------------------------
# Minimal, dependency-free markdown -> HTML conversion
# --------------------------------------------------------------------------
INLINE_CODE_RE = re.compile('`([^`]+)`')
LINK_RE = re.compile('\\[([^\\]]+)\\]\\(([^)]+)\\)')
BOLD_RE = re.compile('(\\*\\*|__)(.+?)\\1')
ITALIC_RE = re.compile('(\\*|_)(.+?)\\1')
HEADING_RE = re.compile('^(#{1,6})\\s+(.*)$')
HR_RE = re.compile('^(-{3,}|\\*{3,}|_{3,})$')
UNORDERED_ITEM_RE = re.compile('^[-*]\\s+(.*)$')
ORDERED_ITEM_RE = re.compile('^\\d+\\.\\s+(.*)$')
LIST_ITEM_RE = re.compile('^(?:\\d+\\.|[-*])\\s+(.*)$')


def convert_inline(text: str) -> str:
    """Convert inline markdown spans (code, links, bold, italic) to HTML."""
    text = html.escape(text, quote=False)
    placeholders: list[str] = []

    def stash(value: str) -> str:
        placeholders.append(value)
        return f'\x00{len(placeholders) - 1}\x00'

    text = INLINE_CODE_RE.sub(lambda m: stash(f'<code>{m.group(1)}</code>'), text)
    text = LINK_RE.sub(lambda m: stash(f'<a href="{m.group(2)}">{m.group(1)}</a>'), text)
    text = BOLD_RE.sub(lambda m: stash(f'<strong>{m.group(2)}</strong>'), text)
    text = ITALIC_RE.sub(lambda m: stash(f'<em>{m.group(2)}</em>'), text)
    return re.sub('\\x00(\\d+)\\x00', lambda m: placeholders[int(m.group(1))], text)


def strip_markdown(text: str) -> str:
    """Reduce inline markdown to plain text (for <title>, excerpts, nav)."""
    text = LINK_RE.sub(lambda m: m.group(1), text)
    text = BOLD_RE.sub(lambda m: m.group(2), text)
    text = ITALIC_RE.sub(lambda m: m.group(2), text)
    text = INLINE_CODE_RE.sub(lambda m: m.group(1), text)
    return text.strip()


@dataclass
class Block:
    """heading | paragraph | blockquote | list | hr"""
    kind: str
    level: int = 0
    text: str = ''
    items: list[str] = field(default_factory=list)
    ordered: bool = False


def parse_blocks(md_text: str) -> list[Block]:
    """Parse a markdown document into a flat list of block elements."""
    lines = md_text.splitlines()
    blocks: list[Block] = []
    paragraph_lines: list[str] = []
    i = 0

    def flush_paragraph() -> None:
        if paragraph_lines:
            blocks.append(Block(kind='paragraph', text=' '.join(paragraph_lines).strip()))
            paragraph_lines.clear()

    while i < len(lines):
        stripped = lines[i].strip()
        if not stripped:
            flush_paragraph()
            i += 1
            continue
        heading = HEADING_RE.match(stripped)
        if heading:
            flush_paragraph()
            blocks.append(Block(kind='heading', level=len(heading.group(1)), text=heading.group(2).strip()))
            i += 1
            continue
        if HR_RE.match(stripped):
            flush_paragraph()
            blocks.append(Block(kind='hr'))
            i += 1
            continue
        if stripped.startswith('>'):
            flush_paragraph()
            quote_lines: list[str] = []
            while i < len(lines) and lines[i].strip().startswith('>'):
                quote_lines.append(re.sub('^>\\s?', '', lines[i].strip()))
                i += 1
            blocks.append(Block(kind='blockquote', text=' '.join(quote_lines)))
            continue
        if UNORDERED_ITEM_RE.match(stripped) or ORDERED_ITEM_RE.match(stripped):
            flush_paragraph()
            ordered = bool(ORDERED_ITEM_RE.match(stripped))
            items: list[str] = []
            while i < len(lines):
                item = LIST_ITEM_RE.match(lines[i].strip())
                if not item:
                    break
                items.append(item.group(1))
                i += 1
            blocks.append(Block(kind='list', items=items, ordered=ordered))
            continue
        paragraph_lines.append(stripped)
        i += 1
    flush_paragraph()
    return blocks


def render_block(block: Block) -> str:
    if block.kind == 'heading':
        # Keep body headings within h2..h5 (h1 is reserved for the page title).
        level = min(block.level + 1, 5)
        return f'<h{level}>{convert_inline(block.text)}</h{level}>'
    if block.kind == 'paragraph':
        return f'<p>{convert_inline(block.text)}</p>'
    if block.kind == 'blockquote':
        return f'<blockquote>{convert_inline(block.text)}</blockquote>'
    if block.kind == 'hr':
        return '<hr />'
    if block.kind == 'list':
        tag = 'ol' if block.ordered else 'ul'
        items = ''.join((f'<li>{convert_inline(item)}</li>' for item in block.items))
        return f'<{tag}>{items}</{tag}>'
    raise ValueError(f'Unknown block kind: {block.kind}')


def extract_title(blocks: list[Block], fallback: str) -> tuple[str, list[Block]]:
    """Pull the first heading out as the page title; return the rest."""
    for index, block in enumerate(blocks):
        if block.kind == 'heading':
            remaining = blocks[:index] + blocks[index + 1:]
            return (block.text, remaining)
    return (fallback, blocks)


def first_excerpt(blocks: list[Block], limit: int = 220) -> str:
    for block in blocks:
        if block.kind == 'paragraph':
            plain = strip_markdown(block.text)
            if len(plain) <= limit:
                return plain
            return plain[:limit].rsplit(' ', 1)[0] + '…'
    return ''


GERMAN_MARKERS = (' der ', ' die ', ' und ', ' ist ', ' nicht ', ' eine ', ' ein ', ' sich ', ' mit ', ' für ')
ENGLISH_MARKERS = (' the ', ' and ', ' is ', ' of ', ' to ', ' that ', ' with ', ' for ')


def detect_language(text: str) -> str:
    """Naive language guess (de/en) used only for the html[lang] attribute."""
    sample = f' {text.lower()} '
    de_hits = sum((sample.count(marker) for marker in GERMAN_MARKERS))
    en_hits = sum((sample.count(marker) for marker in ENGLISH_MARKERS))
    return 'en' if en_hits > de_hits else 'de'


# --------------------------------------------------------------------------
# HTML page templates
# --------------------------------------------------------------------------

def render_article_page(title_html: str, title_plain: str, lang: str, body_html: str) -> str:
    return (
        f'<!doctype html>\n<html lang="{lang}">\n<head>\n'
        f'<meta charset="utf-8">\n'
        f'<meta name="viewport" content="width=device-width, initial-scale=1">\n'
        f'<title>{html.escape(title_plain)} — {SITE_TITLE}</title>\n'
        f'<link rel="stylesheet" href="{CSS_FILENAME}">\n'
        f'</head>\n<body>\n'
        f'<header class="site-header">\n<a class="back-link" href="index.html">&larr; Overview</a>\n</header>\n'
        f'<main class="article">\n<article>\n<h1>{title_html}</h1>\n{body_html}\n</article>\n</main>\n'
        f'<footer class="site-footer">\n<small>{SITE_TITLE}</small>\n</footer>\n'
        f'</body>\n</html>\n'
    )


def render_index_page(sections_html: str) -> str:
    return (
        f'<!doctype html>\n<html lang="en">\n<head>\n'
        f'<meta charset="utf-8">\n'
        f'<meta name="viewport" content="width=device-width, initial-scale=1">\n'
        f'<title>{SITE_TITLE}</title>\n'
        f'<link rel="stylesheet" href="{CSS_FILENAME}">\n'
        f'</head>\n<body>\n'
        f'<header class="site-header">\n<h1>{SITE_TITLE}</h1>\n<p class="tagline">{html.escape(SITE_TAGLINE)}</p>\n</header>\n'
        f'<main>\n{sections_html}\n</main>\n'
        f'<footer class="site-footer">\n<small>{SITE_TITLE}</small>\n</footer>\n'
        f'</body>\n</html>\n'
    )


def render_index_section(label: str, entries_html: str) -> str:
    return f'<section class="topic">\n<h2>{html.escape(label)}</h2>\n<ul class="listing">\n{entries_html}\n</ul>\n</section>'


def render_index_entry(slug: str, title_plain: str, excerpt: str) -> str:
    return (
        f'<li class="listing-entry"><a href="{slug}.html">'
        f'<h3>{html.escape(title_plain)}</h3>'
        f'<p>{html.escape(excerpt)}</p>'
        f'</a></li>'
    )


# --------------------------------------------------------------------------
# Stylesheet (plain, scientific, no animations/effects)
# --------------------------------------------------------------------------
CSS_CONTENT = """\
:root {
  --text: #1a1a1a;
  --muted: #555;
  --bg: #ffffff;
  --border: #ccc;
  --max-width: 52rem;
  --font: Georgia, 'Times New Roman', serif;
}

* { box-sizing: border-box; }

body {
  margin: 0;
  padding: 0 1rem 3rem;
  background: var(--bg);
  color: var(--text);
  font-family: var(--font);
  line-height: 1.6;
}

.site-header, .site-footer, main.article, main > section.topic {
  max-width: var(--max-width);
  margin: 0 auto;
}

.site-header {
  padding: 2rem 0 1rem;
  border-bottom: 1px solid var(--border);
}

.site-header h1 { margin: 0 0 0.25rem; font-size: 1.8rem; }
.tagline { color: var(--muted); margin: 0; }

.back-link {
  color: var(--muted);
  text-decoration: none;
  font-size: 0.9rem;
}
.back-link:hover { text-decoration: underline; }

main { padding-top: 1rem; }

article h1 { font-size: 1.6rem; margin-top: 0; }
article h2 { font-size: 1.3rem; }
article h3 { font-size: 1.1rem; }
article p, article li { font-size: 1rem; }
article blockquote {
  margin: 1rem 0;
  padding-left: 1rem;
  border-left: 3px solid var(--border);
  color: var(--muted);
}
article code {
  background: #f2f2f2;
  padding: 0.1rem 0.3rem;
  font-family: Consolas, Menlo, monospace;
  font-size: 0.9em;
}
article hr { border: none; border-top: 1px solid var(--border); margin: 2rem 0; }

section.topic { margin: 2rem auto; }
section.topic h2 {
  font-size: 1.3rem;
  border-bottom: 1px solid var(--border);
  padding-bottom: 0.3rem;
}

ul.listing {
  list-style: none;
  margin: 1rem 0 0;
  padding: 0;
}

li.listing-entry {
  border-bottom: 1px solid var(--border);
}
li.listing-entry a {
  display: block;
  padding: 0.75rem 0;
  color: inherit;
  text-decoration: none;
}
li.listing-entry h3 {
  margin: 0 0 0.25rem;
  font-size: 1.05rem;
}
li.listing-entry p {
  margin: 0;
  color: var(--muted);
  font-size: 0.95rem;
  /* Wide, single-line listing: entries favour width over height. */
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.site-footer {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border);
  color: var(--muted);
  font-size: 0.85rem;
}
"""


# --------------------------------------------------------------------------
# Build
# --------------------------------------------------------------------------

def build() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUTPUT_DIR / CSS_FILENAME).write_text(CSS_CONTENT, encoding='utf-8')

    sections: list[str] = []
    total = 0

    for topic, src_dir in TOPIC_DIRS.items():
        if not src_dir.is_dir():
            print(f'Warning: source directory not found: {src_dir}')
            continue
        md_files = sorted(src_dir.glob('*.md'), key=lambda p: p.name)
        if not md_files:
            print(f'No markdown files found in {src_dir}')
            continue

        entries: list[str] = []
        for md_path in md_files:
            raw = md_path.read_text(encoding='utf-8')
            blocks = parse_blocks(raw)
            raw_title, body_blocks = extract_title(blocks, fallback=md_path.stem)
            title_plain = strip_markdown(raw_title)
            title_html = convert_inline(raw_title)
            lang = detect_language(raw)
            body_html = '\n'.join((render_block(b) for b in body_blocks))
            excerpt = first_excerpt(body_blocks)
            slug = f'{topic}-{md_path.stem}'

            page_html = render_article_page(title_html, title_plain, lang, body_html)
            (OUTPUT_DIR / f'{slug}.html').write_text(page_html, encoding='utf-8')
            entries.append(render_index_entry(slug, title_plain, excerpt))
            print(f'Compiled {md_path.relative_to(DOCS_DIR)} -> {slug}.html')

        sections.append(render_index_section(TOPIC_LABELS.get(topic, topic.title()), '\n'.join(entries)))
        total += len(md_files)

    if total == 0:
        print('No markdown files found in any topic directory.')
        return

    index_html = render_index_page('\n'.join(sections))
    (OUTPUT_DIR / 'index.html').write_text(index_html, encoding='utf-8')
    print(f'Compiled index.html ({total} entries across {len(sections)} topics)')


if __name__ == '__main__':
    build()
