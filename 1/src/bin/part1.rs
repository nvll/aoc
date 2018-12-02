use env_logger;
use log::trace;
use std::fs;

fn main() {
    env_logger::init();

    let buffer = fs::read_to_string("input.txt").expect("failed to read file");
    let mut total: i32 = 0;
    for line in buffer.trim().split("\n") {
        let val: i32 = line.parse().unwrap();
        trace!("val: {}", val);
        total += val;
    }

    println!("{}", total);
}
