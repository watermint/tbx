use crate::text::version::semantic::Version;

pub mod fs;
pub mod text;
pub mod number;

/// Returns version of `tbx_essential` module.
pub fn version<'a>() -> Version<'a> {
    match option_env!("CARGO_PKG_VERSION") {
        None => Version::zero(),
        Some(v) => Version::parse_or_zero(v),
    }
}
