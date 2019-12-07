use std::convert::TryInto;

#[aoc_generator(day5)]
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

fn run_program(program: &mut [i32], input: i32) -> i32 {
    let mut pointer = 0;
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
                program[to_index(a)] = input;
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
            99 => return output,
            _ => panic!("Unknown op code {:?}", current_instruction),
        };
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    run_program(&mut input, 1)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    run_program(&mut input, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "3,0,4,0,99";
        assert_eq!(solve_part1(&input_generator(&input)), 1);
    }

    #[test]
    fn example2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_program(&mut input_generator(&input), 8), 1);
        assert_eq!(run_program(&mut input_generator(&input), 7), 0);
    }

    #[test]
    fn example3() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run_program(&mut input_generator(&input), 8), 0);
        assert_eq!(run_program(&mut input_generator(&input), 9), 0);
        assert_eq!(run_program(&mut input_generator(&input), 7), 1);
    }

    #[test]
    fn example4() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_program(&mut input_generator(&input), 8), 1);
        assert_eq!(run_program(&mut input_generator(&input), 7), 0);
    }

    #[test]
    fn example5() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run_program(&mut input_generator(&input), 8), 0);
        assert_eq!(run_program(&mut input_generator(&input), 9), 0);
        assert_eq!(run_program(&mut input_generator(&input), 7), 1);
    }

    #[test]
    fn example6() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_program(&mut input_generator(&input), 8), 1000);
        assert_eq!(run_program(&mut input_generator(&input), 7), 999);
        assert_eq!(run_program(&mut input_generator(&input), 9), 1001);
    }
}
