use std::collections::VecDeque;

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    end: bool,
}

pub struct StreamChecker {
    trie: Trie,
    maxlen: usize,
    history: VecDeque<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie: Trie = Default::default();
        let mut maxlen = 0;
        for word in words.iter() {
            maxlen = std::cmp::max(maxlen, word.len());
            let mut node = &mut trie;
            for &c in word.as_bytes().iter().rev() {
                node = node.nodes[(c - b'a') as usize].get_or_insert(Default::default());
            }
            node.end = true;
        }
        Self {
            trie,
            maxlen,
            history: VecDeque::new(),
        }
    }
    pub fn query(&mut self, letter: char) -> bool {
        self.history.push_back((letter as u8 - b'a') as usize);
        if self.history.len() > self.maxlen {
            self.history.pop_front();
        }
        let mut node = &self.trie;
        for &i in self.history.iter().rev() {
            if let Some(n) = &node.nodes[i] {
                if n.end {
                    return true;
                }
                node = n.as_ref();
            } else {
                break;
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
        let words = ["cd", "f", "kl"].iter().map(|&s| s.to_string()).collect();
        let mut obj = StreamChecker::new(words);
        assert_eq!(false, obj.query('a'));
        assert_eq!(false, obj.query('b'));
        assert_eq!(false, obj.query('c'));
        assert_eq!(true, obj.query('d'));
        assert_eq!(false, obj.query('e'));
        assert_eq!(true, obj.query('f'));
        assert_eq!(false, obj.query('g'));
        assert_eq!(false, obj.query('h'));
        assert_eq!(false, obj.query('i'));
        assert_eq!(false, obj.query('j'));
        assert_eq!(false, obj.query('k'));
        assert_eq!(true, obj.query('l'));
    }
}
