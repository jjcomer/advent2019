use crate::intcode;
use crate::intcode::{run_program, IntCodeResult, Program};
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Program {
    intcode::input_generator(input)
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn change_direction(current_direction: &Direction, turn: i64) -> Direction {
    match turn {
        0 => match current_direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        },
        1 => match current_direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        },
        _ => panic!("Unknown direction {}", turn),
    }
}

enum Colour {
    Black,
    White,
}

type Coord = (i64, i64);
type Map = HashMap<Coord, Colour>;

fn move_bot(current_direction: &Direction, (x, y): &Coord) -> Coord {
    match current_direction {
        Direction::Up => (x.clone(), y + 1),
        Direction::Left => (x - 1, y.clone()),
        Direction::Down => (x.clone(), y - 1),
        Direction::Right => (x + 1, y.clone()),
    }
}

fn check_colour<'a>(map: &'a Map, current_position: &Coord) -> &'a Colour {
    map.get(current_position).unwrap_or(&Colour::Black)
}

fn gen_colour(input: i64) -> Colour {
    match input {
        0 => Colour::Black,
        1 => Colour::White,
        _ => panic!("Unexpected colour {}", input),
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Program) -> usize {
    let mut program = input.to_owned();
    let mut pointer = 0;
    let mut relative_index = 0;
    let mut current_position = (0, 0);
    let mut current_direction = Direction::Up;
    let mut map: Map = HashMap::new();

    loop {
        let current_colour = match check_colour(&map, &current_position) {
            Colour::Black => 0,
            Colour::White => 1,
        };
        let result = run_program(&mut program, pointer, relative_index, vec![current_colour]);

        match result {
            IntCodeResult::Halt(_) => {
                break;
            }
            IntCodeResult::Input(new_pointer, new_relative_index, output) => {
                //println!("Output: {:?}", output);
                let new_colour = gen_colour(output[0]);
                map.insert(current_position, new_colour);
                current_direction = change_direction(&current_direction, output[1]);
                current_position = move_bot(&current_direction, &current_position);
                pointer = new_pointer;
                relative_index = new_relative_index;
            }
        };
    }

    map.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Program) -> usize {
    let mut program = input.to_owned();
    let mut pointer = 0;
    let mut relative_index = 0;
    let mut current_position = (0, 5);
    let mut current_direction = Direction::Up;
    let mut map: Map = HashMap::new();

    map.insert(current_position, Colour::White);

    loop {
        let current_colour = match check_colour(&map, &current_position) {
            Colour::Black => 0,
            Colour::White => 1,
        };
        let result = run_program(&mut program, pointer, relative_index, vec![current_colour]);

        match result {
            IntCodeResult::Halt(_) => {
                //println!("Final output: {:?}", output);
                break;
            }
            IntCodeResult::Input(new_pointer, new_relative_index, output) => {
                //println!("Output: {:?}", output);
                let new_colour = gen_colour(output[0]);
                map.insert(current_position, new_colour);
                current_direction = change_direction(&current_direction, output[1]);
                current_position = move_bot(&current_direction, &current_position);
                pointer = new_pointer;
                relative_index = new_relative_index;
            }
        };
    }

    let max_x = map.keys().map(|x| x.0).max().unwrap();
    let max_y = map.keys().map(|x| x.1).max().unwrap();

    println!("X  {} .. Y {}", max_x, max_y);
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            match check_colour(&map, &(x, y)) {
                Colour::Black => print!(" "),
                Colour::White => print!("X"),
            }
        }
        println!();
    }
    map.len()
}
