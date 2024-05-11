use std::env;
use std::str::FromStr;

todo!("optimize without creating new vec?");
fn main() {
    let args = env::args();
    let number_of_args = args.len();

    if number_of_args < 2 {
        panic!("invalid input, please enter valid numbers")
    }

    let mut numbers: Vec<u64> = Vec::with_capacity(number_of_args);

    for num in args.skip(1) {
        numbers.push(u64::from_str(&num).expect("error when parse args: invalid number"));
    }

    let mut d = numbers[0];
    for num in &numbers[1..] {
        d = gcd(d, *num);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
