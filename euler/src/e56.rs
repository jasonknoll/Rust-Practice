// Euler #56 Largest digit sum for a^b where a,b < 100

use num::pow;

// This shit overflows too quickly and i can't solve it lol
pub fn largest_sum() -> i128 {
    let mut largest: i128 = 0;

    for a in 1..100 {
        for b in 1..100 {
            println!("{a} {b}");
            let sum = get_digit_sum(pow(a, b));
            if sum > largest {
                largest = sum;
            }
        }
    }

    return largest;
}

fn get_digit_sum(n: i128) -> i128 {
    let num = n;

    let string = num.to_string();

    let mut sum = 0;

    for ch in string.chars() {
        sum += ch.to_digit(10).unwrap() as i128;
    }

    return sum;
}
