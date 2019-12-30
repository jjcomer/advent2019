use regex::Regex;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Body {
    velocity: (i64, i64, i64),
    position: (i64, i64, i64),
}

impl std::ops::Add for Body {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let velocity = (
            self.velocity.0 + other.velocity.0,
            self.velocity.1 + other.velocity.1,
            self.velocity.2 + other.velocity.2,
        );
        Self {
            velocity,
            position: self.position,
        }
    }
}

pub type Coord = Vec<Body>;
//<x=-1, y=0, z=2>
pub fn generate_input(input: &str) -> Coord {
    let reg = Regex::new(r"^<x=([-\d]+), y=([-\d]+), z=([-\d]+)>$").unwrap();

    input
        .lines()
        .map(|l| {
            let matches = reg.captures(l).unwrap();
            let velocity = (
                matches[1].parse::<i64>().unwrap(),
                matches[2].parse::<i64>().unwrap(),
                matches[3].parse::<i64>().unwrap(),
            );
            Body {
                velocity,
                ..Default::default()
            }
        })
        .collect()
}

fn solve_part1(input: &Coord) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "<x=-1, y=0, z=2>";
        assert_eq!(
            generate_input(&input),
            vec![Body {
                velocity: (-1, 0, 2),
                ..Default::default()
            }]
        );
    }
}
