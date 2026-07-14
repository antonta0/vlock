# Changelog

## [0.2.4] - 2025-07-14

### Performance

-   Align `Data<T>` slots to cache line boundaries. Each version slot's
    `Data<T>` (the atomic refcount and value pair) was packed with no alignment
    guarantee, so multiple slots could land on the same cache line depending on
    allocator behavior — causing false sharing between logically-independent
    slots under concurrent access. This showed up as run-to-run inconsistent
    throughput and an unexplained dip at `N=4` in earlier benchmarking on Apple
    M4 chip:
    ```
    - Running with 4 writer threads and 10 reader threads
    - 10 iterations inside lock, 2 iterations outside lock
    - 10 seconds per test
    vlock::VLock<_, 2>   - [write]     33.312 kHz [read]  16222.295 kHz
    vlock::VLock<_, 4>   - [write]    334.894 kHz [read]   9011.289 kHz
    vlock::VLock<_, 8>   - [write]    758.125 kHz [read]  15123.731 kHz
    ```

    `#[repr(align(64))]` on `Data<T>` gives every slot its own cache line.
    No other primitive in the same benchmark run moved, isolating the
    effect to `VLock` specifically rather than measurement noise.

    Benchmark of 1 writer, 23 readers, x86_64, 12 cores / 24 vCPU AMD Ryzen 9
    5900X, pinned via `taskset` to 0-23:
    ```
                write (kHz)         read (kHz)
    N   before   after  delta   before    after  delta
    2    213.8   334.5   +56%    31711    37952   +20%
    4    672.4  1102.9   +64%    35111    39328   +12%
    8    747.9  1117.3   +49%    36166    39056    +8%
    ```

    Write throughput sees the larger gain, consistent with the writer's
    version-transfer step touching the same contended cache line as
    concurrent readers under the unaligned layout.

    Also observed on a different architecture, smaller effect size. Benchmark of
    1 writer, 11 readers, Apple M4 unpinned:
    ```
                write (kHz)          read (kHz)
    N   before   after  delta      before     after  delta
    2    43.4    41.3     -5%     14278.2   14791.2    +4%
    4   420.5   443.6     +5%     12433.3   14129.7   +14%
    8   595.5   650.3     +9%     15787.6   15732.8    ~0%
    ```

    `N=4` read throughput shows the largest gain, closing most of the gap
    that was previously visible as a dip relative to `N=2` and `N=8`. `N=2`
    write throughput moved within noise (-5%), consistent with a slot
    count small enough that false sharing had little room to occur in the
    first place.

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
