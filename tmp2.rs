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
        Self { input: input.into_bytes(), index: 0 }
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
    let mut res = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}
#[allow(dead_code)]
fn fltinv(x: u64) -> u64 {
    modexp(x, MOD - 2, MOD)
}
#[allow(dead_code)]
fn pcfact(n: usize) -> (Vec<u64>, Vec<u64>) {
    let mut fact = vec![1u64; n + 1];
    let mut inv_fact = vec![1u64; n + 1];

    for i in 1..=n {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[n] = fltinv(fact[n]);
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
    fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
}
#[allow(dead_code)]
fn npr(n: usize, r: usize, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if r > n {
        return 0;
    }
    fact[n] * inv_fact[n - r] % MOD
}

fn solve(scan: &mut Scanner, out: &mut dyn Write) {
    
}

fn main() {
    let mut scan = Scanner::new();
    let mut out = io::BufWriter::new(io::stdout());
    solve(&mut scan, &mut out);
}
