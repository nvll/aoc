const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

fn part1() -> u32 {
    solution(calculate_fuel)
}

fn part2() -> u32 {
    solution(calculate_fuel_with_mass)
}

fn solution<F>(fuel_fn: F) -> u32
where
    F: Fn(u32) -> u32,
{
    process_input()
        .iter()
        .fold(0, |total, mass| total + fuel_fn(*mass))
}

fn process_input() -> Vec<u32> {
    std::fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(|mass| mass.parse::<u32>().unwrap())
        .collect()
}

fn calculate_fuel(mass: u32) -> u32 {
    u32::from(mass / 3).checked_sub(2).unwrap_or(0)
}

fn calculate_fuel_with_mass(mut mass: u32) -> u32 {
    let mut total = 0;
    while mass > 0 {
        mass = calculate_fuel(mass);
        total += mass;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const PART1_ANSWER: u32 = 3252897;
    const PART2_ANSWER: u32 = 4876469;

    #[test]
    fn input_parse() {
        process_input();
    }

    #[test]
    fn part1_regression() {
        assert_eq!(part1(), PART1_ANSWER)
    }

    #[test]
    fn part2_regression() {
        assert_eq!(part2(), PART2_ANSWER)
    }

    #[test]
    fn fuel_calculation_without_mass() {
        let test_data = [(12, 2), (14, 2), (1969, 654), (100756, 33583)];
        for test in &test_data {
            assert_eq!(calculate_fuel(test.0), test.1);
        }

        // Check overflow subtraction
        assert_eq!(calculate_fuel(3), 0);
    }

    #[test]
    fn fuel_calculation_with_mass() {
        let test_data = [(1969, 966), (100756, 50346)];
        for test in &test_data {
            assert_eq!(calculate_fuel_with_mass(test.0), test.1);
        }
    }

}
