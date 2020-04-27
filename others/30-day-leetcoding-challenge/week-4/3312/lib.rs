pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut dp = vec![0; matrix[0].len()];
        let mut max = 0;
        for i in 0..matrix.len() {
            let mut prev = 0;
            for j in 0..matrix[0].len() {
                let tmp = dp[j];
                if matrix[i][j] == '1' {
                    let mut min = std::cmp::min(prev, dp[j]);
                    if j > 0 {
                        min = std::cmp::min(min, dp[j - 1]);
                    }
                    dp[j] = min + 1;
                    max = std::cmp::max(max, dp[j]);
                } else {
                    dp[j] = 0;
                }
                prev = tmp;
            }
        }
        (max * max) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        )
    }

    #[test]
    fn wa_1() {
        assert_eq!(
            4,
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '1', '0', '1'],
                vec!['1', '1', '1', '1', '1', '1'],
                vec!['0', '1', '1', '0', '1', '1'],
                vec!['1', '1', '1', '0', '1', '0'],
                vec!['0', '1', '1', '1', '1', '1'],
                vec!['1', '1', '0', '1', '1', '1']
            ])
        )
    }
}
