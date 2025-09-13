// String algorithms for competitive programming
// Pattern matching, hashing, and string utilities

/// Computes the failure function (prefix function) for KMP algorithm
/// Returns a vector where failure[i] is the length of the longest proper prefix
/// of pattern[0..=i] that is also a suffix of pattern[0..=i]
pub fn kmp_table(pattern: &str) -> Vec<usize> {
    let pattern_bytes = pattern.as_bytes();
    let n = pattern_bytes.len();
    let mut failure = vec![0; n];
    
    if n == 0 {
        return failure;
    }
    
    let mut j = 0;
    for i in 1..n {
        while j > 0 && pattern_bytes[i] != pattern_bytes[j] {
            j = failure[j - 1];
        }
        if pattern_bytes[i] == pattern_bytes[j] {
            j += 1;
        }
        failure[i] = j;
    }
    
    failure
}

/// Finds all occurrences of pattern in text using KMP algorithm
/// Returns a vector of starting indices where pattern occurs in text
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }
    
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let n = text_bytes.len();
    let m = pattern_bytes.len();
    
    if m > n {
        return Vec::new();
    }
    
    let failure = kmp_table(pattern);
    let mut matches = Vec::new();
    let mut j = 0;
    
    for i in 0..n {
        while j > 0 && text_bytes[i] != pattern_bytes[j] {
            j = failure[j - 1];
        }
        if text_bytes[i] == pattern_bytes[j] {
            j += 1;
        }
        if j == m {
            matches.push(i + 1 - m);
            j = failure[j - 1];
        }
    }
    
    matches
}

/// Computes the Z-array for a string
/// Z[i] is the length of the longest substring starting from s[i] 
/// which is also a prefix of s
pub fn z_algorithm(s: &str) -> Vec<usize> {
    let s_bytes = s.as_bytes();
    let n = s_bytes.len();
    let mut z = vec![0; n];
    
    if n == 0 {
        return z;
    }
    
    z[0] = n;
    let mut l = 0;
    let mut r = 0;
    
    for i in 1..n {
        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }
        
        while i + z[i] < n && s_bytes[z[i]] == s_bytes[i + z[i]] {
            z[i] += 1;
        }
        
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    
    z
}

/// Finds all occurrences of pattern in text using Z-algorithm
/// Returns a vector of starting indices where pattern occurs in text
pub fn z_search(text: &str, pattern: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }
    
    let combined = format!("{}${}", pattern, text);
    let z = z_algorithm(&combined);
    let pattern_len = pattern.len();
    let mut matches = Vec::new();
    
    for i in (pattern_len + 1)..z.len() {
        if z[i] == pattern_len {
            matches.push(i - pattern_len - 1);
        }
    }
    
    matches
}

/// Finds all occurrences of multiple patterns in text
/// Returns a vector of tuples (pattern_index, start_position)
pub fn multi_pattern_search(text: &str, patterns: &[&str]) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    
    for (pattern_idx, &pattern) in patterns.iter().enumerate() {
        let positions = kmp_search(text, pattern);
        for pos in positions {
            matches.push((pattern_idx, pos));
        }
    }
    
    // Sort by position for consistent ordering
    matches.sort_by_key(|&(_, pos)| pos);
    matches
}

// String hashing utilities for competitive programming

/// Computes polynomial rolling hash of a string
/// Uses the given base and modulo for hash calculation
pub fn polynomial_hash(s: &str, base: u64, modulo: u64) -> u64 {
    let mut hash = 0u64;
    let mut power = 1u64;
    
    for byte in s.bytes() {
        hash = (hash + (byte as u64 * power) % modulo) % modulo;
        power = (power * base) % modulo;
    }
    
    hash
}

/// Computes polynomial rolling hash with precomputed powers
/// More efficient for multiple hash computations
pub fn polynomial_hash_with_powers(s: &str, base: u64, modulo: u64, powers: &[u64]) -> u64 {
    let mut hash = 0u64;
    
    for (i, byte) in s.bytes().enumerate() {
        if i < powers.len() {
            hash = (hash + (byte as u64 * powers[i]) % modulo) % modulo;
        }
    }
    
    hash
}

/// Precomputes powers of base modulo modulo up to max_len
pub fn precompute_powers(base: u64, modulo: u64, max_len: usize) -> Vec<u64> {
    let mut powers = vec![1; max_len + 1];
    
    for i in 1..=max_len {
        powers[i] = (powers[i - 1] * base) % modulo;
    }
    
    powers
}

/// Structure for efficient rolling hash computations
pub struct RollingHash {
    hash: Vec<u64>,
    powers: Vec<u64>,
    base: u64,
    modulo: u64,
}

impl RollingHash {
    /// Creates a new RollingHash for the given string
    pub fn new(s: &str, base: u64, modulo: u64) -> Self {
        let n = s.len();
        let mut hash = vec![0; n + 1];
        let powers = precompute_powers(base, modulo, n);
        
        for (i, byte) in s.bytes().enumerate() {
            hash[i + 1] = (hash[i] + (byte as u64 * powers[i]) % modulo) % modulo;
        }
        
        RollingHash {
            hash,
            powers,
            base,
            modulo,
        }
    }
    
    /// Gets hash of substring s[l..r] (inclusive)
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        if l > r || r >= self.hash.len() - 1 {
            return 0;
        }
        
        let mut result = (self.hash[r + 1] + self.modulo - self.hash[l]) % self.modulo;
        
        // Normalize by dividing by base^l
        if l > 0 {
            result = (result * mod_inverse(self.powers[l], self.modulo)) % self.modulo;
        }
        
        result
    }
}

/// Double hashing structure for collision avoidance
pub struct DoubleHash {
    hash1: RollingHash,
    hash2: RollingHash,
}

impl DoubleHash {
    /// Creates a new DoubleHash with two different bases and moduli
    pub fn new(s: &str) -> Self {
        const BASE1: u64 = 31;
        const BASE2: u64 = 37;
        const MOD1: u64 = 1_000_000_007;
        const MOD2: u64 = 1_000_000_009;
        
        DoubleHash {
            hash1: RollingHash::new(s, BASE1, MOD1),
            hash2: RollingHash::new(s, BASE2, MOD2),
        }
    }
    
    /// Gets double hash of substring s[l..r] (inclusive)
    pub fn get_hash(&self, l: usize, r: usize) -> (u64, u64) {
        (self.hash1.get_hash(l, r), self.hash2.get_hash(l, r))
    }
    
    /// Compares two substrings using double hashing
    pub fn compare_substrings(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        self.get_hash(l1, r1) == self.get_hash(l2, r2)
    }
}

/// Computes modular inverse using extended Euclidean algorithm
fn mod_inverse(a: u64, m: u64) -> u64 {
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (gcd, x1, y1) = extended_gcd(b % a, a);
            let x = y1 - (b / a) * x1;
            let y = x1;
            (gcd, x, y)
        }
    }
    
    let (_, x, _) = extended_gcd(a as i64, m as i64);
    ((x % m as i64 + m as i64) % m as i64) as u64
}

/// Utility for comparing multiple substrings efficiently
pub struct SubstringComparator {
    double_hash: DoubleHash,
}

impl SubstringComparator {
    /// Creates a new SubstringComparator for the given string
    pub fn new(s: &str) -> Self {
        SubstringComparator {
            double_hash: DoubleHash::new(s),
        }
    }
    
    /// Checks if two substrings are equal using hashing
    pub fn are_equal(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        if r1 - l1 != r2 - l2 {
            return false;
        }
        self.double_hash.compare_substrings(l1, r1, l2, r2)
    }
    
    /// Finds all unique substrings of given length
    pub fn unique_substrings(&self, len: usize, text_len: usize) -> Vec<(u64, u64)> {
        let mut hashes = std::collections::HashSet::new();
        
        for i in 0..=text_len.saturating_sub(len) {
            if i + len > 0 {
                let hash = self.double_hash.get_hash(i, i + len - 1);
                hashes.insert(hash);
            }
        }
        
        hashes.into_iter().collect()
    }
}
