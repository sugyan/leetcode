pub struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix: Vec<Vec<i32>> = matrix;
        let mut answer = 0;
        let mut target = 1;
        loop {
            let mut end = true;
            for i in 0..matrix.len() {
                for j in 0..matrix[0].len() {
                    if matrix[i][j] == target {
                        answer += 1;
                        end = false;
                    }
                    if (i > 0 && j > 0)
                        && matrix[i][j] == target
                        && matrix[i - 1][j] >= target
                        && matrix[i][j - 1] >= target
                        && matrix[i - 1][j - 1] >= target
                    {
                        matrix[i][j] += 1;
                    }
                }
            }
            if end {
                break;
            }
            target += 1;
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
            15,
            Solution::count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            7,
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
        );
    }
}
