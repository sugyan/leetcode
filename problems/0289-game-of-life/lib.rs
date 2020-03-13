pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        if board.is_empty() {
            return;
        }
        let (row, col) = (board.len(), board[0].len());
        for i in 0..row {
            for j in 0..col {
                let mut count = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let p = (i as i32 + di, j as i32 + dj);
                        if (p.0 >= 0 && p.0 < row as i32)
                            && (p.1 >= 0 && p.1 < col as i32)
                            && board[p.0 as usize][p.1 as usize] & 1 != 0
                        {
                            count += 1
                        }
                    }
                }
                if count == 3 || (count == 4 && board[i][j] & 1 == 1) {
                    board[i][j] |= 2
                }
            }
        }
        for r in board {
            for c in r {
                *c >>= 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
            board
        );
    }
}
