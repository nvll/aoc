use failure::Error;
use std::fs;

// Why didn't I just use regex?
fn get_rectangle(line: &str) -> (u32, u32, u32, u32) {
    let v = line
        .split(&[' ', ',', ':', 'x'][..])
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    (v[0], v[1], v[2], v[3])
}

const LENGTH: usize = 1000;
fn main() -> Result<(), Error> {
    let mut fabric = vec![[0; LENGTH]; LENGTH];
    let mut overlapping = 0;
    let buf = fs::read_to_string("input.txt")?;

    for line in buf.lines() {
        let (left, top, width, height) = get_rectangle(line);
        for x in left..left + width {
            for y in top..top + height {
                fabric[x as usize][y as usize] += 1;
                if fabric[x as usize][y as usize] == 2 {
                    // Count it on the pass it reaches 2, then not again
                    overlapping += 1;
                }
            }
        }
    }

    println!("overlapping: {}", overlapping);
    Ok(())
}
