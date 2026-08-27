"""Minimal smoke test: every module under ``xy.ai.mcpc`` must import cleanly.

This is intentionally shallow — it does not exercise behaviour — but it is
exactly the kind of test that catches refactoring fallout: broken imports,
renamed symbols, circular imports, syntax errors, stray top-level code that
raises, etc. Each module is collected as its own parametrized test case so a
single broken module doesn't hide failures in the others.
"""
from __future__ import annotations

import importlib
import pkgutil

import pytest

import xy.ai.mcpc as root_package


def _discover_module_names() -> list[str]:
    names = [root_package.__name__]
    for module_info in pkgutil.walk_packages(
        root_package.__path__, prefix=root_package.__name__ + "."
    ):
        names.append(module_info.name)
    return sorted(names)


MODULE_NAMES = _discover_module_names()


def test_discovery_found_modules():
    # Guards against a broken discovery silently collecting zero tests.
    assert len(MODULE_NAMES) > 30


@pytest.mark.parametrize("module_name", MODULE_NAMES)
def test_module_imports(module_name):
    importlib.import_module(module_name)
