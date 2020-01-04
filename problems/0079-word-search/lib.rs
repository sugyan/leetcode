pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == word.chars().nth(0).unwrap()
                    && Solution::dfs(&mut board, i, j, word.as_str())
                {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, word: &str) -> bool {
        if word.len() <= 1 {
            return true;
        }
        let (c, target) = (board[i][j], word.chars().nth(1).unwrap());
        board[i][j] = '*';
        if i > 0 && board[i - 1][j] == target && Solution::dfs(board, i - 1, j, &word[1..]) {
            return true;
        }
        if j > 0 && board[i][j - 1] == target && Solution::dfs(board, i, j - 1, &word[1..]) {
            return true;
        }
        if i < board.len() - 1
            && board[i + 1][j] == target
            && Solution::dfs(board, i + 1, j, &word[1..])
        {
            return true;
        }
        if j < board[0].len() - 1
            && board[i][j + 1] == target
            && Solution::dfs(board, i, j + 1, &word[1..])
        {
            return true;
        }
        board[i][j] = c;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(true, Solution::exist(board, "ABCCED".to_string()));
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(true, Solution::exist(board, "SEE".to_string()));
    }

    #[test]
    fn example_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(false, Solution::exist(board, "ABCB".to_string()));
    }
}
