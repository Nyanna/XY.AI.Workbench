"""Config object: bundles CLI inputs for the pipeline."""

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Config:
    """Runtime configuration for a single generator run."""

    input_schema: Path
    output_dir: Path
    base_package: str = "xy.api.codegen"
