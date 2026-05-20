"""Type stubs for the Rust-backed `strata_reader._native` extension module."""

from typing import Any

def version() -> str:
    """Return the semver of the underlying Rust crate."""

class ParseOptions:
    """Knobs passed to `parse` / `parse_batch`."""

    profile: str
    use_ia: bool
    max_concurrent_pages: int | None
    media_dir: str | None
    ollama_endpoint: str

    def __init__(
        self,
        profile: str = "balanced",
        use_ia: bool = True,
        max_concurrent_pages: int | None = None,
        media_dir: str | None = None,
        ollama_endpoint: str = "http://localhost:11434",
    ) -> None: ...
    def __repr__(self) -> str: ...

class Document:
    """An immutable parsed PDF document — Markdown + Graph-RAG serialisable."""

    def to_markdown(self) -> str:
        """Render the document as GitHub-Flavoured Markdown."""

    def to_graph_json(self) -> dict[str, Any]:
        """Render the document as a Graph-RAG dict (nodes / edges / meta)."""

    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...

def parse(path: str, options: ParseOptions | None = None) -> Document:
    """Parse a single PDF and return a :class:`Document`."""

def parse_batch(
    paths: list[str], options: ParseOptions | None = None
) -> dict[str, Document | str]:
    """Parse multiple PDFs. Values are :class:`Document` on success and an
    error string on failure (one dict entry per input path)."""
