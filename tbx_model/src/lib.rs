use tbx_essential::text::version::semantic::Version;

/// Returns version of `tbx_model` module.
pub fn version<'a>() -> Version<'a> {
    match option_env!("CARGO_PKG_VERSION") {
        None => Version::zero(),
        Some(v) => Version::parse_or_zero(v),
    }
}
