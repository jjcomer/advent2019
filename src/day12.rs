use regex::Regex;
use std::cmp::Ordering;

#[derive(Default, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Body {
    velocity: [i64; 3],
    position: [i64; 3],
}

impl Body {
    fn apply_velocity(mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }

    fn compute_energy(self) -> i64 {
        self.position.iter().map(|x| x.abs()).sum::<i64>()
            * self.velocity.iter().map(|x| x.abs()).sum::<i64>()
    }
}

fn gravitate(planet: &mut Body, other: &Body) {
    for i in 0..3 {
        // println!(
        //     "Comparing {:?} {:?} {:?}",
        //     planet,
        //     other,
        //     planet.position[i].cmp(&other.position[i])
        // );
        match planet.position[i].cmp(&other.position[i]) {
            Ordering::Equal => {}
            Ordering::Greater => {
                planet.velocity[i] -= 1;
            }
            Ordering::Less => planet.velocity[i] += 1,
        }
        //println!("Compared {:?} {:?}", planet, other);
    }
}

fn apply_velocity(planet: &mut Body) {
    for i in 0..3 {
        planet.position[i] += planet.velocity[i];
    }
}

pub type Coord = Vec<Body>;
//<x=-1, y=0, z=2>

#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Coord {
    let reg = Regex::new(r"^<x=([-\d]+), y=([-\d]+), z=([-\d]+)>$").unwrap();

    input
        .lines()
        .map(|l| {
            let matches = reg.captures(l).unwrap();
            let position = [
                matches[1].parse::<i64>().unwrap(),
                matches[2].parse::<i64>().unwrap(),
                matches[3].parse::<i64>().unwrap(),
            ];
            Body {
                position,
                ..Default::default()
            }
        })
        .collect()
}

fn process(cycles: i64, planets: &mut Coord) -> i64 {
    //println!("Cycle: 0 Planets: {:?}", planets);
    for cycle in 0..cycles {
        let p_copy = planets.to_owned();
        for mut a in planets.iter_mut() {
            for b in p_copy.iter() {
                gravitate(&mut a, &b);
            }
        }
        for i in 0..4 {
            apply_velocity(planets.get_mut(i).unwrap());
        }
        //println!("Cycle: {} Planets: {:?}", cycle, planets);
    }
    planets.iter().map(|x| x.compute_energy()).sum()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Coord) -> i64 {
    let mut planets = input.to_owned();
    process(1000, &mut planets)
}

fn compare_positions(a: &Coord, b: &Coord) -> bool {
    a.iter().zip(b.iter()).all(|x| x.0.position == x.1.position)
}

pub fn solve_part2(input: &Coord) -> i64 {
    let mut planets = input.to_owned();
    let mut counter = 0;
    loop {
        let p_copy = planets.to_owned();
        for mut a in planets.iter_mut() {
            for b in p_copy.iter() {
                gravitate(&mut a, &b);
            }
        }
        for i in 0..4 {
            apply_velocity(planets.get_mut(i).unwrap());
        }
        counter += 1;
        if compare_positions(&planets, input) {
            break;
        }
        //println!("Cycle: {} Planets: {:?}", cycle, planets);
    }
    counter
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
                position: [-1, 0, 2],
                ..Default::default()
            }]
        );
    }

    // #[test]
    // fn example2() {
    //     let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
    //     assert_eq!(process(1, &mut generate_input(&input)), 1940);
    // }

    #[test]
    fn example3() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        assert_eq!(process(100, &mut generate_input(&input)), 1940);
    }

    #[test]
    fn example2() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        assert_eq!(solve_part2(&generate_input(&input)), 4686774924);
    }
}
