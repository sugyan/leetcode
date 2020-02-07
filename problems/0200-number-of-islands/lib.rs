use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut answer = 0;
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' && !hs.contains(&(i, j)) {
                    answer += 1;
                    let mut d: VecDeque<(usize, usize)> = VecDeque::new();
                    d.push_back((i, j));
                    while let Some(f) = d.pop_front() {
                        if hs.contains(&(f.0, f.1)) {
                            continue;
                        }
                        hs.insert((f.0, f.1));
                        if f.0 > 0 && grid[f.0 - 1][f.1] == '1' {
                            d.push_back((f.0 - 1, f.1))
                        }
                        if f.1 > 0 && grid[f.0][f.1 - 1] == '1' {
                            d.push_back((f.0, f.1 - 1))
                        }
                        if f.0 < grid.len() - 1 && grid[f.0 + 1][f.1] == '1' {
                            d.push_back((f.0 + 1, f.1))
                        }
                        if f.1 < grid[0].len() - 1 && grid[f.0][f.1 + 1] == '1' {
                            d.push_back((f.0, f.1 + 1))
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
