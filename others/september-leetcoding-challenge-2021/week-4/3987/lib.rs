use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![None; cols]; rows];
        visited[0][0] = Some(k);
        let mut vd = VecDeque::new();
        vd.push_back(((0, 0), 0, k));
        while let Some(((i, j), steps, k)) = vd.pop_front() {
            if i == rows - 1 && j == cols - 1 {
                return steps;
            }
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..rows).contains(&i) && (0..cols).contains(&j) && k - grid[i][j] >= 0 {
                    if let Some(v) = visited[i][j] {
                        if k - grid[i][j] <= v {
                            continue;
                        }
                    }
                    visited[i][j] = Some(k - grid[i][j]);
                    vd.push_back(((i, j), steps + 1, k - grid[i][j]));
                }
            }
        }
        -1
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
