#[derive(Default)]
pub struct MyHashSet {
    v: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            v: vec![Vec::new(); 1000],
        }
    }
    pub fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.v[key as usize % 1000].push(key)
        }
    }
    pub fn remove(&mut self, key: i32) {
        if let Some(pos) = self.v[key as usize % 1000].iter().position(|&x| x == key) {
            self.v[key as usize % 1000].remove(pos);
        }
    }
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.v[key as usize % 1000].iter().any(|&x| x == key)
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
