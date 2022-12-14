use std::collections::HashSet;

fn main() {
    println!("{}", quadratic_primes());
}

/// Problem 1 - Find the number of multiples of 3 or 5 under 1000.
pub fn multiples_of_3_or_5() -> u64 {
    const MAX: u64 = 1000;
    multiples_under(3, MAX)
        .union(&multiples_under(5, MAX))
        .into_iter()
        .sum()
}

/// The multiples of the number `of` that are less than `n`.
fn multiples_under(of: u64, n: u64) -> HashSet<u64> {
    let mut set = HashSet::new();
    let mut temp = of;
    while temp < n {
        set.insert(temp);
        temp += of;
    }
    set
}

/// Problem 2 - Find the sum of all fibonacci numbers under 4M.
pub fn even_fibonacci_numbers() -> u64 {
    const MAX: u64 = 4_000_000;
    let mut back_two = 0;
    let mut back_one = 1;
    let mut sum = 0;

    while back_one < MAX {
        if back_one % 2 == 0 {
            sum += back_one;
        }
        let next = back_one + back_two;
        back_two = back_one;
        back_one = next;
    }
    sum
}

/// Problem 3 - Find the product of the coefficients `a` and `b` that produce
/// the longest consecutive sequence of primes.
pub fn quadratic_primes() -> i64 {
    // The maximum value of a or b.
    const COEF_MAX: i64 = 1000;
    let mut max = 0;
    let mut max_pair: Option<(i64, i64)> = None;

    for a in (-COEF_MAX + 1)..COEF_MAX {
        for b in (-COEF_MAX)..(COEF_MAX + 1) {
            let result = max_consecutive_primes_for_quadratic(a as i64, b as i64);
            if result > max {
                max = result;
                max_pair = Some((a, b))
            }
        }
    }

    let max_pair = max_pair.expect("Max pair not found, something went wrong...");
    max_pair.0 * max_pair.1
}

/// Count the number of consecutive primes produced by the quadratic expression
/// `n^2 + a*n + b` for `n = 0..`.
fn max_consecutive_primes_for_quadratic(a: i64, b: i64) -> u64 {
    let mut n: u64 = 0;
    loop {
        if !is_prime((n as i64).pow(2) + (a * (n as i64)) + b) {
            break;
        }
        n += 1;
    }
    n
}

/// Whether `n` is a prime number.
fn is_prime(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    factors(n as u64).len() <= 2
}

/// Calculate the factors of `n`.
fn factors(n: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    factors.insert(1);
    factors.insert(n);
    for i in 2..((n as f64).sqrt().ceil() as u64) + 1 {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use crate::{is_prime, max_consecutive_primes_for_quadratic};

    #[test]
    fn is_prime_works() {
        const N: usize = 20;
        const EXPECTED: [bool; N] = [
            true, true, true, false, true, false, true, false, false, false, true, false, true,
            false, false, false, true, false, true, false,
        ];

        for i in 0..N {
            let n = (i + 1) as i64;
            assert_eq!(is_prime(n), EXPECTED[i], "Failed at n = {}", n);
        }
    }

    #[test]
    fn max_consecutive_primes_for_quadratic_works() {
        const PAIRS: [(i64, i64); 2] = [(1, 41), (-79, 1601)];
        const EXPECTED: [u64; 2] = [40, 80];

        for (i, pair) in PAIRS.iter().enumerate() {
            assert_eq!(
                max_consecutive_primes_for_quadratic(pair.0, pair.1),
                EXPECTED[i],
                "Failed at a = {}, b = {}",
                pair.0,
                pair.1
            );
        }
    }
}
