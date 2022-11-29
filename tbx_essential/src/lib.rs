pub mod fs;
pub mod text;

pub fn version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0")
}
