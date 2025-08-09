use std::sync::{Condvar, Mutex};

/// A `LockLatch` starts as false and eventually becomes true. You can block
/// until it becomes true.
pub struct LockLatch {
    m: Mutex<bool>,
    v: Condvar,
}

impl LockLatch {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            m: Mutex::new(false),
            v: Condvar::new(),
        }
    }

    /// Block until latch is set.
    #[inline(always)]
    pub fn wait(&self) {
        let mut guard = self.m.lock().unwrap();
        while !*guard {
            guard = self.v.wait(guard).unwrap();
        }
    }

    // Sets the lock to true and notifies any threads waiting on it.
    #[inline(always)]
    pub fn set(&self) {
        *self.m.lock().unwrap() = true;
        self.v.notify_all();
    }

    // Resets the value of the lock
    #[inline(always)]
    pub fn reset(&self) {
        *self.m.lock().unwrap() = false;
        self.v.notify_all();
    }
}
