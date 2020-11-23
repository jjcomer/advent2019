use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ingredient {
    amount: i64,
    name: String,
}

#[derive(Debug)]
pub struct Recipe {
    result: Ingredient,
    ingredients: Vec<Ingredient>,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^(.*) => (\d+) ([A-Z]+)$").unwrap();
    static ref INGREDIENT_REGEX: Regex = Regex::new(r"^(\d+) ([A-Z]+)$").unwrap();
}

fn parse_line(line: &str) -> Recipe {
    let captures = LINE_REGEX.captures(line).unwrap();
    let result = Ingredient {
        amount: captures[2].parse().unwrap(),
        name: captures[3].to_owned(),
    };
    let ingredients = captures[1]
        .split(", ")
        .map(|element| {
            let captures = INGREDIENT_REGEX.captures(element).unwrap();
            Ingredient {
                amount: captures[1].parse().unwrap(),
                name: captures[2].to_owned(),
            }
        })
        .collect();
    Recipe {
        result,
        ingredients,
    }
}

#[aoc_generator(day14)]
pub fn generate_input(input: &str) -> Vec<Recipe> {
    input.lines().map(parse_line).collect()
}

type Pantry = HashMap<String, i64>;

fn find_needed_ore(
    cook_book: &[Recipe],
    shopping_list: &mut Pantry,
    pantry: &mut Pantry,
    recipe: &Recipe,
    amount: i64,
) {
    let amount = match pantry.entry(recipe.result.name.clone()) {
        Entry::Occupied(mut e) => {
            let store_amount = e.get_mut();
            if *store_amount > 0 {
                let new_amount = amount - *store_amount;
                println!(
                    "Taking from the store {} - {} = {}",
                    amount, store_amount, new_amount
                );
                *store_amount -= amount;
                new_amount
            } else {
                amount
            }
        }
        Entry::Vacant(_) => amount,
    };
    //TODO: Account for overflows
    let batches = if 0 == amount % recipe.result.amount {
        amount / recipe.result.amount
    } else {
        let b = (amount / recipe.result.amount) + 1;
        let overflow = (b * recipe.result.amount) - amount;
        println!("Overflow!!! {} {}", recipe.result.name, overflow);
        pantry
            .entry(recipe.result.name.clone())
            .and_modify(|e| *e += overflow)
            .or_insert(overflow);
        b
    };
    println!("{} -> {} * {}", recipe.result.name, amount, batches);
    let first_ingredient = recipe.ingredients.get(0).unwrap();
    if first_ingredient.name == "ORE" {
        shopping_list
            .entry(recipe.result.name.clone())
            .and_modify(|e| *e += amount)
            .or_insert(amount);
    // println!(
    //     "Added {} {}. Shopping List {:?}",
    //     amount, recipe.result.name, shopping_list
    // );
    } else {
        for ingredient in recipe.ingredients.iter() {
            let missing_recipe = cook_book
                .iter()
                .find(|r| r.result.name == ingredient.name)
                .unwrap();

            find_needed_ore(
                cook_book,
                shopping_list,
                pantry,
                missing_recipe,
                ingredient.amount * batches,
            );
        }
    }
}

fn compute_ore(cook_book: &[Recipe], shopping_list: &Pantry) -> i64 {
    println!("Shopping List: {:?}", shopping_list);
    let mut ore_needed = 0;
    for (ingredient_name, amount) in shopping_list {
        let recipe = cook_book
            .iter()
            .find(|r| *ingredient_name == r.result.name)
            .unwrap();

        let ore_ingredient = recipe.ingredients.get(0).unwrap();
        let batches = if 0 == amount % recipe.result.amount {
            amount / recipe.result.amount
        } else {
            (amount / recipe.result.amount) + 1
        };

        ore_needed += batches * ore_ingredient.amount;
    }
    ore_needed
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Recipe]) -> i64 {
    let fuel_recipe = input.iter().find(|r| r.result.name == "FUEL").unwrap();
    let mut shopping_list = HashMap::new();
    let mut pantry = HashMap::new();
    find_needed_ore(input, &mut shopping_list, &mut pantry, fuel_recipe, 1);
    compute_ore(input, &shopping_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let result = solve_part1(&generate_input(&input));
        assert_eq!(
            result, 31,
            "Expected fuel consumption to be 31 but got {}",
            result
        );
    }

    #[test]
    fn example2() {
        let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        let result = solve_part1(&generate_input(&input));
        assert_eq!(
            result, 165,
            "Expected fuel consumption to be 165 but got {}",
            result
        );
    }

    #[test]
    fn example3() {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let result = solve_part1(&generate_input(&input));
        assert_eq!(
            result, 13312,
            "Expected fuel consumption to be 13312 but got {}",
            result
        );
    }

    #[test]
    fn example4() {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let result = solve_part1(&generate_input(&input));
        assert_eq!(
            result, 2210736,
            "Expected fuel consumption to be 2210736 but got {}",
            result
        );
    }
}
