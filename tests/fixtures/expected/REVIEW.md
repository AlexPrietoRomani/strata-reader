# Strata-Reader — Goldens Review

Every entry here is a human acknowledgement that a regenerated golden was
inspected and intentionally promoted. Plan Maestro §15.T10.1 mandates that
no golden change merges without a matching block in this file.

## Initial state (pre-T10.1)

No goldens have been committed yet — the linker is EDR-blocked on the
maintainer's machine (see `docs/usage/IT_request.md`). The pipeline:

1. Build the CLI: `cargo build -p strata-cli --release` (waits on IT).
2. Boot Ollama with the VLM models: `scripts/dev_up.ps1`.
3. Regenerate the corpus: `uv run python scripts/regen_goldens.py`.
4. Open the diff, eyeball each `tests/fixtures/expected/*.golden.{md,json}`.
5. Append an entry below — one bullet per file with reviewer initials + date.
6. Commit the goldens AND this file in the same PR.

## Updated runs

_(empty — first regeneration pending)_
