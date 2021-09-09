pub struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![1; n]; n];
        for mine in &mines {
            grid[mine[0] as usize][mine[1] as usize] = 0;
        }
        let mut dp = vec![vec![(0, 0, 0, 0); n as usize]; n as usize];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }
                dp[i][j].0 = if i > 0 { dp[i - 1][j].0 } else { 0 } + 1;
                dp[i][j].1 = if j > 0 { dp[i][j - 1].1 } else { 0 } + 1;
            }
        }
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    continue;
                }
                dp[i][j].2 = if i < n - 1 { dp[i + 1][j].2 } else { 0 } + 1;
                dp[i][j].3 = if j < n - 1 { dp[i][j + 1].3 } else { 0 } + 1;
            }
        }
        (0..n)
            .filter_map(|i| {
                (0..n)
                    .map(|j| dp[i][j].0.min(dp[i][j].1).min(dp[i][j].2).min(dp[i][j].3))
                    .max()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]));
    }
}
