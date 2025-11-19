# Repository Guidelines

## Project Structure & Module Organization
Source lives in `src/lib.rs`, exposing the `search`, `FileInput`, and `MatchResult` APIs used by any frontend. `Cargo.toml` declares the crate metadata and dependencies, while `Cargo.lock` pins versions for reproducible builds. Compilation artifacts land in `target/`; do not commit this directory. Unit tests currently sit inline under `#[cfg(test)]` in `src/lib.rs`. When adding broader scenarios, prefer creating integration suites under a new `tests/` directory so the core module stays focused.

## Build, Test, and Development Commands
- `cargo check` — fast validation of syntax and types before pushing.
- `cargo fmt` / `cargo fmt -- --check` — formats the tree with `rustfmt`; use the `--check` variant in CI or pre-commit hooks.
- `cargo clippy -- -D warnings` — runs the static analyzer and upgrades all warnings to errors to keep APIs tidy.
- `cargo test` — executes the inline unit tests as well as any future integration suites under `tests/`.
- `cargo build --release` — produces an optimized library in `target/release/` for embedding in other tools.

## Coding Style & Naming Conventions
Rust code follows the default `rustfmt` configuration: 4-space indentation, trailing commas for multi-line literals, and a ~100 character soft limit. Public items use `CamelCase` types and `snake_case` functions consistent with the standard library. Prefer expressive names such as `case_sensitive` or `regex_builder` over abbreviations. Run `cargo fmt` and `cargo clippy` before sending any change to ensure formatting and lint rules are satisfied.

## Testing Guidelines
Use `cargo test` locally before every PR. Unit test names should read like sentences (`test_case_insensitive_search`) and cover the happy path plus edge cases such as invalid regex syntax or empty files. For new behaviors, add focused unit tests inside the relevant module and create integration-level tests in `tests/` when multiple modules interact. There is no formal coverage gate, but aim to exercise new logic in both case-sensitive and case-insensitive modes so regressions are caught early.

## Commit & Pull Request Guidelines
Recent history shows concise commit subjects (e.g., `リファクタとテスト`), so continue using short, imperative summaries under 50 characters, optionally followed by a blank line and detail. Reference issues with `#123` when applicable. Pull requests should include: a short problem statement, a bullet list of changes, test evidence (`cargo test` output or screenshots if integrating with a UI), and callouts for any follow-up work. Keep PRs scoped so reviewers can validate behavior quickly.
