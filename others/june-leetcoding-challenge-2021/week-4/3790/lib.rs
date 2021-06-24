const MOD: i32 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        dp[start_row as usize][start_column as usize] = 1;
        let mut answer = 0;
        for _ in 0..max_move {
            let mut tmp = vec![vec![0; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if i == 0 {
                        answer = (answer + dp[i][j]) % MOD;
                    }
                    if i == m - 1 {
                        answer = (answer + dp[i][j]) % MOD;
                    }
                    if j == 0 {
                        answer = (answer + dp[i][j]) % MOD;
                    }
                    if j == n - 1 {
                        answer = (answer + dp[i][j]) % MOD;
                    }
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let row = i.wrapping_add(d[0]);
                        let col = j.wrapping_add(d[1]);
                        if (0..m).contains(&row) && (0..n).contains(&col) {
                            tmp[row][col] = (tmp[row][col] + dp[i][j]) % MOD;
                        }
                    }
                }
            }
            dp = tmp;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::find_paths(2, 2, 2, 0, 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(12, Solution::find_paths(1, 3, 3, 0, 1));
    }
}
