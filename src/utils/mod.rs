/// Utility functions for arrays, sorting, and common operations
use std::cmp::Ordering;

/// Binary search variants for competitive programming
pub mod search {
    use std::cmp::Ordering;

    /// Find the first position where predicate becomes true (lower_bound)
    /// Returns the index of the first element >= target
    pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
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

    /// Find the first position where element > target (upper_bound)
    /// Returns the index of the first element > target
    pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
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

    /// Binary search for exact match, returns Option<usize>
    pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let pos = lower_bound(arr, target);
        if pos < arr.len() && arr[pos] == *target {
            Some(pos)
        } else {
            None
        }
    }
}

/// Array manipulation utilities
pub mod array {
    /// Rotate array left by k positions
    pub fn rotate_left<T: Clone>(arr: &mut [T], k: usize) {
        if arr.is_empty() || k == 0 {
            return;
        }
        let n = arr.len();
        let k = k % n;
        if k == 0 {
            return;
        }
        
        // Use reversal algorithm for in-place rotation
        arr[..k].reverse();
        arr[k..].reverse();
        arr.reverse();
    }

    /// Rotate array right by k positions
    pub fn rotate_right<T: Clone>(arr: &mut [T], k: usize) {
        if arr.is_empty() || k == 0 {
            return;
        }
        let n = arr.len();
        let k = k % n;
        if k == 0 {
            return;
        }
        
        rotate_left(arr, n - k);
    }

    /// Reverse a portion of the array
    pub fn reverse_range<T>(arr: &mut [T], start: usize, end: usize) {
        if start < end && end <= arr.len() {
            arr[start..end].reverse();
        }
    }

    /// Find the next permutation of the array (lexicographically)
    pub fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
        if arr.len() <= 1 {
            return false;
        }

        // Find the largest index i such that arr[i] < arr[i + 1]
        let mut i = arr.len() - 2;
        loop {
            if arr[i] < arr[i + 1] {
                break;
            }
            if i == 0 {
                return false; // No next permutation
            }
            i -= 1;
        }

        // Find the largest index j such that arr[i] < arr[j]
        let mut j = arr.len() - 1;
        while arr[i] >= arr[j] {
            j -= 1;
        }

        // Swap arr[i] and arr[j]
        arr.swap(i, j);

        // Reverse the suffix starting at arr[i + 1]
        arr[i + 1..].reverse();
        true
    }

    /// Find the previous permutation of the array (lexicographically)
    pub fn prev_permutation<T: Ord>(arr: &mut [T]) -> bool {
        if arr.len() <= 1 {
            return false;
        }

        // Find the largest index i such that arr[i] > arr[i + 1]
        let mut i = arr.len() - 2;
        loop {
            if arr[i] > arr[i + 1] {
                break;
            }
            if i == 0 {
                return false; // No previous permutation
            }
            i -= 1;
        }

        // Find the largest index j such that arr[i] > arr[j]
        let mut j = arr.len() - 1;
        while arr[i] <= arr[j] {
            j -= 1;
        }

        // Swap arr[i] and arr[j]
        arr.swap(i, j);

        // Reverse the suffix starting at arr[i + 1]
        arr[i + 1..].reverse();
        true
    }
}

/// Custom sorting comparators for common competitive programming use cases
pub mod sort {
    use std::cmp::Ordering;

    /// Sort by absolute value
    pub fn by_abs<T: Ord + Clone>(a: &T, b: &T) -> Ordering 
    where 
        T: std::ops::Sub<Output = T> + Default + PartialOrd
    {
        let zero = T::default();
        let abs_a = if *a >= zero { a.clone() } else { zero.clone() - a.clone() };
        let abs_b = if *b >= zero { b.clone() } else { zero.clone() - b.clone() };
        abs_a.cmp(&abs_b)
    }

    /// Sort pairs by first element, then by second element
    pub fn by_first_then_second<T: Ord, U: Ord>(a: &(T, U), b: &(T, U)) -> Ordering {
        a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
    }

    /// Sort pairs by second element, then by first element
    pub fn by_second_then_first<T: Ord, U: Ord>(a: &(T, U), b: &(T, U)) -> Ordering {
        a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
    }

    /// Sort by distance from a given point (for coordinates)
    pub fn by_distance_from<T>(origin: (T, T)) -> impl Fn(&(T, T), &(T, T)) -> Ordering
    where 
        T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Ord
    {
        move |a: &(T, T), b: &(T, T)| {
            let dist_a = (a.0 - origin.0) * (a.0 - origin.0) + (a.1 - origin.1) * (a.1 - origin.1);
            let dist_b = (b.0 - origin.0) * (b.0 - origin.0) + (b.1 - origin.1) * (b.1 - origin.1);
            dist_a.cmp(&dist_b)
        }
    }

    /// Sort in descending order (reverse of natural ordering)
    pub fn descending<T: Ord>(a: &T, b: &T) -> Ordering {
        b.cmp(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let arr = vec![1, 2, 4, 4, 6, 8];
        assert_eq!(search::lower_bound(&arr, &0), 0);
        assert_eq!(search::lower_bound(&arr, &1), 0);
        assert_eq!(search::lower_bound(&arr, &3), 2);
        assert_eq!(search::lower_bound(&arr, &4), 2);
        assert_eq!(search::lower_bound(&arr, &5), 4);
        assert_eq!(search::lower_bound(&arr, &9), 6);
    }

    #[test]
    fn test_upper_bound() {
        let arr = vec![1, 2, 4, 4, 6, 8];
        assert_eq!(search::upper_bound(&arr, &0), 0);
        assert_eq!(search::upper_bound(&arr, &1), 1);
        assert_eq!(search::upper_bound(&arr, &3), 2);
        assert_eq!(search::upper_bound(&arr, &4), 4);
        assert_eq!(search::upper_bound(&arr, &6), 5);
        assert_eq!(search::upper_bound(&arr, &8), 6);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 4, 4, 6, 8];
        assert_eq!(search::binary_search(&arr, &1), Some(0));
        assert_eq!(search::binary_search(&arr, &4), Some(2));
        assert_eq!(search::binary_search(&arr, &3), None);
        assert_eq!(search::binary_search(&arr, &9), None);
    }

    #[test]
    fn test_rotate_left() {
        let mut arr = vec![1, 2, 3, 4, 5];
        array::rotate_left(&mut arr, 2);
        assert_eq!(arr, vec![3, 4, 5, 1, 2]);
        
        let mut arr = vec![1, 2, 3];
        array::rotate_left(&mut arr, 3);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_right() {
        let mut arr = vec![1, 2, 3, 4, 5];
        array::rotate_right(&mut arr, 2);
        assert_eq!(arr, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_next_permutation() {
        let mut arr = vec![1, 2, 3];
        assert!(array::next_permutation(&mut arr));
        assert_eq!(arr, vec![1, 3, 2]);
        
        assert!(array::next_permutation(&mut arr));
        assert_eq!(arr, vec![2, 1, 3]);
        
        let mut arr = vec![3, 2, 1];
        assert!(!array::next_permutation(&mut arr));
    }

    #[test]
    fn test_prev_permutation() {
        let mut arr = vec![1, 3, 2];
        assert!(array::prev_permutation(&mut arr));
        assert_eq!(arr, vec![1, 2, 3]);
        
        let mut arr = vec![1, 2, 3];
        assert!(!array::prev_permutation(&mut arr));
    }

    #[test]
    fn test_sorting_comparators() {
        let mut pairs = vec![(3, 1), (1, 3), (2, 2)];
        pairs.sort_by(sort::by_first_then_second);
        assert_eq!(pairs, vec![(1, 3), (2, 2), (3, 1)]);
        
        let mut pairs = vec![(3, 1), (1, 3), (2, 2)];
        pairs.sort_by(sort::by_second_then_first);
        assert_eq!(pairs, vec![(3, 1), (2, 2), (1, 3)]);
        
        let mut nums = vec![5, -3, 2, -8, 1];
        nums.sort_by(sort::descending);
        assert_eq!(nums, vec![5, 2, 1, -3, -8]);
    }
}