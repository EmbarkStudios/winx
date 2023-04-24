use crate::cvt::cvt;
use std::io;

pub fn perf_counter_frequency() -> io::Result<u64> {
    unsafe {
        let mut frequency = 0;
        cvt(crate::bindings::QueryPerformanceFrequency(&mut frequency))?;
        Ok(frequency as u64)
    }
}
