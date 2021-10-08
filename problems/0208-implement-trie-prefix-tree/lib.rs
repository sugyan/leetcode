#[derive(Default)]
pub struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }
    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for i in word.bytes().map(|u| (u - b'a') as usize) {
            node = node.children[i].get_or_insert(Default::default());
        }
        node.is_end = true;
    }
    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        let mut node = self;
        for i in word.bytes().map(|u| (u - b'a') as usize) {
            if let Some(n) = &node.children[i] {
                node = n;
            } else {
                return false;
            }
        }
        node.is_end
    }
    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for i in prefix.bytes().map(|u| (u - b'a') as usize) {
            if let Some(n) = &node.children[i] {
                node = n;
            } else {
                return false;
            }
        }
        true
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
