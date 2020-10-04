pub struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a: &Vec<i32>, b: &Vec<i32>| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            ne => ne,
        });
        let mut answer = intervals.len() as i32;
        let mut i = 0;
        while i < intervals.len() {
            let end = intervals[i][1];
            i += 1;
            while i < intervals.len() && intervals[i][1] <= end {
                answer -= 1;
                i += 1;
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
