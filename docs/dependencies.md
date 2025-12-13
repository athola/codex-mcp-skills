# Dependency Monitoring

This document tracks important dependency considerations and monitoring tasks for the skrills project.

## Dependency: `getrandom` Consolidation

**Status**: Monitoring for v0.4.0
**Impact**: Reduce binary size by ~50KB
**Current State** (as of 2025-12-03):

- rustls version: `0.23.35` (latest stable)
- getrandom versions in tree:
  - `getrandom@0.3.4`: Used by tempfile → dialoguer, skrills crates
  - `getrandom@0.2.16`: No normal dependencies (target-specific only)
- rustls does NOT directly depend on getrandom

### Monitoring Plan

Track the following for `getrandom` consolidation:

1. **Monitor rustls releases**: Check if newer versions consolidate to single getrandom version
2. **Check hyper-rustls updates**: Version 0.27.7 currently in use
3. **Review tokio-rustls**: Version 0.26.4 currently in use
4. **Binary size impact**: Measure actual size reduction when consolidation occurs

### Action Items

- [ ] Review rustls 0.24+ changelog when available for getrandom consolidation
- [ ] Test binary size before/after any rustls ecosystem updates
- [ ] Update this document when consolidation is achieved

## Monitoring Commands

```bash
# Check current getrandom versions
cargo tree -i getrandom

# Check rustls version and dependencies
cargo tree -p rustls

# Check binary size
ls -lh target/release/skrills
```
