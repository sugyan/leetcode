pub struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        let mut sums = vec![0; stones.len() + 1];
        for (i, &s) in stones.iter().enumerate() {
            sums[i + 1] = sums[i] + s;
        }
        for i in 1..stones.len() {
            for j in 0..stones.len() - i {
                let k = i + j;
                dp[j][k] = std::cmp::max(
                    sums[k + 1] - sums[j + 1] - dp[j + 1][k],
                    sums[k] - sums[j] - dp[j][k - 1],
                );
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
