"""Runtime configuration for the IA microservice.

Resolution order (highest priority first):
    1. Environment variables prefixed with ``STRATA_IA_``.
    2. ``strata-reader.toml`` if present in the working directory.
    3. Hard-coded defaults below.

See ``docs/plan/plan_maestro.md`` §10 and §13.
"""

from __future__ import annotations

from pathlib import Path

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class IaConfig(BaseSettings):
    model_config = SettingsConfigDict(
        env_prefix="STRATA_IA_",
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
        case_sensitive=False,
    )

    # ----- Ollama connectivity -----
    ollama_endpoint: str = Field(default="http://localhost:11434")
    ollama_request_timeout_s: float = Field(default=60.0, ge=1.0, le=600.0)
    ollama_retry_attempts: int = Field(default=3, ge=1, le=10)

    # ----- Default models per task -----
    model_table: str = Field(default="qwen2.5vl:7b")
    model_image: str = Field(default="qwen2.5vl:7b")
    model_formula: str = Field(default="minicpm-v:8b")
    model_ocr_fallback: str = Field(default="qwen2.5vl:7b")

    # ----- HTTP server -----
    http_host: str = Field(default="0.0.0.0")
    http_port: int = Field(default=8081, ge=1, le=65535)

    # ----- gRPC server -----
    grpc_port: int = Field(default=50051, ge=1, le=65535)
    grpc_enabled: bool = Field(default=True)

    # ----- Cache -----
    cache_path: Path = Field(default=Path.home() / ".strata" / "cache.db")
    cache_enabled: bool = Field(default=True)

    # ----- Resource guard -----
    vram_safety_margin_mb: int = Field(default=512, ge=0)
    """How much VRAM to keep free above the model footprint at all times."""

    # ----- Determinism / generation -----
    seed: int = Field(default=42)
    temperature: float = Field(default=0.0, ge=0.0, le=2.0)

    # ----- Logging -----
    log_level: str = Field(default="INFO")
    """One of DEBUG / INFO / WARNING / ERROR / CRITICAL."""


def load_config() -> IaConfig:
    """Factory used by the FastAPI lifespan and the test fixtures."""
    return IaConfig()
