---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-timezone-fledge-plugin
artifact: testing
---

# Testing

- `specsync check --strict --require-coverage 100 --force`
- `specsync agents status`
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test` (eight tests)
- `cargo build --release`
- Root and convert-command help smoke checks
- `fledge trust doctor`
