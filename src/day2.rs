fn run_program(program: &mut [usize]) {
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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
    let mut input = input.clone();
    input[1] = 12;
    input[2] = 2;

    run_program(&mut input);
    input[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
    for (i, j) in iproduct!(0..99, 0..99) {
        let mut input_copy = input.clone();
        input_copy[1] = i;
        input_copy[2] = j;
        run_program(&mut input_copy);
        if input_copy[0] == 19_690_720 {
            return (i * 100) + j;
        }
    }
    panic!("Unable to find answer")
}
