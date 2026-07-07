"""Command-line entry point: ``python -m xy.ai.mcpc``."""

from __future__ import annotations

import argparse
import dataclasses
import logging
from pathlib import Path

from .config import ServerConfig
from .server import run


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="xy.ai.mcpc",
        description="MCP Controller — stateful Streamable HTTP MCP server.",
    )
    defaults = ServerConfig()
    parser.add_argument("--host", default=defaults.host, help="Bind host (default: %(default)s)")
    parser.add_argument("--port", type=int, default=defaults.port, help="Bind port (default: %(default)s)")
    parser.add_argument("--path", default=defaults.path, help="MCP endpoint path (default: %(default)s)")
    parser.add_argument("--log-dir", type=Path, default=defaults.log_dir,
                        help="Communication log directory (default: %(default)s)")
    parser.add_argument("--session-header", default=defaults.session_header,
                        help="Session id header name (default: %(default)s)")
    parser.add_argument("--log-level", default="INFO",
                        help="Python logging level (default: %(default)s)")
    return parser


def main(argv: list[str] | None = None) -> None:
    args = build_parser().parse_args(argv)
    logging.basicConfig(
        level=getattr(logging, args.log_level.upper(), logging.INFO),
        format="%(asctime)s %(levelname)-7s %(name)s: %(message)s",
    )# 1. basics from env
    config = ServerConfig.from_env()
    
    # 2. add CLI arguments
    valid_fields = {f.name for f in dataclasses.fields(ServerConfig)}
    overrides = {
        k: v for k, v in vars(args).items() 
        if v is not None and k in valid_fields
    }
    
    # 3. Overridesn
    config = config.with_overrides(**overrides)
    run(config)


if __name__ == "__main__":
    main()
