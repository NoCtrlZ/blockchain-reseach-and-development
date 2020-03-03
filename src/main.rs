// mod core;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let unixtime = now.duration_since(UNIX_EPOCH).expect("invalid time");
    println!("{:?}", unixtime.as_secs().type_name_of());
}
