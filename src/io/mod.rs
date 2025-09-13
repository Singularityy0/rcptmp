// I/O utilities for competitive programming
// Fast input/output operations with buffered reading

use std::io::{BufRead, BufReader, stdin};
use std::str::FromStr;
use std::fmt::Display;

/// Initialize a buffered reader for fast input operations
/// Returns a boxed BufRead trait object that can read from stdin
pub fn init_reader() -> Box<dyn BufRead> {
    Box::new(BufReader::new(stdin()))
}

/// Read a single value from the buffered reader
/// Automatically trims whitespace and parses the value
/// Panics with descriptive message if parsing fails
pub fn read<T: FromStr>(reader: &mut dyn BufRead) -> T 
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    reader.read_line(&mut line)
        .expect("Failed to read line from input");
    
    line.trim()
        .parse()
        .expect(&format!("Failed to parse input: '{}'", line.trim()))
}

/// Read a vector of values from a single line
/// Values should be space-separated
/// Automatically handles whitespace and empty elements
pub fn read_vec<T: FromStr>(reader: &mut dyn BufRead) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    reader.read_line(&mut line)
        .expect("Failed to read line from input");
    
    line.trim()
        .split_whitespace()
        .map(|s| s.parse().expect(&format!("Failed to parse vector element: '{}'", s)))
        .collect()
}

/// Read a specific number of values from a single line
/// Useful when you know exactly how many elements to expect
pub fn read_vec_n<T: FromStr>(reader: &mut dyn BufRead, n: usize) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let vec = read_vec(reader);
    if vec.len() != n {
        panic!("Expected {} elements, got {}", n, vec.len());
    }
    vec
}

/// Read a string line (without parsing)
/// Useful for reading raw strings or when you need to handle parsing manually
pub fn read_line(reader: &mut dyn BufRead) -> String {
    let mut line = String::new();
    reader.read_line(&mut line)
        .expect("Failed to read line from input");
    line.trim().to_string()
}

/// Read a matrix (2D vector) from input
/// First reads the number of rows, then reads each row as a vector
/// Each row should be on a separate line with space-separated values
pub fn read_matrix<T: FromStr>(reader: &mut dyn BufRead, rows: usize) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    let mut matrix: Vec<Vec<T>> = Vec::with_capacity(rows);
    for i in 0..rows {
        let row: Vec<T> = read_vec(reader);
        if i == 0 {
            // First row establishes the expected column count
        } else if matrix[0].len() != row.len() {
            panic!("Matrix row {} has {} elements, expected {}", i, row.len(), matrix[0].len());
        }
        matrix.push(row);
    }
    matrix
}

/// Read a matrix with known dimensions
/// More efficient when you know both rows and columns in advance
pub fn read_matrix_sized<T: FromStr>(reader: &mut dyn BufRead, rows: usize, cols: usize) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    let mut matrix: Vec<Vec<T>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row: Vec<T> = read_vec_n(reader, cols);
        matrix.push(row);
    }
    matrix
}

/// Skip an empty line in input
/// Useful for handling input formats with blank lines between sections
pub fn empty_line(reader: &mut dyn BufRead) {
    let mut line = String::new();
    reader.read_line(&mut line)
        .expect("Failed to read empty line");
    
    if !line.trim().is_empty() {
        panic!("Expected empty line, got: '{}'", line.trim());
    }
}

/// Skip any empty lines until a non-empty line is found
/// Returns the first non-empty line found
pub fn skip_empty_lines(reader: &mut dyn BufRead) -> String {
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)
            .expect("Failed to read line while skipping empty lines");
        
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
}

// Output utilities

/// Print a vector with a custom separator
/// Useful for formatting output according to problem requirements
pub fn print_vec<T: Display>(vec: &[T], separator: &str) {
    if vec.is_empty() {
        return;
    }
    
    for (i, item) in vec.iter().enumerate() {
        if i > 0 {
            print!("{}", separator);
        }
        print!("{}", item);
    }
    println!();
}

/// Print a vector with space separation (most common case)
pub fn print_vec_space<T: Display>(vec: &[T]) {
    print_vec(vec, " ");
}

/// Print a vector with newline separation (each element on its own line)
pub fn print_vec_lines<T: Display>(vec: &[T]) {
    for item in vec {
        println!("{}", item);
    }
}

/// Print a matrix with each row on a separate line
/// Elements in each row are space-separated
pub fn print_matrix<T: Display>(matrix: &[Vec<T>]) {
    for row in matrix {
        print_vec_space(row);
    }
}

/// Print a matrix with custom separators
/// Allows customization of both element and row separators
pub fn print_matrix_custom<T: Display>(matrix: &[Vec<T>], element_sep: &str, row_sep: &str) {
    for (i, row) in matrix.iter().enumerate() {
        if i > 0 {
            print!("{}", row_sep);
        }
        print_vec(row, element_sep);
    }
}

/// Print "YES" or "NO" based on boolean value
/// Common in competitive programming for boolean answers
pub fn print_yes_no(condition: bool) {
    println!("{}", if condition { "YES" } else { "NO" });
}

/// Print "Yes" or "No" based on boolean value
/// Alternative capitalization for some problems
pub fn print_yes_no_title(condition: bool) {
    println!("{}", if condition { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_test_reader(input: &str) -> Box<dyn BufRead> {
        Box::new(Cursor::new(input.to_string()))
    }

    #[test]
    fn test_read_integer() {
        let mut reader = create_test_reader("42\n");
        let result: i32 = read(&mut reader);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_read_string() {
        let mut reader = create_test_reader("hello\n");
        let result: String = read(&mut reader);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_read_with_whitespace() {
        let mut reader = create_test_reader("  123  \n");
        let result: i32 = read(&mut reader);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_read_vec_integers() {
        let mut reader = create_test_reader("1 2 3 4 5\n");
        let result: Vec<i32> = read_vec(&mut reader);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_read_vec_strings() {
        let mut reader = create_test_reader("apple banana cherry\n");
        let result: Vec<String> = read_vec(&mut reader);
        assert_eq!(result, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_read_vec_with_extra_whitespace() {
        let mut reader = create_test_reader("  1   2   3  \n");
        let result: Vec<i32> = read_vec(&mut reader);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_read_vec_n() {
        let mut reader = create_test_reader("10 20 30\n");
        let result: Vec<i32> = read_vec_n(&mut reader, 3);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    #[should_panic(expected = "Expected 2 elements, got 3")]
    fn test_read_vec_n_wrong_count() {
        let mut reader = create_test_reader("10 20 30\n");
        let _: Vec<i32> = read_vec_n(&mut reader, 2);
    }

    #[test]
    fn test_read_line() {
        let mut reader = create_test_reader("hello world\n");
        let result = read_line(&mut reader);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_read_line_with_whitespace() {
        let mut reader = create_test_reader("  spaced text  \n");
        let result = read_line(&mut reader);
        assert_eq!(result, "spaced text");
    }

    #[test]
    fn test_multiple_reads() {
        let mut reader = create_test_reader("5\n1 2 3 4 5\nhello\n");
        
        let n: usize = read(&mut reader);
        assert_eq!(n, 5);
        
        let vec: Vec<i32> = read_vec(&mut reader);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
        
        let text = read_line(&mut reader);
        assert_eq!(text, "hello");
    }

    #[test]
    #[should_panic(expected = "Failed to parse input")]
    fn test_read_parse_error() {
        let mut reader = create_test_reader("not_a_number\n");
        let _: i32 = read(&mut reader);
    }

    #[test]
    #[should_panic(expected = "Failed to parse vector element")]
    fn test_read_vec_parse_error() {
        let mut reader = create_test_reader("1 not_a_number 3\n");
        let _: Vec<i32> = read_vec(&mut reader);
    }

    // Tests for advanced I/O utilities
    #[test]
    fn test_read_matrix() {
        let mut reader = create_test_reader("1 2 3\n4 5 6\n7 8 9\n");
        let result: Vec<Vec<i32>> = read_matrix(&mut reader, 3);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);
    }

    #[test]
    fn test_read_matrix_sized() {
        let mut reader = create_test_reader("1 2\n3 4\n5 6\n");
        let result: Vec<Vec<i32>> = read_matrix_sized(&mut reader, 3, 2);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6]
        ]);
    }

    #[test]
    #[should_panic(expected = "Matrix row 1 has 2 elements, expected 3")]
    fn test_read_matrix_inconsistent_columns() {
        let mut reader = create_test_reader("1 2 3\n4 5\n");
        let _: Vec<Vec<i32>> = read_matrix(&mut reader, 2);
    }

    #[test]
    fn test_empty_line() {
        let mut reader = create_test_reader("\n");
        empty_line(&mut reader); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Expected empty line")]
    fn test_empty_line_not_empty() {
        let mut reader = create_test_reader("not empty\n");
        empty_line(&mut reader);
    }

    #[test]
    fn test_skip_empty_lines() {
        let mut reader = create_test_reader("\n\n\nhello\n");
        let result = skip_empty_lines(&mut reader);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_skip_empty_lines_no_empty() {
        let mut reader = create_test_reader("immediate\n");
        let result = skip_empty_lines(&mut reader);
        assert_eq!(result, "immediate");
    }

    // Output utility tests (these test the formatting, not actual printing)
    #[test]
    fn test_print_vec_formatting() {
        let vec = vec![1, 2, 3, 4, 5];
        // We can't easily test print! macros, so we'll test the logic separately
        // This is more of a smoke test to ensure the functions compile and run
        print_vec_space(&vec);
        print_vec_lines(&vec);
    }

    #[test]
    fn test_matrix_and_boolean_output() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6]
        ];
        
        // Smoke tests for output functions
        print_matrix(&matrix);
        print_yes_no(true);
        print_yes_no(false);
        print_yes_no_title(true);
        print_yes_no_title(false);
    }

    #[test]
    fn test_complex_input_scenario() {
        // Test a complex competitive programming input scenario
        let input = "3\n1 2 3\n4 5 6\n7 8 9\n\nhello world\n";
        let mut reader = create_test_reader(input);
        
        let n: usize = read(&mut reader);
        assert_eq!(n, 3);
        
        let matrix: Vec<Vec<i32>> = read_matrix(&mut reader, n);
        assert_eq!(matrix, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);
        
        empty_line(&mut reader);
        
        let text = read_line(&mut reader);
        assert_eq!(text, "hello world");
    }
}