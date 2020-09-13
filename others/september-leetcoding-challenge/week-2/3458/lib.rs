pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        let mut intervals = intervals;
        intervals.push(new_interval);
        intervals.sort();
        for interval in intervals.iter() {
            answer.push(interval.clone());
            let len = answer.len();
            if len > 1 && answer[len - 2][1] >= answer[len - 1][0] {
                answer[len - 2][1] = std::cmp::max(answer[len - 2][1], answer[len - 1][1]);
                answer.pop();
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
