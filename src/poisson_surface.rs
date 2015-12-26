use rand::distributions::{IndependentSample, Range};
use std::slice::Iter;

use point::Point;


struct PoissonSurface {
    width: f64,
    height: f64,
    distance: f64,
    jitter: f64,
    candidates: i8,
    queue: Vec<Point>,
    points: Vec<Point>, // grid…
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
            points: Vec::new(),
        }
    }

    fn random_point(&self) -> Point {
        let mut rng = ::rand::thread_rng();
        let x = Range::new(0.0, self.width).ind_sample(&mut rng);
        let y = Range::new(0.0, self.height).ind_sample(&mut rng);
        Point::new(x, y)
    }

    fn candidate_nearby(&self, seed: Point) -> Option<Point> {
        (0..self.candidates)
            .map(|_| seed.jittered(self.jitter * self.distance, self.distance))
            .find(|&candidate|
                  candidate.is_in_rectangle(self.width, self.height)
                  && !self.is_too_close(candidate))
    }

    fn insert(&mut self, point: Point) {
        // insert in proximity grid
        self.points.push(point)
    }

    fn is_too_close(&self, candidate: Point) -> bool {
        self.neighbours_iter(candidate).any(|pt| candidate.distance(pt) < self.distance)
    }

    fn points_iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn neighbours_iter<'a>(&'a self, point: Point) -> Box<Iterator<Item = &'a Point> + 'a> {
        Box::new(self.points_iter().filter(move |&pt| point.distance(&pt) < self.distance))
    }
}


#[test]
fn test_surface() {
    let mut s = PoissonSurface::new();
    assert!(s.candidates == 10);
    assert!(s.random_point().is_in_rectangle(s.width, s.height));

    assert!(s.points_iter().collect::<Vec<&Point>>().is_empty());

    let p1 = Point::new(0.5, 0.5);
    let p2 = Point::new(0.45, 0.55);

    s.insert(p1);
    assert!(s.points_iter().count() == 1);
    assert!(s.points_iter().collect::<Vec<&Point>>().contains(&&p1));
    assert!(s.is_too_close(p2));

    s.insert(p2);
    let n = s.neighbours_iter(p1).collect::<Vec<&Point>>();
    assert!(n.len() == 2);
    assert!(n.contains(&&p1));
    assert!(n.contains(&&p2));

    if let Some(c) = s.candidate_nearby(p1) {
        assert!(!s.is_too_close(c));
    }
}
