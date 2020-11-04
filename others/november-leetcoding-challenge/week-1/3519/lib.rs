use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(n as usize);
        for edge in edges.iter() {
            graph
                .entry(edge[0])
                .or_insert_with(HashSet::new)
                .insert(edge[1]);
            graph
                .entry(edge[1])
                .or_insert_with(HashSet::new)
                .insert(edge[0]);
        }
        while graph.len() > 2 {
            let leaves: Vec<i32> = graph
                .iter()
                .filter(|(_, dsts)| dsts.len() == 1)
                .map(|(&src, _)| src)
                .collect();
            for leaf in leaves.iter() {
                if let Some(dsts) = graph.get(&leaf) {
                    for &i in dsts.clone().iter() {
                        if let Some(d) = graph.get_mut(&i) {
                            d.remove(leaf);
                        }
                    }
                }
                graph.remove(&leaf);
            }
        }
        graph.iter().map(|(&src, _)| src).collect()
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
