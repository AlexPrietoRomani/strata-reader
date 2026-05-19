from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class TriageRoute(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    TRIAGE_ROUTE_UNSPECIFIED: _ClassVar[TriageRoute]
    TRIAGE_ROUTE_OCR_PAGE: _ClassVar[TriageRoute]
    TRIAGE_ROUTE_TABLE: _ClassVar[TriageRoute]
    TRIAGE_ROUTE_IMAGE: _ClassVar[TriageRoute]
    TRIAGE_ROUTE_FORMULA: _ClassVar[TriageRoute]
TRIAGE_ROUTE_UNSPECIFIED: TriageRoute
TRIAGE_ROUTE_OCR_PAGE: TriageRoute
TRIAGE_ROUTE_TABLE: TriageRoute
TRIAGE_ROUTE_IMAGE: TriageRoute
TRIAGE_ROUTE_FORMULA: TriageRoute

class BBox(_message.Message):
    __slots__ = ("x0", "y0", "x1", "y1")
    X0_FIELD_NUMBER: _ClassVar[int]
    Y0_FIELD_NUMBER: _ClassVar[int]
    X1_FIELD_NUMBER: _ClassVar[int]
    Y1_FIELD_NUMBER: _ClassVar[int]
    x0: float
    y0: float
    x1: float
    y1: float
    def __init__(self, x0: _Optional[float] = ..., y0: _Optional[float] = ..., x1: _Optional[float] = ..., y1: _Optional[float] = ...) -> None: ...

class Crop(_message.Message):
    __slots__ = ("png_bytes", "dpi", "page_no", "bbox", "hint")
    PNG_BYTES_FIELD_NUMBER: _ClassVar[int]
    DPI_FIELD_NUMBER: _ClassVar[int]
    PAGE_NO_FIELD_NUMBER: _ClassVar[int]
    BBOX_FIELD_NUMBER: _ClassVar[int]
    HINT_FIELD_NUMBER: _ClassVar[int]
    png_bytes: bytes
    dpi: int
    page_no: int
    bbox: BBox
    hint: str
    def __init__(self, png_bytes: _Optional[bytes] = ..., dpi: _Optional[int] = ..., page_no: _Optional[int] = ..., bbox: _Optional[_Union[BBox, _Mapping]] = ..., hint: _Optional[str] = ...) -> None: ...

class WordBox(_message.Message):
    __slots__ = ("text", "bbox", "confidence")
    TEXT_FIELD_NUMBER: _ClassVar[int]
    BBOX_FIELD_NUMBER: _ClassVar[int]
    CONFIDENCE_FIELD_NUMBER: _ClassVar[int]
    text: str
    bbox: BBox
    confidence: float
    def __init__(self, text: _Optional[str] = ..., bbox: _Optional[_Union[BBox, _Mapping]] = ..., confidence: _Optional[float] = ...) -> None: ...

class OcrResult(_message.Message):
    __slots__ = ("text", "words", "confidence", "language")
    TEXT_FIELD_NUMBER: _ClassVar[int]
    WORDS_FIELD_NUMBER: _ClassVar[int]
    CONFIDENCE_FIELD_NUMBER: _ClassVar[int]
    LANGUAGE_FIELD_NUMBER: _ClassVar[int]
    text: str
    words: _containers.RepeatedCompositeFieldContainer[WordBox]
    confidence: float
    language: str
    def __init__(self, text: _Optional[str] = ..., words: _Optional[_Iterable[_Union[WordBox, _Mapping]]] = ..., confidence: _Optional[float] = ..., language: _Optional[str] = ...) -> None: ...

class TableCell(_message.Message):
    __slots__ = ("text", "row", "col", "row_span", "col_span")
    TEXT_FIELD_NUMBER: _ClassVar[int]
    ROW_FIELD_NUMBER: _ClassVar[int]
    COL_FIELD_NUMBER: _ClassVar[int]
    ROW_SPAN_FIELD_NUMBER: _ClassVar[int]
    COL_SPAN_FIELD_NUMBER: _ClassVar[int]
    text: str
    row: int
    col: int
    row_span: int
    col_span: int
    def __init__(self, text: _Optional[str] = ..., row: _Optional[int] = ..., col: _Optional[int] = ..., row_span: _Optional[int] = ..., col_span: _Optional[int] = ...) -> None: ...

class TableRow(_message.Message):
    __slots__ = ("cells",)
    CELLS_FIELD_NUMBER: _ClassVar[int]
    cells: _containers.RepeatedCompositeFieldContainer[TableCell]
    def __init__(self, cells: _Optional[_Iterable[_Union[TableCell, _Mapping]]] = ...) -> None: ...

class TableResult(_message.Message):
    __slots__ = ("rows", "confidence", "cell_count")
    ROWS_FIELD_NUMBER: _ClassVar[int]
    CONFIDENCE_FIELD_NUMBER: _ClassVar[int]
    CELL_COUNT_FIELD_NUMBER: _ClassVar[int]
    rows: _containers.RepeatedCompositeFieldContainer[TableRow]
    confidence: float
    cell_count: int
    def __init__(self, rows: _Optional[_Iterable[_Union[TableRow, _Mapping]]] = ..., confidence: _Optional[float] = ..., cell_count: _Optional[int] = ...) -> None: ...

class ImageDescription(_message.Message):
    __slots__ = ("caption", "description", "alt_text", "confidence")
    CAPTION_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ALT_TEXT_FIELD_NUMBER: _ClassVar[int]
    CONFIDENCE_FIELD_NUMBER: _ClassVar[int]
    caption: str
    description: str
    alt_text: str
    confidence: float
    def __init__(self, caption: _Optional[str] = ..., description: _Optional[str] = ..., alt_text: _Optional[str] = ..., confidence: _Optional[float] = ...) -> None: ...

class FormulaResult(_message.Message):
    __slots__ = ("latex", "mathml", "confidence")
    LATEX_FIELD_NUMBER: _ClassVar[int]
    MATHML_FIELD_NUMBER: _ClassVar[int]
    CONFIDENCE_FIELD_NUMBER: _ClassVar[int]
    latex: str
    mathml: str
    confidence: float
    def __init__(self, latex: _Optional[str] = ..., mathml: _Optional[str] = ..., confidence: _Optional[float] = ...) -> None: ...

class Provenance(_message.Message):
    __slots__ = ("model_id", "backend", "latency_ms", "retries", "cache_hit")
    MODEL_ID_FIELD_NUMBER: _ClassVar[int]
    BACKEND_FIELD_NUMBER: _ClassVar[int]
    LATENCY_MS_FIELD_NUMBER: _ClassVar[int]
    RETRIES_FIELD_NUMBER: _ClassVar[int]
    CACHE_HIT_FIELD_NUMBER: _ClassVar[int]
    model_id: str
    backend: str
    latency_ms: int
    retries: int
    cache_hit: bool
    def __init__(self, model_id: _Optional[str] = ..., backend: _Optional[str] = ..., latency_ms: _Optional[int] = ..., retries: _Optional[int] = ..., cache_hit: bool = ...) -> None: ...

class OcrResponse(_message.Message):
    __slots__ = ("result", "provenance")
    RESULT_FIELD_NUMBER: _ClassVar[int]
    PROVENANCE_FIELD_NUMBER: _ClassVar[int]
    result: OcrResult
    provenance: Provenance
    def __init__(self, result: _Optional[_Union[OcrResult, _Mapping]] = ..., provenance: _Optional[_Union[Provenance, _Mapping]] = ...) -> None: ...

class TableResponse(_message.Message):
    __slots__ = ("result", "provenance")
    RESULT_FIELD_NUMBER: _ClassVar[int]
    PROVENANCE_FIELD_NUMBER: _ClassVar[int]
    result: TableResult
    provenance: Provenance
    def __init__(self, result: _Optional[_Union[TableResult, _Mapping]] = ..., provenance: _Optional[_Union[Provenance, _Mapping]] = ...) -> None: ...

class ImageResponse(_message.Message):
    __slots__ = ("result", "provenance")
    RESULT_FIELD_NUMBER: _ClassVar[int]
    PROVENANCE_FIELD_NUMBER: _ClassVar[int]
    result: ImageDescription
    provenance: Provenance
    def __init__(self, result: _Optional[_Union[ImageDescription, _Mapping]] = ..., provenance: _Optional[_Union[Provenance, _Mapping]] = ...) -> None: ...

class FormulaResponse(_message.Message):
    __slots__ = ("result", "provenance")
    RESULT_FIELD_NUMBER: _ClassVar[int]
    PROVENANCE_FIELD_NUMBER: _ClassVar[int]
    result: FormulaResult
    provenance: Provenance
    def __init__(self, result: _Optional[_Union[FormulaResult, _Mapping]] = ..., provenance: _Optional[_Union[Provenance, _Mapping]] = ...) -> None: ...

class StreamCrop(_message.Message):
    __slots__ = ("correlation_id", "route", "crop")
    CORRELATION_ID_FIELD_NUMBER: _ClassVar[int]
    ROUTE_FIELD_NUMBER: _ClassVar[int]
    CROP_FIELD_NUMBER: _ClassVar[int]
    correlation_id: str
    route: TriageRoute
    crop: Crop
    def __init__(self, correlation_id: _Optional[str] = ..., route: _Optional[_Union[TriageRoute, str]] = ..., crop: _Optional[_Union[Crop, _Mapping]] = ...) -> None: ...

class StreamResult(_message.Message):
    __slots__ = ("correlation_id", "ocr", "table", "image", "formula", "error")
    CORRELATION_ID_FIELD_NUMBER: _ClassVar[int]
    OCR_FIELD_NUMBER: _ClassVar[int]
    TABLE_FIELD_NUMBER: _ClassVar[int]
    IMAGE_FIELD_NUMBER: _ClassVar[int]
    FORMULA_FIELD_NUMBER: _ClassVar[int]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    correlation_id: str
    ocr: OcrResponse
    table: TableResponse
    image: ImageResponse
    formula: FormulaResponse
    error: StreamError
    def __init__(self, correlation_id: _Optional[str] = ..., ocr: _Optional[_Union[OcrResponse, _Mapping]] = ..., table: _Optional[_Union[TableResponse, _Mapping]] = ..., image: _Optional[_Union[ImageResponse, _Mapping]] = ..., formula: _Optional[_Union[FormulaResponse, _Mapping]] = ..., error: _Optional[_Union[StreamError, _Mapping]] = ...) -> None: ...

class StreamError(_message.Message):
    __slots__ = ("code", "message")
    CODE_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    code: int
    message: str
    def __init__(self, code: _Optional[int] = ..., message: _Optional[str] = ...) -> None: ...
