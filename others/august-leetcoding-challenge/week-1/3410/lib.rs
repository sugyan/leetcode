#[derive(Default)]
pub struct MyHashSet {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            v: vec![0; 1_000_000 / 32 + 1],
        }
    }
    pub fn add(&mut self, key: i32) {
        self.v[key as usize / 32] |= 1 << (key % 32)
    }
    pub fn remove(&mut self, key: i32) {
        self.v[key as usize / 32] &= !(1 << (key % 32))
    }
    /** Returns true if this set contains the specified element */
    pub fn contains(&self, key: i32) -> bool {
        self.v[key as usize / 32] & 1 << (key % 32) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MyHashSet::new();
        obj.add(1);
        obj.add(2);
        assert_eq!(true, obj.contains(1));
        assert_eq!(false, obj.contains(3));
        obj.add(2);
        assert_eq!(true, obj.contains(2));
        obj.remove(2);
        assert_eq!(false, obj.contains(2));
    }
}
