use std::fs;
use std::str;

pub fn day_2() {
    let input: Vec<_> = fs::read_to_string("input/day2.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<usize>)
        .map(|x| x.unwrap())
        .collect();

    let mut solution_1 = input.clone();
    run_program(&mut solution_1, 12, 2);

    println!(
        "After running the value at position 0 is: {}",
        solution_1[0]
    );

    let goal_output = 19_690_720;

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = input.clone();
            run_program(&mut memory, noun, verb);
            if memory[0] == goal_output {
                println!(
                    "The input {}, {} results in output {}",
                    noun, verb, memory[0]
                );
                println!("The checksum is {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

fn run_program(program: &mut Vec<usize>, noun: usize, verb: usize) {
    let mut pc = 0;

    program[1] = noun;
    program[2] = verb;

    loop {
        match program[pc] {
            1 => {
                let src1 = program[pc + 1];
                let src2 = program[pc + 2];
                let dest = program[pc + 3];
                program[dest] = program[src1] + program[src2];
            }
            2 => {
                let src1 = program[pc + 1];
                let src2 = program[pc + 2];
                let dest = program[pc + 3];
                program[dest] = program[src1] * program[src2];
            }
            99 => break,
            _ => panic!("Unexpected opcode"),
        }
        pc += 4;
    }
}
