# Strata-Reader — Golden Fixture Corpus

This directory holds the canonical PDFs used by unit, integration and E2E tests.
See `docs/plan/plan_maestro.md` §4 and `docs/task/tareas.md` T0.7 for the
provenance contract.

## Layout

```
fixtures/
├── pdfs/                  # The 8 canonical PDFs (one per Triage branch)
│   └── CHECKSUMS.sha256   # SHA-256 of every PDF in this directory
├── sources/               # Reproducible LaTeX / scripts that regenerate fixtures
└── expected/              # Golden outputs (*.golden.json, *.golden.md, ground-truth text)
```

## Reproducibility

| Fixture                    | Source                                                                 | Status |
| -------------------------- | ---------------------------------------------------------------------- | ------ |
| `two_column_paper.pdf`     | arXiv 1706.03762 — Vaswani et al., *Attention Is All You Need* (2017). | ✔ downloaded |
| `native_simple.pdf`        | Generated from `sources/native_simple.tex` (LaTeX).                    | TODO   |
| `scanned_paper.pdf`        | Raster of `two_column_paper.pdf` first 6 pages via `pdftoppm`+`img2pdf`. | TODO  |
| `cid_corrupted.pdf`        | LaTeX with CJK font and no `ToUnicode` mapping.                        | TODO   |
| `borderless_table.pdf`     | LaTeX with `booktabs` (no vertical rules).                             | TODO   |
| `equation_heavy.pdf`       | LaTeX with 30+ display equations.                                      | TODO   |
| `figure_with_caption.pdf`  | LaTeX `\includegraphics` + `\caption`.                                  | TODO   |
| `mixed_lang_arabic.pdf`    | LaTeX `polyglossia` with Arabic + Latin paragraphs.                    | TODO   |

To rebuild or verify the corpus:

```bash
uv run python scripts/seed_fixtures.py --verify     # checksum-only
uv run python scripts/seed_fixtures.py --download   # re-download arXiv PDFs
uv run python scripts/seed_fixtures.py --build      # regenerate LaTeX-based fixtures (requires TeX Live)
```

## Licensing

- `two_column_paper.pdf`: arXiv non-exclusive distribution license, original
  copyright Vaswani et al. Redistributed solely for test purposes; treat as
  read-only.
- All `sources/*.tex` files are authored within this project, Apache-2.0.

## Manual review trail

When fixtures change in a way that affects `expected/*.golden.*`, document the
diff in `expected/REVIEW.md` (see T10.1.A10.1.2 in `docs/task/tareas.md`).
