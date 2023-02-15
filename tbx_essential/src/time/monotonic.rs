use std::time::Instant as StdTimeInstant;
use crate::time::duration::Duration;

/// This Instant is the monotonically non-decreasing clock to measure the elapsed time.
/// Opaque and useful only with [`time::duration::Duration`]
pub struct Instant {
    t: StdTimeInstant,
}

impl Instant {
    pub fn elapsed() -> Duration {
        todo!()
    }
}