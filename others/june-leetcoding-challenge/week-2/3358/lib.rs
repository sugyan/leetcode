use rand::Rng;
use std::collections::HashSet;

#[derive(Default)]
pub struct RandomizedSet {
    hs: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    pub fn insert(&mut self, val: i32) -> bool {
        if self.hs.contains(&val) {
            false
        } else {
            self.hs.insert(val);
            true
        }
    }
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    pub fn remove(&mut self, val: i32) -> bool {
        if self.hs.contains(&val) {
            self.hs.remove(&val);
            true
        } else {
            false
        }
    }
    /** Get a random element from the set. */
    pub fn get_random(&self) -> i32 {
        let v: Vec<&i32> = self.hs.iter().collect();
        let mut rng = rand::thread_rng();
        *v[rng.gen_range(0, v.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = RandomizedSet::new();
        assert_eq!(true, obj.insert(1));
        assert_eq!(false, obj.remove(2));
        assert_eq!(true, obj.insert(2));
        let ret: i32 = obj.get_random();
        assert!(ret == 1 || ret == 2);
        assert_eq!(true, obj.remove(1));
        assert_eq!(false, obj.insert(2));
        assert_eq!(2, obj.get_random());
    }
}
