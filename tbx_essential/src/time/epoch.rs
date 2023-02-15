pub trait Epoch {
    /// Number of seconds from the epoch (1970-01-01T00:00:00Z).
    fn epoch_second(&self) -> u128;

    /// Number of seconds from the epoch (1970-01-01T00:00:00Z) includes fractional (nano-sec) part.
    fn epoch_second_as_f32(&self) -> f32;

    /// Number of seconds from the epoch (1970-01-01T00:00:00Z) includes factional (nano-sec) part.
    fn epoch_second_as_f64(&self) -> f64;

    /// Number of milli-seconds from the epoch (1970-01-01T00:00:00Z).
    fn epoch_millis(&self) -> u128;

    /// Number of micro-seconds from the epoch (1970-01-01T00:00:00Z).
    fn epoch_micros(&self) -> u128;

    /// Number of nano-seconds from the epoch (1970-01-01T00:00:00Z).
    fn epoch_nanos(&self) -> u128;
}

