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

fn check_long_pairs(password: &[i8]) -> bool {
    let mut count = 1;
    let mut last = -1;

    for p in password {
        if last == *p {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        last = *p;
    }
    count == 2
}

fn check_ascending(password: &[i8]) -> bool {
    password
        .iter()
        .scan(-1 as i8, |state, x| {
            if *state <= *x {
                *state = *x;
                Some(true)
            } else {
                Some(false)
            }
        })
        .all(|x| x)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (i32, i32) {
    let numbers = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (numbers[0], numbers[1])
}

#[aoc(day4, part1)]
pub fn solve_part1((low, high): &(i32, i32)) -> usize {
    (*low..*high)
        .map(|x| split_number(x))
        .filter(|x| check_pairs(x))
        .filter(|x| check_ascending(x))
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2((low, high): &(i32, i32)) -> usize {
    (*low..*high)
        .map(|x| split_number(x))
        .filter(|x| check_long_pairs(x))
        .filter(|x| check_ascending(x))
        .count()
}
