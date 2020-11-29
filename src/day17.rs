use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};

use std::char;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

fn display_output(input: &[i64]) {
    for c in input {
        print!("{}", (*c as u8) as char);
    }
}

const SCAFFOLD: i64 = 35;
const SPACE: i64 = 46;
const NEW_LINE: i64 = 10;

fn find_line_length(input: &[i64]) -> usize {
    input.iter().take_while(|x| **x != NEW_LINE).count() + 1
}

fn get_location(input: &[i64], line_length: usize, x: usize, y: usize) -> i64 {
    let index = x + (y * line_length);
    //println!("Looking up {} {:?}", index, input.get(index));
    *input.get(index).unwrap_or(&SPACE)
}

fn is_intersection(input: &[i64], line_length: usize, x: usize, y: usize) -> bool {
    get_location(input, line_length, x, y) == SCAFFOLD
        && get_location(input, line_length, x + 1, y) == SCAFFOLD
        && get_location(input, line_length, x - 1, y) == SCAFFOLD
        && get_location(input, line_length, x, y + 1) == SCAFFOLD
        && get_location(input, line_length, x, y - 1) == SCAFFOLD
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut program = input.to_owned();
    let result = run_program(&mut program, 0, 0, vec![]);

    match result {
        IntCodeResult::Halt(output) => {
            display_output(&output);
            let line_length = find_line_length(&output);
            println!("Size: {}", line_length);
            println!(
                "First Line: {:?}",
                output
                    .iter()
                    .take(line_length)
                    .enumerate()
                    .collect::<Vec<(usize, &i64)>>()
            );

            let intersections = iproduct!(0..line_length, 0..(output.len() / line_length))
                .filter(move |(x, y)| is_intersection(&output, line_length, *x, *y))
                .map(|x| {
                    println!("Intersection: {:?}", x);
                    x
                })
                .map(|(x, y)| x * y)
                .sum();

            intersections
        }
        IntCodeResult::Input(x, y, output) => {
            println!("Looking for input {} {} {}", x, y, output.len());
            0
        }
    }
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Program) -> i64 {
    let mut program = input.to_owned();
    program.insert(0, 2);
    let main_sequence = "A,A,B,C,B,A,C,B,C,A\n".chars();
    let a_sequence = "L,6,R,12,L,6,L,8,L,8\n".chars();
    let b_sequence = "L,6,R,12,R,8,L,8\n".chars();
    let c_sequence = "L,4,L,4,L,6\n".chars();
    let display = "n\n".chars();

    let input: Vec<i64> = main_sequence
        .chain(a_sequence)
        .chain(b_sequence)
        .chain(c_sequence)
        .chain(display)
        .map(|x| x as i64)
        .rev()
        .collect();

    println!("INPUT: {:?}", input);

    let result = run_program(&mut program, 0, 0, input);

    match result {
        IntCodeResult::Halt(output) => {
            println!("Halted!! {:?}", output);
            display_output(&output);
            *output.first().unwrap()
        }
        IntCodeResult::Input(_x, _y, o) => {
            println!("Unexpected input {:?}", o);
            -1
        }
    }
}
/*
A: L6 R12 L6 L8 L8
B: L6 R12 R8 L8
C: L4 L4 L6

A  A   A  A  A  |A  A   A  A  A | B  B   B  B | C  C  C | B  B   B  B | A  A   A  A  A | C  C  C | B  B   B  B | C  C  C  |A  A   A  A  A
L6 R12 L6 L8 L8 L6 R12 L6 L8 L8 L6 R12 R8 L8 L4 L4 L6 L6 R12 R8 L8 L6 R12 L6 L8 L8 L4 L4 L6 L6 R12 R8 L8 L4 L4 L6 L6 R12 L6 L8 L8

A,A,B,C,B,A,C,B,C,A
....................................#########
....................................#.......#
....................................#.......#
....................................#.......#
........................#######.....#.......#
........................#.....#.....#.......#
........................#.....#.#############
........................#.....#.#...#........
........................#.....#.#...#######..
........................#.....#.#.........#..
........................#.....#.#.........#..
........................#.....#.#.........#..
........................#########.........#..
..............................#...........#..
..............................#...........#..
..............................#...........#..
..............................######^.....#..
..........................................#..
..........................................#..
..........................................#..
..................................#########..
..................................#..........
............#########.............#..........
............#.......#.............#..........
..........#############.........#######......
..........#.#.......#.#.........#.#...#......
....#####.#.#.......#.#.........#.#...#......
....#...#.#.#.......#.#.........#.#...#......
#############.......#.#...#####.#.#####......
#...#...#.#.........#.#...#...#.#............
#...#######.........#############............
#.......#.............#...#...#..............
#.......#.............#########..............
#.......#.................#..................
#########.................#######............
................................#............
................................#............
................................#............
..............................#########......
................................#.....#......
................................#.....#......
................................#.....#......
................................#.....#......
................................#.....#......
................................#.....#......
................................#.....#......
................................#######......
*/
