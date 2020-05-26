pub struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp: Vec<usize> = vec![0; b.len() + 1];
        for (_, na) in a.iter().enumerate() {
            let mut prev = 0;
            for (j, nb) in b.iter().enumerate() {
                let curr = dp[j + 1];
                dp[j + 1] = if na == nb {
                    prev + 1
                } else {
                    std::cmp::max(dp[j], dp[j + 1])
                };
                prev = curr;
            }
        }
        dp[dp.len() - 1] as i32
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
