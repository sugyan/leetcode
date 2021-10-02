pub struct Solution {}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![1000; m]; n];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                dp[i][j] = 1.max(
                    match (i < n - 1, j < m - 1) {
                        (true, true) => dp[i + 1][j].min(dp[i][j + 1]),
                        (true, false) => dp[i + 1][j],
                        (false, true) => dp[i][j + 1],
                        _ => 1,
                    } - dungeon[i][j],
                );
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            7,
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ])
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::calculate_minimum_hp(vec![vec![0]]))
    }
}
