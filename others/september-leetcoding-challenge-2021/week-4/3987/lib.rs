use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut mins = vec![vec![HashMap::new(); cols]; rows];
        let mut vd = VecDeque::new();
        vd.push_back(((0, 0), 0, k));
        while let Some(((i, j), steps, k)) = vd.pop_front() {
            if let Some(&min) = mins[i][j].get(&k) {
                if min <= steps {
                    continue;
                }
            }
            mins[i][j].insert(k, steps);
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..rows).contains(&i) && (0..cols).contains(&j) && k - grid[i][j] >= 0 {
                    vd.push_back(((i, j), steps + 1, k - grid[i][j]));
                }
            }
        }
        *mins[rows - 1][cols - 1].values().min().unwrap_or(&-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            6,
            Solution::shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            -1,
            Solution::shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1)
        );
    }
}
