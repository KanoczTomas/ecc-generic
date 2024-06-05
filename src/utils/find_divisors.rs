use crate::types::U256;

pub fn find_divisors<T: Into<U256>>(n: T) -> Vec<U256> {
    let mut divisors = vec![];
    let n:U256 = n.into();
    let mut i = U256::one();
    let n_sqrt = n.integer_sqrt();
    while i <= n_sqrt {
        if n % i == 0.into() {
            divisors.push(i);
            if i != n/i {
                divisors.push(n/i);
            }
        }
        i += U256::one();
    }
    divisors.sort_unstable();
    divisors.dedup();
    divisors
}