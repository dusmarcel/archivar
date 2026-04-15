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

The binary must be run from a directory that contains at least one of the top-level archive directories — it looks for them relative to the working directory, not the repo root.

## Architecture

`archivar` is a Rust CLI tool that analyzes a fixed archive directory structure, creates `.tar.xz` archives of old directories, and records them in a local SQLite database.

**CLI flags:**
- `-d` / `--dry-run`: prints what would be done without making changes
- `-r` / `--remove`: removes empty client directories

**Call chain:**

`main` → `archive_top_dir` → `archive_year_dir` → `create_archive`

- `main.rs`: Parses CLI args, opens/creates `archivar.db` (SQLite), then calls `archive_top_dir` for each of the top-level directories found in CWD: `kanzlei`, `ablage2`, `ablage4`, `ablage6`, `ablage8`.
- `archive_top_dir.rs`: Iterates a top-level directory and dispatches to `archive_year_dir` for any subdirectory whose name is exactly two digits (e.g. `24`, `25`).
- `archive_year_dir.rs`: Iterates a year directory and processes any subdirectory whose name starts with exactly three digits (e.g. `123 Mandant A`). Age is determined by the most recent modification time of any file inside that subdirectory (not the directory itself). Directories older than 2 years are archived; all entries are upserted into the SQLite `archive` table with `(year, no, change_time, hash)`. `hash` is the SHA-256 of the archive file; `NULL` for recent directories.
- `create_archive.rs`: Creates a `.tar.xz` archive of a given directory path into `$TMPDIR`, returns the open `File` handle for hashing.

**Age bucketing special rule:** For the `>6 years` and `>8 years` buckets, the cutoff is measured from January 1st of the year *after* the last modification, not from the modification date itself. This logic lives in `age_bucket` in `archive_year_dir.rs`.

**SQLite schema:**
```sql
CREATE TABLE archive (
    year INTEGER NOT NULL,
    no INTEGER NOT NULL,
    change_time REAL NOT NULL,       -- unix timestamp (seconds, float)
    hash BLOB UNIQUE CHECK (length(hash) = 32),  -- SHA-256, NULL if not archived
    PRIMARY KEY (year, no)
)
```
