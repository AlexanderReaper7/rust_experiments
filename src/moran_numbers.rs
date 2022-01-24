/*
A Harshad number is a number which is divisible by the sum of its digits. For example, 132 is divisible by 6 (1+3+2).
A subset of the Harshad numbers are the Moran numbers. Moran numbers yield a prime when divided by the sum of their digits. For example, 133 divided by 7 (1+3+3) yields 19, a prime.
Create a function that takes a number and returns "M" if the number is a Moran number, "H" if it is a (non-Moran) Harshad number, or "Neither".

Examples
moran(132) ➞ "H"
moran(133) ➞ "M"
moran(134) ➞ "Neither"
*/

use std::env::args;
fn main() {
    let mut i: u32 = 1;
    if args().len() == 2 {
        for arg in args().enumerate() {
            if arg.0 == 1 {
                i = arg.1.parse::<u32>().unwrap();
                break;
            }
        }
    }
    loop {
        print!("{:?} | ", i);
        let sum = digit_sum(i);
        print!("{:?} | ", sum);
        if i % sum == 0 {
            if is_prime(i / sum) {
                println!("M {:?}", i / sum);
            }
            else {
                println!("H");
            }
        }
        else {
            println!("Neither");
        }
        i += 1;
    }
}


/// Determines wheter a number is a 'M'oran, 'H'arshad, or 'N'either.
fn moran(n: u32) -> char {
    let sum = digit_sum(n);
    if n % sum == 0 {
        if is_prime(n / sum) {
            return 'M';
        }
        else {
            return 'H';
        }
    }
    'N'
}

/// Returns the sum of the digits
fn digit_sum(n: u32) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

struct Infinity {
    i: u32
}
impl Iterator for Infinity {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.i += 1;
        Some(self.i-1)
    }
}
impl Infinity {
    fn infinity(start: u32) -> Infinity {
        Infinity {i: start}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum() {
        assert_eq!(1, digit_sum(10));
        assert_eq!(20, digit_sum(5555));
        assert_eq!(10, digit_sum(1234));
    }
    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(1));
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
        assert_eq!(false, is_prime(6));
        assert_eq!(true, is_prime(7));
        assert_eq!(false, is_prime(8));
        assert_eq!(true, is_prime(997));
        assert_eq!(false, is_prime(1000));
    }

    #[test]
    fn test_moran() {
        assert_eq!('H', moran(132));
        assert_eq!('H', moran(13608));
        assert_eq!('M', moran(133));
        assert_eq!('M', moran(13605));
        assert_eq!('N', moran(134));
        assert_eq!('N', moran(13606));
    }
}