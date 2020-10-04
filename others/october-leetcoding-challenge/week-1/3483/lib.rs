pub struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|v: &Vec<i32>| (v[0], -v[1]));
        let mut answer = 0;
        let mut end = 0;
        for interval in intervals.iter() {
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

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![0, 10], vec![5, 12]])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            2,
            Solution::remove_covered_intervals(vec![vec![3, 10], vec![4, 10], vec![5, 11]])
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            1,
            Solution::remove_covered_intervals(vec![vec![1, 2], vec![1, 4], vec![3, 4]])
        );
    }
}
