use std::cmp::Ordering;
use std::num::ParseIntError;

pub fn cmp_pre_release(x: &str, y: &str) -> Ordering {
    let vx: Result<i64, ParseIntError> = x.parse();
    let vy: Result<i64, ParseIntError> = y.parse();
    match (vx, vy) {
        (Ok(nx), Ok(ny)) => nx.cmp(&ny),
        (Ok(_nx), Err(_e)) => Ordering::Less,
        (Err(_e), Ok(_ny)) => Ordering::Greater,
        _ => x.cmp(y),
    }
}
