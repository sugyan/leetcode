use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct WordDictionary {
    hm: HashMap<usize, HashSet<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        WordDictionary { hm: HashMap::new() }
    }
    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let len = word.len();
        if let Some(hs) = self.hm.get_mut(&len) {
            hs.insert(word);
        } else {
            let mut hs: HashSet<String> = HashSet::new();
            hs.insert(word);
            self.hm.insert(len, hs);
        }
    }
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        if let Some(hs) = self.hm.get(&word.len()) {
            let v: Vec<char> = word.chars().collect();
            for s in hs.iter() {
                if (0..).zip(s.chars()).all(|e| v[e.0] == '.' || v[e.0] == e.1) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = WordDictionary::new();
        obj.add_word("bad".to_string());
        obj.add_word("dad".to_string());
        obj.add_word("mad".to_string());
        assert_eq!(false, obj.search("pad".to_string()));
        assert_eq!(true, obj.search("bad".to_string()));
        assert_eq!(true, obj.search(".ad".to_string()));
        assert_eq!(true, obj.search("b..".to_string()));
    }
}
