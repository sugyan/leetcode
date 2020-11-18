pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable();
        let mut answer = Vec::new();
        answer.push(intervals[0].clone());
        for interval in intervals.iter() {
            if let Some(last) = answer.last_mut() {
                if last[1] < interval[0] {
                    answer.push(interval.clone());
                } else {
                    last[1] = std::cmp::max(last[1], interval[1]);
                }
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
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::merge(vec![vec![1, 4], vec![4, 5]])
        );
    }
}
