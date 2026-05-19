"""SQLite-backed cache for IA results keyed by crop SHA-256.

Plan Maestro §10.T5.6 — reprocessing the same crop with the same model
version must be ≥ 50× faster than the original call. Implemented with
``aiosqlite`` so it integrates cleanly with the FastAPI event loop.

Key design points:
- Composite primary key ``(crop_sha256, model_id, version)``. The
  ``version`` field is the *backend* version (e.g. ``"ollama-0.4.2"``
  or ``"surya-0.6"``) so model upgrades invalidate stale entries without
  needing a global ``DROP TABLE``.
- ``result_json`` stores the response body as a string. Re-decoding to
  Pydantic in the caller keeps this module storage-agnostic.
- Single writer / many reader thanks to SQLite's WAL mode (enabled on
  every connection).
"""

from __future__ import annotations

import hashlib
import time
from contextlib import asynccontextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import AsyncIterator

import aiosqlite

CACHE_SCHEMA = """
CREATE TABLE IF NOT EXISTS cache (
    crop_sha256 TEXT NOT NULL,
    model_id    TEXT NOT NULL,
    version     TEXT NOT NULL,
    result_json TEXT NOT NULL,
    created_at  REAL NOT NULL,
    PRIMARY KEY (crop_sha256, model_id, version)
);
CREATE INDEX IF NOT EXISTS idx_cache_created ON cache(created_at);
"""


@dataclass(frozen=True, slots=True)
class CacheKey:
    crop_sha256: str
    model_id: str
    version: str


def sha256_hex(payload: bytes) -> str:
    """Stable SHA-256 helper exposed so callers don't need to import hashlib."""
    return hashlib.sha256(payload).hexdigest()


class ResultCache:
    """Async SQLite cache. Use as a context manager via :func:`open_cache`."""

    def __init__(self, conn: aiosqlite.Connection) -> None:
        self._conn = conn

    async def get(self, key: CacheKey) -> str | None:
        async with self._conn.execute(
            "SELECT result_json FROM cache WHERE crop_sha256 = ? AND model_id = ? AND version = ?",
            (key.crop_sha256, key.model_id, key.version),
        ) as cursor:
            row = await cursor.fetchone()
            return row[0] if row else None

    async def put(self, key: CacheKey, result_json: str) -> None:
        await self._conn.execute(
            """
            INSERT OR REPLACE INTO cache(crop_sha256, model_id, version, result_json, created_at)
            VALUES (?, ?, ?, ?, ?)
            """,
            (key.crop_sha256, key.model_id, key.version, result_json, time.time()),
        )
        await self._conn.commit()

    async def count(self) -> int:
        async with self._conn.execute("SELECT COUNT(*) FROM cache") as cursor:
            row = await cursor.fetchone()
            return int(row[0]) if row else 0

    async def prune_older_than(self, days: int) -> int:
        """Delete entries older than ``days`` days. Returns rows removed."""
        threshold = time.time() - (days * 86_400)
        cursor = await self._conn.execute(
            "DELETE FROM cache WHERE created_at < ?", (threshold,)
        )
        await self._conn.commit()
        return cursor.rowcount


@asynccontextmanager
async def open_cache(path: Path) -> AsyncIterator[ResultCache]:
    """Open a cache at ``path``. Creates parent directories if needed."""
    path.parent.mkdir(parents=True, exist_ok=True)
    async with aiosqlite.connect(path) as conn:
        await conn.execute("PRAGMA journal_mode=WAL")
        await conn.executescript(CACHE_SCHEMA)
        await conn.commit()
        yield ResultCache(conn)
