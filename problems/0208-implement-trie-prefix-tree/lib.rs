use std::collections::HashSet;

pub struct Trie {
    hs: HashSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie { hs: HashSet::new() }
    }
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        self.hs.insert(word);
    }
    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
        self.hs.contains(&word)
    }
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
        for word in self.hs.iter() {
            if word.starts_with(&prefix) {
                return true;
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
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(true, trie.search("apple".to_string()));
        assert_eq!(false, trie.search("app".to_string()));
        assert_eq!(true, trie.starts_with("apple".to_string()));
        trie.insert("app".to_string());
        assert_eq!(true, trie.search("app".to_string()));
    }
}
