use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn day_6() {
    let raw_input = fs::read_to_string("input/day6.txt").expect("Could not read file!");

    let input: Vec<Vec<_>> = raw_input.lines().map(|s| s.split(')').collect()).collect();
    let mut orbit_map: HashMap<&str, Vec<&str>> = HashMap::new();

    //make a map where each key returns a list of all nodes that orbit it
    for line in input {
        let centre = line[0];
        let orbiter = line[1];

        orbit_map
            .entry(centre)
            .or_insert_with(Vec::new)
            .push(orbiter)
    }

    let orbit_counts = count_orbits(&orbit_map, "COM");
    println!("The numer of orbits is: {}", orbit_counts);

    let orbit_graph = create_graph(&orbit_map);

    let (parent_of_YOU, _) = orbit_graph.get("YOU").unwrap();
    let (parent_of_SAN, _) = orbit_graph.get("SAN").unwrap();

    let num_transfers = distance_between(&orbit_graph, parent_of_SAN, parent_of_YOU);

    println!(
        "The number of orbital transfers needed is: {}",
        num_transfers
    );
}

fn count_orbits(orbit_map: &HashMap<&str, Vec<&str>>, root: &str) -> u32 {
    //First we make a queue to store the elements we're searching for. We need
    //a queue because we want to search layer by layer (each layer being
    //a set of planets with the same number of direct/indirect orbits)
    let mut orbit_queue = VecDeque::new();
    //The total count of orbits
    let mut count = 0;
    //The layer we are currently on, it starts at zero because COM orbits nothing
    let mut curr_layer_count = 0;
    //there is only 1 object that orbits nothing: COM (as in the problem statement)
    let mut num_in_curr_layer = 1;
    //a counter for layer size, so we know when we move up a layer when popping off
    //the queue
    let mut num_in_next_layer = 0;

    //start our queue with the root node (in this case COM)
    orbit_queue.push_back(root);

    //iterate until we run out of things on the queue, which will happen once we reach the end
    //and run our of objects that are the centers of orbits
    while !orbit_queue.is_empty() {
        let curr_node = orbit_queue.pop_front().unwrap(); //we can call unwrap cause we already checked the node exists

        //add every object that orbits our current node to the back of the queue, and add their orbit counts to the
        //total count
        for orbiter in orbit_map.get(curr_node).unwrap_or(&Vec::new()) {
            orbit_queue.push_back(orbiter);
            num_in_next_layer += 1;
            count += curr_layer_count + 1; //plus one cause they orbit the current node.
                                           //curr_layer_count is the number of orbits the current node has
        }

        num_in_curr_layer -= 1;
        //if we're done with this layer, set up the next layer
        if num_in_curr_layer <= 0 {
            curr_layer_count += 1;
            num_in_curr_layer = num_in_next_layer;
            num_in_next_layer = 0;
        }
    }

    count
}

//create a bi-directional graph by adding each node's parent to the
//map in a tuple with the children list. Now we can walk both ways in the map.
//Each node is guaranteed to only have one parent, as per the problem statement.
fn create_graph<'a>(
    orbit_map: &'a HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, (&'a str, Vec<&'a str>)> {
    let mut orbit_graph: HashMap<&'a str, (&'a str, Vec<&'a str>)> = HashMap::new();
    for (centre, orbiters) in orbit_map {
        for orbiter in orbiters {
            orbit_graph.insert(
                orbiter,
                (
                    centre,
                    orbit_map.get(orbiter).unwrap_or(&Vec::new()).to_vec(),
                ),
            );
        }
    }

    //add the root node too. We'll say it's parent is itself, which should be fine for part 2
    orbit_graph.insert("COM", ("COM", orbit_map.get("COM").unwrap().to_vec()));
    orbit_graph
}

fn distance_between(orbit_graph: &HashMap<&str, (&str, Vec<&str>)>, src: &str, dest: &str) -> u32 {
    //like before we'll set up a queue, this time for bredth first search
    let mut node_queue: VecDeque<&str> = VecDeque::new();
    //We don't want to walk in loops, so we'll keep track of the nodes we've visited
    let mut nodes_visited: HashSet<&str> = HashSet::new();
    //We'll do a similar thing as last time, keeping track of what level of search we're on.
    let mut current_distance = 0;
    let mut nodes_at_current_distance = 1;
    let mut nodes_at_next_distance = 0;

    //put the start node onto the queue
    node_queue.push_back(src);

    while !node_queue.is_empty() {
        let curr_node = node_queue.pop_front().unwrap(); //we can call unwrap cause we already checked the node exists

        //If we've reached our destination, we're done!
        if curr_node == dest {
            return current_distance;
        }

        //otherwise, get the neighbors
        let (parent, children) = orbit_graph.get(curr_node).unwrap();

        // add the parents and children to the queue only if we haven't been there before
        if !nodes_visited.contains(parent) {
            node_queue.push_back(parent);
            nodes_at_next_distance += 1;
        }
        for child in children {
            if !nodes_visited.contains(child) {
                node_queue.push_back(child);
                nodes_at_next_distance += 1;
            }
        }

        //and mark this node as visited
        nodes_visited.insert(curr_node);

        nodes_at_current_distance -= 1;
        //if we're done will all nodes a specific distance away, move to the next distance
        if nodes_at_current_distance <= 0 {
            current_distance += 1;
            nodes_at_current_distance = nodes_at_next_distance;
            nodes_at_next_distance = 0;
        }
    }

    //if we don't find a path, panic
    panic!("No Path Found!!")
}
