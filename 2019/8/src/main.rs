mod tests;

// Size of layers
const W: usize = 25;
const H: usize = 6;

fn main() {
    let buffer = input(None);
    println!("part 1: {}", part1(&buffer));
    println!("part2");
    part2(&buffer);
}

fn part1(buffer: &[u32]) -> u32 {
    let layer_digits: Vec<[u32; 3]> = read_layers(W, H, buffer)
        .iter()
        .map(|l| {
            let mut d_v = [0; 3];
            for d in l.iter() {
                d_v[*d as usize] += 1
            }
            d_v
        })
        .collect();

    // Find the layer with the fewest zero digits
    let mut min_layer = &layer_digits[0];
    for layer in &layer_digits {
        if layer[0] < min_layer[0] {
            min_layer = &layer;
        }
    }

    // Requested answer is # of 1s * # of 2s
    min_layer[1] * min_layer[2]
}

fn part2(buffer: &[u32]) {
    let layers = read_layers(W, H, buffer);
    let mut image: [u32; W * H] = [0; W * H];
    for i in 0..image.len() {
        image[i] = layers.iter().find(|&r| r[i] != 2).unwrap()[i];
    }

    for pos in (0..image.len()).step_by(W) {
        for digit in &image[pos..pos + W] {
            match digit {
                1 => print!("\u{001b}[47m \u{001b}[0m"),
                0 | _ => print!("\u{001b}[40m \u{001b}[0m"),
            }
        }
        println!();
    }
}

// Takes the input digits and returns a vector of layers
fn read_layers(w: usize, h: usize, buffer: &[u32]) -> Vec<&[u32]> {
    let l_size = w * h;
    let mut layers = Vec::new();

    for start in (0..buffer.len()).step_by(l_size) {
        layers.push(&buffer[start..start + l_size]);
    }
    layers
}

// Read the file in and parse all the characters as digits
fn input(buffer: Option<&str>) -> Vec<u32> {
    buffer
        .map(|b| b.to_string())
        .unwrap_or_else(|| std::fs::read_to_string("input.txt").unwrap())
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
