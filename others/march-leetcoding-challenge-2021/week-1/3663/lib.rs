const TABLE_SIZE: usize = 32;

#[derive(Default)]
pub struct MyHashMap {
    table: [Vec<(i32, i32)>; TABLE_SIZE],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** value will always be non-negative. */
    pub fn put(&mut self, key: i32, value: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i][j].1 = value;
        } else {
            self.table[i].push((key, value));
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        let i = (key as usize) % TABLE_SIZE;
        self.table[i]
            .iter()
            .position(|&(k, _)| k == key)
            .map(|j| self.table[i][j].1)
            .unwrap_or(-1)
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    pub fn remove(&mut self, key: i32) {
        let i = (key as usize) % TABLE_SIZE;
        if let Some(j) = self.table[i].iter().position(|&(k, _)| k == key) {
            self.table[i].remove(j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(-1, obj.get(3));
        obj.put(2, 1);
        assert_eq!(1, obj.get(2));
        obj.remove(2);
        assert_eq!(-1, obj.get(2));
    }
}
