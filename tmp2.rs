use std::collections::BTreeMap;
use std::io::{self, Read, Write};
const MOD: u64 = 1000000007;

struct Scanner {
    input: Vec<u8>,
    index: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            input: input.into_bytes(),
            index: 0,
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        let start = self.index;
        while self.index < self.input.len() && !self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        std::str::from_utf8(&self.input[start..self.index])
            .unwrap()
            .parse()
            .ok()
            .expect("Parse error")
    }
    fn next_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn next_line(&mut self) -> String {
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        let start = self.index;
        while self.index < self.input.len() && self.input[self.index] != b'\n' {
            self.index += 1;
        }
        let line = std::str::from_utf8(&self.input[start..self.index])
            .unwrap()
            .to_string();
        if self.index < self.input.len() && self.input[self.index] == b'\n' {
            self.index += 1;
        }
        line
    }
}
#[allow(dead_code)]
fn modexp(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut ans = 1;
    if exp <= 0 {
        return 1;
    }
    loop {
        if exp == 1 {
            return (ans * base) % m;
        }
        if exp & 1 == 0 {
            base = (base * base) % m;
            exp >>= 1;
            continue;
        } else {
            ans = (ans * base) % m;
            exp -= 1;
        }
    }
}
#[allow(dead_code)]
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

#[allow(dead_code)]
fn fltinv(n: u64, m: u64) -> u64 {
    if m <= 1 || gcd(n as usize, m as usize) > 1 {
        return 0;
    }
    return modexp(n, m - 2, m);
}
#[allow(dead_code)]
fn intsqrt(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    let mut low = 1;
    let mut high = n;
    let mut ans = 0;

    while low <= high {
        let mid = low + (high - low) / 2;
        if mid <= n / mid {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    ans
}
#[allow(dead_code)]
fn factors(n: i128) -> Vec<i128> {
    let mut fcts: Vec<i128> = Vec::new();
    let mut i: i128 = 1;
    while i * i <= n {
        if n % i == 0 {
            fcts.push(i);
            if i * i != n {
                fcts.push(n / i);
            }
        }
        i += 1;
    }
    fcts.sort();
    return fcts;
}
#[allow(dead_code)]
fn pcfact(n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1u64; n + 1];
    let mut inv_fact = vec![1u64; n + 1];

    for i in 1..=n {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[n] = fltinv(fact[n], MOD);
    for i in (1..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * ((i + 1) as u64) % MOD;
    }

    (fact, inv_fact)
}
#[allow(dead_code)]
fn ncr(n: usize, r: usize, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if r > n {
        return 0;
    }
    let num = fact[n] as u128;
    let den = (inv_fact[r] as u128 * inv_fact[n - r] as u128) % MOD as u128;
    ((num * den) % MOD as u128) as u64
}
#[allow(dead_code)]
fn npr(n: usize, r: usize, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if r > n {
        return 0;
    }
    fact[n] * inv_fact[n - r] % MOD
}

#[allow(dead_code)]
fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::with_capacity(n / 10);
    primes.push(2);
    for i in (4..=n).step_by(2) {
        is_prime[i] = false;
    }
    let mut i = 3;
    while i * i <= n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(2 * i) {
                is_prime[j] = false;
            }
        }
        i += 2;
    }
    for i in (3..=n).step_by(2) {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}

#[allow(dead_code)]

fn pfactors(mut n: u64) -> BTreeMap<u64, u32> {
    let mut fcts1 = BTreeMap::new();
    if n % 2 == 0 {
        let mut count = 0;
        while n % 2 == 0 {
            count += 1;
            n /= 2;
        }
        fcts1.insert(2, count);
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                count += 1;
                n /= i;
            }
            fcts1.insert(i, count);
        }
        i += 2;
    }
    if n > 1 {
        fcts1.insert(n, 1);
    }

    fcts1
}

fn solve(scan: &mut Scanner, out: &mut dyn Write) {
    let mut n: usize = scan.next();
    let a: usize = scan.next();
    let b: usize = scan.next();
    let n1: usize = scan.next();
    let r: usize = scan.next();
    let a1 = factors(n as i128);
    writeln!(out, "{:?}", a1).ok();
    let gc = gcd(a, b);
    writeln!(out, "{} {}", gc, lcm(a, b)).ok();
    let val = (5 * fltinv(3, MOD)) % MOD;
    writeln!(out, "{}", val).ok();
    writeln!(out, "{}", modexp(2, 100000, 1000000007)).ok();
    writeln!(out, "{:?}", pcfact(n)).ok();
    writeln!(out, "{}", intsqrt(n)).ok();
    let (fact, inv_fact) = pcfact(n);
    let combinations = ncr(n1, r, &fact, &inv_fact);
    let permutations = npr(n1, r, &fact, &inv_fact);
    writeln!(out, "15C3 = {}", combinations).ok();
    writeln!(out, "15P3 = {}", permutations).ok();
    writeln!(out, "{:?}", sieve(n)).ok();
    writeln!(out, "{:?}", pfactors(n)).ok();
}

fn main() {
    let mut scan = Scanner::new();
    let mut out = io::BufWriter::new(io::stdout());
    solve(&mut scan, &mut out);
}
