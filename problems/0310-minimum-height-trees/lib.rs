use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        for edge in edges.iter() {
            v[edge[0] as usize].insert(edge[1] as usize);
            v[edge[1] as usize].insert(edge[0] as usize);
        }
        let mut hs: HashSet<usize> = (0..n as usize).collect();
        while hs.len() > 2 {
            let leaves: Vec<usize> = hs.iter().filter(|&i| v[*i].len() == 1).copied().collect();
            for leaf in leaves.iter() {
                let nodes: Vec<usize> = v[*leaf].iter().copied().collect();
                for i in nodes.iter() {
                    v[*i].remove(leaf);
                }
                hs.remove(leaf);
            }
        }
        hs.iter().map(|&i| i as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]);
        ret.sort();
        assert_eq!(vec![1], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::find_min_height_trees(
            6,
            vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]],
        );
        ret.sort();
        assert_eq!(vec![3, 4], ret);
    }
}
