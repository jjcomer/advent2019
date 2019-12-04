use crate::util;
use anyhow::Result;

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

pub fn process() -> Result<()> {
    let input = util::read_input("input/day_one.txt")?;
    let result: i32 = input
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| (n / 3) - 2)
        .map(add_extra_fuel)
        .sum();

    println!("Fuel required: {}", result);
    Ok(())
}
