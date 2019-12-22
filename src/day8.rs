#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn compare_images(a: &&[i32], b: &&[i32]) -> std::cmp::Ordering {
    let a_ones = a.into_iter().filter(|x| **x == 0).count();
    let b_ones = b.into_iter().filter(|x| **x == 0).count();

    a_ones.cmp(&b_ones)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[i32]) -> usize {
    let input = input.to_owned();
    let mut layers = Vec::new();
    let layer_size = 6 * 25;
    let layer_count = input.len() / layer_size;
    for x in 0..layer_count {
        let start = x * layer_size;
        layers.push(&input[start..start + layer_size]);
    }
    layers.sort_by(compare_images);
    let biggest = layers.first().unwrap();
    let ones = biggest.into_iter().filter(|x| **x == 1).count();
    let twos = biggest.into_iter().filter(|x| **x == 2).count();

    ones * twos
}

fn gen_pixel(input: &[i32], index: usize, layer_size: usize, layer_count: usize) -> i32 {
    for i in 0..layer_count {
        let x = input[index + (i * layer_size)];
        if x != 2 {
            return x;
        }
    }
    2
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[i32]) -> usize {
    let layer_size = 6 * 25;
    let layer_count = input.len() / layer_size;

    print!("\n");

    for x in 0..layer_size {
        let pixel = gen_pixel(input, x, layer_size, layer_count);

        if pixel == 1 {
            print!("{}", pixel);
        } else {
            print!(" ");
        }
        if (x + 1) % 25 == 0 {
            print!("\n");
        }
    }
    print!("\n");
    0
}
