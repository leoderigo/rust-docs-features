use std::{thread::sleep};
use std::time;
use testing_the_docs::start;

fn main() {
    start();
    let five_seconds = time::Duration::from_millis(5000);
    sleep(five_seconds);
}
