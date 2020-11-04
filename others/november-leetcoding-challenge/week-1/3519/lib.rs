use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
        let mut neighbors: Vec<usize> = vec![0; n as usize];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
            neighbors[edge[0] as usize] += 1;
            neighbors[edge[1] as usize] += 1;
        }
        let mut n = n;
        let mut vd: VecDeque<usize> = (0..n as usize).filter(|&i| neighbors[i] == 1).collect();
        while n > 2 {
            for _ in 0..vd.len() {
                if let Some(front) = vd.pop_front() {
                    n -= 1;
                    neighbors[front] -= 1;
                    for &node in graph[front].iter() {
                        neighbors[node] -= 1;
                        if neighbors[node] == 1 {
                            vd.push_back(node);
                        }
                    }
                }
            }
        }
        vd.iter().map(|&i| i as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        );
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::find_min_height_trees(
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
        );
        ret.sort_unstable();
        assert_eq!(vec![3, 4], ret);
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0], Solution::find_min_height_trees(1, vec![]));
    }

    #[test]
    fn example_4() {
        let mut ret = Solution::find_min_height_trees(2, vec![vec![0, 1]]);
        ret.sort_unstable();
        assert_eq!(vec![0, 1], ret);
    }
}
