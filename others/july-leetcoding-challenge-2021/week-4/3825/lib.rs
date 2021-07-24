use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut dict = HashMap::new();
        for word in &word_list {
            for i in 0..word.len() {
                let key = String::new() + &word[..i] + "*" + &word[i + 1..];
                dict.entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(word.as_str());
            }
        }
        let graph = Self::bfs(&dict, &begin_word, &word_list);
        let mut answers = Vec::new();
        let mut v = Vec::new();
        Self::dfs(&graph, &mut v, &begin_word, &end_word, &mut answers);
        answers
    }
    fn bfs(
        dict: &HashMap<String, HashSet<&str>>,
        begin: &str,
        word_list: &[String],
    ) -> HashMap<String, HashSet<String>> {
        let mut graph = HashMap::new();
        let mut hs = word_list.iter().collect::<HashSet<_>>();
        let mut vd = VecDeque::new();
        vd.push_back(begin);
        hs.remove(&begin.to_string());
        while !vd.is_empty() {
            let mut visited = Vec::new();
            for _ in 0..vd.len() {
                if let Some(word) = vd.pop_front() {
                    let mut neighbors = HashSet::<&str>::new();
                    for i in 0..word.len() {
                        let key = String::new() + &word[..i] + "*" + &word[i + 1..];
                        if let Some(s) = dict.get(&key) {
                            neighbors.extend(s);
                        }
                    }
                    neighbors.retain(|n| hs.contains(&n.to_string()));
                    for &n in &neighbors {
                        graph
                            .entry(word.to_string())
                            .or_insert_with(HashSet::new)
                            .insert(n.to_string());
                        visited.push(n);
                        vd.push_back(n);
                    }
                }
            }
            for v in &visited {
                hs.remove(&v.to_string());
            }
        }
        graph
    }
    fn dfs(
        graph: &HashMap<String, HashSet<String>>,
        v: &mut Vec<String>,
        begin: &str,
        end: &str,
        answers: &mut Vec<Vec<String>>,
    ) {
        if begin == end {
            let mut answer = v.clone();
            answer.push(end.to_string());
            answers.push(answer);
            return;
        }
        if let Some(s) = graph.get(begin) {
            for word in s {
                if v.contains(&word.to_string()) {
                    continue;
                }
                v.push(begin.to_string());
                Self::dfs(graph, v, &word, end, answers);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::find_ladders(
            String::from("hit"),
            String::from("cog"),
            vec!["hot", "dot", "dog", "lot", "log", "cog"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        ret.sort();
        assert_eq!(
            vec![
                vec!["hit", "hot", "dot", "dog", "cog"],
                vec!["hit", "hot", "lot", "log", "cog"]
            ],
            ret
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<Vec<String>>::new(),
            Solution::find_ladders(
                String::from("hit"),
                String::from("cog"),
                vec!["hot", "dot", "dog", "lot", "log"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
        );
    }
}
