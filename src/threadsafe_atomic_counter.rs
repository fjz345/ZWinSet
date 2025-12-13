use std::sync::{
    LazyLock,
    atomic::{AtomicUsize, Ordering},
};

static THREAD_COUNTER: LazyLock<AtomicUsize> = LazyLock::new(|| AtomicUsize::new(0));

pub struct ThreadsafeAtomicCounter {}

impl ThreadsafeAtomicCounter {
    pub fn new() -> Self {
        THREAD_COUNTER.fetch_add(1, Ordering::SeqCst); // increment when created
        ThreadsafeAtomicCounter {}
    }

    pub fn count(&self) -> usize {
        THREAD_COUNTER.load(Ordering::SeqCst)
    }

    pub fn peek_count() -> usize {
        THREAD_COUNTER.load(Ordering::SeqCst)
    }
}

impl Drop for ThreadsafeAtomicCounter {
    fn drop(&mut self) {
        THREAD_COUNTER.fetch_sub(1, Ordering::SeqCst); // decrement when dropped
    }
}
