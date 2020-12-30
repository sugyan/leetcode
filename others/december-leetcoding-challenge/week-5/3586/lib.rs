pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let count = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .filter(|&d| {
                    let (ii, jj) = (i as i32 + d.0, j as i32 + d.1);
                    (0..board.len() as i32).contains(&ii)
                        && (0..board[0].len() as i32).contains(&jj)
                        && board[ii as usize][jj as usize] & 1 != 0
                })
                .count();
                if count == 3 || (board[i][j] & 1 != 0 && count == 2) {
                    board[i][j] |= 1 << 1;
                }
            }
        }
        board
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|col| *col >>= 1));
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

    #[test]
    fn example_2() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(vec![vec![1, 1], vec![1, 1]], board);
    }
}
