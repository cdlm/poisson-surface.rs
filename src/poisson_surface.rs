use rand::distributions::{IndependentSample, Range};
use std::slice::Iter;

use point::Point;
use random_queue::RandomQueue;


pub struct PoissonSurface {
    width: f64,
    height: f64,
    distance: f64,
    jitter: f64,
    candidates: i8,
    queue: Vec<Point>,
    points: Vec<Point>, // grid…
}

impl PoissonSurface {
    pub fn new() -> PoissonSurface {
        PoissonSurface {
            width: 640.0,
            height: 480.0,
            distance: 10.0,
            candidates: 10,
            jitter: 2.0,
            queue: Vec::new(),
            points: Vec::new(),
        }
    }

    pub fn random_point(&self) -> Point {
        let mut rng = ::rand::thread_rng();
        let x = Range::new(0.0, self.width).ind_sample(&mut rng);
        let y = Range::new(0.0, self.height).ind_sample(&mut rng);
        Point::new(x, y)
    }

    pub fn candidate_nearby(&self, seed: Point) -> Option<Point> {
        (0..self.candidates)
            .map(|_| seed.jittered(self.jitter * self.distance, self.distance))
            .find(|&candidate| {
                candidate.is_in_rectangle(self.width, self.height) && !self.is_too_close(candidate)
            })
    }

    pub fn insert(&mut self, point: Point) {
        // insert in proximity grid
        self.queue.push(point);
        self.points.push(point);
    }

    pub fn is_too_close(&self, candidate: Point) -> bool {
        self.neighbours_iter(candidate).any(|pt| candidate.distance(pt) < self.distance)
    }

    pub fn points_iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    pub fn neighbours_iter<'a>(&'a self, point: Point) -> Box<Iterator<Item = &'a Point> + 'a> {
        Box::new(self.points_iter().filter(move |&pt| point.distance(&pt) < self.distance))
    }

    pub fn generate_point(&mut self) -> Option<Point> {
        if let Some(seed) = self.queue.pick(&mut ::rand::thread_rng()) {
            if let Some(candidate) = self.candidate_nearby(seed) {
                self.queue.push(seed); // put the seed back since it's not done yet
                return Some(candidate);
            }
        }
        None
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
