pub struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut v: Vec<Vec<i32>> = matrix;
        let mut answer = 0;
        for i in 0..v.len() {
            for j in 0..v[0].len() {
                if i > 0 && j > 0 && v[i][j] > 0 {
                    let mut min = v[i - 1][j - 1];
                    min = std::cmp::min(min, v[i - 1][j]);
                    min = std::cmp::min(min, v[i][j - 1]);
                    v[i][j] = min + 1;
                }
                answer += v[i][j];
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
