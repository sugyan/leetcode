use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_cached_key(|v| (v[0], Reverse(v[1])));
        let (mut answer, mut end) = (0, 0);
        for interval in &intervals {
            if interval[1] > end {
                answer += 1;
                end = interval[1];
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]])
        );
    }
}
