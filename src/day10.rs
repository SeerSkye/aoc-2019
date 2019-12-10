use std::collections::HashSet;
use std::fs;

pub fn day_10() {
    let raw_input = fs::read_to_string("input/day10.txt").expect("Could not read file!");

    let input: Vec<_> = raw_input.lines().collect();

    let width = input[0].len();
    let height = input.len();

    let mut max_count = 0;
    let mut station_x = 0;
    let mut station_y = 0;

    for y in 0..height {
        for x in 0..width {
            if input[y].chars().nth(x).unwrap() == '#' {
                let count = count_asteroids(x, y, &input);
                if count > max_count {
                    max_count = count;
                    station_x = x;
                    station_y = y;
                }
            }
        }
    }

    println!(
        "The max asteroids visible from new station is: {}",
        max_count
    );

    let (vaporized_x, vaporized_y) = nth_vaporizied(station_x, station_y, &input, 200);

    print!(
        "The two hundredth asteroid to be vaporized is at: {}, {}",
        vaporized_x, vaporized_y
    );
}

//make a vec of int tuples in a square spiral, which we'll use for checking.
//this method was stolen from stackexchange
//skips (0, 0)
fn make_spiral(width: usize) -> Vec<(i32, i32)> {
    let mut spiral = Vec::new();
    let mut pos_x = 0;
    let mut pos_y = -1;
    let mut dir_x = 0;
    let mut dir_y = -1;

    for _ in 0..width * width - 1 {
        spiral.push((pos_x, pos_y));

        //if we are at the bottom left, the top right, the bottom right, or 1 past the top left
        if pos_x == -pos_y || (pos_y > 0 && pos_y == pos_x) || (pos_y < 0 && pos_y == pos_x - 1) {
            //turn right
            let tmp = dir_x;
            dir_x = -dir_y;
            dir_y = tmp;
        }

        pos_x += dir_x;
        pos_y += dir_y;
    }

    spiral
}

//this is really slow, especially when run on every asteroid.
fn count_asteroids(x: usize, y: usize, map: &[&str]) -> i32 {
    let mut count = 0;

    let width = map[0].len();
    let height = map.len();

    // we add one cause even spirals are lopsided
    let spiral = make_spiral(width.max(height) * 2 + 1);

    let mut locations_blocked: HashSet<(usize, usize)> = HashSet::new();

    for (dx, dy) in spiral {
        let (loc_x, loc_y) = (dx + x as i32, dy + y as i32);

        //if it's inbounds, not blocked, and not an asteroid:
        if 0 <= loc_x && loc_x < width as i32 
            && 0 <= loc_y && loc_y < height as i32 
            && !locations_blocked.contains(&(loc_x as usize, loc_y as usize))
            && map[loc_y as usize].chars().nth(loc_x as usize).unwrap() == '#'
        {
            count += 1;
            //extend a ray back till it goes out of bounds, marking off all locations as blocked.
            let mut ray_x = loc_x;
            let mut ray_y = loc_y;
            while 0 <= ray_x && ray_x < width as i32 && 0 <= ray_y && ray_y < height as i32 {
                locations_blocked.insert((ray_x as usize, ray_y as usize));
                ray_x += dx / gcd(dx, dy);
                ray_y += dy / gcd(dx, dy);
            }
        }
    }

    count
}

fn nth_vaporizied(station_x: usize, station_y: usize, map: &[&str], n: usize) -> (usize, usize) {
    let num_asteroids: usize = map
        .iter()
        .map(|line| line.chars().filter(|c| *c == '#').count())
        .sum();

    let mut vaporize_count = 0;
    let mut asteroids_vaporized: HashSet<(usize, usize)> = HashSet::new();

    let width = map[0].len();
    let height = map.len();

    let spiral = make_spiral(width.max(height) * 2 + 1);

    while vaporize_count < num_asteroids {
        //each rotation, we keep track of which locations are spared and which are vaporized, using a similar
        //method as pt 1
        let mut locations_spared: HashSet<(usize, usize)> = HashSet::new();
        let mut vaporized_this_cycle: Vec<(usize, usize)> = Vec::new();

        for (dx, dy) in &spiral {
            let (loc_x, loc_y) = (dx + station_x as i32, dy + station_y as i32);

            if 0 <= loc_x && loc_x < width as i32 //in bounds in x
                && 0 <= loc_y && loc_y < height as i32 //in bounds in y
                && !asteroids_vaporized.contains(&(loc_x as usize, loc_y as usize)) //not zapped earlier
                && !locations_spared.contains(&(loc_x as usize, loc_y as usize)) //not a blocked location
                && map[loc_y as usize].chars().nth(loc_x as usize).unwrap() == '#'
            //is an asteroid
            {
                vaporized_this_cycle.push((loc_x as usize, loc_y as usize));

                //extend a ray back till it goes out of bounds, marking off all locations as blocked
                let mut ray_x = loc_x;
                let mut ray_y = loc_y;
                while 0 <= ray_x && ray_x < width as i32 && 0 <= ray_y && ray_y < height as i32 {
                    locations_spared.insert((ray_x as usize, ray_y as usize));
                    ray_x += dx / gcd(*dx, *dy);
                    ray_y += dy / gcd(*dx, *dy);
                }
            }
        }

        //now we have to sort the vaporized asteroids by angle
        vaporized_this_cycle.sort_by(|(x1, y1), (x2, y2)| {
            angle_from_vertical((*x1 as i32 - station_x as i32, *y1 as i32 - station_y as i32))
                .partial_cmp(&angle_from_vertical((
                    *x2 as i32 - station_x as i32,
                    *y2 as i32 - station_y as i32,
                )))
                .unwrap() //we should hopefully not get NaNs from angle_from_vertical
        });

        for asteroid in vaporized_this_cycle {
            asteroids_vaporized.insert(asteroid);
            vaporize_count += 1;

            if vaporize_count == n {
                return asteroid;
            }
        }
    }

    panic!("Ran out of asteroids!");
}

//returns the angle from a point to vertical
fn angle_from_vertical(point: (i32, i32)) -> f64 {
    let (px, py) = point;
    let distance = ((px as f64).powi(2) + (py as f64).powi(2)).sqrt();
    let normal_y = py as f64 / distance;

    let mut angle = (-normal_y).acos();

    //make the angle wrap around
    if px < 0 {
        angle = std::f64::consts::PI * 2.0 - angle;
    }

    angle
}

//apparently this isn't in the standard library. welp.
fn gcd(in_1: i32, in_2: i32) -> i32 {
    let mut a = in_1;
    let mut b = in_2;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a.abs()
}
