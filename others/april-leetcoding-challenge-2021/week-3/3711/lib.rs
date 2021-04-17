use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let sums = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .scan(0, |sum, &x| {
                        *sum += x;
                        Some(*sum)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let len = matrix[0].len();
        let mut answer = 0;
        let mut hm = HashMap::new();
        for i in 0..len {
            for j in i..len {
                hm.clear();
                hm.insert(0, 1);
                let mut sum = 0;
                for row in &sums {
                    sum += row[j] - if i > 0 { row[i - 1] } else { 0 };
                    if let Some(&count) = hm.get(&(sum - target)) {
                        answer += count;
                    }
                    *hm.entry(sum).or_default() += 1;
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::num_submatrix_sum_target(
                vec![
                    vec![0, 1, 0],
                    vec![1, 1, 1],
                    vec![0, 1, 0]
                ],
                0
            )
        );
    }

    #[rustfmt::skip]
    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::num_submatrix_sum_target(
                vec![
                    vec![1, -1],
                    vec![-1, 1]
                ],
                0
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::num_submatrix_sum_target(vec![vec![904]], 0));
    }
}
