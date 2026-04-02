# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` contains the CLI entry point that scans the working directory and prepares archive folders. Shared logic lives in `src/archive_top_dir.rs` and is re-exported from `src/lib.rs`, so reusable code should stay in library modules rather than `main.rs`. `Cargo.toml` defines the crate and dependencies, and `target/` is Cargo build output and should not be edited by hand.

## Build, Test, and Development Commands
Use Cargo for all routine work:

- `cargo build` compiles the crate.
- `cargo run` runs the archive tool from the current directory.
- `cargo test` runs unit and integration tests.
- `cargo fmt` formats the codebase with `rustfmt`.
- `cargo clippy --all-targets --all-features` checks for common Rust issues before a PR.

Run commands from the repository root so relative path handling matches production behavior.

## Coding Style & Naming Conventions
Follow standard Rust formatting with 4-space indentation and keep `cargo fmt` output authoritative. Use `snake_case` for files, modules, functions, and variables, and `CamelCase` for types and traits. Prefer small functions with explicit `Result`-based error handling over panics. Keep CLI orchestration in `main.rs` and move archive logic into `src/` modules when it may be reused or tested independently.

## Testing Guidelines
There is no dedicated `tests/` directory yet, so add focused unit tests alongside the code with `#[cfg(test)]` blocks. When behavior spans modules or filesystem interactions, add integration tests under `tests/`. Name tests after observable behavior, for example `creates_missing_archive_directories` or `rejects_missing_kanzlei_dir`. Run `cargo test` locally before opening a PR.

## Commit & Pull Request Guidelines
Recent commits use short, plain-language summaries such as `added walkdir` and `checking directories`. Keep commit messages concise, imperative, and focused on one change. For pull requests, include a brief description of the behavior change, note any directory or filesystem assumptions, and paste the relevant verification commands you ran. Include sample output when CLI behavior changes.

## Environment Notes
This tool works with real directories in the current working tree. Avoid hard-coded absolute paths, and document any new required folder names or archive rules in the PR that introduces them.
