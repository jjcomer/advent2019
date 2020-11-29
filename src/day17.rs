use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};

use std::char;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

fn display_output(input: &Vec<i64>) {
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

//....................................#########
