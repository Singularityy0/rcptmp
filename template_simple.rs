// Competitive Programming Template - Single File Version
// Copy this file and implement your solution in the solve() function

use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, BTreeMap, BTreeSet};
use std::cmp::{min, max, Reverse};

// Constants
const MOD: i64 = 1_000_000_007;
const MOD2: i64 = 998_244_353;
const INF: i64 = 1_000_000_000_000_000_000;
const EPS: f64 = 1e-9;

// Direction vectors
const DX4: [i32; 4] = [-1, 0, 1, 0];
const DY4: [i32; 4] = [0, 1, 0, -1];

// Macros
macro_rules! read_tuple {
    ($reader:expr, $($t:ty),+) => {{
        let line = read_line($reader);
        let mut iter = line.split_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
    }};
}

// I/O Functions
fn init_reader() -> Box<dyn BufRead> {
    Box::new(BufReader::new(io::stdin()))
}

fn read<T: std::str::FromStr>(reader: &mut dyn BufRead) -> T {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>(reader: &mut dyn BufRead) -> Vec<T> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
}

fn read_line(reader: &mut dyn BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_matrix<T: std::str::FromStr>(reader: &mut dyn BufRead, rows: usize) -> Vec<Vec<T>> {
    (0..rows).map(|_| read_vec(reader)).collect()
}

// Math Functions
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
    let mut result = 1;
    base %= modulo;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulo;
        }
        base = (base * base) % modulo;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: i64, modulo: i64) -> i64 {
    mod_pow(a, modulo - 2, modulo)
}

// Union-Find (Disjoint Set Union)
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        
        if px == py { return false; }
        
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
            self.rank[px] += 1;
        }
        true
    }
    
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    
    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

// Fenwick Tree (Binary Indexed Tree)
struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
            n: size,
        }
    }
    
    fn update(&mut self, mut idx: usize, delta: i64) {
        idx += 1; // Convert to 1-indexed
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }
    
    fn prefix_sum(&self, mut idx: usize) -> i64 {
        idx += 1; // Convert to 1-indexed
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & (!idx + 1);
        }
        sum
    }
    
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }
}

// Graph structure
struct Graph {
    adj: Vec<Vec<usize>>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
            n,
        }
    }
    
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }
    
    fn dfs(&self, start: usize) -> Vec<bool> {
        let mut visited = vec![false; self.n];
        let mut stack = vec![start];
        
        while let Some(node) = stack.pop() {
            if visited[node] { continue; }
            visited[node] = true;
            
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
        
        visited
    }
    
    fn bfs(&self, start: usize) -> Vec<i32> {
        let mut dist = vec![-1; self.n];
        let mut queue = VecDeque::new();
        
        dist[start] = 0;
        queue.push_back(start);
        
        while let Some(node) = queue.pop_front() {
            for &neighbor in &self.adj[node] {
                if dist[neighbor] == -1 {
                    dist[neighbor] = dist[node] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
        
        dist
    }
}

// Utility functions
fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

// Main solve function - implement your solution here
fn solve(reader: &mut dyn BufRead) {
    // TODO: Implement your solution here
    
    // Common patterns:
    // let n: usize = read(reader);
    // let arr: Vec<i32> = read_vec(reader);
    // let (a, b) = read_tuple!(reader, i64, i64);
    // let matrix: Vec<Vec<i32>> = read_matrix(reader, n);
    
    // Example usage of data structures:
    // let mut uf = UnionFind::new(n);
    // let mut ft = FenwickTree::new(n);
    // let mut graph = Graph::new(n);
    
    println!("Hello, World!");
}

fn main() {
    let mut reader = init_reader();
    
    // Single test case
    solve(&mut reader);
    
    // Multiple test cases (uncomment when needed)
    // let t: usize = read(&mut reader);
    // for _ in 0..t {
    //     solve(&mut reader);
    // }
}