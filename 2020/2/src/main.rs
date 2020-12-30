use anyhow::Error;
use std::cmp::Ordering;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = read_input("input")?;
    println!("p1: {}", p1(&input)?);
    println!("p2: {}", p2(&input)?);
    Ok(())
}

struct Password {
    min: u64,
    max: u64,
    digit: char,
    password: String,
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let v0: Vec<&str> = s.split(": ").collect();
        let password = v0[1];
        let v1: Vec<&str> = v0[0].split(' ').collect();
        let digit = v1[1];
        let rules: Vec<&str> = v1[0].split('-').collect();
        Ok(Password {
            min: rules[0].parse()?,
            max: rules[1].parse()?,
            digit: digit.chars().next().unwrap(),
            password: password.to_string(),
        })
    }
}

fn read_input(path: &str) -> Result<Vec<Password>, Error> {
    let input: Vec<Password> = std::fs::read_to_string(path)?
        .lines()
        .flat_map(|l| l.parse::<Password>())
        .collect();
    Ok(input)
}

fn p1(input: &[Password]) -> Result<u64, Error> {
    let total = input.iter().fold(0, |total, entry| {
        let sum = entry
            .password
            .chars()
            .fold(0, |acc, c| if c == entry.digit { acc + 1 } else { acc });

        if sum >= entry.min && sum <= entry.max {
            total + 1
        } else {
            total
        }
    });

    Ok(total)
}

fn p2(input: &[Password]) -> Result<u64, Error> {
    let total = input.iter().fold(0, |total, entry| {
        let mut chars = entry.password.chars();
        let min = chars.nth(entry.min as usize - 1).unwrap();
        // nth() consumes elements
        let max = chars.nth((entry.max - entry.min) as usize - 1).unwrap();
        let count = (min == entry.digit) as u32 + (max == entry.digit) as u32;

        match count {
            1 => total + 1,
            _ => total,
        }
    });

    Ok(total)
}

#[cfg(test)]
mod test {
    use super::{p1, p2, read_input, Error};

    #[test]
    fn p1_sample() -> Result<(), Error> {
        let input = read_input("sample")?;
        assert_eq!(2, p1(&input)?);
        Ok(())
    }

    #[test]
    fn p2_sample() -> Result<(), Error> {
        let input = read_input("sample")?;
        assert_eq!(1, p2(&input)?);
        Ok(())
    }
}
