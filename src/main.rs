use anyhow::Result;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;

fn main() -> Result<()> {
    println!("Advent of code!!!");
    let mut day_choice = String::new();
    println!("Which day should I run?  ");
    std::io::stdin().read_line(&mut day_choice).unwrap();

    match day_choice.trim().parse::<u16>()? {
        1 => days::day_one::process(),
        2 => days::day_two::process(),
        3 => days::day_three::process(),
        _ => {
            println!("Unknown day: {}", day_choice);
            Ok(())
        }
    }
}

mod days;
pub mod util;
