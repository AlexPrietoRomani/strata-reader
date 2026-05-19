"""Tests for the IA microservice configuration loader."""

from __future__ import annotations

import pytest

from strata_ia.config import IaConfig, load_config


def test_defaults_match_plan(monkeypatch: pytest.MonkeyPatch) -> None:
    # Clear any env that might leak from the host.
    for key in list(monkeypatch._setenv if hasattr(monkeypatch, "_setenv") else {}):
        monkeypatch.delenv(key, raising=False)
    cfg = IaConfig(_env_file=None)  # type: ignore[call-arg]
    assert cfg.ollama_endpoint == "http://localhost:11434"
    assert cfg.model_table == "qwen2.5vl:7b"
    assert cfg.temperature == 0.0
    assert cfg.seed == 42
    assert cfg.cache_enabled is True


def test_env_override(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("STRATA_IA_OLLAMA_ENDPOINT", "http://ollama:11434")
    monkeypatch.setenv("STRATA_IA_MODEL_TABLE", "minicpm-v:8b")
    monkeypatch.setenv("STRATA_IA_TEMPERATURE", "0.2")
    cfg = load_config()
    assert cfg.ollama_endpoint == "http://ollama:11434"
    assert cfg.model_table == "minicpm-v:8b"
    assert cfg.temperature == 0.2


def test_temperature_must_be_in_range() -> None:
    with pytest.raises(Exception):
        IaConfig(temperature=3.0, _env_file=None)  # type: ignore[call-arg]


def test_http_port_validated() -> None:
    with pytest.raises(Exception):
        IaConfig(http_port=70000, _env_file=None)  # type: ignore[call-arg]
