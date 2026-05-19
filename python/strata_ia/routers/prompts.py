"""Per-task prompt templates.

Centralised so prompt-engineering changes are reviewable in isolation.
The templates are JSON-shaped instructions — the VLM is requested to
respond as a single JSON object that maps cleanly into our Pydantic
response models.
"""

from __future__ import annotations

EXTRACT_TABLE_PROMPT = """\
You are a precise table extractor. The image contains a table from a scientific
paper. Return ONLY a JSON object with the following shape — no prose, no
markdown fences:

{
  "rows": [
    {"cells": [{"text": "...", "row": 0, "col": 0, "rowSpan": 1, "colSpan": 1}, ...]}
  ],
  "confidence": 0.0,
  "cellCount": 0
}

Rules:
- Read every cell exactly as printed, preserving capitalization and units.
- Use rowSpan / colSpan when cells visually span multiple rows or columns.
- Confidence is a single number in [0, 1] reflecting how confident you are
  about the overall extraction.
- cellCount is the total number of <cells> emitted.
"""

DESCRIBE_IMAGE_PROMPT = """\
You are a visual scientist. Look at the image and return ONLY a JSON object:

{
  "caption": "one-line summary <= 120 chars",
  "description": "two-to-four-sentence description for the body of the paper",
  "altText": "screen-reader-friendly alt text",
  "confidence": 0.0
}

Be neutral, factual, and concise. Do not invent details that are not visible.
"""

OCR_FORMULA_PROMPT = """\
The image contains a single mathematical formula. Return ONLY a JSON object:

{
  "latex": "<LaTeX source, no $$ wrappers>",
  "mathml": null,
  "confidence": 0.0
}

Use exactly the LaTeX commands required to typeset the formula. Do not
include surrounding text, equation numbers, or labels.
"""

OCR_PAGE_PROMPT = """\
Transcribe every legible word in the image, preserving line breaks and
paragraph structure. Return ONLY a JSON object:

{
  "text": "...",
  "words": [],
  "confidence": 0.0,
  "language": "en"
}

The words array can be empty if you cannot reliably box each word; the
caller will not attempt to derive layout from it.
"""
