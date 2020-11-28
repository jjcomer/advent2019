use std::iter;

#[aoc_generator(day16)]
pub fn generate_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Keep,
    Negate,
    Drop,
}

fn process_digit(signal: &Vec<i32>, position: usize) -> i32 {
    let cycle = iter::repeat(Action::Drop)
        .take(position)
        .chain(iter::repeat(Action::Keep).take(position))
        .chain(iter::repeat(Action::Drop).take(position))
        .chain(iter::repeat(Action::Negate).take(position))
        .cycle()
        .skip(1);
    signal
        .iter()
        .zip(cycle)
        .map(|(n, action)| match action {
            Action::Keep => *n,
            Action::Negate => -n,
            Action::Drop => 0,
        })
        .sum::<i32>()
        .abs()
        % 10
}

fn process(signal: &Vec<i32>) -> Vec<i32> {
    (0..signal.len())
        .map(|i| process_digit(signal, i + 1))
        .collect()
}

#[aoc(day16, part1)]
pub fn solve_part_1(input: &Vec<i32>) -> i32 {
    let result = (0..100).fold(input.clone(), |acc, _| process(&acc));
    let answer: String = result.iter().take(8).map(|d| format!("{}", d)).collect();

    answer.parse().unwrap()
}

#[aoc(day16, part2)]
pub fn solve_part_2(input: &Vec<i32>) -> i32 {
    let input: Vec<i32> = input
        .iter()
        .cycle()
        .take(input.len() * 10000)
        .cloned()
        .collect();

    let result = (0..100).fold(input, |acc, i| {
        println!("Processing: {}", i);
        process(&acc)
    });
    let offset: usize = result
        .iter()
        .take(7)
        .map(|d| format!("{}", d))
        .collect::<String>()
        .parse()
        .unwrap();

    result
        .iter()
        .skip(offset)
        .take(8)
        .map(|d| format!("{}", d))
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = generate_input("12345678");
        let process_1 = process(&input);

        assert_eq!(input, generate_input("12345678"));
        assert_eq!(process_1, generate_input("48226158"));
    }

    #[test]
    fn example() {
        let cases = vec![
            ("80871224585914546619083218645595", 24176176),
            ("19617804207202209144916044189917", 73745418),
            ("69317163492948606335995924319873", 52432133),
        ];

        for (test, result) in cases {
            let input = generate_input(test);
            assert_eq!(result, solve_part_1(&input));
        }
    }
}
