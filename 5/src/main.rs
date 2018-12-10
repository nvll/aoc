use failure::Error;
use std::collections::LinkedList;
use std::fs;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let list = load_list(&input);
    println!("part 1 result: {}", list.len());
}

fn part2(input: &str) {
    let mut len_min = 9808; // value from part 1
    let mut letter = 'a';

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let new_len = part2_filter(input, c).len();
        if new_len < len_min {
            len_min = new_len;
            letter = c;
        }
    }

    println!("part 2 result: {}/{}", letter, len_min);
}

fn part2_filter(input: &str, d: char) -> LinkedList<char> {
    let d_l = d.to_lowercase().next().unwrap();
    let d_u = d.to_uppercase().next().unwrap();

    load_list(&input
        .chars()
        .filter(|c| {
            if *c == d_l || *c == d_u {
                return false
            } 
            true
        })
        .collect::<String>())
}

fn reactive(b: char, c: char) -> bool{
    // Check if we have a polymer reaction
    // For example: A = 65, a = 97
    // abs|A - A| = 0
    // abs|a - a| = 0
    // abs|a - A| = 32
    // abs|A - a| = 32
    return (b as i64 - c as i64).abs() == 32
}

fn load_list(s: &str) -> LinkedList<char> {
    let mut list: LinkedList<char> = LinkedList::new();
    for c in s.trim().chars() {
        if list.is_empty() {
            list.push_back(c);
            continue;
        }

        let b = *list.back().unwrap();
        if reactive(b, c) {
            list.pop_back();
            continue;
        }

        list.push_back(c);
    }

    list
}

#[test]
fn test_input() {
    let list = load_list("dabAcCaCBAcCcaDA");

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&'d'));
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'b'));
    assert_eq!(iter.next(), Some(&'C'));
    assert_eq!(iter.next(), Some(&'B'));
    assert_eq!(iter.next(), Some(&'A'));
    assert_eq!(iter.next(), Some(&'c'));
    assert_eq!(iter.next(), Some(&'a'));
    assert_eq!(iter.next(), Some(&'D'));
    assert_eq!(iter.next(), Some(&'A'));
}
