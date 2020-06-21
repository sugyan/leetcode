pub struct Solution {}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![1000; dungeon[0].len()]; dungeon.len()];
        for i in (0..dungeon.len()).rev() {
            for j in (0..dungeon[i].len()).rev() {
                let hp = -dungeon[i][j]
                    + if i < dungeon.len() - 1 && j < dungeon[i].len() - 1 {
                        std::cmp::min(dp[i + 1][j], dp[i][j + 1])
                    } else if i < dungeon.len() - 1 {
                        dp[i + 1][j]
                    } else if j < dungeon[i].len() - 1 {
                        dp[i][j + 1]
                    } else {
                        1
                    };
                dp[i][j] = if hp > 0 { hp } else { 1 };
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
}
