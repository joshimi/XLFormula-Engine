# Rust Project Guidelines

## General Rust Guidance (common to all projects)

- Prefer idiomatic Rust conventions.
- Never use `.unwrap()`, `.expect()`, `panic!()`, or any other function, method, or
  macro that unwinds the stack. Return `Result` with meaningful errors instead.
  - These **are** allowed in tests.
- Unless there are naming conflicts or you are building Rust macros, import the
  object instead of using a qualified path.
- Only include comments that explain **why** code does something, not how or what
  it does.
  - If a comment describes follow-up work, reference its Linear issue number.
- Avoid the `let _ = ...;` / `let _name = ...;` pattern — just call the function.
  Allowed only when required for lifetime behavior.
- Use concrete types that derive `serde::Serialize` / `serde::Deserialize` instead
  of raw JSON via `serde_json`. Use `serde_json::Value` only in unit tests or at
  genuinely dynamic, fixture, or interoperability boundaries, converting to typed
  data as early as practical.

## Dependencies

- Reuse existing workspace crates and utilities before adding a dependency.
- A crate with more than 100,000 downloads
  on crates.io may be added without asking; otherwise ask first.

## Post-change checks (all projects)

Run from the project directory after every code edit:

```sh
cargo +nightly fmt
cargo clippy -- -D warnings
```

## Commits and pull requests

- Keep commit messages to at most two sentences.
- If a Linear ticket is available, start the commit subject (and PR title) with the
  Linear issue key, e.g. `ENG-4456: Set up existing creed crate for Rust QA`.
- Include the Linear issue link in the PR body when the PR corresponds to an issue.
- Squash multiple commits belonging to the same Linear issue before pushing or when
  updating an open PR (prefer amend/rebase of fixups), unless they are genuinely
  separate reviewable units.

## Unit tests

- Do not add unit tests that merely mirror literals, field assignment, `Default`,
  trivial match arms, or the implementation line by line.
- Add a unit test when it exercises behavior that could plausibly regress without
  an obvious compile failure: non-trivial algorithms, subtle serialization
  semantics, date or money calculations, state transitions, authorization, or
  validation edge cases.
- Applies to inline `#[cfg(test)]` tests; integration/E2E suites are
  governed separately.

## Code review

- Library version compatibility: do not request compatibility code, polyfills,
  fallbacks, or workarounds for older library versions unless repository evidence
  (manifests, lockfiles, runtime constraints) shows the older version is currently
  in use or explicitly supported. Review against the resolved version otherwise.
