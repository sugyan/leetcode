use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (grid.len(), grid[0].len());
        let mut answer = vec![vec![None; grid[0].len()]; grid.len()];
        let mut vd = VecDeque::new();
        if grid[0][0] == 0 {
            vd.push_back(((0, 0), 1));
            answer[0][0] = Some(1);
        }
        while let Some((p, len)) = vd.pop_front() {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if (0..r as i32).contains(&(p.0 + i)) && (0..c as i32).contains(&(p.1 + j)) {
                        let q = ((p.0 + i) as usize, (p.1 + j) as usize);
                        if grid[q.0][q.1] == 0 && answer[q.0][q.1].is_none() {
                            answer[q.0][q.1] = Some(len + 1);
                            vd.push_back(((p.0 + i, p.1 + j), len + 1));
                        }
                    }
                }
            }
        }
        if let Some(len) = answer[r - 1][c - 1] {
            len
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
        );
    }
}
