use std::f64::consts::PI;
#[derive(Debug, PartialEq, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    pub fn distance(&self, p2: &Point) -> f64 {
        let diff = Point {
            x: self.x - p2.x,
            y: self.y - p2.y,
        };
        (diff.x.powi(2) + diff.y.powi(2)).sqrt()
    }
}
impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point::new(x, y),
            radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    pub fn intersect(&self, other: &Self) -> bool {
        self.center.distance(&other.center) < self.radius + other.radius
    }
}