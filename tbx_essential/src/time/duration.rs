use std::time::Duration as StdTimeDuration;
use crate::time::error::TimeError;

/// Represent a span of time.
/// This implementation is the wrapper of [`std::time::Duration`].
pub struct Duration {
    d: StdTimeDuration,
}

pub trait Measure {
}
