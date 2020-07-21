pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if word[0] == board[i][j] {
                    let mut visited: Vec<Vec<bool>> =
                        vec![vec![false; board[0].len()]; board.len()];
                    visited[i][j] = true;
                    if Solution::dfs(&board, &word[1..], (i, j), &mut visited) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn dfs(
        board: &[Vec<char>],
        word: &[char],
        pos: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if word.is_empty() {
            return true;
        }
        let mut v: Vec<(usize, usize)> = Vec::new();
        if pos.0 > 0 && board[pos.0 - 1][pos.1] == word[0] {
            v.push((pos.0 - 1, pos.1));
        }
        if pos.1 > 0 && board[pos.0][pos.1 - 1] == word[0] {
            v.push((pos.0, pos.1 - 1));
        }
        if pos.0 < board.len() - 1 && board[pos.0 + 1][pos.1] == word[0] {
            v.push((pos.0 + 1, pos.1));
        }
        if pos.1 < board[0].len() - 1 && board[pos.0][pos.1 + 1] == word[0] {
            v.push((pos.0, pos.1 + 1));
        }
        for p in v.iter() {
            if !visited[p.0][p.1] {
                visited[p.0][p.1] = true;
                if Solution::dfs(board, &word[1..], (p.0, p.1), visited) {
                    return true;
                }
                visited[p.0][p.1] = false;
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
        assert_eq!(
            true,
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_string()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE".to_string()
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            false,
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB".to_string()
            )
        );
    }
}
