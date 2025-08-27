# Repository Guidelines

## Project Structure & Modules
- `src/lib.rs`: Library entry; exports `modules`, `dataelem`, `dicts`.
- `src/main.rs`: CLI example; runs `read_dicom` on `test_data/*.dcm`.
- `src/modules/io/`: DICOM Part 10 reader (transfer syntax, headers, values).
- `src/dataset.rs`: Dataset container types.
- `src/dicts.rs`: Auto-generated; do not edit manually.
- `src/dataelem.rs`: DataElement inside Dataset.
- `tests/`: Integration tests (e.g., `integration_test.rs`).
- `test_data/`: Sample anonymized DICOM files; keep small and non-PHI.

## Build, Test, and Dev Commands
- `cargo build`: Compile library and binary (debug).
- `cargo run`: Run the example CLI; edit path in `src/main.rs` or pass args if extended.
- `cargo test`: Run unit/integration tests.
- `cargo fmt --all`: Format with rustfmt.
- `cargo clippy --all-targets -- -D warnings`: Lint with strictness.

## Coding Style & Conventions
- Rust 2024 edition; 4-space indent; max line length ~100 where practical.
- Naming: modules/files `snake_case`; types `PascalCase`; functions/vars `snake_case`; constants `SCREAMING_SNAKE_CASE`.
- Prefer small, focused modules; keep I/O parsing in `src/modules/io` and data model in `dataset`/`dataelem`.
- Do not hand-edit generated files (`src/dataelem.rs`, dictionary tables). Update the generator upstream instead.

## Testing Guidelines
- Framework: Rust built-in `#[test]` and integration tests under `tests/`.
- Add unit tests next to code where useful; name tests descriptively (e.g., `parses_transfer_syntax_explicit_le`).
- Run `cargo test` locally; keep tests deterministic and file-size friendly. Use fixtures under `test_data/`.

## Commit & Pull Request Guidelines
- Commits: clear, imperative subject (≤72 chars), body detailing rationale and scope (e.g., "io: handle undefined length SQ").
- Prefer small, logically grouped commits. Reference issues like `Fixes #123` when applicable.
- PRs: include summary, screenshots/logs when parsing changes affect output, and reproduction steps. Link related issues and note any breaking changes.

## Security & Data Handling
- Do not commit PHI or large medical images. Keep samples anonymized and minimal.
- Be cautious with unbounded reads; for large files prefer streaming over buffering whole files when extending I/O.
