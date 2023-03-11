// Project Euler #9 Pythagorean Triplets

pub fn get_result() -> i32 {
    // a + b + c = 1000 && a^2 + b^2 = c^2
    // return abc

    // straight up brute forcing
    for a in 0..400 {
        for b in a..500 {
            for c in b..600 {
                //println!("{} {}", a + b + c, is_triplet(a, b, c));
                if is_triplet(a, b, c) == true && (a + b + c == 1000) {
                    return a * b * c;
                }
            }
        }
    }

    return -1;
}

fn is_triplet(a: i32, b: i32, c: i32) -> bool {
    return (a * a) + (b * b) == c * c;
}
