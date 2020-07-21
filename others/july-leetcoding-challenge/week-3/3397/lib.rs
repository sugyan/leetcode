pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if word[0] == board[i][j] {
                    let mut visited: Vec<Vec<bool>> =
                        vec![vec![false; board[0].len()]; board.len()];
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
        if visited[pos.0][pos.1] {
            return false;
        }
        if word.is_empty() {
            return true;
        }
        visited[pos.0][pos.1] = true;
        if (false
            || (pos.0 > 0
                && board[pos.0 - 1][pos.1] == word[0]
                && Solution::dfs(board, &word[1..], (pos.0 - 1, pos.1), visited)))
            || (pos.1 > 0
                && board[pos.0][pos.1 - 1] == word[0]
                && Solution::dfs(board, &word[1..], (pos.0, pos.1 - 1), visited))
            || (pos.0 < board.len() - 1
                && board[pos.0 + 1][pos.1] == word[0]
                && Solution::dfs(board, &word[1..], (pos.0 + 1, pos.1), visited))
            || (pos.1 < board[0].len() - 1
                && board[pos.0][pos.1 + 1] == word[0]
                && Solution::dfs(board, &word[1..], (pos.0, pos.1 + 1), visited))
        {
            return true;
        }
        visited[pos.0][pos.1] = false;
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
