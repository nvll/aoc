use failure::Error;
use std::fs;

fn diff(first: &str, second: &str) -> Option<String> {
    let mut s = String::new();
    for (c1, c2) in first.chars().zip(second.chars()) {
        if c1 == c2 {
            s.push(c1);
        }
    }

    if s.len() == first.len() - 1 {
        return Some(s);
    }
    None
}

fn main() -> Result<(), Error> {
    let buf = fs::read_to_string("input.txt")?;
    let mut box_ids: Vec<&str> = buf.lines().collect();

    box_ids.sort();
    for i in 0..box_ids.len() - 1 {
        match diff(box_ids[i], box_ids[i + 1]) {
            Some(x) => {
                println!("{}", x);
                break;
            }
            _ => (),
        }
    }

    Ok(())
}
