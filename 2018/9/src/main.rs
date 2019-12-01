use failure::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let (player_cnt, end_value) = read_input(&fs::read_to_string("input.txt")?)?;
    run_game(9, 25);

    Ok(())
}

fn run_game(players: u64, final_marble: u64) -> (u64, u64) {
    let mut circle = vec![0; 1];
    let mut current = 0;
    for m in 1..=final_marble {
        let mut next = (current + 1) % circle.len() - 1;
        next = (next + 1) % circle.len() - 1;
        
        circle.insert(next, m);
        current = next;
        for (i, e) in circle.iter().enumerate() { 
            if i == next {
                print!("({}) ", e);
            } else {
                print!("{} ", e);
            }
        }
        println!("");
    }
    (0, 0)
}

fn read_input(s: &str) -> Result<(u64, u64), Error> {
    let split = s
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    Ok((split[0], split[1]))
}

#[test]
fn parse_input() {
    let s = "429 players; last marble is worth 70901 points";
    let (players, value) = read_input(&s).unwrap();
    assert_eq!(players, 429);
    assert_eq!(value, 70901);
}
