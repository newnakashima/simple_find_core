# simple_find_core

## Overview
`simple_find_core` is a lightweight Rust library that powers text search across many files. It exposes a single `search` function that accepts a regular-expression pattern, an in-memory collection of files, and a case-sensitivity flag, returning rich match metadata (path, line, column, and full line text). The crate is designed to be embedded inside GUI tools, CLIs, or background services that need fast, dependable regex results without bringing in an entire editor engine.

## Features
- Regex search built on top of the battle-tested [`regex`](https://docs.rs/regex) crate.
- Case-sensitive and case-insensitive modes via a simple boolean switch.
- Detailed match reporting so frontends can highlight exact positions.
- Thorough unit tests covering invalid patterns, multi-line files, and edge cases like empty inputs.

## Getting Started
1. Install the latest stable Rust toolchain (Edition 2024 requires Rust 1.75 or newer).
2. Add this crate as a dependency or build it directly:
   ```bash
   cargo build
   ```
3. Run the test suite to verify your environment:
   ```bash
   cargo test
   ```

## Usage
```rust
use simple_find_core::{search, FileInput};

fn main() {
    let files = vec![
        FileInput { path: "notes.txt".into(), content: "Hello, World!".into() },
        FileInput { path: "readme.md".into(), content: "Hello, Rustaceans!".into() },
    ];

    let matches = search("Hello", &files, true).expect("regex should compile");
    for m in matches {
        println!("{}:{}:{} -> {}", m.path, m.line, m.column, m.line_text);
    }
}
```
When searching without case sensitivity, pass `false` as the third argument; invalid regex patterns return an `Err` with a descriptive message that should be surfaced to users.

## Development Workflow
- Format code with `cargo fmt` before committing.
- Keep lints clean using `cargo clippy -- -D warnings`.
- Run `cargo test` to execute the inline unit suite and any future integration tests under `tests/`.
- Benchmarks or profiling binaries can be added inside `benches/` or `examples/` if you need to measure performance before integrating into a UI.

## Contributing
Please open an issue describing the problem you intend to solve, then submit focused pull requests with `cargo fmt`, `cargo clippy`, and `cargo test` passing. Include a short summary, screenshots or logs if applicable, and test output to help reviewers validate the change quickly. For detailed contributor expectations see `AGENTS.md`.
