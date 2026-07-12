use std::{sync::Arc, thread, time};

use vlock::VLock;

fn main() {
    let lock = Arc::new(VLock::<String, 4>::new(String::from("hi there!")));
    let lock_clone = Arc::clone(&lock);
    let t = thread::spawn(move || {
        for _ in 0..5 {
            println!("{}", *lock_clone.read());
            thread::sleep(time::Duration::from_millis(1));
        }
        lock_clone.update(
            |_, value| {
                value.clear();
                value.push_str("bye!");
            },
            String::new,
        );
    });
    thread::sleep(time::Duration::from_millis(2));
    lock.update(
        |_, value| {
            value.clear();
            value.push_str("here's some text for you");
        },
        String::new,
    );
    if let Err(err) = t.join() {
        println!("thread has failed: {err:?}");
    }
    assert_eq!(*lock.read(), "bye!");
}
