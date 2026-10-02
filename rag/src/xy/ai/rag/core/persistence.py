"""Gemeinsame, auf Disk persistierte Infrastruktur unter ``.xyrag``.

Enthält den zentralen Shared-Index (Change Detection über Layer-Grenzen
hinweg) und ein abstraktes Interface, über das jeder Layer seine eigenen
Persistenzeinheiten (Cache-Einträge für Chunks, Zeilen, Dateien, Composite-
Objekte, ...) verwaltet, ohne dass die Struktur dieser Einheiten von der
Engine vorgeschrieben wird.
"""
from __future__ import annotations
import json
import sqlite3
import threading
from pathlib import Path
from typing import Any, Iterator
PERSISTENCE_DIRNAME = '.xyrag'

def resolve_root(root: Path | str | None=None) -> Path:
    """Ermittelt das Persistenz-Root: explizites Argument oder CWD/.xyrag."""
    base = Path(root) if root is not None else Path.cwd() / PERSISTENCE_DIRNAME
    base.mkdir(parents=True, exist_ok=True)
    return base

class SharedIndex:
    """Zentraler, geteilter Index für Change Detection über alle Layer.

    Minimaler Grundstock: Pfad, Hash, Größe, mtime und eine monotone
    Sequenznummer je Eintrag; Löschungen als Tombstone. Konkrete Layer
    nutzen dies als gemeinsame Quelle für "hat sich etwas geändert".
    """

    def __init__(self, db_path: Path) -> None:
        self._lock = threading.RLock()
        self._conn = sqlite3.connect(db_path, check_same_thread=False)
        self._conn.execute('PRAGMA journal_mode=WAL')
        self._conn.execute('\n            CREATE TABLE IF NOT EXISTS file_index (\n                path TEXT PRIMARY KEY,\n                hash TEXT,\n                size INTEGER,\n                mtime REAL,\n                seq INTEGER,\n                deleted INTEGER NOT NULL DEFAULT 0\n            )\n            ')
        self._conn.execute('CREATE TABLE IF NOT EXISTS seq_counter (name TEXT PRIMARY KEY, value INTEGER)')
        self._conn.commit()

    def _next_seq(self) -> int:
        cur = self._conn.execute(
            "INSERT INTO seq_counter(name, value) VALUES ('global', 1) ON CONFLICT(name) DO UPDATE SET value = value + 1 RETURNING value")
        return cur.fetchone()[0]

    def upsert(self, path: str, content_hash: str, size: int, mtime: float) -> int:
        """Legt einen Dateieintrag an/aktualisiert ihn, liefert die neue Sequenznummer."""
        with self._lock:
            seq = self._next_seq()
            self._conn.execute(
                'INSERT INTO file_index(path, hash, size, mtime, seq, deleted) VALUES (?, ?, ?, ?, ?, 0) ON CONFLICT(path) DO UPDATE SET hash=?, size=?, mtime=?, seq=?, deleted=0',
                (path,
                 content_hash,
                 size,
                 mtime,
                 seq,
                 content_hash,
                 size,
                 mtime,
                 seq))
            self._conn.commit()
            return seq

    def mark_deleted(self, path: str) -> int:
        with self._lock:
            seq = self._next_seq()
            self._conn.execute(
                'INSERT INTO file_index(path, hash, size, mtime, seq, deleted) VALUES (?, NULL, NULL, NULL, ?, 1) ON CONFLICT(path) DO UPDATE SET deleted=1, seq=?',
                (path,
                 seq,
                 seq))
            self._conn.commit()
            return seq

    def get(self, path: str) -> dict[str, Any] | None:
        row = self._conn.execute(
            'SELECT path, hash, size, mtime, seq, deleted FROM file_index WHERE path = ?', (path,)).fetchone()
        if row is None:
            return None
        return dict(zip(('path', 'hash', 'size', 'mtime', 'seq', 'deleted'), row))

    def changes_since(self, seq: int) -> list[dict[str, Any]]:
        """Liefert alle Änderungen seit ``seq`` - Grundlage für den Per-Layer-Cursor."""
        rows = self._conn.execute(
            'SELECT path, hash, size, mtime, seq, deleted FROM file_index WHERE seq > ? ORDER BY seq ASC',
            (seq,
             )).fetchall()
        return [dict(zip(('path', 'hash', 'size', 'mtime', 'seq', 'deleted'), r)) for r in rows]

    def close(self) -> None:
        self._conn.close()

class LayerStorage:
    """Abstrahiertes Persistenz-Interface für einen einzelnen Layer.

    Unspezifischer Key/Value-Cache (JSON-serialisiert) je Layer, plus ein
    eigener Dateisystem-Namespace für Sidecars (Vektoren, Indexdateien,...).
    Layer entscheiden selbst, welche Struktur ihre IDs haben (Chunk-ID,
    Zeilenbereich, Dateipfad, Composite-Key) und ob/wie sie Ressourcen mit
    anderen Layern teilen (z.B. über denselben Schlüsselraum).
    """

    def __init__(self, layer_id: str, db_path: Path, dir_path: Path) -> None:
        self.layer_id = layer_id
        self.dir = dir_path
        self.dir.mkdir(parents=True, exist_ok=True)
        self._lock = threading.RLock()
        self._conn = sqlite3.connect(db_path, check_same_thread=False)
        self._conn.execute('PRAGMA journal_mode=WAL')
        self._conn.execute('CREATE TABLE IF NOT EXISTS cache (key TEXT PRIMARY KEY, value TEXT)')
        self._conn.execute('CREATE TABLE IF NOT EXISTS layer_meta (name TEXT PRIMARY KEY, value TEXT)')
        self._conn.commit()

    def put(self, key: str, value: Any) -> None:
        with self._lock:
            self._conn.execute(
                'INSERT INTO cache(key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=?',
                (key,
                 json.dumps(value),
                 json.dumps(value)))
            self._conn.commit()

    def get(self, key: str, default: Any=None) -> Any:
        row = self._conn.execute('SELECT value FROM cache WHERE key = ?', (key,)).fetchone()
        return json.loads(row[0]) if row else default

    def delete(self, key: str) -> None:
        with self._lock:
            self._conn.execute('DELETE FROM cache WHERE key = ?', (key,))
            self._conn.commit()

    def keys(self, prefix: str='') -> Iterator[str]:
        rows = self._conn.execute('SELECT key FROM cache WHERE key LIKE ? ORDER BY key', (f'{prefix}%',)).fetchall()
        return (r[0] for r in rows)

    def path_for(self, name: str) -> Path:
        """Pfad für layer-eigene Sidecar-Dateien (z.B. Vektor-Sidecars)."""
        return self.dir / name
    '# Per-Layer-Cursor über den SharedIndex (Change Detection)'

    def get_cursor(self) -> int:
        row = self._conn.execute("SELECT value FROM layer_meta WHERE name = 'cursor'").fetchone()
        return int(row[0]) if row else 0

    def set_cursor(self, seq: int) -> None:
        with self._lock:
            self._conn.execute(
                "INSERT INTO layer_meta(name, value) VALUES ('cursor', ?) ON CONFLICT(name) DO UPDATE SET value=?",
                (str(seq),
                 str(seq)))
            self._conn.commit()

    def close(self) -> None:
        self._conn.close()

class PersistenceManager:
    """Verwaltet das ``.xyrag`` Verzeichnis und reicht Layern ihre Storage aus."""

    def __init__(self, root: Path | str | None=None) -> None:
        self.root = resolve_root(root)
        self.shared_index = SharedIndex(self.root / 'index.db')
        self._layer_storages: dict[str, LayerStorage] = {}
        self._lock = threading.RLock()

    def layer_storage(self, layer_id: str) -> LayerStorage:
        with self._lock:
            storage = self._layer_storages.get(layer_id)
            if storage is None:
                layer_dir = self.root / 'layers' / layer_id
                storage = LayerStorage(layer_id, layer_dir / 'cache.db', layer_dir)
                self._layer_storages[layer_id] = storage
            return storage

    def close(self) -> None:
        self.shared_index.close()
        for storage in self._layer_storages.values():
            storage.close()