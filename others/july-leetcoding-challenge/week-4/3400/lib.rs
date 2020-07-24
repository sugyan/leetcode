pub struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut v: Vec<i32> = vec![0];
        Solution::dfs(&graph, &mut v, 0, &mut answer);
        answer
    }
    fn dfs(graph: &[Vec<i32>], v: &mut Vec<i32>, src: usize, answer: &mut Vec<Vec<i32>>) {
        if src == graph.len() - 1 {
            answer.push(v.clone());
        }
        for &dst in graph[src].iter() {
            v.push(dst as i32);
            Solution::dfs(graph, v, dst as usize, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret: Vec<Vec<i32>> =
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]);
        ret.sort();
        assert_eq!(vec![vec![0, 1, 3], vec![0, 2, 3]], ret);
    }
}
