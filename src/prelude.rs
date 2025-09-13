// Prelude module for competitive programming template
// Import this module to get access to the most commonly used utilities

// Re-export the most essential items for competitive programming

// I/O utilities - most commonly used
pub use crate::io::{init_reader, read, read_vec, read_matrix, read_line, print_vec, print_matrix};

// Math utilities - frequently needed
pub use crate::math::{gcd, lcm, mod_pow, mod_inv, sieve, factorial, combination, mod_combination};

// Graph algorithms - common in competitive programming
pub use crate::graph::Graph;

// Data structures - essential for many problems
pub use crate::data_structures::{SegmentTree, SumSegmentTree, FenwickTree, UnionFind};

// String algorithms - for string problems
pub use crate::string::{kmp_search, z_algorithm, polynomial_hash, RollingHash};

// Geometry utilities - for geometric problems
pub use crate::geometry::{Point, Line, Polygon};

// Utility functions - commonly used
pub use crate::utils::search::{lower_bound, upper_bound, binary_search};
pub use crate::utils::array::{next_permutation, prev_permutation};

// Standard library imports that are commonly used in competitive programming
pub use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};
pub use std::cmp::{min, max, Reverse};
pub use std::io::BufRead;

/// Convenience macro for reading multiple values in one line
/// Usage: let (a, b, c) = read_tuple!(reader, i32, i32, i32);
#[macro_export]
macro_rules! read_tuple {
    ($reader:expr, $($t:ty),+) => {{
        let line = read_line($reader);
        let mut iter = line.split_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
    }};
}

/// Convenience macro for creating vectors with a specific value
/// Usage: let vec = vec_with![0; 10]; // creates vec![0, 0, 0, ..., 0] with 10 elements
#[macro_export]
macro_rules! vec_with {
    ($val:expr; $n:expr) => {
        vec![$val; $n]
    };
}

/// Convenience macro for 2D vector initialization
/// Usage: let matrix = matrix![0; 5, 10]; // creates 5x10 matrix filled with 0
#[macro_export]
macro_rules! matrix {
    ($val:expr; $rows:expr, $cols:expr) => {
        vec![vec![$val; $cols]; $rows]
    };
}

/// Convenience macro for min/max of multiple values
/// Usage: let result = min!(a, b, c, d);
#[macro_export]
macro_rules! min {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr),+) => {
        std::cmp::min($x, min!($($xs),+))
    };
}

#[macro_export]
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($xs:expr),+) => {
        std::cmp::max($x, max!($($xs),+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_reader(input: &str) -> Box<dyn BufRead> {
        Box::new(Cursor::new(input.to_string()))
    }

    #[test]
    fn test_read_tuple_macro() {
        let input = "10 20 30\n";
        let mut reader = create_reader(input);
        let (a, b, c) = read_tuple!(&mut reader, i32, i32, i32);
        assert_eq!((a, b, c), (10, 20, 30));
    }

    #[test]
    fn test_vec_with_macro() {
        let vec = vec_with![42; 5];
        assert_eq!(vec, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_matrix_macro() {
        let matrix = matrix![0; 3, 4];
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 4);
        assert_eq!(matrix[2][3], 0);
    }

    #[test]
    fn test_min_max_macros() {
        assert_eq!(min!(5, 3, 8, 1, 9), 1);
        assert_eq!(max!(5, 3, 8, 1, 9), 9);
        assert_eq!(min!(42), 42);
        assert_eq!(max!(42), 42);
    }

    #[test]
    fn test_prelude_imports() {
        // Test that we can use the imported functions
        assert_eq!(gcd(12, 8), 4);
        
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        
        let point = Point::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
    }
}