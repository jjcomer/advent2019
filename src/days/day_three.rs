use crate::util;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    // Answer:159
    static ref SAMPLE_1: Vec<String> = vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()
    ];
    // Answer:135
    static ref SAMPLE_2: Vec<String> = vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()
    ];
}

type DistanceMap = HashMap<(i32, i32), i32>;

fn update_distance(distances: &mut DistanceMap, point: (i32, i32), count: i32) {
    distances.entry(point).or_insert(count);
}

fn build_points(directions: Vec<String>) -> (DistanceMap, HashSet<(i32, i32)>) {
    let mut current_location = (0, 0);
    let mut points = HashSet::new();
    let mut distance = HashMap::new();
    let mut counter = 0;
    for direction in directions {
        let d = &direction[..1];
        let n = direction[1..].parse::<usize>().unwrap();
        let (xo, yo) = match d {
            "R" => (1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            "L" => (-1, 0),
            _ => panic!("Unknown direction {}", d),
        };
        for _ in 0..n {
            counter += 1;
            let (x1, y1) = current_location;
            let new_location = (x1 + xo, y1 + yo);
            current_location = new_location;
            points.insert(new_location);
            update_distance(&mut distance, new_location, counter);
        }
    }
    (distance, points)
}

fn parse_points(raw_points: &str) -> Vec<String> {
    raw_points.split(',').map(|s| s.to_owned()).collect()
}

// fn calc_distance((x, y): &(i32, i32)) -> i32 {
//     //println!("Intersection: {} {} {}", x, y, x.abs() + y.abs());
//     x.abs() + y.abs()
// }

fn calc_distance_2(d1: &DistanceMap, d2: &DistanceMap, point: (i32, i32)) -> i32 {
    let w1 = d1.get(&point).unwrap();
    let w2 = d2.get(&point).unwrap();

    w1 + w2
}

pub fn process() -> Result<()> {
    let input = util::read_input("input/day_three.txt")?;
    let input = input
        .iter()
        .map(|x| parse_points(x))
        .map(build_points)
        .collect::<Vec<_>>();
    let (distance_a, wire_a) = &input[0];
    let (distance_b, wire_b) = &input[1];
    // let point: i32 = wire_a
    //     .intersection(wire_b)
    //     .map(calc_distance)
    //     .min()
    //     .unwrap();

    let point: i32 = wire_a
        .intersection(wire_b)
        .map(|p| calc_distance_2(&distance_a, &distance_b, *p))
        .min()
        .unwrap();

    println!("Shortest path: {}", point);

    Ok(())
}
