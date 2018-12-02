use env_logger;
use log::trace;
use std::collections::HashMap;
use std::fs;

fn main() {
    env_logger::init();
    let mut frequencies: HashMap<i32, u32> = HashMap::new();
    let buffer = fs::read_to_string("input.txt").expect("failed to read file");
    let mut frequency = 0;
    let mut found = false;

    while !found {
        for line in buffer.trim().split("\n") {
            let val: i32 = line.parse().unwrap();
            frequency += val;
            let counter = frequencies.entry(frequency).or_insert(0);
            *counter += 1;
            trace!(
                "val: {:<8}\tfrequency: {:<8}\tcount: {}",
                val,
                frequency,
                *counter
            );
            if *counter == 2 {
                found = true;
                break;
            }
        }
    }
    println!("frequency: {}", frequency);
}
