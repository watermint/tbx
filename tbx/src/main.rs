use tbx_essential;
use tbx_essential::text::essential::StringEssential;

fn version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0")
}

fn main() {
    let v = version();
    let w = v.substring(2, 3).unwrap();
    println!(
        "tbx version {}, essential {:?}",
        w,
//        tbx_essential::version()
        "000012".parse::<i64>(),
    );
}
