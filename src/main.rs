// Competitive Programming Template - Main File
// This template provides a flexible structure for solving competitive programming problems

use std::io::BufRead;

// Import all utilities from the competitive programming library
use competitive_template::*;

/// Main solve function - implement your solution here
/// 
/// # Arguments
/// * `reader` - Buffered reader for input operations
/// 
/// # Example Usage
/// ```
/// fn solve(reader: &mut impl BufRead) {
///     let n: usize = read(reader);
///     let arr: Vec<i32> = read_vec(reader);
///     
///     // Your solution logic here
///     println!("{}", result);
/// }
/// ```
fn solve(reader: &mut impl BufRead) {
    // TODO: Implement your solution here
    
    // Common patterns for competitive programming:
    
    // 1. Reading single values
    // let n: usize = read(reader);
    // let x: i64 = read(reader);
    
    // 2. Reading vectors
    // let arr: Vec<i32> = read_vec(reader);
    
    // 3. Reading matrix/2D arrays
    // let matrix: Vec<Vec<i32>> = read_matrix(reader, n);
    
    // 4. Using mathematical utilities
    // let g = gcd(a, b);
    // let primes = sieve(1000000);
    
    // 5. Using graph algorithms
    // let mut graph = Graph::new(n);
    // graph.add_edge(u, v);
    // let distances = graph.dijkstra(0);
    
    // 6. Using data structures
    // let mut seg_tree = SegmentTree::new(&arr);
    // let mut uf = UnionFind::new(n);
    
    // 7. Debug output (only in debug mode)
    // dbg_print!("Debug info: {}", value);
    
    println!("Hello, World!"); // Replace with your output
}

fn main() {
    // Initialize the buffered reader for fast I/O
    let mut reader = init_reader();
    
    // Single test case (default)
    solve(&mut reader);
    
    // Multiple test cases (uncomment when needed)
    // Example: First line contains number of test cases
    /*
    let t: usize = read(&mut reader);
    for _ in 0..t {
        solve(&mut reader);
    }
    */
    
    // Interactive problems (uncomment when needed)
    // Note: For interactive problems, you may need to flush output
    /*
    use std::io::{self, Write};
    solve(&mut reader);
    io::stdout().flush().unwrap();
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    /// Helper function to create a reader from a string
    fn create_reader(input: &str) -> Box<dyn BufRead> {
        Box::new(Cursor::new(input.to_string()))
    }
    
    #[test]
    fn test_solve_basic() {
        // Test with sample input
        let input = "5\n1 2 3 4 5\n";
        let mut reader = create_reader(input);
        
        // This is a basic test - modify according to your problem
        // For now, just ensure solve doesn't panic
        solve(&mut reader);
    }
    
    #[test]
    fn test_multiple_test_cases() {
        let input = "2\n3\n1 2 3\n4\n4 3 2 1\n";
        let mut reader = create_reader(input);
        
        // Example of testing multiple test cases
        let t: usize = read(&mut reader);
        assert_eq!(t, 2);
        
        for _ in 0..t {
            solve(&mut reader);
        }
    }
    
    #[test]
    fn test_io_utilities() {
        let input = "42\n1 2 3 4 5\n";
        let mut reader = create_reader(input);
        
        let n: i32 = read(&mut reader);
        assert_eq!(n, 42);
        
        let vec: Vec<i32> = read_vec(&mut reader);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_math_utilities() {
        // Test basic math functions
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(lcm(12, 8), 24);
        
        // Test modular arithmetic
        assert_eq!(mod_pow(2, 10, 1000), 24);
    }
    
    #[test]
    fn test_graph_utilities() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        
        let visited = graph.dfs(0);
        assert_eq!(visited.len(), 4);
        assert!(visited[0] && visited[1] && visited[2] && visited[3]);
    }
    
    #[test]
    fn test_data_structures() {
        // Test Union-Find
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        
        assert!(uf.connected(0, 1));
        assert!(!uf.connected(0, 2));
        
        // Test Fenwick Tree
        let mut ft = FenwickTree::new(5);
        ft.update(0, 5);
        ft.update(2, 3);
        
        assert_eq!(ft.prefix_sum(2), 8); // sum from 0 to 2
    }
}