use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::{Add, Sub};

pub fn day_18() {
    let input = fs::read_to_string("input/day18.txt").expect("Could not read file!");

    let board = Board::board_from_str(&input);

    let graph = Graph::from_board(&board);

    println!(
        "The solution to part 1 is: {}",
        graph.solve(&[SquareType::StartOne])
    );

    let pt_2_in = fs::read_to_string("input/day18_pt2.txt").expect("Could not read file!");

    let pt_2_board = Board::board_from_str(&pt_2_in);

    let pt_2_graph = Graph::from_board(&pt_2_board);

    println!(
        "The solution to part 2 is: {}",
        pt_2_graph.solve(&[
            SquareType::StartOne,
            SquareType::StartTwo,
            SquareType::StartThree,
            SquareType::StartFour
        ])
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point(i32, i32);

impl Point {
    fn manhattan_distace(self, other: Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

//we won't keep track of walls, everything in our hashmap will
//be one of these.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum SquareType {
    Empty,
    StartOne,
    StartTwo,
    StartThree,
    StartFour,
    Key(char),
    Door(char),
}

impl SquareType {
    fn is_key(self) -> bool {
        match self {
            SquareType::Key(_) => true,
            _ => false,
        }
    }
}

impl SquareType {
    fn from_char(c: char) -> Option<SquareType> {
        if c == '.' {
            Some(SquareType::Empty)
        //to make it easy to track different starts I gave each an enum key, and
        //when I edited my input I used different characters instead of 4 '@'s
        } else if c == '@' || c == '1' {
            Some(SquareType::StartOne)
        } else if c == '2' {
            Some(SquareType::StartTwo)
        } else if c == '3' {
            Some(SquareType::StartThree)
        } else if c == '4' {
            Some(SquareType::StartFour)
        } else if c.is_ascii_alphabetic() {
            if c.is_ascii_lowercase() {
                Some(SquareType::Key(c))
            } else if c.is_ascii_uppercase() {
                Some(SquareType::Door(c.to_ascii_lowercase()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

struct Board(HashMap<Point, SquareType>);

impl Board {
    fn board_from_str(input: &str) -> Board {
        let mut board = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(t) = SquareType::from_char(c) {
                    board.insert(Point(x as i32, y as i32), t);
                }
            }
        }

        Board(board)
    }

    fn nodes_and_distances_from_point (&self, start: Point) -> Vec<Edge> {
        let mut edges = Vec::new();
        let mut node_queue: VecDeque<Point> = VecDeque::new();
        node_queue.push_back(start + Point(0, -1));
        node_queue.push_back(start + Point(0, 1));
        node_queue.push_back(start + Point(1, 0));
        node_queue.push_back(start + Point(-1, 0));

        let mut visited_nodes = HashSet::new();
        visited_nodes.insert(start);

        //similar bfs as I've been doing a lot this AoC
        let mut curr_distance = 1;
        let mut nodes_at_curr_distance = 4;
        let mut nodes_at_next_distance = 0;

        while !node_queue.is_empty() {
            let curr_node = node_queue.pop_front().unwrap();
            visited_nodes.insert(curr_node);

            if let Some(ty) = self.0.get(&curr_node) {
                match ty {
                    SquareType::Empty => {
                        if !visited_nodes.contains(&(curr_node + Point(0, -1))) {
                            node_queue.push_back(curr_node + Point(0, -1));
                            nodes_at_next_distance += 1
                        }
                        if !visited_nodes.contains(&(curr_node + Point(0, 1))) {
                            node_queue.push_back(curr_node + Point(0, 1));
                            nodes_at_next_distance += 1
                        }
                        if !visited_nodes.contains(&(curr_node + Point(1, 0))) {
                            node_queue.push_back(curr_node + Point(1, 0));
                            nodes_at_next_distance += 1
                        }
                        if !visited_nodes.contains(&(curr_node + Point(-1, 0))) {
                            node_queue.push_back(curr_node + Point(-1, 0));
                            nodes_at_next_distance += 1
                        }
                    }
                    t => edges.push(Edge{key: *t, dist: curr_distance}),
                }
            }

            nodes_at_curr_distance -= 1;
            if nodes_at_curr_distance == 0 {
                curr_distance += 1;
                nodes_at_curr_distance = nodes_at_next_distance;
                nodes_at_next_distance = 0;
            }
        }

        edges
    }
}

#[derive(Debug)]
struct Edge {
    key: SquareType,
    dist: i32,
}

#[derive(Debug)]
struct Graph {
    graph: HashMap<SquareType, Vec<Edge>>,
}

impl Graph {
    //assumes that board only has 1 of each unique type
    fn from_board(board: &Board) -> Graph {
        let mut graph: HashMap<SquareType, Vec<Edge>> = HashMap::new();

        let mut ne_nodes: Vec<_> = board
            .0
            .iter()
            .filter(|(_, &v)| v != SquareType::Empty)
            .collect();

        while !ne_nodes.is_empty() {
            let (&curr_pos, &curr_key) = ne_nodes.pop().unwrap();

            let edges = board.nodes_and_distances_from_point(curr_pos);

            graph.insert(curr_key, edges);
        }

        Graph { graph }
    }

    fn solve(&self, start_nodes: &[SquareType]) -> i32 {
        let mut priority_queue: BinaryHeap<SearchNode> = BinaryHeap::new();
        priority_queue.push(Reverse((0, start_nodes.to_vec(), start_nodes.to_vec())));

        //nodes_visited keeps track of the a node + keyset pair, so we don't spin in circles
        //too much.
        let mut nodes_visited: HashSet<(SquareType, Vec<SquareType>)> = HashSet::new();

        //the number of keys to collect. when we get them all we're done!
        let num_keys = self.graph.keys().filter(|k| k.is_key()).count();

        while !priority_queue.is_empty() {
            let Reverse((curr_dist, curr_path, curr_nodes)) = priority_queue.pop().unwrap();

            //add all neighbor nodes to the priority queue
            for (i, curr_node) in curr_nodes.iter().enumerate() {
                for Edge { key, dist } in self.graph.get(&curr_node).unwrap_or(&Vec::new()) {
                    match key {
                        //if we don't have a matching key, visiting doors is useless
                        SquareType::Door(c) if !curr_path.contains(&SquareType::Key(*c)) => {
                            continue
                        }
                        _ => {
                            let mut key_list: Vec<SquareType> =
                                curr_path.iter().filter(|k| k.is_key()).copied().collect();
                            key_list.sort();
                            key_list.dedup();

                            if key.is_key() && !key_list.contains(key) {
                                key_list.push(*key);
                            }

                            if key_list.len() == num_keys {
                                // we finished!!
                                return curr_dist + dist;
                            }

                            let new_node_dist = curr_dist + dist;
                            let mut new_path = curr_path.clone();
                            new_path.push(*key);
                            let mut new_curr_nodes = curr_nodes.clone();
                            new_curr_nodes[i] = *key;

                            let new_node = (new_node_dist, new_path, new_curr_nodes);

                            if nodes_visited.insert((*key, key_list)) {
                                priority_queue.push(Reverse(new_node));
                            }
                        }
                    }
                }
            }
        }

        panic!("Failed to find a solution!!")
    }
}

//A search node has the distance, a path, and a list of nodes we can use to extend the path
type SearchNode = Reverse<(i32, Vec<SquareType>, Vec<SquareType>)>;
