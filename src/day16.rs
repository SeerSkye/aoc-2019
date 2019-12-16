use std::fs;
use std::time::Instant;

pub fn day_16() {
    let input: Vec<_> = fs::read_to_string("input/day16.txt")
        .expect("Could not read file!")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut pt_1 = input.clone();

    for _i in 0..100 {
        fft(&mut pt_1, 0);
    }

    print!("The first 8 digits are: ");
    for digit in pt_1.iter().take(8) {
        print!("{}", digit);
    }
    println!();

    //let's just try to solve pt2 naively

    let message_offset_vec: Vec<i32> = input.iter().take(7).rev().copied().collect();
    let mut message_offset = 0;
    for (i, digit) in message_offset_vec.iter().enumerate() {
        message_offset += *digit as usize * 10usize.pow(i as u32);
    }

    let mut pt_2: Vec<i32> = input
        .clone()
        .into_iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(message_offset as usize)
        .collect();

    for _i in 0..100 {
        fft(&mut pt_2, message_offset);
    }

    print!("The pt 2 answer is");
    for digit in pt_2.iter().take(8) {
        print!("{}", digit);
    }
    println!();
}

fn fft(list: &mut Vec<i32>, offset: usize) {
    let time = Instant::now();
    for digit in 0..list.len() {
        let sum: i32 = list[digit..list.len()]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, n)| {
                let cycle = (i + offset) / (digit + offset + 1) % 4;
                if cycle == offset % 4 {
                    acc + n
                } else if cycle == offset % 4 + 2 {
                    acc - n
                } else {
                    acc
                }
            })
            .abs()
            % 10;
        list[digit] = sum;
    }

    println!("FFT took {}s", time.elapsed().as_secs());
}
