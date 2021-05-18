use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        for input in &paths {
            let v = input.split(' ').collect::<Vec<_>>();
            for &file in &v[1..] {
                if let Some(pos) = file.chars().position(|c| c == '(') {
                    hm.entry(&file[pos + 1..file.len() - 1])
                        .or_insert_with(Vec::new)
                        .push(String::new() + v[0] + "/" + &file[..pos]);
                }
            }
        }
        hm.values().filter(|&v| v.len() > 1).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::find_duplicate(
            vec![
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
                "root 4.txt(efgh)",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        );
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(
            ret,
            vec![
                vec!["root/4.txt", "root/a/2.txt", "root/c/d/4.txt"],
                vec!["root/a/1.txt", "root/c/3.txt"]
            ]
        );
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::find_duplicate(
            vec![
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        );
        for v in ret.iter_mut() {
            v.sort_unstable();
        }
        ret.sort();
        assert_eq!(
            ret,
            vec![
                vec!["root/a/1.txt", "root/c/3.txt"],
                vec!["root/a/2.txt", "root/c/d/4.txt"]
            ]
        );
    }
}
