use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    hm: RefCell<HashMap<i32, i32>>,
    vd: RefCell<VecDeque<i32>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        return LRUCache {
            hm: RefCell::new(HashMap::new()),
            vd: RefCell::new(VecDeque::new()),
            capacity: capacity as usize,
        };
    }

    pub fn get(&self, key: i32) -> i32 {
        let mut vd = self.vd.borrow_mut();
        let mut hm = self.hm.borrow_mut();
        if let Some(val) = hm.get(&key) {
            if let Some(i) = vd.iter().position(|v| *v == key) {
                vd.remove(i);
            } else {
                hm.remove(&key);
                return -1;
            }
            vd.push_front(key);
            return *val;
        }
        return -1;
    }

    pub fn put(&self, key: i32, value: i32) {
        let mut vd = self.vd.borrow_mut();
        if let Some(i) = vd.iter().position(|v| *v == key) {
            vd.remove(i);
        }
        vd.push_front(key);
        vd.truncate(self.capacity);
        self.hm.borrow_mut().insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(1, obj.get(1));
        obj.put(3, 3);
        assert_eq!(-1, obj.get(2));
        obj.put(4, 4);
        assert_eq!(-1, obj.get(1));
        assert_eq!(3, obj.get(3));
        assert_eq!(4, obj.get(4));
    }
}
