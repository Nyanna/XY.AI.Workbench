---
name: markdown-format
description: Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.
when_to_use: Use this to acquire Markdown formatting rules.
disable-model-invocation: false
user-invocable: true
---

* Use a line containing only `***` to insert a page break in PDF output.
* Insert page breaks before major numbered chapters at the beginning of the file.
* Use `\n---\n` as a section separator before second-order chapters at the beginning of the file.
* All files must end with an additional newline to prevent Markdown formatting errors on merge.
* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.
* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.
* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas.