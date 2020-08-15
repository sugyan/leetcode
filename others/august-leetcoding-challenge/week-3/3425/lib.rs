pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_cached_key(|interval| interval[1]);
        let mut answer = 0;
        let mut end = std::i32::MIN;
        for interval in intervals.iter() {
            if interval[0] < end {
                answer += 1;
            } else {
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
            1,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
        );
    }
}
