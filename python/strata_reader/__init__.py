"""Strata-Reader — Python SDK.

The native module is built by `maturin` from `crates/strata-py`.
Phase 0 only exposes :func:`version`.
"""

from __future__ import annotations

try:
    from ._native import version
except ImportError as exc:  # pragma: no cover - native module not yet built
    msg = (
        "strata_reader._native is not available. Run `uv run maturin develop` "
        "to build the Rust extension, or `pip install strata-reader` once "
        "wheels are published."
    )
    raise ImportError(msg) from exc

__all__ = ["version"]
