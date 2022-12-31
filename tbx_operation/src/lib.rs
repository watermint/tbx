use tbx_essential::text::version::semantic;
use tbx_essential::text::version::semantic::Version;

/// Returns version of `tbx_operation` module.
pub fn version<'a>() -> Version<'a> {
    semantic::package_version(option_env!("CARGO_PKG_VERSION"))
}
