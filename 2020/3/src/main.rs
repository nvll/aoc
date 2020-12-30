use anyhow::Error;

#[derive(Debug)]
struct Slope {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Error> {
    let input = read_input("input")?;
    println!("p1: {}", p1(&Slope { x: 3, y: 1 }, &input)?);
    println!("p2: {}", p2(&input)?);
    Ok(())
}

fn read_input(path: &str) -> Result<Vec<Vec<bool>>, Error> {
    let input: Vec<Vec<bool>> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    Ok(input)
}

fn p1(slope: &Slope, hill: &[Vec<bool>]) -> Result<usize, Error> {
    let rows = hill.len();
    let cols = hill[0].len();

    let mut hits: usize = 0;
    let mut pos = Slope { x: 0, y: 0 };
    while pos.y + slope.y < rows {
        pos.x = (pos.x + slope.x) % cols;
        pos.y = (pos.y + slope.y) % rows;
        hits += hill[pos.y][pos.x] as usize;
    }

    Ok(hits)
}

fn p2(hill: &[Vec<bool>]) -> Result<usize, Error> {
    let slopes = &[
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    let mut hits = p1(&slopes[0], &hill)?;
    for slope in slopes[1..slopes.len()].iter() {
        hits *= p1(&slope, &hill)?;
    }
    Ok(hits)
}

#[cfg(test)]
mod test {
    use super::{p1, p2, read_input, Error, Slope};

    #[test]
    fn p1_sample() -> Result<(), Error> {
        let input = read_input("sample")?;
        let slope = Slope { x: 3, y: 1 };
        assert_eq!(7, p1(&slope, &input)?);
        Ok(())
    }

    #[test]
    fn p2_sample() -> Result<(), Error> {
        let input = read_input("sample")?;
        assert_eq!(336, p2(&input)?);
        Ok(())
    }
}
