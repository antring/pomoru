use chrono::{Duration, Utc};
use std::{thread, time};

fn main() {
    let countdown_time = 10;
    let start_time = Utc::now();

    while Utc::now() < start_time + Duration::seconds(countdown_time) {
        thread::sleep(time::Duration::from_secs(1));
        println!("{} seconds remaining", countdown_time - (Utc::now() - start_time).num_seconds());
    }
}
