pub struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut matchsticks = matchsticks.iter().map(|&m| m as i64).collect::<Vec<_>>();
        let sum = matchsticks.iter().sum::<i64>();
        if sum % 4 != 0 {
            return false;
        }
        matchsticks.sort_unstable();
        let target = sum / 4;
        let mut used = vec![false; matchsticks.len()];
        for _ in 0..4 {
            let mut v = Vec::new();
            if !Self::backtrack(&matchsticks, &mut used, &mut v, target) {
                return false;
            }
        }
        true
    }
    fn backtrack(
        matchsticks: &[i64],
        used: &mut Vec<bool>,
        v: &mut Vec<usize>,
        target: i64,
    ) -> bool {
        if target == 0 {
            v.iter().for_each(|&i| used[i] = true);
            return true;
        }
        for i in (0..*v.last().unwrap_or(&matchsticks.len())).rev() {
            if used[i] || matchsticks[i] > target {
                continue;
            }
            v.push(i);
            if Self::backtrack(matchsticks, used, v, target - matchsticks[i]) {
                return true;
            }
            v.pop();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::makesquare(vec![1, 1, 2, 2, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::makesquare(vec![3, 3, 3, 3, 4]));
    }
}
