use anyhow::{anyhow, Error};
use std::cmp::Ordering;

fn main() -> Result<(), Error> {
    let mut input: Vec<u64> = std::fs::read_to_string("input")?
        .lines()
        .flat_map(|l| l.parse::<u64>())
        .collect();

    input.sort_unstable();
    println!("p1: {}", p1(&input)?);
    println!("p2: {}", p2(&input)?);

    Ok(())
}

fn p1(input: &[u64]) -> Result<u64, Error> {
    for i in 0..input.len() {
        for j in 0..input.len() {
            let sum = input[i] + input[j];
            match sum.cmp(&2020) {
                Ordering::Greater => break,
                Ordering::Equal => return Ok(input[i] * input[j]),
                _ => continue,
            };
        }
    }
    Err(anyhow!("2020 not found in set"))
}

fn p2(input: &[u64]) -> Result<u64, Error> {
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                let sum = input[i] + input[j] + input[k];
                match sum.cmp(&2020) {
                    Ordering::Greater => break,
                    Ordering::Equal => return Ok(input[i] * input[j] * input[k]),
                    _ => continue,
                };
            }
        }
    }
    Err(anyhow!("2020 not found in set"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_sample() -> Result<(), Error> {
        let mut data = vec![1721, 979, 366, 299, 675, 1456];
        data.sort_unstable();
        assert_eq!(p1(&data)?, 514579);
        Ok(())
    }

    #[test]
    fn p2_sample() -> Result<(), Error> {
        let mut data = vec![1721, 979, 366, 299, 675, 1456];
        data.sort_unstable();
        assert_eq!(p2(&data)?, 241861950);
        Ok(())
    }
}
