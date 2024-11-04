use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub(crate) struct IdGenerator {
    id: usize,
}

impl IdGenerator {
    pub(crate) fn new() -> Self {
        let id = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
        IdGenerator { id }
    }

    pub(crate) fn get_id(&self) -> usize {
        self.id
    }
}
