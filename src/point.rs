use ::rand::distributions::{IndependentSample, Range};


#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn origin() -> Point {
        Point::new(0.0, 0.0)
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn jittered(&self, within: f64, beyond: f64) -> Point {
        use std::f64::consts::PI;
        let mut rng = ::rand::thread_rng();
        let (inner, outer) = (beyond.powi(2), within.powi(2));
        let radius = Range::new(inner, outer).ind_sample(&mut rng).sqrt();
        let angle = Range::new(0.0, 2.0 * PI).ind_sample(&mut rng);
        let x = self.x + radius * angle.cos();
        let y = self.y + radius * angle.sin();
        Point::new(x, y)
    }

    pub fn is_in_rectangle(&self, width: f64, height: f64) -> bool {
        self.x > 0.0 && self.y < width && self.y > 0.0 && self.y < height
    }
}

#[test]
fn test_point() {
    let pt = Point::new(4.2, 5.1);
    assert!(pt.x == 4.2);
    assert!(pt.y == 5.1);
}
