use env_logger;
use failure::Error;
use log::trace;
use std::fs;

fn main() -> Result<(), Error> {
    env_logger::init();

    let buf = fs::read_to_string("input.txt")?;
    let mut twos = 0;
    let mut threes = 0;

    for line in buf.lines() {
        trace!("line '{}'", line);
        let mut counts = vec![0; 26]; // create a table for our letters, cheaper than a HashMap
        for c in line.chars() {
            let i: usize = (c as usize) - 97;
            counts[i] += 1;
        }

        for e in &counts {
            if *e == 2 {
                trace!("'{}' has twos", line);
                twos += 1;
                break;
            }
        }

        for e in &counts {
            if *e == 3 {
                trace!("'{}' has threes", line);
                threes += 1;
                break;
            }
        }
    }

    println!("checksum: {}", twos * threes);

    Ok(())
}
