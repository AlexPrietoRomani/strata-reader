# mypy: disable-error-code="no-any-return,no-untyped-call"
"""gRPC server exposing the same handlers as the FastAPI routers.

This is the **primary** transport used by the Rust bridge (``strata-ia-bridge``).
The FastAPI HTTP surface remains for debugging and for clients that prefer
JSON, but the Triage Engine and the scheduler talk gRPC for throughput.

Implements:

- ``grpc.health.v1.Health`` — minimal Check method that returns ``SERVING``
  unconditionally once the server is up. Plan Maestro §10.T5.1 AC:
  ``grpcurl localhost:50051 strata.ia.v1.Health/Check`` → SERVING.
- ``strata.ia.v1.IaService`` — the four unary RPCs (OcrPage, ExtractTable,
  DescribeImage, OcrFormula) plus the bidi ``ProcessStream``.

The actual work is delegated to the same adapter / VLM-dispatch helpers
the FastAPI routers use, so behaviour is identical across transports.
"""

from __future__ import annotations

import asyncio
import time
from collections.abc import AsyncIterator
from typing import Any

import grpc
import structlog
from pydantic import ValidationError

from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import FormulaResult, ImageDescription, OcrResult, TableResult
from strata_ia.proto import strata_ia_pb2 as pb
from strata_ia.proto import strata_ia_pb2_grpc as pb_grpc
from strata_ia.routers.prompts import (
    DESCRIBE_IMAGE_PROMPT,
    EXTRACT_TABLE_PROMPT,
    OCR_FORMULA_PROMPT,
    OCR_PAGE_PROMPT,
)

logger = structlog.get_logger(__name__)


# ---------------------------------------------------------------------------
# Minimal grpc.health.v1.Health implementation (we avoid the optional
# grpcio-health-checking dep — it's a single Check method).
# ---------------------------------------------------------------------------

HEALTH_SERVICE_DESCRIPTOR_NAME = "grpc.health.v1.Health"


class HealthServicer:
    """Implements the standard grpc.health.v1.Health/Check RPC manually."""

    SERVING = 1

    async def Check(self, request: Any, context: grpc.aio.ServicerContext) -> Any:
        # Construct the response struct on the fly using grpc's reflection-free
        # generic message machinery — we don't ship the standard health
        # protos to keep the dep graph small.
        from google.protobuf.descriptor_pool import Default as _default_pool
        from google.protobuf.message_factory import GetMessageClass

        try:
            descriptor = _default_pool().FindMessageTypeByName("grpc.health.v1.HealthCheckResponse")
            response_cls = GetMessageClass(descriptor)
            return response_cls(status=self.SERVING)
        except KeyError:
            # The health proto descriptor isn't registered (the standard
            # health module wasn't imported). Fall back to an opaque
            # placeholder — the Rust bridge only cares about the
            # status code being SERVING (1).
            class _Placeholder:
                status = self.SERVING

            return _Placeholder()


# ---------------------------------------------------------------------------
# IaServiceServicer
# ---------------------------------------------------------------------------


class IaServiceServicer(pb_grpc.IaServiceServicer):
    """Routes gRPC RPCs through the same adapters as the FastAPI surface."""

    def __init__(self, ollama: OllamaClient, config: IaConfig) -> None:
        self._ollama = ollama
        self._config = config

    # ---- unary -----------------------------------------------------------

    async def OcrPage(self, request: pb.Crop, context: grpc.aio.ServicerContext) -> pb.OcrResponse:
        return await self._run_unary(
            request=request,
            context=context,
            model=self._config.model_ocr_fallback,
            prompt=OCR_PAGE_PROMPT,
            pyd_model=OcrResult,
            wrap=_wrap_ocr,
        )

    async def ExtractTable(self, request: pb.Crop, context: grpc.aio.ServicerContext) -> pb.TableResponse:
        return await self._run_unary(
            request=request,
            context=context,
            model=self._config.model_table,
            prompt=EXTRACT_TABLE_PROMPT,
            pyd_model=TableResult,
            wrap=_wrap_table,
        )

    async def DescribeImage(self, request: pb.Crop, context: grpc.aio.ServicerContext) -> pb.ImageResponse:
        return await self._run_unary(
            request=request,
            context=context,
            model=self._config.model_image,
            prompt=DESCRIBE_IMAGE_PROMPT,
            pyd_model=ImageDescription,
            wrap=_wrap_image,
        )

    async def OcrFormula(self, request: pb.Crop, context: grpc.aio.ServicerContext) -> pb.FormulaResponse:
        return await self._run_unary(
            request=request,
            context=context,
            model=self._config.model_formula,
            prompt=OCR_FORMULA_PROMPT,
            pyd_model=FormulaResult,
            wrap=_wrap_formula,
        )

    # ---- bidi streaming --------------------------------------------------

    async def ProcessStream(
        self,
        request_iterator: AsyncIterator[pb.StreamCrop],
        context: grpc.aio.ServicerContext,
    ) -> AsyncIterator[pb.StreamResult]:
        async for stream_crop in request_iterator:
            corr_id = stream_crop.correlation_id
            try:
                if stream_crop.route == pb.TRIAGE_ROUTE_TABLE:
                    table_resp = await self.ExtractTable(stream_crop.crop, context)
                    yield pb.StreamResult(correlation_id=corr_id, table=table_resp)
                elif stream_crop.route == pb.TRIAGE_ROUTE_IMAGE:
                    img_resp = await self.DescribeImage(stream_crop.crop, context)
                    yield pb.StreamResult(correlation_id=corr_id, image=img_resp)
                elif stream_crop.route == pb.TRIAGE_ROUTE_FORMULA:
                    f_resp = await self.OcrFormula(stream_crop.crop, context)
                    yield pb.StreamResult(correlation_id=corr_id, formula=f_resp)
                elif stream_crop.route == pb.TRIAGE_ROUTE_OCR_PAGE:
                    ocr_resp = await self.OcrPage(stream_crop.crop, context)
                    yield pb.StreamResult(correlation_id=corr_id, ocr=ocr_resp)
                else:
                    yield pb.StreamResult(
                        correlation_id=corr_id,
                        error=pb.StreamError(code=3, message=f"unknown route {stream_crop.route}"),
                    )
            except Exception as exc:
                logger.warning("stream_handler_failed", corr=corr_id, error=str(exc))
                yield pb.StreamResult(
                    correlation_id=corr_id, error=pb.StreamError(code=13, message=str(exc))
                )

    # ---- internals -------------------------------------------------------

    async def _run_unary(
        self,
        *,
        request: pb.Crop,
        context: grpc.aio.ServicerContext,
        model: str,
        prompt: str,
        pyd_model: type,
        wrap: Any,
    ) -> Any:
        start = time.perf_counter()
        try:
            result = await self._ollama.generate(
                model=model,
                prompt=prompt,
                images=[bytes(request.png_bytes)],
                format_json=True,
            )
        except OllamaUnreachable as exc:
            await context.abort(grpc.StatusCode.UNAVAILABLE, str(exc))
            raise
        except OllamaError as exc:
            await context.abort(grpc.StatusCode.INTERNAL, str(exc))
            raise

        try:
            parsed = pyd_model.model_validate_json(result.text)  # type: ignore[attr-defined]
        except ValidationError as exc:
            await context.abort(grpc.StatusCode.INTERNAL, f"VLM returned malformed JSON: {exc.errors()}")
            raise

        latency_ms = int((time.perf_counter() - start) * 1000)
        provenance = pb.Provenance(model_id=model, backend="ollama", latency_ms=latency_ms)
        return wrap(parsed, provenance)


# ---------------------------------------------------------------------------
# Pydantic ↔ proto wrappers
# ---------------------------------------------------------------------------


def _wrap_ocr(parsed: OcrResult, provenance: pb.Provenance) -> pb.OcrResponse:
    return pb.OcrResponse(
        result=pb.OcrResult(
            text=parsed.text,
            words=[
                pb.WordBox(
                    text=w.text,
                    bbox=pb.BBox(x0=w.bbox.x0, y0=w.bbox.y0, x1=w.bbox.x1, y1=w.bbox.y1),
                    confidence=w.confidence,
                )
                for w in parsed.words
            ],
            confidence=parsed.confidence,
            language=parsed.language or "",
        ),
        provenance=provenance,
    )


def _wrap_table(parsed: TableResult, provenance: pb.Provenance) -> pb.TableResponse:
    return pb.TableResponse(
        result=pb.TableResult(
            rows=[
                pb.TableRow(
                    cells=[
                        pb.TableCell(
                            text=c.text,
                            row=c.row,
                            col=c.col,
                            row_span=c.row_span,
                            col_span=c.col_span,
                        )
                        for c in row.cells
                    ]
                )
                for row in parsed.rows
            ],
            confidence=parsed.confidence,
            cell_count=parsed.cell_count,
        ),
        provenance=provenance,
    )


def _wrap_image(parsed: ImageDescription, provenance: pb.Provenance) -> pb.ImageResponse:
    return pb.ImageResponse(
        result=pb.ImageDescription(
            caption=parsed.caption,
            description=parsed.description,
            alt_text=parsed.alt_text,
            confidence=parsed.confidence,
        ),
        provenance=provenance,
    )


def _wrap_formula(parsed: FormulaResult, provenance: pb.Provenance) -> pb.FormulaResponse:
    return pb.FormulaResponse(
        result=pb.FormulaResult(
            latex=parsed.latex,
            mathml=parsed.mathml or "",
            confidence=parsed.confidence,
        ),
        provenance=provenance,
    )


# ---------------------------------------------------------------------------
# Server lifecycle
# ---------------------------------------------------------------------------


async def serve(ollama: OllamaClient, config: IaConfig, port: int = 50051) -> grpc.aio.Server:
    """Create and start an aio gRPC server bound to ``port``. Returns the
    Server handle so the caller can ``await server.wait_for_termination()``
    or ``await server.stop(grace_seconds)``."""
    server = grpc.aio.server()
    pb_grpc.add_IaServiceServicer_to_server(IaServiceServicer(ollama, config), server)

    # Register the manual Health servicer under its expected service name.
    # We avoid the optional grpcio-health-checking dep — Health.Check is
    # trivial and we only need the Rust client to see SERVING.
    _attach_minimal_health(server)

    server.add_insecure_port(f"0.0.0.0:{port}")
    await server.start()
    logger.info("grpc_server_started", port=port)
    return server


def _attach_minimal_health(server: grpc.aio.Server) -> None:
    """Register a tiny custom-coded grpc.health.v1.Health/Check handler.

    We build a generic-handler tuple manually so we don't pull in the
    `grpcio-health-checking` package.
    """
    health_servicer = HealthServicer()

    async def _check(request_bytes: bytes, context: grpc.aio.ServicerContext) -> bytes:
        # The standard HealthCheckResponse is { status: 1 } where 1 = SERVING.
        # Encoded as a varint: 0x08 0x01. Field 1 (status), wire-type 0
        # (varint), value 1 (SERVING).
        _ = await health_servicer.Check(request_bytes, context)
        return b"\x08\x01"

    rpc_method_handlers = {
        "Check": grpc.unary_unary_rpc_method_handler(
            _check,
            request_deserializer=bytes,
            response_serializer=bytes,
        ),
    }
    handler = grpc.method_handlers_generic_handler(HEALTH_SERVICE_DESCRIPTOR_NAME, rpc_method_handlers)
    server.add_generic_rpc_handlers((handler,))


def main() -> None:
    """Entry point for ``python -m strata_ia.grpc_server``."""
    from strata_ia.config import load_config

    async def _main() -> None:
        config = load_config()
        async with OllamaClient(
            endpoint=config.ollama_endpoint,
            timeout_s=config.ollama_request_timeout_s,
            retry_attempts=config.ollama_retry_attempts,
            seed=config.seed,
            temperature=config.temperature,
        ) as client:
            server = await serve(client, config, port=50051)
            await server.wait_for_termination()

    asyncio.run(_main())


if __name__ == "__main__":
    main()
