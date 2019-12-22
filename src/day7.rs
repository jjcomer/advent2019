use permute::permutations_of;
use std::convert::TryInto;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

fn parse_instruction(params: i32) -> Vec<i32> {
    format!("{:05}", params)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn get_value(value_type: i32, i: i32, program: &[i32]) -> i32 {
    if value_type == 1 {
        //println!("Direct {}", i);
        program[to_index(i)]
    } else {
        let lookup_index = to_index(program[to_index(i)]);
        //println!("Indirect {} -> {}", i, lookup_index);
        program[lookup_index]
    }
}

fn to_index(i: i32) -> usize {
    i.try_into().unwrap()
}

enum IntCodeResult {
    Halt(i32),
    Input(i32, i32),
}

fn run_program(program: &mut [i32], pointer: i32, mut input: Vec<i32>) -> IntCodeResult {
    let mut pointer = pointer;
    let mut output = 0;
    loop {
        let current_instruction = parse_instruction(program[to_index(pointer)]);
        //println!("Command {} {:?}", pointer, current_instruction);
        let opcode = (current_instruction[3] * 10) + current_instruction[4];
        match opcode {
            1 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                let b = get_value(current_instruction[1], pointer + 2, program);
                let r = program[to_index(pointer + 3)];
                //println!("ADD {} + {} = {} WRITE TO {}", a, b, a + b, r);
                program[to_index(r)] = a + b;
                pointer += 4;
            }
            2 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                let b = get_value(current_instruction[1], pointer + 2, program);
                let r = program[to_index(pointer + 3)];
                //println!("MULTIPLY {} + {} = {} WRITE TO {}", a, b, a * b, r);
                program[to_index(r)] = a * b;
                pointer += 4;
            }
            3 => {
                let a = program[to_index(pointer + 1)];
                program[to_index(a)] = input.pop().unwrap();
                pointer += 2;
            }
            4 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                output = a;
                //println!("Output: {} {} {:?}", pointer + 1, a, current_instruction);
                pointer += 2;
            }
            5 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                if a != 0 {
                    let b = get_value(current_instruction[1], pointer + 2, program);
                    pointer = b;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                if a == 0 {
                    let b = get_value(current_instruction[1], pointer + 2, program);
                    pointer = b;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                let b = get_value(current_instruction[1], pointer + 2, program);
                let r = program[to_index(pointer + 3)];
                if a < b {
                    program[to_index(r)] = 1
                } else {
                    program[to_index(r)] = 0
                }
                pointer += 4
            }
            8 => {
                let a = get_value(current_instruction[2], pointer + 1, program);
                let b = get_value(current_instruction[1], pointer + 2, program);
                let r = program[to_index(pointer + 3)];
                if a == b {
                    program[to_index(r)] = 1
                } else {
                    program[to_index(r)] = 0
                }
                pointer += 4
            }
            99 => return IntCodeResult::Halt(output),
            _ => panic!("Unknown op code {:?}", current_instruction),
        };
    }
}

fn run_series<'a>(program: Vec<i32>, series: impl Iterator<Item = &'a i32>) -> i32 {
    let mut output = 0;
    for phase in series {
        match run_program(&mut program.clone(), 0, vec![output, *phase]) {
            IntCodeResult::Halt(o) => output = o,
            IntCodeResult::Input(_, _) => (),
        }
    }
    output
}

fn run_series_2<'a>(program: Vec<i32>, series: impl Iterator<Item = &'a i32>) -> i32 {
    0
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let series = (0..5).collect::<Vec<i32>>();
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
