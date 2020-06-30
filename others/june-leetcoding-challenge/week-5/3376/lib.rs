use std::collections::HashSet;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

pub struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie: Trie = Default::default();
        for word in words.iter() {
            let mut node = &mut trie;
            for c in word.as_bytes() {
                node =
                    node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            }
            node.word = Some(word.clone());
        }
        let mut answer: HashSet<String> = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
                Solution::dfs(&board, (i, j), &trie, &mut visited, &mut answer);
            }
        }
        answer.into_iter().collect()
    }
    fn dfs(
        board: &[Vec<char>],
        pos: (usize, usize),
        trie: &Trie,
        visited: &mut Vec<Vec<bool>>,
        answer: &mut HashSet<String>,
    ) {
        if visited[pos.0][pos.1] {
            return;
        }
        visited[pos.0][pos.1] = true;
        if let Some(node) = &trie.children[(board[pos.0][pos.1] as u8 - b'a') as usize] {
            if let Some(word) = &node.word {
                answer.insert(word.clone());
            }
            if pos.0 > 0 {
                Solution::dfs(board, (pos.0 - 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 > 0 {
                Solution::dfs(board, (pos.0, pos.1 - 1), node.as_ref(), visited, answer);
            }
            if pos.0 < board.len() - 1 {
                Solution::dfs(board, (pos.0 + 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 < board[0].len() - 1 {
                Solution::dfs(board, (pos.0, pos.1 + 1), node.as_ref(), visited, answer);
            }
        }
        visited[pos.0][pos.1] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret: Vec<String> = Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_string(),
                "pea".to_string(),
                "eat".to_string(),
                "rain".to_string(),
            ],
        );
        ret.sort();
        assert_eq!(vec!["eat", "oath"], ret);
    }
}
