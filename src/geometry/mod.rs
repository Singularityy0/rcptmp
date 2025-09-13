/// Coordinate geometry utilities for competitive programming
/// Points, lines, polygons, and geometric calculations

use std::ops::{Add, Sub, Mul};

/// Floating point precision constant for geometric calculations
pub const EPS: f64 = 1e-9;

/// 2D Point structure with basic operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Origin point (0, 0)
    pub fn origin() -> Self {
        Point::new(0.0, 0.0)
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate squared distance to another point (avoids sqrt for performance)
    pub fn distance_squared_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Calculate dot product with another point (treating as vectors)
    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Calculate cross product with another point (treating as vectors)
    pub fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Calculate magnitude (length) of the point as a vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Normalize the point to unit vector
    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        if mag < EPS {
            *self
        } else {
            Point::new(self.x / mag, self.y / mag)
        }
    }

    /// Rotate point around origin by angle (in radians)
    pub fn rotate(&self, angle: f64) -> Point {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Point::new(
            self.x * cos_a - self.y * sin_a,
            self.x * sin_a + self.y * cos_a,
        )
    }

    /// Check if two points are approximately equal (within EPS)
    pub fn approx_eq(&self, other: &Point) -> bool {
        (self.x - other.x).abs() < EPS && (self.y - other.y).abs() < EPS
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;
    
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, scalar: f64) -> Point {
        Point::new(self.x * scalar, self.y * scalar)
    }
}

/// Line structure with intersection and distance calculations
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    /// Create a new line from two points
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    /// Create line from point and direction vector
    pub fn from_point_direction(point: Point, direction: Point) -> Self {
        Line::new(point, point + direction)
    }

    /// Get direction vector of the line
    pub fn direction(&self) -> Point {
        self.b - self.a
    }

    /// Get length of the line segment
    pub fn length(&self) -> f64 {
        self.a.distance_to(&self.b)
    }

    /// Check if point lies on the line segment
    pub fn contains_point(&self, point: &Point) -> bool {
        let cross = (point.y - self.a.y) * (self.b.x - self.a.x) - (point.x - self.a.x) * (self.b.y - self.a.y);
        if cross.abs() > EPS {
            return false;
        }
        
        let dot = (point.x - self.a.x) * (self.b.x - self.a.x) + (point.y - self.a.y) * (self.b.y - self.a.y);
        let len_sq = self.a.distance_squared_to(&self.b);
        
        dot >= -EPS && dot <= len_sq + EPS
    }

    /// Find intersection point with another line
    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let d1 = self.direction();
        let d2 = other.direction();
        
        let cross = d1.cross(&d2);
        if cross.abs() < EPS {
            return None; // Lines are parallel
        }
        
        let diff = other.a - self.a;
        let t = diff.cross(&d2) / cross;
        
        Some(self.a + d1 * t)
    }

    /// Calculate distance from point to line
    pub fn distance_to_point(&self, point: &Point) -> f64 {
        let d = self.direction();
        let diff = *point - self.a;
        let cross = diff.cross(&d);
        cross.abs() / d.magnitude()
    }

    /// Calculate distance from point to line segment
    pub fn distance_to_point_segment(&self, point: &Point) -> f64 {
        let d = self.direction();
        let diff = *point - self.a;
        let dot = diff.dot(&d);
        let len_sq = d.dot(&d);
        
        if dot < 0.0 {
            // Closest point is a
            self.a.distance_to(point)
        } else if dot > len_sq {
            // Closest point is b
            self.b.distance_to(point)
        } else {
            // Closest point is on the segment
            let t = dot / len_sq;
            let closest = self.a + d * t;
            closest.distance_to(point)
        }
    }
}

/// Polygon utilities for area and containment
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    /// Create a new polygon from vertices
    pub fn new(vertices: Vec<Point>) -> Self {
        Polygon { vertices }
    }

    /// Calculate area using shoelace formula
    pub fn area(&self) -> f64 {
        if self.vertices.len() < 3 {
            return 0.0;
        }
        
        let mut area = 0.0;
        let n = self.vertices.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.vertices[i].x * self.vertices[j].y;
            area -= self.vertices[j].x * self.vertices[i].y;
        }
        
        area.abs() / 2.0
    }

    /// Calculate perimeter
    pub fn perimeter(&self) -> f64 {
        if self.vertices.len() < 2 {
            return 0.0;
        }
        
        let mut perimeter = 0.0;
        let n = self.vertices.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            perimeter += self.vertices[i].distance_to(&self.vertices[j]);
        }
        
        perimeter
    }

    /// Check if point is inside polygon using ray casting algorithm
    pub fn contains_point(&self, point: &Point) -> bool {
        if self.vertices.len() < 3 {
            return false;
        }
        
        let mut inside = false;
        let n = self.vertices.len();
        
        let mut j = n - 1;
        for i in 0..n {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];
            
            if ((vi.y > point.y) != (vj.y > point.y)) &&
               (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
                inside = !inside;
            }
            j = i;
        }
        
        inside
    }

    /// Check if polygon is convex
    pub fn is_convex(&self) -> bool {
        if self.vertices.len() < 3 {
            return false;
        }
        
        let n = self.vertices.len();
        let mut sign = 0;
        
        for i in 0..n {
            let p1 = self.vertices[i];
            let p2 = self.vertices[(i + 1) % n];
            let p3 = self.vertices[(i + 2) % n];
            
            let cross = (p2 - p1).cross(&(p3 - p2));
            
            if cross.abs() > EPS {
                let current_sign = if cross > 0.0 { 1 } else { -1 };
                if sign == 0 {
                    sign = current_sign;
                } else if sign != current_sign {
                    return false;
                }
            }
        }
        
        true
    }

    /// Get centroid of the polygon
    pub fn centroid(&self) -> Point {
        if self.vertices.is_empty() {
            return Point::origin();
        }
        
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut area = 0.0;
        let n = self.vertices.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            let cross = self.vertices[i].x * self.vertices[j].y - self.vertices[j].x * self.vertices[i].y;
            area += cross;
            cx += (self.vertices[i].x + self.vertices[j].x) * cross;
            cy += (self.vertices[i].y + self.vertices[j].y) * cross;
        }
        
        area /= 2.0;
        if area.abs() < EPS {
            // Degenerate case, return average of vertices
            let sum = self.vertices.iter().fold(Point::origin(), |acc, &p| acc + p);
            return sum * (1.0 / self.vertices.len() as f64);
        }
        
        cx /= 6.0 * area;
        cy /= 6.0 * area;
        
        Point::new(cx, cy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_operations() {
        let p1 = Point::new(3.0, 4.0);
        let p2 = Point::new(1.0, 2.0);
        
        assert!((p1.magnitude() - 5.0).abs() < EPS);
        assert!((p1.distance_to(&p2) - (8.0_f64).sqrt()).abs() < EPS);
        assert!((p1.dot(&p2) - 11.0).abs() < EPS);
        assert!((p1.cross(&p2) - 2.0).abs() < EPS);
        
        let p3 = p1 + p2;
        assert!(p3.approx_eq(&Point::new(4.0, 6.0)));
        
        let p4 = p1 - p2;
        assert!(p4.approx_eq(&Point::new(2.0, 2.0)));
    }

    #[test]
    fn test_line_operations() {
        let line1 = Line::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
        let line2 = Line::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0));
        
        let intersection = line1.intersection(&line2).unwrap();
        assert!(intersection.approx_eq(&Point::new(1.0, 1.0)));
        
        let distance = line1.distance_to_point(&Point::new(1.0, 0.0));
        assert!((distance - (2.0_f64).sqrt() / 2.0).abs() < EPS);
        
        assert!(line1.contains_point(&Point::new(1.0, 1.0)));
        assert!(!line1.contains_point(&Point::new(1.0, 0.0)));
    }

    #[test]
    fn test_polygon_operations() {
        // Square with vertices at (0,0), (1,0), (1,1), (0,1)
        let square = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ]);
        
        assert!((square.area() - 1.0).abs() < EPS);
        assert!((square.perimeter() - 4.0).abs() < EPS);
        assert!(square.is_convex());
        
        assert!(square.contains_point(&Point::new(0.5, 0.5)));
        assert!(!square.contains_point(&Point::new(1.5, 0.5)));
        
        let centroid = square.centroid();
        assert!(centroid.approx_eq(&Point::new(0.5, 0.5)));
    }

    #[test]
    fn test_triangle_area() {
        // Triangle with vertices at (0,0), (1,0), (0,1)
        let triangle = Polygon::new(vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        ]);
        
        assert!((triangle.area() - 0.5).abs() < EPS);
        assert!(triangle.is_convex());
    }
}