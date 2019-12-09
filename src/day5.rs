use std::fs;

pub fn day_5() {
    let input: Vec<_> = fs::read_to_string("input/day5.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();
    let mut computer1 = intcode::Computer::new(input.clone(), vec![1]);
    let part1 = computer1.run();
    println!("The soultion to part 1 is: {}", part1.last().unwrap());

    let mut computer2 = intcode::Computer::new(input.clone(), vec![5]);
    let part2 = computer2.run();
    println!("The soultion to part 2 is: {}", part2.last().unwrap())
}

pub mod intcode {
    enum Parameter {
        Literal(i64),
        Address(usize),
        Relative(i64),
    }

    impl Parameter {
        fn make_param(mode: i64, val: i64) -> Parameter {
            match mode {
                0 => Parameter::Address(val as usize),
                1 => Parameter::Literal(val),
                2 => Parameter::Relative(val),
                _ => panic!("Unrecognized Addressing Mode!"),
            }
        }
    }

    enum Opcode {
        Add(Parameter, Parameter, Parameter),
        Mul(Parameter, Parameter, Parameter),
        Read(Parameter),
        Print(Parameter),
        JumpNonZero(Parameter, Parameter),
        JumpEqualZero(Parameter, Parameter),
        LessThan(Parameter, Parameter, Parameter),
        Equals(Parameter, Parameter, Parameter),
        AdjustRO(Parameter),
        Halt,
    }

    ///An intcode computer. Intcode computers operate according to the specification from the problems in
    /// advent of code 2019. Our implementation also has the ability to suspend itself if it lacks input needed
    /// to continue.
    pub struct Computer {
        memory: Vec<i64>,
        instruction_pointer: usize,
        halt: bool,
        inputs: Vec<i64>,
        outputs: Vec<i64>,
        suspend: bool,
        relative_offset: i64,
    }

    impl Computer {
        ///Initializes a new intcode computer. `memory` is the initial memory state, and `inputs` is the initial
        /// input stack. Remember that the input buffer is a *stack*, meaning the computer will read from the
        /// given buffer from back to front.
        pub fn new(memory: Vec<i64>, inputs: Vec<i64>) -> Computer {
            Computer {
                memory,
                instruction_pointer: 0,
                halt: false,
                inputs,
                outputs: Vec::new(),
                suspend: false,
                relative_offset: 0
            }
        }

        ///Runs an intcode computer until it either suspends itself or halts, returning a reference to its
        /// output buffer. You can poll the state of the machine after it returns control with `has_halted` and
        /// `is_suspended`. Will simply return a reference to the output buffer if it's already halted or suspended.
        pub fn run(&mut self) -> &Vec<i64> {
            while !(self.halt ||self.suspend) {
                self.run_step();
            }

            &self.outputs
        }

        ///Pushes a new value onto the input stack. Remember that if you call this multiple times between run
        /// statements it will read the values in reverse order, as the input is processed as a LIFO stack.
        /// You must call run again to resume execution after pushing a new value onto the stack.
        pub fn receive_input(&mut self, input: i64) {
            self.inputs.push(input);
            self.suspend = false;
        }

        ///Returns whether the intcode machine has halted.
        pub fn has_halted (&self) -> bool {
            self.halt
        }

        ///Returns whether the intcode machine is suspended and awaiting input.
        pub fn is_suspended (&self) -> bool {
            self.suspend
        }

        fn run_step(&mut self) {
            if self.halt {
                return;
            }

            //make sure we have space to run the largest instruction possible.
            self.resize_for_index(self.instruction_pointer + 3);

            match self.parse_opcode() {
                Opcode::Add(p1, p2, p3) => {
                    let sum = self.read_param_value(p1) + self.read_param_value(p2);
                    self.write_param(sum, p3);
                    self.instruction_pointer += 4
                }
                Opcode::Mul(p1, p2, p3) => {
                    let product = self.read_param_value(p1) * self.read_param_value(p2);
                    self.write_param(product, p3);
                    self.instruction_pointer += 4
                }
                Opcode::Read(p1) => {
                    let input = self.inputs.pop();

                    match input {
                        Some(i) => {
                            self.write_param(i, p1);
                            self.instruction_pointer += 2;
                        }
                        None => self.suspend = true
                    }
                }
                Opcode::Print(p1) => {
                    let val = self.read_param_value(p1);
                    self.outputs.push(val);
                    self.instruction_pointer += 2
                }
                Opcode::JumpNonZero(p1, p2) => {
                    if self.read_param_value(p1) != 0 {
                        self.instruction_pointer = self.read_param_value(p2) as usize
                    } else {
                        self.instruction_pointer += 3
                    }
                }
                Opcode::JumpEqualZero(p1, p2) => {
                    if self.read_param_value(p1) == 0 {
                        self.instruction_pointer = self.read_param_value(p2) as usize
                    } else {
                        self.instruction_pointer += 3
                    }
                }
                Opcode::LessThan(p1, p2, p3) => {
                    if self.read_param_value(p1) < self.read_param_value(p2) {
                        self.write_param(1, p3)
                    } else {
                        self.write_param(0, p3)
                    }
                    self.instruction_pointer += 4
                }
                Opcode::Equals(p1, p2, p3) => {
                    if self.read_param_value(p1) == self.read_param_value(p2) {
                        self.write_param(1, p3)
                    } else {
                        self.write_param(0, p3)
                    }
                    self.instruction_pointer += 4
                }
                Opcode::AdjustRO(p1) => {
                    self.relative_offset = self.read_param_value(p1) + self.relative_offset as i64;
                    self.instruction_pointer += 2;
                }
                Opcode::Halt => self.halt = true,
            }
        }

        fn parse_opcode(&self) -> Opcode {
            //parses the opcode under the instruction pointer.
            match self.memory[self.instruction_pointer] % 100 {
                1 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;
                    let mode_p3 = (self.memory[self.instruction_pointer] / 10000) % 10;

                    Opcode::Add(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                        Parameter::make_param(mode_p3, self.memory[self.instruction_pointer + 3]),
                    )
                }
                2 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;
                    let mode_p3 = (self.memory[self.instruction_pointer] / 10000) % 10;

                    Opcode::Mul(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                        Parameter::make_param(mode_p3, self.memory[self.instruction_pointer + 3]),
                    )
                }
                3 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;

                    Opcode::Read(Parameter::make_param(
                        mode_p1,
                        self.memory[self.instruction_pointer + 1],
                    ))
                }
                4 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;

                    Opcode::Print(Parameter::make_param(
                        mode_p1,
                        self.memory[self.instruction_pointer + 1],
                    ))
                }
                5 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;

                    Opcode::JumpNonZero(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                    )
                }
                6 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;

                    Opcode::JumpEqualZero(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                    )
                }
                7 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;
                    let mode_p3 = (self.memory[self.instruction_pointer] / 10000) % 10;

                    Opcode::LessThan(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                        Parameter::make_param(mode_p3, self.memory[self.instruction_pointer + 3]),
                    )
                }
                8 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;
                    let mode_p2 = (self.memory[self.instruction_pointer] / 1000) % 10;
                    let mode_p3 = (self.memory[self.instruction_pointer] / 10000) % 10;

                    Opcode::Equals(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1]),
                        Parameter::make_param(mode_p2, self.memory[self.instruction_pointer + 2]),
                        Parameter::make_param(mode_p3, self.memory[self.instruction_pointer + 3]),
                    )
                }
                9 => {
                    let mode_p1 = (self.memory[self.instruction_pointer] / 100) % 10;

                    Opcode::AdjustRO(
                        Parameter::make_param(mode_p1, self.memory[self.instruction_pointer + 1])
                    )
                }
                99 => Opcode::Halt,
                _ => panic!("Invalid opcode under instruction pointer!"),
            }
        }

        ///Resizes memory if there's not enough space.
        fn resize_for_index(&mut self, index: usize) {
            if self.memory.len() < index + 2 {
                self.memory.resize(index + 2, 0)
            }
        }

        fn read_param_value(&mut self, param: Parameter) -> i64 {
            match param {
                Parameter::Literal(i) => i,
                Parameter::Address(i) => {
                    self.resize_for_index(i);
                    self.memory[i]
                }
                Parameter::Relative(i) => {
                    let index = self.relative_offset + i;
                    assert!(index >= 0); //crash if index is bad. at some point proper error handling would be nice
                    self.resize_for_index(index as usize);
                    self.memory[index as usize]
                }
            }
        }

        fn write_param(&mut self, src: i64, dest: Parameter) {
            match dest {
                Parameter::Literal(_) => panic!("Cannot write in immediate mode!!"),
                Parameter::Address(i) => {
                    self.resize_for_index(i);
                    self.memory[i] = src
                }
                Parameter::Relative(i) => {
                    let index = self.relative_offset + i;
                    assert!(index >= 0); //crash if index is bad. at some point proper error handling would be nice
                    self.resize_for_index(index as usize);
                    self.memory[index as usize] = src
                }
            }
        }
    }
}
