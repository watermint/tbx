use tbx_essential;
use tbx_essential::text::version::semantic;
use tbx_essential::text::version::semantic::Version;

fn version<'a>() -> Version<'a> {
    semantic::package_version(option_env!("CARGO_PKG_VERSION"))
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
