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

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn test_new_point() {
    let pt = Point::new(4.2, 5.1);
    assert!(pt.x == 4.2);
    assert!(pt.y == 5.1);
}

#[test]
fn test_point_origin() {
    let pt = Point::origin();
        assert!(pt.x == 0.0);
        assert!(pt.y == 0.0);
}

#[test]
fn test_distance_to_self() {
    let pt = Point::new(4.2, 5.1);
    let d = pt.distance(&pt);
    assert!(d == 0.0);
}

#[test]
fn test_distance_to_origin() {
    let pt = Point::new(4.0, 3.0);
    let d1 = pt.distance(&Point::origin());
    let d2 = Point::origin().distance(&pt);
    assert!(d1 == 5.0);
    assert!(d2 == 5.0);
}
