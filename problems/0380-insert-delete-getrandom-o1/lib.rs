use rand::{rngs::ThreadRng, Rng};
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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.hm.contains_key(&val) {
            return false;
        }
        self.hm.insert(val, self.v.len());
        self.v.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&i) = self.hm.get(&val) {
            let len = self.v.len();
            self.hm.insert(self.v[len - 1], i);
            self.hm.remove(&val);
            self.v.swap(i, len - 1);
            self.v.pop();
            true
        } else {
            false
        }
    }

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
        assert!(matches!(obj.get_random(), 1 | 2));
        assert_eq!(true, obj.remove(1));
        assert_eq!(false, obj.insert(2));
        assert_eq!(2, obj.get_random());
    }
}
