"""Strata-Reader — Python SDK.

The heavy lifting lives in `strata_reader._native`, a Rust extension
module built by `maturin` (see ``crates/strata-py/``). This file
re-exports the public surface so consumers can write::

    from strata_reader import parse, parse_batch, ParseOptions, version

without remembering the internal module name.

When the native extension is missing (e.g. the user installed from
source without running ``maturin develop``), every public name raises
``ImportError`` with a clear pointer.
"""

from __future__ import annotations

try:
    from ._native import (
        Document,
        ParseOptions,
        parse,
        parse_batch,
        version,
    )
except ImportError as exc:  # pragma: no cover — native module missing
    msg = (
        "strata_reader._native is not available. Run `uv run maturin develop` "
        "to build the Rust extension, or `pip install strata-reader` once "
        "wheels are published."
    )
    raise ImportError(msg) from exc

__all__ = ["Document", "ParseOptions", "parse", "parse_batch", "version"]
