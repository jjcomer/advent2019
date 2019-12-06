use std::collections::HashMap;
use std::collections::HashSet;

type DistanceMap = HashMap<(i32, i32), i32>;
type PointSet = HashSet<(i32, i32)>;

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

fn calc_distance((x, y): &(i32, i32)) -> i32 {
    //println!("Intersection: {} {} {}", x, y, x.abs() + y.abs());
    x.abs() + y.abs()
}

fn calc_distance_2(d1: &DistanceMap, d2: &DistanceMap, point: (i32, i32)) -> i32 {
    let w1 = d1.get(&point).unwrap();
    let w2 = d2.get(&point).unwrap();

    w1 + w2
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(DistanceMap, PointSet)> {
    input
        .lines()
        .map(|x| parse_points(x))
        .map(build_points)
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[(DistanceMap, PointSet)]) -> i32 {
    let (_, wire_a) = &input[0];
    let (_, wire_b) = &input[1];
    wire_a
        .intersection(wire_b)
        .map(calc_distance)
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[(DistanceMap, PointSet)]) -> i32 {
    let (distance_a, wire_a) = &input[0];
    let (distance_b, wire_b) = &input[1];
    wire_a
        .intersection(wire_b)
        .map(|p| calc_distance_2(&distance_a, &distance_b, *p))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve_part1(&input_generator(&input)), 159);
    }

    #[test]
    fn example2() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve_part1(&input_generator(&input)), 135);
    }

    #[test]
    fn example3() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(solve_part2(&input_generator(&input)), 610);
    }

    #[test]
    fn example4() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(solve_part2(&input_generator(&input)), 410);
    }
}
