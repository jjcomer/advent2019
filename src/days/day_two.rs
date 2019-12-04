use crate::util;
use anyhow::Result;

fn run_program(program: &mut Vec<usize>) {
    let mut pointer = 0;
    loop {
        let current_instruction = program[pointer];
        match current_instruction {
            1 => {
                let pos_a = program[pointer + 1];
                let pos_b = program[pointer + 2];
                let result = program[pointer + 3];
                program[result] = program[pos_a] + program[pos_b];
            }
            2 => {
                let pos_a = program[pointer + 1];
                let pos_b = program[pointer + 2];
                let result = program[pointer + 3];
                program[result] = program[pos_a] * program[pos_b];
            }
            99 => return,
            _ => {}
        };
        pointer += 4;
    }
}

pub fn process() -> Result<()> {
    let input = util::read_input("input/day_two.txt")?;
    let input = input[0]
        .split(",")
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for (i, j) in iproduct!(0..99, 0..99) {
        let mut input_copy = input.clone();
        input_copy[1] = i;
        input_copy[2] = j;
        run_program(&mut input_copy);
        if input_copy[0] == 19690720 {
            println!("Result {}", (i * 100) + j);
            return Ok(());
        }
    }
    println!("Couldn't find the answer");
    Ok(())
}
