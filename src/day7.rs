//We're gonna use an external library for the first time today
//Cause I don't feel like implementing permutations
extern crate permutohedron;
use permutohedron::LexicalPermutation;

use crate::day5::intcode; //We can just pull in our intcode from before

use std::fs;

pub fn day_7() {
    let input: Vec<_> = fs::read_to_string("input/day7.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    let mut permutation = vec![0,1,2,3,4];

    let mut highest_signal = 0;

    loop {
        let mut signal = 0;

        for i in &permutation {
            let mut amplifier = intcode::Computer::new(input.clone(), vec![*i, signal]);
            let output = amplifier.run()[0];
            signal = output;
        }

        if signal > highest_signal {
            highest_signal = signal;
        }

        if !permutation.next_permutation() {
            break;
        }
    }

    println!("The highest signal is: {}", highest_signal);

    let mut permutation_2 = vec![5,6,7,8,9];

    let mut highest_signal2 = 0;

    loop {
        let mut computers: Vec<intcode::Computer> = Vec::new();
        let mut signal = 0;

        //set up the computers
        for i in &permutation_2 {
            let mut amplifier = intcode::Computer::new(input.clone(), vec![*i]);
            amplifier.run(); //run them, initiallizing their id and letting them suspend waiting for a signal
            computers.push(amplifier);
        }

        //run the set of 5 computers until one of them halts and is no longer awaiting input
        //we can't run through the loop again if any of them are halted.
        while !computers.iter().any(|c| c.has_halted()) {
            for computer in &mut computers {
                computer.receive_input(signal);
                let output = computer.run();
                signal = *output.last().unwrap(); //the most recent output of a machine is at the back of the output stack.
            }
        }

        if signal > highest_signal2 {
            highest_signal2 = signal
        }

        if !permutation_2.next_permutation() {
            break;
        }
    }

    println!("The highest signal in feedback loop mode is: {}", highest_signal2);
}