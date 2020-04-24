use std::collections::{BinaryHeap, HashMap};

pub struct LRUCache {
    hm: HashMap<i32, i32>,
    lu: HashMap<i32, usize>,
    bh: BinaryHeap<(usize, i32)>,
    capacity: usize,
    idx: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            hm: HashMap::new(),
            lu: HashMap::new(),
            bh: BinaryHeap::new(),
            capacity: capacity as usize,
            idx: std::usize::MAX,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.hm.get(&key) {
            self.lu.insert(key, self.idx);
            self.bh.push((self.idx, key));
            self.idx -= 1;
            *value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if !self.hm.contains_key(&key) && self.hm.len() == self.capacity {
            while let Some(top) = self.bh.peek() {
                if let Some(v) = self.lu.get(&top.1) {
                    if *v == top.0 {
                        self.hm.remove(&top.1);
                        self.lu.remove(&top.1);
                        break;
                    }
                }
                self.bh.pop();
            }
        }
        self.hm.insert(key, value);
        self.lu.insert(key, self.idx);
        self.bh.push((self.idx, key));
        self.idx -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = LRUCache::new(2);
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
