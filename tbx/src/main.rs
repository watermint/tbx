use tbx_essential;
use tbx_essential::text2::message;

fn main() {
    println!("Hello, {}", message::hello());
    println!("Calc {}", tbx_essential::essential_add(10, 20))
}
