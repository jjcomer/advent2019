use std::collections::HashMap;
use std::convert::TryInto;

fn parse_instruction(params: i64) -> Vec<i64> {
    format!("{:05}", params)
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn get_value(value_type: i64, i: i64, program: &Program, relative_base: i64) -> i64 {
    //println!("LOOKUP: {} {} {}", value_type, i, relative_base);
    match value_type {
        1 => *program.get(&to_index(i)).unwrap_or(&0),
        2 => {
            let relative_offset = *program.get(&to_index(i)).unwrap_or(&0);
            // println!(
            //     "RELATIVE: {} {} {}",
            //     relative_base,
            //     relative_offset,
            //     relative_base + relative_offset
            // );

            *program
                .get(&to_index(relative_offset + relative_base))
                .unwrap_or(&0)
        }
        0 => {
            let lookup_index = to_index(*program.get(&to_index(i)).unwrap_or(&0));
            *program.get(&lookup_index).unwrap_or(&0)
        }
        _ => panic!("Unknown value type: {}", value_type),
    }
}

fn get_index(value_type: i64, i: i64, program: &Program, relative_base: i64) -> usize {
    match value_type {
        2 => {
            let relative_offset = *program.get(&to_index(i)).unwrap_or(&0);
            // println!(
            //     "RELATIVE INDEX: {} {} {}",
            //     relative_base,
            //     relative_offset,
            //     relative_base + relative_offset
            // );
            to_index(relative_offset + relative_base)
        }
        0 => to_index(*program.get(&to_index(i)).unwrap_or(&0)),
        _ => panic!("Unknown value type: {}", value_type),
    }
}

fn to_index(i: i64) -> usize {
    i.try_into().unwrap()
}

pub type Program = HashMap<usize, i64>;
pub type Output = Vec<i64>;

pub enum IntCodeResult {
    Halt(Output),
    Input(i64, Output),
}

pub fn run_program(program: &mut Program, pointer: i64, mut input: Vec<i64>) -> IntCodeResult {
    let mut pointer = pointer;
    let mut output = Vec::new();
    let mut relative_base = 0;
    loop {
        let current_instruction = parse_instruction(program[&to_index(pointer)]);
        // println!(
        //     "Command {} {} {:?}",
        //     pointer, relative_base, current_instruction
        // );
        let opcode = (current_instruction[3] * 10) + current_instruction[4];
        match opcode {
            1 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                let r = get_index(current_instruction[0], pointer + 3, program, relative_base);
                //println!("ADD {} + {} = {} WRITE TO {}", a, b, a + b, r);
                program.insert(r, a + b);
                pointer += 4;
            }
            2 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                let r = get_index(current_instruction[0], pointer + 3, program, relative_base);
                //println!("MULTIPLY {} + {} = {} WRITE TO {}", a, b, a * b, r);
                program.insert(r, a * b);
                pointer += 4;
            }
            3 => {
                let a = get_index(current_instruction[2], pointer + 1, program, relative_base);
                //println!("INPUT {:?} {:?} {}", current_instruction, input, a);
                if let Some(to_insert) = input.pop() {
                    program.insert(a, to_insert);
                    pointer += 2;
                } else {
                    return IntCodeResult::Input(pointer, output);
                }
            }
            4 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                output.push(a);
                //println!("Output: {} {} {:?}", pointer + 1, a, current_instruction);
                pointer += 2;
            }
            5 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                if a != 0 {
                    let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                    pointer = b;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                if a == 0 {
                    let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                    pointer = b;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                let r = get_index(current_instruction[0], pointer + 3, program, relative_base);
                if a < b {
                    program.insert(r, 1);
                } else {
                    program.insert(r, 0);
                }
                pointer += 4
            }
            8 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                let b = get_value(current_instruction[1], pointer + 2, program, relative_base);
                let r = get_index(current_instruction[0], pointer + 3, program, relative_base);
                //println!("EQUALITY: {} {} {}", a, b, r);
                if a == b {
                    program.insert(r, 1);
                } else {
                    program.insert(r, 0);
                }
                pointer += 4
            }
            9 => {
                let a = get_value(current_instruction[2], pointer + 1, program, relative_base);
                //println!("SHIFT BASE {} {}", relative_base, a);
                relative_base += a;
                pointer += 2;
            }
            99 => return IntCodeResult::Halt(output),
            _ => panic!("Unknown op code {:?}", current_instruction),
        };
    }
}

pub fn input_generator(input: &str) -> Program {
    input
        .split(',')
        .map(|d| d.parse::<i64>().unwrap())
        .enumerate()
        .fold(HashMap::new(), |mut acc, instruction| {
            let (index, instruction) = instruction;
            acc.insert(index, instruction);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let expected = input
            .split(',')
            .map(|d| d.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut program = input_generator(&input);
        match run_program(&mut program, 0, Vec::new()) {
            IntCodeResult::Halt(output) => assert_eq!(expected, output),
            _ => panic!("Unexpected Input Request"),
        }
    }

    #[test]
    fn example2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = input_generator(&input);
        match run_program(&mut program, 0, Vec::new()) {
            IntCodeResult::Halt(output) => {
                let str_num = format!("{}", output[0]);
                assert_eq!(16, str_num.len());
            }
            _ => panic!("Unexpected Input Request"),
        }
    }

    #[test]
    fn example3() {
        let input = "104,1125899906842624,99";
        let expected = vec![1125899906842624];
        let mut program = input_generator(&input);
        match run_program(&mut program, 0, Vec::new()) {
            IntCodeResult::Halt(output) => assert_eq!(expected, output),
            _ => panic!("Unxepected Input Request"),
        }
    }
}
