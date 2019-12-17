use std::collections::HashMap;
use std::fs;
use std::time::Instant;

pub fn day_16() {
    let input: Vec<_> = fs::read_to_string("input/day16.txt")
        .expect("Could not read file!")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut pt_1 = input.clone();

    let mut memoized_indexes: HashMap<(usize, usize), (Vec<usize>, Vec<usize>)> = HashMap::new();

    for _i in 0..100 {
        fft(&mut pt_1, 0, &mut memoized_indexes);
    }

    print!("The first 8 digits are: ");
    for digit in pt_1.iter().take(8) {
        print!("{}", digit);
    }
    println!();

    let message_offset_vec: Vec<i64> = input.iter().take(7).rev().copied().collect();
    let mut message_offset = 0;
    for (i, digit) in message_offset_vec.iter().enumerate() {
        message_offset += *digit as usize * 10usize.pow(i as u32);
    }

    let mut pt_2: Vec<i64> = input
        .clone()
        .into_iter()
        .cycle()
        .take(input.len() * 10000)
        .skip(message_offset as usize)
        .collect();

    //make the table in advance

    for i in 0..pt_2.len() {
        get_nontrivial_indexes_affected(i + message_offset, message_offset, &mut memoized_indexes);
    }

    println!("Offsets built");

    for _i in 0..100 {
        fft(&mut pt_2, message_offset, &mut memoized_indexes);
    }

    print!("The pt 2 answer is ");
    for digit in pt_2.iter().take(8) {
        print!("{}", digit);
    }
    println!();
}

fn fft(
    list: &mut Vec<i64>,
    offset: usize,
    mut memoize_table: &mut HashMap<(usize, usize), (Vec<usize>, Vec<usize>)>,
) {
    //force list to be a power of two size.
    let timer = Instant::now();
    let orig_len = list.len();

    if (orig_len & (orig_len - 1)) != 0 {
        let mut next_power_of_two = 1;
        while next_power_of_two < list.len() {
            next_power_of_two *= 2;
        }
        list.resize(next_power_of_two, 0);
    }

    fft_helper(list, offset, 0, &mut memoize_table);

    for digit in list.iter_mut() {
        *digit = digit.abs() % 10;
    }

    println!("FFT Took: {}", timer.elapsed().as_secs_f32());

    list.resize(orig_len, 0);
}

fn fft_helper(
    work_list: &mut [i64],
    orig_offset: usize,
    offset: usize,
    mut memoize_table: &mut HashMap<(usize, usize), (Vec<usize>, Vec<usize>)>,
) {
    //are we done?
    if work_list.len() <= 1 {
        return;
    }

    //println!("slice at start: {:?}", work_list);

    let (first_half, last_half) = work_list.split_at_mut(work_list.len() / 2);
    //first, fft the first half
    fft_helper(first_half, orig_offset, offset, &mut memoize_table);

    //then, get the hard to calc parts
    for digit in 0..last_half.len() {
        let (indexes_0, indexes_2) = get_nontrivial_indexes_affected(
            orig_offset + offset + first_half.len() + digit,
            orig_offset,
            &mut memoize_table,
        );

        for i in indexes_0 {
            if let Some(val) = first_half.get_mut(i - offset - orig_offset) {
                *val += last_half[digit];
            }
        }

        for i in ((orig_offset + offset)
            .max((offset + orig_offset + first_half.len() + digit + 1) / 2))
            ..(offset + orig_offset + first_half.len())
        {
            if let Some(val) = first_half.get_mut(i - offset - orig_offset) {
                *val += last_half[digit];
            }
        }

        for i in indexes_2 {
            if let Some(val) = first_half.get_mut(i - offset - orig_offset) {
                *val -= last_half[digit];
            }
        }
    }

    //then calculate the back half
    for i in 0..last_half.len() {
        for j in (i + 1)..last_half.len() {
            last_half[i] += last_half[j];
        }
    }

    //println!("slices at end: {:?}, {:?}", first_half, last_half);
}

fn get_nontrivial_indexes_affected(
    digit: usize,
    first_index: usize,
    memoize_table: &mut HashMap<(usize, usize), (Vec<usize>, Vec<usize>)>,
) -> &(Vec<usize>, Vec<usize>) {
    let mut mod_0_indexes = Vec::new();
    let mut mod_2_indexes = Vec::new();

    if memoize_table.get(&(digit, first_index)).is_none() {
        for i in first_index..digit / 2 {
            match (digit - i) / (i + 1) % 4 {
                0 => mod_0_indexes.push(i),
                2 => mod_2_indexes.push(i),
                _ => (),
            }
        }
    } else {
        return memoize_table.get(&(digit, first_index)).unwrap();
    }

    memoize_table.insert((digit, first_index), (mod_0_indexes, mod_2_indexes));

    memoize_table.get(&(digit, first_index)).unwrap()
}
