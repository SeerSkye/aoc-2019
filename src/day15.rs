use crate::day5::intcode;
use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn day_15() {
    let input: Vec<_> = fs::read_to_string("input/day15.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();

    //bfs time, just like day 6

    //each node will be a set of instructions to feed the machine
    let mut node_queue: VecDeque<((i64, i64), intcode::Computer)> = VecDeque::new();
    //we'll also keep track of which nodes we've visited
    let mut nodes_visited: HashSet<(i64, i64)> = HashSet::new();

    //let's populate the initial conditions
    node_queue.push_back(((0, -1), intcode::Computer::new(input.clone(), vec![1])));
    node_queue.push_back(((0, 1), intcode::Computer::new(input.clone(), vec![2])));
    node_queue.push_back(((-1, 0), intcode::Computer::new(input.clone(), vec![3])));
    node_queue.push_back(((1, 0), intcode::Computer::new(input.clone(), vec![4])));

    nodes_visited.insert((0, 0));

    //and keep track of length like day 6
    let mut curr_distance = 1;
    let mut nodes_at_current_distance = 4;
    let mut nodes_at_next_distance = 0;

    let node_at_o2sys;

    loop {
        let candidate = node_queue.pop_front().unwrap();
        nodes_at_current_distance -= 1;

        let mut computer = candidate.1;
        let loc = candidate.0;

        let output = computer.run();
        match output[0]{
            0 => {
                //we hit a wall with this path. Say it's visited but don't do anything more.
                nodes_visited.insert(loc);
            }
            1 => {
                //add this path as visited
                nodes_visited.insert(loc);

                //add the directions off of this
                for i in 1..=4 {
                    let new_loc = match i {
                        1 => (loc.0, loc.1 - 1),
                        2 => (loc.0, loc.1 + 1),
                        3 => (loc.0 - 1, loc.1),
                        4 => (loc.0 + 1, loc.1),
                        _ => panic!("should never happen"),
                    };

                    //only add a path if we haven't visited the location it corresponds to, however.
                    if !nodes_visited.contains(&new_loc) {
                        let mut new_comp = computer.clone();
                        new_comp.receive_input(i);

                        node_queue.push_back((new_loc, new_comp));
                        nodes_at_next_distance += 1
                    }
                }
            }
            2 => {
                //we found it!
                println!(
                    "The least steps to reach the o2 system is: {}",
                    curr_distance
                );
                node_at_o2sys = (loc, computer);
                break;
            }
            _ => println!("Unrecognized output from computer"),
        }

        if nodes_at_current_distance == 0 {
            curr_distance += 1;
            nodes_at_current_distance = nodes_at_next_distance;
            nodes_at_next_distance = 0;
        }
    }

    //part 2: bfs again, but exhaustively looking for the max depth

    //same as before
    let mut o2_node_queue: VecDeque<((i64, i64), intcode::Computer)> = VecDeque::new();
    let mut o2_nodes_visited: HashSet<(i64, i64)> = HashSet::new();

    //reset our counts
    //the tuple in the node represents the location being checked when running then node.
    //having distance as 1 worked for pt 1 cause we broke out of the loop when we found the answer.
    //here we want the distance to the node before checking, not after checking, so we start the count
    //at 0 instead.
    curr_distance = 0; 
    nodes_at_current_distance = 0;
    nodes_at_next_distance = 0;
    let mut curr_dist_has_new_area = false;

    //set up the initial states to check
    for i in 1..=4 {
        let mut new_comp = node_at_o2sys.1.clone();
        let loc = node_at_o2sys.0;

        let new_loc = match i {
            1 => (loc.0, loc.1 - 1),
            2 => (loc.0, loc.1 + 1),
            3 => (loc.0 - 1, loc.1),
            4 => (loc.0 + 1, loc.1),
            _ => panic!("should never happen"),
        };

        new_comp.receive_input(i);

        o2_node_queue.push_back((new_loc, new_comp));
        nodes_at_current_distance += 1
    }

    //mark the current location as visited
    o2_nodes_visited.insert(node_at_o2sys.0);

    while !o2_node_queue.is_empty() {
        let candidate = o2_node_queue.pop_front().unwrap();
        nodes_at_current_distance -= 1;

        let mut computer = candidate.1;
        let loc = candidate.0;

        let output = computer.run();
        match output[0]{
            0 => {
                //we hit a wall with this path. Say it's visited but don't do anything more.
                o2_nodes_visited.insert(loc);
            }
            1 => {
                //add this path as visited
                o2_nodes_visited.insert(loc);
                curr_dist_has_new_area = true;

                //add the directions off of this
                for i in 1..=4 {
                    let new_loc = match i {
                        1 => (loc.0, loc.1 - 1),
                        2 => (loc.0, loc.1 + 1),
                        3 => (loc.0 - 1, loc.1),
                        4 => (loc.0 + 1, loc.1),
                        _ => panic!("should never happen"),
                    };

                    //only add a path if we haven't visited the location it corresponds to, however.
                    if !o2_nodes_visited.contains(&new_loc) {
                        let mut new_comp = computer.clone();
                        new_comp.receive_input(i);

                        o2_node_queue.push_back((new_loc, new_comp));
                        nodes_at_next_distance += 1
                    }
                }
            }
            2 => {
                println!("This shouldn't happen")
            }
            _ => println!("Unrecognized output from computer"),
        }

        if nodes_at_current_distance == 0
        {
            if curr_dist_has_new_area  //we only want to inc if we found new area to fill
            {
                curr_distance += 1;
            }
            nodes_at_current_distance = nodes_at_next_distance;
            nodes_at_next_distance = 0;
            curr_dist_has_new_area = false;
        }
    }

    println!(
        "The time it takes for the o2 to refil is: {}",
        curr_distance
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
