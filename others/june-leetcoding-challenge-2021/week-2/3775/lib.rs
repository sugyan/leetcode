pub struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        if stones.len() % 2 == 0 {
            for (i, &s) in stones.iter().enumerate() {
                dp[i][i] = s;
            }
        }
        for i in 0..stones.len() - 1 {
            for j in 0..stones.len() - i - 1 {
                let k = j + i + 1;
                dp[j][k] = if i % 2 != stones.len() % 2 {
                    (dp[j][k - 1] + stones[k]).min(dp[j + 1][k] + stones[j])
                } else {
                    dp[j][k - 1].max(dp[j + 1][k])
                };
            }
        }
        dp[0][stones.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::stone_game_vii(vec![5, 3, 1, 4, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            122,
            Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2])
        );
    }
}
