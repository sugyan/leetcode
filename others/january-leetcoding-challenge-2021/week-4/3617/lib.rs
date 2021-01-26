use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        let (r, c) = (heights.len(), heights[0].len());
        for i in 0..r {
            for j in 0..c {
                let height = heights[i][j];
                if i > 0 {
                    graph
                        .entry((i, j))
                        .or_insert_with(Vec::new)
                        .push(((i - 1, j), (heights[i - 1][j] - height).abs()));
                }
                if j > 0 {
                    graph
                        .entry((i, j))
                        .or_insert_with(Vec::new)
                        .push(((i, j - 1), (heights[i][j - 1] - height).abs()));
                }
                if i < r - 1 {
                    graph
                        .entry((i, j))
                        .or_insert_with(Vec::new)
                        .push(((i + 1, j), (heights[i + 1][j] - height).abs()));
                }
                if j < c - 1 {
                    graph
                        .entry((i, j))
                        .or_insert_with(Vec::new)
                        .push(((i, j + 1), (heights[i][j + 1] - height).abs()));
                }
            }
        }
        let (mut lo, mut hi) = (0, 1_000_000);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Self::reachable(&graph, (r - 1, c - 1), mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
    fn reachable(
        graph: &HashMap<(usize, usize), Vec<((usize, usize), i32)>>,
        target: (usize, usize),
        k: i32,
    ) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![(0, 0)];
        while let Some(pos) = stack.pop() {
            if pos == target {
                return true;
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            if let Some(v) = graph.get(&pos) {
                for (p, effort) in v {
                    if !visited.contains(p) && *effort <= k {
                        stack.push(*p);
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn example_1() {
        assert_eq!(
            2,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 2],
                vec![3, 8, 2],
                vec![5, 3, 5]
            ])
        );
    }

    #[test]
    #[rustfmt::skip]
    fn example_2() {
        assert_eq!(
            1,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 3],
                vec![3, 8, 4],
                vec![5, 3, 5]
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::minimum_effort_path(vec![
                vec![1, 2, 1, 1, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 1, 2, 1]
            ])
        );
    }
}
