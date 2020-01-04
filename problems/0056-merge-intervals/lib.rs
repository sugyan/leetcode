pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut intervals = intervals;
        intervals.sort();
        for interval in intervals {
            if match answer.last() {
                Some(last) => interval[0] > last[1],
                None => true,
            } {
                answer.push(interval.clone());
            } else if let Some(last) = answer.last_mut() {
                last[1] = std::cmp::max(last[1], interval[1]);
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
        let mut ret = Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
        ret.sort();
        assert_eq!(vec![vec![1, 6], vec![8, 10], vec![15, 18]], ret)
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::merge(vec![vec![1, 4], vec![4, 5]]);
        ret.sort();
        assert_eq!(vec![vec![1, 5]], ret)
    }
}
