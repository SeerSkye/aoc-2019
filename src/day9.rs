use std::fs;
use crate::day5::intcode;

pub fn day_9() {
    let input: Vec<_> = fs::read_to_string("input/day9.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    let mut boost_test = intcode::Computer::new(input.clone(), vec![1]);
    let output = boost_test.run().to_vec();

    println!("Running with parameter 1:");
    for line in output {
        println!("{}", line)
    }

    let mut boost_sensor = intcode::Computer::new(input.clone(), vec![2]);
    let output = boost_sensor.run().to_vec();

    println!("Running with parameter 1:");
    for line in output {
        println!("{}", line)
    }
}