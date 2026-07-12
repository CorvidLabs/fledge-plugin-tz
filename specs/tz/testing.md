---
spec: tz.spec.md
---

## Test Plan

- Validate supported aliases and explicit IANA zones.
- Validate supported time forms and invalid input failures.
- Validate add/remove persistence and idempotency against isolated state.

- Run `cargo fmt --check`, Clippy with warnings denied, and `cargo test`.
- Build the release binary and verify its root and subcommand help without network access.
