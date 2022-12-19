use tbx_essential;
use tbx_essential::text::version::semantic;
use tbx_essential::text::version::semantic::Version;
use tbx_essential::number::random::{Generator, Random};

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

    let mut r = Random::new_thread_local();
    for _i in 0..100 {
        println!("bool {}, u8 {}, i32 {}, i64 {}, f32 {}, f64 {}",
                 r.next_bool(),
                 r.next_u8(),
                 r.next_i32(),
                 r.next_i64(),
                 r.next_f32(),
                 r.next_f64(),
        )
    }
}
