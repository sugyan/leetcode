#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

pub struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        for word in &words {
            let mut node = &mut trie;
            for u in word.bytes() {
                node = node.children[(u - b'a') as usize].get_or_insert(Default::default());
            }
            node.is_end = true;
        }
        let mut board = board;
        let mut answer = Vec::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::backtrack(
                    &mut board,
                    &mut trie,
                    (i, j),
                    &mut String::new(),
                    &mut answer,
                );
            }
        }
        answer
    }
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        trie: &mut Trie,
        pos: (usize, usize),
        s: &mut String,
        answer: &mut Vec<String>,
    ) {
        let c = board[pos.0][pos.1];
        if let Some(node) = &mut trie.children[(c as u8 - b'a') as usize] {
            s.push(c);
            if node.is_end {
                answer.push(s.clone());
                node.is_end = false;
            }
            board[pos.0][pos.1] = '*';
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = pos.0.wrapping_add(d[0]);
                let j = pos.1.wrapping_add(d[1]);
                if (0..board.len()).contains(&i)
                    && (0..board[0].len()).contains(&j)
                    && board[i][j] != '*'
                {
                    Self::backtrack(board, node, (i, j), s, answer);
                }
            }
            board[pos.0][pos.1] = c;
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec!["oath", "eat"],
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec!["oath", "pea", "eat", "rain"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::find_words(
                vec![vec!['a', 'b'], vec!['c', 'd']],
                vec!["abcd"].into_iter().map(String::from).collect()
            )
        )
    }
}
