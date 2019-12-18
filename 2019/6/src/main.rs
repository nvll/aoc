use std::collections::HashMap;
mod tests;

fn main() {
    println!("part 1: {}", count_orbits(None));
    println!("part 2: {}", count_orbital_transfers(None));
}

fn input(buffer: Option<&str>) -> String {
    buffer
        .map(|b| b.to_string())
        .unwrap_or_else(|| std::fs::read_to_string("input.txt").unwrap())
}

fn count_orbits(buffer: Option<&str>) -> u32 {
    let mut h: HashMap<&str, Vec<&str>> = HashMap::new();
    let s = input(buffer);
    // Build a tree out of a Map of Vectors since we don't have input
    // sorted to see the relations in respect to the root.
    for line in s.lines() {
        let v: Vec<&str> = line.trim().split(')').collect();
        if h.get_mut(v[0]).is_none() {
            h.insert(v[0], Vec::new());
        }
        h.get_mut(v[0]).unwrap().push(v[1]);
    }

    bfs_orbit_count(0, "COM", &h)
}

fn bfs_orbit_count(depth: u32, parent: &str, tree: &HashMap<&str, Vec<&str>>) -> u32 {
    if let Some(children) = tree.get(parent) {
        children
            .iter()
            .map(|c| bfs_orbit_count(depth + 1, c, &tree))
            .sum::<u32>()
            + depth
    } else {
        depth
    }
}

fn count_orbital_transfers(buffer: Option<&str>) -> u32 {
    let mut h: HashMap<&str, &str> = HashMap::new();
    let s = input(buffer);
    // This time build a tree so we can track child -> parent relationships
    for line in s.lines() {
        let v: Vec<&str> = line.trim().split(')').collect();
        h.insert(v[1], v[0]);
    }

    let you = get_path_to_root("YOU", &h);
    let san = get_path_to_root("SAN", &h);

    let mut common_nodes = 0;
    // Find the end of the common path they share
    for (l, r) in you.iter().zip(&san) {
        if l == r {
            common_nodes += 1;
        }
    }
    (you.len() + san.len() - (common_nodes * 2)) as u32
}

// Find the path to root, then reverse it
fn get_path_to_root<'a>(mut node: &'a str, tree: &'a HashMap<&str, &str>) -> Vec<&'a str> {
    let mut v = Vec::new();
    loop {
        if let Some(&parent) = tree.get(node) {
            v.push(parent);
            node = parent;
        } else {
            break;
        }
    }
    v.into_iter().rev().collect()
}
