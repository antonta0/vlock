# Changelog

## [0.2.3] - 2025-07-13

### Changed

-   Allow no-std usage of the crate. Defaults to `std`.

### Fixed

-   Wrap locking for writes into a RAII guard. Previously, if a closure passed
    to `compare_update` or `update` panicked while the write lock was held, the
    lock was never released, permanently deadlocking all future writes calls.
    Readers were unaffected.

### Performance

-   Drop a few inlines on generic and initialization functions.

## [0.2.2] - 2024-12-06

### Fixed

-   Fix potential data race when accessing new version by synchronizing access
    to the offset pointing to the current version, such that all mutations to
    the new version happen before subsequent reads. This race was found by miri.

## [0.2.1] - 2024-03-21

Update crate metadata.

## [0.2.0] - 2023-09-04

### Added

-   Add new public API functions. `compare_update` for comparing the value using
    a predicate under the lock. Two complementary `compare_update_default` and
    `update_default` functions for initializing the version with the `Default`
    trait.

## [0.1.0] - 2023-09-02

Initial release of `vlock` crate.
