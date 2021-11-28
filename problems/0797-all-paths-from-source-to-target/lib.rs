pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        Self::backtrack(&graph, &mut vec![0], 0, &mut answer);
        answer
    }
    fn backtrack(graph: &[Vec<i32>], v: &mut Vec<i32>, src: usize, answer: &mut Vec<Vec<i32>>) {
        if src == graph.len() - 1 {
            answer.push(v.clone());
            return;
        }
        for &i in &graph[src] {
            v.push(i);
            Self::backtrack(graph, v, i as usize, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]);
        ret.sort();
        assert_eq!(vec![vec![0, 1, 3], vec![0, 2, 3]], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::all_paths_source_target(vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![],
        ]);
        ret.sort();
        assert_eq!(
            vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 4],
                vec![0, 3, 4],
                vec![0, 4],
            ],
            ret
        );
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::all_paths_source_target(vec![vec![1], vec![]]);
        ret.sort();
        assert_eq!(vec![vec![0, 1]], ret);
    }

    #[test]
    fn example_4() {
        let mut ret =
            Solution::all_paths_source_target(vec![vec![1, 2, 3], vec![2], vec![3], vec![]]);
        ret.sort();
        assert_eq!(vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]], ret);
    }

    #[test]
    fn example_5() {
        let mut ret = Solution::all_paths_source_target(vec![vec![1, 3], vec![2], vec![3], vec![]]);
        ret.sort();
        assert_eq!(vec![vec![0, 1, 2, 3], vec![0, 3]], ret);
    }
}
