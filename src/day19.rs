use std::collections::HashSet;

use cached::proc_macro::cached;

use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};
use cached::stores::UnboundCache;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

#[cached(
    convert = r#"{format!("{},{}",x,y)}"#,
    create = "{UnboundCache::new()}",
    type = "UnboundCache<String,bool>"
)]
fn is_beam(program: &Program, x: i64, y: i64) -> bool {
    let mut program = program.to_owned();
    if let IntCodeResult::Halt(output) = run_program(&mut program, 0, 0, vec![x, y]) {
        1 == *output.first().unwrap()
    } else {
        false
    }
}

fn check_box(program: &Program, x: i64, y: i64) -> bool {
    is_beam(program, x, y) && is_beam(program, x + 99, y) && is_beam(program, x, y + 99)
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut beam = HashSet::new();

    for (x, y) in iproduct!(0..50 as i64, 0..50 as i64) {
        if is_beam(input, x, y) {
            beam.insert((x, y));
        }
    }
    beam.len()
}

fn print_beam(program: &Program, size: i64, x: i64, y: i64, highlights: Vec<(i64, i64)>) {
    for x in x..=x + size {
        for y in y..=y + size {
            if highlights.contains(&(x, y)) && is_beam(program, x, y) {
                print!("1");
            } else if highlights.contains(&(x, y)) && !is_beam(program, x, y) {
                print!("0")
            } else if is_beam(program, x, y) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Program) -> i64 {
    for i in 1800 as i64.. {
        if i % 100 == 0 {
            println!("Round {}", i);
        }
        for (x, y) in (0..=i).zip((0..=i).rev()) {
            //println!("Testing {} {}", x, y);
            if check_box(input, x, y) {
                println!("Found it {} {}", x, y);
                print_beam(
                    input,
                    110,
                    x - 5,
                    y - 5,
                    vec![(x, y), (x + 99, y), (x, y + 99)],
                );
                return x + (y * 10000);
            }
        }
    }
    -1
}
