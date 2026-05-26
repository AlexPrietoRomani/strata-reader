"""VRAM-based admission control for the IA microservice.

Plan Maestro §10.T5.5 — the Triage Engine in Rust will gracefully back
off (AIMD on the bridge) when this module reports ``RESOURCE_EXHAUSTED``.
The decorator [`guarded`] wraps async FastAPI handlers and rejects
incoming work when free VRAM is below the model's estimated footprint
plus a configurable safety margin.

NVIDIA is the primary supported backend (pynvml). ROCm and Apple Metal
hosts return ``None`` for VRAM and the guard degrades to "always admit"
— resource control on those hosts happens at the OS / driver level.
"""

from __future__ import annotations

import functools
import threading
from collections.abc import Awaitable, Callable
from dataclasses import dataclass

import structlog

try:
    import pynvml

    _HAS_NVML = True
except ImportError:  # pragma: no cover - depends on host
    _HAS_NVML = False


logger = structlog.get_logger(__name__)


@dataclass(frozen=True, slots=True)
class VramSnapshot:
    """Free / total VRAM in megabytes for a single device."""

    device_index: int
    free_mb: int
    total_mb: int


class ResourceExhausted(Exception):
    """Raised by [`guarded`] when free VRAM is below the requested budget."""

    def __init__(self, free_mb: int, needed_mb: int):
        super().__init__(f"resource exhausted: {free_mb} MiB free, {needed_mb} MiB needed")
        self.free_mb = free_mb
        self.needed_mb = needed_mb


_nvml_lock = threading.Lock()
_nvml_initialized = False


def _ensure_nvml() -> bool:
    global _nvml_initialized
    if not _HAS_NVML:
        return False
    with _nvml_lock:
        if _nvml_initialized:
            return True
        try:
            pynvml.nvmlInit()
            _nvml_initialized = True
            return True
        except Exception as exc:
            logger.warning("nvml_init_failed", error=str(exc))
            return False


def read_vram(device_index: int = 0) -> VramSnapshot | None:
    """Return free/total VRAM for ``device_index`` or ``None`` when unsupported."""
    if not _ensure_nvml():
        return None
    try:
        handle = pynvml.nvmlDeviceGetHandleByIndex(device_index)
        info = pynvml.nvmlDeviceGetMemoryInfo(handle)
        return VramSnapshot(
            device_index=device_index,
            free_mb=int(info.free) // (1024 * 1024),
            total_mb=int(info.total) // (1024 * 1024),
        )
    except Exception as exc:
        logger.warning("nvml_query_failed", device=device_index, error=str(exc))
        return None


def has_enough_vram(estimated_vram_mb: int, safety_margin_mb: int) -> bool:
    """Decide if the current GPU has room for a job that needs ``estimated_vram_mb``."""
    snapshot = read_vram(0)
    if snapshot is None:
        # No GPU info available — admit. Better to overload Ollama than to
        # reject every request on a CPU-only host.
        return True
    return snapshot.free_mb >= estimated_vram_mb + safety_margin_mb


def guarded(
    estimated_vram_mb: int,
    safety_margin_mb_getter: Callable[[], int] = lambda: 512,
) -> Callable[[Callable[..., Awaitable[object]]], Callable[..., Awaitable[object]]]:
    """Decorate an async handler so it raises [`ResourceExhausted`] when
    the current free VRAM is below ``estimated_vram_mb + safety_margin``.

    ``safety_margin_mb_getter`` is a callable so the FastAPI dependency
    that holds [`IaConfig`] can be threaded in lazily (config may be
    reloaded between requests).
    """

    def decorator(fn: Callable[..., Awaitable[object]]) -> Callable[..., Awaitable[object]]:
        @functools.wraps(fn)
        async def wrapper(*args: object, **kwargs: object) -> object:
            safety = safety_margin_mb_getter()
            if not has_enough_vram(estimated_vram_mb, safety):
                snapshot = read_vram(0)
                free = snapshot.free_mb if snapshot else 0
                logger.warning(
                    "vram_admission_denied",
                    needed_mb=estimated_vram_mb,
                    safety_mb=safety,
                    free_mb=free,
                )
                raise ResourceExhausted(free_mb=free, needed_mb=estimated_vram_mb + safety)
            return await fn(*args, **kwargs)

        return wrapper

    return decorator
