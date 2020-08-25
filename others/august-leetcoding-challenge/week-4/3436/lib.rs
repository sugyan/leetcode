use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let hs: HashSet<&i32> = days.iter().collect();
        let mut dp = vec![0; 366];
        for day in 1..=365 {
            dp[day as usize] = if hs.contains(&day) {
                let mut min = std::i32::MAX;
                min = std::cmp::min(min, dp[std::cmp::max(0, day - 1) as usize] + costs[0]);
                min = std::cmp::min(min, dp[std::cmp::max(0, day - 7) as usize] + costs[1]);
                min = std::cmp::min(min, dp[std::cmp::max(0, day - 30) as usize] + costs[2]);
                min
            } else {
                dp[day as usize - 1]
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            11,
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            17,
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
        );
    }
}
