extern crate rand;
use rand::distributions::{IndependentSample, Range};

use std::iter::Filter;
use std::slice::Iter;
use std::iter::FromIterator;


#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y:f64) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point::new(0.0, 0.0)
    }

    fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y -self.y).powi(2)).sqrt()
    }

    fn jittered(&self, within: f64, beyond: f64) -> Point {
        use std::f64::consts::PI;
        let mut rng = rand::thread_rng();
        let (inner, outer) = (beyond.powi(2), within.powi(2));
        let radius = Range::new(inner, outer).ind_sample(&mut rng).sqrt();
        let angle = Range::new(0.0, 2.0 * PI).ind_sample(&mut rng);
        let x = self.x + radius * angle.cos();
        let y = self.y + radius * angle.sin();
        Point::new(x, y)
    }

    fn is_in_rectangle(&self, width: f64, height: f64) -> bool {
        self.x > 0.0
            && self.y < width
            && self.y > 0.0
            && self.y < height
    }
}

struct PoissonSurface {
    width: f64,
    height: f64,
    distance: f64,
    jitter: f64,
    candidates: i8,
    queue: Vec<Point>,
    points: Vec<Point>
        // grid…
}

impl PoissonSurface {
    fn new() -> PoissonSurface {
        PoissonSurface {
            width: 1.0,
            height: 1.0,
            distance: 0.1,
            candidates: 10,
            jitter: 2.0,
            queue: Vec::new(),
            points: Vec::new()
        }
    }

    fn random_point(&self) -> Point {
        let mut rng = rand::thread_rng();
        let x = Range::new(0.0, self.width).ind_sample(&mut rng);
        let y = Range::new(0.0, self.height).ind_sample(&mut rng);
        Point::new(x, y)
    }

    fn candidate_nearby(&self, seed: Point) -> Option<Point> {
        let candidate = seed.jittered(self.jitter * self.distance, self.distance);
        if !candidate.is_in_rectangle(self.width, self.height) ||
            self.is_too_close(candidate) {
                return None
            }
        Some(candidate)
    }

    fn insert(&mut self, point: Point) {
        // insert in proximity grid
        self.points.push(point)
    }

    fn is_too_close(&self, candidate: Point) -> bool {
        self.neighbours_iter(candidate).any(|pt| candidate.distance(pt) < self.distance)
    }

    fn points_iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }

    fn neighbours_iter<'a>(&'a self, point: Point) -> Box<Iterator<Item=&'a Point> + 'a> {
        Box::new(self.points_iter().filter(move |&pt| point.distance(&pt) < self.distance))
    }


}

#[test]
fn test_point() {
    let pt = Point::new(4.2, 5.1);
    assert!(pt.x == 4.2);
    assert!(pt.y == 5.1);
}

#[test]
fn test_surface() {
    let mut s = PoissonSurface::new();
    assert!(s.candidates == 10);
    assert!(s.random_point().x <= 1.0);
    assert!(s.random_point().y <= 1.0);

    assert!(s.points_iter().collect::<Vec<&Point>>().is_empty());

    let p = Point::new(0.5, 0.5);
    s.insert(p);
    assert!(s.points_iter().collect::<Vec<&Point>>().len() == 1);
    assert!(s.is_too_close(Point::new(0.55, 0.45)));

    if let Some(c) = s.candidate_nearby(p) {
        assert!(!s.is_too_close(c));
        println!("Candidate: {:?}", c);
    } else {
        println!("Unlucky candidate");
    }
}
