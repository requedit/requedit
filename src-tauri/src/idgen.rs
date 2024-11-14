use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn gen_id() -> usize {
    // fetch add return the original value before the addition operation.
    let id: usize = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
    id
}


#[test]
fn test_gen_id() {
    let initial_value = GLOBAL_COUNTER.load(Ordering::Relaxed);
    let id1 = gen_id();
    assert_eq!(id1, initial_value);

    let id2 = gen_id();
    assert_eq!(id2, initial_value + 1);

    let id3 = gen_id();
    assert_eq!(id3, initial_value + 2);

    let id4 = gen_id();
    assert_eq!(id4, initial_value + 3);
}
