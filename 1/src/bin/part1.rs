use env_logger;
use failure::Error;
use log::trace;
use std::fs;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut total: i32 = 0;
    for line in fs::read_to_string("input.txt")?.lines() {
        let val: i32 = line.parse()?;
        total += val;
        trace!("val: {}", val);
    }

    println!("{}", total);
    Ok(())
}
