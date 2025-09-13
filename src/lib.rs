// Competitive Programming Template Library
// Modules for common algorithms and utilities

pub mod io;
pub mod math;
pub mod graph;
pub mod data_structures;
pub mod string;
pub mod geometry;
pub mod debug;
pub mod utils;
pub mod prelude;

// Re-export all modules for full access
pub use io::*;
pub use math::*;
pub use graph::*;
pub use data_structures::*;
pub use string::*;
pub use geometry::*;
pub use debug::*;
pub use utils::*;

// Macros are automatically available at crate root due to #[macro_export]

// Common constants used in competitive programming
pub const INF: i64 = 1_000_000_000_000_000_000;
pub const MOD: i64 = 1_000_000_007;
pub const MOD2: i64 = 998_244_353;
pub const EPS: f64 = 1e-9;

// Direction vectors for 4-directional movement (up, right, down, left)
pub const DX4: [i32; 4] = [-1, 0, 1, 0];
pub const DY4: [i32; 4] = [0, 1, 0, -1];

// Direction vectors for 8-directional movement
pub const DX8: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
pub const DY8: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];