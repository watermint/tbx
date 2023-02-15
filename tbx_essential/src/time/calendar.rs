use std::borrow::Cow;

/// RFC 3339 Calendar (Gregorian calendar).
/// <https://www.ietf.org/rfc/rfc3339.txt>
/// <https://en.wikipedia.org/wiki/ISO_8601>
pub trait Calendar {
    /// Year.
    fn year(&self) -> u64;

    /// Numerical month of year from 1 (January) to 12 (December).
    fn month(&self) -> u64;

    /// Day of month from 1 to 31
    fn day(&self) -> u64;

    /// Hour of day in 24-hour from 0 to 23.
    fn hour(&self) -> u64;

    /// Minute of hour from 0 to 59.
    fn minute(&self) -> u64;

    /// Second of minute from 0 to 59.
    /// If the system supports leap seconds the range may varies 0-58 (subtraction) or 0-60 (addition).
    fn second(&self) -> u64;

    /// The offset from UTC in seconds.
    /// For example, Japan has an offset of +09:00, which is 32,400 seconds.
    fn offset_seconds(&self) -> u64;

    /// Date, time & timezone offset in RFC 3339 format like `YYYY-MM-DDThh:mm:ss+hh:mm` or
    /// `YYYY-MM-DDThh:mm:ssZ` for zero offset.
    fn to_rfc3339_date_time_offset<'a>(&self) -> Cow<'a, str>;

    /// Date, time & timezone offset in RFC 3339 format like `YYYY-MM-DDThh:mm:ss+hh:mm`.
    /// This function always return `+hh:mm` offset even for zero offset.
    fn to_rfc3339_date_time_num_offset<'a>(&self) -> Cow<'a, str>;

    /// Date & time in RFC 3339 format like `YYYY-MM-DDThh:mm:ss`.
    fn to_rfc3339_date_time<'a>(&self) -> Cow<'a, str>;

    /// Date in RFC 3339 format like `YYYY-MM-DD`.
    fn to_rfc3339_date<'a>(&self) -> Cow<'a, str>;

    /// Time in RFC 3339 format like `hh:mm:ss`.
    fn to_rfc3339_time<'a>(&self) -> Cow<'a, str>;

    /// Timezone offset in RFC 3339 format like `+hh:mm` or `Z` for zero offset.
    fn to_rfc3339_offset<'a>(&self) -> Cow<'a, str>;

    /// Timezone offset in RFC 3339 format like `+hh:mm`.
    /// This function always returns `+hh:mm` even for zero offset.
    fn to_rfc3339_num_offset<'a>(&self) -> Cow<'a, str>;
}
