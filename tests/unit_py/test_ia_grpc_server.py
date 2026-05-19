"""Integration tests for the gRPC server using an in-process channel.

We spin up `serve()` on a free port, dial it with a real gRPC client (the
generated stub), and verify the four unary RPCs + the bidi stream work
end-to-end with a mocked Ollama backend.
"""

from __future__ import annotations

import asyncio
import json
import socket
from collections.abc import AsyncIterator
from contextlib import asynccontextmanager

import httpx
import pytest

from strata_ia.adapters.ollama import OllamaClient
from strata_ia.config import IaConfig
from strata_ia.grpc_server import serve
from strata_ia.proto import strata_ia_pb2 as pb
from strata_ia.proto import strata_ia_pb2_grpc as pb_grpc

import grpc


def _free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return int(s.getsockname()[1])


def _make_ollama_with_handler(handler) -> OllamaClient:  # type: ignore[no-untyped-def]
    transport = httpx.MockTransport(handler)
    http = httpx.AsyncClient(transport=transport, base_url="http://mock-ollama:11434")
    return OllamaClient(endpoint="http://mock-ollama:11434", retry_attempts=1, http_client=http)


@asynccontextmanager
async def _running_server(handler):  # type: ignore[no-untyped-def]
    port = _free_port()
    ollama = _make_ollama_with_handler(handler)
    cfg = IaConfig(_env_file=None)  # type: ignore[call-arg]
    server = await serve(ollama, cfg, port=port)
    try:
        yield port
    finally:
        await server.stop(grace=0)
        await ollama.aclose()


def _crop() -> pb.Crop:
    return pb.Crop(
        png_bytes=b"\x89PNG-fake",
        dpi=200,
        page_no=1,
        bbox=pb.BBox(x0=0, y0=0, x1=100, y1=50),
        hint="table-borderless",
    )


@pytest.mark.asyncio
async def test_extract_table_round_trip() -> None:
    vlm_payload = {
        "rows": [{"cells": [{"text": "a", "row": 0, "col": 0}, {"text": "b", "row": 0, "col": 1}]}],
        "confidence": 0.9,
        "cellCount": 2,
    }

    def handler(_request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            stub = pb_grpc.IaServiceStub(ch)
            resp = await stub.ExtractTable(_crop())
        assert resp.provenance.backend == "ollama"
        assert resp.provenance.model_id == "qwen2.5vl:7b"
        assert resp.result.cell_count == 2
        assert resp.result.confidence == pytest.approx(0.9, abs=1e-3)


@pytest.mark.asyncio
async def test_describe_image_round_trip() -> None:
    vlm_payload = {
        "caption": "graph",
        "description": "A bar chart with 4 bars.",
        "altText": "bar chart",
        "confidence": 0.85,
    }

    def handler(_request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            stub = pb_grpc.IaServiceStub(ch)
            resp = await stub.DescribeImage(_crop())
        assert resp.result.caption == "graph"
        assert resp.result.alt_text == "bar chart"


@pytest.mark.asyncio
async def test_ocr_formula_round_trip() -> None:
    vlm_payload = {"latex": r"\frac{1}{2}", "mathml": None, "confidence": 0.92}

    def handler(_request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            stub = pb_grpc.IaServiceStub(ch)
            resp = await stub.OcrFormula(_crop())
        assert resp.result.latex == r"\frac{1}{2}"
        # MathML is optional — the wrapper folds None → "".
        assert resp.result.mathml == ""


@pytest.mark.asyncio
async def test_ollama_unreachable_translates_to_unavailable() -> None:
    def handler(_request: httpx.Request) -> httpx.Response:
        raise httpx.ConnectError("nope")

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            stub = pb_grpc.IaServiceStub(ch)
            with pytest.raises(grpc.aio.AioRpcError) as exc:
                await stub.ExtractTable(_crop())
        assert exc.value.code() == grpc.StatusCode.UNAVAILABLE


@pytest.mark.asyncio
async def test_process_stream_correlates_responses() -> None:
    """Bi-directional streaming: send 3 crops, receive 3 results, correlation_id matched."""

    vlm_payload = {
        "caption": "x",
        "description": "y",
        "altText": "z",
        "confidence": 0.5,
    }

    def handler(_request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            stub = pb_grpc.IaServiceStub(ch)

            async def producer() -> AsyncIterator[pb.StreamCrop]:
                for i, route in enumerate(
                    [pb.TRIAGE_ROUTE_IMAGE, pb.TRIAGE_ROUTE_IMAGE, pb.TRIAGE_ROUTE_IMAGE]
                ):
                    yield pb.StreamCrop(correlation_id=f"req-{i}", route=route, crop=_crop())

            seen: set[str] = set()
            async for result in stub.ProcessStream(producer()):
                seen.add(result.correlation_id)
                assert result.WhichOneof("payload") == "image"
                assert result.image.result.caption == "x"
            assert seen == {"req-0", "req-1", "req-2"}


@pytest.mark.asyncio
async def test_health_check_returns_serving() -> None:
    """Plan Maestro §10.T5.1 AC — Health.Check must return SERVING (status=1)."""

    def handler(_request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": "{}"})

    async with _running_server(handler) as port:
        async with grpc.aio.insecure_channel(f"127.0.0.1:{port}") as ch:
            # The minimal Health implementation accepts an empty request body
            # and returns the wire-encoded { status: 1 } (2 bytes: 0x08 0x01).
            response = await ch.unary_unary(
                "/grpc.health.v1.Health/Check",
                request_serializer=lambda _b: b"",
                response_deserializer=lambda b: b,
            )(b"")
        assert response == b"\x08\x01"
