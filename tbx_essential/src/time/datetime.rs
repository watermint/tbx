use std::time::SystemTime;
use crate::time::duration::Duration;
use crate::time::error::TimeError;

/// Date-time in the ISO 8601 calendar system.
pub struct DateTime {
    t: SystemTime,
}

impl DateTime {
    pub fn elapsed() -> Result<Duration, TimeError> {
        todo!()
    }
}