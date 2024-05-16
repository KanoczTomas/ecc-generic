use std::collections::HashSet;

use crate::types::U256;

pub fn find_factors<T: Into<U256>>(n: T) -> Vec<U256> {
    let mut n = n.into();
    let mut factors = vec![1.into()]; //1 is always a factor
    let mut candidate = U256::one();
    while n > 1.into() {
        candidate += 1.into();
        while n % candidate == 0.into() {
            n /= candidate;
            factors.push(candidate);
        }
    }
    let factors: HashSet<_> = factors.into_iter().collect(); //make vec unique
    let factors: Vec<_> = factors.into_iter().collect(); //transform to vec again
    factors
}