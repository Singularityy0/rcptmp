// new tmp , maybe faster

use std::io::{self, Read, Write};

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
        
        while self.index < self.input.len()
            && self.input[self.index].is_ascii_whitespace()
        {
            self.index += 1;
        }
        let start = self.index;
        while self.index < self.input.len()
            && self.input[self.index] != b'\n'
        {
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

fn solve(scan: &mut Scanner, out: &mut dyn Write) {
    
}

fn main() {
    let mut scan = Scanner::new();
    let mut out = io::BufWriter::new(io::stdout());
    solve(&mut scan, &mut out);

}
