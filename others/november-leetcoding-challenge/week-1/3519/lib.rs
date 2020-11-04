use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges.iter() {
            graph.entry(edge[0]).or_insert_with(Vec::new).push(edge[1]);
            graph.entry(edge[1]).or_insert_with(Vec::new).push(edge[0]);
        }
        let mut max = Vec::new();
        let mut v: Vec<i32> = Vec::new();
        Solution::dfs(0, &mut v, &mut max, &graph);
        let mut answer = Vec::new();
        if let Some(&last) = max.last() {
            max.clear();
            v.clear();
            Solution::dfs(last, &mut v, &mut max, &graph);
            answer.extend(max[(max.len() - 1) / 2..=max.len() / 2].iter())
        }
        answer
    }
    fn dfs(src: i32, v: &mut Vec<i32>, max: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>) {
        let mut leaf = true;
        if let Some(dsts) = graph.get(&src) {
            for &dst in dsts.iter() {
                if let Some(&prev) = v.last() {
                    if prev == dst {
                        continue;
                    }
                }
                v.push(src);
                Solution::dfs(dst, v, max, graph);
                v.pop();
                leaf = false;
            }
        }
        if leaf && v.len() >= max.len() {
            *max = v.clone();
            max.push(src);
        }
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
