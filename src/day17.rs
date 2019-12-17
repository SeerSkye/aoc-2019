use std::fs;
use crate::day5::intcode;
use std::collections::HashSet;

pub fn day_17() {
    let input: Vec<_> = fs::read_to_string("input/day17.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();
    
    let mut computer = intcode::Computer::new(input.clone(), Vec::new());

    let board_vec = computer.run();

    //this board is pretty sparse looking, let's convert it to a map
    let mut board: HashSet<Position> = HashSet::new();

    for (y, line) in board_vec.split(|n| *n == 10).enumerate() {
        for (x, loc) in line.iter().enumerate() {
            match std::char::from_u32(*loc as u32).unwrap_or('?') {
                '#' | '^' | '<' | '>' | 'v' => {
                    board.insert((x,y));
                }
                '.' => (),
                c => println!("Unrecognized character {}", c),
            }
        }
    }

    //now let's solve part 1
    let mut sum_of_align = 0;
    for (x, y) in board.iter() {
        //if every adjacent spot is scaffold.
        if *x > 0 && *y > 0
            &&board.contains(&(x+1, *y)) 
            && board.contains(&(x-1, *y))
            && board.contains(&(*x, y-1))
            && board.contains(&(*x, y+1))
        {
            sum_of_align += x * y;
        }
    }

    println!("The sum of the alignment parameters is: {}", sum_of_align);

    //I gave up on part 2 algorithmically and solved with manual inspection
    let fn_a = "L,6,R,8,L,4,R,8,L,12";
    let fn_b = "L,12,R,10,L,4";
    let fn_c = "L,12,L,6,L,4,L,4";
    let overall = "A,B,B,C,B,C,B,C,A,A";

    let input_str: String = format!("{}\n{}\n{}\n{}\nn\n", overall, fn_a, fn_b, fn_c);
    let input_ints: Vec<i64> = input_str.into_bytes().into_iter().map(|i| i as i64).collect();

    let mut new_program = Vec::with_capacity(input.len());
    new_program.push(2);
    new_program.extend_from_slice(&input[1..input.len()]);

    let mut pt_2_computer = intcode::Computer::new(new_program, input_ints);

    let output = pt_2_computer.run();

    println!("The final output for pt 2 is: {}", output.last().unwrap());
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_pair(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn get_side_dirs(&self) -> (Direction, Direction) {
        match self {
            Direction::Up => (Direction::Left, Direction::Right),
            Direction::Down => (Direction::Right, Direction::Left),
            Direction::Left => (Direction::Down, Direction::Up),
            Direction::Right => (Direction::Up, Direction::Down),
        }
    }
}