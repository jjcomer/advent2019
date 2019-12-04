use anyhow::Result;

fn split_number(password: i32) -> Vec<i8> {
    let str_number = format!("{}", password);
    str_number
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect()
}

fn pair_test(password: &[i8], (i1, i2): &(usize, usize)) -> bool {
    password[*i1] == password[*i2]
}

fn check_pairs(password: &[i8]) -> bool {
    (0..5).zip(1..6).any(|x| pair_test(password, &x))
}

fn ascending_test(password: &[i8], (i1, i2): &(usize, usize)) -> bool {
    password[*i1] <= password[*i2]
}

fn check_ascending(password: &[i8]) -> bool {
    (0..5)
        .zip(1..6)
        .map(|x| ascending_test(password, &x))
        .all(|x| x)
}

pub fn process() -> Result<()> {
    let passwords = (387_638..919_123)
        .map(split_number)
        .filter(|x| check_pairs(x))
        .filter(|x| check_ascending(x))
        .count();

    println!("Found {} passwords", passwords);
    Ok(())
}
