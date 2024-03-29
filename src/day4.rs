//Clippy does not like that boolean expression, but doesn't actually have a better suggestion.
#[allow(clippy::nonminimal_bool)]

pub fn day_4() {
    let mut num_passwords_p1 = 0;
    let mut num_passwords_p2 = 0;

    for i in 134_564..585_159 {
        let mut decreases = false;
        let mut has_double = false;
        let mut has_exact_double = false;
        let digits = split_to_digits(i);
        let mut last_digit = digits[5];

        for index in (0..5).rev() {
            if digits[index] < last_digit {
                decreases = true
            }
            if last_digit == digits[index] {
                has_double = true
            }
            last_digit = digits[index]
        }

        //this is horrendous lmao
        if digits[0] == digits[1] && digits[1] != digits[2]
            || digits[0] != digits[1] && digits[1] == digits[2] && digits[2] != digits[3]
            || digits[1] != digits[2] && digits[2] == digits[3] && digits[3] != digits[4]
            || digits[2] != digits[3] && digits[3] == digits[4] && digits[4] != digits[5]
            || digits[3] != digits[4] && digits[4] == digits[5]
        {
            has_exact_double = true
        }

        if has_double && !decreases {
            num_passwords_p1 += 1
        }

        if has_exact_double && !decreases {
            num_passwords_p2 += 1
        }
    }

    println!("The number of passwords in range is: {}", num_passwords_p1);
    println!(
        "The number of part 2 passwords in range is: {}",
        num_passwords_p2
    )
}

fn split_to_digits(number: u32) -> Vec<u32> {
    let mut n = number;
    let mut digits = Vec::with_capacity(6);
    while n != 0 {
        digits.push(n % 10);
        n /= 10
    }
    digits
}
