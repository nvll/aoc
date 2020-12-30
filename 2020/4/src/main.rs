use anyhow::Error;

fn main() -> Result<(), Error> {
    let input = read_input("input")?;
    let p1 = p1(&input)?;
    println!("p1: {}", p1);
    Ok(())
}

fn validate_passport(s: &str) -> Result<bool, Error> {
    let mut buffer = String::new();
    let mut val = 0;
    for c in s.chars() {
        if c == ':' {
            let field = &buffer[buffer.len() - 3..buffer.len()];
            val |= match field {
                "cid" => 1 << 0,
                "byr" => 1 << 1,
                "iyr" => 1 << 2,
                "eyr" => 1 << 3,
                "hgt" => 1 << 4,
                "hcl" => 1 << 5,
                "ecl" => 1 << 6,
                "pid" => 1 << 7,
                _ => panic!("Unknown field: {}", field),
            };
        } else {
            buffer.push(c);
        }
    }

    Ok(val == 0xFF || val == 0xFE)
}

fn read_input(path: &str) -> Result<Vec<String>, Error> {
    let buffer = std::fs::read_to_string(path)?;
    let mut entries: Vec<String> = Vec::new();
    let mut passport = String::new();
    for line in buffer.lines() {
        match line.trim().is_empty() {
            true => {
                entries.push(passport);
                passport = String::new();
            }
            false => {
                if !passport.is_empty() {
                    passport.push(' ');
                }
                passport.push_str(line)
            }
        }
    }

    // We won't have the final newline due to how lines() works
    if !passport.is_empty() {
        entries.push(passport);
    }
    Ok(entries)
}

fn p1(passports: &[String]) -> Result<usize, Error> {
    let mut total = 0;
    for passport in passports {
        total += validate_passport(passport)? as usize;
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::{p1, read_input, Error};
    #[test]
    fn input() -> Result<(), Error> {
        let entries = read_input("sample")?;
        let expected = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in",
        ];
        for i in 0..entries.len() {
            assert_eq!(expected[i], entries[i]);
        }
        Ok(())
    }

    #[test]
    fn p1_sample() -> Result<(), Error> {
        let entries = read_input("sample")?;
        assert_eq!(2, p1(&entries)?);
        Ok(())
    }
}
