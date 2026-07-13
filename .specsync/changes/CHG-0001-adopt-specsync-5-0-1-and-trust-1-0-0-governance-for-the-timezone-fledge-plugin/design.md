---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-timezone-fledge-plugin
artifact: design
---

# Design

Keep existing CI, release, and Pages workflows intact. Add a separate job named `trust` pinned to immutable Trust 1.0.0. The Trust lifecycle delegates to a Fledge lane that runs formatting, Clippy, unit tests, a release build, and help smoke checks. Contract coverage is blocking at 100 percent, risk is blocking, provenance is progressive, and Trust-managed Atlas is disabled because Pages remains independent.
