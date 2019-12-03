use std::fs;
use std::str;
use std::u32;

pub fn day_1() {
    let input: Vec<_> = fs::read_to_string("input/day1.txt")
        .expect("Could not read file!")
        .lines()
        .map(str::parse::<u32>)
        .map(|x| x.unwrap())
        .collect();

    let solution_1: u32 = input.iter().map(|x| x / 3 - 2).sum();

    println!("The sum of all fuel requirements is: {}", solution_1);

    let solution_2: u32 = input.iter().map(|x| solution_2_fuel(*x)).sum();

    println!(
        "The sum of all fuel requirements including fuel fuel requirements is: {}",
        solution_2
    );
}

fn solution_2_fuel(mass: u32) -> u32 {
    let initial_fuel = mass / 3 - 2;

    let fuel_fuels = std::iter::successors(Some(initial_fuel), |fuel| (fuel / 3).checked_sub(2));

    fuel_fuels.sum()
}
