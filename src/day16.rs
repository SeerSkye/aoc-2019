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

    let mut memoized_indexes: IndexTable = HashMap::new();

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


/// Okay, I ran this for 11.5 hours and got a solution and tbh I'm kind of done with this, but
/// I want to at least write down some reasoning and whatnot so maybe this code is understandable
/// 
/// So, to start, let's analyze the simple example case 12345678. This case is nice because the
/// indexes and the values mostly line up well for talking about (though I will be indexing starting at 0)
/// 1 -> 1 - 3 + 5 - 7
/// 2 -> 2 + 3 - 6 - 7
/// 3 -> 3 + 4 + 5
/// 4 -> 4 + 5 + 6 + 7
/// 5 -> 5 + 6 + 7 + 8
/// 6 -> 6 + 7 + 8
/// 7 -> 7 + 8
/// 8 -> 8
/// 
/// First of all, note that a value can only affect values earlier in the list. The 1 at the start is
/// only able to affect itself, because there's no values before it. The 8 at the end will stay constant,
/// because there are no values after it to be affected by. This means that we can truncate the list
/// to the part starting at the number we're looking for for part 2, cutting the size of list we FFT from
/// 6.5 million to around 500k. We still need to keep track of the amount of elements we cut off the front
/// however, in order to properly calculate whice values to add where.
/// 
/// Next, consider the list split into it's front and back halves, and FFT on the front half
/// 1 -> 1 - 3          5
/// 2 -> 2 + 3          6
/// 3 -> 3 + 4          7
/// 4 -> 4              8
/// 
/// Notice how the only elements in the sums in the front half of the full 12345678 are exactly
/// the solution to FFT of the front half of the list. This lets us simplify as
/// 1 -> FFT(Front)[0] + 5 - 7
/// 2 -> FFT(Front)[1] - 6 - 7
/// 3 -> FFT(Front)[2] + 5
/// 4 -> FFT(Front)[3] + 5 + 6 + 7
/// 5 -> 5 + 6 + 7 + 8
/// 6 -> 6 + 7 + 8
/// 7 -> 7 + 8
/// 8 -> 8
/// 
/// Note too how the end result of back half of the list is only affected by the back half of the list, and
/// how it's a simple patterned sum. This naturally falls out of how the indexes are determined. In order for
/// an index i to have a value at index n added to it, n must be greater than i (as noted earlier as "a 
/// value can only affect values earlier in the list"), and floor((n - i)/(i+1)) % 4 == 0. The subtraction
/// eqution is much the same, but == 2 instead of 0. The last half of the list is where (n-i) is less than (i+1),
/// which is always 0 when truncated and as such always has modulus 0. Now our simplification looks like:
/// 1 -> FFT(Front)[0] + 5 - 7
/// 2 -> FFT(Front)[1] - 6
/// 3 -> FFT(Front)[2] + 5
/// 4 -> FFT(Front)[3] + 5 + 6 + 7
/// 5 -> sum([5..=8])
/// 6 -> sum([6..=8])
/// 7 -> sum([7..=8])
/// 8 -> sum([8])
/// 
/// The only hard part left is determining the values left over. For that we use our floor((n - i)/(i+1)) % 4 == 0 
/// to generate a list of indexes that a given number affects ahead of it in the list. In order to save time we
/// store these lists of indexes too, although we only store the affected indedxes that cannot be simply calculated
/// as a result of (n-i) being less than (i+1) to keep from allocating all our memory on the real part 2 test case.
/// 
/// This recursive approach works, but I did not prove that it worked for lists of any size, so in order to guarantee it
/// working I made sure to adjust the list size to a power of 2 by adding 0s onto the end. 0s do not affect the output
/// as it doesn't matter if they are added or subtracted, and also do not ever change (cause they can only be affected
/// by later elements, which are also 0). It may be the case that this solution does work without rezising, but last night
/// I did not feel like taking the time to test it.
/// 
/// Note that we only take the last digit of the number after all calculations, as otherwise bugs occur as a sum switches
/// from positive to negative
/// 
/// For the small test case given for pt2, with optimizations this runs at 0.1s per fft, but unfortunately it does not
/// scale well to the full input, which still takes around 390s per fft while the computer is not doing anything. I'm sure
/// I'll end up seeing other people solutions, seeing what I missed, and feeling silly though.
fn fft(
    list: &mut Vec<i64>,
    offset: usize,
    mut memoize_table: &mut IndexTable,
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

    fft_helper(list, offset, &mut memoize_table);

    for digit in list.iter_mut() {
        *digit = digit.abs() % 10;
    }

    println!("FFT Took: {}", timer.elapsed().as_secs_f32());
}

fn fft_helper(
    work_list: &mut [i64],
    orig_offset: usize,
    mut memoize_table: &mut IndexTable,
) {
    //are we done?
    if work_list.len() <= 1 {
        return;
    }

    let (first_half, last_half) = work_list.split_at_mut(work_list.len() / 2);
    //first, fft the first half
    fft_helper(first_half, orig_offset, &mut memoize_table);

    //then, get the hard to calc parts
    for (digit_index, digit) in last_half.iter().enumerate() {
        let (indexes_0, indexes_2) = get_nontrivial_indexes_affected(
            orig_offset + first_half.len() + digit_index,
            orig_offset,
            &mut memoize_table,
        );

        for i in indexes_0 {
            if let Some(val) = first_half.get_mut(i - orig_offset) {
                *val += digit;
            }
        }

        //this sort of nonsense just makes sure that if we're far into a list we cut the front 
        //off of we only iterate through the values actually in the list by grabbing the maximum
        //of orig_offset (which is the start of the list) and the difit index we're working with
        //over 2 (which is the halfway point where afterwards our true digit index - i is less than i + 1)
        for i in ((orig_offset)
            .max((orig_offset + first_half.len() + digit_index + 1) / 2))
            ..(orig_offset + first_half.len())
        {
            if let Some(val) = first_half.get_mut(i - orig_offset) {
                *val += digit;
            }
        }

        for i in indexes_2 {
            if let Some(val) = first_half.get_mut(i - orig_offset) {
                *val -= digit;
            }
        }
    }

    //then calculate the back half
    for i in 0..last_half.len() {
        for j in (i + 1)..last_half.len() {
            last_half[i] += last_half[j];
        }
    }
}

type IndexTable = HashMap<(usize, usize), (Vec<usize>, Vec<usize>)>;

fn get_nontrivial_indexes_affected(
    digit: usize,
    first_index: usize,
    memoize_table: &mut IndexTable,
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
