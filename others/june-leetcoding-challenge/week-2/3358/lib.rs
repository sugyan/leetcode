use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
pub struct RandomizedSet {
    v: Vec<i32>,
    hm: HashMap<i32, usize>,
    tr: ThreadRng,
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
        if self.hm.contains_key(&val) {
            return false;
        }
        self.hm.insert(val, self.v.len());
        self.v.push(val);
        true
    }
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&mut i) = self.hm.get_mut(&val) {
            if i < self.v.len() - 1 {
                let last = self.v[self.v.len() - 1];
                self.v[i] = last;
                self.hm.insert(last, i);
            }
            self.v.pop();
            self.hm.remove(&val);
            true
        } else {
            false
        }
    }
    /** Get a random element from the set. */
    pub fn get_random(&mut self) -> i32 {
        self.v[self.tr.gen_range(0, self.v.len())]
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
