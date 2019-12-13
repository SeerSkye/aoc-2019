use std::fs;
use crate::day5::intcode;
use std::collections::HashMap;
use std::io;

pub fn day_13 () {
    let input: Vec<_> = fs::read_to_string("input/day13.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    let computer = intcode::Computer::new(input.clone(), Vec::new());
    let mut cabinet = ArcadeCabinet::new(computer);

    cabinet.run();

    println!("The number of blocks when finished is: {}", cabinet.count_blocks());

    let mut modified_input = vec![2];
    modified_input.extend_from_slice(&input[1..input.len()]);

    //A script of inputs that beats the game, made by hand cause breakout is fun!
    let tas_script: Vec<_> = fs::read_to_string("src/day_13_script.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    let modified_computer = intcode::Computer::new(modified_input, tas_script);
    let mut with_quarters = ArcadeCabinet::new(modified_computer);

    with_quarters.run();
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn parse_tile(id: i64) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unrecognized tile!"),
        }
    }

    fn to_char (&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '█',
            Tile::Block => '▒',
            Tile::Paddle => '▔',
            Tile::Ball => '•',
        }
    }
}

struct ArcadeCabinet {
    computer: intcode::Computer,
    board: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl ArcadeCabinet {
    fn new(computer: intcode::Computer) -> ArcadeCabinet {
        ArcadeCabinet {
            computer,
            board: HashMap::new(),
            score: 0,
        }
    }

    fn run(&mut self) {
        while !self.computer.has_halted(){
            let output = self.computer.run();

            for i in (0..output.len()-2).step_by(3) {
                if (output[i], output[i+1]) == (-1, 0) {
                    self.score = output[i+2];
                } else {
                    self.board.insert((output[i], output[i+1]), Tile::parse_tile(output[i+2]));
                }
            }

            self.draw_game();

            if self.computer.is_suspended() {
                self.computer.receive_input(ArcadeCabinet::get_input());
            }
        }
    }

    fn get_input() -> i64 {
        let mut input = String::new();
        loop {
            match io::stdin().read_line(&mut input)
                {
                    Ok(_) => {
                        match input.trim().chars().next() {
                            Some('z') => return -1,
                            Some('x') => return 0,
                            Some('c') => return 1,
                            _ => {
                                println!("Invalid instruction");
                                continue;
                            }
                        }
                    }
                    Err(error) => { 
                        println!("Error: {}", error);
                        continue 
                    }
                }
        }
    }

    fn draw_game(&self) {
        let min_x = self.board.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = self.board.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = self.board.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = self.board.keys().max_by_key(|p| p.1).unwrap().1;

        println!("Score: {}", self.score);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.board.get(&(x, y)) {
                    Some(tile) => print!("{}", tile.to_char()),
                    _ => print!(" "),
                }
            }
            println!();
        }
    }

    fn count_blocks(&self) -> usize {
        self.board.values().filter(|t| **t == Tile::Block).count()
    }
}