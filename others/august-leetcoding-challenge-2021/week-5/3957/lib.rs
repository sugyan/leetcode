pub struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mins = ops
            .iter()
            .fold((m, n), |acc, op| (acc.0.min(op[0]), acc.1.min(op[1])));
        mins.0 * mins.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]))
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            )
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(9, Solution::max_count(3, 3, vec![]))
    }
}
