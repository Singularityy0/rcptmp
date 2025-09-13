// Graph algorithms for competitive programming
// DFS, BFS, shortest paths, and graph utilities

use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

/// Graph structure with adjacency list representation
/// Supports both weighted and unweighted edges
#[derive(Debug, Clone)]
pub struct Graph {
    /// Number of vertices
    n: usize,
    /// Adjacency list for unweighted edges
    adj: Vec<Vec<usize>>,
    /// Adjacency list for weighted edges (vertex, weight)
    weighted_adj: Vec<Vec<(usize, i64)>>,
    /// Whether the graph is directed
    directed: bool,
}

impl Graph {
    /// Create a new graph with n vertices
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            weighted_adj: vec![Vec::new(); n],
            directed: false,
        }
    }

    /// Create a new directed graph with n vertices
    pub fn new_directed(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            weighted_adj: vec![Vec::new(); n],
            directed: true,
        }
    }

    /// Get the number of vertices
    pub fn size(&self) -> usize {
        self.n
    }

    /// Add an unweighted edge between vertices u and v
    pub fn add_edge(&mut self, u: usize, v: usize) {
        debug_assert!(u < self.n && v < self.n, "Vertex index out of bounds");
        
        self.adj[u].push(v);
        if !self.directed && u != v {
            self.adj[v].push(u);
        }
    }

    /// Add a weighted edge between vertices u and v with given weight
    pub fn add_weighted_edge(&mut self, u: usize, v: usize, weight: i64) {
        debug_assert!(u < self.n && v < self.n, "Vertex index out of bounds");
        
        self.weighted_adj[u].push((v, weight));
        if !self.directed && u != v {
            self.weighted_adj[v].push((u, weight));
        }
    }

    /// Get neighbors of vertex u (unweighted)
    pub fn neighbors(&self, u: usize) -> &[usize] {
        debug_assert!(u < self.n, "Vertex index out of bounds");
        &self.adj[u]
    }

    /// Get weighted neighbors of vertex u
    pub fn weighted_neighbors(&self, u: usize) -> &[(usize, i64)] {
        debug_assert!(u < self.n, "Vertex index out of bounds");
        &self.weighted_adj[u]
    }

    /// Check if the graph is directed
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Get the degree of vertex u (for unweighted edges)
    pub fn degree(&self, u: usize) -> usize {
        debug_assert!(u < self.n, "Vertex index out of bounds");
        self.adj[u].len()
    }

    /// Get the weighted degree of vertex u
    pub fn weighted_degree(&self, u: usize) -> usize {
        debug_assert!(u < self.n, "Vertex index out of bounds");
        self.weighted_adj[u].len()
    }

    /// Depth-First Search from a starting vertex
    /// Returns a vector indicating which vertices were visited
    pub fn dfs(&self, start: usize) -> Vec<bool> {
        debug_assert!(start < self.n, "Start vertex index out of bounds");
        
        let mut visited = vec![false; self.n];
        self.dfs_helper(start, &mut visited);
        visited
    }

    /// DFS helper function for recursive traversal
    fn dfs_helper(&self, u: usize, visited: &mut Vec<bool>) {
        visited[u] = true;
        
        // Visit unweighted neighbors
        for &v in &self.adj[u] {
            if !visited[v] {
                self.dfs_helper(v, visited);
            }
        }
        
        // Visit weighted neighbors
        for &(v, _) in &self.weighted_adj[u] {
            if !visited[v] {
                self.dfs_helper(v, visited);
            }
        }
    }

    /// Breadth-First Search from a starting vertex
    /// Returns a vector with distances from start vertex (-1 if unreachable)
    pub fn bfs(&self, start: usize) -> Vec<i32> {
        debug_assert!(start < self.n, "Start vertex index out of bounds");
        
        let mut distances = vec![-1; self.n];
        let mut queue = VecDeque::new();
        
        distances[start] = 0;
        queue.push_back(start);
        
        while let Some(u) = queue.pop_front() {
            // Process unweighted neighbors
            for &v in &self.adj[u] {
                if distances[v] == -1 {
                    distances[v] = distances[u] + 1;
                    queue.push_back(v);
                }
            }
            
            // Process weighted neighbors (treating as unweighted for BFS)
            for &(v, _) in &self.weighted_adj[u] {
                if distances[v] == -1 {
                    distances[v] = distances[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        
        distances
    }

    /// Find all connected components in the graph
    /// Returns a vector where each element is the component ID of that vertex
    pub fn connected_components(&self) -> Vec<usize> {
        let mut component = vec![usize::MAX; self.n];
        let mut component_id = 0;
        
        for i in 0..self.n {
            if component[i] == usize::MAX {
                self.dfs_component(i, &mut component, component_id);
                component_id += 1;
            }
        }
        
        component
    }

    /// DFS helper for finding connected components
    fn dfs_component(&self, u: usize, component: &mut Vec<usize>, comp_id: usize) {
        component[u] = comp_id;
        
        // Visit unweighted neighbors
        for &v in &self.adj[u] {
            if component[v] == usize::MAX {
                self.dfs_component(v, component, comp_id);
            }
        }
        
        // Visit weighted neighbors
        for &(v, _) in &self.weighted_adj[u] {
            if component[v] == usize::MAX {
                self.dfs_component(v, component, comp_id);
            }
        }
    }

    /// Count the number of connected components
    pub fn count_components(&self) -> usize {
        let components = self.connected_components();
        if components.is_empty() {
            return 0;
        }
        components.iter().max().unwrap() + 1
    }

    /// Check if two vertices are in the same connected component
    pub fn are_connected(&self, u: usize, v: usize) -> bool {
        debug_assert!(u < self.n && v < self.n, "Vertex index out of bounds");
        
        let visited = self.dfs(u);
        visited[v]
    }

    /// Dijkstra's algorithm for shortest paths from a source vertex
    /// Returns distances to all vertices (i64::MAX if unreachable)
    /// Only works with weighted edges - returns empty vector if no weighted edges exist
    pub fn dijkstra(&self, start: usize) -> Vec<i64> {
        debug_assert!(start < self.n, "Start vertex index out of bounds");
        
        let mut distances = vec![i64::MAX; self.n];
        let mut heap = BinaryHeap::new();
        
        distances[start] = 0;
        heap.push(Reverse((0i64, start)));
        
        while let Some(Reverse((dist, u))) = heap.pop() {
            // Skip if we've already found a better path
            if dist > distances[u] {
                continue;
            }
            
            // Process weighted neighbors
            for &(v, weight) in &self.weighted_adj[u] {
                let new_dist = distances[u].saturating_add(weight);
                if new_dist < distances[v] {
                    distances[v] = new_dist;
                    heap.push(Reverse((new_dist, v)));
                }
            }
        }
        
        distances
    }

    /// Dijkstra's algorithm with path reconstruction
    /// Returns (distances, predecessors) where predecessors[v] is the previous vertex in shortest path to v
    pub fn dijkstra_with_path(&self, start: usize) -> (Vec<i64>, Vec<Option<usize>>) {
        debug_assert!(start < self.n, "Start vertex index out of bounds");
        
        let mut distances = vec![i64::MAX; self.n];
        let mut predecessors = vec![None; self.n];
        let mut heap = BinaryHeap::new();
        
        distances[start] = 0;
        heap.push(Reverse((0i64, start)));
        
        while let Some(Reverse((dist, u))) = heap.pop() {
            if dist > distances[u] {
                continue;
            }
            
            for &(v, weight) in &self.weighted_adj[u] {
                let new_dist = distances[u].saturating_add(weight);
                if new_dist < distances[v] {
                    distances[v] = new_dist;
                    predecessors[v] = Some(u);
                    heap.push(Reverse((new_dist, v)));
                }
            }
        }
        
        (distances, predecessors)
    }

    /// Reconstruct shortest path from start to target using predecessors from dijkstra_with_path
    pub fn reconstruct_path(&self, start: usize, target: usize, predecessors: &[Option<usize>]) -> Option<Vec<usize>> {
        debug_assert!(start < self.n && target < self.n, "Vertex index out of bounds");
        
        if predecessors[target].is_none() && start != target {
            return None; // No path exists
        }
        
        let mut path = Vec::new();
        let mut current = target;
        
        while current != start {
            path.push(current);
            if let Some(pred) = predecessors[current] {
                current = pred;
            } else {
                return None; // Path broken
            }
        }
        path.push(start);
        path.reverse();
        
        Some(path)
    }

    /// Check if the graph has negative edge weights
    pub fn has_negative_edges(&self) -> bool {
        for adj_list in &self.weighted_adj {
            for &(_, weight) in adj_list {
                if weight < 0 {
                    return true;
                }
            }
        }
        false
    }

    /// Get shortest distance between two vertices using Dijkstra
    pub fn shortest_distance(&self, start: usize, target: usize) -> Option<i64> {
        debug_assert!(start < self.n && target < self.n, "Vertex index out of bounds");
        
        let distances = self.dijkstra(start);
        if distances[target] == i64::MAX {
            None
        } else {
            Some(distances[target])
        }
    }

    /// Get shortest path between two vertices
    pub fn shortest_path(&self, start: usize, target: usize) -> Option<Vec<usize>> {
        debug_assert!(start < self.n && target < self.n, "Vertex index out of bounds");
        
        let (_, predecessors) = self.dijkstra_with_path(start);
        self.reconstruct_path(start, target, &predecessors)
    }
}
