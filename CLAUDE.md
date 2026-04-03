# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build
cargo run
cargo test
cargo fmt
cargo clippy --all-targets --all-features
```

Run a single test by name:
```bash
cargo test <test_name>
```

The binary must be run from a directory that contains a `kanzlei/` subdirectory — it looks for that directory relative to the working directory, not the repo root.

## Architecture

`archivar` is a Rust CLI tool that analyzes a fixed archive directory structure. It currently only analyzes and classifies directories — archiving logic is not yet implemented. The SQLite database (`archivar.db`) is opened but not yet used for analysis.

**Call chain:**

`main` → `archive_top_dir` → `archive_year_dir`

- `main.rs`: Parses CLI args (`--dry-run`), opens `archivar.db`, scans the CWD for a `kanzlei/` directory, then calls `archive_top_dir`.
- `archive_top_dir.rs`: Iterates `kanzlei/` and dispatches to `archive_year_dir` for any subdirectory whose name is exactly two digits (e.g. `24`, `25`).
- `archive_year_dir.rs`: Iterates a year directory and reports the age classification for any subdirectory whose name starts with exactly three digits (e.g. `123 Mandant A`). Age is determined by the most recent modification time of any file inside that subdirectory (not the directory itself).

**Age bucketing special rule:** For the `>6 years` and `>8 years` buckets, the cutoff is measured from January 1st of the year *after* the last modification, not from the modification date itself. This logic lives in `age_bucket_from_modification_time`.
