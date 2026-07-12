---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-timezone-fledge-plugin
artifact: context
---

# Context

The timezone plugin predates verified SDD governance. It already has a Rust implementation, unit tests, a three-operating-system CI matrix, release packaging, and independent Pages publication. This migration adds one unified Trust gate without replacing those specialized workflows.
