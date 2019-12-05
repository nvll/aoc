const INPUT_LOW: u32 = 264793;
const INPUT_HIGH: u32 = 803935;

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

fn part1() -> u32 {
    (INPUT_LOW..=INPUT_HIGH).fold(0, |acc, p| acc + (is_valid_password(p) as u32))
}

fn part2() -> u32 {
    (INPUT_LOW..=INPUT_HIGH).fold(0, |acc, p| acc + (is_valid_strict_password(p) as u32))
}

fn is_valid_password(p: u32) -> bool {
    let mut has_valid_pair = false;
    let mut is_ascending = true;
    let mut prev_digit = None;
    let digits: Vec<u32> = p
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    for digit in digits {
        if let Some(prev) = prev_digit {
            if digit == prev {
                has_valid_pair = true;
            } else {
                if digit < prev {
                    is_ascending = false;
                }
            }
        }
        prev_digit = Some(digit);
    }

    has_valid_pair && is_ascending
}

fn is_valid_strict_password(p: u32) -> bool {
    let mut has_valid_pair = false;
    let mut is_ascending = true;
    let mut prev_digit = None;
    let mut pair_len = 1;
    let digits: Vec<u32> = p
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    for digit in digits {
        if let Some(prev) = prev_digit {
            if digit == prev {
                pair_len += 1;
            } else {
                if pair_len == 2 {
                    has_valid_pair = true;
                }
                pair_len = 1;

                if digit < prev {
                    is_ascending = false;
                }
            }
        }
        prev_digit = Some(digit);
    }

    // Handle the case where we have a valid pair at the end of the password
    if pair_len == 2 {
        has_valid_pair = true;
    }

    has_valid_pair && is_ascending
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_passwords() {
        let test_data = [
            (111111, true),
            (223450, false),
            (123789, false),
            (111123, true),
            (122345, true),
        ];

        for test in &test_data {
            assert_eq!(is_valid_password(test.0), test.1);
        }
    }

    #[test]
    fn valid_strict_passwords() {
        let test_data = [
            (111111, false),
            (112233, true),
            (123444, false),
            (111122, true),
        ];

        for test in &test_data {
            assert_eq!(is_valid_strict_password(test.0), test.1);
        }
    }

    #[test]
    fn part1_regression() {
        assert_eq!(part1(), 966);
    }

    #[test]
    fn part2_regression() {
        assert_eq!(part2(), 628);
    }

}
