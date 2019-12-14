use std::collections::HashMap;
use std::fs;

pub fn day_14() {
    let raw_input = fs::read_to_string("input/day14.txt").expect("Could not read file!");

    let equations: Vec<_> = raw_input.lines().map(|l| Equation::parse(l)).collect();

    let original_eq = equations
        .iter()
        .find(|e| e.rhs.get("FUEL").is_some())
        .unwrap()
        .clone();

    let mut fuel_eq = original_eq.clone();

    reduce(&mut fuel_eq, &equations);

    let ore_for_exactly_1_fuel = fuel_eq.lhs.get("ORE").unwrap();

    println!(
        "The amount of ore needed for exactly 1 fuel is: {}",
        ore_for_exactly_1_fuel
    );

    //for part two, scale the starting equation by some amount, then reduce. Binary search to try and find the
    //exact scaling factor that maximizes fuel under the cap.

    //First, we reduce the equation scaled by the fuel we know we can produce, then use that to generate an initial guess for
    //the amount of additional fuel we can make
    let mut est_eq = original_eq.clone();
    est_eq.scale_eq(1_000_000_000_000 / ore_for_exactly_1_fuel);
    reduce(&mut est_eq, &equations);

    //the ore per fuel here is most likely more accurate than the pt1 solution
    let est_ore_per_fuel = *est_eq.lhs.get("ORE").unwrap() as f64
        / (1_000_000_000_000.0 / *ore_for_exactly_1_fuel as f64) as f64;
    //so use it as the guess
    let mut additional_fuel = ((1_000_000_000_000.0 / est_ore_per_fuel as f64)
        - (1_000_000_000_000.0 / *ore_for_exactly_1_fuel as f64))
        as u64;

    //starting step size for a binary search approach. Our guess is really close, so the starting step is small
    let mut step_size: i64 = 4;

    loop {
        let mut scaled_eq = original_eq.clone();

        let scale_factor = (1_000_000_000_000 / ore_for_exactly_1_fuel) + additional_fuel;
        scaled_eq.scale_eq(scale_factor);

        scaled_eq.add_eq(&original_eq);

        reduce(&mut scaled_eq, &equations);

        if *scaled_eq.lhs.get("ORE").unwrap() > 1_000_000_000_000u64 {
            if step_size == 1 {
                break;
            } else if step_size > 0 {
                step_size /= -2;
            }
        } else if step_size < 0 {
            step_size /= -2;
        }
        additional_fuel = (additional_fuel as i64 + step_size) as u64;
    }

    println!(
        "The max fuel with 1,000,000,000,000 ore is: {}",
        (1_000_000_000_000 / ore_for_exactly_1_fuel) + additional_fuel
    );
}

fn reduce(eq: &mut Equation, eq_list: &[Equation]) {
    while !eq.is_solved() {
        let (key_to_substitute, lhs_amount) = eq.lhs.iter().find(|(k, _)| *k != "ORE").unwrap();
        let substitute_eq = eq_list
            .iter()
            .find(|e| e.rhs.get(key_to_substitute).is_some())
            .unwrap();
        let mut scaled_eq = substitute_eq.clone();
        let rhs_amount = substitute_eq.rhs.get(key_to_substitute).unwrap();

        if lhs_amount > rhs_amount {
            scaled_eq.scale_eq(lhs_amount / rhs_amount);
        }

        eq.add_eq(&scaled_eq);
    }
}

#[derive(Debug, Clone)]
struct Equation {
    lhs: HashMap<String, u64>,
    rhs: HashMap<String, u64>,
}

impl Equation {
    fn parse(input: &str) -> Equation {
        let sides: Vec<_> = input.split(" => ").collect();
        let lhs: Vec<_> = sides[0]
            .split(", ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.split(' ').collect::<Vec<_>>())
            .collect();
        let rhs: Vec<_> = sides[1]
            .split(", ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.split(' ').collect::<Vec<_>>())
            .collect();

        let mut lhs_map = HashMap::new();
        let mut rhs_map = HashMap::new();

        for term in lhs {
            lhs_map.insert(term[1].to_string(), term[0].parse().unwrap());
        }

        for term in rhs {
            rhs_map.insert(term[1].to_string(), term[0].parse().unwrap());
        }

        Equation {
            lhs: lhs_map,
            rhs: rhs_map,
        }
    }

    fn reduce(&mut self) {
        let mut lhs_keys_to_remove = Vec::new();

        for (key, amount) in self.lhs.iter_mut() {
            if let Some(rhs_amount) = self.rhs.get_mut(key) {
                if rhs_amount < amount {
                    *amount -= *rhs_amount;
                    self.rhs.remove(key);
                } else if rhs_amount > amount {
                    *rhs_amount -= *amount;
                    lhs_keys_to_remove.push(key.to_string());
                } else {
                    self.rhs.remove(key);
                    lhs_keys_to_remove.push(key.to_string());
                }
            }
        }

        for key in lhs_keys_to_remove {
            self.lhs.remove(&key);
        }
    }

    fn is_solved(&self) -> bool {
        self.lhs.len() == 1 && self.lhs.get("ORE").is_some() && self.rhs.get("FUEL").is_some()
    }

    fn add_eq(&mut self, other: &Equation) {
        for (key, val) in other.lhs.iter() {
            match self.lhs.get_mut(key) {
                Some(value) => *value += val,
                None => {
                    self.lhs.insert(key.to_string(), *val);
                }
            }
        }

        for (key, val) in other.rhs.iter() {
            match self.rhs.get_mut(key) {
                Some(value) => *value += val,
                None => {
                    self.rhs.insert(key.to_string(), *val);
                }
            }
        }

        self.reduce();
    }

    fn scale_eq(&mut self, scalar: u64) {
        for val in self.lhs.values_mut() {
            *val *= scalar;
        }

        for val in self.rhs.values_mut() {
            *val *= scalar
        }
    }
}
