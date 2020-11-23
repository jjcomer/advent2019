use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

fn count_blocks(output: &[i64]) -> usize {
    let mut iter = output.iter();
    let mut blocks = HashSet::new();
    while let Some(x) = iter.next() {
        let y = iter.next().unwrap();
        let t = iter.next().unwrap();
        if *t == 2 {
            blocks.insert((x, y));
        }
    }
    blocks.len()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut input = input.to_owned();
    match run_program(&mut input, 0, 0, vec![1]) {
        IntCodeResult::Halt(output) => {
            println!("{:?} {}", output, output.len());
            count_blocks(&output)
        }
        _ => panic!("Not expecting more input"),
    }
}

fn find_score(output: &[i64]) -> usize {
    let mut iter = output.iter();
    loop {
        if let Some(x) = iter.next() {
            let y = iter.next().unwrap();
            let score = iter.next().unwrap();
            if *x == -1 && *y == 0 {
                return *score as usize;
            }
        } else {
            return 0;
        }
    }
}

type Screen = HashMap<(i64, i64), i64>;

fn render_screen(screen: &mut Screen, output: &[i64]) {
    let mut iter = output.iter();

    while let Some(x) = iter.next() {
        let y = iter.next().unwrap();
        let block_type = iter.next().unwrap();
        screen.insert((*x, *y), *block_type);
    }

    let max_x = screen.keys().map(|k| k.0).max().unwrap();
    let max_y = screen.keys().map(|k| k.1).max().unwrap();

    println!("Max X: {} Max Y: {}", max_x, max_y);

    for y in 0..=19 {
        for x in 0..=36 {
            if let Some(block_type) = screen.get(&(x, y)) {
                match block_type {
                    0 => print!(" "),
                    1 => print!("W"),
                    2 => print!("B"),
                    3 => print!("-"),
                    4 => print!("*"),
                    _ => {}
                }
            } else {
                print!("?");
            }
        }
        println!();
    }
    println!("SCORE: {}", screen.get(&(-1, 0)).unwrap());
}

fn find_move(screen: &Screen) -> i64 {
    let (ball_x, _) = screen.iter().find(|e| *e.1 == 4).unwrap_or((&(0, 0), &0)).0;
    let (paddle_x, _) = screen.iter().find(|e| *e.1 == 3).unwrap_or((&(0, 0), &0)).0;

    println!("B: {}, P:{}", ball_x, paddle_x);

    match ball_x.cmp(paddle_x) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut input = input.to_owned();
    input.insert(0, 2);
    let mut pointer = 0;
    let mut relative_index = 0;
    let mut counter = 0;
    let mut screen = HashMap::new();

    loop {
        let joystick = find_move(&screen);
        println!("Move {} --> joystick {}", counter, joystick);

        match run_program(&mut input, pointer, relative_index, vec![joystick]) {
            IntCodeResult::Halt(output) => {
                if 0 != count_blocks(&output) {
                    println!("Hmm, expected no blocks... found {}", count_blocks(&output));
                }
                render_screen(&mut screen, &output);
                return find_score(&output);
            }
            IntCodeResult::Input(new_pointer, new_relative_index, output) => {
                println!(
                    "Remaining blocks: {}, Score: {}",
                    count_blocks(&output),
                    find_score(&output)
                );
                //println!("{:?}", output);
                render_screen(&mut screen, &output);
                pointer = new_pointer;
                relative_index = new_relative_index;
            }
        }
        counter += 1;
    }
}
