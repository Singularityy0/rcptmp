// Advanced data structures for competitive programming
// Segment trees, Fenwick trees, Union-Find, etc.

/// Segment Tree specifically for sum operations with lazy propagation
pub struct SumSegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    original_size: usize,
}

impl SumSegmentTree {
    /// Create a new sum segment tree
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; 4 * size],
            lazy: vec![0; 4 * size],
            original_size: size,
        }
    }

    /// Build segment tree from array
    pub fn from_array(arr: &[i64]) -> Self {
        let mut seg_tree = Self::new(arr.len());
        if !arr.is_empty() {
            seg_tree.build(arr, 1, 0, arr.len() - 1);
        }
        seg_tree
    }

    fn build(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node, start, mid);
            self.build(arr, 2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    /// Push lazy updates down the tree
    fn push(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            let range_size = (end - start + 1) as i64;
            self.tree[node] += self.lazy[node] * range_size;
            if start != end {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }

    /// Update range [l, r] by adding val
    pub fn update_range(&mut self, l: usize, r: usize, val: i64) {
        if l < self.original_size && r < self.original_size {
            self.update_range_helper(1, 0, self.original_size - 1, l, r, val);
        }
    }

    fn update_range_helper(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i64) {
        self.push(node, start, end);
        if start > r || end < l {
            return;
        }
        if start >= l && end <= r {
            self.lazy[node] += val;
            self.push(node, start, end);
            return;
        }
        let mid = (start + end) / 2;
        self.update_range_helper(2 * node, start, mid, l, r, val);
        self.update_range_helper(2 * node + 1, mid + 1, end, l, r, val);
        self.push(2 * node, start, mid);
        self.push(2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Point update at index idx
    pub fn update_point(&mut self, idx: usize, val: i64) {
        if idx < self.original_size {
            self.update_point_helper(1, 0, self.original_size - 1, idx, val);
        }
    }

    fn update_point_helper(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        self.push(node, start, end);
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_point_helper(2 * node, start, mid, idx, val);
            } else {
                self.update_point_helper(2 * node + 1, mid + 1, end, idx, val);
            }
            self.push(2 * node, start, mid);
            self.push(2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    /// Query range [l, r]
    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        if l < self.original_size && r < self.original_size {
            self.query_helper(1, 0, self.original_size - 1, l, r)
        } else {
            0
        }
    }

    fn query_helper(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if start > r || end < l {
            return 0;
        }
        self.push(node, start, end);
        if start >= l && end <= r {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        let left_query = self.query_helper(2 * node, start, mid, l, r);
        let right_query = self.query_helper(2 * node + 1, mid + 1, end, l, r);
        left_query + right_query
    }
}

/// Generic Segment Tree for min/max operations (without lazy propagation for simplicity)
pub struct SegmentTree<T> {
    tree: Vec<T>,
    original_size: usize,
    identity: T,
    op: fn(T, T) -> T,
}

impl<T> SegmentTree<T>
where
    T: Copy + PartialOrd + std::fmt::Debug,
{
    /// Create a new segment tree with given size and operation
    pub fn new(size: usize, identity: T, op: fn(T, T) -> T) -> Self {
        Self {
            tree: vec![identity; 4 * size],
            original_size: size,
            identity,
            op,
        }
    }

    /// Build segment tree from array
    pub fn from_array(arr: &[T], identity: T, op: fn(T, T) -> T) -> Self {
        let mut seg_tree = Self::new(arr.len(), identity, op);
        if !arr.is_empty() {
            seg_tree.build(arr, 1, 0, arr.len() - 1);
        }
        seg_tree
    }

    fn build(&mut self, arr: &[T], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node, start, mid);
            self.build(arr, 2 * node + 1, mid + 1, end);
            self.tree[node] = (self.op)(self.tree[2 * node], self.tree[2 * node + 1]);
        }
    }

    /// Point update at index idx
    pub fn update_point(&mut self, idx: usize, val: T) {
        if idx < self.original_size {
            self.update_point_helper(1, 0, self.original_size - 1, idx, val);
        }
    }

    fn update_point_helper(&mut self, node: usize, start: usize, end: usize, idx: usize, val: T) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_point_helper(2 * node, start, mid, idx, val);
            } else {
                self.update_point_helper(2 * node + 1, mid + 1, end, idx, val);
            }
            self.tree[node] = (self.op)(self.tree[2 * node], self.tree[2 * node + 1]);
        }
    }

    /// Query range [l, r]
    pub fn query(&self, l: usize, r: usize) -> T {
        if l < self.original_size && r < self.original_size {
            self.query_helper(1, 0, self.original_size - 1, l, r)
        } else {
            self.identity
        }
    }

    fn query_helper(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> T {
        if start > r || end < l {
            return self.identity;
        }
        if start >= l && end <= r {
            return self.tree[node];
        }
        let mid = (start + end) / 2;
        let left_query = self.query_helper(2 * node, start, mid, l, r);
        let right_query = self.query_helper(2 * node + 1, mid + 1, end, l, r);
        (self.op)(left_query, right_query)
    }
}

/// Convenience constructors for common operations
impl SegmentTree<i64> {
    /// Create segment tree for min operations
    pub fn min(size: usize) -> Self {
        Self::new(size, i64::MAX, |a, b| a.min(b))
    }

    /// Create segment tree for max operations
    pub fn max(size: usize) -> Self {
        Self::new(size, i64::MIN, |a, b| a.max(b))
    }

    /// Create min segment tree from array
    pub fn min_from_array(arr: &[i64]) -> Self {
        Self::from_array(arr, i64::MAX, |a, b| a.min(b))
    }

    /// Create max segment tree from array
    pub fn max_from_array(arr: &[i64]) -> Self {
        Self::from_array(arr, i64::MIN, |a, b| a.max(b))
    }
}

#[cfg(test)]
mod segment_tree_tests {
    use super::*;

    #[test]
    fn test_sum_segment_tree_basic() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        
        // Test range queries
        assert_eq!(seg_tree.query(0, 2), 9);  // 1 + 3 + 5
        assert_eq!(seg_tree.query(1, 4), 24); // 3 + 5 + 7 + 9
        assert_eq!(seg_tree.query(0, 5), 36); // sum of all elements
        
        // Test point updates
        seg_tree.update_point(2, 10);
        assert_eq!(seg_tree.query(0, 2), 14); // 1 + 3 + 10
        assert_eq!(seg_tree.query(0, 5), 41); // updated sum
    }

    #[test]
    fn test_sum_segment_tree_range_updates() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        
        // Test range update
        seg_tree.update_range(1, 3, 10); // Add 10 to indices 1, 2, 3
        assert_eq!(seg_tree.query(0, 4), 45); // 1 + 12 + 13 + 14 + 5
        assert_eq!(seg_tree.query(1, 3), 39); // 12 + 13 + 14
        
        // Test multiple range updates
        seg_tree.update_range(0, 1, 5); // Add 5 to indices 0, 1
        assert_eq!(seg_tree.query(0, 1), 23); // 6 + 17
    }

    #[test]
    fn test_min_segment_tree() {
        let arr = vec![4, 2, 8, 1, 6, 3];
        let seg_tree = SegmentTree::min_from_array(&arr);
        
        assert_eq!(seg_tree.query(0, 2), 2);  // min(4, 2, 8)
        assert_eq!(seg_tree.query(2, 5), 1);  // min(8, 1, 6, 3)
        assert_eq!(seg_tree.query(0, 5), 1);  // min of all
        
        // Test point update
        let mut seg_tree = seg_tree;
        seg_tree.update_point(3, 10);
        assert_eq!(seg_tree.query(2, 5), 3);  // min(8, 10, 6, 3)
    }

    #[test]
    fn test_max_segment_tree() {
        let arr = vec![4, 2, 8, 1, 6, 3];
        let seg_tree = SegmentTree::max_from_array(&arr);
        
        assert_eq!(seg_tree.query(0, 2), 8);  // max(4, 2, 8)
        assert_eq!(seg_tree.query(3, 5), 6);  // max(1, 6, 3)
        assert_eq!(seg_tree.query(0, 5), 8);  // max of all
        
        // Test point update
        let mut seg_tree = seg_tree;
        seg_tree.update_point(1, 15);
        assert_eq!(seg_tree.query(0, 2), 15); // max(4, 15, 8)
    }

    #[test]
    fn test_segment_tree_edge_cases() {
        // Single element
        let arr = vec![42];
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        assert_eq!(seg_tree.query(0, 0), 42);
        
        seg_tree.update_point(0, 100);
        assert_eq!(seg_tree.query(0, 0), 100);
        
        // Empty range handling with larger array
        let arr = vec![1, 2, 3, 4, 5];
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        
        // Test single element queries
        assert_eq!(seg_tree.query(2, 2), 3);
        assert_eq!(seg_tree.query(4, 4), 5);
    }

    #[test]
    fn test_segment_tree_performance() {
        // Test with larger array to ensure performance
        let size = 1000;
        let arr: Vec<i64> = (1..=size as i64).collect();
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        
        // Test multiple operations
        for i in 0..100 {
            seg_tree.update_point(i % size, i as i64);
            let _ = seg_tree.query(i % size, (i + 10) % size);
        }
        
        // Test range updates
        seg_tree.update_range(0, 99, 5);
        let result = seg_tree.query(0, 99);
        assert!(result > 0); // Should have some positive sum
    }

    #[test]
    fn test_lazy_propagation() {
        let arr = vec![0; 8]; // Array of zeros
        let mut seg_tree = SumSegmentTree::from_array(&arr);
        
        // Multiple overlapping range updates
        seg_tree.update_range(0, 3, 1); // Add 1 to [0,3]
        seg_tree.update_range(2, 5, 2); // Add 2 to [2,5]
        seg_tree.update_range(1, 6, 3); // Add 3 to [1,6]
        
        // Check individual positions
        assert_eq!(seg_tree.query(0, 0), 1); // Only first update
        assert_eq!(seg_tree.query(1, 1), 4); // First + third update
        assert_eq!(seg_tree.query(2, 2), 6); // All three updates
        assert_eq!(seg_tree.query(3, 3), 6); // All three updates
        assert_eq!(seg_tree.query(4, 4), 5); // Second + third update
        assert_eq!(seg_tree.query(5, 5), 5); // Second + third update
        assert_eq!(seg_tree.query(6, 6), 3); // Only third update
        assert_eq!(seg_tree.query(7, 7), 0); // No updates
    }
}
/// Fenwick Tree (Binary Indexed Tree) for efficient prefix sum queries
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    /// Create a new Fenwick tree with given size
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
            n: size,
        }
    }

    /// Build Fenwick tree from array
    pub fn from_array(arr: &[i64]) -> Self {
        let mut fenwick = Self::new(arr.len());
        for (i, &val) in arr.iter().enumerate() {
            fenwick.update(i, val);
        }
        fenwick
    }

    /// Update point at index idx by adding delta
    pub fn update(&mut self, mut idx: usize, delta: i64) {
        idx += 1; // Convert to 1-indexed
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1); // Add LSB
        }
    }

    /// Query prefix sum from 0 to idx (inclusive)
    pub fn prefix_sum(&self, mut idx: usize) -> i64 {
        idx += 1; // Convert to 1-indexed
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & (!idx + 1); // Remove LSB
        }
        sum
    }

    /// Query range sum from l to r (inclusive)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }

    /// Set value at index idx (not add, but set)
    pub fn set(&mut self, idx: usize, val: i64) {
        let current = if idx == 0 {
            self.prefix_sum(0)
        } else {
            self.prefix_sum(idx) - self.prefix_sum(idx - 1)
        };
        self.update(idx, val - current);
    }
}

/// Range Update Fenwick Tree using difference array technique
pub struct RangeUpdateFenwickTree {
    tree: FenwickTree,
}

impl RangeUpdateFenwickTree {
    /// Create a new range update Fenwick tree
    pub fn new(size: usize) -> Self {
        Self {
            tree: FenwickTree::new(size),
        }
    }

    /// Build from array
    pub fn from_array(arr: &[i64]) -> Self {
        let mut fenwick = Self::new(arr.len());
        for (i, &val) in arr.iter().enumerate() {
            fenwick.update_range(i, i, val);
        }
        fenwick
    }

    /// Update range [l, r] by adding delta
    pub fn update_range(&mut self, l: usize, r: usize, delta: i64) {
        self.tree.update(l, delta);
        if r + 1 < self.tree.n {
            self.tree.update(r + 1, -delta);
        }
    }

    /// Query value at index idx
    pub fn query(&self, idx: usize) -> i64 {
        self.tree.prefix_sum(idx)
    }
}

#[cfg(test)]
mod fenwick_tree_tests {
    use super::*;

    #[test]
    fn test_fenwick_tree_basic() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        let mut fenwick = FenwickTree::from_array(&arr);
        
        // Test prefix sums
        assert_eq!(fenwick.prefix_sum(0), 1);   // 1
        assert_eq!(fenwick.prefix_sum(1), 4);   // 1 + 3
        assert_eq!(fenwick.prefix_sum(2), 9);   // 1 + 3 + 5
        assert_eq!(fenwick.prefix_sum(5), 36);  // sum of all
        
        // Test range sums
        assert_eq!(fenwick.range_sum(1, 3), 15); // 3 + 5 + 7
        assert_eq!(fenwick.range_sum(0, 2), 9);  // 1 + 3 + 5
        assert_eq!(fenwick.range_sum(4, 5), 20); // 9 + 11
    }

    #[test]
    fn test_fenwick_tree_updates() {
        let mut fenwick = FenwickTree::new(5);
        
        // Add values
        fenwick.update(0, 5);
        fenwick.update(1, 3);
        fenwick.update(2, 7);
        fenwick.update(3, 2);
        fenwick.update(4, 8);
        
        assert_eq!(fenwick.prefix_sum(2), 15); // 5 + 3 + 7
        assert_eq!(fenwick.range_sum(1, 3), 12); // 3 + 7 + 2
        
        // Update existing values
        fenwick.update(1, 2); // Add 2 to index 1
        assert_eq!(fenwick.prefix_sum(2), 17); // 5 + 5 + 7
        assert_eq!(fenwick.range_sum(1, 3), 14); // 5 + 7 + 2
    }

    #[test]
    fn test_fenwick_tree_set() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut fenwick = FenwickTree::from_array(&arr);
        
        // Set value at index 2 to 10
        fenwick.set(2, 10);
        assert_eq!(fenwick.range_sum(2, 2), 10); // Should be 10 now
        assert_eq!(fenwick.prefix_sum(4), 22); // 1 + 2 + 10 + 4 + 5
        
        // Set value at index 0 to 100
        fenwick.set(0, 100);
        assert_eq!(fenwick.range_sum(0, 0), 100);
        assert_eq!(fenwick.prefix_sum(4), 121); // 100 + 2 + 10 + 4 + 5
    }

    #[test]
    fn test_range_update_fenwick_tree() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut fenwick = RangeUpdateFenwickTree::from_array(&arr);
        
        // Check initial values
        assert_eq!(fenwick.query(0), 1);
        assert_eq!(fenwick.query(2), 3);
        assert_eq!(fenwick.query(4), 5);
        
        // Range update: add 10 to indices 1-3
        fenwick.update_range(1, 3, 10);
        assert_eq!(fenwick.query(0), 1);  // unchanged
        assert_eq!(fenwick.query(1), 12); // 2 + 10
        assert_eq!(fenwick.query(2), 13); // 3 + 10
        assert_eq!(fenwick.query(3), 14); // 4 + 10
        assert_eq!(fenwick.query(4), 5);  // unchanged
        
        // Another range update: add 5 to indices 0-2
        fenwick.update_range(0, 2, 5);
        assert_eq!(fenwick.query(0), 6);  // 1 + 5
        assert_eq!(fenwick.query(1), 17); // 2 + 10 + 5
        assert_eq!(fenwick.query(2), 18); // 3 + 10 + 5
        assert_eq!(fenwick.query(3), 14); // 4 + 10 (unchanged)
        assert_eq!(fenwick.query(4), 5);  // unchanged
    }

    #[test]
    fn test_fenwick_tree_edge_cases() {
        // Single element
        let mut fenwick = FenwickTree::new(1);
        fenwick.update(0, 42);
        assert_eq!(fenwick.prefix_sum(0), 42);
        assert_eq!(fenwick.range_sum(0, 0), 42);
        
        // Empty tree operations
        let fenwick = FenwickTree::new(5);
        assert_eq!(fenwick.prefix_sum(3), 0);
        assert_eq!(fenwick.range_sum(1, 3), 0);
    }

    #[test]
    fn test_fenwick_tree_performance() {
        let size = 10000;
        let mut fenwick = FenwickTree::new(size);
        
        // Perform many updates and queries
        for i in 0..1000 {
            fenwick.update(i % size, i as i64);
            let _ = fenwick.prefix_sum(i % size);
            let _ = fenwick.range_sum(i % size, (i + 100) % size);
        }
        
        // Should complete without issues
        assert!(fenwick.prefix_sum(999) >= 0);
    }

    #[test]
    fn test_fenwick_tree_negative_values() {
        let mut fenwick = FenwickTree::new(5);
        
        // Add positive and negative values
        fenwick.update(0, 10);
        fenwick.update(1, -5);
        fenwick.update(2, 3);
        fenwick.update(3, -2);
        fenwick.update(4, 7);
        
        assert_eq!(fenwick.prefix_sum(1), 5);  // 10 + (-5)
        assert_eq!(fenwick.prefix_sum(4), 13); // 10 - 5 + 3 - 2 + 7
        assert_eq!(fenwick.range_sum(1, 3), -4); // -5 + 3 + (-2)
    }
}

/// Union-Find (Disjoint Set Union) data structure with path compression and union by rank
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    /// Create a new Union-Find structure with n elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            components: n,
        }
    }

    /// Find the root of element x with path compression
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    /// Union two elements by rank
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }

        self.components -= 1;
        true
    }

    /// Check if two elements are in the same set
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Get the size of the component containing element x
    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    /// Get the number of connected components
    pub fn component_count(&self) -> usize {
        self.components
    }

    /// Get all components as a vector of vectors
    pub fn get_components(&mut self) -> Vec<Vec<usize>> {
        let mut components: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
        
        for i in 0..self.parent.len() {
            let root = self.find(i);
            components.entry(root).or_insert_with(Vec::new).push(i);
        }
        
        components.into_values().collect()
    }

    /// Reset the Union-Find structure
    pub fn reset(&mut self) {
        let n = self.parent.len();
        self.parent = (0..n).collect();
        self.rank = vec![0; n];
        self.size = vec![1; n];
        self.components = n;
    }
}

/// Union-Find with additional utilities for competitive programming
impl UnionFind {
    /// Check if all elements are in the same component
    pub fn is_fully_connected(&self) -> bool {
        self.components == 1
    }

    /// Get the largest component size
    pub fn max_component_size(&mut self) -> usize {
        let mut max_size = 0;
        for i in 0..self.parent.len() {
            let root = self.find(i);
            max_size = max_size.max(self.size[root]);
        }
        max_size
    }

    /// Union multiple elements into one component
    pub fn union_all(&mut self, elements: &[usize]) -> bool {
        if elements.len() < 2 {
            return false;
        }
        
        let mut changed = false;
        for i in 1..elements.len() {
            if self.union(elements[0], elements[i]) {
                changed = true;
            }
        }
        changed
    }

    /// Get representatives (roots) of all components
    pub fn get_representatives(&mut self) -> Vec<usize> {
        let mut representatives = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            representatives.insert(self.find(i));
        }
        representatives.into_iter().collect()
    }
}#
[cfg(test)]
mod union_find_tests {
    use super::*;

    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);
        
        // Initially all elements are separate
        assert_eq!(uf.component_count(), 5);
        assert!(!uf.connected(0, 1));
        assert!(!uf.connected(2, 3));
        
        // Union some elements
        assert!(uf.union(0, 1));
        assert!(uf.connected(0, 1));
        assert_eq!(uf.component_count(), 4);
        
        assert!(uf.union(2, 3));
        assert!(uf.connected(2, 3));
        assert_eq!(uf.component_count(), 3);
        
        // Union already connected elements should return false
        assert!(!uf.union(0, 1));
        assert_eq!(uf.component_count(), 3);
    }

    #[test]
    fn test_union_find_component_sizes() {
        let mut uf = UnionFind::new(6);
        
        // Initially all components have size 1
        assert_eq!(uf.component_size(0), 1);
        assert_eq!(uf.component_size(3), 1);
        
        // Union elements and check sizes
        uf.union(0, 1);
        assert_eq!(uf.component_size(0), 2);
        assert_eq!(uf.component_size(1), 2);
        
        uf.union(2, 3);
        uf.union(3, 4);
        assert_eq!(uf.component_size(2), 3);
        assert_eq!(uf.component_size(3), 3);
        assert_eq!(uf.component_size(4), 3);
        
        // Union two components
        uf.union(1, 4);
        assert_eq!(uf.component_size(0), 5);
        assert_eq!(uf.component_size(2), 5);
        assert_eq!(uf.component_size(5), 1); // Still separate
    }

    #[test]
    fn test_union_find_path_compression() {
        let mut uf = UnionFind::new(10);
        
        // Create a long chain: 0-1-2-3-4-5-6-7-8-9
        for i in 0..9 {
            uf.union(i, i + 1);
        }
        
        // All should be connected
        for i in 0..10 {
            for j in i+1..10 {
                assert!(uf.connected(i, j));
            }
        }
        
        assert_eq!(uf.component_count(), 1);
        assert_eq!(uf.component_size(0), 10);
        assert_eq!(uf.component_size(9), 10);
    }

    #[test]
    fn test_union_find_components() {
        let mut uf = UnionFind::new(8);
        
        // Create components: {0,1,2}, {3,4}, {5}, {6,7}
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(6, 7);
        
        let components = uf.get_components();
        assert_eq!(components.len(), 4);
        
        // Check component sizes
        let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
        sizes.sort();
        assert_eq!(sizes, vec![1, 2, 2, 3]);
        
        // Check representatives
        let representatives = uf.get_representatives();
        assert_eq!(representatives.len(), 4);
    }

    #[test]
    fn test_union_find_utilities() {
        let mut uf = UnionFind::new(6);
        
        // Initially not fully connected
        assert!(!uf.is_fully_connected());
        assert_eq!(uf.max_component_size(), 1);
        
        // Union some elements
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);
        
        assert_eq!(uf.component_count(), 3);
        assert_eq!(uf.max_component_size(), 2);
        assert!(!uf.is_fully_connected());
        
        // Union all into one component
        uf.union(0, 2);
        uf.union(2, 4);
        
        assert_eq!(uf.component_count(), 1);
        assert_eq!(uf.max_component_size(), 6);
        assert!(uf.is_fully_connected());
    }

    #[test]
    fn test_union_find_union_all() {
        let mut uf = UnionFind::new(8);
        
        // Union multiple elements at once
        let elements = vec![0, 2, 4, 6];
        assert!(uf.union_all(&elements));
        
        // All should be connected
        for &i in &elements {
            for &j in &elements {
                assert!(uf.connected(i, j));
            }
        }
        
        assert_eq!(uf.component_size(0), 4);
        assert_eq!(uf.component_count(), 5); // 4 connected + 4 separate
        
        // Union already connected elements should return false
        assert!(!uf.union_all(&elements));
    }

    #[test]
    fn test_union_find_reset() {
        let mut uf = UnionFind::new(5);
        
        // Make some unions
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.component_count(), 3);
        
        // Reset
        uf.reset();
        assert_eq!(uf.component_count(), 5);
        assert!(!uf.connected(0, 1));
        assert!(!uf.connected(2, 3));
        
        for i in 0..5 {
            assert_eq!(uf.component_size(i), 1);
        }
    }

    #[test]
    fn test_union_find_performance() {
        let size = 10000;
        let mut uf = UnionFind::new(size);
        
        // Perform many union operations
        for i in 0..size-1 {
            uf.union(i, i + 1);
        }
        
        // All should be in one component
        assert_eq!(uf.component_count(), 1);
        assert_eq!(uf.component_size(0), size);
        
        // Test many find operations
        for i in 0..1000 {
            assert!(uf.connected(i, size - 1 - i));
        }
    }

    #[test]
    fn test_union_find_edge_cases() {
        // Single element
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.component_count(), 1);
        assert_eq!(uf.component_size(0), 1);
        assert!(uf.is_fully_connected());
        
        // Empty union_all
        let mut uf = UnionFind::new(3);
        assert!(!uf.union_all(&[]));
        assert!(!uf.union_all(&[0]));
        
        // Self union
        assert!(!uf.union(0, 0));
        assert_eq!(uf.component_count(), 3);
    }
}