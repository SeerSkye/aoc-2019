use crate::day5::intcode;
use std::collections::{HashSet, VecDeque};
use std::fs;

//runs kind of slow, but was quick to write.
pub fn day_15() {
    let input: Vec<_> = fs::read_to_string("input/day15.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    //bfs time, just like day 6

    //each node will be a set of instructions to feed the machine
    let mut node_queue: VecDeque<Vec<i64>> = VecDeque::new();
    //we'll also keep track of which nodes we've visited
    let mut nodes_visited: HashSet<(i64, i64)> = HashSet::new();

    //let's populate the initial conditions
    node_queue.push_back(vec![1]);
    node_queue.push_back(vec![2]);
    node_queue.push_back(vec![3]);
    node_queue.push_back(vec![4]);

    nodes_visited.insert((0, 0));

    let mut path_to_o2sys = Vec::new();

    while !node_queue.is_empty() {
        let candidate_path = node_queue.pop_front().unwrap();

        let mut computer = intcode::Computer::new(input.clone(), candidate_path.clone());

        let output = computer.run();
        match output.last().unwrap() {
            0 => {
                //we hit a wall with this path. Say it's visited but don't do anything more.
                nodes_visited.insert(get_location(&candidate_path));
            }
            1 => {
                //add this path as visited
                nodes_visited.insert(get_location(&candidate_path));

                //add the directions off of this
                for i in 1..=4 {
                    let mut new_path = candidate_path.clone();
                    new_path.push(i);

                    //only add a path if we haven't visited the location it corresponds to, however.
                    if !nodes_visited.contains(&get_location(&new_path)) {
                        node_queue.push_back(new_path);
                    }
                }
            }
            2 => {
                //we found it!
                println!(
                    "The least steps to reach the o2 system is: {}",
                    candidate_path.len()
                );
                path_to_o2sys = candidate_path;
                break;
            }
            _ => println!("Unrecognized output from computer"),
        }
    }

    //part 2: bfs again, but exhaustively looking for the max depth

    //same as before
    let mut o2_node_queue: VecDeque<Vec<i64>> = VecDeque::new();
    let mut o2_nodes_visited: HashSet<(i64, i64)> = HashSet::new();

    for i in 1..=4 {
        let mut new_path = path_to_o2sys.clone();
        new_path.push(i);

        o2_node_queue.push_back(new_path);
    }

    o2_nodes_visited.insert(get_location(&path_to_o2sys));

    //also keep track of how long our paths are
    let mut max_len = 0;

    while !o2_node_queue.is_empty() {
        let candidate_path = o2_node_queue.pop_front().unwrap();

        let mut computer = intcode::Computer::new(input.clone(), candidate_path.clone());

        let output = computer.run();
        match output.last().unwrap() {
            0 => {
                //we hit a wall with this path. Say it's visited but don't do anything more.
                o2_nodes_visited.insert(get_location(&candidate_path));
            }
            1 => {
                //add this path as visited
                o2_nodes_visited.insert(get_location(&candidate_path));

                //check if this is a new longest path
                if candidate_path.len() > max_len {
                    max_len = candidate_path.len();
                }

                //add the directions off of this
                for i in 1..=4 {
                    let mut new_path = candidate_path.clone();
                    new_path.push(i);

                    //only add a path if we haven't visited the location it corresponds to, however.
                    if !o2_nodes_visited.contains(&get_location(&new_path)) {
                        o2_node_queue.push_back(new_path);
                    }
                }
            }
            2 => println!("This case shouldn't happen."),
            _ => println!("Unrecognized output from computer"),
        }
    }

    println!(
        "The time it takes for the o2 to refil is: {}",
        max_len - path_to_o2sys.len()
    );
}

fn get_location(path: &[i64]) -> (i64, i64) {
    let mut loc = (0, 0);
    for dir in path {
        match dir {
            1 => loc = (loc.0, loc.1 - 1),
            2 => loc = (loc.0, loc.1 + 1),
            3 => loc = (loc.0 - 1, loc.1),
            4 => loc = (loc.0 + 1, loc.1),
            i => panic!("Invalid direction {}", i),
        }
    }

    loc
}
