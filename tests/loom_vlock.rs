use loom::sync::Arc;
use loom::thread;

/// A reader racing a single update should observe either the pre- or
/// post-update value — never a torn read or garbage.
#[test]
fn read_during_update_sees_a_consistent_value() {
    loom::model(|| {
        let lock = Arc::new(vlock::VLock::<usize, 2>::new(10));

        let reader_lock = Arc::clone(&lock);
        let reader = thread::spawn(move || {
            let value = *reader_lock.read();
            assert!(value == 10 || value == 42, "torn/garbage read: {value}");
        });

        lock.update(|_, v| *v = 42, || 0);

        reader.join().unwrap();
    });
}

// With N=2, a second update must reclaim the slot the first reader is
// still (potentially) holding — this exercises acquire()'s refcount
// check and the release-sequence reasoning behind it directly.
#[test]
fn version_reuse_does_not_race_with_lagging_reader() {
    loom::model(|| {
        let lock = Arc::new(vlock::VLock::<usize, 2>::new(1));

        let reader_lock = Arc::clone(&lock);
        let reader = thread::spawn(move || {
            let r = reader_lock.read();
            let v = *r;
            assert!(matches!(v, 1 | 2 | 3), "garbage value: {v}");
        });

        lock.update(|_, v| *v = 2, || 0);
        lock.update(|_, v| *v = 3, || 0);

        reader.join().unwrap();
        assert!(matches!(*lock.read(), 3));
    });
}
