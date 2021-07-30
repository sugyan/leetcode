use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    sum: i32,
    children: [Option<Box<Trie>>; 26],
}

#[derive(Default)]
pub struct MapSum {
    trie: Trie,
    values: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: String, val: i32) {
        let delta = val - self.values.get(&key).unwrap_or(&0);
        let mut node = &mut self.trie;
        node.sum += delta;
        for &u in key.as_bytes() {
            node = node.children[(u - b'a') as usize].get_or_insert_with(Default::default);
            node.sum += delta;
        }
        self.values.insert(key, val);
    }

    pub fn sum(&self, prefix: String) -> i32 {
        let mut node = &self.trie;
        for &u in prefix.as_bytes() {
            if let Some(n) = &node.children[(u - b'a') as usize] {
                node = n;
            } else {
                return 0;
            }
        }
        node.sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MapSum::new();
        obj.insert(String::from("apple"), 3);
        assert_eq!(3, obj.sum(String::from("ap")));
        obj.insert(String::from("app"), 2);
        assert_eq!(5, obj.sum(String::from("ap")));
    }
}
