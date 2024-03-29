pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let len = profit.len();
        let mut index = (0..len).collect::<Vec<_>>();
        index.sort_unstable_by_key(|&i| end_time[i]);
        let mut dp = vec![0; len];
        for i in 0..len {
            let profit = profit[index[i]]
                + match index[..i].binary_search_by_key(&start_time[index[i]], |&j| end_time[j]) {
                    Ok(j) => dp[j],
                    Err(j) if j == 0 => 0,
                    Err(j) => dp[j - 1],
                };
            dp[i] = (if i > 0 { dp[i - 1] } else { 0 }).max(profit);
        }
        dp[len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            120,
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            150,
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            6,
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4])
        );
    }
}
