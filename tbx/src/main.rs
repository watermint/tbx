use tbx_essential;
use tbx_essential::text::message;

fn version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0")
}

fn main() {
    println!(
        "tbx version {}, essential {}",
        version(),
        tbx_essential::version()
    );
}
