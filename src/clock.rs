use std::sync::RwLock;
use std::thread;

use time;

pub trait Clock {
    fn now(&self) -> time::Timespec;
    fn sleep_ms(&self, ms: u32);
}

pub struct RealClock;

unsafe impl Sync for RealClock{}

impl Clock for RealClock {
    fn now(&self) -> time::Timespec {
        time::now().to_timespec()
    }

    fn sleep_ms(&self, ms: u32) {
        thread::sleep_ms(ms)
    }
}

pub struct TestClock {
    inner: RwLock<time::Timespec>
}

impl TestClock {
    pub fn new() -> TestClock {
        TestClock {
            inner: RwLock::new(time::Timespec{
                sec: 0,
                nsec: 0,
            })
        }
    }
}

impl Clock for TestClock {
    fn now(&self) -> time::Timespec {
        let inner = self.inner.read().unwrap();
        *inner
    }

    fn sleep_ms(&self, ms: u32) {
        let mut inner = self.inner.write().unwrap();
        let ns = (ms % 1e6 as u32) * 1e6 as u32;
        inner.nsec += ns as i32;
        if inner.nsec > 1e9 as i32 {
            inner.sec += (inner.nsec / 1e9 as i32) as i64;
            inner.nsec = (inner.nsec % 1e9 as i32) as i32;
        }
    }
}
