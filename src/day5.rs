use std::fs;

pub fn day_5() {
    let input: Vec<_> = fs::read_to_string("input/day5.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i32>)
        .map(|x| x.unwrap())
        .collect();
    let mut computer = intcode::Computer::new(input);
    computer.run();
}

mod intcode {
    use std::io;

    enum Parameter {
        Literal(i32),
        Address(usize),
    }

    impl Parameter {
        fn make_param(mode: i32, val: i32) -> Parameter {
            match mode {
                0 => Parameter::Address(val as usize),
                1 => Parameter::Literal(val),
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
        Halt,
    }

    pub struct Computer {
        memory: Vec<i32>,
        instruction_pointer: usize,
        halt: bool,
    }

    impl Computer {
        pub fn new(initial_memory: Vec<i32>) -> Computer {
            Computer {
                memory: initial_memory,
                instruction_pointer: 0,
                halt: false,
            }
        }

        pub fn run(&mut self) {
            while !self.halt {
                self.run_step();
            }
        }

        fn run_step(&mut self) {
            if self.halt {
                return;
            }

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
                    let mut input = String::new();

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Could not read input!");

                    let parsed_input: i32 =
                        input.trim().parse().expect("Input must be an integer!");

                    self.write_param(parsed_input, p1);
                    self.instruction_pointer += 2
                }
                Opcode::Print(p1) => {
                    println!("{}", self.read_param_value(p1));
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
                99 => Opcode::Halt,
                _ => panic!("Invalid opcode under instruction pointer!"),
            }
        }

        fn read_param_value(&self, param: Parameter) -> i32 {
            match param {
                Parameter::Literal(i) => i,
                Parameter::Address(i) => self.memory[i],
            }
        }

        fn write_param(&mut self, src: i32, dest: Parameter) {
            match dest {
                Parameter::Literal(_) => panic!("Cannot write in immediate mode!!"),
                Parameter::Address(i) => self.memory[i] = src,
            }
        }
    }
}
