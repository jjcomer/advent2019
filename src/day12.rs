use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Default, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Body {
    velocity: [i64; 3],
    position: [i64; 3],
}

impl Body {
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

pub type Coord = [Body];
//<x=-1, y=0, z=2>

#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Vec<Body> {
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
    for _cycle in 0..cycles {
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

fn check_for_periods(
    periods: &mut HashMap<usize, i64>,
    original: &Coord,
    current: &Coord,
    dimension: usize,
    cycle: i64,
) {
    for (i, x) in original.iter().enumerate() {
        if !periods.contains_key(&i) {
            let body = current.get(i).unwrap();
            if x.position[dimension] == body.position[dimension] && body.velocity[dimension] == 0 {
                // println!(
                //     "Found cycle for {:?} {} on cycle: {}",
                //     current.get(i).unwrap(),
                //     dimension,
                //     cycle
                // );
                periods.insert(i, cycle);
            }
        }
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Coord) -> usize {
    let mut planets = input.to_owned();
    let mut counter = 0;
    let mut x_cycles = HashMap::new();
    let mut y_cycles = HashMap::new();
    let mut z_cycles = HashMap::new();
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
        check_for_periods(&mut x_cycles, input, &planets, 0, counter);
        check_for_periods(&mut y_cycles, input, &planets, 1, counter);
        check_for_periods(&mut z_cycles, input, &planets, 2, counter);

        if x_cycles.len() == 4 && y_cycles.len() == 4 && z_cycles.len() == 4 {
            break;
        }
        //println!("Cycle: {} Planets: {:?}", cycle, planets);
    }

    println!("{:?} {:?} {:?}", x_cycles, y_cycles, z_cycles);

    x_cycles
        .values()
        .chain(y_cycles.values())
        .chain(z_cycles.values())
        .fold(0, |acc, x| {
            if acc == 0 {
                *x as usize
            } else {
                println!("{} {}", x, acc);
                lcm(*x as usize, acc)
            }
        })
}

fn lcm(a: usize, b: usize) -> usize {
    let div = gcd(a, b);
    (a / div) * b
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t
    }
    a
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

    #[test]
    fn example3() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        assert_eq!(process(100, &mut generate_input(&input)), 1940);
    }

    #[test]
    fn example4() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        assert_eq!(solve_part2(&generate_input(&input)), 4686774924);
    }

    #[test]
    fn example5() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        assert_eq!(solve_part2(&generate_input(&input)), 2772);
    }
}
