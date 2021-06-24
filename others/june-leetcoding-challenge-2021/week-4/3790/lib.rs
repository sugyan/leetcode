use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut grid = HashMap::new();
        grid.insert((start_row, start_column), 1);
        let mut answer = 0;
        for _ in 0..max_move {
            for ((row, col), k) in grid.drain().collect::<Vec<_>>() {
                if row == 0 {
                    answer = (answer + k) % MOD
                }
                if row == m - 1 {
                    answer = (answer + k) % MOD
                }
                if col == 0 {
                    answer = (answer + k) % MOD
                }
                if col == n - 1 {
                    answer = (answer + k) % MOD
                }
                for d in [0, 1, 0, -1, 0].windows(2) {
                    let (row, col) = (row + d[0], col + d[1]);
                    if (0..m).contains(&row) && (0..n).contains(&col) {
                        let v = grid.entry((row, col)).or_default();
                        *v = (*v + k) % MOD;
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

    #[test]
    fn example_1() {
        assert_eq!(6, Solution::find_paths(2, 2, 2, 0, 0));
    }

    #[test]
    fn example_2() {
        assert_eq!(12, Solution::find_paths(1, 3, 3, 0, 1));
    }
}
