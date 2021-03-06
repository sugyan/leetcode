pub struct Solution;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::default();
        for word in &words {
            let mut node = &mut trie;
            for &u in word.as_bytes().iter().rev() {
                node = node.children[(u - b'a') as usize].get_or_insert_with(Default::default);
            }
        }
        Self::dfs(&trie, 0) as i32
    }
    fn dfs(t: &Trie, depth: usize) -> usize {
        let (mut ret, mut end) = (0, true);
        for child in &t.children {
            if let Some(c) = child {
                end = false;
                ret += Self::dfs(&c, depth + 1);
            }
        }
        if end {
            depth + 1
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            10,
            Solution::minimum_length_encoding(
                vec!["time", "me", "bell"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::minimum_length_encoding(vec![String::from("t")])
        );
    }
}
