use crate::types::U256;
pub fn is_prime<T: Into<U256>>(n: T) -> bool {
    let n = n.into();
    if n <= U256::from(1) {
        return false; // Numbers less than 2 are not prime
    }
    if n <= U256::from(3) {
        return true; // 2 and 3 are prime
    }
    if n % U256::from(2) == U256::from(0) || n % U256::from(3) == U256::from(0) {
        return false; // Even numbers and multiples of 3 are not prime
    }
    let mut i = U256::from(5);
    while i * i <= n {
        if n % i == U256::from(0) || n % (i + U256::from(2)) == U256::from(0) {
            return false; // Not prime
        }
        i += U256::from(6); // Increment by 6 to check both even and odd divisors
    }
    true // If none of the above conditions were met, the number is prime
}
