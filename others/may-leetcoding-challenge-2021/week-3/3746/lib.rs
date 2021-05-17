pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut groups = vec![Vec::new(); 17];
        for (i, word) in words.iter().enumerate() {
            groups[word.len()].push((word, i));
        }
        let mut graph = vec![Vec::new(); words.len()];
        for i in (1..groups.len()).rev() {
            for &(word, dst) in &groups[i] {
                for j in 0..word.len() {
                    let removed = String::new() + &word[0..j] + &word[j + 1..];
                    for &(word, src) in &groups[i - 1] {
                        if &removed == word {
                            graph[src].push(dst);
                        }
                    }
                }
            }
        }
        let mut depth = vec![None; words.len()];
        for i in 0..words.len() {
            if depth[i].is_none() {
                Self::dfs(&graph, &mut depth, i);
            }
        }
        depth.iter().filter_map(|&o| o).max().unwrap()
    }
    fn dfs(graph: &[Vec<usize>], depth: &mut Vec<Option<i32>>, i: usize) -> i32 {
        if let Some(memo) = depth[i] {
            return memo;
        }
        let mut ret = 1;
        for &dst in &graph[i] {
            ret = ret.max(Self::dfs(graph, depth, dst) + 1);
        }
        depth[i] = Some(ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            4,
            Solution::longest_str_chain(
                vec!["a", "b", "ba", "bca", "bda", "bdca"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            5,
            Solution::longest_str_chain(
                vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
                    .into_iter()
                    .map(str::to_string)
                    .collect()
            )
        );
    }
}
