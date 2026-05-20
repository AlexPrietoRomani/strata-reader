"""Tests for the VRAM admission control."""

from __future__ import annotations

from dataclasses import dataclass

import pytest
from strata_ia import resource_guard
from strata_ia.resource_guard import ResourceExhausted, VramSnapshot, guarded


@dataclass
class FakeMem:
    free: int
    total: int


@pytest.mark.asyncio
async def test_admits_when_enough_vram(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(
        resource_guard,
        "read_vram",
        lambda *_: VramSnapshot(device_index=0, free_mb=8192, total_mb=24576),
    )
    calls = {"n": 0}

    @guarded(estimated_vram_mb=4000, safety_margin_mb_getter=lambda: 512)
    async def handler() -> str:
        calls["n"] += 1
        return "ok"

    assert await handler() == "ok"
    assert calls["n"] == 1


@pytest.mark.asyncio
async def test_rejects_when_vram_insufficient(monkeypatch: pytest.MonkeyPatch) -> None:
    # 500 MB free, ask for 4000 MB + 512 MB margin → must reject.
    monkeypatch.setattr(
        resource_guard,
        "read_vram",
        lambda *_: VramSnapshot(device_index=0, free_mb=500, total_mb=24576),
    )

    @guarded(estimated_vram_mb=4000, safety_margin_mb_getter=lambda: 512)
    async def handler() -> str:
        return "should not run"

    with pytest.raises(ResourceExhausted) as exc:
        await handler()
    assert exc.value.free_mb == 500
    assert exc.value.needed_mb == 4512


@pytest.mark.asyncio
async def test_admits_when_no_gpu_detected(monkeypatch: pytest.MonkeyPatch) -> None:
    """CPU-only hosts get a free pass — the OS handles back-pressure."""
    monkeypatch.setattr(resource_guard, "read_vram", lambda *_: None)

    @guarded(estimated_vram_mb=999_999, safety_margin_mb_getter=lambda: 0)
    async def handler() -> str:
        return "cpu"

    assert await handler() == "cpu"


def test_vram_snapshot_units_are_megabytes() -> None:
    snap = VramSnapshot(device_index=0, free_mb=10, total_mb=100)
    assert snap.free_mb == 10
    assert snap.total_mb == 100


def test_read_vram_returns_none_when_nvml_absent(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(resource_guard, "_HAS_NVML", False)
    assert resource_guard.read_vram() is None
