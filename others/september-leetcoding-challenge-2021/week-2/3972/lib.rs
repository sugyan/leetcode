use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let mut graph = vec![Vec::new(); n as usize];
        for edge in &edges {
            graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
            graph[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
        let mut remains = vec![None; n as usize];
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), 0));
        let mut answer = 0;
        while let Some((Reverse(min), u)) = bh.pop() {
            if min > max_moves {
                break;
            }
            if remains[u].is_some() {
                continue;
            }
            answer += 1;
            remains[u] = Some(max_moves - min);
            for &(v, c) in graph[u].iter().filter(|(v, _)| remains[*v].is_none()) {
                bh.push((Reverse(min + c + 1), v));
            }
        }
        for edge in &edges {
            answer += edge[2].min(
                remains[edge[0] as usize].unwrap_or(0) + remains[edge[1] as usize].unwrap_or(0),
            );
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
            13,
            Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            23,
            Solution::reachable_nodes(
                vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
                10,
                4
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            1,
            Solution::reachable_nodes(
                vec![
                    vec![1, 2, 4],
                    vec![1, 4, 5],
                    vec![1, 3, 1],
                    vec![2, 3, 4],
                    vec![3, 4, 5]
                ],
                17,
                5
            )
        );
    }
}
