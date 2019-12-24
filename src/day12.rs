use regex::Regex;

pub type Coord = Vec<(i64, i64, i64)>;

pub fn generate_input(input: &str) -> Coord {
    let reg = Regex::new(r"^<x=(\d+), y=(\d+), z=(\d+)>$").unwrap();

    input
        .lines()
        .map(|l| {
            let matches = reg.captures(l).unwrap();
            (
                matches[1].parse::<i64>().unwrap(),
                matches[2].parse::<i64>().unwrap(),
                matches[3].parse::<i64>().unwrap(),
            )
        })
        .collect()
}
