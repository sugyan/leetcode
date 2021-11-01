use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut vd = VecDeque::new();
        for (i, row) in board.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if (i == 0 || i == m - 1 || j == 0 || j == n - 1) && *col == 'O' {
                    vd.push_back((i, j));
                }
            }
        }
        while let Some((i, j)) = vd.pop_front() {
            board[i][j] = '#';
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..m).contains(&i) && (0..n).contains(&j) && board[i][j] == 'O' {
                    vd.push_back((i, j));
                }
            }
        }
        for row in board {
            for col in row {
                match *col {
                    'O' => *col = 'X',
                    '#' => *col = 'O',
                    _ => {}
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

    #[test]
    fn example_2() {
        let mut board: Vec<Vec<char>> = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(vec![vec!['X']], board);
    }
}
