"""Validate the generated `strata_ia.v1` proto stubs.

These tests guarantee the stubs:
1. Import cleanly (regression guard for the relative-import fix).
2. Round-trip every message + oneof we serialise on the wire.
3. Expose the 5 IaService RPCs the Rust client expects.
"""

from __future__ import annotations

import pytest
from strata_ia.proto import strata_ia_pb2 as pb
from strata_ia.proto import strata_ia_pb2_grpc as pb_grpc


def test_package_version_is_v1() -> None:
    """Schema namespace must stay `strata.ia.v1` until a v2 ADR ships."""
    assert pb.DESCRIPTOR.package == "strata.ia.v1"


def test_iaservice_exposes_expected_rpcs() -> None:
    service = pb.DESCRIPTOR.services_by_name["IaService"]
    method_names = {m.name for m in service.methods}
    assert method_names == {"OcrPage", "ExtractTable", "DescribeImage", "OcrFormula", "ProcessStream"}


def test_processstream_is_bidirectional() -> None:
    service = pb.DESCRIPTOR.services_by_name["IaService"]
    process_stream = next(m for m in service.methods if m.name == "ProcessStream")
    assert process_stream.client_streaming is True
    assert process_stream.server_streaming is True


def test_grpc_stubs_present() -> None:
    assert hasattr(pb_grpc, "IaServiceStub")
    assert hasattr(pb_grpc, "IaServiceServicer")
    assert hasattr(pb_grpc, "add_IaServiceServicer_to_server")


@pytest.mark.parametrize(
    "msg_factory",
    [
        lambda: pb.BBox(x0=0.0, y0=0.0, x1=10.0, y1=10.0),
        lambda: pb.Crop(png_bytes=b"\x89PNG", dpi=200, page_no=1, bbox=pb.BBox(x0=0, y0=0, x1=10, y1=10), hint="table-borderless"),
        lambda: pb.OcrResult(text="hello", words=[pb.WordBox(text="hi", bbox=pb.BBox(x0=0,y0=0,x1=1,y1=1), confidence=0.9)], confidence=0.85, language="en"),
        lambda: pb.TableResult(rows=[pb.TableRow(cells=[pb.TableCell(text="a", row=0, col=0, row_span=1, col_span=1)])], confidence=0.95, cell_count=1),
        lambda: pb.ImageDescription(caption="cat", description="A cat sitting.", alt_text="cat photo", confidence=0.8),
        lambda: pb.FormulaResult(latex=r"\frac{1}{2}", mathml="", confidence=0.92),
        lambda: pb.Provenance(model_id="qwen2.5vl:7b", backend="ollama", latency_ms=230, retries=1, cache_hit=False),
    ],
)
def test_message_round_trip(msg_factory) -> None:  # type: ignore[no-untyped-def]
    msg = msg_factory()
    raw = msg.SerializeToString()
    decoded = msg.__class__.FromString(raw)
    assert decoded == msg


def test_stream_result_oneof_discriminates() -> None:
    sr = pb.StreamResult(
        correlation_id="abc",
        ocr=pb.OcrResponse(result=pb.OcrResult(text="hi", confidence=0.5), provenance=pb.Provenance(model_id="m", backend="b", latency_ms=1)),
    )
    assert sr.WhichOneof("payload") == "ocr"
    raw = sr.SerializeToString()
    back = pb.StreamResult.FromString(raw)
    assert back.WhichOneof("payload") == "ocr"
    assert back.ocr.result.text == "hi"


def test_triage_route_enum_values_match_plan() -> None:
    # The numeric values are referenced by the Rust scheduler; lock them in.
    assert pb.TRIAGE_ROUTE_UNSPECIFIED == 0
    assert pb.TRIAGE_ROUTE_OCR_PAGE == 1
    assert pb.TRIAGE_ROUTE_TABLE == 2
    assert pb.TRIAGE_ROUTE_IMAGE == 3
    assert pb.TRIAGE_ROUTE_FORMULA == 4
