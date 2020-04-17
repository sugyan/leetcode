use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let (r, c) = (grid.len(), grid[0].len());
        let mut grid = grid;
        let mut vd: VecDeque<(usize, usize)> = VecDeque::new();
        let mut answer = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == '1' {
                    answer += 1;
                    vd.clear();
                    vd.push_back((i, j));
                    while let Some(f) = vd.pop_front() {
                        if grid[f.0][f.1] == '0' {
                            continue;
                        }
                        grid[f.0][f.1] = '0';
                        if f.0 > 0 && grid[f.0 - 1][f.1] == '1' {
                            vd.push_back((f.0 - 1, f.1))
                        }
                        if f.1 > 0 && grid[f.0][f.1 - 1] == '1' {
                            vd.push_back((f.0, f.1 - 1))
                        }
                        if f.0 < r - 1 && grid[f.0 + 1][f.1] == '1' {
                            vd.push_back((f.0 + 1, f.1))
                        }
                        if f.1 < c - 1 && grid[f.0][f.1 + 1] == '1' {
                            vd.push_back((f.0, f.1 + 1))
                        }
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
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ])
        );
    }
}
