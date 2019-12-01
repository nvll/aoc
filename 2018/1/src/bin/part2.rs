use env_logger;
use failure::Error;
use log::trace;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Error> {
    env_logger::init();
    let mut frequencies: HashMap<i32, u32> = HashMap::new();
    let mut frequency = 0;
    let mut found = false;
    let buffer = fs::read_to_string("input.txt")?;

    while !found {
        for line in buffer.lines() {
            let val: i32 = line.parse()?;
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
    Ok(())
}
