---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-timezone-fledge-plugin
artifact: research
---

# Research

The source is a single Rust binary with eight unit tests. Its normal behavior is offline; the only persistent effect is a JSON zone list under the Fledge plugin data directory. Existing CI builds and tests on Linux, macOS, and Windows, while release automation packages four targets. SpecSync can measure the Rust source directly, so the existing policy can enforce 100 percent coverage.
