use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let last: i32 = *days.last().unwrap();
        let days: HashSet<&i32> = days.iter().collect();
        Solution::dp(0, last, &days, &costs, &mut memo)
    }
    fn dp(
        day: i32,
        last: i32,
        days: &HashSet<&i32>,
        costs: &[i32],
        memo: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(&ret) = memo.get(&day) {
            ret
        } else {
            let ret = if day > last {
                0
            } else if days.contains(&day) {
                let mut min = std::i32::MAX;
                min = std::cmp::min(
                    min,
                    Solution::dp(day + 1, last, days, costs, memo) + costs[0],
                );
                min = std::cmp::min(
                    min,
                    Solution::dp(day + 7, last, days, costs, memo) + costs[1],
                );
                min = std::cmp::min(
                    min,
                    Solution::dp(day + 30, last, days, costs, memo) + costs[2],
                );
                min
            } else {
                Solution::dp(day + 1, last, days, costs, memo)
            };
            memo.insert(day, ret);
            ret
        }
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
