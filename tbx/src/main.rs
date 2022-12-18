use tbx_essential;
use tbx_essential::text::version::semantic::Version;


fn version<'a>() -> Version<'a> {
    match option_env!("CARGO_PKG_VERSION") {
        Some(v) => Version::parse_or_zero(v),
        _ => Version::zero(),
    }
}

fn main() {
    println!(
        "tbx version {}, essential {}, foundation {}, model {}, operation {}",
        version(),
        tbx_essential::version(),
        tbx_foundation::version(),
        tbx_model::version(),
        tbx_operation::version(),
    );
}
