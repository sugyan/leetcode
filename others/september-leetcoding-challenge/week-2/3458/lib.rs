pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            answer.push(intervals[i].clone());
            i += 1;
        }
        let mut new_interval = new_interval;
        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
            new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        answer.push(new_interval);
        while i < intervals.len() {
            answer.push(intervals[i].clone());
            i += 1;
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
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }
}
