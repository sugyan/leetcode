pub struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); n as usize];
        for edge in &edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut answer = vec![0; n as usize];
        let mut counts = vec![1; n as usize];
        Self::dfs1(&graph, &mut answer, &mut counts, 0, None);
        Self::dfs2(&graph, &mut answer, &mut counts, 0, None);
        answer
    }
    fn dfs1(
        graph: &[Vec<usize>],
        answer: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        i: usize,
        prev: Option<usize>,
    ) {
        for &j in graph[i].iter().filter(|&j| Some(*j) != prev) {
            Self::dfs1(graph, answer, counts, j, Some(i));
            counts[i] += counts[j];
            answer[i] += answer[j] + counts[j];
        }
    }
    fn dfs2(
        graph: &[Vec<usize>],
        answer: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        i: usize,
        prev: Option<usize>,
    ) {
        for &j in graph[i].iter().filter(|&j| Some(*j) != prev) {
            answer[j] = answer[i] - counts[j] + answer.len() as i32 - counts[j];
            Self::dfs2(graph, answer, counts, j, Some(i));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![8, 12, 6, 10, 10, 10],
            Solution::sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![0], Solution::sum_of_distances_in_tree(1, vec![]));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![1, 1],
            Solution::sum_of_distances_in_tree(2, vec![vec![1, 0]])
        );
    }
}
