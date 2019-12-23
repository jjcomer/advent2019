use crate::intcode;
use permute::permutations_of;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> intcode::Program {
    intcode::input_generator(input)
}

fn run_series<'a>(program: intcode::Program, series: impl Iterator<Item = &'a i64>) -> i64 {
    let mut output = 0;
    for phase in series {
        match intcode::run_program(&mut program.clone(), 0, vec![output, *phase]) {
            intcode::IntCodeResult::Halt(o) => output = o[0],
            intcode::IntCodeResult::Input(_, _) => (),
        }
    }
    output
}

fn run_series_2<'a>(_program: Vec<i32>, _series: impl Iterator<Item = &'a i32>) -> i32 {
    0
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &intcode::Program) -> i64 {
    let series = (0..5).collect::<Vec<i64>>();
    permutations_of(&series)
        .map(|x| run_series(input.to_owned(), x))
        .max()
        .unwrap()
}

pub fn solve_part2(input: &[i32]) -> i32 {
    let series = (5..10).collect::<Vec<i32>>();
    permutations_of(&series)
        .map(|x| run_series_2(input.to_owned(), x))
        .max()
        .unwrap()
}
