# Repository Guidelines

## Project Structure & Module Organization
This workspace-level `Cargo.toml` only lists the `core` crate so all commands run from the repo root. Search logic and the public APIs (`search`, `FileInput`, `MatchResult`) live in `core/src/lib.rs`. Internal fixtures stay beside that file under inline `#[cfg(test)]` blocks; prefer adding heavier end-to-end suites under `core/tests/` if you need to execute `cargo test --test ...`. Build output lands in `target/`; keep it ignored. Use `README.md` for behavior context and update it whenever APIs change.

## Build, Test, and Development Commands
- `cargo check -p simple_find_core` validates syntax and signatures quickly.
- `cargo fmt` / `cargo fmt -- --check` keep formatting consistent; the `--check` variant fails CI on drift.
- `cargo clippy -p simple_find_core -- -D warnings` enforces lints so the public API stays tidy.
- `cargo test -p simple_find_core` runs unit tests in `core/src/lib.rs` plus any future integration suites.
- `./test-core.sh` runs core crate tests using a convenience script (equivalent to the above).
- `cargo build --release -p simple_find_core` emits optimized artifacts in `target/release/` for embedding elsewhere.
- `cd wasm && wasm-pack build --target web` builds WebAssembly bindings for web browsers (outputs to `wasm/pkg/`).
- `cd wasm && wasm-pack build --target nodejs` builds WebAssembly bindings for Node.js environment.
- `./build-wasm.sh [target]` builds WebAssembly bindings using a convenience script (default target: `web`).
- `cd wasm && wasm-pack test --node` runs WebAssembly bindings tests in Node.js environment.
- `./test-wasm.sh` runs WebAssembly bindings tests using a convenience script (equivalent to the above).

## Coding Style & Naming Conventions
Follow default `rustfmt`: 4-space indentation, trailing commas on multi-line literals, and ~100-character lines. Public types stay `CamelCase` (`FileInput`), functions and methods stay `snake_case` (`search`, `case_sensitive`). Prefer descriptive booleans over abbreviations. Document non-obvious logic with short comments only when behavior would surprise readers.

## Testing Guidelines
Unit names should read like phrases (`test_case_insensitive_search`) and cover case-sensitive and insensitive modes, invalid regexes, and empty files. Keep new tests colocated with the logic they exercise until they span modules, at which point move them into `core/tests/`. Run `cargo test` locally before pushing and attach failing seeds or sample inputs to PR discussions. There is no coverage gate, but exercise error paths whenever you extend the API.

## Commit & Pull Request Guidelines
Match the existing history of short, imperative subjects (e.g., `リファクタとテスト`) under 50 characters, optionally followed by a detailed body. Reference issues as `#123` when relevant. Each PR should include a problem statement, a bullet list of changes, test evidence (`cargo test` output or screenshots if UI-adjacent), and callouts for follow-up work or roll-back plans. Keep scopes tight so reviewers can reason about search behavior quickly.

## Security & Configuration Tips
Ensure regex patterns supplied by higher layers are sanitized; this crate assumes trusted input and surfaces `regex` compilation errors. Never commit `target/` artifacts or ad-hoc binaries such as `core/test_regex`; treat them as local helpers only.
