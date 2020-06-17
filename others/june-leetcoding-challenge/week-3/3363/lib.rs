use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let (row, col) = (board.len(), board[0].len());
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        let mut vd: VecDeque<(usize, usize)> = VecDeque::new();
        for (i, r) in board.iter().enumerate() {
            if r[0] == 'O' {
                vd.push_back((i, 0));
            }
            if r[col - 1] == 'O' {
                vd.push_back((i, col - 1));
            }
        }
        for j in 0..col {
            if board[0][j] == 'O' {
                vd.push_back((0, j));
            }
            if board[row - 1][j] == 'O' {
                vd.push_back((row - 1, j));
            }
        }
        while let Some(front) = vd.pop_front() {
            if hs.contains(&front) {
                continue;
            }
            hs.insert(front);
            let (i, j) = front;
            board[i][j] = '*';
            if i > 0 && board[i - 1][j] == 'O' {
                vd.push_back((i - 1, j));
            }
            if j > 0 && board[i][j - 1] == 'O' {
                vd.push_back((i, j - 1));
            }
            if i < row - 1 && board[i + 1][j] == 'O' {
                vd.push_back((i + 1, j));
            }
            if j < col - 1 && board[i][j + 1] == 'O' {
                vd.push_back((i, j + 1));
            }
        }
        for r in board.iter_mut() {
            for c in r.iter_mut() {
                *c = match *c {
                    'O' => 'X',
                    '*' => 'O',
                    c => c,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
            board
        );
    }
}
