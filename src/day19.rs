use std::fs;
use crate::day5::intcode;

pub fn day_19() {
    let input: Vec<_> = fs::read_to_string("input/day19.txt")
        .expect("Could not read file!")
        .split(',')
        .map(str::parse::<i64>)
        .map(|x| x.unwrap())
        .collect();


    let mut num_points = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut computer = intcode::Computer::new(input.clone(), vec![x,y]);

            let output = computer.run();

            if output[0] == 1 {
                num_points += 1;
            }
        }
    }

    println!("The number of points in a 50x50 region is: {}", num_points);

    //part 2

    // last_x is the last starting point of the beam's x coordinate, which will
    //always be <= the start of the beam on the next line.
    let mut last_x = 0;
    for y in 500.. { //start a bit late, mostly to skip over the bits near the start where there's gaps
        for x in last_x.. {
            let mut computer = intcode::Computer::new(input.clone(), vec![x,y]);

            let output = computer.run();

            match output[0] {
                0 => (), //haven't reached the beam yet, do nothing
                1 => {
                    //we hit the beam, mark this x down as the new furthest x
                    last_x = x;

                    //check the bottom right corner
                    let mut br = intcode::Computer::new(input.clone(), vec![x+99,y]);

                    let br_val = br.run()[0];

                    //if the result is 0 the beam is too small on the bottom edge, go to the next line
                    if br_val != 1 {
                        break;
                    } else {
                        //otherwise, check the top corners
                        let mut tl = intcode::Computer::new(input.clone(), vec![x,y-99]);
                        let mut tr = intcode::Computer::new(input.clone(), vec![x+99,y-99]);
    
                        let tl_val = tl.run()[0];
                        let tr_val = tr.run()[0];
    
                        if tl_val == 1 && tr_val == 1 {
                            println!("The solution to part 2 is: {}", x*10_000 + y - 99);
                            return;
                        } else {
                            break;
                        }
                    }
                    
                 } 
                _ => println!("Something unexpected happened"),
            }
        }
    }
}