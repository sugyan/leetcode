use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};

pub struct LRUCache {
    hm: RefCell<HashMap<i32, i32>>,
    lu: RefCell<HashMap<i32, usize>>,
    bh: RefCell<BinaryHeap<(usize, i32)>>,
    capacity: usize,
    idx: RefCell<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        return LRUCache {
            hm: RefCell::new(HashMap::new()),
            lu: RefCell::new(HashMap::new()),
            bh: RefCell::new(BinaryHeap::new()),
            capacity: capacity as usize,
            idx: RefCell::new(std::usize::MAX),
        };
    }

    pub fn get(&self, key: i32) -> i32 {
        if let Some(value) = self.hm.borrow().get(&key) {
            let mut idx = self.idx.borrow_mut();
            self.lu.borrow_mut().insert(key, *idx);
            self.bh.borrow_mut().push((*idx, key));
            *idx -= 1;
            return *value;
        }
        return -1;
    }

    pub fn put(&self, key: i32, value: i32) {
        let mut hm = self.hm.borrow_mut();
        let mut lu = self.lu.borrow_mut();
        let mut bh = self.bh.borrow_mut();
        let mut idx = self.idx.borrow_mut();
        if !hm.contains_key(&key) && hm.len() == self.capacity {
            loop {
                if let Some(top) = bh.peek() {
                    if let Some(v) = lu.get(&top.1) {
                        if *v == top.0 {
                            hm.remove(&top.1);
                            lu.remove(&top.1);
                            break;
                        }
                    }
                } else {
                    break;
                }
                bh.pop();
            }
        }
        hm.insert(key, value);
        lu.insert(key, *idx);
        bh.push((*idx, key));
        *idx -= 1;
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
