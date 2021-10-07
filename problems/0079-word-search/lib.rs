pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if word.starts_with(board[i][j])
                    && Solution::backtrack(&mut board, i, j, &word[1..])
                {
                    return true;
                }
            }
        }
        false
    }
    fn backtrack(board: &mut Vec<Vec<char>>, i: usize, j: usize, word: &str) -> bool {
        if word.is_empty() {
            return true;
        }
        let (r, c) = (board.len(), board[0].len());
        let ch = board[i][j];
        board[i][j] = '*';
        for d in [0, 1, 0, !0, 0].windows(2) {
            let i = i.wrapping_add(d[0]);
            let j = j.wrapping_add(d[1]);
            if (0..r).contains(&i)
                && (0..c).contains(&j)
                && word.starts_with(board[i][j])
                && Solution::backtrack(board, i, j, &word[1..])
            {
                return true;
            }
        }
        board[i][j] = ch;
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
                String::from("ABCCED")
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
                String::from("SEE")
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
                String::from("ABCB")
            )
        );
    }
}
