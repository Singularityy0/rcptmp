pub mod cp_template {
    pub use crate::cp_template::{ds, geom, graph, io, math, string, utils};
    pub use std::cmp::{max, min, Reverse};
    pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
    pub use std::io::{self, BufRead, BufReader, Write};

    pub const INF: i64 = 1_000_000_000_000_000_000;
    pub const MOD: i64 = 1_000_000_007;

    pub const DX4: [i32; 4] = [-1, 0, 1, 0];
    pub const DY4: [i32; 4] = [0, 1, 0, -1];
    pub const DX8: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    pub const DY8: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    pub mod io {
        use std::fmt::Display;
        use std::io::{stdin, BufRead, BufReader};
        use std::str::FromStr;

        pub fn init_reader() -> Box<dyn BufRead> {
            Box::new(BufReader::new(stdin()))
        }

        pub fn read<T: FromStr>(reader: &mut dyn BufRead) -> T
        where
            T::Err: std::fmt::Debug,
        {
            let mut line = String::new();
            reader.read_line(&mut line).expect("Failed to read line");
            line.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse input: '{}'", line.trim()))
        }

        pub fn read_vec<T: FromStr>(reader: &mut dyn BufRead) -> Vec<T>
        where
            T::Err: std::fmt::Debug,
        {
            let mut line = String::new();
            reader.read_line(&mut line).expect("Failed to read line");
            line.trim()
                .split_whitespace()
                .map(|s| {
                    s.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse vector element: '{}'", s))
                })
                .collect()
        }
        
        pub fn read_line(reader: &mut dyn BufRead) -> String {
            let mut line = String::new();
            reader.read_line(&mut line).expect("Failed to read line");
            line.trim().to_string()
        }

        pub fn read_matrix<T: FromStr>(reader: &mut dyn BufRead, rows: usize) -> Vec<Vec<T>>
        where
            T::Err: std::fmt::Debug,
        {
            (0..rows).map(|_| read_vec(reader)).collect()
        }

        pub fn print_vec<T: Display>(vec: &[T], separator: &str) {
            if let Some(first) = vec.first() {
                print!("{}", first);
                for item in vec.iter().skip(1) {
                    print!("{}{}", separator, item);
                }
            }
            println!();
        }

        pub fn print_yes_no(condition: bool) {
            println!("{}", if condition { "YES" } else { "NO" });
        }
    }

    pub mod macros {
        #[macro_export]
        macro_rules! read_tuple {
            ($line:expr, $($t:ty),+) => {{
                let mut iter = $line.split_whitespace();
                ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
            }};
        }
    }

    pub mod math {
        pub fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a.abs()
        }

        pub fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
            if modulo == 1 { return 0; }
            let mut result = 1;
            base %= modulo;
            while exp > 0 {
                if exp % 2 == 1 {
                    result = (result * base) % modulo;
                }
                exp >>= 1;
                base = (base * base) % modulo;
            }
            result
        }

        pub fn sieve(n: usize) -> Vec<bool> {
            if n < 2 { return vec![false; n + 1]; }
            let mut is_prime = vec![true; n + 1];
            is_prime[0] = false;
            is_prime[1] = false;
            for i in 2..=((n as f64).sqrt() as usize) {
                if is_prime[i] {
                    for j in (i * i..=n).step_by(i) {
                        is_prime[j] = false;
                    }
                }
            }
            is_prime
        }
    }

    pub mod utils {
        pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
            arr.binary_search(target).unwrap_or_else(|x| x)
        }

        pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
            arr.binary_search(target)
                .map(|x| {
                    let mut i = x + 1;
                    while i < arr.len() && arr[i] == *target {
                        i += 1;
                    }
                    i
                })
                .unwrap_or_else(|x| x)
        }
    }

    pub mod ds {
        pub struct UnionFind {
            parent: Vec<usize>,
            size: Vec<usize>,
            components: usize,
        }

        impl UnionFind {
            pub fn new(n: usize) -> Self {
                Self {
                    parent: (0..n).collect(),
                    size: vec![1; n],
                    components: n,
                }
            }

            pub fn find(&mut self, x: usize) -> usize {
                if self.parent[x] != x {
                    self.parent[x] = self.find(self.parent[x]);
                }
                self.parent[x]
            }

            pub fn union(&mut self, x: usize, y: usize) -> bool {
                let root_x = self.find(x);
                let root_y = self.find(y);
                if root_x == root_y { return false; }

                if self.size[root_x] < self.size[root_y] {
                    self.parent[root_x] = root_y;
                    self.size[root_y] += self.size[root_x];
                } else {
                    self.parent[root_y] = root_x;
                    self.size[root_x] += self.size[root_y];
                }
                self.components -= 1;
                true
            }
            
            pub fn component_count(&self) -> usize {
                self.components
            }

            pub fn component_size(&mut self, x: usize) -> usize {
                let root = self.find(x);
                self.size[root]
            }
        }
    }

    pub mod graph {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, VecDeque};

        pub struct Graph {
            n: usize,
            adj: Vec<Vec<usize>>,
            weighted_adj: Vec<Vec<(usize, i64)>>,
            directed: bool,
        }

        impl Graph {
            pub fn new(n: usize, directed: bool) -> Self {
                Self {
                    n,
                    adj: vec![vec![]; n],
                    weighted_adj: vec![vec![]; n],
                    directed,
                }
            }

            pub fn add_edge(&mut self, u: usize, v: usize) {
                self.adj[u].push(v);
                if !self.directed {
                    self.adj[v].push(u);
                }
            }

            pub fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i64) {
                self.weighted_adj[u].push((v, weight));
                if !self.directed {
                    self.weighted_adj[v].push((u, weight));
                }
            }

            pub fn bfs(&self, start: usize) -> Vec<i32> {
                let mut dist = vec![-1; self.n];
                let mut q = VecDeque::new();
                dist[start] = 0;
                q.push_back(start);
                while let Some(u) = q.pop_front() {
                    for &v in &self.adj[u] {
                        if dist[v] == -1 {
                            dist[v] = dist[u] + 1;
                            q.push_back(v);
                        }
                    }
                }
                dist
            }

            pub fn dijkstra(&self, start: usize) -> Vec<i64> {
                let mut dist = vec![super::INF; self.n];
                let mut pq = BinaryHeap::new();
                dist[start] = 0;
                pq.push(Reverse((0, start)));
                while let Some(Reverse((d, u))) = pq.pop() {
                    if d > dist[u] { continue; }
                    for &(v, w) in &self.weighted_adj[u] {
                        if dist[u] + w < dist[v] {
                            dist[v] = dist[u] + w;
                            pq.push(Reverse((dist[v], v)));
                        }
                    }
                }
                dist
            }
        }
    }

    pub mod string {}
    pub mod geom {}
}

use cp_template::*;

fn solve(reader: &mut impl BufRead) {
    let nm: Vec<u64> = io::read_vec(reader);
    let n = nm[0] as usize;
    let m = nm[1];
    let mut rates: Vec<u64> = io::read_vec(reader);
    rates.sort_unstable_by(|a, b| b.cmp(a));
    let k = min(n as u64, m);
    let mut total_cakes: u128 = 0;
    for i in 0..(k as usize) {
        total_cakes += rates[i] as u128 * (m - i as u64) as u128;
    }
    println!("{}", total_cakes);
}

fn main() {
    let mut reader = io::init_reader();
    let t: usize = io::read(&mut reader);
    for _ in 0..t {
        solve(&mut reader);
    }
}