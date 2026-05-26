# Graph Report - strata-reader  (2026-05-26)

## Corpus Check
- 120 files · ~649,266 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1258 nodes · 2066 edges · 78 communities (69 shown, 9 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 184 edges (avg confidence: 0.72)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d58144d3`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 48|Community 48]]
- [[_COMMUNITY_Community 49|Community 49]]
- [[_COMMUNITY_Community 50|Community 50]]
- [[_COMMUNITY_Community 51|Community 51]]
- [[_COMMUNITY_Community 52|Community 52]]
- [[_COMMUNITY_Community 53|Community 53]]
- [[_COMMUNITY_Community 54|Community 54]]
- [[_COMMUNITY_Community 55|Community 55]]
- [[_COMMUNITY_Community 56|Community 56]]
- [[_COMMUNITY_Community 57|Community 57]]
- [[_COMMUNITY_Community 58|Community 58]]
- [[_COMMUNITY_Community 59|Community 59]]
- [[_COMMUNITY_Community 60|Community 60]]
- [[_COMMUNITY_Community 61|Community 61]]
- [[_COMMUNITY_Community 62|Community 62]]
- [[_COMMUNITY_Community 63|Community 63]]
- [[_COMMUNITY_Community 64|Community 64]]
- [[_COMMUNITY_Community 65|Community 65]]
- [[_COMMUNITY_Community 66|Community 66]]
- [[_COMMUNITY_Community 68|Community 68]]
- [[_COMMUNITY_Community 69|Community 69]]
- [[_COMMUNITY_Community 70|Community 70]]
- [[_COMMUNITY_Community 71|Community 71]]

## God Nodes (most connected - your core abstractions)
1. `OllamaClient` - 19 edges
2. `IaServiceServicer` - 18 edges
3. `BBox` - 16 edges
4. `parse()` - 15 edges
5. `render()` - 15 edges
6. `main()` - 14 edges
7. `IaConfig` - 14 edges
8. `build_tree()` - 13 edges
9. `render()` - 13 edges
10. `version()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `parse()`  [INFERRED]
  xtask/src/main.rs → crates/strata-py/src/lib.rs
- `test_defaults_match_plan()` --calls--> `IaConfig`  [INFERRED]
  tests/unit_py/test_ia_config.py → python/strata_ia/config.py
- `test_temperature_must_be_in_range()` --calls--> `IaConfig`  [INFERRED]
  tests/unit_py/test_ia_config.py → python/strata_ia/config.py
- `test_http_port_validated()` --calls--> `IaConfig`  [INFERRED]
  tests/unit_py/test_ia_config.py → python/strata_ia/config.py
- `_make_app()` --calls--> `IaConfig`  [INFERRED]
  tests/unit_py/test_ia_routers.py → python/strata_ia/config.py

## Communities (78 total, 9 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.05
Nodes (32): detect(), detect_never_panics_on_cpu_only_host(), GpuBackend, GpuDeviceSnapshot, GpuMonitor, GpuMonitorError, GpuSnapshot, MetalMonitor (+24 more)

### Community 1 - "Community 1"
Cohesion: 0.06
Nodes (20): area_zero_when_degenerate(), BBox, bbox_strategy(), finite_f32(), GeometryError, intersect_disjoint_returns_none(), intersect_identical_iou_is_one(), Point (+12 more)

### Community 2 - "Community 2"
Cohesion: 0.07
Nodes (17): bench(), canned_crop(), canned_ocr(), canned_provenance(), EchoIa, spawn_server(), BridgeClient, BridgeClientConfig (+9 more)

### Community 3 - "Community 3"
Cohesion: 0.07
Nodes (36): BaseSettings, IaConfig, load_config(), Runtime configuration for the IA microservice.  Resolution order (highest priori, Factory used by the FastAPI lifespan and the test fixtures., create_app(), lifespan(), main() (+28 more)

### Community 4 - "Community 4"
Cohesion: 0.11
Nodes (21): Job, job_id_from_str_parses_ulid(), job_id_round_trips_through_json(), job_omits_result_fields_when_none(), job_round_trip_through_json_preserves_optional_results(), job_store_error_has_useful_display(), JobId, JobStatus (+13 more)

### Community 5 - "Community 5"
Cohesion: 0.07
Nodes (21): bbox_from_rect(), char_to_glyph(), extract_glyphs(), Glyph, glyph_round_trips_through_json(), pack_rgba(), bbox_from_quad(), dpi_estimate_basic() (+13 more)

### Community 6 - "Community 6"
Cohesion: 0.07
Nodes (17): Block, block_id_serializes_as_ulid_string(), block_round_trips_through_json(), block_type_round_trip(), BlockId, BlockType, sample_provenance(), decision_round_trips_through_json() (+9 more)

### Community 7 - "Community 7"
Cohesion: 0.07
Nodes (30): main(), Archivo: orchestrate_benchmarks.py Fecha de modificación: 26/05/2026 Autor: Al, Orquesta de forma secuencial todo el flujo de benchmarking comparativo., generate_benchmark_plot(), Archivo: plot_benchmark.py Fecha de modificación: 26/05/2026 Autor: Alex Priet, Genera dos gráficos de barras horizontales lado a lado para comparar     la pre, compute_extraction_accuracy(), compute_file_quality() (+22 more)

### Community 8 - "Community 8"
Cohesion: 0.12
Nodes (24): re_exports_are_reachable(), block(), doc(), empty_document_renders_to_empty_string(), equation_uses_display_math_by_default(), equation_uses_inline_math_when_short_and_threshold_set(), estimate_size(), header_footer_pagenumber_are_skipped() (+16 more)

### Community 9 - "Community 9"
Cohesion: 0.11
Nodes (28): _build_parser(), _build_scanned_paper(), cmd_build(), cmd_download(), cmd_regen_expected(), cmd_verify(), main(), Reproducible fixture management for strata-reader.  Usage::      uv run python s (+20 more)

### Community 10 - "Community 10"
Cohesion: 0.18
Nodes (19): ProfileName, profiles_are_strictly_ordered_by_aggressiveness(), round_trips_through_json(), TriageProfile, big_image_goes_to_vlm(), block(), BlockContext, bordered_table_escalates_in_scientific() (+11 more)

### Community 11 - "Community 11"
Cohesion: 0.09
Nodes (23): is_available(), _load_models(), Surya-OCR adapter — primary GPU OCR backend.  Surya is a GPU-friendly OCR + layo, Raised when Surya is not importable or model load failed., Load Surya's detection and recognition models. Idempotent., Probe without raising. Returns True iff Surya can be loaded., Run Surya on ``png_bytes`` and return text + average confidence., run_surya() (+15 more)

### Community 12 - "Community 12"
Cohesion: 0.1
Nodes (22): Exception, BridgeError, status_to_bridge_error_mapping(), _ensure_nvml(), guarded(), has_enough_vram(), VRAM-based admission control for the IA microservice.  Plan Maestro §10.T5.5 — t, Decorate an async handler so it raises [`ResourceExhausted`] when     the curren (+14 more)

### Community 13 - "Community 13"
Cohesion: 0.08
Nodes (13): object, IaService, IaServiceServicer, IaServiceStub, Bi-directional streaming for batch jobs. The Rust scheduler can push         ma, ---------------------------------------------------------------------------, ---------------------------------------------------------------------------, Constructor.          Args:             channel: A grpc.Channel. (+5 more)

### Community 14 - "Community 14"
Cohesion: 0.18
Nodes (13): BackoffReason, BackpressureConfig, BackpressureController, initial_clamps_to_max(), initial_concurrency_within_bounds(), LatencyWindow, on_failure_halves_concurrency(), on_failure_never_drops_below_min() (+5 more)

### Community 15 - "Community 15"
Cohesion: 0.14
Nodes (21): CacheOp, Cli, Cmd, cmd_bench(), cmd_cache_prune(), cmd_doctor(), cmd_gen_schema(), cmd_models_list() (+13 more)

### Community 16 - "Community 16"
Cohesion: 0.11
Nodes (15): CacheKey, open_cache(), SQLite-backed cache for IA results keyed by crop SHA-256.  Plan Maestro §10.T5.6, Stable SHA-256 helper exposed so callers don't need to import hashlib., Async SQLite cache. Use as a context manager via :func:`open_cache`., Delete entries older than ``days`` days. Returns rows removed., Open a cache at ``path``. Creates parent directories if needed., ResultCache (+7 more)

### Community 17 - "Community 17"
Cohesion: 0.09
Nodes (18): expected_for(), fixture_paths(), Shared fixtures for the E2E suite.  The whole tree is gated behind the ``@pyte, Return every golden PDF under tests/fixtures/pdfs/., `two_column_paper.pdf` + `.golden.json` → expected dir path., Skip the test when Ollama is unreachable on the configured endpoint., Resuelve el binario CLI `strata` y lo precalienta para el EDR corporativo., require_ollama() (+10 more)

### Community 18 - "Community 18"
Cohesion: 0.23
Nodes (15): add_block_to_chunk(), block(), build_block_page_index(), Chunk, ChunkBuilder, ChunkOptions, count_tokens(), doc_from() (+7 more)

### Community 19 - "Community 19"
Cohesion: 0.17
Nodes (14): cluster_axis_values(), count_cell_borders(), covers(), detect_table_borders(), detects_3x3_grid(), grid_3x3(), h(), LineSegment (+6 more)

### Community 20 - "Community 20"
Cohesion: 0.19
Nodes (21): attach_caption_edges(), block(), block_type_kebab(), doc(), each_block_becomes_one_node(), EdgeRelation, empty_document_yields_empty_graph(), figure_caption_pair_emits_caption_of_edge() (+13 more)

### Community 21 - "Community 21"
Cohesion: 0.13
Nodes (21): BaseModel, BBox, Crop, FormulaResult, ImageDescription, OcrResult, Provenance, Pydantic request / response models for every IA endpoint.  These models will be (+13 more)

### Community 22 - "Community 22"
Cohesion: 0.19
Nodes (17): Axis, bb(), cut_recursive(), empty_returns_empty(), find_largest_gap(), Gap, median_height(), order_partitions() (+9 more)

### Community 23 - "Community 23"
Cohesion: 0.15
Nodes (14): get_pdfium(), pdfium_available(), is_likely_scan(), arxiv_paper_is_not_a_scan(), extracts_images_without_panic(), extracts_vector_paths_without_panic(), first_page_has_glyphs(), fixture_path() (+6 more)

### Community 24 - "Community 24"
Cohesion: 0.23
Nodes (18): create_glyph(), create_line(), filter_noise_lines(), is_arxiv_watermark(), is_page_number(), is_stray_char(), median_font_size(), test_filter_noise_lines() (+10 more)

### Community 25 - "Community 25"
Cohesion: 0.25
Nodes (8): counter_increments_visible_in_output(), ia_request_duration_uses_task_and_model_labels(), Inner, Metrics, metrics_is_cloneable_and_shares_state(), queue_depth_gauge_can_decrease(), registers_all_expected_series(), vram_used_label_separates_devices()

### Community 26 - "Community 26"
Cohesion: 0.14
Nodes (14): _collect(), GenerateResult, OllamaError, OllamaUnreachable, Ollama HTTP adapter.  Wraps the Ollama ``/api/generate`` endpoint with determini, Blocking helper. Spins a fresh event loop — DO NOT call from inside     one (Fas, Raised when Ollama responds with a non-2xx HTTP status after all retries., Raised when the HTTP layer fails to even connect after all retries. (+6 more)

### Community 27 - "Community 27"
Cohesion: 0.27
Nodes (14): attach_section(), block(), blocks_before_first_heading_attach_to_root(), build_tree(), doc_from(), empty_document_yields_empty_root(), iter_blocks_visits_in_reading_order(), nested_headings_produce_nested_sections() (+6 more)

### Community 28 - "Community 28"
Cohesion: 0.21
Nodes (14): average_glyph_width(), close_word(), cluster_lines(), empty_input_yields_no_lines(), GlyphInput, jitter_under_tolerance_collapses_to_one_line(), Line, median_font_size() (+6 more)

### Community 29 - "Community 29"
Cohesion: 0.21
Nodes (15): aligned_columns_yield_candidate(), BorderlessCandidate, bounding_region(), ColumnAnchor, compute_column_anchors(), count_hits(), detect_table_candidates(), group_words_by_line() (+7 more)

### Community 30 - "Community 30"
Cohesion: 0.2
Nodes (14): all_unmapped_is_critical(), CidEvaluation, clean_english_text_is_severity_none(), degenerate_low_entropy_is_critical(), describe(), empty_page_is_severity_none(), evaluate_cid_health(), evaluation_round_trips_through_json() (+6 more)

### Community 31 - "Community 31"
Cohesion: 0.23
Nodes (13): big_gpu_host_prefers_surya_and_balanced(), Capabilities, cpu_only(), cpu_only_host_prefers_tesseract_and_fast(), detect_never_panics(), gpu_snap_with(), num_cpus(), OcrPreference (+5 more)

### Community 32 - "Community 32"
Cohesion: 0.26
Nodes (8): create_then_get_round_trips(), delete_is_idempotent(), get_unknown_returns_none(), job(), list_orders_newest_first(), MemoryJobStore, put_unknown_id_returns_not_found(), put_updates_existing_job()

### Community 33 - "Community 33"
Cohesion: 0.2
Nodes (6): Glyph, Image, Segment, VectorPath, version(), version_matches_pkg()

### Community 34 - "Community 34"
Cohesion: 0.28
Nodes (13): apply_payload(), FusionError, IaPayload, make_block(), make_doc(), merge(), merge_page(), merge_preserves_blocks_not_in_results() (+5 more)

### Community 35 - "Community 35"
Cohesion: 0.29
Nodes (13): allowlist_filters_gpus(), cpu_only_host_yields_empty_pool(), describe_pool(), describe_pool_is_deterministic(), labels_are_sanitized(), multiple_replicas_per_gpu(), one_worker_per_gpu_by_default(), plan_from_monitor() (+5 more)

### Community 36 - "Community 36"
Cohesion: 0.2
Nodes (6): finite_f32(), Matrix, matrix_strategy(), point_strategy(), scale_doubles_distance_from_origin(), translation_shifts_point()

### Community 37 - "Community 37"
Cohesion: 0.27
Nodes (8): available_permits_tracks_active_tasks(), default_limit_is_at_least_one(), num_cpus_estimate(), run_processes_every_item(), run_respects_concurrency_limit(), Scheduler, SchedulerConfig, spawn_handle_completes()

### Community 38 - "Community 38"
Cohesion: 0.24
Nodes (10): body_text_size(), build_heading_levels(), classify_headings(), dummy_bbox(), heading_content_filter(), heading_position_filter(), HeadingClass, histogram() (+2 more)

### Community 39 - "Community 39"
Cohesion: 0.16
Nodes (8): _attach_minimal_health(), HealthServicer, main(), Create and start an aio gRPC server bound to ``port``. Returns the     Server ha, Register a tiny custom-coded grpc.health.v1.Health/Check handler.      We build, Entry point for ``python -m strata_ia.grpc_server``., Implements the standard grpc.health.v1.Health/Check RPC manually., serve()

### Community 40 - "Community 40"
Cohesion: 0.15
Nodes (3): PureRustBackend, PureRustDoc, PureRustPage

### Community 41 - "Community 41"
Cohesion: 0.3
Nodes (10): cer_score(), _fmt(), main(), Fidelity benchmark — strata-reader vs opendataloader-pdf.  Plan Maestro §15.T10., render_report(), run_opendataloader(), run_strata(), Score (+2 more)

### Community 42 - "Community 42"
Cohesion: 0.23
Nodes (7): parse(), parse_batch(), PyParseOptions, sha256_file(), convert(), Archivo: __init__.py Fecha de modificación: 22/05/2026 Autor: Strata-Reader Co, Convierte uno o más archivos PDF a Markdown semántico y/o JSON estructurado para

### Community 43 - "Community 43"
Cohesion: 0.38
Nodes (11): _crop_payload(), _make_app(), Tests for the FastAPI routers using dependency overrides + MockTransport., Construct a minimal FastAPI app with a mocked Ollama client., test_describe_image_happy_path(), test_extract_table_happy_path(), test_invalid_json_from_vlm_returns_502(), test_ocr_formula_happy_path() (+3 more)

### Community 44 - "Community 44"
Cohesion: 0.29
Nodes (9): aggregate(), count_pages(), main(), _percentile(), Throughput benchmark — pages/second and GB/hour over the corpus.  Plan Maestro §, Best-effort page count without pulling pypdf. Reads the file and     counts `/Ty, render_report(), run_once() (+1 more)

### Community 45 - "Community 45"
Cohesion: 0.29
Nodes (9): main(), pixel_diff_percent(), Visual-regression bench — diff annotated PDFs against pixel goldens.  Plan Maest, Return the fraction of pixels (0..100) that differ between the     two RGB image, Render every page to PNG via the bundled `pdftoppm` if available,     otherwise, Build the annotated PDF (when strata is available) and render     every page. Re, render_pdf_pages(), render_report() (+1 more)

### Community 46 - "Community 46"
Cohesion: 0.31
Nodes (8): merge_lines_into_paragraphs(), ParagraphGroup, ParagraphKind, test_empty_lines_returns_empty(), test_heading_splits_body_paragraphs(), test_merge_consecutive_body_lines_small_gap(), test_only_headings_are_independent(), test_split_body_lines_large_gap()

### Community 47 - "Community 47"
Cohesion: 0.29
Nodes (3): Decoder, DecoderError, missing_file_yields_io_error()

### Community 48 - "Community 48"
Cohesion: 0.22
Nodes (4): OllamaClient, Stream incremental tokens. Used by long-running OCR jobs., Async client around the Ollama HTTP API., Return the tags currently pulled on the server (cheap reachability probe).

### Community 49 - "Community 49"
Cohesion: 0.47
Nodes (9): Invoke-ModelPull(), Show-Summary(), Start-OllamaIfNeeded(), Start-StrataServer(), Test-OllamaInstalled(), Test-OllamaReachable(), Write-Ok(), Write-Section() (+1 more)

### Community 50 - "Community 50"
Cohesion: 0.2
Nodes (3): Validate the generated `strata_ia.v1` proto stubs.  These tests guarantee the st, Schema namespace must stay `strata.ia.v1` until a v2 ADR ships., test_package_version_is_v1()

### Community 51 - "Community 51"
Cohesion: 0.25
Nodes (3): _crop(), Pydantic model round-trip tests for the IA microservice., test_crop_round_trips_with_aliases()

### Community 52 - "Community 52"
Cohesion: 0.25
Nodes (7): extract_table(), get_ollama(), POST /v1/extract/table — borderless table extraction via Ollama VLM., Table result enveloped with provenance for end-to-end tracing., Dependency: pull the singleton Ollama client out of app state., TableResponse, TableResult

### Community 54 - "Community 54"
Cohesion: 0.25
Nodes (6): _golden_files(), Cross-language schema validation.  Loads ``docs/schema/strata-document.schema., The generated file is itself a valid Draft 2020-12 JSON Schema., Soft signal — once goldens exist, this test stops being a no-op., test_at_least_one_golden_or_skip(), test_schema_is_valid_json_schema()

### Community 55 - "Community 55"
Cohesion: 0.32
Nodes (3): fix_letter_spacing(), normalize_text(), normalize_whitespace()

### Community 56 - "Community 56"
Cohesion: 0.33
Nodes (4): ImageDescription, describe_image(), ImageDescriptionResponse, POST /v1/describe/image — figure / chart description via Ollama VLM.

### Community 57 - "Community 57"
Cohesion: 0.33
Nodes (4): FormulaResult, FormulaResponse, ocr_formula(), POST /v1/ocr/formula — math formula → LaTeX via Ollama VLM.

### Community 58 - "Community 58"
Cohesion: 0.43
Nodes (6): main(), Regenerate the golden corpus under ``tests/fixtures/expected/``.  Plan Maestro §, Run `strata parse` and return (md, json) text contents., regen_one(), resolve_strata_bin(), write_review_template()

### Community 59 - "Community 59"
Cohesion: 0.47
Nodes (4): landscape_when_width_greater_than_height(), Page, PageOrientation, portrait_when_height_greater_than_width()

### Community 60 - "Community 60"
Cohesion: 0.4
Nodes (5): find_strata_binary(), main(), Archivo: cli.py Fecha de modificación: 22/05/2026 Autor: Strata-Reader Contribut, Invoca el binario native 'strata', cayendo en cascada o mostrando error instruct, Busca el binario nativo compilado de Rust 'strata' en cascada de fallbacks.

### Community 61 - "Community 61"
Cohesion: 0.33
Nodes (5): Archivo: test_pdfium_discovery.py Fecha de modificación: 26/05/2026 Autor: Str, Verifica que si la carpeta '_pdfium' existe en el directorio del paquete,     s, Verifica que si la variable de entorno 'STRATA_PDFIUM_LIB_PATH' ya está     est, test_pdfium_discovery_embedded(), test_pdfium_discovery_env_fallback()

### Community 63 - "Community 63"
Cohesion: 0.5
Nodes (3): PdfBackend, PdfDoc, PdfPage

## Knowledge Gaps
- **208 isolated node(s):** `Throughput benchmark — pages/second and GB/hour over the corpus.  Plan Maestro §`, `Best-effort page count without pulling pypdf. Reads the file and     counts `/Ty`, `Fidelity benchmark — strata-reader vs opendataloader-pdf.  Plan Maestro §15.T10.`, `Visual-regression bench — diff annotated PDFs against pixel goldens.  Plan Maest`, `Return the fraction of pixels (0..100) that differ between the     two RGB image` (+203 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **9 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `serve()` connect `Community 39` to `Community 3`, `Community 53`, `Community 15`?**
  _High betweenness centrality (0.281) - this node is a cross-community bridge._
- **Why does `IaServiceServicer` connect `Community 53` to `Community 3`, `Community 39`, `Community 48`, `Community 21`, `Community 26`?**
  _High betweenness centrality (0.269) - this node is a cross-community bridge._
- **Why does `main()` connect `Community 15` to `Community 42`, `Community 39`?**
  _High betweenness centrality (0.260) - this node is a cross-community bridge._
- **Are the 10 inferred relationships involving `OllamaClient` (e.g. with `HealthServicer` and `IaServiceServicer`) actually correct?**
  _`OllamaClient` has 10 INFERRED edges - model-reasoned connections that need verification._
- **Are the 8 inferred relationships involving `IaServiceServicer` (e.g. with `OllamaClient` and `OllamaError`) actually correct?**
  _`IaServiceServicer` has 8 INFERRED edges - model-reasoned connections that need verification._
- **Are the 10 inferred relationships involving `parse()` (e.g. with `main()` and `.open()`) actually correct?**
  _`parse()` has 10 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Throughput benchmark — pages/second and GB/hour over the corpus.  Plan Maestro §`, `Best-effort page count without pulling pypdf. Reads the file and     counts `/Ty`, `Fidelity benchmark — strata-reader vs opendataloader-pdf.  Plan Maestro §15.T10.` to the rest of the system?**
  _208 weakly-connected nodes found - possible documentation gaps or missing edges._