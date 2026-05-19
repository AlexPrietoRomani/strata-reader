"""Tests for the SQLite-backed IA result cache."""

from __future__ import annotations

import time
from pathlib import Path

import pytest

from strata_ia.cache import CacheKey, open_cache, sha256_hex


@pytest.mark.asyncio
async def test_put_get_round_trip(tmp_path: Path) -> None:
    async with open_cache(tmp_path / "c.db") as cache:
        key = CacheKey(crop_sha256="abc", model_id="m", version="v")
        assert await cache.get(key) is None
        await cache.put(key, '{"text":"hello"}')
        assert await cache.get(key) == '{"text":"hello"}'
        assert await cache.count() == 1


@pytest.mark.asyncio
async def test_different_models_do_not_collide(tmp_path: Path) -> None:
    async with open_cache(tmp_path / "c.db") as cache:
        crop = "h"
        await cache.put(CacheKey(crop, "model_a", "v1"), "A")
        await cache.put(CacheKey(crop, "model_b", "v1"), "B")
        await cache.put(CacheKey(crop, "model_a", "v2"), "A2")
        assert await cache.count() == 3
        assert await cache.get(CacheKey(crop, "model_a", "v1")) == "A"
        assert await cache.get(CacheKey(crop, "model_a", "v2")) == "A2"
        assert await cache.get(CacheKey(crop, "model_b", "v1")) == "B"


@pytest.mark.asyncio
async def test_prune_older_than(tmp_path: Path) -> None:
    async with open_cache(tmp_path / "c.db") as cache:
        await cache.put(CacheKey("a", "m", "v"), "{}")
        # Manually backdate the row to two days ago.
        await cache._conn.execute(  # noqa: SLF001 — test reaches into impl
            "UPDATE cache SET created_at = ? WHERE crop_sha256 = ?",
            (time.time() - 2 * 86_400, "a"),
        )
        await cache._conn.commit()  # noqa: SLF001
        removed = await cache.prune_older_than(days=1)
        assert removed == 1
        assert await cache.count() == 0


@pytest.mark.asyncio
async def test_hit_is_dramatically_faster_than_recompute(tmp_path: Path) -> None:
    """Sanity check on the cache speedup AC (≥ 50×)."""
    async with open_cache(tmp_path / "c.db") as cache:
        payload = b"x" * 4096
        sha = sha256_hex(payload)
        key = CacheKey(sha, "m", "v")

        async def expensive_compute() -> str:
            # Simulate a 200 ms "VLM" call.
            await _async_sleep(0.2)
            return '{"text":"answer"}'

        async def lookup_or_compute() -> str:
            cached = await cache.get(key)
            if cached is not None:
                return cached
            value = await expensive_compute()
            await cache.put(key, value)
            return value

        t0 = time.perf_counter()
        first = await lookup_or_compute()
        first_ms = (time.perf_counter() - t0) * 1000

        t1 = time.perf_counter()
        second = await lookup_or_compute()
        second_ms = (time.perf_counter() - t1) * 1000

        assert first == second
        # AC: cache hit ≥ 50× faster. We use 50× as a hard floor.
        assert first_ms / max(second_ms, 1e-3) >= 50, (
            f"speedup only {first_ms / second_ms:.1f}× (first={first_ms:.1f}ms, second={second_ms:.3f}ms)"
        )


def test_sha256_hex_is_deterministic() -> None:
    assert sha256_hex(b"hello") == sha256_hex(b"hello")
    assert sha256_hex(b"hello") != sha256_hex(b"hellp")


async def _async_sleep(seconds: float) -> None:
    import asyncio

    await asyncio.sleep(seconds)
