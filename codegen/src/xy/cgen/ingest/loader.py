"""Loads an OpenAPI YAML document into a plain Python structure."""

from pathlib import Path

import yaml


def load_yaml(path: Path) -> dict:
    """Parse a YAML file into nested dicts/lists. No schema-specific processing."""
    with path.open("r", encoding="utf-8") as stream:
        return yaml.safe_load(stream)
