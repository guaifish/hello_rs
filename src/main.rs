use bumpalo::{boxed::Box, Bump};
use std::sync::atomic::{AtomicUsize, Ordering};

static NUM_DROPPED: AtomicUsize = AtomicUsize::new(0);

struct CountDrops;

impl Drop for CountDrops {
    fn drop(&mut self) {
        NUM_DROPPED.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    // Create a new bump arena.
    let bump = Bump::new();

    // Create a `CountDrops` inside the bump arena.
    let c = Box::new_in(CountDrops, &bump);

    // No `CountDrops` have been dropped yet.
    assert_eq!(NUM_DROPPED.load(Ordering::SeqCst), 0);

    // Drop our `Box<CountDrops>`.
    drop(c);

    // Its `Drop` implementation was run, and so `NUM_DROPS` has been incremented.
    assert_eq!(NUM_DROPPED.load(Ordering::SeqCst), 1);
}
