use failure::Error;
use lazy_static;
use regex::Regex;
use std::fs;

fn get_rectangle(line: &str) -> Result<(usize, u64, u64, u64, u64), Error> {
    // Only compile the regex once
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
    }

    let caps = RE.captures(line).unwrap();
    let id: usize = caps[1].parse()?;
    let left: u64 = caps[2].parse()?;
    let top: u64 = caps[3].parse()?;
    let width: u64 = caps[4].parse()?;
    let height = caps[5].parse()?;
    Ok((id, left, top, width, height))
}

// We have a table with one entry per claim along with the overall fabric matrix. Much
// like in part one we mark the fabric as we check claims, but in this case we mark the inch
// with the claim id. If there's an existing claim id when we mark the current claim then we
// know that both the current claim, as well as the claim already stored both have overlaps.
const LENGTH: usize = 1000;
fn main() -> Result<(), Error> {
    let mut fabric = vec![[0; LENGTH]; LENGTH];
    let buf = fs::read_to_string("input.txt")?;
    let elf_cnt = buf.lines().count();
    let mut overlap_tbl = vec![false; elf_cnt];

    for line in buf.lines() {
        let (id, left, top, width, height) = get_rectangle(line)?;
        let mut self_overlap = false;

        for x in left..left + width {
            for y in top..top + height {
                // Every inch can overlap with a different claim so we need to set
                // each claim's overlap status each inch
                if fabric[x as usize][y as usize] != 0 {
                    overlap_tbl[fabric[x as usize][y as usize] - 1] = true;
                    self_overlap = true;
                }
                fabric[x as usize][y as usize] = id;
            }
        }

        if self_overlap {
            overlap_tbl[id - 1] = true;
        }
    }

    // If we wanted to be fancy we could have used a set, then run a filter across
    // it or something similar, but the runtime would likely be far worse.
    for (id, overlapped) in overlap_tbl.iter().enumerate() {
        if !overlapped {
            println!("{} never overlapped", id + 1);
            break;
        }
    }

    Ok(())
}
