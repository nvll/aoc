const GRID_WIDTH: usize = 300;
const SERIAL: i64 = 6392;

fn main() {
    println!("part1: {:?}", part1(SERIAL, 3));
    println!("part2: {:?}", part2(SERIAL, 1, 300));
}

fn calc_cell_power_level(x: usize, y: usize, serial: i64) -> i64 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = (x as i64) + 10;
    // Begin with a power level of the rack ID times the Y coordinate.
    let mut power_lvl = rack_id * (y as i64);
    // Increase the power level by the value of the grid serial number (your puzzle input).
    power_lvl += serial;
    // Set the power level to itself multiplied by the rack ID.
    power_lvl *= rack_id;
    // Keep only the hundreds digit of the power level
    power_lvl /= 10;
    power_lvl /= 10;
    power_lvl %= 10;
    // Subtract 5 from the power level.
    power_lvl -= 5;

    power_lvl
}

fn part1(serial: i64, width: usize) -> (usize, usize, i64) {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; GRID_WIDTH]; GRID_WIDTH];
    // Populate the matrix
    for r in 0..GRID_WIDTH {
        for c in 0..GRID_WIDTH {
            // +1 on each coordinate since the problem starts at (1, 1) top left
            grid[r][c] = calc_cell_power_level(r + 1, c + 1, serial);
        }
    }

    let mut x_max = 0;
    let mut y_max = 0;
    let mut max: i64 = 0;
    for r in 0..GRID_WIDTH - width {
        for c in 0..GRID_WIDTH - width {
            let mut level: i64 = 0;
            for x in 0..width {
                for y in 0..width {
                    level += grid[r + x][c + y];
                }
            }
            if level > max {
                max = level;
                // Remembering that top left is (1, 1) again
                x_max = r + 1;
                y_max = c + 1;
            }
        }
    }
    (x_max, y_max, max)
}

fn part2(serial: i64, width_min: usize, width_max: usize) -> (usize, usize, usize) {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; GRID_WIDTH]; GRID_WIDTH];
    let mut cache: Vec<Vec<i64>> = vec![vec![0; GRID_WIDTH]; GRID_WIDTH];

    for r in 0..GRID_WIDTH {
        for c in 0..GRID_WIDTH {
            // +1 on each coordinate since the problem starts at (1, 1) top left
            grid[r][c] = calc_cell_power_level(r + 1, c + 1, serial);
        }
    }

    // We need to cache the lower values to calculate the higher ones, but each size
    // increase should be additive.
    let mut global_max_x = 0;
    let mut global_max_y = 0;
    let mut global_max_width = 0;
    let mut global_max_level = 0;

    for s_w in 1..=width_max {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut level_max: i64 = 0;
        for r in 0..GRID_WIDTH - s_w {
            for c in 0..GRID_WIDTH - s_w {
                let mut level: i64 = cache[r][c];
                // Add the new column in its entirety
                for y in 0..s_w {
                    level += grid[r + s_w - 1][c + y];
                }
                // Add the new row, leaving off the final corner because it was added above
                for x in 0..s_w - 1 {
                    level += grid[r + x][c + s_w - 1];
                }
                cache[r][c] = level;
                if s_w >= width_min && level > level_max {
                    level_max = level;
                    max_x = r + 1;
                    max_y = c + 1;
                }
            }
        }
        if level_max > global_max_level {
            global_max_x = max_x;
            global_max_y = max_y;
            global_max_width = s_w;
            global_max_level = level_max;
        }
    }
    (global_max_x, global_max_y, global_max_width)
}

#[test]
fn test_power_levels() {
    assert_eq!(calc_cell_power_level(3, 5, 8), 4);
    assert_eq!(calc_cell_power_level(122, 79, 57), -5);
    assert_eq!(calc_cell_power_level(217, 196, 39), 0);
    assert_eq!(calc_cell_power_level(101, 153, 71), 4);
}

#[test]
fn test_grid_examples() {
    assert_eq!(part1(18, 3), (33, 45, 29));
    assert_eq!(part1(42, 3), (21, 61, 30));
    assert_eq!(part1(18, 16), (90, 269, 113));
    assert_eq!(part1(42, 12), (232, 251, 119));
}
