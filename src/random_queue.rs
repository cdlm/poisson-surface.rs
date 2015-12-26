use rand::Rng;

pub trait RandomQueue<T> {
    fn pick<R: Rng>(&mut self, rng: &mut R) -> Option<T>;
}

impl<T> RandomQueue<T> for Vec<T> {
    fn pick<R: Rng>(&mut self, rng: &mut R) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let picked = rng.gen_range(0, self.len());
        Some(self.swap_remove(picked))
    }
}

#[test]
fn test_pick_empty() {
    let mut rng = ::rand::thread_rng();
    let mut vec = Vec::<usize>::new();
    assert!(vec.pick(&mut rng) == None);
}

#[test]
fn test_pick_one() {
    let mut rng = ::rand::thread_rng();
    let mut vec = vec![1];
    assert!(vec.pick(&mut rng) == Some(1));
}

#[test]
fn test_pick_many() {
    let mut rng = ::rand::thread_rng();
    let mut vec = vec![1, 2, 3];
    if let Some(picked) = vec.pick(&mut rng) {
        assert!(picked == 1 || picked == 2 || picked == 3);
        assert!(vec.len() == 2);
        assert!(!vec.contains(&picked));
    } else {
        assert!(false);
    }
}
