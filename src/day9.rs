use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Program) -> i64 {
    let mut input = input.to_owned();
    match run_program(&mut input, 0, 0, vec![1]) {
        IntCodeResult::Halt(output) => {
            println!("{:?}", output);
            output[0]
        }
        _ => panic!("Not expecting more input"),
    }
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Program) -> i64 {
    let mut input = input.to_owned();
    match run_program(&mut input, 0, 0, vec![2]) {
        IntCodeResult::Halt(output) => {
            println!("{:?}", output);
            output[0]
        }
        _ => panic!("Not expecting more input"),
    }
}
