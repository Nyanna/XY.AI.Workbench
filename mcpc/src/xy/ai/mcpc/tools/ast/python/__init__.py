"""Python back-end for the ``ast_*`` tools, built on the standard-library ``ast``.

Split into :mod:`xy.ai.mcpc.tools.ast.python._comments` (comment-preserving
pre-processing), :mod:`xy.ai.mcpc.tools.ast.python._nodes` (node
classification/formatting and statement grouping) and
:mod:`xy.ai.mcpc.tools.ast.python._engine` (the :class:`PythonEngine` itself);
this module re-exports the package's public surface.
"""
from __future__ import annotations
from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
from xy.ai.mcpc.tools.ast.python._engine import ENGINE, PythonEngine
from xy.ai.mcpc.tools.ast.python._nodes import import_names
__all__ = ['ENGINE', 'PythonEngine', 'comments_to_annotations', 'import_names']