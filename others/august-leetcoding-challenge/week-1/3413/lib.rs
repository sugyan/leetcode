#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

#[derive(Default)]
pub struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }
    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.trie;
        for &c in word.as_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }
        node.end = true;
    }
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        WordDictionary::search_trie(&self.trie, &word.as_bytes())
    }
    fn search_trie(trie: &Trie, word: &[u8]) -> bool {
        if let Some(&c) = word.first() {
            if c == b'.' {
                for child in &trie.children {
                    if let Some(node) = child {
                        if WordDictionary::search_trie(&node, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else if let Some(node) = &trie.children[(c - b'a') as usize] {
                return WordDictionary::search_trie(&node, &word[1..]);
            }
            false
        } else {
            trie.end
        }
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
