pub struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                dp[i + 1][j + 1] = if a[i] == b[j] {
                    dp[i][j] + 1
                } else {
                    std::cmp::max(dp[i + 1][j], dp[i][j + 1])
                };
            }
        }
        *dp.last().unwrap().last().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1])
        );
    }
}
