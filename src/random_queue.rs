use rand::Rng;

trait RandomQueue<T> {
    fn pick<R: Rng>(&mut self, rng: &mut R) -> Option<T>;
}

impl<T> RandomQueue<T> for Vec<T> {
    fn pick<R: Rng>(&mut self, rng: &mut R) -> Option<T> {
        if self.is_empty() { return None }
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
