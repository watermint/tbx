use crate::text::version::semantic::{package_version, Version};

pub mod fs;
pub mod number;
pub mod text;
pub mod time;

/// Returns version of `tbx_essential` module.
pub fn version<'a>() -> Version<'a> { package_version(option_env!("CARGO_PKG_VERSION")) }
