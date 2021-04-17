pub struct Solution;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (r, c) = (matrix.len(), matrix[0].len());
        let mut sum = vec![vec![0; c]; r];
        for i in 0..r {
            for j in 0..c {
                sum[i][j] = matrix[i][j]
                    + if i > 0 { sum[i - 1][j] } else { 0 }
                    + if j > 0 { sum[i][j - 1] } else { 0 }
                    - if i > 0 && j > 0 { sum[i - 1][j - 1] } else { 0 };
            }
        }
        let mut answer = 0;
        for i in 0..r {
            for j in 0..c {
                for k in 0..=i {
                    for l in 0..=j {
                        if target
                            == sum[i][j]
                                - if k > 0 { sum[k - 1][j] } else { 0 }
                                - if l > 0 { sum[i][l - 1] } else { 0 }
                                + if k > 0 && l > 0 { sum[k - 1][l - 1] } else { 0 }
                        {
                            answer += 1;
                        }
                    }
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
