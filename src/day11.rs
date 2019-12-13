use std::fs;
use std::collections::HashMap;
use crate::day5::intcode;

pub fn day_11() {
    let input: Vec<_> = fs::read_to_string("input/day11.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    let computer = intcode::Computer::new(input.clone(), Vec::new());

    let mut painter = Painter::new(computer, Colour::Black);

    painter.run();

    println!("The number of squares painted is: {}", painter.get_num_painted());

    let computer = intcode::Computer::new(input.clone(), Vec::new());

    let mut part2_painter = Painter::new(computer, Colour::White);

    part2_painter.run();

    let map = part2_painter.get_map();
    let min_x = map.keys().min_by_key(|p| p.0).unwrap().0;
    let max_x = map.keys().max_by_key(|p| p.0).unwrap().0;
    let min_y = map.keys().min_by_key(|p| p.1).unwrap().1;
    let max_y = map.keys().max_by_key(|p| p.1).unwrap().1;

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(Colour::White) => print!("░"),
                _ => print!("█"),
            }
        }
        println!();
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

enum Colour {
    Black,
    White
}

struct Painter {
    computer: intcode::Computer,
    position: (i32, i32),
    facing: Direction,
    squares_painted: HashMap<(i32, i32), Colour>,
}

impl Painter {
    fn new (computer: intcode::Computer, start_colour: Colour) -> Painter {
        let mut starting_map = HashMap::new();
        starting_map.insert((0, 0), start_colour);
        Painter {
            computer,
            position: (0, 0),
            facing: Direction::North,
            squares_painted: starting_map,
        }
    }

    fn run(&mut self) {
        while !self.computer.has_halted() {
            self.run_step();
        }
    }

    fn get_num_painted (&self) -> usize {
        self.squares_painted.len()
    }

    fn get_map (&self) -> &HashMap<(i32, i32), Colour> {
        &self.squares_painted
    }

    fn run_step(&mut self) {
        if !self.computer.has_halted() {
            match self.squares_painted.get(&self.position) {
                Some(Colour::White) => self.computer.receive_input(1),
                _ => self.computer.receive_input(0),
            }

            let output = self.computer.run();

            //check the first output for the paint instruction
            match output[0] {
                0 => self.squares_painted.insert(self.position, Colour::Black),
                1 => self.squares_painted.insert(self.position, Colour::White),
                c => panic!("{} is not a colour", c),
            };

            //now turn
            match output[1] {
                0 => { 
                    self.facing = match self.facing {
                        Direction::North => Direction::West,
                        Direction::West => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                    }
                },
                1 => { 
                    self.facing = match self.facing {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    }
                }
                d => panic!("{} is not a direction", d),
            }

            //now step forward
            match self.facing {
                Direction::North => self.position = (self.position.0, self.position.1 + 1),
                Direction::East => self.position = (self.position.0 + 1, self.position.1),
                Direction::South => self.position = (self.position.0, self.position.1 - 1),
                Direction::West => self.position = (self.position.0 - 1, self.position.1),
            }
        }
    }
}