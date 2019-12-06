fn add_extra_fuel(fuel: i32) -> i32 {
    let mut total_fuel = fuel;
    let mut current_fuel = fuel;
    loop {
        current_fuel = (current_fuel / 3) - 2;
        if current_fuel <= 0 {
            return total_fuel;
        }
        total_fuel += current_fuel;
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().map(|n| (n / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input.iter().map(|n| (n / 3) - 2).map(add_extra_fuel).sum()
}
