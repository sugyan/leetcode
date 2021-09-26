pub struct Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        for i in 1..n {
            if (1..n as i32).contains(&(0..n).map(|j| board[i][j] ^ board[0][j]).sum::<i32>()) {
                return -1;
            }
            if (1..n as i32).contains(&(0..n).map(|j| board[j][i] ^ board[j][0]).sum::<i32>()) {
                return -1;
            }
        }
        if ((0..n).map(|i| board[0][i]).sum::<i32>() * 2 - n as i32).abs() > 1 {
            return -1;
        }
        if ((0..n).map(|i| board[i][0]).sum::<i32>() * 2 - n as i32).abs() > 1 {
            return -1;
        }
        let mut rowdiff = (0..n).filter(|&i| board[0][i] == i as i32 % 2).count();
        if rowdiff % 2 != 0 || (n % 2 == 0 && rowdiff * 2 > n) {
            rowdiff = n - rowdiff;
        }
        let mut coldiff = (0..n).filter(|&i| board[i][0] == i as i32 % 2).count();
        if coldiff % 2 != 0 || (n % 2 == 0 && coldiff * 2 > n) {
            coldiff = n - coldiff;
        }
        (rowdiff + coldiff) as i32 / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::moves_to_chessboard(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 1]
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            -1,
            Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]])
        );
    }
}
