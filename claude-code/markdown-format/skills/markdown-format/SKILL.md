---
name: markdown-format
description: Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.
when_to_use: Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.
disable-model-invocation: false
user-invocable: true
---

* Use a line containing only `***` to insert a page break in PDF output.
* Insert page breaks before top-level chapters (H1) at the start of each chapter.
* Use `\n---\n` as a section separator before second-order chapters (H2) at the start of each chapter.
* All files must end with an additional newline to prevent Markdown formatting errors on merge.
* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.
* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.
* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas.
* Never hard-wrap prose at a fixed line width.
* Avoid internal references and cross-references