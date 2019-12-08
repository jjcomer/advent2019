use std::collections::HashMap;

type Orbits = HashMap<String, String>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Orbits {
    let mut nodes = HashMap::new();

    for x in input.lines() {
        let mut n = x.split(')');
        let node_a = n.next().unwrap().to_owned();
        let node_b = n.next().unwrap().to_owned();
        nodes.insert(node_b, node_a);
    }
    nodes
}

fn count_orbits(o: &str, orbits: &Orbits) -> i32 {
    if let Some(o) = orbits.get(o) {
        1 + count_orbits(o, orbits)
    } else {
        0
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Orbits) -> i32 {
    let mut orbit_count = 0;
    for k in input.keys() {
        orbit_count += count_orbits(k, input);
    }
    orbit_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(solve_part1(&input_generator(&input)), 42);
    }
}
