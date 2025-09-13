// Mathematical utilities for competitive programming
// Number theory, combinatorics, and modular arithmetic

/// Computes the greatest common divisor of two integers using Euclidean algorithm
/// 
/// # Examples
/// ```
/// use competitive_template::math::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// ```
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Computes the least common multiple of two integers
/// 
/// # Examples
/// ```
/// use competitive_template::math::lcm;
/// assert_eq!(lcm(12, 18), 36);
/// assert_eq!(lcm(0, 5), 0);
/// ```
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a.abs() / gcd(a, b)) * b.abs()
    }
}

/// Computes (base^exp) % modulo efficiently using binary exponentiation
/// 
/// # Examples
/// ```
/// use competitive_template::math::mod_pow;
/// assert_eq!(mod_pow(2, 10, 1000), 24);
/// assert_eq!(mod_pow(3, 4, 7), 4);
/// ```
pub fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
    if modulo == 1 { return 0; }
    
    let mut result = 1;
    base %= modulo;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;
    }
    
    result
}

/// Computes modular multiplicative inverse using extended Euclidean algorithm
/// Returns the inverse of a modulo m, or None if it doesn't exist
/// 
/// # Examples
/// ```
/// use competitive_template::math::mod_inv;
/// assert_eq!(mod_inv(3, 7), Some(5)); // 3 * 5 ≡ 1 (mod 7)
/// assert_eq!(mod_inv(2, 4), None);    // No inverse exists
/// ```
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        None // Inverse doesn't exist
    } else {
        Some(((x % m) + m) % m)
    }
}

/// Extended Euclidean algorithm
/// Returns (gcd(a, b), x, y) such that ax + by = gcd(a, b)
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, y1, x1) = extended_gcd(b % a, a);
        let x = x1 - (b / a) * y1;
        let y = y1;
        (g, x, y)
    }
}

/// Sieve of Eratosthenes to find all prime numbers up to n
/// Returns a boolean vector where is_prime[i] indicates if i is prime
/// 
/// # Examples
/// ```
/// use competitive_template::math::sieve;
/// let primes = sieve(10);
/// assert_eq!(primes[2], true);
/// assert_eq!(primes[4], false);
/// assert_eq!(primes[7], true);
/// ```
pub fn sieve(n: usize) -> Vec<bool> {
    if n < 2 {
        return vec![false; n + 1];
    }
    
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
}

/// Precomputes factorials up to n
/// Returns a vector where factorial[i] = i!
/// 
/// # Examples
/// ```
/// use competitive_template::math::factorial;
/// let fact = factorial(5);
/// assert_eq!(fact[0], 1);
/// assert_eq!(fact[3], 6);
/// assert_eq!(fact[5], 120);
/// ```
pub fn factorial(n: usize) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * (i as i64);
    }
    fact
}

/// Precomputes modular factorials up to n
/// Returns a vector where mod_factorial[i] = i! % modulo
/// 
/// # Examples
/// ```
/// use competitive_template::math::mod_factorial;
/// let fact = mod_factorial(5, 1000000007);
/// assert_eq!(fact[3], 6);
/// assert_eq!(fact[5], 120);
/// ```
pub fn mod_factorial(n: usize, modulo: i64) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * (i as i64)) % modulo;
    }
    fact
}

/// Precomputes modular inverse factorials up to n
/// Returns a vector where inv_factorial[i] = (i!)^(-1) % modulo
/// Requires modulo to be prime for modular inverse to exist
/// 
/// # Examples
/// ```
/// use competitive_template::math::{mod_factorial, mod_inv_factorial};
/// let fact = mod_factorial(5, 1000000007);
/// let inv_fact = mod_inv_factorial(5, 1000000007, &fact);
/// assert_eq!((fact[3] * inv_fact[3]) % 1000000007, 1);
/// ```
pub fn mod_inv_factorial(n: usize, modulo: i64, factorial: &[i64]) -> Vec<i64> {
    let mut inv_fact = vec![1; n + 1];
    if n > 0 {
        inv_fact[n] = mod_pow(factorial[n], modulo - 2, modulo); // Fermat's little theorem
        for i in (1..n).rev() {
            inv_fact[i] = (inv_fact[i + 1] * ((i + 1) as i64)) % modulo;
        }
    }
    inv_fact
}

/// Computes nCr (n choose r) using precomputed factorials
/// 
/// # Examples
/// ```
/// use competitive_template::math::{factorial, combination};
/// let fact = factorial(10);
/// assert_eq!(combination(5, 2, &fact), 10);
/// assert_eq!(combination(10, 3, &fact), 120);
/// ```
pub fn combination(n: usize, r: usize, factorial: &[i64]) -> i64 {
    if r > n {
        return 0;
    }
    factorial[n] / (factorial[r] * factorial[n - r])
}

/// Computes nCr % modulo using precomputed modular factorials and inverse factorials
/// 
/// # Examples
/// ```
/// use competitive_template::math::{mod_factorial, mod_inv_factorial, mod_combination};
/// let modulo = 1000000007;
/// let fact = mod_factorial(10, modulo);
/// let inv_fact = mod_inv_factorial(10, modulo, &fact);
/// assert_eq!(mod_combination(5, 2, &fact, &inv_fact, modulo), 10);
/// ```
pub fn mod_combination(n: usize, r: usize, factorial: &[i64], inv_factorial: &[i64], modulo: i64) -> i64 {
    if r > n {
        return 0;
    }
    (factorial[n] * inv_factorial[r] % modulo) * inv_factorial[n - r] % modulo
}

/// Computes nPr (n permute r) using precomputed factorials
/// 
/// # Examples
/// ```
/// use competitive_template::math::{factorial, permutation};
/// let fact = factorial(10);
/// assert_eq!(permutation(5, 2, &fact), 20);
/// assert_eq!(permutation(10, 3, &fact), 720);
/// ```
pub fn permutation(n: usize, r: usize, factorial: &[i64]) -> i64 {
    if r > n {
        return 0;
    }
    factorial[n] / factorial[n - r]
}

/// Computes nPr % modulo using precomputed modular factorials and inverse factorials
/// 
/// # Examples
/// ```
/// use competitive_template::math::{mod_factorial, mod_inv_factorial, mod_permutation};
/// let modulo = 1000000007;
/// let fact = mod_factorial(10, modulo);
/// let inv_fact = mod_inv_factorial(10, modulo, &fact);
/// assert_eq!(mod_permutation(5, 2, &fact, &inv_fact, modulo), 20);
/// ```
pub fn mod_permutation(n: usize, r: usize, factorial: &[i64], inv_factorial: &[i64], modulo: i64) -> i64 {
    if r > n {
        return 0;
    }
    (factorial[n] * inv_factorial[n - r]) % modulo
}

