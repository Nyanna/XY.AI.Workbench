"""CLI Entry Point der xy.ai.rag Engine (on-demand, kein Daemon)."""
from __future__ import annotations
import argparse
import asyncio
import json
import sys
from xy.ai.rag.core.engine import Engine, ExecutionMode
from xy.ai.rag.core.query import Query
from xy.ai.rag.core.registry import LayerRegistry

def _parse_query(args: list[str], json_query: str | None) -> Query:
    fields: dict[str, object] = {}
    if json_query:
        fields.update(json.loads(json_query))
    for item in args:
        if '=' not in item:
            raise SystemExit(f'Ungültiges Query-Feld (erwartet key=value): {item}')
        key, _, value = item.partition('=')
        fields[key] = value
    return Query(fields)

def build_arg_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog='xyrag', description='xy.ai.rag - Layered Anytime Retrieval Engine')
    parser.add_argument('query', nargs='*', help='Query-Felder als key=value')
    parser.add_argument('--json', dest='json_query', default=None, help='Query als JSON-Objekt')
    parser.add_argument('--root', default=None, help='Persistenz-Root (Default: ./.xyrag)')
    parser.add_argument(
        '--mode',
        choices=[
            m.value for m in ExecutionMode],
        default=ExecutionMode.PARALLEL.value,
        help='Ausführungsmodell der Layer-Topologie')
    return parser

def build_default_registry() -> LayerRegistry:
    """Erzeugt die Registry mit allen bekannten Layer-Implementierungen.

    Konkrete Layer werden an dieser Stelle später ergänzt.
    """
    return LayerRegistry()

async def _run(args: argparse.Namespace) -> int:
    query = _parse_query(args.query, args.json_query)
    registry = build_default_registry()
    engine = Engine(registry, root=args.root, mode=ExecutionMode(args.mode))
    engine.start_background()
    try:
        result_set, statuses = await engine.run_query(query)
        output = {'results': [entry.to_dict() for entry in result_set], 'layers': [
            status.__dict__ for status in statuses]}
        print(json.dumps(output, default=str, indent=2))
    finally:
        await engine.shutdown()
        engine.persistence.close()
    return 0

def main(argv: list[str] | None=None) -> int:
    parser = build_arg_parser()
    args = parser.parse_args(argv)
    return asyncio.run(_run(args))
if __name__ == '__main__':
    sys.exit(main())