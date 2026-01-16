# AGENTS.md

This document provides conventions and actionable commands for agents operating in the repository. It covers build, lint, test workflows, and code style guidelines, with explicit guidance for running single tests and common Rust tooling. It also captures any repository-specific rules (Cursor or Copilot) if present.


**1. Build, Test, Lint**

- Base commands
  - Build the project: `cargo build`.
  - Run all tests: `cargo test`.
  - Quick type-check without running tests: `cargo check`.
  - Lint with Clippy: `cargo clippy --all-targets --all-features -- -D warnings`.
  - Format code: `cargo fmt` (or `cargo fmt --all`).
  - Generate docs: `cargo doc --no-deps --open` (or omit `--open` to just build docs).
  - Run tests in release mode: `cargo test --release` (useful for performance-sensitive tests).
  - Run tests for a single crate in a workspace: `cargo test -p <cratename>`, for example `cargo test -p rusty-bybit`.
  - Verify dependencies for security: `cargo audit` (requires `cargo-audit` to be installed).

- Running a single test (typical patterns)
  - Exact test name: `cargo test -- --exact <TestName>`
  - Module-scoped test: `cargo test <module>::<TestName>`
  - If you know the crate, target a specific binary or library test: `cargo test -p rusty-bybit -- <TestName>`
  - Include output: `cargo test -- --nocapture` to show print statements in tests.
  - Example: if you have a test named `parsing_handles_empty_input` in module `parser`, run:
    - `cargo test parser::parsing_handles_empty_input --nocapture` or
    - `cargo test -- --exact parser::parsing_handles_empty_input`.

- Workspace checks
  - Ensure all tests pass across the workspace: `cargo test --workspace`.
  - Check a specific feature: `cargo test --features <feat>` or `--no-default-features`.

- CI-friendly routines
  - Run with verbose cargo output in CI-free mode: `CARGO_TERM_COLOR=always cargo test --verbose`.
  - Run tests with timeouts if the runner has limits; in local shell you can rely on your CI.

- Quick failure triage
  - After a failing test, run a targeted test with `--nocapture` and inspect logs.
  - Use `RUST_LOG=debug` to raise log verbosity when tests respect logging.

- Safety and reproducibility
  - Prefer `cargo fmt` and `cargo clippy` to catch issues early.
  - Avoid editing generated files manually; use `build.rs` or generator scripts if present.


**2. Code Style Guidelines**

- Language & edition
  - The project uses Rust 2024 edition (as per `edition = "2024"`). Follow modern Rust idioms accordingly.
  - Enable unstable or nightly features only when the project explicitly requires them and with approved changes.

- General principles
  - Be idiomatic: favor standard library facilities, avoid premature optimizations.
  - Prioritize correctness, readability, and maintainability over cleverness.
  - Write small, focused functions with a clear single responsibility.
  - Document public APIs with `///` comments; ensure examples compile if feasible.

- Imports and module layout
  - Group imports in this order: std, external crates, crate-local modules.
  - Each group separated by a blank line.
  - Use explicit imports rather than wildcard imports where practical.
  - Keep module boundaries small; prefer explicit `pub mod` declarations and private helpers.

- Formatting
  - Use `rustfmt` conventions: 4-space indentation, line width around 100-120 characters where possible.
  - Break long function signatures and chain calls across multiple lines with trailing closure braces aligned.
  - Use trailing commas for multi-line lists or parameters.
  - Run `cargo fmt` regularly and enforce via CI if possible.

- Naming conventions
  - Functions and variables: snake_case.
  - Structs, Enums, Traits: CamelCase.
  - Constants: ALL_CAPS or UPPER_SNAKE_CASE.
  - Modules: snake_case.
  - Async fns: suffix with `_async` when ambiguous.

- Types and errors
  - Prefer `Result<T, E>` for fallible operations; propagate with `?`.
  - Define small, explicit error enums for modules with `thiserror` or manually implement `std::error::Error` and `Display`.
  - Avoid panics in library code; use `unwrap` only in tests or during prototyping in `main` with clear intent.
  - When converting errors, provide context with `map_err`/`with_context` (via `anyhow` or `thiserror` if used).

- Error handling patterns
  - Propagate errors up the call stack; avoid swallowing errors.
  - Use `match` only when you need to handle specific cases; otherwise use `?` to propagate.
  - For IO and parsing, wrap errors with contextual messages: `Err(e).map_err(|e| MyError::IoError(format!("...: {}", e)))`.

- Documentation and comments
  - Public items must have `///` docs with examples when helpful.
  - Keep comments focused on why, not what; the code should explain what through naming.
  - Document module responsibilities at the top of each file with a brief paragraph.

- Testing guidelines
  - Unit tests live in `#[cfg(test)]` modules within the same file.
  - Integration tests live in the `tests/` directory.
  - Tests should be deterministic and fast; avoid external dependencies when possible.
  - Use `assert!`, `assert_eq!`, and descriptive messages.
  - Use `#[test]` annotations with clear naming; test function names should indicate intent.

- Performance considerations
  - Prefer avoiding allocations in hot paths; reuse buffers when practical.
  - Use profiling- or benchmarking-oriented setup only when requested.
  - Document any performance trade-offs in code comments.

- Security and resilience
  - Validate inputs early; fail fast with clear error messages.
  - Sanitize external inputs; avoid leaking secrets in logs.
  - Avoid unsafe blocks unless strictly necessary and well-justified.

- Tooling expectations
  - Use `cargo check`, `cargo fmt`, and `cargo clippy` during development.
  - Keep `CI` configurations aligned with local commands.


**3. Tests & Examples**

- Single-file tests
  - Place within the same module using `#[cfg(test)]` and `mod tests { ... }`.
  - Use `assert_eq!`, `assert_ne!`, and other standard macros.

- Integration tests
  - Create new file in `tests/`, named after the feature being tested.
  - Tests should exercise public APIs across modules.

- Example conventions
  - Public API: `pub fn do_something(...) -> Result<Output, MyError>`.
  - Internal helper: `fn helper(...) -> Result<Internal, MyError>`.

- Documentation tests
  - Include doctests in `///` examples when feasible to validate usage.


**4. Cursor Rules**

- Cursor rules: none detected in this repository (.cursor/rules not found).
- If rules are added later, merge them under this section with per-rule notes (path, trigger, and expected behavior).

**5. Copilot Rules**

- Copilot rules: none detected in this repository (no .github/copilot-instructions.md).
- If Copilot guidance exists, summarize key constraints here and apply them in code generation or edits.


**6. Additional Guidance**

- Versioning and commits
  - Write small, focused commits describing the intent, e.g., "refactor: apply idiomatic Rust imports".
  - Do not mix large structural changes in a single commit.

- Documentation
  - Update module docs and readme if APIs or usage patterns change.

- Validation steps after changes
  - Run `cargo fmt`.
  - Run `cargo clippy --all-targets --all-features -- -D warnings`.
  - Run a full test: `cargo test --workspace --all-features -- --nocapture`.

- Collaboration note
  - If a change affects public API, bump the version and add a changelog entry in a consistent manner.


This AGENTS.md is intended to support repeated, safe, and predictable edits by agents operating within this repository. If you would like any section adjusted or expanded, tell me and I’ll tailor it to your workflow.
