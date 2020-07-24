pub struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rev: Vec<Vec<usize>> = vec![Vec::new(); graph.len()];
        for (src, v) in graph.iter().enumerate() {
            for &dst in v.iter() {
                rev[dst as usize].push(src);
            }
        }
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let mut v: Vec<i32> = vec![graph.len() as i32 - 1];
        Solution::dfs(&rev, &mut v, graph.len() - 1, &mut answer);
        answer
    }
    fn dfs(rev: &[Vec<usize>], v: &mut Vec<i32>, src: usize, answer: &mut Vec<Vec<i32>>) {
        if src == 0 {
            let mut v = v.clone();
            v.reverse();
            answer.push(v);
        }
        for &dst in rev[src].iter() {
            v.push(dst as i32);
            Solution::dfs(rev, v, dst, answer);
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
