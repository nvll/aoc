use failure::Error;
use std::collections::HashMap;
use std::fs;

fn read_input() -> Result<(String, HashMap<String, char>), Error> {
    let input = fs::read_to_string("input.txt")?;
    let initial = &input.lines().nth(0).unwrap()[15..];
    let mut map = HashMap::new();
    for line in input.lines().skip(2) {
        let v = line.trim().split(" => ").collect::<Vec<&str>>();
        map.insert(v[0].to_string(), v[1].chars().next().unwrap());
    }

    Ok((initial.to_string(), map))
}

fn main() -> Result<(), Error> {
    let (initial, map) = read_input()?;

    Ok(())
}
