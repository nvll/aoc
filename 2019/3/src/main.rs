const INPUT_FILE: &str = "input.txt";
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct WirePoint {
    wire_id: usize,
    steps: usize,
}

// Hash & PartialEq are implemented manually to ensure the WirePoint hashset
// never includes more than a single step value at a given point.
impl PartialEq for WirePoint {
    fn eq(&self, other: &Self) -> bool {
        self.wire_id == other.wire_id
    }
}

impl Hash for WirePoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wire_id.hash(state);
    }
}

impl Eq for WirePoint {}

type WireGrid = HashMap<(i32, i32), HashSet<WirePoint>>;

#[derive(Debug, PartialEq)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        let c = s.chars().next().unwrap();
        match c {
            'U' => Direction::Up(s[1..].parse::<i32>().unwrap()),
            'D' => Direction::Down(s[1..].parse::<i32>().unwrap()),
            'L' => Direction::Left(s[1..].parse::<i32>().unwrap()),
            'R' => Direction::Right(s[1..].parse::<i32>().unwrap()),
            _ => panic!("Unknown supported direction {}", c),
        }
    }
}

struct Grid {
    grid: WireGrid,
}

impl Grid {
    pub fn new(wires: Vec<Vec<Direction>>) -> Grid {
        let mut grid: WireGrid = HashMap::new();
        for (wire_id, wire) in wires.iter().enumerate() {
            let mut x = 0;
            let mut y = 0;
            let mut steps = 0;
            for direction in wire {
                // For each turn we have an associated distance we need to mark to calculate
                // intersection at points that are not an endpoint. The wires only can move in a
                // single axis at a time so track which axes needs to change and the coordinate
                // where it needs to end up.
                let (target_x, target_y) = Grid::route_wire(x, y, direction);
                let change_x: bool = x != target_x;
                let grid_step = if x < target_x || y < target_y { 1 } else { -1 };
                let mut iterations = (x - target_x).abs() + (y - target_y).abs();

                while iterations > 0 {
                    if change_x {
                        x += grid_step;
                    } else {
                        y += grid_step;
                    }
                    steps += 1;
                    grid.entry((x, y))
                        .or_insert_with(HashSet::new)
                        .insert(WirePoint { wire_id, steps });

                    iterations -= 1;
                }
            }
        }

        Grid { grid }
    }

    pub fn find_closest_intersection_mdistance(&self) -> u32 {
        let mut mdistance: Option<u32> = None;
        for (pos, wires) in &self.grid {
            if wires.len() > 1 {
                let md = (pos.0.abs() + pos.1.abs()) as u32;
                if mdistance.is_none() || mdistance.unwrap() > md {
                    mdistance = Some(md);
                }
            }
        }
        mdistance.unwrap()
    }

    pub fn find_lowest_intersection_step(&self) -> u32 {
        let mut lowest_step: Option<u32> = None;
        for wires in self.grid.values() {
            if wires.len() > 1 {
                let intersection_step = wires.iter().fold(0, |acc, wp| acc + wp.steps) as u32;
                if lowest_step.is_none() || intersection_step < lowest_step.unwrap() {
                    lowest_step = Some(intersection_step);
                }
            }
        }
        lowest_step.unwrap()
    }

    fn route_wire(x: i32, y: i32, d: &Direction) -> (i32, i32) {
        match d {
            Direction::Up(n) => (x, y + *n),
            Direction::Down(n) => (x, y - *n),
            Direction::Right(n) => (x + *n, y),
            Direction::Left(n) => (x - *n, y),
        }
    }
}

fn main() {
    let grid = Grid::new(process_input(None));
    println!("part 1: {}", part1(&grid));
    println!("part 2: {}", part2(&grid));
}

fn part1(grid: &Grid) -> u32 {
    grid.find_closest_intersection_mdistance()
}

fn part2(grid: &Grid) -> u32 {
    grid.find_lowest_intersection_step()
}

fn process_line(line: &str) -> Vec<Direction> {
    line.split(',').map(|s| Direction::from_str(s)).collect()
}

fn process_input(s: Option<&str>) -> Vec<Vec<Direction>> {
    let input = match s {
        Some(s) => s.to_owned(),
        None => std::fs::read_to_string(INPUT_FILE).unwrap(),
    };
    input.lines().map(|line| process_line(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parse() {
        process_input(None);
    }

    #[test]
    fn direction_parse() {
        assert_eq!(Direction::from_str("D1234"), Direction::Down(1234));
        {
            let line = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83";
            let wire = vec![
                Direction::Right(75),
                Direction::Down(30),
                Direction::Right(83),
                Direction::Up(83),
                Direction::Left(12),
                Direction::Down(49),
                Direction::Right(71),
                Direction::Up(7),
                Direction::Left(72),
                Direction::Up(62),
                Direction::Right(66),
                Direction::Up(55),
                Direction::Right(34),
                Direction::Down(71),
                Direction::Right(55),
                Direction::Down(58),
                Direction::Right(83),
            ];
            assert_eq!(process_line(line), wire);
        }
    }

    #[test]
    fn mdistance() {
        let test_examples = [
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];

        for test in &test_examples {
            let grid = Grid::new(process_input(Some(test.0)));
            assert_eq!(grid.find_closest_intersection_mdistance(), test.1);
        }
    }

    #[test]
    fn lowest_step() {
        let test_examples = [
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 30),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ];

        for test in &test_examples {
            let grid = Grid::new(process_input(Some(test.0)));
            assert_eq!(grid.find_lowest_intersection_step(), test.1);
        }
    }
}
