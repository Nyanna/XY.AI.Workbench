"""Command-line argument parsing and entry point."""

import argparse
from pathlib import Path

from xy.cgen.config import Config
from xy.cgen.pipeline import run_pipeline


def parse_args(argv=None) -> Config:
    """Parse CLI args into a Config object."""
    parser = argparse.ArgumentParser(
        prog="cgen",
        description="Generates type-safe Java code from an OpenAPI 3.1 YAML schema.",
    )
    parser.add_argument("--schema", required=True, type=Path, help="Path to the OpenAPI 3.1 YAML schema.")
    parser.add_argument("--out", required=True, type=Path, help="Output directory for generated Java sources.")
    parser.add_argument(
        "--base-package",
        default="xy.api.codegen",
        help="Root Java package for generated code (default: xy.api.codegen).",
    )
    args = parser.parse_args(argv)
    return Config(input_schema=args.schema, output_dir=args.out, base_package=args.base_package)


def main(argv=None) -> None:
    """CLI entry point: parses args and runs the pipeline."""
    config = parse_args(argv)
    run_pipeline(config)


if __name__ == "__main__":
    main()
