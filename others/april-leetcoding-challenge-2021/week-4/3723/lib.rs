pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let (m, n) = (obstacle_grid[0].len(), obstacle_grid.len());
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = 1;
        for i in 0..n {
            for j in 0..m {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                if i > 0 && obstacle_grid[i - 1][j] == 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 && obstacle_grid[i][j - 1] == 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
        dp[n - 1][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 1],
                vec![0, 0]
            ])
        );
    }
}
