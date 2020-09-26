pub struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.is_empty() {
            return 0;
        }
        let mut answer = 0;
        for i in 1..time_series.len() {
            answer += std::cmp::min(time_series[i] - time_series[i - 1], duration);
        }
        answer + duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::find_poisoned_duration(vec![1, 4], 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::find_poisoned_duration(vec![1, 2], 2));
    }
}
