"""Command-line entry point: ``python -m xy.ai.mcpc``."""
import argparse
import dataclasses
import logging
import sys
from pathlib import Path
from xy.ai import mcpc
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.server.server import run

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog='xy.ai.mcpc', description=mcpc.__doc__)
    defaults = ServerConfig()
    parser.add_argument('--host', default=defaults.host, help='Bind host (default: %(default)s)')
    parser.add_argument('--port', type=int, default=defaults.port, help='Bind port (default: %(default)s)')
    parser.add_argument('--path', default=defaults.path, help='MCP endpoint path (default: %(default)s)')
    parser.add_argument('--log-dir', type=Path, default=defaults.log_dir,
                        help='Communication log directory (default: %(default)s)')
    parser.add_argument('--session-header', default=defaults.session_header,
                        help='Session id header name (default: %(default)s)')
    parser.add_argument('--log-level', default='INFO', help='Python logging level (default: %(default)s)')
    parser.add_argument('--ws-host', default=defaults.ws_host, help='WebSocket bind host (default: same as --host)')
    parser.add_argument('--ws-port', type=int, default=defaults.ws_port,
                        help='WebSocket bind port (default: %(default)s)')
    parser.add_argument('--ws-path', default=defaults.ws_path, help='WebSocket endpoint path (default: %(default)s)')
    parser.add_argument(
        '--no-ws',
        dest='ws_enabled',
        action='store_false',
        default=defaults.ws_enabled,
        help='Disable the WebSocket transport')
    return parser

class _MaxLevelFilter(logging.Filter):

    def __init__(self, max_level: int) -> None:
        super().__init__()
        self.max_level = max_level

    def filter(self, record: logging.LogRecord) -> bool:
        return record.levelno < self.max_level

def _configure_logging(level: int) -> None:
    formatter = logging.Formatter('%(asctime)s %(levelname)-7s %(name)s: %(message)s')
    stdout_handler = logging.StreamHandler(sys.stdout)
    stdout_handler.setLevel(logging.DEBUG)
    stdout_handler.addFilter(_MaxLevelFilter(logging.WARNING))
    stdout_handler.setFormatter(formatter)
    stderr_handler = logging.StreamHandler(sys.stderr)
    stderr_handler.setLevel(logging.WARNING)
    stderr_handler.setFormatter(formatter)
    root = logging.getLogger()
    root.setLevel(level)
    root.handlers.clear()
    root.addHandler(stdout_handler)
    root.addHandler(stderr_handler)

def main(argv: list[str] | None=None) -> None:
    args = build_parser().parse_args(argv)
    _configure_logging(getattr(logging, args.log_level.upper(), logging.INFO))
    config = ServerConfig.from_env()
    valid_fields = {f.name for f in dataclasses.fields(ServerConfig)}
    overrides = {k: v for k, v in vars(args).items() if v is not None and k in valid_fields}
    config = config.with_overrides(**overrides)
    run(config)
if __name__ == '__main__':
    main()