use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        for (i, interval) in intervals.iter().enumerate() {
            btm.insert(interval[0], i);
        }
        let mut answer: Vec<i32> = Vec::with_capacity(intervals.len());
        for interval in intervals.iter() {
            if let Some(e) = btm.range(interval[1]..).next() {
                answer.push(*e.1 as i32);
            } else {
                answer.push(-1);
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
        assert_eq!(vec![-1], Solution::find_right_interval(vec![vec![1, 2]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![-1, 0, 1],
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![-1, 2, -1],
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]])
        );
    }
}
